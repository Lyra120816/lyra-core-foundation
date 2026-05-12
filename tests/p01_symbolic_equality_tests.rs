use lyra_phase0::p01::{
    canonical_symbolic_equality_registry_signature, canonical_symbolic_term,
    evaluate_substitution_case, normalize_symbolic_term, parse_symbolic_equality_surface,
    substitute_symbolic_term, symbolic_terms_equal, validate_symbolic_equality_surface, ErrorCode,
    SymbolicEqualityError, SymbolicTerm, REQUIRED_SYMBOLIC_EQUALITY_LAWS,
    REQUIRED_SYMBOLIC_EQUALITY_RECEIPTS, REQUIRED_SYMBOLIC_EQUALITY_RULES,
    REQUIRED_SYMBOLIC_EQUIVALENCE_CLASSES, REQUIRED_SYMBOLIC_NORMALIZATIONS,
    REQUIRED_SYMBOLIC_SUBSTITUTIONS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/symbolic_equality_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_symbolic_equality_surface() {
    let input = fixture("valid_symbolic_equality.lyra");
    let parsed = parse_symbolic_equality_surface(&input).expect("valid symbolic equality parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-006");
    assert_eq!(parsed.rules.len(), REQUIRED_SYMBOLIC_EQUALITY_RULES.len());
    assert_eq!(
        parsed.equality_rules.len(),
        REQUIRED_SYMBOLIC_EQUALITY_LAWS.len()
    );
    assert_eq!(
        parsed.equivalence_classes.len(),
        REQUIRED_SYMBOLIC_EQUIVALENCE_CLASSES.len()
    );
    assert_eq!(
        parsed.normalizations.len(),
        REQUIRED_SYMBOLIC_NORMALIZATIONS.len()
    );
    assert_eq!(
        parsed.substitutions.len(),
        REQUIRED_SYMBOLIC_SUBSTITUTIONS.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SYMBOLIC_EQUALITY_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_symbolic_equality_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn symbolic_equality_registry_and_normalizer_are_stable() {
    let signature = canonical_symbolic_equality_registry_signature();
    assert!(signature.contains("equality_rule:reflexive|domain:term"));
    assert!(signature.contains("equivalence_class:record_order_class"));
    assert!(signature.contains("normalization:record_key_sort_normal"));
    assert!(signature.contains("substitution:capture_rejection"));

    let unsorted = SymbolicTerm::Record(vec![
        ("b".to_string(), SymbolicTerm::Integer(1)),
        ("a".to_string(), SymbolicTerm::Integer(0)),
    ]);
    let sorted = SymbolicTerm::Record(vec![
        ("a".to_string(), SymbolicTerm::Integer(0)),
        ("b".to_string(), SymbolicTerm::Integer(1)),
    ]);
    assert_eq!(
        canonical_symbolic_term(&unsorted).unwrap(),
        "record(a=integer.0,b=integer.1)"
    );
    assert_eq!(
        normalize_symbolic_term(&unsorted).unwrap(),
        normalize_symbolic_term(&sorted).unwrap()
    );
    let witness = symbolic_terms_equal(&unsorted, &sorted).expect("equality witness");
    assert!(witness.equal);
    assert!(witness.witness_hash.starts_with("fnv1a128:"));
}

#[test]
fn substitution_is_capture_avoiding_and_normalized() {
    assert_eq!(
        evaluate_substitution_case("substitute_record").unwrap(),
        "record(a=unit,b=integer.0)"
    );
    assert_eq!(
        evaluate_substitution_case("binder_shadow_guard").unwrap(),
        "bind($0=unit in symbol.$0)"
    );
    assert_eq!(
        evaluate_substitution_case("capture_rejection").unwrap(),
        "reject_capture_risk"
    );

    let scope = SymbolicTerm::Bind {
        symbol: "y".to_string(),
        value: Box::new(SymbolicTerm::Unit),
        body: Box::new(SymbolicTerm::Symbol("x".to_string())),
    };
    let replacement = SymbolicTerm::Symbol("y".to_string());
    let result = substitute_symbolic_term(&scope, "x", &replacement);
    assert!(matches!(
        result,
        Err(SymbolicEqualityError::CaptureRisk { .. })
    ));
}

#[test]
fn rejects_required_symbolic_equality_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_equality_law.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_equivalence_class.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_normalization.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_substitution.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_symbolic_equality_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_symbolic_equality_descriptor_digest_and_execution_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_equality_law.lyra",
            ErrorCode::DuplicateCanonicalModel,
        ),
        (
            "invalid_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_normalization_digest.lyra",
            ErrorCode::ReceiptHashMismatch,
        ),
        (
            "invalid_substitution_digest.lyra",
            ErrorCode::ReceiptHashMismatch,
        ),
        (
            "invalid_normalization_output.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_unknown_substitution.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_receipt_target.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_symbolic_equality_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_symbolic_equality_truth_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_equality.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_equality.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_symbolic_equality_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
