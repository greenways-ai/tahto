//! SQLite implementation of the `tahto-metadata-store/1` provider contract.
//!
//! The provider persists opaque canonical HTA state. It does not execute Hara,
//! interpret application payloads, verify signatures, or own key authority.

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tahto_metadata_store_abi::{
    validate_digest, Adapter, CommitPlan, CommitReceipt, CommitStatus, Error, Snapshot,
};

pub const PACKAGE_COORDINATE: &str = "gh:greenways-ai/tahto/native/provider/metadata-sqlite";
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONTRACT: &str = tahto_metadata_store_abi::ABI_ID;
pub const CONTRACT_VERSION: &str = tahto_metadata_store_abi::ABI_VERSION;
pub const NATIVE_ABI: &str = tahto_metadata_store_abi::NATIVE_ABI;
pub const SCHEMA_VERSION: i64 = 1;

const SCHEMA: &str = "
PRAGMA journal_mode=WAL;
PRAGMA synchronous=FULL;
PRAGMA foreign_keys=ON;
CREATE TABLE IF NOT EXISTS metadata_snapshot (
  singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
  revision INTEGER NOT NULL CHECK (revision >= 0),
  state BLOB NOT NULL,
  state_digest TEXT NOT NULL
) STRICT;
CREATE TABLE IF NOT EXISTS metadata_receipts (
  plan_digest TEXT PRIMARY KEY,
  revision INTEGER NOT NULL UNIQUE CHECK (revision > 0),
  request_digest TEXT NOT NULL,
  result_digest TEXT NOT NULL,
  state_digest TEXT NOT NULL,
  completed_at TEXT NOT NULL
) STRICT;
CREATE INDEX IF NOT EXISTS metadata_receipts_request
  ON metadata_receipts(request_digest);
PRAGMA user_version=1;
";

pub struct SqliteMetadataStore {
    path: PathBuf,
    connection: Connection,
}

impl SqliteMetadataStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref().to_path_buf();
        let connection = Connection::open(&path).map_err(database_error)?;
        connection
            .busy_timeout(Duration::from_secs(5))
            .map_err(database_error)?;
        connection.execute_batch(SCHEMA).map_err(database_error)?;
        let store = Self { path, connection };
        store.verify()?;
        Ok(store)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn verify(&self) -> Result<(), Error> {
        let foreign_keys: i64 = self
            .connection
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .map_err(database_error)?;
        if foreign_keys != 1 {
            return Err(Error::new(
                "sqlite-foreign-keys-disabled",
                "SQLite metadata stores must enforce foreign keys",
            ));
        }

        let version: i64 = self
            .connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .map_err(database_error)?;
        if version != SCHEMA_VERSION {
            return Err(Error::new(
                "sqlite-schema-version",
                format!("expected schema {SCHEMA_VERSION}, found {version}"),
            ));
        }

        let synchronous: i64 = self
            .connection
            .query_row("PRAGMA synchronous", [], |row| row.get(0))
            .map_err(database_error)?;
        if synchronous < 2 {
            return Err(Error::new(
                "sqlite-synchronous-unsafe",
                "SQLite metadata stores require FULL synchronous commits",
            ));
        }

        let journal_mode: String = self
            .connection
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .map_err(database_error)?;
        if !self.is_memory() && !journal_mode.eq_ignore_ascii_case("wal") {
            return Err(Error::new(
                "sqlite-journal-mode",
                format!("expected WAL journal mode, found {journal_mode}"),
            ));
        }

        let table_count: i64 = self
            .connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'table'
                   AND name IN ('metadata_snapshot', 'metadata_receipts')",
                [],
                |row| row.get(0),
            )
            .map_err(database_error)?;
        if table_count != 2 {
            return Err(Error::new(
                "sqlite-schema-incomplete",
                format!("expected two metadata tables, found {table_count}"),
            ));
        }

        let quick_check: String = self
            .connection
            .query_row("PRAGMA quick_check", [], |row| row.get(0))
            .map_err(database_error)?;
        if quick_check != "ok" {
            return Err(Error::new("sqlite-quick-check", quick_check));
        }

        let snapshot = load_snapshot_on(&self.connection)?;
        let receipt_count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM metadata_receipts", [], |row| row.get(0))
            .map_err(database_error)?;
        if snapshot.is_none() && receipt_count != 0 {
            return Err(Error::new(
                "sqlite-store-corrupt",
                "receipts exist without a metadata snapshot",
            ));
        }

        if let Some(snapshot) = snapshot {
            let maximum_receipt_revision: Option<i64> = self
                .connection
                .query_row("SELECT MAX(revision) FROM metadata_receipts", [], |row| row.get(0))
                .map_err(database_error)?;
            if maximum_receipt_revision
                .map(|revision| revision < 0 || revision as u64 > snapshot.revision)
                .unwrap_or(false)
            {
                return Err(Error::new(
                    "sqlite-store-corrupt",
                    "a receipt revision is newer than the installed snapshot",
                ));
            }
        }

        let plan_digests = {
            let mut statement = self
                .connection
                .prepare("SELECT plan_digest FROM metadata_receipts ORDER BY revision")
                .map_err(database_error)?;
            let rows = statement
                .query_map([], |row| row.get::<_, String>(0))
                .map_err(database_error)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(database_error)?
        };
        for plan_digest in plan_digests {
            receipt_on(&self.connection, &plan_digest)?.ok_or_else(|| {
                Error::new(
                    "sqlite-store-corrupt",
                    "a listed metadata receipt could not be loaded",
                )
            })?;
        }

        Ok(())
    }

    fn is_memory(&self) -> bool {
        self.path.to_string_lossy() == ":memory:"
    }
}

