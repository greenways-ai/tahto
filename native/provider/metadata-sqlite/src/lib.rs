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

const CONNECTION_PRAGMAS: &str = "
PRAGMA journal_mode=WAL;
PRAGMA synchronous=FULL;
PRAGMA foreign_keys=ON;
";

const INITIAL_SCHEMA: &str = "
CREATE TABLE metadata_snapshot (
  singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
  revision INTEGER NOT NULL CHECK (revision >= 0),
  state BLOB NOT NULL,
  state_digest TEXT NOT NULL
) STRICT;
CREATE TABLE metadata_receipts (
  plan_digest TEXT PRIMARY KEY,
  revision INTEGER NOT NULL UNIQUE CHECK (revision > 0),
  request_digest TEXT NOT NULL,
  result_digest TEXT NOT NULL,
  state_digest TEXT NOT NULL,
  completed_at TEXT NOT NULL
) STRICT;
CREATE INDEX metadata_receipts_request
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

        let version = user_version_on(&connection)?;
        if version != 0 && version != SCHEMA_VERSION {
            return Err(Error::new(
                "sqlite-schema-version",
                format!("expected schema {SCHEMA_VERSION}, found {version}"),
            ));
        }

        connection
            .execute_batch(CONNECTION_PRAGMAS)
            .map_err(database_error)?;
        if version == 0 {
            connection
                .execute_batch(INITIAL_SCHEMA)
                .map_err(database_error)?;
        }

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

        let version = user_version_on(&self.connection)?;
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
                params![
                    to_i64(snapshot.revision)?,
                    &snapshot.state,
                    &snapshot.state_digest
                ],
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
                    &plan.state,
                    &plan.state_digest,
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
                    &receipt.plan_digest,
                    to_i64(receipt.revision)?,
                    &receipt.request_digest,
                    &receipt.result_digest,
                    &receipt.state_digest,
                    &receipt.completed_at
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

fn user_version_on(connection: &Connection) -> Result<i64, Error> {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(database_error)
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
        let snapshot = Snapshot::new(revision as u64, state, state_digest)
            .map_err(|error| Error::new("metadata-snapshot-corrupt", error.to_string()))?;
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
mod tests;
