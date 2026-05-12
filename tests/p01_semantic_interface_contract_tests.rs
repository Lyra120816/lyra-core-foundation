use std::fs;
use std::path::Path;

use lyra_phase0::p01::{
    parse_semantic_interface_surface, validate_semantic_interface_surface,
    P01_SEMANTIC_INTERFACE_CONTRACT, REQUIRED_SEMANTIC_INTERFACE_ARTIFACTS,
    REQUIRED_SEMANTIC_INTERFACE_COMMANDS, REQUIRED_SEMANTIC_INTERFACE_EXAMPLES,
    REQUIRED_SEMANTIC_INTERFACE_PROOFS, REQUIRED_SEMANTIC_INTERFACE_RULES,
    REQUIRED_SEMANTIC_INTERFACE_WORKFLOWS,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/valid_semantic_interface.lyra");

#[test]
fn p01_018_contract_binds_operator_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_interface.v1.lyra")
        .expect("P01-018 semantic interface contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-INTERFACE-CONTRACT v1"));
    assert!(contract.contains("task=P01-018"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-INTERFACE v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0047_semantic_interface.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-interface-check.rs"));
    assert!(Path::new("ops/p01/control/semantic_interface_law.v1.lyra").exists());
}

#[test]
fn p01_018_valid_surface_matches_required_inventory() {
    let surface = parse_semantic_interface_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P01_SEMANTIC_INTERFACE_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_SEMANTIC_INTERFACE_RULES.len());
    assert_eq!(
        surface.commands.len(),
        REQUIRED_SEMANTIC_INTERFACE_COMMANDS.len()
    );
    assert_eq!(
        surface.workflows.len(),
        REQUIRED_SEMANTIC_INTERFACE_WORKFLOWS.len()
    );
    assert_eq!(
        surface.examples.len(),
        REQUIRED_SEMANTIC_INTERFACE_EXAMPLES.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_SEMANTIC_INTERFACE_PROOFS.len()
    );
    assert_eq!(
        surface.artifacts.len(),
        REQUIRED_SEMANTIC_INTERFACE_ARTIFACTS.len()
    );
}

#[test]
fn p01_018_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_semantic_interface_surface(VALID);
    let golden = fs::read_to_string("goldens/p01/valid_semantic_interface.receipt")
        .expect("semantic interface golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
