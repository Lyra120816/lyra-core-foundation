use lyra_phase0::p01::{
    canonical_identity_digest_from_parts, canonical_semantic_identity_registry_signature,
    parse_semantic_identity_surface, validate_semantic_identity_surface, ErrorCode,
    REQUIRED_SEMANTIC_COLLISION_CASES, REQUIRED_SEMANTIC_DIGEST_CASES,
    REQUIRED_SEMANTIC_IDENTITY_DOMAINS, REQUIRED_SEMANTIC_IDENTITY_RECEIPTS,
    REQUIRED_SEMANTIC_IDENTITY_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/semantic_identity_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_identity_surface() {
    let input = fixture("valid_semantic_identity.lyra");
    let parsed = parse_semantic_identity_surface(&input).expect("valid semantic identity parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-004");
    assert_eq!(parsed.rules.len(), REQUIRED_SEMANTIC_IDENTITY_RULES.len());
    assert_eq!(
        parsed.identities.len(),
        REQUIRED_SEMANTIC_IDENTITY_DOMAINS.len()
    );
    assert_eq!(
        parsed.digest_cases.len(),
        REQUIRED_SEMANTIC_DIGEST_CASES.len()
    );
    assert_eq!(
        parsed.collision_cases.len(),
        REQUIRED_SEMANTIC_COLLISION_CASES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SEMANTIC_IDENTITY_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_semantic_identity_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn semantic_identity_registry_and_digest_are_stable() {
    let signature = canonical_semantic_identity_registry_signature();
    assert!(signature.contains("semantic_identity:symbol|scope:global_symbol_table"));
    assert!(signature.contains("semantic_identity:artifact|scope:artifact_manifest_table"));
    let digest = canonical_identity_digest_from_parts(
        "symbol",
        "symbol_core",
        "lyralang",
        "lyra.symbol.core",
        "lower_ascii_symbolic_path",
    )
    .expect("digest");
    assert_eq!(digest, "fnv1a128:a89e4697828a07a86bcc3c14b757dfc4");
}

#[test]
fn rejects_required_semantic_identity_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_identity.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_digest_case.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_collision_case.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_identity_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_identity_digest_and_collision_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_identity.lyra",
            ErrorCode::DuplicateCanonicalModel,
        ),
        (
            "invalid_digest_mismatch.lyra",
            ErrorCode::ReceiptHashMismatch,
        ),
        (
            "invalid_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_unknown_domain.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
        (
            "invalid_wrong_digest_algorithm.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_collision_alias.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_identity_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_identity_truth_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_identity.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_identity.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_identity_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
