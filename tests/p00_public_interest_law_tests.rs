use lyra_phase0::p00::{validate_public_interest_law_surface, ErrorCode};

const VALID: &str =
    include_str!("../fixtures/p00/public_interest_law_inputs/valid_public_interest_law.lyra");
const INVALID_MISSING_RULE: &str = include_str!(
    "../fixtures/p00/public_interest_law_inputs/invalid_missing_public_benefit_rule.lyra"
);
const INVALID_MISSING_SAFEGUARD: &str =
    include_str!("../fixtures/p00/public_interest_law_inputs/invalid_missing_safeguard.lyra");
const INVALID_DUPLICATE_SAFEGUARD: &str =
    include_str!("../fixtures/p00/public_interest_law_inputs/invalid_duplicate_safeguard.lyra");
const INVALID_MISSING_LABOR_RIGHT: &str =
    include_str!("../fixtures/p00/public_interest_law_inputs/invalid_missing_labor_right.lyra");
const INVALID_RETALIATION_ALLOWED: &str =
    include_str!("../fixtures/p00/public_interest_law_inputs/invalid_retaliation_allowed.lyra");
const INVALID_MISSING_DUTY: &str = include_str!(
    "../fixtures/p00/public_interest_law_inputs/invalid_missing_anti_extract_duty.lyra"
);
const INVALID_CAPTURE_ALLOWED: &str =
    include_str!("../fixtures/p00/public_interest_law_inputs/invalid_capture_allowed.lyra");
const INVALID_EXTRACT_DEFAULT: &str =
    include_str!("../fixtures/p00/public_interest_law_inputs/invalid_extract_default_allowed.lyra");
const INVALID_UNKNOWN_STEWARDSHIP: &str = include_str!(
    "../fixtures/p00/public_interest_law_inputs/invalid_unknown_stewardship_binding.lyra"
);
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p00/public_interest_law_inputs/invalid_missing_receipt.lyra");

fn assert_rejects_with(input: &str, expected: ErrorCode) {
    let (verdict, receipt) = validate_public_interest_law_surface(input);
    assert!(
        !verdict.accepted,
        "input unexpectedly accepted with receipt {}",
        receipt.receipt_hash
    );
    assert!(
        verdict.errors.iter().any(|error| error.code == expected),
        "expected {:?}, got {:?}",
        expected,
        verdict.errors
    );
}

#[test]
fn valid_public_interest_law_is_accepted() {
    let (verdict, receipt) = validate_public_interest_law_surface(VALID);
    assert!(
        verdict.accepted,
        "valid public-interest law rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.verdict.status_token(), "ACCEPTED");
}

#[test]
fn rejects_missing_required_public_benefit_rule() {
    assert_rejects_with(INVALID_MISSING_RULE, ErrorCode::MissingPublicInterestRule);
}

#[test]
fn rejects_missing_required_safeguard() {
    assert_rejects_with(INVALID_MISSING_SAFEGUARD, ErrorCode::MissingSafeguard);
}

#[test]
fn rejects_duplicate_safeguard() {
    assert_rejects_with(INVALID_DUPLICATE_SAFEGUARD, ErrorCode::DuplicateSafeguard);
}

#[test]
fn rejects_missing_labor_participation_right() {
    assert_rejects_with(
        INVALID_MISSING_LABOR_RIGHT,
        ErrorCode::MissingParticipationRight,
    );
}

#[test]
fn rejects_retaliation_allowance() {
    assert_rejects_with(INVALID_RETALIATION_ALLOWED, ErrorCode::RetaliationAllowed);
}

#[test]
fn rejects_missing_anti_extractive_duty() {
    assert_rejects_with(INVALID_MISSING_DUTY, ErrorCode::MissingAntiExtractiveDuty);
}

#[test]
fn rejects_capture_allowance() {
    assert_rejects_with(INVALID_CAPTURE_ALLOWED, ErrorCode::CaptureRiskAllowed);
}

#[test]
fn rejects_extractive_default_allowance() {
    assert_rejects_with(INVALID_EXTRACT_DEFAULT, ErrorCode::ExtractiveDefaultAllowed);
}

#[test]
fn rejects_unknown_stewardship_binding() {
    assert_rejects_with(
        INVALID_UNKNOWN_STEWARDSHIP,
        ErrorCode::InvalidStewardshipClaim,
    );
}

#[test]
fn rejects_missing_stewardship_receipt() {
    assert_rejects_with(INVALID_MISSING_RECEIPT, ErrorCode::MissingReceiptProof);
}
