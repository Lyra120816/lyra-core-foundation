use lyra_phase0::p01::{
    canonical_semantic_core_serialization_registry_hash,
    canonical_semantic_core_serialization_registry_signature,
    parse_semantic_serialization_hashing_surface, semantic_core_object_hashes,
    semantic_core_object_refs, semantic_core_serialization_descriptor,
    semantic_core_serialization_family_ids, semantic_core_serialization_round_trip_identity,
    validate_semantic_serialization_hashing_surface, ErrorCode,
    REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES, REQUIRED_SEMANTIC_SERIALIZATION_RECEIPTS,
    REQUIRED_SEMANTIC_SERIALIZATION_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/semantic_serialization_hashing_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_serialization_hashing_surface() {
    let input = fixture("valid_semantic_serialization_hashing.lyra");
    let parsed = parse_semantic_serialization_hashing_surface(&input)
        .expect("valid semantic serialization hashing parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-008");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_SEMANTIC_SERIALIZATION_RULES.len()
    );
    assert_eq!(
        parsed.serializers.len(),
        REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES.len()
    );
    assert_eq!(
        parsed.object_hashes.len(),
        semantic_core_object_refs().len()
    );
    assert_eq!(
        parsed.round_trips.len(),
        REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SEMANTIC_SERIALIZATION_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_semantic_serialization_hashing_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_binds_every_semantic_core_object_ref() {
    let signature = canonical_semantic_core_serialization_registry_signature();
    assert!(signature.contains("object_ref:semantic_atom:symbol"));
    assert!(signature.contains("object_ref:core_ir:text_ir"));
    assert!(signature.contains("object_ref:semantic_object:program"));
    assert!(signature.contains("object_ref:symbolic_equality_rule:reflexive"));
    assert!(signature.contains("object_ref:error_object:parse_missing_token"));
    assert_eq!(
        semantic_core_serialization_family_ids().len(),
        REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES.len()
    );
    assert!(canonical_semantic_core_serialization_registry_hash().starts_with("fnv1a128:"));
}

#[test]
fn object_hashes_and_round_trips_are_stable() {
    let refs = semantic_core_object_refs();
    assert!(refs.len() >= 80);
    for object_ref in [
        "semantic_atom:symbol",
        "core_ir:binary_ir",
        "semantic_object:proof",
        "reference_literal:unit",
        "challenge_object:challenge_parse_error",
    ] {
        let descriptor =
            semantic_core_serialization_descriptor(object_ref).expect("descriptor exists");
        let (payload_hash, record_hash) =
            semantic_core_object_hashes(object_ref).expect("hashes exist");
        assert!(payload_hash.starts_with("fnv1a128:"));
        assert!(record_hash.starts_with("fnv1a128:"));
        assert_eq!(descriptor.object_ref, object_ref);
        assert!(semantic_core_serialization_round_trip_identity(object_ref)
            .expect("round trip identity"));
    }
}

#[test]
fn rejects_required_semantic_serialization_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_serializer.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_object_hash.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_round_trip.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_serialization_hashing_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_hash_descriptor_and_receipt_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_object_hash.lyra",
            ErrorCode::DuplicateModelBinding,
        ),
        ("invalid_payload_hash.lyra", ErrorCode::ReceiptHashMismatch),
        ("invalid_record_hash.lyra", ErrorCode::ReceiptHashMismatch),
        ("invalid_object_ref.lyra", ErrorCode::CanonicalModelUnbound),
        (
            "invalid_serializer_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_receipt_target.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_serialization_hashing_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_serialization_truth_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_truth.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_serialization.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_serialization_hashing_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
