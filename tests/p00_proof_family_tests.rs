use lyra_phase0::p00::{
    parse_proof_family_table_surface, validate_proof_family_table_surface, ErrorCode,
    REQUIRED_PROOF_FAMILIES, REQUIRED_PROOF_PATHS, REQUIRED_PROOF_RECEIPTS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p00/proof_family_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_proof_family_table_surface() {
    let input = fixture("valid_proof_family_table.lyra");
    let parsed = parse_proof_family_table_surface(&input).expect("valid proof family table parses");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-X02");
    assert_eq!(parsed.families.len(), REQUIRED_PROOF_FAMILIES.len());
    assert_eq!(parsed.receipts.len(), REQUIRED_PROOF_RECEIPTS.len());
    assert_eq!(parsed.paths.len(), REQUIRED_PROOF_PATHS.len());
    let (verdict, receipt) = validate_proof_family_table_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_proof_family_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_family.lyra",
            ErrorCode::MissingClosureProof,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        (
            "invalid_missing_path.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_proof_family_table_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_unknown_and_invalid_receipt_bindings() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_receipt.lyra",
            ErrorCode::DuplicateClosureProof,
        ),
        (
            "invalid_unknown_receipt_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_unknown_coverage_target.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_unknown_path_receipt.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        ("invalid_receipt_path.lyra", ErrorCode::InvalidClosureProof),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_proof_family_table_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_docs_only_unreceipted_global_and_drift_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::ClosureNetworkDependency,
        ),
        (
            "invalid_docs_only_proof_table.lyra",
            ErrorCode::ClosureDocsOnly,
        ),
        (
            "invalid_unreceipted_proof_family.lyra",
            ErrorCode::ClosureUnreceipted,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_drift_accepted.lyra",
            ErrorCode::ClosureDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_proof_family_table_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
