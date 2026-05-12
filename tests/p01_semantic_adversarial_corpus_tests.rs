use lyra_phase0::p01::{
    canonical_semantic_adversarial_corpus_registry_hash,
    canonical_semantic_adversarial_corpus_registry_signature,
    parse_semantic_adversarial_corpus_surface, semantic_adversarial_all_case_ids,
    semantic_adversarial_harness_ids, semantic_ambiguity_probe_descriptor,
    semantic_ambiguity_probe_ids, semantic_collision_probe_descriptor,
    semantic_collision_probe_ids, semantic_malformed_object_descriptor,
    semantic_malformed_object_ids, validate_semantic_adversarial_corpus_surface, ErrorCode,
    REQUIRED_SEMANTIC_ADVERSARIAL_HARNESSES, REQUIRED_SEMANTIC_ADVERSARIAL_RECEIPTS,
    REQUIRED_SEMANTIC_ADVERSARIAL_RULES, REQUIRED_SEMANTIC_AMBIGUITY_PROBES,
    REQUIRED_SEMANTIC_COLLISION_PROBES, REQUIRED_SEMANTIC_MALFORMED_OBJECTS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/semantic_adversarial_corpus_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_adversarial_corpus_surface() {
    let input = fixture("valid_semantic_adversarial_corpus.lyra");
    let parsed = parse_semantic_adversarial_corpus_surface(&input)
        .expect("valid semantic adversarial corpus parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-009");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_SEMANTIC_ADVERSARIAL_RULES.len()
    );
    assert_eq!(
        parsed.collision_probes.len(),
        REQUIRED_SEMANTIC_COLLISION_PROBES.len()
    );
    assert_eq!(
        parsed.ambiguity_probes.len(),
        REQUIRED_SEMANTIC_AMBIGUITY_PROBES.len()
    );
    assert_eq!(
        parsed.malformed_objects.len(),
        REQUIRED_SEMANTIC_MALFORMED_OBJECTS.len()
    );
    assert_eq!(
        parsed.harnesses.len(),
        REQUIRED_SEMANTIC_ADVERSARIAL_HARNESSES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SEMANTIC_ADVERSARIAL_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_semantic_adversarial_corpus_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_binds_collision_ambiguity_and_malformed_cases() {
    let signature = canonical_semantic_adversarial_corpus_registry_signature();
    assert!(signature.contains("collision_probe:digest_alias_rewrite"));
    assert!(signature.contains("ambiguity_probe:text_ir_header_case_drift"));
    assert!(signature.contains("malformed_object:unknown_serializer_object_ref"));
    assert_eq!(
        semantic_collision_probe_ids().len(),
        REQUIRED_SEMANTIC_COLLISION_PROBES.len()
    );
    assert_eq!(
        semantic_ambiguity_probe_ids().len(),
        REQUIRED_SEMANTIC_AMBIGUITY_PROBES.len()
    );
    assert_eq!(
        semantic_malformed_object_ids().len(),
        REQUIRED_SEMANTIC_MALFORMED_OBJECTS.len()
    );
    assert_eq!(
        semantic_adversarial_harness_ids().len(),
        REQUIRED_SEMANTIC_ADVERSARIAL_HARNESSES.len()
    );
    assert_eq!(semantic_adversarial_all_case_ids().len(), 11);
    assert!(canonical_semantic_adversarial_corpus_registry_hash().starts_with("fnv1a128:"));
}

#[test]
fn descriptor_fixtures_are_checked_in_and_error_bound() {
    for id in semantic_collision_probe_ids() {
        let descriptor =
            semantic_collision_probe_descriptor(id).expect("collision descriptor exists");
        assert_eq!(descriptor.status, "artifact_emitted");
        assert!(
            std::fs::metadata(descriptor.fixture_path).is_ok(),
            "missing {}",
            descriptor.fixture_path
        );
        assert!(!descriptor.expected_error.is_empty());
    }
    for id in semantic_ambiguity_probe_ids() {
        let descriptor =
            semantic_ambiguity_probe_descriptor(id).expect("ambiguity descriptor exists");
        assert_eq!(descriptor.status, "artifact_emitted");
        assert!(
            std::fs::metadata(descriptor.fixture_path).is_ok(),
            "missing {}",
            descriptor.fixture_path
        );
        assert!(!descriptor.expected_error.is_empty());
    }
    for id in semantic_malformed_object_ids() {
        let descriptor =
            semantic_malformed_object_descriptor(id).expect("malformed descriptor exists");
        assert_eq!(descriptor.status, "artifact_emitted");
        assert!(
            std::fs::metadata(descriptor.fixture_path).is_ok(),
            "missing {}",
            descriptor.fixture_path
        );
        assert!(!descriptor.expected_error.is_empty());
    }
}

#[test]
fn rejects_required_semantic_adversarial_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_collision_probe.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_ambiguity_probe.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_malformed_object.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_harness.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_adversarial_corpus_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_descriptor_and_receipt_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_collision_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_ambiguity_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_malformed_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_duplicate_probe.lyra",
            ErrorCode::DuplicateModelBinding,
        ),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_receipt_target.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_adversarial_corpus_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_adversarial_truth_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_acceptance.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_adversarial_corpus_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
