use lyra_phase0::p01::{
    deterministic_semantic_redteam_rollback_report, semantic_redteam_artifacts_bind_paths,
    semantic_redteam_no_forbidden_descriptor_claims, semantic_redteam_proofs_bind_registry,
    semantic_redteam_receipts_cover_p01_001_through_p01_023,
    semantic_redteam_rollbacks_bind_proofs, semantic_redteam_scenarios_bind_rollbacks,
    validate_semantic_redteam_surface, ErrorCode, REQUIRED_SEMANTIC_REDTEAM_PROOFS,
    REQUIRED_SEMANTIC_REDTEAM_SCENARIOS, REQUIRED_SEMANTIC_ROLLBACK_PATHS,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/valid_semantic_redteam.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_missing_rule.lyra");
const INVALID_MISSING_SCENARIO: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_missing_scenario.lyra");
const INVALID_DUPLICATE_SCENARIO: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_duplicate_scenario.lyra");
const INVALID_MISSING_ROLLBACK: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_missing_rollback.lyra");
const INVALID_UNKNOWN_SCENARIO: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_unknown_scenario_reference.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_missing_proof.lyra");
const INVALID_NETWORK_REQUIRED: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_network_required.lyra");
const INVALID_UNRECEIPTED_ROLLBACK: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_unreceipted_rollback.lyra");
const INVALID_CHALLENGE_BYPASS: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_challenge_bypass.lyra");
const INVALID_REMOTE_TRUTH_REWRITE: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_remote_truth_rewrite.lyra");
const INVALID_REDTEAM_DRIFT: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_redteam_drift.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_phase_closure_claim.lyra");
const INVALID_UNKNOWN_COMMAND: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_unknown_command.lyra");
const INVALID_BAD_AUTHORITY: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_bad_authority.lyra");
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/invalid_missing_receipt_binding.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_semantic_redteam_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_semantic_redteam_surface() {
    let (verdict, receipt) = validate_semantic_redteam_surface(VALID);
    assert!(
        verdict.accepted,
        "valid semantic redteam rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn semantic_redteam_deterministic_report_is_stable_and_counted() {
    let scenarios = vec![
        (
            "z_scenario".to_string(),
            "core_ir".to_string(),
            "fixtures/p01/z.lyra".to_string(),
            vec!["core_ir".to_string()],
            vec!["lyra-p01-semantic-redteam-check".to_string()],
            vec!["z_reject".to_string()],
            vec!["receipts/p01/z.receipt".to_string()],
            "artifact_emitted".to_string(),
        ),
        (
            "a_scenario".to_string(),
            "semantic_atom".to_string(),
            "fixtures/p01/a.lyra".to_string(),
            vec!["semantic_atoms".to_string()],
            vec!["lyra-p01-semantic-redteam-check".to_string()],
            vec!["a_reject".to_string()],
            vec!["receipts/p01/a.receipt".to_string()],
            "artifact_emitted".to_string(),
        ),
    ];
    let rollbacks = vec![(
        "z_rollback".to_string(),
        "core_ir".to_string(),
        "products/p01/z.lyra".to_string(),
        "receipt_chain".to_string(),
        vec!["z_scenario".to_string()],
        vec!["z_proof".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["lyra-p01-semantic-redteam-check".to_string()],
        "artifact_emitted".to_string(),
    )];
    let proofs = vec![(
        "z_proof".to_string(),
        "redteam".to_string(),
        vec!["z_scenario".to_string()],
        vec!["z_rollback".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["lyra-p01-semantic-redteam-check".to_string()],
        vec![
            "phase_closure".to_string(),
            "unreceipted_rollback".to_string(),
            "remote_truth_rewrite".to_string(),
            "challenge_bypass".to_string(),
        ],
        "artifact_emitted".to_string(),
    )];
    let report = deterministic_semantic_redteam_rollback_report(&scenarios, &rollbacks, &proofs);
    assert_eq!(report.scenario_count, 2);
    assert_eq!(report.rollback_count, 1);
    assert_eq!(report.proof_count, 1);
    assert_eq!(report.scenario_reports[0].id, "a_scenario");
    assert!(report.suite_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_redteam_gaps() {
    for (input, expected) in [
        (INVALID_MISSING_RULE, ErrorCode::MissingRedTeamRule),
        (INVALID_MISSING_SCENARIO, ErrorCode::MissingRedTeamScenario),
        (INVALID_MISSING_ROLLBACK, ErrorCode::MissingRollbackPath),
        (INVALID_MISSING_PROOF, ErrorCode::MissingRedTeamProof),
    ] {
        assert_rejects_with(input, expected);
    }
}

#[test]
fn rejects_duplicate_unbound_and_invalid_semantic_redteam_bindings() {
    assert_rejects_with(
        INVALID_DUPLICATE_SCENARIO,
        ErrorCode::DuplicateRedTeamScenario,
    );
    assert_rejects_with(INVALID_UNKNOWN_SCENARIO, ErrorCode::RedTeamProofUnbound);
    assert_rejects_with(INVALID_UNBOUND_PROOF, ErrorCode::RedTeamProofUnbound);
    assert_rejects_with(INVALID_UNKNOWN_COMMAND, ErrorCode::InvalidRedTeamScenario);
    assert_rejects_with(INVALID_BAD_AUTHORITY, ErrorCode::InvalidRollbackPath);
    assert_rejects_with(INVALID_MISSING_RECEIPT, ErrorCode::InvalidRedTeamScenario);
}

#[test]
fn rejects_network_unreceipted_challenge_remote_drift_and_closure_claims() {
    assert_rejects_with(
        INVALID_NETWORK_REQUIRED,
        ErrorCode::RedTeamNetworkDependency,
    );
    assert_rejects_with(
        INVALID_UNRECEIPTED_ROLLBACK,
        ErrorCode::RedTeamRollbackUnreceipted,
    );
    assert_rejects_with(INVALID_CHALLENGE_BYPASS, ErrorCode::RedTeamChallengeBypass);
    assert_rejects_with(
        INVALID_REMOTE_TRUTH_REWRITE,
        ErrorCode::RemoteTruthRewriteAllowed,
    );
    assert_rejects_with(INVALID_REDTEAM_DRIFT, ErrorCode::RedTeamDriftAccepted);
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn semantic_redteam_descriptor_registry_is_bound() {
    assert!(semantic_redteam_scenarios_bind_rollbacks());
    assert!(semantic_redteam_rollbacks_bind_proofs());
    assert!(semantic_redteam_proofs_bind_registry());
    assert!(semantic_redteam_artifacts_bind_paths());
    assert!(semantic_redteam_no_forbidden_descriptor_claims());
    assert!(semantic_redteam_receipts_cover_p01_001_through_p01_023());
}

#[test]
fn required_semantic_redteam_inventory_counts_are_bound() {
    assert_eq!(REQUIRED_SEMANTIC_REDTEAM_SCENARIOS.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_ROLLBACK_PATHS.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_REDTEAM_PROOFS.len(), 6);
}
