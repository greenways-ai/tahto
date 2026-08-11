use tahto_metadata_store_abi::{
    validate_digest, validate_timestamp, CommitPlan, CommitReceipt, CommitStatus, Snapshot,
};

fn digest(fill: char) -> String {
    format!("sha256:{}", fill.to_string().repeat(64))
}

#[test]
fn downstream_provider_can_construct_and_validate_the_closed_contract() {
    let snapshot = Snapshot::new(0, b"HTA0state".to_vec(), digest('a')).unwrap();
    assert_eq!(snapshot.revision, 0);

    let plan = CommitPlan::new(
        0,
        1,
        digest('b'),
        digest('c'),
        digest('d'),
        b"HTA0next".to_vec(),
        digest('e'),
        "2026-08-07T13:00:00Z",
    )
    .unwrap();
    let receipt = CommitReceipt::from_plan(&plan, CommitStatus::Applied);
    receipt.validate().unwrap();

    validate_digest(&receipt.plan_digest, "plan").unwrap();
    validate_timestamp(&receipt.completed_at).unwrap();
}
