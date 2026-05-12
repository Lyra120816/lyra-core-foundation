use lyra_phase0::p01::{
    deterministic_semantic_packaging_suite_report, validate_semantic_packaging_surface, ErrorCode,
    REQUIRED_SEMANTIC_DISTRIBUTION_CHECKS, REQUIRED_SEMANTIC_PACKAGE_UNITS,
    REQUIRED_SEMANTIC_PACKAGING_PROOFS, REQUIRED_SEMANTIC_RELEASE_BUNDLES,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/valid_semantic_packaging.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_missing_rule.lyra");
const INVALID_MISSING_PACKAGE: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_missing_package.lyra");
const INVALID_DUPLICATE_PACKAGE: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_duplicate_package.lyra");
const INVALID_UNKNOWN_BUNDLE_PACKAGE: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_unknown_bundle_package.lyra");
const INVALID_MISSING_BUNDLE: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_missing_bundle.lyra");
const INVALID_MISSING_CHECK: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_missing_check.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_missing_proof.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_NETWORK_REQUIRED: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_network_required.lyra");
const INVALID_PACKAGE_DRIFT: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_package_drift.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_phase_closure_claim.lyra");
const INVALID_OWNER: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_invalid_artifact_owner.lyra");
const INVALID_RECEIPT: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/invalid_missing_receipt_binding.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_semantic_packaging_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_semantic_packaging_surface() {
    let (verdict, receipt) = validate_semantic_packaging_surface(VALID);
    assert!(
        verdict.accepted,
        "valid semantic packaging rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn semantic_packaging_deterministic_report_is_stable_and_counted() {
    let packages = vec![
        (
            "z_package".to_string(),
            "receipt_set".to_string(),
            "receipts".to_string(),
            vec!["receipts/p01/z.receipt".to_string()],
            vec!["lyra-p01-z-check".to_string()],
            vec!["receipts/p01/z.receipt".to_string()],
            "artifact_emitted".to_string(),
        ),
        (
            "a_package".to_string(),
            "binary_group".to_string(),
            "src".to_string(),
            vec!["src/bin/a.rs".to_string()],
            vec!["lyra-p01-a-check".to_string()],
            vec!["receipts/p01/a.receipt".to_string()],
            "artifact_emitted".to_string(),
        ),
    ];
    let bundles = vec![(
        "bundle_z".to_string(),
        "002".to_string(),
        vec!["z_package".to_string()],
        vec!["products/p01/z.lyra".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["check_z".to_string()],
        vec!["drift".to_string()],
        "artifact_emitted".to_string(),
    )];
    let checks = vec![(
        "check_z".to_string(),
        "bundle".to_string(),
        "bundle_z".to_string(),
        vec!["z_package".to_string()],
        vec!["drift".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        "artifact_emitted".to_string(),
    )];
    let proofs = vec![(
        "proof_z".to_string(),
        "bundle".to_string(),
        vec!["z_package".to_string()],
        vec!["bundle_z".to_string()],
        vec!["check_z".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["lyra-p01-z-check".to_string()],
        vec!["drift".to_string()],
        "artifact_emitted".to_string(),
    )];
    let report =
        deterministic_semantic_packaging_suite_report(&packages, &bundles, &checks, &proofs);
    assert_eq!(report.package_count, 2);
    assert_eq!(report.bundle_count, 1);
    assert_eq!(report.check_count, 1);
    assert_eq!(report.proof_count, 1);
    assert_eq!(report.packages[0].id, "a_package");
    assert!(report.suite_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_packaging_gaps() {
    for (input, expected) in [
        (INVALID_MISSING_RULE, ErrorCode::MissingPackagingRule),
        (INVALID_MISSING_PACKAGE, ErrorCode::MissingPackageUnit),
        (INVALID_MISSING_BUNDLE, ErrorCode::MissingReleaseBundle),
        (INVALID_MISSING_CHECK, ErrorCode::MissingDistributionCheck),
        (INVALID_MISSING_PROOF, ErrorCode::MissingPackagingProof),
    ] {
        assert_rejects_with(input, expected);
    }
}

#[test]
fn rejects_duplicate_and_invalid_package_bindings() {
    assert_rejects_with(INVALID_DUPLICATE_PACKAGE, ErrorCode::DuplicatePackageUnit);
    assert_rejects_with(INVALID_OWNER, ErrorCode::InvalidPackageUnit);
    assert_rejects_with(INVALID_RECEIPT, ErrorCode::InvalidPackageUnit);
}

#[test]
fn rejects_bundle_and_proof_unbound_references() {
    assert_rejects_with(
        INVALID_UNKNOWN_BUNDLE_PACKAGE,
        ErrorCode::InvalidReleaseBundle,
    );
    assert_rejects_with(INVALID_UNBOUND_PROOF, ErrorCode::PackagingProofUnbound);
}

#[test]
fn rejects_network_packaging_drift_and_closure_claims() {
    assert_rejects_with(
        INVALID_NETWORK_REQUIRED,
        ErrorCode::PackagingNetworkDependency,
    );
    assert_rejects_with(INVALID_PACKAGE_DRIFT, ErrorCode::PackagingDriftAccepted);
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn required_semantic_packaging_inventory_counts_are_bound() {
    assert_eq!(REQUIRED_SEMANTIC_PACKAGE_UNITS.len(), 7);
    assert_eq!(REQUIRED_SEMANTIC_RELEASE_BUNDLES.len(), 4);
    assert_eq!(REQUIRED_SEMANTIC_DISTRIBUTION_CHECKS.len(), 7);
    assert_eq!(REQUIRED_SEMANTIC_PACKAGING_PROOFS.len(), 5);
}
