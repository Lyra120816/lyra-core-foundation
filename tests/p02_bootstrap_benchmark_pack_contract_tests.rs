use std::fs;

use lyra_phase0::p02::{
    bootstrap_benchmark_artifact_ids, bootstrap_benchmark_artifacts_bind_paths,
    bootstrap_benchmark_carrier_signature, bootstrap_benchmark_evidence_bind_registry,
    bootstrap_benchmark_evidence_ids, bootstrap_benchmark_families_bind_targets,
    bootstrap_benchmark_family_ids, bootstrap_benchmark_no_forbidden_descriptor_claims,
    bootstrap_benchmark_registry_hash, bootstrap_benchmark_target_ids,
    bootstrap_benchmark_targets_bind_receipts, validate_bootstrap_benchmark_pack_surface,
    P02_BOOTSTRAP_BENCHMARK_PACK_CONTRACT, REQUIRED_BOOTSTRAP_BENCHMARK_EVIDENCE,
    REQUIRED_BOOTSTRAP_BENCHMARK_FAMILIES, REQUIRED_BOOTSTRAP_BENCHMARK_PACK_RULES,
    REQUIRED_BOOTSTRAP_BENCHMARK_TARGETS,
};

#[test]
fn bootstrap_benchmark_pack_contract_exports_required_registry() {
    assert_eq!(
        P02_BOOTSTRAP_BENCHMARK_PACK_CONTRACT,
        "LYRA-P02-BOOTSTRAP-BENCHMARK-PACK v1"
    );
    assert_eq!(REQUIRED_BOOTSTRAP_BENCHMARK_FAMILIES.len(), 6);
    assert_eq!(REQUIRED_BOOTSTRAP_BENCHMARK_TARGETS.len(), 12);
    assert_eq!(REQUIRED_BOOTSTRAP_BENCHMARK_EVIDENCE.len(), 6);
    assert!(REQUIRED_BOOTSTRAP_BENCHMARK_PACK_RULES.len() >= 12);
    assert!(bootstrap_benchmark_artifact_ids().len() >= 8);
    assert_eq!(bootstrap_benchmark_family_ids().len(), 6);
    assert_eq!(bootstrap_benchmark_target_ids().len(), 12);
    assert_eq!(bootstrap_benchmark_evidence_ids().len(), 6);
    assert!(bootstrap_benchmark_artifacts_bind_paths());
    assert!(bootstrap_benchmark_families_bind_targets());
    assert!(bootstrap_benchmark_targets_bind_receipts());
    assert!(bootstrap_benchmark_evidence_bind_registry());
    assert!(bootstrap_benchmark_no_forbidden_descriptor_claims());
    assert!(bootstrap_benchmark_registry_hash().starts_with("fnv1a128:"));
    assert!(bootstrap_benchmark_carrier_signature().starts_with("fnv1a128:"));
}

#[test]
fn bootstrap_benchmark_pack_golden_matches_validator_receipt() {
    let input = fs::read_to_string(
        "fixtures/p02/bootstrap_benchmark_pack_inputs/valid_bootstrap_benchmark_pack.lyra",
    )
    .expect("fixture exists");
    let (verdict, receipt) = validate_bootstrap_benchmark_pack_surface(&input);
    assert!(verdict.accepted);
    let golden = fs::read_to_string("goldens/p02/valid_bootstrap_benchmark_pack.receipt")
        .expect("golden exists");
    assert_eq!(receipt.to_text(), golden);
}
