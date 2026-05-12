use lyra_phase0::p01::{
    parse_semantic_closure_surface, validate_semantic_closure_surface, ErrorCode,
    REQUIRED_SEMANTIC_CLOSURE_OUTPUTS, REQUIRED_SEMANTIC_CLOSURE_PROOFS,
    REQUIRED_SEMANTIC_CLOSURE_TASKS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/semantic_closure_inputs/{name}"))
        .expect("semantic closure fixture must exist")
}

#[test]
fn accepts_valid_semantic_closure_gate_surface() {
    let input = fixture("valid_semantic_closure.lyra");
    let parsed =
        parse_semantic_closure_surface(&input).expect("valid semantic closure gate parses");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-024");
    assert_eq!(parsed.bounded_closure, "true");
    assert_eq!(parsed.global_closure, "false");
    assert_eq!(parsed.next_frontier, "P01-X01");
    assert_eq!(parsed.tasks.len(), REQUIRED_SEMANTIC_CLOSURE_TASKS.len());
    assert_eq!(
        parsed.outputs.len(),
        REQUIRED_SEMANTIC_CLOSURE_OUTPUTS.len()
    );
    assert_eq!(parsed.proofs.len(), REQUIRED_SEMANTIC_CLOSURE_PROOFS.len());
    let (verdict, receipt) = validate_semantic_closure_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_closure_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        ("invalid_missing_task.lyra", ErrorCode::MissingClosureTask),
        (
            "invalid_missing_output_gate.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        ("invalid_missing_proof.lyra", ErrorCode::MissingClosureProof),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_closure_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_and_unbound_semantic_closure_bindings() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_task.lyra",
            ErrorCode::DuplicateClosureTask,
        ),
        (
            "invalid_unknown_task_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_unbound_output_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_closure_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_closure_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::ClosureNetworkDependency,
        ),
        ("invalid_docs_only_closure.lyra", ErrorCode::ClosureDocsOnly),
        (
            "invalid_unreceipted_closure.lyra",
            ErrorCode::ClosureUnreceipted,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_output_prematurely_closed.lyra",
            ErrorCode::ClosureOutputPremature,
        ),
        (
            "invalid_closure_drift.lyra",
            ErrorCode::ClosureDriftAccepted,
        ),
        ("invalid_task_status.lyra", ErrorCode::InvalidClosureTask),
        (
            "invalid_wrong_next_frontier.lyra",
            ErrorCode::ClosureFormulaViolation,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_closure_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
