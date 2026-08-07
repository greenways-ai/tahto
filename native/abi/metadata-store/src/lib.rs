//! Dependency-free native contract for durable Tahto metadata snapshots.
//!
//! The ABI carries canonical HTA state bytes and closed transaction evidence.
//! It does not parse application payloads, execute Hara transitions, verify
//! signatures, or grant installation and key authority.

use std::fmt;

pub const ABI_ID: &str = "tahto/metadata-store";
pub const ABI_VERSION: &str = "1.0.0";
pub const TRANSPORT: &str = "hta.v1";
pub const NATIVE_ABI: &str = "tahto-metadata-store/1";

pub const MAX_IDENTIFIER_BYTES: usize = 128;
pub const MAX_CANONICAL_STATE_BYTES: usize = 256 * 1024 * 1024;
pub const MAX_REVISION: u64 = i64::MAX as u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub revision: u64,
    pub state: Vec<u8>,
    pub state_digest: String,
}

impl Snapshot {
    pub fn new(
        revision: u64,
        state: Vec<u8>,
        state_digest: impl Into<String>,
    ) -> Result<Self, Error> {
        let snapshot = Self {
            revision,
            state,
            state_digest: state_digest.into(),
        };
        snapshot.validate()?;
        Ok(snapshot)
    }

