use std::fs;
use std::path::Path;

use lyra_phase0::p02::{
    parse_bootstrap_packaging_surface, validate_bootstrap_packaging_surface,
    P02_BOOTSTRAP_PACKAGING_CONTRACT, REQUIRED_BOOTSTRAP_DISTRIBUTION_CHECKS,
    REQUIRED_BOOTSTRAP_PACKAGE_UNITS, REQUIRED_BOOTSTRAP_PACKAGING_PROOFS,
    REQUIRED_BOOTSTRAP_PACKAGING_RULES, REQUIRED_BOOTSTRAP_RELEASE_BUNDLES,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_packaging_inputs/valid_bootstrap_packaging.lyra");

#[test]
fn p02_019_contract_binds_packaging_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p02/contracts/bootstrap_packaging.v1.lyra")
        .expect("P02-019 bootstrap packaging contract must exist");
    assert!(contract.starts_with("LYRA-P02-BOOTSTRAP-PACKAGING-CONTRACT v1"));
    assert!(contract.contains("task=P02-019"));
    assert!(contract.contains("surface=LYRA-P02-BOOTSTRAP-PACKAGING v1"));
    assert!(contract.contains("receipt=receipts/p02/pass_0077_bootstrap_packaging.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p02-bootstrap-packaging-check.rs"));
    assert!(Path::new("ops/p02/packaging/bootstrap_packaging.v1.lyra").exists());
    assert!(Path::new("products/p02/bootstrap_package_manifest.v1.lyra").exists());
}

#[test]
fn p02_019_valid_surface_matches_required_inventory() {
    let surface = parse_bootstrap_packaging_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P02_BOOTSTRAP_PACKAGING_CONTRACT);
    assert_eq!(
        surface.rules.len(),
        REQUIRED_BOOTSTRAP_PACKAGING_RULES.len()
    );
    assert_eq!(
        surface.packages.len(),
        REQUIRED_BOOTSTRAP_PACKAGE_UNITS.len()
    );
    assert_eq!(
        surface.bundles.len(),
        REQUIRED_BOOTSTRAP_RELEASE_BUNDLES.len()
    );
    assert_eq!(
        surface.checks.len(),
        REQUIRED_BOOTSTRAP_DISTRIBUTION_CHECKS.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_BOOTSTRAP_PACKAGING_PROOFS.len()
    );
}

#[test]
fn p02_019_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_bootstrap_packaging_surface(VALID);
    let golden = fs::read_to_string("goldens/p02/valid_bootstrap_packaging.receipt")
        .expect("bootstrap packaging golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
