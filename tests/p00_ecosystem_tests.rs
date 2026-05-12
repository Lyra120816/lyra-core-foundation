use lyra_phase0::p00::{
    parse_ecosystem_surface, validate_ecosystem_surface, ErrorCode, REQUIRED_ECOSYSTEM_DOCS,
    REQUIRED_ECOSYSTEM_EXAMPLES, REQUIRED_ECOSYSTEM_PROOFS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p00/ecosystem_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_ecosystem_docs_examples_surface() {
    let input = fixture("valid_ecosystem_docs_examples.lyra");
    let parsed = parse_ecosystem_surface(&input).expect("valid ecosystem surface parses");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-021");
    assert_eq!(parsed.docs.len(), REQUIRED_ECOSYSTEM_DOCS.len());
    assert_eq!(parsed.examples.len(), REQUIRED_ECOSYSTEM_EXAMPLES.len());
    assert_eq!(parsed.proofs.len(), REQUIRED_ECOSYSTEM_PROOFS.len());
    let (verdict, receipt) = validate_ecosystem_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_ecosystem_surface_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingEcosystemRule),
        ("invalid_missing_doc.lyra", ErrorCode::MissingEcosystemDoc),
        (
            "invalid_missing_example.lyra",
            ErrorCode::MissingEcosystemExample,
        ),
        (
            "invalid_missing_proof.lyra",
            ErrorCode::MissingEcosystemProof,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_ecosystem_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_and_unknown_bindings() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_doc.lyra",
            ErrorCode::DuplicateEcosystemDoc,
        ),
        (
            "invalid_unknown_example_reference.lyra",
            ErrorCode::InvalidEcosystemDoc,
        ),
        (
            "invalid_unbound_proof_reference.lyra",
            ErrorCode::EcosystemProofUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_ecosystem_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_docs_only_and_drift_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::EcosystemNetworkDependency,
        ),
        ("invalid_docs_only_claim.lyra", ErrorCode::EcosystemDocsOnly),
        (
            "invalid_ecosystem_drift.lyra",
            ErrorCode::EcosystemDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_ecosystem_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_phase_closure_claim() {
    let input = fixture("invalid_phase_closure_claim.lyra");
    let (verdict, _) = validate_ecosystem_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::UnsupportedGlobalClosure));
}
