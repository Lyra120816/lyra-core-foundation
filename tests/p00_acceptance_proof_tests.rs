use lyra_phase0::p00::{validate_acceptance_proof_surface, ErrorCode};

const VALID: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/valid_acceptance_proof.lyra");
const INVALID_MISSING_GOLDEN: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_missing_golden.lyra");
const INVALID_DUPLICATE_GOLDEN: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_duplicate_golden.lyra");
const INVALID_MISSING_CHALLENGE: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_missing_challenge_fixture.lyra");
const INVALID_FIXTURE_ACCEPTED: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_fixture_accepted.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_missing_acceptance_proof.lyra");
const INVALID_UNKNOWN_GOLDEN: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_unknown_golden_binding.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_phase_closure_claim.lyra");
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_missing_receipt.lyra");
const INVALID_MISSING_COMMAND: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_missing_command.lyra");
const INVALID_GOLDEN_PATH: &str =
    include_str!("../fixtures/p00/acceptance_proof_inputs/invalid_golden_path.lyra");

fn assert_rejects_with(input: &str, expected: ErrorCode) {
    let (verdict, receipt) = validate_acceptance_proof_surface(input);
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
fn valid_acceptance_proof_surface_is_accepted() {
    let (verdict, receipt) = validate_acceptance_proof_surface(VALID);
    assert!(
        verdict.accepted,
        "valid acceptance proof rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.verdict.status_token(), "ACCEPTED");
}

#[test]
fn rejects_missing_acceptance_golden() {
    assert_rejects_with(INVALID_MISSING_GOLDEN, ErrorCode::MissingAcceptanceGolden);
}

#[test]
fn rejects_duplicate_acceptance_golden() {
    assert_rejects_with(
        INVALID_DUPLICATE_GOLDEN,
        ErrorCode::DuplicateAcceptanceGolden,
    );
}

#[test]
fn rejects_missing_challenge_fixture() {
    assert_rejects_with(
        INVALID_MISSING_CHALLENGE,
        ErrorCode::MissingChallengeFixture,
    );
}

#[test]
fn rejects_fixture_that_claims_accepted() {
    assert_rejects_with(INVALID_FIXTURE_ACCEPTED, ErrorCode::InvalidChallengeFixture);
}

#[test]
fn rejects_missing_acceptance_proof() {
    assert_rejects_with(INVALID_MISSING_PROOF, ErrorCode::MissingAcceptanceProof);
}

#[test]
fn rejects_unknown_golden_binding() {
    assert_rejects_with(INVALID_UNKNOWN_GOLDEN, ErrorCode::AcceptanceProofUnbound);
}

#[test]
fn rejects_phase_level_working_slice_closure_claim() {
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn rejects_missing_receipt_binding() {
    assert_rejects_with(INVALID_MISSING_RECEIPT, ErrorCode::MissingReceiptProof);
}

#[test]
fn rejects_missing_command_binding() {
    assert_rejects_with(INVALID_MISSING_COMMAND, ErrorCode::MissingCommandRecord);
}

#[test]
fn rejects_invalid_golden_path() {
    assert_rejects_with(INVALID_GOLDEN_PATH, ErrorCode::InvalidAcceptanceGolden);
}
