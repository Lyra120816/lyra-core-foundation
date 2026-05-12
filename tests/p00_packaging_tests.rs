use lyra_phase0::p00::{
    parse_packaging_surface, validate_packaging_surface, ErrorCode, REQUIRED_DISTRIBUTION_CHECKS,
    REQUIRED_PACKAGE_UNITS, REQUIRED_PACKAGING_PROOFS, REQUIRED_RELEASE_BUNDLES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p00/packaging_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_packaging_surface() {
    let input = fixture("valid_packaging_surface.lyra");
    let parsed = parse_packaging_surface(&input).expect("valid packaging surface parses");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-019");
    assert_eq!(parsed.packages.len(), REQUIRED_PACKAGE_UNITS.len());
    assert_eq!(parsed.bundles.len(), REQUIRED_RELEASE_BUNDLES.len());
    assert_eq!(parsed.checks.len(), REQUIRED_DISTRIBUTION_CHECKS.len());
    assert_eq!(parsed.proofs.len(), REQUIRED_PACKAGING_PROOFS.len());
    let (verdict, receipt) = validate_packaging_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_packaging_surface_gaps() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingPackagingRule),
        (
            "invalid_missing_package.lyra",
            ErrorCode::MissingPackageUnit,
        ),
        (
            "invalid_missing_bundle.lyra",
            ErrorCode::MissingReleaseBundle,
        ),
        (
            "invalid_missing_check.lyra",
            ErrorCode::MissingDistributionCheck,
        ),
        (
            "invalid_missing_proof.lyra",
            ErrorCode::MissingPackagingProof,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_packaging_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_package_identity() {
    let input = fixture("invalid_duplicate_package.lyra");
    let (verdict, _) = validate_packaging_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::DuplicatePackageUnit));
}

#[test]
fn rejects_unknown_bundle_package_and_unbound_proof() {
    for (fixture_name, expected) in [
        (
            "invalid_unknown_bundle_package.lyra",
            ErrorCode::InvalidReleaseBundle,
        ),
        (
            "invalid_unbound_proof_reference.lyra",
            ErrorCode::PackagingProofUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_packaging_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_network_required_packaging_and_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::PackagingNetworkDependency,
        ),
        (
            "invalid_package_drift.lyra",
            ErrorCode::PackagingDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_packaging_surface(&input);
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
    let (verdict, _) = validate_packaging_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::UnsupportedGlobalClosure));
}
