use lyra_phase0::p00::{
    parse_deployment_surface, validate_deployment_surface, ErrorCode, REQUIRED_COMPLIANCE_HOOKS,
    REQUIRED_DEPLOYMENT_PROOFS, REQUIRED_DEPLOYMENT_TARGETS, REQUIRED_RELEASE_EVIDENCE,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p00/deployment_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_deployment_surface() {
    let input = fixture("valid_deployment_hooks.lyra");
    let parsed = parse_deployment_surface(&input).expect("valid deployment surface parses");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-020");
    assert_eq!(parsed.targets.len(), REQUIRED_DEPLOYMENT_TARGETS.len());
    assert_eq!(parsed.hooks.len(), REQUIRED_COMPLIANCE_HOOKS.len());
    assert_eq!(parsed.evidence.len(), REQUIRED_RELEASE_EVIDENCE.len());
    assert_eq!(parsed.proofs.len(), REQUIRED_DEPLOYMENT_PROOFS.len());
    let (verdict, receipt) = validate_deployment_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_deployment_surface_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingDeploymentRule,
        ),
        (
            "invalid_missing_target.lyra",
            ErrorCode::MissingDeploymentTarget,
        ),
        (
            "invalid_missing_hook.lyra",
            ErrorCode::MissingComplianceHook,
        ),
        (
            "invalid_missing_evidence.lyra",
            ErrorCode::MissingReleaseEvidence,
        ),
        (
            "invalid_missing_proof.lyra",
            ErrorCode::MissingDeploymentProof,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_deployment_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_target_and_unknown_hook_target() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_target.lyra",
            ErrorCode::DuplicateDeploymentTarget,
        ),
        (
            "invalid_unknown_hook_target.lyra",
            ErrorCode::InvalidComplianceHook,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_deployment_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_unbound_proof_network_dependency_and_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_unbound_proof_reference.lyra",
            ErrorCode::DeploymentProofUnbound,
        ),
        (
            "invalid_network_required.lyra",
            ErrorCode::DeploymentNetworkDependency,
        ),
        (
            "invalid_deployment_drift.lyra",
            ErrorCode::DeploymentDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_deployment_surface(&input);
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
    let (verdict, _) = validate_deployment_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::UnsupportedGlobalClosure));
}