impl Adapter for SqliteMetadataStore {
    fn load(&self) -> Result<Option<Snapshot>, Error> {
        load_snapshot_on(&self.connection)
    }

    fn initialize(&mut self, snapshot: Snapshot) -> Result<Snapshot, Error> {
        snapshot.validate()?;
        verify_state_digest(&snapshot.state, &snapshot.state_digest)?;

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(database_error)?;

        if let Some(existing) = load_snapshot_on(&transaction)? {
            if existing == snapshot {
                transaction.commit().map_err(database_error)?;
                return Ok(existing);
            }
            return Err(Error::new(
                "metadata-already-initialized",
                format!("metadata store already has revision {}", existing.revision),
            ));
        }

        let receipt_count: i64 = transaction
            .query_row("SELECT COUNT(*) FROM metadata_receipts", [], |row| row.get(0))
            .map_err(database_error)?;
        if receipt_count != 0 {
            return Err(Error::new(
                "metadata-store-corrupt",
                "cannot initialize a store containing orphan receipts",
            ));
        }

        transaction
            .execute(
                "INSERT INTO metadata_snapshot(singleton, revision, state, state_digest)
                 VALUES (1, ?1, ?2, ?3)",
                params![to_i64(snapshot.revision)?, snapshot.state, snapshot.state_digest],
            )
            .map_err(database_error)?;
        transaction.commit().map_err(database_error)?;
        Ok(snapshot)
    }

