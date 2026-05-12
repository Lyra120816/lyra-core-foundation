use lyra_phase0::p00::{
    deterministic_falsification_report, validate_falsification_surface, ErrorCode,
};

const VALID: &str =
    include_str!("../fixtures/p00/falsification_inputs/valid_falsification_corpus.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_missing_falsification_rule.lyra");
const INVALID_MISSING_CASE: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_missing_negative_case.lyra");
const INVALID_DUPLICATE_CASE: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_duplicate_negative_case.lyra");
const INVALID_NEGATIVE_ACCEPTED: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_negative_fixture_accepted.lyra");
const INVALID_UNKNOWN_TARGET: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_unknown_target_validator.lyra");
const INVALID_MISSING_HARNESS: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_missing_harness.lyra");
const INVALID_DUPLICATE_ORDER: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_duplicate_harness_order.lyra");
const INVALID_MISSING_ASSERTION: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_missing_assertion.lyra");
const INVALID_WRONG_CODE: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_assertion_wrong_code.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_missing_proof.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_MANUAL_ONLY: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_manual_only_corpus.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p00/falsification_inputs/invalid_phase_closure_claim.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_falsification_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_falsification_corpus_surface() {
    let (verdict, receipt) = validate_falsification_surface(VALID);
    assert!(
        verdict.accepted,
        "valid falsification corpus rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P00-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn deterministic_falsification_report_is_stable_and_sorted() {
    let cases = [
        ("z_case", "fake_closure_claim", "fixture_z"),
        ("a_case", "ambient_time_allowed", "fixture_a"),
    ];
    let left = deterministic_falsification_report("P00-016", &cases);
    let right = deterministic_falsification_report("P00-016", &cases);
    assert_eq!(left, right);
    assert_eq!(left.case_count, 2);
    assert_eq!(left.cases[0].order, "001");
    assert_eq!(left.cases[0].case_id, "a_case");
}

#[test]
fn rejects_missing_falsification_rule() {
    assert_rejects_with(INVALID_MISSING_RULE, ErrorCode::MissingFalsificationRule);
}
#[test]
fn rejects_missing_negative_case() {
    assert_rejects_with(INVALID_MISSING_CASE, ErrorCode::MissingNegativeCase);
}
#[test]
fn rejects_duplicate_negative_case() {
    assert_rejects_with(INVALID_DUPLICATE_CASE, ErrorCode::DuplicateNegativeCase);
}
#[test]
fn rejects_negative_fixture_accepted() {
    assert_rejects_with(
        INVALID_NEGATIVE_ACCEPTED,
        ErrorCode::NegativeFixtureAccepted,
    );
}
#[test]
fn rejects_unknown_target_validator() {
    assert_rejects_with(INVALID_UNKNOWN_TARGET, ErrorCode::InvalidNegativeCase);
}
#[test]
fn rejects_missing_harness() {
    assert_rejects_with(
        INVALID_MISSING_HARNESS,
        ErrorCode::MissingFalsificationHarness,
    );
}
#[test]
fn rejects_duplicate_harness_order() {
    assert_rejects_with(
        INVALID_DUPLICATE_ORDER,
        ErrorCode::InvalidFalsificationHarness,
    );
}
#[test]
fn rejects_missing_assertion() {
    assert_rejects_with(
        INVALID_MISSING_ASSERTION,
        ErrorCode::MissingRejectionAssertion,
    );
}
#[test]
fn rejects_assertion_wrong_code() {
    assert_rejects_with(INVALID_WRONG_CODE, ErrorCode::InvalidRejectionAssertion);
}
#[test]
fn rejects_missing_proof() {
    assert_rejects_with(INVALID_MISSING_PROOF, ErrorCode::MissingFalsificationProof);
}
#[test]
fn rejects_unbound_proof_reference() {
    assert_rejects_with(INVALID_UNBOUND_PROOF, ErrorCode::FalsificationProofUnbound);
}
#[test]
fn rejects_manual_only_corpus() {
    assert_rejects_with(INVALID_MANUAL_ONLY, ErrorCode::CorpusDriftAccepted);
}
#[test]
fn rejects_phase_closure_claim() {
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}
