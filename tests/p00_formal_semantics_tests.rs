use lyra_phase0::p00::{validate_formal_semantics_surface, ErrorCode};

const VALID: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/valid_formal_semantics.lyra");
const INVALID_MISSING_DOMAIN: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_missing_semantic_domain.lyra");
const INVALID_DUPLICATE_DOMAIN: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_duplicate_semantic_domain.lyra");
const INVALID_OWNER_ROOT: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_owner_root_domain.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_missing_semantic_rule.lyra");
const INVALID_UNKNOWN_RULE_DOMAIN: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_unknown_rule_domain.lyra");
const INVALID_TRANSITION_GUARD: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_transition_guard.lyra");
const INVALID_MISSING_TRANSITION: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_missing_transition.lyra");
const INVALID_MISSING_INVARIANT: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_missing_invariant.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_unbound_proof_rule.lyra");
const INVALID_MISSING_PROOF_RECEIPT: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_missing_proof_receipt.lyra");
const INVALID_SEMANTIC_DRIFT: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_semantic_drift.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p00/formal_semantics_inputs/invalid_phase_closure_claim.lyra");

fn assert_rejects_with(input: &str, expected: ErrorCode) {
    let (verdict, receipt) = validate_formal_semantics_surface(input);
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
fn valid_formal_semantics_surface_is_accepted() {
    let (verdict, receipt) = validate_formal_semantics_surface(VALID);
    assert!(
        verdict.accepted,
        "valid formal semantics rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.verdict.status_token(), "ACCEPTED");
}

#[test]
fn rejects_missing_semantic_domain() {
    assert_rejects_with(INVALID_MISSING_DOMAIN, ErrorCode::MissingSemanticDomain);
}

#[test]
fn rejects_duplicate_semantic_domain() {
    assert_rejects_with(INVALID_DUPLICATE_DOMAIN, ErrorCode::DuplicateSemanticDomain);
}

#[test]
fn rejects_invalid_domain_owner_root() {
    assert_rejects_with(INVALID_OWNER_ROOT, ErrorCode::InvalidSemanticDomain);
}

#[test]
fn rejects_missing_semantic_rule_binding() {
    assert_rejects_with(INVALID_MISSING_RULE, ErrorCode::MissingSemanticRuleBinding);
}

#[test]
fn rejects_unknown_semantic_rule_domain() {
    assert_rejects_with(INVALID_UNKNOWN_RULE_DOMAIN, ErrorCode::SemanticProofUnbound);
}

#[test]
fn rejects_invalid_transition_guard() {
    assert_rejects_with(INVALID_TRANSITION_GUARD, ErrorCode::InvalidTransitionLaw);
}

#[test]
fn rejects_missing_transition_law() {
    assert_rejects_with(INVALID_MISSING_TRANSITION, ErrorCode::MissingTransitionLaw);
}

#[test]
fn rejects_missing_invariant_binding() {
    assert_rejects_with(
        INVALID_MISSING_INVARIANT,
        ErrorCode::MissingInvariantBinding,
    );
}

#[test]
fn rejects_unbound_proof_rule() {
    assert_rejects_with(INVALID_UNBOUND_PROOF, ErrorCode::SemanticProofUnbound);
}

#[test]
fn rejects_missing_proof_receipt() {
    assert_rejects_with(
        INVALID_MISSING_PROOF_RECEIPT,
        ErrorCode::MissingReceiptProof,
    );
}

#[test]
fn rejects_semantic_drift() {
    assert_rejects_with(INVALID_SEMANTIC_DRIFT, ErrorCode::SemanticDriftAccepted);
}

#[test]
fn rejects_phase_closure_claim() {
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}
