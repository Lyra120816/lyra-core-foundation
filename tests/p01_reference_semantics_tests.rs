use lyra_phase0::p01::{
    canonical_reference_semantics_registry_signature, evaluate_reference_expression,
    parse_reference_semantics_surface, validate_reference_semantics_surface, ErrorCode,
    ReferenceExpression, ReferenceLiteral, REQUIRED_REFERENCE_COMPOSITIONS,
    REQUIRED_REFERENCE_EVAL_SEEDS, REQUIRED_REFERENCE_LITERALS,
    REQUIRED_REFERENCE_SEMANTICS_RECEIPTS, REQUIRED_REFERENCE_SEMANTICS_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/reference_semantics_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_reference_semantics_surface() {
    let input = fixture("valid_reference_semantics.lyra");
    let parsed =
        parse_reference_semantics_surface(&input).expect("valid reference semantics parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-005");
    assert_eq!(parsed.rules.len(), REQUIRED_REFERENCE_SEMANTICS_RULES.len());
    assert_eq!(parsed.literals.len(), REQUIRED_REFERENCE_LITERALS.len());
    assert_eq!(
        parsed.compositions.len(),
        REQUIRED_REFERENCE_COMPOSITIONS.len()
    );
    assert_eq!(parsed.eval_seeds.len(), REQUIRED_REFERENCE_EVAL_SEEDS.len());
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_REFERENCE_SEMANTICS_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_reference_semantics_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn reference_semantics_registry_and_evaluator_are_stable() {
    let signature = canonical_reference_semantics_registry_signature();
    assert!(signature.contains("reference_literal:unit|atom:value"));
    assert!(signature.contains("reference_composition:record|operator:compose.record"));
    assert!(signature.contains(
        "reference_eval_seed:apply_symbolic|input:apply(literal.symbol.core,literal.unit)"
    ));

    let expr = ReferenceExpression::Record(vec![
        (
            "b".to_string(),
            ReferenceExpression::Literal(ReferenceLiteral::Integer(1)),
        ),
        (
            "a".to_string(),
            ReferenceExpression::Literal(ReferenceLiteral::Integer(0)),
        ),
    ]);
    let result = evaluate_reference_expression(&expr).expect("deterministic evaluation");
    assert_eq!(
        result.canonical_input,
        "record(a:literal.integer.0,b:literal.integer.1)"
    );
    assert_eq!(result.canonical_output, "record(a=integer.0,b=integer.1)");
    assert!(result
        .steps
        .contains(&"record_key_order_canonical".to_string()));
    assert!(result.trace_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_reference_semantics_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_literal.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_composition.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_eval_seed.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_reference_semantics_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_reference_semantics_descriptor_trace_and_binding_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_literal.lyra",
            ErrorCode::DuplicateCanonicalModel,
        ),
        (
            "invalid_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_eval_trace_mismatch.lyra",
            ErrorCode::ReceiptHashMismatch,
        ),
        (
            "invalid_unknown_literal.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_receipt_target.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_reference_semantics_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_reference_semantics_truth_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_semantics.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_semantics.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_reference_semantics_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