    fn compare_and_swap(&mut self, plan: CommitPlan) -> Result<CommitReceipt, Error> {
        plan.validate()?;
        verify_state_digest(&plan.state, &plan.state_digest)?;

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(database_error)?;

        if let Some(existing) = receipt_on(&transaction, &plan.plan_digest)? {
            if existing.matches_plan(&plan) {
                let replayed = CommitReceipt {
                    status: CommitStatus::Replayed,
                    ..existing
                };
                transaction.commit().map_err(database_error)?;
                return Ok(replayed);
            }
            return Err(Error::new(
                "metadata-plan-conflict",
                "the plan digest is already bound to different commit evidence",
            ));
        }

        let receipt_at_revision: Option<String> = transaction
            .query_row(
                "SELECT plan_digest FROM metadata_receipts WHERE revision = ?1",
                [to_i64(plan.revision)?],
                |row| row.get(0),
            )
            .optional()
            .map_err(database_error)?;
        if let Some(plan_digest) = receipt_at_revision {
            return Err(Error::new(
                "metadata-revision-receipt-conflict",
                format!(
                    "revision {} is already bound to plan {plan_digest}",
                    plan.revision
                ),
            ));
        }

        let current = load_snapshot_on(&transaction)?.ok_or_else(|| {
            Error::new(
                "metadata-not-initialized",
                "initialize the metadata store before committing plans",
            )
        })?;
        if current.revision != plan.expected_revision {
            return Err(Error::new(
                "metadata-revision-conflict",
                format!(
                    "expected revision {}, current revision {}",
                    plan.expected_revision, current.revision
                ),
            ));
        }

        let changed = transaction
            .execute(
                "UPDATE metadata_snapshot
                 SET revision = ?1, state = ?2, state_digest = ?3
                 WHERE singleton = 1 AND revision = ?4",
                params![
                    to_i64(plan.revision)?,
                    plan.state,
                    plan.state_digest,
                    to_i64(plan.expected_revision)?
                ],
            )
            .map_err(database_error)?;
        if changed != 1 {
            return Err(Error::new(
                "metadata-revision-conflict",
                "the metadata snapshot changed during compare-and-swap",
            ));
        }

        let receipt = CommitReceipt::from_plan(&plan, CommitStatus::Applied);
        receipt.validate()?;
        transaction
            .execute(
                "INSERT INTO metadata_receipts(
                   plan_digest, revision, request_digest, result_digest,
                   state_digest, completed_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    receipt.plan_digest,
                    to_i64(receipt.revision)?,
                    receipt.request_digest,
                    receipt.result_digest,
                    receipt.state_digest,
                    receipt.completed_at
                ],
            )
            .map_err(database_error)?;
        transaction.commit().map_err(database_error)?;
        Ok(receipt)
    }

    fn receipt(&self, plan_digest: &str) -> Result<Option<CommitReceipt>, Error> {
        validate_digest(plan_digest, "plan digest")?;
        receipt_on(&self.connection, plan_digest)
    }
}

pub fn canonical_state_digest(state: &[u8]) -> String {
    let digest = Sha256::digest(state);
    let mut value = String::with_capacity(71);
    value.push_str("sha256:");
    for byte in digest {
        write!(&mut value, "{byte:02x}").expect("writing to a String cannot fail");
    }
    value
}

fn verify_state_digest(state: &[u8], expected: &str) -> Result<(), Error> {
    validate_digest(expected, "state digest")?;
    let actual = canonical_state_digest(state);
    if actual == expected {
        Ok(())
    } else {
        Err(Error::new(
            "metadata-state-digest-mismatch",
            format!("expected {expected}, calculated {actual}"),
        ))
    }
}

fn load_snapshot_on(connection: &Connection) -> Result<Option<Snapshot>, Error> {
    let row: Option<(i64, Vec<u8>, String)> = connection
        .query_row(
            "SELECT revision, state, state_digest
             FROM metadata_snapshot WHERE singleton = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()
        .map_err(database_error)?;

    row.map(|(revision, state, state_digest)| {
        if revision < 0 {
            return Err(Error::new(
                "metadata-snapshot-corrupt",
                "stored revision is negative",
            ));
        }
        let snapshot = Snapshot::new(revision as u64, state, state_digest).map_err(|error| {
            Error::new("metadata-snapshot-corrupt", error.to_string())
        })?;
        verify_state_digest(&snapshot.state, &snapshot.state_digest)?;
        Ok(snapshot)
    })
    .transpose()
}

fn receipt_on(
    connection: &Connection,
    plan_digest: &str,
) -> Result<Option<CommitReceipt>, Error> {
    validate_digest(plan_digest, "plan digest")?;
    let row: Option<(i64, String, String, String, String)> = connection
        .query_row(
            "SELECT revision, request_digest, result_digest, state_digest, completed_at
             FROM metadata_receipts WHERE plan_digest = ?1",
            [plan_digest],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            },
        )
        .optional()
        .map_err(database_error)?;

    row.map(
        |(revision, request_digest, result_digest, state_digest, completed_at)| {
            if revision <= 0 {
                return Err(Error::new(
                    "metadata-receipt-corrupt",
                    "stored receipt revision must be positive",
                ));
            }
            let receipt = CommitReceipt {
                status: CommitStatus::Applied,
                revision: revision as u64,
                plan_digest: plan_digest.to_owned(),
                request_digest,
                result_digest,
                state_digest,
                completed_at,
            };
            receipt
                .validate()
                .map_err(|error| Error::new("metadata-receipt-corrupt", error.to_string()))?;
            Ok(receipt)
        },
    )
    .transpose()
}

