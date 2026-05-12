use lyra_phase0::{
    k0_verdict::ErrorCode,
    p02_bootstrap_retirement_law::{
        parse_bootstrap_retirement_supersession_surface,
        validate_bootstrap_retirement_supersession_surface, REQUIRED_BOOTSTRAP_RETIREMENT_GATES,
        REQUIRED_BOOTSTRAP_RETIREMENT_RECEIPTS, REQUIRED_BOOTSTRAP_RETIREMENT_SURFACES,
        REQUIRED_BOOTSTRAP_SUPERSESSIONS,
    },
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p02/bootstrap_retirement_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_bootstrap_retirement_surface() {
    let input = fixture("valid_bootstrap_retirement_supersession.lyra");
    let parsed = parse_bootstrap_retirement_supersession_surface(&input).expect("valid retirement parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-X05");
    assert_eq!(parsed.global_closure, "denied");
    assert_eq!(parsed.next_frontier, "P03");
    assert_eq!(parsed.surfaces.len(), REQUIRED_BOOTSTRAP_RETIREMENT_SURFACES.len());
    assert_eq!(parsed.gates.len(), REQUIRED_BOOTSTRAP_RETIREMENT_GATES.len());
    assert_eq!(parsed.supersessions.len(), REQUIRED_BOOTSTRAP_SUPERSESSIONS.len());
    assert_eq!(parsed.receipts.len(), REQUIRED_BOOTSTRAP_RETIREMENT_RECEIPTS.len());
    let (verdict, receipt) = validate_bootstrap_retirement_supersession_surface(&input);
    assert!(verdict.accepted, "expected acceptance, got {:?}", verdict.errors);
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_bootstrap_retirement_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        ("invalid_missing_surface.lyra", ErrorCode::MissingClosureOutputGate),
        ("invalid_missing_gate.lyra", ErrorCode::MissingClosureOutputGate),
        ("invalid_missing_supersession.lyra", ErrorCode::MissingClosureOutputGate),
        ("invalid_missing_receipt.lyra", ErrorCode::MissingClosureProof),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_bootstrap_retirement_supersession_surface(&input);
        assert!(verdict.errors.iter().any(|error| error.code == expected), "{fixture_name} should reject with {expected:?}: {:?}", verdict.errors);
    }
}

#[test]
fn rejects_forbidden_and_invalid_claims() {
    for (fixture_name, expected) in [
        ("invalid_network_required.lyra", ErrorCode::ClosureNetworkDependency),
        ("invalid_docs_only_bootstrap_retirement.lyra", ErrorCode::ClosureDocsOnly),
        ("invalid_unreceipted_bootstrap_retirement.lyra", ErrorCode::ClosureUnreceipted),
        ("invalid_global_closure_claim.lyra", ErrorCode::UnsupportedGlobalClosure),
        ("invalid_drift_accepted.lyra", ErrorCode::ClosureDriftAccepted),
        ("invalid_ambient_time.lyra", ErrorCode::AmbientTimeAllowed),
        ("invalid_bad_next_frontier.lyra", ErrorCode::ClosureOutputPremature),
        ("invalid_bad_surface_owner.lyra", ErrorCode::InvalidOwnerRoot),
        ("invalid_bad_gate_trigger.lyra", ErrorCode::InvalidClosureOutputGate),
        ("invalid_bad_supersession_archive.lyra", ErrorCode::InvalidClosureOutputGate),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_bootstrap_retirement_supersession_surface(&input);
        assert!(verdict.errors.iter().any(|error| error.code == expected), "{fixture_name} should reject with {expected:?}: {:?}", verdict.errors);
    }
}
