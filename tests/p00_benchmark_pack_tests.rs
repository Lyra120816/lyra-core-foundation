use lyra_phase0::p00::{
    parse_benchmark_pack_surface, validate_benchmark_pack_surface, ErrorCode,
    REQUIRED_BENCHMARK_EVIDENCE, REQUIRED_BENCHMARK_FAMILIES, REQUIRED_BENCHMARK_PACK_TARGETS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p00/benchmark_pack_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_benchmark_pack_surface() {
    let input = fixture("valid_benchmark_pack.lyra");
    let parsed = parse_benchmark_pack_surface(&input).expect("valid benchmark pack parses");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-X03");
    assert_eq!(parsed.families.len(), REQUIRED_BENCHMARK_FAMILIES.len());
    assert_eq!(parsed.targets.len(), REQUIRED_BENCHMARK_PACK_TARGETS.len());
    assert_eq!(parsed.evidence.len(), REQUIRED_BENCHMARK_EVIDENCE.len());
    let (verdict, receipt) = validate_benchmark_pack_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_benchmark_pack_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_family.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_missing_target.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_missing_evidence.lyra",
            ErrorCode::MissingClosureProof,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_benchmark_pack_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_unknown_and_invalid_benchmark_bindings() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_target.lyra",
            ErrorCode::DuplicateClosureOutputGate,
        ),
        (
            "invalid_unknown_family_reference.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_unknown_target_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_unknown_evidence_target.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_threshold.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_benchmark_pack_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_docs_only_unreceipted_global_and_drift_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::ClosureNetworkDependency,
        ),
        (
            "invalid_docs_only_benchmark_pack.lyra",
            ErrorCode::ClosureDocsOnly,
        ),
        (
            "invalid_unreceipted_benchmark_pack.lyra",
            ErrorCode::ClosureUnreceipted,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_drift_accepted.lyra",
            ErrorCode::ClosureDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_benchmark_pack_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