fn to_i64(revision: u64) -> Result<i64, Error> {
    i64::try_from(revision).map_err(|_| {
        Error::new(
            "metadata-revision-invalid",
            "revision does not fit in SQLite INTEGER",
        )
    })
}

fn database_error(error: rusqlite::Error) -> Error {
    Error::new("sqlite", error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_DATABASE: AtomicU64 = AtomicU64::new(1);

    struct TemporaryDatabase {
        path: PathBuf,
    }

    impl TemporaryDatabase {
        fn new(label: &str) -> Self {
            let id = NEXT_DATABASE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "tahto-metadata-{label}-{}-{id}.sqlite",
                std::process::id()
            ));
            remove_database_files(&path);
            Self { path }
        }
    }

    impl Drop for TemporaryDatabase {
        fn drop(&mut self) {
            remove_database_files(&self.path);
        }
    }

    fn remove_database_files(path: &Path) {
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(format!("{}-wal", path.display()));
        let _ = fs::remove_file(format!("{}-shm", path.display()));
    }

    fn frame(value: &str) -> Vec<u8> {
        format!("HTA1{value}").into_bytes()
    }

    fn digest(fill: char) -> String {
        format!("sha256:{}", fill.to_string().repeat(64))
    }

    fn snapshot(revision: u64, state: Vec<u8>) -> Snapshot {
        Snapshot::new(revision, state.clone(), canonical_state_digest(&state)).unwrap()
    }

    fn plan(
        expected_revision: u64,
        plan_fill: char,
        request_fill: char,
        result_fill: char,
        state: Vec<u8>,
        completed_at: &str,
    ) -> CommitPlan {
        CommitPlan::new(
            expected_revision,
            expected_revision + 1,
            digest(plan_fill),
            digest(request_fill),
            digest(result_fill),
            state.clone(),
            canonical_state_digest(&state),
            completed_at,
        )
        .unwrap()
    }

    #[test]
    fn opens_and_verifies_the_complete_schema() {
        let database = TemporaryDatabase::new("schema");
        let store = SqliteMetadataStore::open(&database.path).unwrap();
        assert_eq!(store.path(), database.path.as_path());
        store.verify().unwrap();
        assert_eq!(CONTRACT, "tahto/metadata-store");
        assert_eq!(NATIVE_ABI, "tahto-metadata-store/1");
    }

    #[test]
    fn initializes_loads_and_reopens_the_snapshot() {
        let database = TemporaryDatabase::new("reopen");
        let expected = snapshot(0, frame("initial"));
        {
            let mut store = SqliteMetadataStore::open(&database.path).unwrap();
            assert_eq!(store.initialize(expected.clone()).unwrap(), expected);
            assert_eq!(store.load().unwrap(), Some(expected.clone()));
        }
        let reopened = SqliteMetadataStore::open(&database.path).unwrap();
        assert_eq!(reopened.load().unwrap(), Some(expected));
    }

    #[test]
    fn initialization_replays_only_the_exact_snapshot() {
        let database = TemporaryDatabase::new("initialize");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        let initial = snapshot(0, frame("initial"));
        store.initialize(initial.clone()).unwrap();
        assert_eq!(store.initialize(initial.clone()).unwrap(), initial);

        let conflict = store
            .initialize(snapshot(0, frame("different")))
            .unwrap_err();
        assert_eq!(conflict.code, "metadata-already-initialized");
    }

    #[test]
    fn compare_and_swap_persists_one_snapshot_and_receipt() {
        let database = TemporaryDatabase::new("commit");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        store.initialize(snapshot(0, frame("initial"))).unwrap();
        let plan = plan(
            0,
            'a',
            'b',
            'c',
            frame("revision-one"),
            "2026-08-07T14:00:00Z",
        );

        let receipt = store.compare_and_swap(plan.clone()).unwrap();
        assert_eq!(receipt.status, CommitStatus::Applied);
        assert!(receipt.matches_plan(&plan));
        assert_eq!(store.load().unwrap().unwrap().revision, 1);
        assert_eq!(store.receipt(&plan.plan_digest).unwrap(), Some(receipt));
    }

    #[test]
    fn stale_revision_conflicts_do_not_mutate_state() {
        let database = TemporaryDatabase::new("stale");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        store.initialize(snapshot(0, frame("initial"))).unwrap();
        let first = plan(
            0,
            'a',
            'b',
            'c',
            frame("first"),
            "2026-08-07T14:01:00Z",
        );
        store.compare_and_swap(first.clone()).unwrap();
        let installed = store.load().unwrap().unwrap();

        let stale = plan(
            0,
            'd',
            'e',
            'f',
            frame("stale"),
            "2026-08-07T14:02:00Z",
        );
        let error = store.compare_and_swap(stale.clone()).unwrap_err();
        assert_eq!(error.code, "metadata-revision-conflict");
        assert_eq!(store.load().unwrap(), Some(installed));
        assert!(store.receipt(&stale.plan_digest).unwrap().is_none());
    }

    #[test]
    fn state_digest_mismatch_rolls_back_before_the_transaction() {
        let database = TemporaryDatabase::new("digest");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        let initial = snapshot(0, frame("initial"));
        store.initialize(initial.clone()).unwrap();
        let mut plan = plan(
            0,
            'a',
            'b',
            'c',
            frame("correct"),
            "2026-08-07T14:03:00Z",
        );
        plan.state = frame("tampered");

        let error = store.compare_and_swap(plan.clone()).unwrap_err();
        assert_eq!(error.code, "metadata-state-digest-mismatch");
        assert_eq!(store.load().unwrap(), Some(initial));
        assert!(store.receipt(&plan.plan_digest).unwrap().is_none());
    }

    #[test]
    fn exact_plan_retry_is_replayed_even_after_later_commits() {
        let database = TemporaryDatabase::new("replay");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        store.initialize(snapshot(0, frame("initial"))).unwrap();
        let first = plan(
            0,
            'a',
            'b',
            'c',
            frame("first"),
            "2026-08-07T14:04:00Z",
        );
        let second = plan(
            1,
            'd',
            'e',
            'f',
            frame("second"),
            "2026-08-07T14:05:00Z",
        );
        store.compare_and_swap(first.clone()).unwrap();
        store.compare_and_swap(second).unwrap();

        let replay = store.compare_and_swap(first.clone()).unwrap();
        assert_eq!(replay.status, CommitStatus::Replayed);
        assert!(replay.matches_plan(&first));
        assert_eq!(store.load().unwrap().unwrap().revision, 2);
    }

    #[test]
    fn one_plan_digest_cannot_be_rebound() {
        let database = TemporaryDatabase::new("plan-conflict");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        store.initialize(snapshot(0, frame("initial"))).unwrap();
        let first = plan(
            0,
            'a',
            'b',
            'c',
            frame("first"),
            "2026-08-07T14:06:00Z",
        );
        store.compare_and_swap(first.clone()).unwrap();

        let conflicting = CommitPlan::new(
            1,
            2,
            first.plan_digest.clone(),
            digest('d'),
            digest('e'),
            frame("conflicting"),
            canonical_state_digest(&frame("conflicting")),
            "2026-08-07T14:07:00Z",
        )
        .unwrap();
        let error = store.compare_and_swap(conflicting).unwrap_err();
        assert_eq!(error.code, "metadata-plan-conflict");
        assert_eq!(store.load().unwrap().unwrap().revision, 1);
    }

    #[test]
    fn public_plan_fields_are_revalidated_at_the_provider_boundary() {
        let database = TemporaryDatabase::new("revalidate");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        store.initialize(snapshot(0, frame("initial"))).unwrap();
        let mut invalid = plan(
            0,
            'a',
            'b',
            'c',
            frame("invalid"),
            "2026-08-07T14:08:00Z",
        );
        invalid.revision = 2;

        let error = store.compare_and_swap(invalid).unwrap_err();
        assert_eq!(error.code, "revision-step-invalid");
        assert_eq!(store.load().unwrap().unwrap().revision, 0);
    }

    #[test]
    fn two_connections_reject_the_stale_writer() {
        let database = TemporaryDatabase::new("writers");
        let mut first_store = SqliteMetadataStore::open(&database.path).unwrap();
        first_store
            .initialize(snapshot(0, frame("initial")))
            .unwrap();
        let mut second_store = SqliteMetadataStore::open(&database.path).unwrap();
        assert_eq!(second_store.load().unwrap().unwrap().revision, 0);

        first_store
            .compare_and_swap(plan(
                0,
                'a',
                'b',
                'c',
                frame("winner"),
                "2026-08-07T14:09:00Z",
            ))
            .unwrap();
        let error = second_store
            .compare_and_swap(plan(
                0,
                'd',
                'e',
                'f',
                frame("loser"),
                "2026-08-07T14:10:00Z",
            ))
            .unwrap_err();
        assert_eq!(error.code, "metadata-revision-conflict");
        assert_eq!(second_store.load().unwrap().unwrap().revision, 1);
    }

    #[test]
    fn receipt_insert_failure_rolls_back_snapshot_replacement() {
        let database = TemporaryDatabase::new("atomic");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        let initial = snapshot(0, frame("initial"));
        store.initialize(initial.clone()).unwrap();
        store
            .connection
            .execute_batch(
                "CREATE TRIGGER fail_metadata_receipt
                 BEFORE INSERT ON metadata_receipts
                 BEGIN
                   SELECT RAISE(ABORT, 'forced receipt failure');
                 END;",
            )
            .unwrap();
        let plan = plan(
            0,
            'a',
            'b',
            'c',
            frame("must-rollback"),
            "2026-08-07T14:11:00Z",
        );

        let error = store.compare_and_swap(plan.clone()).unwrap_err();
        assert_eq!(error.code, "sqlite");
        assert_eq!(store.load().unwrap(), Some(initial));
        assert!(store.receipt(&plan.plan_digest).unwrap().is_none());
    }

    #[test]
    fn corrupt_stored_state_is_detected_on_load_and_verify() {
        let database = TemporaryDatabase::new("corrupt");
        let mut store = SqliteMetadataStore::open(&database.path).unwrap();
        store.initialize(snapshot(0, frame("initial"))).unwrap();
        store
            .connection
            .execute(
                "UPDATE metadata_snapshot SET state_digest = ?1 WHERE singleton = 1",
                [digest('a')],
            )
            .unwrap();

        assert_eq!(
            store.load().unwrap_err().code,
            "metadata-state-digest-mismatch"
        );
        assert_eq!(
            store.verify().unwrap_err().code,
            "metadata-state-digest-mismatch"
        );
    }

    #[test]
    fn receipt_lookup_rejects_noncanonical_identity() {
        let database = TemporaryDatabase::new("receipt-input");
        let store = SqliteMetadataStore::open(&database.path).unwrap();
        assert_eq!(store.receipt("not-a-digest").unwrap_err().code, "digest-invalid");
    }
}
