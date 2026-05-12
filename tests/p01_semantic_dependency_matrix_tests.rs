use lyra_phase0::p01::{
    parse_semantic_dependency_matrix_surface, validate_semantic_dependency_matrix_surface,
    ErrorCode, REQUIRED_SEMANTIC_BLOCKERS, REQUIRED_SEMANTIC_DEPENDENCY_NODES,
    REQUIRED_SEMANTIC_PARALLEL_LANES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/semantic_dependency_matrix_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_dependency_matrix_surface() {
    let input = fixture("valid_semantic_dependency_matrix.lyra");
    let parsed = parse_semantic_dependency_matrix_surface(&input)
        .expect("valid semantic dependency matrix parses");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-X01");
    assert_eq!(
        parsed.dependencies.len(),
        REQUIRED_SEMANTIC_DEPENDENCY_NODES.len()
    );
    assert_eq!(parsed.blockers.len(), REQUIRED_SEMANTIC_BLOCKERS.len());
    assert_eq!(parsed.lanes.len(), REQUIRED_SEMANTIC_PARALLEL_LANES.len());
    let (verdict, receipt) = validate_semantic_dependency_matrix_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_dependency_matrix_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_dependency.lyra",
            ErrorCode::MissingClosureTask,
        ),
        (
            "invalid_missing_blocker.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_missing_lane.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_dependency_matrix_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_unknown_and_cyclic_semantic_references() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_dependency.lyra",
            ErrorCode::DuplicateClosureTask,
        ),
        (
            "invalid_unknown_dependency_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_unknown_lane_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        ("invalid_cycle.lyra", ErrorCode::ClosureDriftAccepted),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_dependency_matrix_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_docs_only_unreceipted_global_and_wrong_status() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::ClosureNetworkDependency,
        ),
        ("invalid_docs_only_matrix.lyra", ErrorCode::ClosureDocsOnly),
        (
            "invalid_unreceipted_matrix.lyra",
            ErrorCode::ClosureUnreceipted,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_wrong_status.lyra",
            ErrorCode::UnsupportedClosureStatus,
        ),
        (
            "invalid_premature_output.lyra",
            ErrorCode::ClosureOutputPremature,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_dependency_matrix_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
