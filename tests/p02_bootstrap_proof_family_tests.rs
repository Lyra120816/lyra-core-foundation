use std::fs;

use lyra_phase0::p02::{
    deterministic_bootstrap_proof_family_table_report, parse_bootstrap_proof_family_surface,
    validate_bootstrap_proof_family_surface,
};

#[test]
fn valid_bootstrap_proof_family_surface_accepts_and_reports_counts() {
    let input = fs::read_to_string(
        "fixtures/p02/bootstrap_proof_family_inputs/valid_bootstrap_proof_family.lyra",
    )
    .expect("fixture exists");
    let (verdict, receipt) = validate_bootstrap_proof_family_surface(&input);
    assert!(verdict.accepted, "{}", receipt.to_text());
    let parsed = parse_bootstrap_proof_family_surface(&input).expect("surface parses");
    let families = parsed
        .families
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family_kind.clone(),
                item.receipts.clone(),
                item.covers.clone(),
                item.proofs.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let receipts = parsed
        .receipts
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family.clone(),
                item.path.clone(),
                item.covers.clone(),
                item.verdict.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let paths = parsed
        .paths
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family.clone(),
                item.path_kind.clone(),
                item.entry_receipts.clone(),
                item.challenge_receipts.clone(),
                item.rollback_receipts.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let report = deterministic_bootstrap_proof_family_table_report(&families, &receipts, &paths);
    assert_eq!(report.family_count, 5);
    assert_eq!(report.path_count, 5);
    assert_eq!(report.receipt_count, 48);
    assert!(report.happy_path_receipt_count >= 25);
    assert!(report.dependency_path_receipt_count >= 5);
    assert!(report.table_hash.starts_with("fnv1a128:"));
}

#[test]
fn invalid_bootstrap_proof_family_corpus_rejects() {
    let mut rejected = 0usize;
    for entry in
        fs::read_dir("fixtures/p02/bootstrap_proof_family_inputs").expect("fixture dir exists")
    {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.starts_with("invalid_") {
            continue;
        }
        let input = fs::read_to_string(&path).expect("fixture readable");
        let (verdict, _receipt) = validate_bootstrap_proof_family_surface(&input);
        assert!(!verdict.accepted, "invalid fixture accepted: {name}");
        rejected += 1;
    }
    assert_eq!(rejected, 24);
}
