use lyra_phase0::p01::{
    canonical_registry_signature, parse_semantic_atom_surface, validate_semantic_atom_surface,
    ErrorCode, REQUIRED_SEMANTIC_ATOMS, REQUIRED_SEMANTIC_ATOM_FAMILIES,
    REQUIRED_SEMANTIC_ATOM_RECEIPTS, REQUIRED_SEMANTIC_ATOM_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/semantic_atom_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_atom_surface() {
    let input = fixture("valid_semantic_atoms.lyra");
    let parsed = parse_semantic_atom_surface(&input).expect("valid semantic atoms parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-001");
    assert_eq!(parsed.rules.len(), REQUIRED_SEMANTIC_ATOM_RULES.len());
    assert_eq!(parsed.atoms.len(), REQUIRED_SEMANTIC_ATOMS.len());
    assert_eq!(parsed.families.len(), REQUIRED_SEMANTIC_ATOM_FAMILIES.len());
    assert_eq!(parsed.receipts.len(), REQUIRED_SEMANTIC_ATOM_RECEIPTS.len());
    let (verdict, receipt) = validate_semantic_atom_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_signature_is_byte_stable_and_contains_all_core_atoms() {
    let signature = canonical_registry_signature();
    for atom in REQUIRED_SEMANTIC_ATOMS {
        assert!(
            signature.contains(&format!("atom:{atom}|")),
            "registry missing {atom}"
        );
    }
    assert!(signature.contains("serialization:text_binary_ir_parity"));
}

#[test]
fn rejects_required_semantic_atom_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_atom.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_atom_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_unbound_owner_and_registry_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_atom.lyra",
            ErrorCode::DuplicateCanonicalModel,
        ),
        (
            "invalid_unknown_family_member.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
        ("invalid_owner_root.lyra", ErrorCode::InvalidOwnerRoot),
        (
            "invalid_registry_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_atom_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_atom_truth_claims() {
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
            "invalid_placeholder_atom.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_atom_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
