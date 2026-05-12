use lyra_phase0::p00::{deterministic_engine_trace, validate_engine_surface, ErrorCode};

const VALID: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/valid_deterministic_engine.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_missing_engine_rule.lyra");
const INVALID_MISSING_UNIT: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_missing_engine_unit.lyra");
const INVALID_DUPLICATE_UNIT: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_duplicate_engine_unit.lyra");
const INVALID_OWNER_ROOT: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_engine_owner_root.lyra");
const INVALID_AMBIENT_TIME: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_ambient_time.lyra");
const INVALID_MISSING_TRANSITION: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_missing_transition.lyra");
const INVALID_UNKNOWN_ENGINE: &str = include_str!(
    "../fixtures/p00/deterministic_engine_inputs/invalid_transition_unknown_engine.lyra"
);
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_missing_receipt.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_phase_closure_claim.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_missing_proof.lyra");
const INVALID_ENGINE_DRIFT: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/invalid_engine_drift.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_engine_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_deterministic_engine_surface() {
    let (verdict, receipt) = validate_engine_surface(VALID);
    assert!(
        verdict.accepted,
        "valid engine surface rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P00-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn deterministic_engine_trace_is_stable_and_ordered() {
    let left = deterministic_engine_trace("P00-015", VALID).expect("trace should build");
    let right = deterministic_engine_trace("P00-015", VALID).expect("trace should replay");
    assert_eq!(left, right);
    assert_eq!(left.steps.len(), 3);
    assert_eq!(left.steps[0].order, "001");
    assert_eq!(left.steps[1].order, "002");
    assert_eq!(left.steps[2].order, "003");
}

#[test]
fn rejects_missing_engine_rule() {
    assert_rejects_with(INVALID_MISSING_RULE, ErrorCode::MissingEngineRule);
}

#[test]
fn rejects_missing_engine_unit() {
    assert_rejects_with(INVALID_MISSING_UNIT, ErrorCode::MissingEngineUnit);
}

#[test]
fn rejects_duplicate_engine_unit() {
    assert_rejects_with(INVALID_DUPLICATE_UNIT, ErrorCode::DuplicateEngineUnit);
}

#[test]
fn rejects_invalid_engine_owner_root() {
    assert_rejects_with(INVALID_OWNER_ROOT, ErrorCode::InvalidEngineUnit);
}

#[test]
fn rejects_ambient_time() {
    assert_rejects_with(INVALID_AMBIENT_TIME, ErrorCode::AmbientTimeAllowed);
}

#[test]
fn rejects_missing_transition() {
    assert_rejects_with(
        INVALID_MISSING_TRANSITION,
        ErrorCode::MissingTransitionBinding,
    );
}

#[test]
fn rejects_unknown_transition_engine() {
    assert_rejects_with(INVALID_UNKNOWN_ENGINE, ErrorCode::EngineProofUnbound);
}

#[test]
fn rejects_missing_receipt_binding() {
    assert_rejects_with(INVALID_MISSING_RECEIPT, ErrorCode::MissingReceiptProof);
}

#[test]
fn rejects_phase_closure_claim() {
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn rejects_missing_engine_proof() {
    assert_rejects_with(INVALID_MISSING_PROOF, ErrorCode::MissingEngineProof);
}

#[test]
fn rejects_engine_drift() {
    assert_rejects_with(INVALID_ENGINE_DRIFT, ErrorCode::EngineDriftAccepted);
}
