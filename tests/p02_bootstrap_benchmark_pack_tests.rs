use std::fs;

use lyra_phase0::p02::{
    deterministic_bootstrap_benchmark_pack_report, parse_bootstrap_benchmark_pack_surface,
    validate_bootstrap_benchmark_pack_surface,
};

#[test]
fn valid_bootstrap_benchmark_pack_accepts_and_reports_counts() {
    let input = fs::read_to_string(
        "fixtures/p02/bootstrap_benchmark_pack_inputs/valid_bootstrap_benchmark_pack.lyra",
    )
    .expect("fixture exists");
    let (verdict, receipt) = validate_bootstrap_benchmark_pack_surface(&input);
    assert!(verdict.accepted, "{}", receipt.to_text());
    let parsed = parse_bootstrap_benchmark_pack_surface(&input).expect("surface parses");
    let families = parsed
        .families
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family_kind.clone(),
                item.targets.clone(),
                item.proofs.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let targets = parsed
        .targets
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family.clone(),
                item.metric.clone(),
                item.unit.clone(),
                item.threshold.clone(),
                item.command.clone(),
                item.fixture.clone(),
                item.golden.clone(),
                item.receipt.clone(),
                item.expected.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let evidence = parsed
        .evidence
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family.clone(),
                item.targets.clone(),
                item.artifacts.clone(),
                item.proof_receipts.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let report = deterministic_bootstrap_benchmark_pack_report(&families, &targets, &evidence);
    assert_eq!(report.family_count, 6);
    assert_eq!(report.target_count, 12);
    assert_eq!(report.evidence_count, 6);
    assert_eq!(report.throughput_target_count, 2);
    assert_eq!(report.latency_target_count, 2);
    assert_eq!(report.correctness_target_count, 2);
    assert_eq!(report.stability_target_count, 2);
    assert_eq!(report.adversarial_target_count, 2);
    assert_eq!(report.rollback_target_count, 2);
    assert!(report.pack_hash.starts_with("fnv1a128:"));
}

#[test]
fn invalid_bootstrap_benchmark_pack_corpus_rejects() {
    let mut rejected = 0usize;
    for entry in
        fs::read_dir("fixtures/p02/bootstrap_benchmark_pack_inputs").expect("fixture dir exists")
    {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.starts_with("invalid_") {
            continue;
        }
        let input = fs::read_to_string(&path).expect("fixture readable");
        let (verdict, _receipt) = validate_bootstrap_benchmark_pack_surface(&input);
        assert!(!verdict.accepted, "invalid fixture accepted: {name}");
        rejected += 1;
    }
    assert_eq!(rejected, 26);
}
