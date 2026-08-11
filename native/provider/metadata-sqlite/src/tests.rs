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
    format!("HTA0{value}").into_bytes()
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
    assert_eq!(NATIVE_ABI, "tahto-metadata-store/0-alpha");
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
    let first = plan(0, 'a', 'b', 'c', frame("first"), "2026-08-07T14:01:00Z");
    store.compare_and_swap(first).unwrap();
    let installed = store.load().unwrap().unwrap();

    let stale = plan(0, 'd', 'e', 'f', frame("stale"), "2026-08-07T14:02:00Z");
    let error = store.compare_and_swap(stale.clone()).unwrap_err();
    assert_eq!(error.code, "metadata-revision-receipt-conflict");
    assert_eq!(store.load().unwrap(), Some(installed));
    assert!(store.receipt(&stale.plan_digest).unwrap().is_none());
}

#[test]
fn state_digest_mismatch_rolls_back_before_the_transaction() {
    let database = TemporaryDatabase::new("digest");
    let mut store = SqliteMetadataStore::open(&database.path).unwrap();
    let initial = snapshot(0, frame("initial"));
    store.initialize(initial.clone()).unwrap();
    let mut plan = plan(0, 'a', 'b', 'c', frame("correct"), "2026-08-07T14:03:00Z");
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
    let first = plan(0, 'a', 'b', 'c', frame("first"), "2026-08-07T14:04:00Z");
    let second = plan(1, 'd', 'e', 'f', frame("second"), "2026-08-07T14:05:00Z");
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
    let first = plan(0, 'a', 'b', 'c', frame("first"), "2026-08-07T14:06:00Z");
    store.compare_and_swap(first.clone()).unwrap();

    let conflicting_state = frame("conflicting");
    let conflicting = CommitPlan::new(
        1,
        2,
        first.plan_digest.clone(),
        digest('d'),
        digest('e'),
        conflicting_state.clone(),
        canonical_state_digest(&conflicting_state),
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
    let mut invalid = plan(0, 'a', 'b', 'c', frame("invalid"), "2026-08-07T14:08:00Z");
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
    assert_eq!(error.code, "metadata-revision-receipt-conflict");
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
fn unsupported_schema_versions_are_rejected_without_rewriting_them() {
    let database = TemporaryDatabase::new("version");
    {
        let connection = Connection::open(&database.path).unwrap();
        connection.pragma_update(None, "user_version", 7).unwrap();
    }

    let error = SqliteMetadataStore::open(&database.path).err().unwrap();
    assert_eq!(error.code, "sqlite-schema-version");
    let connection = Connection::open(&database.path).unwrap();
    let version: i64 = connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .unwrap();
    assert_eq!(version, 7);
}

#[test]
fn receipt_lookup_rejects_noncanonical_identity() {
    let database = TemporaryDatabase::new("receipt-input");
    let store = SqliteMetadataStore::open(&database.path).unwrap();
    assert_eq!(
        store.receipt("not-a-digest").unwrap_err().code,
        "digest-invalid"
    );
}
