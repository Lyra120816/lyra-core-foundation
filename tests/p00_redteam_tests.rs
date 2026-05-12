use lyra_phase0::p00::{
    parse_redteam_rollback_surface, validate_redteam_rollback_surface, ErrorCode,
    REQUIRED_REDTEAM_PROOFS, REQUIRED_REDTEAM_SCENARIOS, REQUIRED_ROLLBACK_PATHS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p00/redteam_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_redteam_rollback_surface() {
    let input = fixture("valid_redteam_rollback.lyra");
    let parsed =
        parse_redteam_rollback_surface(&input).expect("valid red-team rollback surface parses");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-023");
    assert_eq!(parsed.scenarios.len(), REQUIRED_REDTEAM_SCENARIOS.len());
    assert_eq!(parsed.rollbacks.len(), REQUIRED_ROLLBACK_PATHS.len());
    assert_eq!(parsed.proofs.len(), REQUIRED_REDTEAM_PROOFS.len());
    let (verdict, receipt) = validate_redteam_rollback_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_redteam_surface_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingRedTeamRule),
        (
            "invalid_missing_scenario.lyra",
            ErrorCode::MissingRedTeamScenario,
        ),
        (
            "invalid_missing_rollback.lyra",
            ErrorCode::MissingRollbackPath,
        ),
        ("invalid_missing_proof.lyra", ErrorCode::MissingRedTeamProof),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_redteam_rollback_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_and_unknown_bindings() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_scenario.lyra",
            ErrorCode::DuplicateRedTeamScenario,
        ),
        (
            "invalid_unknown_scenario_reference.lyra",
            ErrorCode::RedTeamProofUnbound,
        ),
        (
            "invalid_unbound_proof_reference.lyra",
            ErrorCode::RedTeamProofUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_redteam_rollback_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_unreceipted_bypass_remote_and_drift_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::RedTeamNetworkDependency,
        ),
        (
            "invalid_unreceipted_rollback.lyra",
            ErrorCode::RedTeamRollbackUnreceipted,
        ),
        (
            "invalid_challenge_bypass.lyra",
            ErrorCode::RedTeamChallengeBypass,
        ),
        (
            "invalid_remote_truth_rewrite.lyra",
            ErrorCode::RemoteTruthRewriteAllowed,
        ),
        (
            "invalid_redteam_drift.lyra",
            ErrorCode::RedTeamDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_redteam_rollback_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_phase_closure_claim() {
    let input = fixture("invalid_phase_closure_claim.lyra");
    let (verdict, _) = validate_redteam_rollback_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::UnsupportedGlobalClosure));
}