    pub fn validate(&self) -> Result<(), Error> {
        validate_stored_revision(self.revision)?;
        validate_hta(&self.state)?;
        validate_digest(&self.state_digest, "state digest")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitPlan {
    pub expected_revision: u64,
    pub revision: u64,
    pub plan_digest: String,
    pub request_digest: String,
    pub result_digest: String,
    pub state: Vec<u8>,
    pub state_digest: String,
    pub completed_at: String,
}

impl CommitPlan {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        expected_revision: u64,
        revision: u64,
        plan_digest: impl Into<String>,
        request_digest: impl Into<String>,
        result_digest: impl Into<String>,
        state: Vec<u8>,
        state_digest: impl Into<String>,
        completed_at: impl Into<String>,
    ) -> Result<Self, Error> {
        let plan = Self {
            expected_revision,
            revision,
            plan_digest: plan_digest.into(),
            request_digest: request_digest.into(),
            result_digest: result_digest.into(),
            state,
            state_digest: state_digest.into(),
            completed_at: completed_at.into(),
        };
        plan.validate()?;
        Ok(plan)
    }

    pub fn validate(&self) -> Result<(), Error> {
        validate_revision_step(self.expected_revision, self.revision)?;
        validate_hta(&self.state)?;
        validate_digest(&self.plan_digest, "plan digest")?;
        validate_digest(&self.request_digest, "request digest")?;
        validate_digest(&self.result_digest, "result digest")?;
        validate_digest(&self.state_digest, "state digest")?;
        validate_timestamp(&self.completed_at)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitStatus {
    Applied,
    Replayed,
}

impl CommitStatus {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::Replayed => "replayed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitReceipt {
    pub status: CommitStatus,
    pub revision: u64,
    pub plan_digest: String,
    pub request_digest: String,
    pub result_digest: String,
    pub state_digest: String,
    pub completed_at: String,
}

impl CommitReceipt {
    pub fn from_plan(plan: &CommitPlan, status: CommitStatus) -> Self {
        Self {
            status,
            revision: plan.revision,
            plan_digest: plan.plan_digest.clone(),
            request_digest: plan.request_digest.clone(),
            result_digest: plan.result_digest.clone(),
            state_digest: plan.state_digest.clone(),
            completed_at: plan.completed_at.clone(),
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.revision == 0 || self.revision > MAX_REVISION {
            return Err(Error::new(
                "receipt-revision-invalid",
                "commit receipt revision must be between 1 and i64::MAX",
            ));
        }
        validate_digest(&self.plan_digest, "plan digest")?;
        validate_digest(&self.request_digest, "request digest")?;
        validate_digest(&self.result_digest, "result digest")?;
        validate_digest(&self.state_digest, "state digest")?;
        validate_timestamp(&self.completed_at)
    }

    pub fn matches_plan(&self, plan: &CommitPlan) -> bool {
        self.revision == plan.revision
            && self.plan_digest == plan.plan_digest
            && self.request_digest == plan.request_digest
            && self.result_digest == plan.result_digest
            && self.state_digest == plan.state_digest
            && self.completed_at == plan.completed_at
    }
}

pub trait Adapter {
    fn load(&self) -> Result<Option<Snapshot>, Error>;

    fn initialize(&mut self, snapshot: Snapshot) -> Result<Snapshot, Error>;

    fn compare_and_swap(&mut self, plan: CommitPlan) -> Result<CommitReceipt, Error>;

    fn receipt(&self, plan_digest: &str) -> Result<Option<CommitReceipt>, Error>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub code: String,
    pub detail: String,
}

impl Error {
    pub fn new(code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            detail: detail.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.detail)
    }
}

impl std::error::Error for Error {}

pub fn validate_identifier(value: &str, label: &'static str) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || value.chars().any(char::is_whitespace)
    {
        Err(Error::new(
            "identifier-invalid",
            format!("{label} must be 1-{MAX_IDENTIFIER_BYTES} non-whitespace UTF-8 bytes"),
        ))
    } else {
        Ok(())
    }
}

pub fn validate_digest(value: &str, label: &'static str) -> Result<(), Error> {
    let bytes = value.as_bytes();
    let valid = bytes.len() == 71
        && bytes.starts_with(b"sha256:")
        && bytes[7..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte));
    if valid {
        Ok(())
    } else {
        Err(Error::new(
            "digest-invalid",
            format!("{label} must be sha256: followed by 64 lowercase hex digits"),
        ))
    }
}

pub fn validate_timestamp(value: &str) -> Result<(), Error> {
    let bytes = value.as_bytes();
    let digit_positions = [0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18];
    let fixed = bytes.len() >= 20
        && bytes.len() <= 35
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[bytes.len() - 1] == b'Z'
        && digit_positions
            .iter()
            .all(|index| bytes[*index].is_ascii_digit());
    let fractional = bytes.len() == 20
        || (bytes.len() > 21
            && bytes[19] == b'.'
            && bytes[20..bytes.len() - 1]
                .iter()
                .all(|byte| byte.is_ascii_digit()));
    if fixed && fractional {
        Ok(())
    } else {
        Err(Error::new(
            "timestamp-invalid",
            "timestamp must be a bounded UTC RFC3339 value",
        ))
    }
}

pub fn validate_hta(state: &[u8]) -> Result<(), Error> {
    if state.len() < 4 || state.len() > MAX_CANONICAL_STATE_BYTES || !state.starts_with(b"HTA1") {
        Err(Error::new(
            "state-not-canonical-hta",
            format!("state must be an HTA1 frame no larger than {MAX_CANONICAL_STATE_BYTES} bytes"),
        ))
    } else {
        Ok(())
    }
}

pub fn validate_stored_revision(revision: u64) -> Result<(), Error> {
    if revision <= MAX_REVISION {
        Ok(())
    } else {
        Err(Error::new(
            "revision-invalid",
            "revision must fit in a signed 64-bit storage column",
        ))
    }
}

pub fn validate_revision_step(expected_revision: u64, revision: u64) -> Result<(), Error> {
    validate_stored_revision(expected_revision)?;
    validate_stored_revision(revision)?;
    if expected_revision < MAX_REVISION && revision == expected_revision + 1 {
        Ok(())
    } else {
        Err(Error::new(
            "revision-step-invalid",
            "next revision must equal expected revision plus one",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(fill: char) -> String {
        format!("sha256:{}", fill.to_string().repeat(64))
    }

    fn plan() -> CommitPlan {
        CommitPlan::new(
            0,
            1,
            digest('a'),
            digest('b'),
            digest('c'),
            b"HTA1state".to_vec(),
            digest('d'),
            "2026-08-07T12:00:00Z",
        )
        .unwrap()
    }

    #[test]
    fn snapshot_accepts_only_bounded_canonical_hta_evidence() {
        let snapshot = Snapshot::new(0, b"HTA1state".to_vec(), digest('a')).unwrap();
        assert_eq!(snapshot.revision, 0);
        assert_eq!(snapshot.state_digest, digest('a'));

        assert_eq!(
            Snapshot::new(0, b"json".to_vec(), digest('a'))
                .unwrap_err()
                .code,
            "state-not-canonical-hta"
        );
        assert_eq!(
            Snapshot::new(0, b"HTA1state".to_vec(), "sha256:ABC")
                .unwrap_err()
                .code,
            "digest-invalid"
        );
    }

    #[test]
    fn commit_plan_requires_one_exact_revision_step() {
        assert_eq!(plan().revision, 1);
        assert_eq!(
            CommitPlan::new(
                0,
                2,
                digest('a'),
                digest('b'),
                digest('c'),
                b"HTA1state".to_vec(),
                digest('d'),
                "2026-08-07T12:00:00Z",
            )
            .unwrap_err()
            .code,
            "revision-step-invalid"
        );
    }

    #[test]
    fn commit_plan_rejects_unverified_evidence_shapes() {
        assert_eq!(
            CommitPlan::new(
                0,
                1,
                digest('a'),
                digest('b'),
                digest('c'),
                b"HTA1state".to_vec(),
                digest('d'),
                "2026-08-07 12:00:00",
            )
            .unwrap_err()
            .code,
            "timestamp-invalid"
        );
        assert_eq!(
            validate_digest(&format!("sha256:{}", "A".repeat(64)), "digest")
                .unwrap_err()
                .code,
            "digest-invalid"
        );
    }

    #[test]
    fn public_values_must_be_revalidated_after_mutation() {
        let mut snapshot = Snapshot::new(0, b"HTA1state".to_vec(), digest('a')).unwrap();
        snapshot.state = b"json".to_vec();
        assert_eq!(
            snapshot.validate().unwrap_err().code,
            "state-not-canonical-hta"
        );

        let mut plan = plan();
        plan.revision = 2;
        assert_eq!(plan.validate().unwrap_err().code, "revision-step-invalid");
    }

    #[test]
    fn receipt_preserves_the_exact_plan_evidence() {
        let plan = plan();
        let receipt = CommitReceipt::from_plan(&plan, CommitStatus::Applied);
        receipt.validate().unwrap();
        assert!(receipt.matches_plan(&plan));
        assert_eq!(receipt.status.name(), "applied");
        assert_eq!(receipt.plan_digest, plan.plan_digest);
        assert_eq!(receipt.request_digest, plan.request_digest);
        assert_eq!(receipt.result_digest, plan.result_digest);
        assert_eq!(receipt.state_digest, plan.state_digest);
    }

    #[test]
    fn revision_and_identifier_bounds_fail_closed() {
        assert!(validate_stored_revision(MAX_REVISION).is_ok());
        assert_eq!(
            validate_stored_revision(MAX_REVISION + 1).unwrap_err().code,
            "revision-invalid"
        );
        assert!(validate_identifier("provider.sqlite", "provider").is_ok());
        assert_eq!(
            validate_identifier("provider sqlite", "provider")
                .unwrap_err()
                .code,
            "identifier-invalid"
        );
    }

    #[test]
    fn fractional_utc_timestamps_are_bounded() {
        assert!(validate_timestamp("2026-08-07T12:00:00.123456Z").is_ok());
        assert_eq!(
            validate_timestamp("2026-08-07T12:00:00.Z")
                .unwrap_err()
                .code,
            "timestamp-invalid"
        );
    }
}
