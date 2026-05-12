use lyra_phase0::{
    k0_verdict::ErrorCode,
    p02_bootstrap_output_table_law::{
        parse_bootstrap_output_table_surface, validate_bootstrap_output_table_surface,
        REQUIRED_BOOTSTRAP_OUTPUT_ARTIFACTS, REQUIRED_BOOTSTRAP_OUTPUT_AUDIENCES,
        REQUIRED_BOOTSTRAP_OUTPUT_CONTRACTS, REQUIRED_BOOTSTRAP_OUTPUT_GAPS,
        REQUIRED_BOOTSTRAP_OUTPUT_RECEIPTS,
    },
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p02/bootstrap_output_table_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_bootstrap_output_table_surface() {
    let input = fixture("valid_bootstrap_output_table.lyra");
    let parsed = parse_bootstrap_output_table_surface(&input).expect("valid output table parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-X04");
    assert_eq!(parsed.global_closure, "denied");
    assert_eq!(parsed.next_frontier, "P02-X05");
    assert_eq!(
        parsed.audiences.len(),
        REQUIRED_BOOTSTRAP_OUTPUT_AUDIENCES.len()
    );
    assert_eq!(
        parsed.artifacts.len(),
        REQUIRED_BOOTSTRAP_OUTPUT_ARTIFACTS.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_BOOTSTRAP_OUTPUT_RECEIPTS.len()
    );
    assert_eq!(
        parsed.contracts.len(),
        REQUIRED_BOOTSTRAP_OUTPUT_CONTRACTS.len()
    );
    assert_eq!(parsed.gaps.len(), REQUIRED_BOOTSTRAP_OUTPUT_GAPS.len());
    let (verdict, receipt) = validate_bootstrap_output_table_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_bootstrap_output_table_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_audience.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_missing_artifact.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        (
            "invalid_missing_contract.lyra",
            ErrorCode::MissingClosureProof,
        ),
        (
            "invalid_missing_gap.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_bootstrap_output_table_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_unknown_and_invalid_bindings() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_artifact.lyra",
            ErrorCode::DuplicateClosureOutputGate,
        ),
        (
            "invalid_unknown_audience_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_unknown_artifact_reference.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_invalid_artifact_path.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_bad_gap_frontier.lyra",
            ErrorCode::ClosureOutputPremature,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_bootstrap_output_table_surface(&input);
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
            "invalid_docs_only_output_table.lyra",
            ErrorCode::ClosureDocsOnly,
        ),
        (
            "invalid_unreceipted_output_table.lyra",
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
        let (verdict, _) = validate_bootstrap_output_table_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
