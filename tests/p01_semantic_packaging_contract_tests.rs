use std::fs;
use std::path::Path;

use lyra_phase0::p01::{
    parse_semantic_packaging_surface, validate_semantic_packaging_surface,
    P01_SEMANTIC_PACKAGING_CONTRACT, REQUIRED_SEMANTIC_DISTRIBUTION_CHECKS,
    REQUIRED_SEMANTIC_PACKAGE_UNITS, REQUIRED_SEMANTIC_PACKAGING_PROOFS,
    REQUIRED_SEMANTIC_PACKAGING_RULES, REQUIRED_SEMANTIC_RELEASE_BUNDLES,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_packaging_inputs/valid_semantic_packaging.lyra");

#[test]
fn p01_019_contract_binds_packaging_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_packaging.v1.lyra")
        .expect("P01-019 semantic packaging contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-PACKAGING-CONTRACT v1"));
    assert!(contract.contains("task=P01-019"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-PACKAGING v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0048_semantic_packaging.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-packaging-check.rs"));
    assert!(Path::new("ops/p01/control/semantic_packaging_law.v1.lyra").exists());
}

#[test]
fn p01_019_valid_surface_matches_required_inventory() {
    let surface = parse_semantic_packaging_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P01_SEMANTIC_PACKAGING_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_SEMANTIC_PACKAGING_RULES.len());
    assert_eq!(
        surface.packages.len(),
        REQUIRED_SEMANTIC_PACKAGE_UNITS.len()
    );
    assert_eq!(
        surface.bundles.len(),
        REQUIRED_SEMANTIC_RELEASE_BUNDLES.len()
    );
    assert_eq!(
        surface.checks.len(),
        REQUIRED_SEMANTIC_DISTRIBUTION_CHECKS.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_SEMANTIC_PACKAGING_PROOFS.len()
    );
}

#[test]
fn p01_019_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_semantic_packaging_surface(VALID);
    let golden = fs::read_to_string("goldens/p01/valid_semantic_packaging.receipt")
        .expect("semantic packaging golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
