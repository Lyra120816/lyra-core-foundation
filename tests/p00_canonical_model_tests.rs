use lyra_phase0::p00::{validate_canonical_model_surface, ErrorCode};

const VALID: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/valid_canonical_model.lyra");
const INVALID_MISSING_MODEL: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_missing_model.lyra");
const INVALID_DUPLICATE_MODEL: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_duplicate_model.lyra");
const INVALID_UNKNOWN_SCHEMA_MODEL: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_unknown_schema_model.lyra");
const INVALID_UNSTABLE_FIELD_ORDER: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_unstable_field_order.lyra");
const INVALID_DUPLICATE_FIELD_ORDER: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_duplicate_field_order.lyra");
const INVALID_MISSING_RECEIPT_BINDING: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_missing_receipt_binding.lyra");
const INVALID_BINDING_UNKNOWN_MODEL: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_binding_unknown_model.lyra");
const INVALID_SCHEMA_REQUIRED_UNLISTED: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_schema_required_unlisted.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_phase_closure_claim.lyra");
const INVALID_MODEL_DRIFT: &str =
    include_str!("../fixtures/p00/canonical_model_inputs/invalid_model_drift.lyra");

fn assert_rejects_with(input: &str, expected: ErrorCode) {
    let (verdict, receipt) = validate_canonical_model_surface(input);
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
fn valid_canonical_model_surface_is_accepted() {
    let (verdict, receipt) = validate_canonical_model_surface(VALID);
    assert!(
        verdict.accepted,
        "valid canonical model rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.verdict.status_token(), "ACCEPTED");
}

#[test]
fn rejects_missing_required_model() {
    assert_rejects_with(INVALID_MISSING_MODEL, ErrorCode::MissingCanonicalModel);
}

#[test]
fn rejects_duplicate_model_identity() {
    assert_rejects_with(INVALID_DUPLICATE_MODEL, ErrorCode::DuplicateCanonicalModel);
}

#[test]
fn rejects_unknown_schema_model_reference() {
    assert_rejects_with(
        INVALID_UNKNOWN_SCHEMA_MODEL,
        ErrorCode::CanonicalModelUnbound,
    );
}

#[test]
fn rejects_unstable_field_order() {
    assert_rejects_with(INVALID_UNSTABLE_FIELD_ORDER, ErrorCode::InvalidFieldBinding);
}

#[test]
fn rejects_duplicate_field_order_inside_model() {
    assert_rejects_with(
        INVALID_DUPLICATE_FIELD_ORDER,
        ErrorCode::InvalidFieldBinding,
    );
}

#[test]
fn rejects_missing_receipt_binding() {
    assert_rejects_with(
        INVALID_MISSING_RECEIPT_BINDING,
        ErrorCode::MissingReceiptProof,
    );
}

#[test]
fn rejects_binding_to_unknown_model() {
    assert_rejects_with(
        INVALID_BINDING_UNKNOWN_MODEL,
        ErrorCode::CanonicalModelUnbound,
    );
}

#[test]
fn rejects_required_field_missing_from_schema_field_list() {
    assert_rejects_with(
        INVALID_SCHEMA_REQUIRED_UNLISTED,
        ErrorCode::InvalidSchemaBinding,
    );
}

#[test]
fn rejects_phase_closure_claim() {
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn rejects_canonical_model_drift() {
    assert_rejects_with(INVALID_MODEL_DRIFT, ErrorCode::CanonicalModelDriftAccepted);
}
