use lyra_phase0::p01::{
    canonical_formal_semantic_constitution_registry_hash,
    canonical_formal_semantic_constitution_registry_signature, formal_semantic_domain_descriptor,
    formal_semantic_domain_digest, formal_semantic_domain_ids,
    formal_semantic_domains_bind_one_core, formal_semantic_invariant_descriptor,
    formal_semantic_invariant_digest, formal_semantic_invariant_ids,
    formal_semantic_invariants_reference_admitted_evidence, formal_semantic_law_descriptor,
    formal_semantic_law_digest, formal_semantic_law_ids, formal_semantic_laws_cover_primitive_core,
    formal_semantic_no_forbidden_law_claims, formal_semantic_proof_descriptor,
    formal_semantic_proof_digest, formal_semantic_proof_ids,
    formal_semantic_proofs_bind_known_laws, parse_formal_semantic_constitution_surface,
    validate_formal_semantic_constitution_surface, ErrorCode,
    REQUIRED_FORMAL_SEMANTIC_CONSTITUTION_RULES, REQUIRED_FORMAL_SEMANTIC_DOMAINS,
    REQUIRED_FORMAL_SEMANTIC_INVARIANTS, REQUIRED_FORMAL_SEMANTIC_LAWS,
    REQUIRED_FORMAL_SEMANTIC_PROOFS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/formal_semantic_constitution_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_formal_semantic_constitution_surface() {
    let input = fixture("valid_formal_semantic_constitution.lyra");
    let parsed = parse_formal_semantic_constitution_surface(&input)
        .expect("valid formal semantic constitution parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-013");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_FORMAL_SEMANTIC_CONSTITUTION_RULES.len()
    );
    assert_eq!(parsed.domains.len(), REQUIRED_FORMAL_SEMANTIC_DOMAINS.len());
    assert_eq!(parsed.laws.len(), REQUIRED_FORMAL_SEMANTIC_LAWS.len());
    assert_eq!(
        parsed.invariants.len(),
        REQUIRED_FORMAL_SEMANTIC_INVARIANTS.len()
    );
    assert_eq!(parsed.proofs.len(), REQUIRED_FORMAL_SEMANTIC_PROOFS.len());
    let (verdict, receipt) = validate_formal_semantic_constitution_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_binds_domains_laws_invariants_and_proofs() {
    let signature = canonical_formal_semantic_constitution_registry_signature();
    assert!(signature.contains("domain:canonical_symbols_domain"));
    assert!(signature.contains("law:canonical_symbol_identity_law"));
    assert!(signature.contains("invariant:one_core_invariant"));
    assert!(signature.contains("proof:canonical_symbols_proof"));
    assert_eq!(
        formal_semantic_domain_ids().len(),
        REQUIRED_FORMAL_SEMANTIC_DOMAINS.len()
    );
    assert_eq!(
        formal_semantic_law_ids().len(),
        REQUIRED_FORMAL_SEMANTIC_LAWS.len()
    );
    assert_eq!(
        formal_semantic_invariant_ids().len(),
        REQUIRED_FORMAL_SEMANTIC_INVARIANTS.len()
    );
    assert_eq!(
        formal_semantic_proof_ids().len(),
        REQUIRED_FORMAL_SEMANTIC_PROOFS.len()
    );
    assert!(canonical_formal_semantic_constitution_registry_hash().starts_with("fnv1a128:"));
}

#[test]
fn descriptors_bind_one_core_and_real_artifacts() {
    assert!(formal_semantic_domains_bind_one_core());
    assert!(formal_semantic_laws_cover_primitive_core());
    assert!(formal_semantic_invariants_reference_admitted_evidence());
    assert!(formal_semantic_proofs_bind_known_laws());
    assert!(formal_semantic_no_forbidden_law_claims());

    for id in formal_semantic_domain_ids() {
        let descriptor = formal_semantic_domain_descriptor(id).expect("domain descriptor exists");
        assert_eq!(descriptor.core_ref, "lyra_p01_semantic_core");
        assert!(formal_semantic_domain_digest(id)
            .expect("domain digest")
            .starts_with("fnv1a128:"));
    }
    for id in formal_semantic_law_ids() {
        let descriptor = formal_semantic_law_descriptor(id).expect("law descriptor exists");
        assert_eq!(descriptor.status, "artifact_emitted");
        assert!(formal_semantic_law_digest(id)
            .expect("law digest")
            .starts_with("fnv1a128:"));
    }
    for id in formal_semantic_invariant_ids() {
        let descriptor =
            formal_semantic_invariant_descriptor(id).expect("invariant descriptor exists");
        assert_eq!(descriptor.status, "execution_proven");
        assert!(formal_semantic_invariant_digest(id)
            .expect("invariant digest")
            .starts_with("fnv1a128:"));
    }
    for id in formal_semantic_proof_ids() {
        let descriptor = formal_semantic_proof_descriptor(id).expect("proof descriptor exists");
        assert!(
            std::fs::metadata(descriptor.fixture).is_ok(),
            "missing {}",
            descriptor.fixture
        );
        assert!(
            std::fs::metadata(descriptor.golden).is_ok(),
            "missing {}",
            descriptor.golden
        );
        assert!(
            std::fs::metadata(descriptor.receipt).is_ok(),
            "missing {}",
            descriptor.receipt
        );
        assert!(formal_semantic_proof_digest(id)
            .expect("proof digest")
            .starts_with("fnv1a128:"));
    }
}

#[test]
fn rejects_required_formal_semantic_gaps_and_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingFormalSemanticRule,
        ),
        (
            "invalid_missing_domain.lyra",
            ErrorCode::MissingSemanticDomain,
        ),
        (
            "invalid_missing_law.lyra",
            ErrorCode::MissingSemanticRuleBinding,
        ),
        (
            "invalid_missing_invariant.lyra",
            ErrorCode::MissingInvariantBinding,
        ),
        (
            "invalid_missing_proof.lyra",
            ErrorCode::MissingSemanticProof,
        ),
        ("invalid_duplicate_domain.lyra", ErrorCode::DuplicateEntry),
        (
            "invalid_domain_descriptor_drift.lyra",
            ErrorCode::SemanticDriftAccepted,
        ),
        (
            "invalid_law_descriptor_drift.lyra",
            ErrorCode::SemanticDriftAccepted,
        ),
        (
            "invalid_invariant_descriptor_drift.lyra",
            ErrorCode::SemanticDriftAccepted,
        ),
        (
            "invalid_proof_descriptor_drift.lyra",
            ErrorCode::SemanticDriftAccepted,
        ),
        ("invalid_wrong_task.lyra", ErrorCode::InvalidTask),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_forked_core_claim.lyra",
            ErrorCode::SemanticDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_formal_semantic_constitution_surface(&input);
        assert!(!verdict.accepted, "{fixture_name} should reject");
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} expected {:?}, got {:?}",
            expected,
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_formal_semantic_claims() {
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
            "invalid_placeholder_semantics.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_formal_semantic_constitution_surface(&input);
        assert!(!verdict.accepted, "{fixture_name} should reject");
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} expected {:?}, got {:?}",
            expected,
            verdict.errors
        );
    }
}
