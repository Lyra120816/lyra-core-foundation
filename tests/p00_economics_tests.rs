use lyra_phase0::p00::{
    parse_economics_surface, validate_economics_surface, ErrorCode, REQUIRED_ECONOMICS_FRAMES,
    REQUIRED_ECONOMICS_PROOFS, REQUIRED_PUBLIC_INTEREST_OUTPUTS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p00/economics_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_economics_public_interest_surface() {
    let input = fixture("valid_economics_public_interest.lyra");
    let parsed = parse_economics_surface(&input).expect("valid economics surface parses");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-022");
    assert_eq!(parsed.frames.len(), REQUIRED_ECONOMICS_FRAMES.len());
    assert_eq!(parsed.outputs.len(), REQUIRED_PUBLIC_INTEREST_OUTPUTS.len());
    assert_eq!(parsed.proofs.len(), REQUIRED_ECONOMICS_PROOFS.len());
    let (verdict, receipt) = validate_economics_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_economics_surface_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingEconomicsRule),
        (
            "invalid_missing_frame.lyra",
            ErrorCode::MissingEconomicsFrame,
        ),
        (
            "invalid_missing_output.lyra",
            ErrorCode::MissingEconomicsOutput,
        ),
        (
            "invalid_missing_proof.lyra",
            ErrorCode::MissingEconomicsProof,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_economics_surface(&input);
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
            "invalid_duplicate_frame.lyra",
            ErrorCode::DuplicateEconomicsFrame,
        ),
        (
            "invalid_unknown_output_reference.lyra",
            ErrorCode::InvalidEconomicsFrame,
        ),
        (
            "invalid_unbound_proof_reference.lyra",
            ErrorCode::EconomicsProofUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_economics_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_capture_extraction_and_drift_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::EconomicsNetworkDependency,
        ),
        (
            "invalid_capture_allowed.lyra",
            ErrorCode::EconomicsCaptureAllowed,
        ),
        (
            "invalid_extractive_default.lyra",
            ErrorCode::EconomicsExtractiveDefault,
        ),
        (
            "invalid_economics_drift.lyra",
            ErrorCode::EconomicsDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_economics_surface(&input);
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
    let (verdict, _) = validate_economics_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::UnsupportedGlobalClosure));
}
