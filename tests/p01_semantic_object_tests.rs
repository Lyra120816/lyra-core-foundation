use lyra_phase0::p01::{
    canonical_semantic_object_registry_signature, canonical_semantic_object_text,
    validate_semantic_object_surface, ErrorCode, SemanticObjectRecord, REQUIRED_SEMANTIC_OBJECTS,
    REQUIRED_SEMANTIC_OBJECT_CONFORMANCES, REQUIRED_SEMANTIC_OBJECT_INVARIANTS,
    REQUIRED_SEMANTIC_OBJECT_RECEIPTS, REQUIRED_SEMANTIC_OBJECT_RELATIONS,
    REQUIRED_SEMANTIC_OBJECT_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/semantic_object_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_object_surface() {
    let input = fixture("valid_semantic_objects.lyra");
    let parsed = lyra_phase0::p01::parse_semantic_object_surface(&input)
        .expect("valid semantic object parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-003");
    assert_eq!(parsed.rules.len(), REQUIRED_SEMANTIC_OBJECT_RULES.len());
    assert_eq!(parsed.objects.len(), REQUIRED_SEMANTIC_OBJECTS.len());
    assert_eq!(
        parsed.relations.len(),
        REQUIRED_SEMANTIC_OBJECT_RELATIONS.len()
    );
    assert_eq!(
        parsed.invariants.len(),
        REQUIRED_SEMANTIC_OBJECT_INVARIANTS.len()
    );
    assert_eq!(
        parsed.conformances.len(),
        REQUIRED_SEMANTIC_OBJECT_CONFORMANCES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SEMANTIC_OBJECT_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_semantic_object_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn semantic_object_registry_and_text_are_stable() {
    let signature = canonical_semantic_object_registry_signature();
    assert!(signature.contains("semantic_object:module|atom:symbol|owner:lyralang"));
    assert!(signature.contains("semantic_object:world|atom:resource|owner:k0"));
    assert!(signature.contains("semantic_object:proof|atom:proof|owner:interfaces"));
    let text = canonical_semantic_object_text(
        &SemanticObjectRecord::new("module_main", "module", "main", "v1")
            .expect("record")
            .with_field("exports", "symbol")
            .expect("field"),
    )
    .expect("canonical text");
    assert!(text.starts_with("LYRA-SEMANTIC-OBJECT v1\n"));
    assert!(text.contains("kind=module\n"));
    assert!(text.contains("field:exports=symbol\n"));
}

#[test]
fn rejects_required_semantic_object_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_object.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_relation.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_invariant.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_conformance.lyra",
            ErrorCode::MissingFixtureProof,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_object_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_drift_unbound_and_cyclic_semantic_object_rows() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_object.lyra",
            ErrorCode::DuplicateCanonicalModel,
        ),
        (
            "invalid_unknown_parent.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_object_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        ("invalid_cycle.lyra", ErrorCode::CanonicalModelDriftAccepted),
        ("invalid_ir_form.lyra", ErrorCode::CanonicalModelUnbound),
        (
            "invalid_relation_endpoint.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_object_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_object_truth_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_object.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_placeholder_object.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_object_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
