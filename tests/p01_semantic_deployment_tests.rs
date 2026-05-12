use lyra_phase0::p01::{
    deterministic_semantic_deployment_suite_report, validate_semantic_deployment_surface,
    ErrorCode, REQUIRED_SEMANTIC_COMPLIANCE_HOOKS, REQUIRED_SEMANTIC_DEPLOYMENT_PROOFS,
    REQUIRED_SEMANTIC_DEPLOYMENT_TARGETS, REQUIRED_SEMANTIC_RELEASE_EVIDENCE,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/valid_semantic_deployment.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_missing_rule.lyra");
const INVALID_MISSING_TARGET: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_missing_target.lyra");
const INVALID_DUPLICATE_TARGET: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_duplicate_target.lyra");
const INVALID_UNKNOWN_HOOK_TARGET: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_unknown_hook_target.lyra");
const INVALID_MISSING_HOOK: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_missing_hook.lyra");
const INVALID_MISSING_EVIDENCE: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_missing_evidence.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_missing_proof.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_NETWORK_REQUIRED: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_network_required.lyra");
const INVALID_DEPLOYMENT_DRIFT: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_deployment_drift.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_phase_closure_claim.lyra");
const INVALID_RECEIPT: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_missing_receipt_binding.lyra");
const INVALID_REMOTE_SERVICE_TARGET: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/invalid_remote_service_target.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_semantic_deployment_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_semantic_deployment_surface() {
    let (verdict, receipt) = validate_semantic_deployment_surface(VALID);
    assert!(
        verdict.accepted,
        "valid semantic deployment rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn semantic_deployment_deterministic_report_is_stable_and_counted() {
    let targets = vec![
        (
            "z_target".to_string(),
            "archive".to_string(),
            "airgap".to_string(),
            vec!["receipts/p01/z.receipt".to_string()],
            vec!["lyra-p01-semantic-deployment-check".to_string()],
            vec!["receipts/p01/z.receipt".to_string()],
            vec!["network_required".to_string()],
            "artifact_emitted".to_string(),
        ),
        (
            "a_target".to_string(),
            "workstation".to_string(),
            "offline".to_string(),
            vec!["src/bin/a.rs".to_string()],
            vec!["lyra-p01-semantic-deployment-check".to_string()],
            vec!["receipts/p01/a.receipt".to_string()],
            vec!["network_required".to_string()],
            "artifact_emitted".to_string(),
        ),
    ];
    let hooks = vec![(
        "hook_z".to_string(),
        "target".to_string(),
        "z_target".to_string(),
        vec!["artifact".to_string()],
        vec!["evidence_z".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        "artifact_emitted".to_string(),
    )];
    let evidence = vec![(
        "evidence_z".to_string(),
        "receipt".to_string(),
        "receipts/p01/z.receipt".to_string(),
        vec!["z_target".to_string()],
        vec!["hook_z".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["lyra-p01-semantic-deployment-check".to_string()],
        "artifact_emitted".to_string(),
    )];
    let proofs = vec![(
        "proof_z".to_string(),
        "target".to_string(),
        vec!["z_target".to_string()],
        vec!["hook_z".to_string()],
        vec!["evidence_z".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["lyra-p01-semantic-deployment-check".to_string()],
        vec!["phase_closure".to_string()],
        "blocked".to_string(),
    )];
    let report =
        deterministic_semantic_deployment_suite_report(&targets, &hooks, &evidence, &proofs);
    assert_eq!(report.target_count, 2);
    assert_eq!(report.hook_count, 1);
    assert_eq!(report.evidence_count, 1);
    assert_eq!(report.proof_count, 1);
    assert_eq!(report.targets[0].id, "a_target");
    assert!(report.suite_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_deployment_gaps() {
    for (input, expected) in [
        (INVALID_MISSING_RULE, ErrorCode::MissingDeploymentRule),
        (INVALID_MISSING_TARGET, ErrorCode::MissingDeploymentTarget),
        (INVALID_MISSING_HOOK, ErrorCode::MissingComplianceHook),
        (INVALID_MISSING_EVIDENCE, ErrorCode::MissingReleaseEvidence),
        (INVALID_MISSING_PROOF, ErrorCode::MissingDeploymentProof),
    ] {
        assert_rejects_with(input, expected);
    }
}

#[test]
fn rejects_duplicate_and_invalid_target_bindings() {
    assert_rejects_with(
        INVALID_DUPLICATE_TARGET,
        ErrorCode::DuplicateDeploymentTarget,
    );
    assert_rejects_with(INVALID_RECEIPT, ErrorCode::InvalidDeploymentTarget);
    assert_rejects_with(
        INVALID_REMOTE_SERVICE_TARGET,
        ErrorCode::DeploymentNetworkDependency,
    );
}

#[test]
fn rejects_hook_and_proof_unbound_references() {
    assert_rejects_with(
        INVALID_UNKNOWN_HOOK_TARGET,
        ErrorCode::InvalidComplianceHook,
    );
    assert_rejects_with(INVALID_UNBOUND_PROOF, ErrorCode::DeploymentProofUnbound);
}

#[test]
fn rejects_network_deployment_drift_and_closure_claims() {
    assert_rejects_with(
        INVALID_NETWORK_REQUIRED,
        ErrorCode::DeploymentNetworkDependency,
    );
    assert_rejects_with(INVALID_DEPLOYMENT_DRIFT, ErrorCode::DeploymentDriftAccepted);
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn required_semantic_deployment_inventory_counts_are_bound() {
    assert_eq!(REQUIRED_SEMANTIC_DEPLOYMENT_TARGETS.len(), 4);
    assert_eq!(REQUIRED_SEMANTIC_COMPLIANCE_HOOKS.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_RELEASE_EVIDENCE.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_DEPLOYMENT_PROOFS.len(), 6);
}
