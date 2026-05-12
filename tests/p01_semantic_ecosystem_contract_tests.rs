use std::fs;
use std::path::Path;

use lyra_phase0::p01::{
    parse_semantic_ecosystem_surface, validate_semantic_ecosystem_surface,
    P01_SEMANTIC_ECOSYSTEM_CONTRACT, REQUIRED_SEMANTIC_ECOSYSTEM_DOCS,
    REQUIRED_SEMANTIC_ECOSYSTEM_EXAMPLES, REQUIRED_SEMANTIC_ECOSYSTEM_PROOFS,
    REQUIRED_SEMANTIC_ECOSYSTEM_RULES,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/valid_semantic_ecosystem.lyra");

#[test]
fn p01_021_contract_binds_ecosystem_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_ecosystem.v1.lyra")
        .expect("P01-021 semantic ecosystem contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-ECOSYSTEM-CONTRACT v1"));
    assert!(contract.contains("task=P01-021"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-ECOSYSTEM-DOCS-EXAMPLES v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0050_semantic_ecosystem.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-ecosystem-check.rs"));
    assert!(Path::new("ops/p01/control/semantic_ecosystem_law.v1.lyra").exists());
}

#[test]
fn p01_021_valid_surface_matches_required_inventory() {
    let surface = parse_semantic_ecosystem_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P01_SEMANTIC_ECOSYSTEM_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_SEMANTIC_ECOSYSTEM_RULES.len());
    assert_eq!(surface.docs.len(), REQUIRED_SEMANTIC_ECOSYSTEM_DOCS.len());
    assert_eq!(
        surface.examples.len(),
        REQUIRED_SEMANTIC_ECOSYSTEM_EXAMPLES.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_SEMANTIC_ECOSYSTEM_PROOFS.len()
    );
}

#[test]
fn p01_021_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_semantic_ecosystem_surface(VALID);
    let golden = fs::read_to_string("goldens/p01/valid_semantic_ecosystem.receipt")
        .expect("semantic ecosystem golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
