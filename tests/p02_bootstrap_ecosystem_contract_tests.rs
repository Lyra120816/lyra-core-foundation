use std::fs;
use std::path::Path;

use lyra_phase0::p02::{
    parse_bootstrap_ecosystem_surface, validate_bootstrap_ecosystem_surface,
    P02_BOOTSTRAP_ECOSYSTEM_CONTRACT, REQUIRED_BOOTSTRAP_ECOSYSTEM_DOCS,
    REQUIRED_BOOTSTRAP_ECOSYSTEM_EXAMPLES, REQUIRED_BOOTSTRAP_ECOSYSTEM_PROOFS,
    REQUIRED_BOOTSTRAP_ECOSYSTEM_RULES,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_ecosystem_inputs/valid_bootstrap_ecosystem.lyra");

#[test]
fn p02_021_contract_binds_bootstrap_ecosystem_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p02/contracts/bootstrap_ecosystem.v1.lyra")
        .expect("P02-021 bootstrap ecosystem contract must exist");

    assert!(contract.starts_with("LYRA-P02-BOOTSTRAP-ECOSYSTEM-CONTRACT v1"));
    assert!(contract.contains("task=P02-021"));
    assert!(contract.contains("surface=LYRA-P02-BOOTSTRAP-ECOSYSTEM-DOCS-EXAMPLES v1"));
    assert!(contract.contains("receipt=receipts/p02/pass_0079_bootstrap_ecosystem.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p02-bootstrap-ecosystem-check.rs"));
    assert!(Path::new("ops/p02/control/bootstrap_ecosystem_law.v1.lyra").exists());
}

#[test]
fn p02_021_valid_surface_matches_required_inventory() {
    let surface = parse_bootstrap_ecosystem_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P02_BOOTSTRAP_ECOSYSTEM_CONTRACT);
    assert_eq!(
        surface.rules.len(),
        REQUIRED_BOOTSTRAP_ECOSYSTEM_RULES.len()
    );
    assert_eq!(surface.docs.len(), REQUIRED_BOOTSTRAP_ECOSYSTEM_DOCS.len());
    assert_eq!(
        surface.examples.len(),
        REQUIRED_BOOTSTRAP_ECOSYSTEM_EXAMPLES.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_BOOTSTRAP_ECOSYSTEM_PROOFS.len()
    );
}

#[test]
fn p02_021_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_bootstrap_ecosystem_surface(VALID);
    let golden = fs::read_to_string("goldens/p02/valid_bootstrap_ecosystem.receipt")
        .expect("bootstrap ecosystem golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
