use lyra_phase0::p02::{
    parse_bootstrap_formal_semantics_surface, validate_bootstrap_formal_semantics_surface,
    ErrorCode, REQUIRED_BOOTSTRAP_CONSTITUTIONAL_LAWS, REQUIRED_BOOTSTRAP_FORMAL_DOMAINS,
    REQUIRED_BOOTSTRAP_FORMAL_INVARIANTS, REQUIRED_BOOTSTRAP_FORMAL_PROOFS,
    REQUIRED_BOOTSTRAP_FORMAL_RECEIPTS, REQUIRED_BOOTSTRAP_FORMAL_RULES,
    REQUIRED_BOOTSTRAP_FORMAL_TRANSITIONS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/bootstrap_formal_semantics_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_bootstrap_formal_semantics() {
    let input = fixture("valid_bootstrap_formal_semantics.lyra");
    let parsed = parse_bootstrap_formal_semantics_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-013");
    assert_eq!(parsed.rules.len(), REQUIRED_BOOTSTRAP_FORMAL_RULES.len());
    assert_eq!(
        parsed.domains.len(),
        REQUIRED_BOOTSTRAP_FORMAL_DOMAINS.len()
    );
    assert_eq!(
        parsed.laws.len(),
        REQUIRED_BOOTSTRAP_CONSTITUTIONAL_LAWS.len()
    );
    assert_eq!(
        parsed.transitions.len(),
        REQUIRED_BOOTSTRAP_FORMAL_TRANSITIONS.len()
    );
    assert_eq!(
        parsed.invariants.len(),
        REQUIRED_BOOTSTRAP_FORMAL_INVARIANTS.len()
    );
    assert_eq!(parsed.proofs.len(), REQUIRED_BOOTSTRAP_FORMAL_PROOFS.len());
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_BOOTSTRAP_FORMAL_RECEIPTS.len()
    );
    for domain in REQUIRED_BOOTSTRAP_FORMAL_DOMAINS {
        let row = parsed
            .domain_by_id(domain)
            .expect("required domain present");
        assert!(
            row.constitutional(),
            "domain must be constitutionally bound"
        );
        assert!(
            !parsed.laws_for_domain(domain).is_empty(),
            "domain must have law coverage"
        );
        assert!(
            !parsed.invariants_for_domain(domain).is_empty(),
            "domain must have invariant coverage"
        );
    }
    let (verdict, receipt) = validate_bootstrap_formal_semantics_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_duplicate_and_invalid_bootstrap_formal_semantics() {
    for (name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingFormalSemanticRule,
        ),
        (
            "invalid_missing_domain.lyra",
            ErrorCode::MissingSemanticDomain,
        ),
        ("invalid_duplicate_domain.lyra", ErrorCode::DuplicateEntry),
        ("invalid_bad_domain_root.lyra", ErrorCode::InvalidOwnerRoot),
        (
            "invalid_missing_law.lyra",
            ErrorCode::MissingSemanticRuleBinding,
        ),
        (
            "invalid_bad_law_class.lyra",
            ErrorCode::InvalidSemanticRuleBinding,
        ),
        (
            "invalid_missing_transition.lyra",
            ErrorCode::MissingTransitionLaw,
        ),
        (
            "invalid_bad_transition_guard.lyra",
            ErrorCode::InvalidTransitionLaw,
        ),
        (
            "invalid_unreceipted_transition.lyra",
            ErrorCode::SemanticProofUnbound,
        ),
        (
            "invalid_missing_invariant.lyra",
            ErrorCode::MissingInvariantBinding,
        ),
        (
            "invalid_bad_invariant_reject.lyra",
            ErrorCode::InvalidInvariantBinding,
        ),
        (
            "invalid_missing_proof.lyra",
            ErrorCode::MissingSemanticProof,
        ),
        (
            "invalid_bad_proof_scope.lyra",
            ErrorCode::InvalidSemanticProof,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingReceiptProof,
        ),
        (
            "invalid_bad_receipt_path.lyra",
            ErrorCode::UnknownEvidencePath,
        ),
        (
            "invalid_bad_status.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_formal_semantics_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_bootstrap_formal_semantics_claims() {
    for (name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        ("invalid_ambient_time.lyra", ErrorCode::AmbientTimeAllowed),
        (
            "invalid_probabilistic_truth.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_placeholder_semantics.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_foreign_truth_drift.lyra",
            ErrorCode::ClosureDriftAccepted,
        ),
        (
            "invalid_constitution_override.lyra",
            ErrorCode::OperatorOverrideConstitution,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_formal_semantics_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
