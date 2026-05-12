use lyra_phase0::p01::{
    deterministic_semantic_benchmark_pack_report, parse_semantic_benchmark_pack_surface,
    validate_semantic_benchmark_pack_surface, ErrorCode, REQUIRED_SEMANTIC_BENCHMARK_EVIDENCE,
    REQUIRED_SEMANTIC_BENCHMARK_FAMILIES, REQUIRED_SEMANTIC_BENCHMARK_TARGETS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/semantic_benchmark_pack_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_benchmark_pack_surface() {
    let input = fixture("valid_semantic_benchmark_pack.lyra");
    let parsed = parse_semantic_benchmark_pack_surface(&input)
        .expect("valid semantic benchmark pack parses");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-X03");
    assert_eq!(
        parsed.families.len(),
        REQUIRED_SEMANTIC_BENCHMARK_FAMILIES.len()
    );
    assert_eq!(
        parsed.targets.len(),
        REQUIRED_SEMANTIC_BENCHMARK_TARGETS.len()
    );
    assert_eq!(
        parsed.evidence.len(),
        REQUIRED_SEMANTIC_BENCHMARK_EVIDENCE.len()
    );
    let (verdict, receipt) = validate_semantic_benchmark_pack_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn deterministic_report_counts_all_benchmark_families() {
    let input = fixture("valid_semantic_benchmark_pack.lyra");
    let parsed = parse_semantic_benchmark_pack_surface(&input)
        .expect("valid semantic benchmark pack parses");
    let families = parsed
        .families
        .iter()
        .map(|family| {
            (
                family.id.clone(),
                family.family_kind.clone(),
                family.targets.clone(),
                family.proofs.clone(),
                family.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let targets = parsed
        .targets
        .iter()
        .map(|target| {
            (
                target.id.clone(),
                target.family.clone(),
                target.metric.clone(),
                target.unit.clone(),
                target.threshold.clone(),
                target.command.clone(),
                target.fixture.clone(),
                target.golden.clone(),
                target.receipt.clone(),
                target.status.clone(),
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
    let report = deterministic_semantic_benchmark_pack_report(&families, &targets, &evidence);
    assert_eq!(report.family_count, 4);
    assert_eq!(report.target_count, 8);
    assert_eq!(report.evidence_count, 4);
    assert_eq!(report.throughput_target_count, 2);
    assert_eq!(report.latency_target_count, 2);
    assert_eq!(report.correctness_target_count, 2);
    assert_eq!(report.stability_target_count, 2);
    assert!(report.pack_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_benchmark_gaps() {
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
        let (verdict, _) = validate_semantic_benchmark_pack_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_unknown_and_invalid_semantic_benchmark_bindings() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_target.lyra",
            ErrorCode::DuplicateClosureOutputGate,
        ),
        (
            "invalid_unknown_target_family.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_unknown_family_target_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_wrong_command.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_bad_threshold.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_benchmark_pack_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_docs_only_unreceipted_global_drift_and_wrong_status() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::ClosureNetworkDependency,
        ),
        ("invalid_docs_only.lyra", ErrorCode::ClosureDocsOnly),
        ("invalid_unreceipted.lyra", ErrorCode::ClosureUnreceipted),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_benchmark_drift.lyra",
            ErrorCode::ClosureDriftAccepted,
        ),
        (
            "invalid_wrong_status.lyra",
            ErrorCode::UnsupportedClosureStatus,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_benchmark_pack_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
