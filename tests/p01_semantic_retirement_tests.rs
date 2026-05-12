use lyra_phase0::p01::{
    parse_semantic_retirement_supersession_surface,
    validate_semantic_retirement_supersession_surface, ErrorCode,
    REQUIRED_SEMANTIC_RETIREMENT_GATES, REQUIRED_SEMANTIC_RETIREMENT_RECEIPTS,
    REQUIRED_SEMANTIC_RETIREMENT_SURFACES, REQUIRED_SEMANTIC_SUPERSESSIONS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/semantic_retirement_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_retirement_supersession_surface() {
    let input = fixture("valid_semantic_retirement_supersession.lyra");
    let parsed = parse_semantic_retirement_supersession_surface(&input)
        .expect("valid semantic retirement law parses");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-X05");
    assert_eq!(
        parsed.surfaces.len(),
        REQUIRED_SEMANTIC_RETIREMENT_SURFACES.len()
    );
    assert_eq!(parsed.gates.len(), REQUIRED_SEMANTIC_RETIREMENT_GATES.len());
    assert_eq!(
        parsed.supersessions.len(),
        REQUIRED_SEMANTIC_SUPERSESSIONS.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SEMANTIC_RETIREMENT_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_semantic_retirement_supersession_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_retirement_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_surface.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_missing_gate.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_missing_supersession.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_retirement_supersession_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_unknown_and_invalid_semantic_retirement_rows() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_surface.lyra",
            ErrorCode::DuplicateClosureOutputGate,
        ),
        (
            "invalid_unknown_gate_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_unknown_receipt_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_invalid_owner_root.lyra",
            ErrorCode::InvalidOwnerRoot,
        ),
        (
            "invalid_missing_bootstrap_surface.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_retirement_supersession_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_docs_only_unreceipted_global_drift_and_ambient_time_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::ClosureNetworkDependency,
        ),
        (
            "invalid_docs_only_retirement.lyra",
            ErrorCode::ClosureDocsOnly,
        ),
        (
            "invalid_unreceipted_retirement.lyra",
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
        (
            "invalid_ambient_time_gate.lyra",
            ErrorCode::AmbientTimeAllowed,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_retirement_supersession_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
