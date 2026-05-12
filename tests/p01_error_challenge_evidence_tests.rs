use lyra_phase0::p01::{
    canonical_error_challenge_evidence_registry_signature, canonical_first_class_object_text,
    challenge_object_ids, error_object_ids, evidence_object_ids, first_class_object_digest,
    linked_object_pairs, object_link_ids, parse_error_challenge_evidence_surface,
    validate_error_challenge_evidence_references, validate_error_challenge_evidence_surface,
    ErrorCode, FirstClassDiagnosticObjectKind, REQUIRED_CHALLENGE_OBJECTS,
    REQUIRED_ERROR_CHALLENGE_EVIDENCE_RECEIPTS, REQUIRED_ERROR_CHALLENGE_EVIDENCE_RULES,
    REQUIRED_ERROR_OBJECTS, REQUIRED_EVIDENCE_OBJECTS, REQUIRED_OBJECT_LINKS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/error_challenge_evidence_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_error_challenge_evidence_surface() {
    let input = fixture("valid_error_challenge_evidence.lyra");
    let parsed = parse_error_challenge_evidence_surface(&input)
        .expect("valid error challenge evidence parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-007");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_ERROR_CHALLENGE_EVIDENCE_RULES.len()
    );
    assert_eq!(parsed.error_objects.len(), REQUIRED_ERROR_OBJECTS.len());
    assert_eq!(
        parsed.challenge_objects.len(),
        REQUIRED_CHALLENGE_OBJECTS.len()
    );
    assert_eq!(
        parsed.evidence_objects.len(),
        REQUIRED_EVIDENCE_OBJECTS.len()
    );
    assert_eq!(parsed.object_links.len(), REQUIRED_OBJECT_LINKS.len());
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_ERROR_CHALLENGE_EVIDENCE_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_error_challenge_evidence_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn diagnostic_objects_project_to_first_class_symbolic_terms() {
    let signature = canonical_error_challenge_evidence_registry_signature();
    assert!(signature.contains("error_object:parse_missing_token"));
    assert!(signature.contains("challenge_object:challenge_parse_error"));
    assert!(signature.contains("evidence_object:evidence_parser_replay"));
    assert!(signature.contains("object_link:error_parse_supported"));

    let error_text = canonical_first_class_object_text(
        FirstClassDiagnosticObjectKind::Error,
        "parse_missing_token",
    )
    .expect("error object text");
    assert!(error_text.contains("kind=text.\"error\""));
    assert!(error_text.contains("symbol.evidence_parser_replay"));
    let challenge_digest = first_class_object_digest(
        FirstClassDiagnosticObjectKind::Challenge,
        "challenge_parse_error",
    )
    .expect("challenge digest");
    assert!(challenge_digest.starts_with("fnv1a128:"));
    assert!(validate_error_challenge_evidence_references().is_ok());
}

#[test]
fn registry_ids_and_links_are_stable() {
    assert_eq!(error_object_ids().len(), REQUIRED_ERROR_OBJECTS.len());
    assert_eq!(
        challenge_object_ids().len(),
        REQUIRED_CHALLENGE_OBJECTS.len()
    );
    assert_eq!(evidence_object_ids().len(), REQUIRED_EVIDENCE_OBJECTS.len());
    assert_eq!(object_link_ids().len(), REQUIRED_OBJECT_LINKS.len());
    let pairs = linked_object_pairs();
    assert!(pairs.contains(&("parse_missing_token", "evidence_parser_replay")));
    assert!(pairs.contains(&("challenge_parse_error", "parse_missing_token")));
}

#[test]
fn rejects_required_error_challenge_evidence_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_error_object.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_challenge_object.lyra",
            ErrorCode::MissingChallengeRule,
        ),
        (
            "invalid_missing_evidence_object.lyra",
            ErrorCode::MissingEvidenceBinding,
        ),
        (
            "invalid_missing_object_link.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_error_challenge_evidence_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_descriptor_digest_and_reference_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_error_object.lyra",
            ErrorCode::DuplicateModelBinding,
        ),
        ("invalid_error_digest.lyra", ErrorCode::ReceiptHashMismatch),
        (
            "invalid_challenge_target.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_counter_evidence.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_evidence_digest.lyra",
            ErrorCode::ReceiptHashMismatch,
        ),
        (
            "invalid_object_link_endpoint.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_object_link_digest.lyra",
            ErrorCode::ReceiptHashMismatch,
        ),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_receipt_target.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_error_challenge_evidence_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_error_challenge_evidence_truth_claims() {
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
            "invalid_placeholder_object.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_error_challenge_evidence_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
