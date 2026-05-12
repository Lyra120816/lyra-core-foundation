use lyra_phase0::p02::{
    deterministic_bootstrap_falsification_suite_report, validate_bootstrap_falsification_surface,
    ErrorCode,
};

const VALID: &str = include_str!(
    "../fixtures/p02/bootstrap_falsification_inputs/valid_bootstrap_falsification.lyra"
);
const INVALID_MISSING_RULE: &str = include_str!("../fixtures/p02/bootstrap_falsification_inputs/invalid_missing_bootstrap_falsification_rule.lyra");
const INVALID_MISSING_CASE: &str = include_str!("../fixtures/p02/bootstrap_falsification_inputs/invalid_missing_bootstrap_falsification_case.lyra");
const INVALID_DUPLICATE_CASE: &str = include_str!("../fixtures/p02/bootstrap_falsification_inputs/invalid_duplicate_bootstrap_falsification_case.lyra");
const INVALID_TARGET: &str = include_str!(
    "../fixtures/p02/bootstrap_falsification_inputs/invalid_unknown_target_domain.lyra"
);
const INVALID_EXPECTED_ERROR: &str =
    include_str!("../fixtures/p02/bootstrap_falsification_inputs/invalid_expected_error.lyra");
const INVALID_HARNESS_UNBOUND: &str = include_str!(
    "../fixtures/p02/bootstrap_falsification_inputs/invalid_harness_unbound_case.lyra"
);
const INVALID_ASSERTION_ERROR: &str = include_str!(
    "../fixtures/p02/bootstrap_falsification_inputs/invalid_assertion_wrong_error.lyra"
);
const INVALID_ARTIFACT_OWNER: &str =
    include_str!("../fixtures/p02/bootstrap_falsification_inputs/invalid_artifact_owner.lyra");
const INVALID_PROOF_UNBOUND: &str = include_str!(
    "../fixtures/p02/bootstrap_falsification_inputs/invalid_proof_unbound_assertion.lyra"
);
const INVALID_FORBIDDEN: &str = include_str!(
    "../fixtures/p02/bootstrap_falsification_inputs/invalid_forbidden_accept_negative.lyra"
);
const INVALID_DESCRIPTOR_DRIFT: &str =
    include_str!("../fixtures/p02/bootstrap_falsification_inputs/invalid_descriptor_drift.lyra");
const INVALID_AMBIENT_TIME: &str =
    include_str!("../fixtures/p02/bootstrap_falsification_inputs/invalid_ambient_time_text.lyra");
const INVALID_BAD_TASK: &str =
    include_str!("../fixtures/p02/bootstrap_falsification_inputs/invalid_bad_task.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_bootstrap_falsification_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_bootstrap_falsification_surface() {
    let (verdict, receipt) = validate_bootstrap_falsification_surface(VALID);
    assert!(
        verdict.accepted,
        "valid bootstrap falsification surface rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn deterministic_bootstrap_falsification_report_is_stable_and_domain_counted() {
    let cases = vec![
        (
            "z".to_string(),
            "receipt_commit".to_string(),
            "bootstrap_receipt_validator".to_string(),
            "m".to_string(),
            "receipt_hash_mismatch".to_string(),
            "fixtures/p02/bootstrap_falsification_cases/z.lyra".to_string(),
            "artifact_emitted".to_string(),
        ),
        (
            "a".to_string(),
            "bootstrap_trust".to_string(),
            "bootstrap_authority_ingest".to_string(),
            "m".to_string(),
            "ambient_authority".to_string(),
            "fixtures/p02/bootstrap_falsification_cases/a.lyra".to_string(),
            "artifact_emitted".to_string(),
        ),
    ];
    let harnesses = vec![(
        "h".to_string(),
        "runner".to_string(),
        vec!["z".to_string(), "a".to_string()],
        "expected_error_exact".to_string(),
        "receipt_bound_replay".to_string(),
        "artifact_emitted".to_string(),
    )];
    let assertions = vec![(
        "r".to_string(),
        "a".to_string(),
        "ambient_authority".to_string(),
        "surface".to_string(),
        vec!["negative_fixture_accepted".to_string()],
        "artifact_emitted".to_string(),
    )];
    let artifacts = vec![(
        "artifact".to_string(),
        "k0".to_string(),
        "k0/determinism/src/bootstrap_falsification.rs".to_string(),
        "report".to_string(),
        "artifact_emitted".to_string(),
    )];
    let proofs = vec![(
        "proof".to_string(),
        vec!["a".to_string()],
        vec!["h".to_string()],
        vec!["r".to_string()],
        vec!["artifact".to_string()],
        "receipts/p02/pass.receipt".to_string(),
        "artifact_emitted".to_string(),
    )];
    let left = deterministic_bootstrap_falsification_suite_report(
        &cases,
        &harnesses,
        &assertions,
        &artifacts,
        &proofs,
    );
    let right = deterministic_bootstrap_falsification_suite_report(
        &cases,
        &harnesses,
        &assertions,
        &artifacts,
        &proofs,
    );
    assert_eq!(left, right);
    assert_eq!(left.case_count, 2);
    assert_eq!(left.bootstrap_trust_case_count, 1);
    assert_eq!(left.receipt_commit_case_count, 1);
}

#[test]
fn rejects_missing_bootstrap_falsification_rule() {
    assert_rejects_with(INVALID_MISSING_RULE, ErrorCode::MissingFalsificationRule);
}
#[test]
fn rejects_missing_bootstrap_falsification_case() {
    assert_rejects_with(INVALID_MISSING_CASE, ErrorCode::MissingNegativeCase);
}
#[test]
fn rejects_duplicate_bootstrap_falsification_case() {
    assert_rejects_with(INVALID_DUPLICATE_CASE, ErrorCode::DuplicateNegativeCase);
}
#[test]
fn rejects_unknown_target_domain() {
    assert_rejects_with(INVALID_TARGET, ErrorCode::InvalidNegativeCase);
}
#[test]
fn rejects_invalid_expected_error() {
    assert_rejects_with(INVALID_EXPECTED_ERROR, ErrorCode::InvalidNegativeCase);
}
#[test]
fn rejects_harness_unbound_case() {
    assert_rejects_with(
        INVALID_HARNESS_UNBOUND,
        ErrorCode::FalsificationProofUnbound,
    );
}
#[test]
fn rejects_assertion_wrong_error() {
    assert_rejects_with(INVALID_ASSERTION_ERROR, ErrorCode::CorpusDriftAccepted);
}
#[test]
fn rejects_artifact_owner_error() {
    assert_rejects_with(INVALID_ARTIFACT_OWNER, ErrorCode::InvalidOwnerRoot);
}
#[test]
fn rejects_proof_unbound_assertion() {
    assert_rejects_with(INVALID_PROOF_UNBOUND, ErrorCode::FalsificationProofUnbound);
}
#[test]
fn rejects_forbidden_accept_negative_text() {
    assert_rejects_with(INVALID_FORBIDDEN, ErrorCode::NegativeFixtureAccepted);
}
#[test]
fn rejects_descriptor_drift() {
    assert_rejects_with(INVALID_DESCRIPTOR_DRIFT, ErrorCode::CorpusDriftAccepted);
}
#[test]
fn rejects_ambient_time_text() {
    assert_rejects_with(INVALID_AMBIENT_TIME, ErrorCode::AmbientTimeAllowed);
}
#[test]
fn rejects_bad_task() {
    assert_rejects_with(INVALID_BAD_TASK, ErrorCode::InvalidTask);
}
