use lyra_phase0::p02::{
    bootstrap_redteam_artifacts_bind_paths, bootstrap_redteam_no_forbidden_descriptor_claims,
    bootstrap_redteam_proofs_bind_registry,
    bootstrap_redteam_receipts_cover_p02_001_through_p02_023,
    bootstrap_redteam_rollbacks_bind_challenge_rights, bootstrap_redteam_scenarios_bind_rollbacks,
    deterministic_bootstrap_redteam_suite_report, validate_bootstrap_redteam_surface, ErrorCode,
    REQUIRED_BOOTSTRAP_REDTEAM_PROOFS, REQUIRED_BOOTSTRAP_REDTEAM_SCENARIOS,
    REQUIRED_BOOTSTRAP_ROLLBACK_PATHS,
};

fn bootstrap_redteam_fixture(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/p02/bootstrap_redteam_inputs")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("bootstrap redteam fixture missing/unreadable: {name}"))
}

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_bootstrap_redteam_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_bootstrap_redteam_surface() {
    let valid = bootstrap_redteam_fixture("valid_bootstrap_redteam.lyra");
    let (verdict, receipt) = validate_bootstrap_redteam_surface(&valid);
    assert!(
        verdict.accepted,
        "valid bootstrap redteam rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn bootstrap_redteam_deterministic_report_is_stable_and_counted() {
    let scenarios = vec![
        (
            "z_scenario".to_string(),
            "host_dependency".to_string(),
            "examples/p02/z.lyra".to_string(),
            vec!["bootstrap_trust".to_string()],
            vec!["z_rollback".to_string()],
            vec!["lyra-p02-bootstrap-redteam-check".to_string()],
            vec!["receipts/p02/z.receipt".to_string()],
            vec!["remote_truth_rewrite".to_string()],
            "artifact_emitted".to_string(),
        ),
        (
            "a_scenario".to_string(),
            "receipt_replay".to_string(),
            "examples/p02/a.lyra".to_string(),
            vec!["receipt_replay".to_string()],
            vec!["z_rollback".to_string()],
            vec!["lyra-p02-bootstrap-redteam-check".to_string()],
            vec!["receipts/p02/a.receipt".to_string()],
            vec!["challenge_bypass".to_string()],
            "artifact_emitted".to_string(),
        ),
    ];
    let rollbacks = vec![(
        "z_rollback".to_string(),
        "quarantine".to_string(),
        "ops/p02/z.lyra".to_string(),
        vec!["trigger".to_string()],
        vec!["bootstrap_trust".to_string()],
        vec!["receipts/p02/z.receipt".to_string()],
        vec!["lyra-p02-bootstrap-redteam-check".to_string()],
        vec!["operator_challenge".to_string()],
        "artifact_emitted".to_string(),
    )];
    let proofs = vec![(
        "z_proof".to_string(),
        "redteam".to_string(),
        vec!["z_scenario".to_string()],
        vec!["z_rollback".to_string()],
        vec!["receipts/p02/z.receipt".to_string()],
        vec!["lyra-p02-bootstrap-redteam-check".to_string()],
        vec![
            "remote_truth_rewrite".to_string(),
            "phase_closure".to_string(),
        ],
        "artifact_emitted".to_string(),
    )];
    let report = deterministic_bootstrap_redteam_suite_report(&scenarios, &rollbacks, &proofs);
    assert_eq!(report.scenario_count, 2);
    assert_eq!(report.rollback_count, 1);
    assert_eq!(report.proof_count, 1);
    assert_eq!(report.scenario_reports[0].id, "a_scenario");
    assert!(report.suite_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_bootstrap_redteam_gaps() {
    for (fixture_file, expected) in [
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
        let input = bootstrap_redteam_fixture(fixture_file);
        assert_rejects_with(&input, expected);
    }
}

#[test]
fn rejects_duplicate_redteam_scenario_fixture() {
    let duplicate = bootstrap_redteam_fixture("invalid_duplicate_scenario.lyra");
    assert_rejects_with(&duplicate, ErrorCode::DuplicateRedTeamScenario);
}

#[test]
fn rejects_unknown_rollback_fixture() {
    let unknown = bootstrap_redteam_fixture("invalid_unknown_rollback_reference.lyra");
    assert_rejects_with(&unknown, ErrorCode::InvalidRedTeamScenario);
}

#[test]
fn rejects_unbound_proof_fixture() {
    let fixture = bootstrap_redteam_fixture("invalid_unbound_proof_reference.lyra");
    assert_rejects_with(&fixture, ErrorCode::RedTeamProofUnbound);
}

#[test]
fn rejects_missing_receipt_binding_fixture() {
    let fixture = bootstrap_redteam_fixture("invalid_missing_receipt_binding.lyra");
    assert_rejects_with(&fixture, ErrorCode::InvalidRedTeamScenario);
}

#[test]
fn rejects_unknown_command_fixture() {
    let fixture = bootstrap_redteam_fixture("invalid_unknown_command.lyra");
    assert_rejects_with(&fixture, ErrorCode::InvalidRedTeamScenario);
}

#[test]
fn rejects_missing_coverage_anchor_fixture() {
    let fixture = bootstrap_redteam_fixture("invalid_missing_coverage_anchor.lyra");
    assert_rejects_with(&fixture, ErrorCode::InvalidRedTeamScenario);
}

#[test]
fn rejects_descriptor_drift_fixture() {
    let fixture = bootstrap_redteam_fixture("invalid_descriptor_drift.lyra");
    assert_rejects_with(&fixture, ErrorCode::RedTeamDriftAccepted);
}

#[test]
fn rejects_network_rollback_challenge_remote_drift_docs_and_closure_claims() {
    let network = bootstrap_redteam_fixture("invalid_network_required.lyra");
    assert_rejects_with(&network, ErrorCode::RedTeamNetworkDependency);
    let rollback_unreceipted = bootstrap_redteam_fixture("invalid_rollback_unreceipted.lyra");
    assert_rejects_with(&rollback_unreceipted, ErrorCode::RedTeamRollbackUnreceipted);
    let challenge_bypass = bootstrap_redteam_fixture("invalid_challenge_bypass.lyra");
    assert_rejects_with(&challenge_bypass, ErrorCode::RedTeamChallengeBypass);
    let remote_rewrite = bootstrap_redteam_fixture("invalid_remote_truth_rewrite_allowed.lyra");
    assert_rejects_with(&remote_rewrite, ErrorCode::RemoteTruthRewriteAllowed);
    let redteam_drift = bootstrap_redteam_fixture("invalid_redteam_drift.lyra");
    assert_rejects_with(&redteam_drift, ErrorCode::RedTeamDriftAccepted);
    let corpus_drift = bootstrap_redteam_fixture("invalid_corpus_drift.lyra");
    assert_rejects_with(&corpus_drift, ErrorCode::CorpusDriftAccepted);
    let docs_only = bootstrap_redteam_fixture("invalid_docs_only_claim.lyra");
    assert_rejects_with(&docs_only, ErrorCode::DocsOnlyImplementation);
    let phase_closure = bootstrap_redteam_fixture("invalid_phase_closure_claim.lyra");
    assert_rejects_with(&phase_closure, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn bootstrap_redteam_descriptor_registry_is_bound() {
    assert!(bootstrap_redteam_scenarios_bind_rollbacks());
    assert!(bootstrap_redteam_rollbacks_bind_challenge_rights());
    assert!(bootstrap_redteam_proofs_bind_registry());
    assert!(bootstrap_redteam_artifacts_bind_paths());
    assert!(bootstrap_redteam_no_forbidden_descriptor_claims());
    assert!(bootstrap_redteam_receipts_cover_p02_001_through_p02_023());
}

#[test]
fn required_bootstrap_redteam_inventory_counts_are_bound() {
    assert_eq!(REQUIRED_BOOTSTRAP_REDTEAM_SCENARIOS.len(), 7);
    assert_eq!(REQUIRED_BOOTSTRAP_ROLLBACK_PATHS.len(), 7);
    assert_eq!(REQUIRED_BOOTSTRAP_REDTEAM_PROOFS.len(), 6);
}
