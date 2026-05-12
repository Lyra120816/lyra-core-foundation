use std::fs;
use std::path::Path;

use lyra_phase0::p01::{
    parse_semantic_closure_surface, validate_semantic_closure_surface,
    P01_SEMANTIC_CLOSURE_CONTRACT, REQUIRED_SEMANTIC_CLOSURE_OUTPUTS,
    REQUIRED_SEMANTIC_CLOSURE_PROOFS, REQUIRED_SEMANTIC_CLOSURE_RULES,
    REQUIRED_SEMANTIC_CLOSURE_TASKS,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_closure_inputs/valid_semantic_closure.lyra");

#[test]
fn p01_024_contract_binds_semantic_closure_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_closure.v1.lyra")
        .expect("P01-024 semantic closure contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-CLOSURE-CONTRACT v1"));
    assert!(contract.contains("task=P01-024"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-CLOSURE-GATE v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0053_semantic_closure.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-closure-check.rs"));
    assert!(Path::new("ops/p01/control/semantic_closure_law.v1.lyra").exists());
}

#[test]
fn p01_024_valid_surface_matches_required_inventory() {
    let surface = parse_semantic_closure_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P01_SEMANTIC_CLOSURE_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_SEMANTIC_CLOSURE_RULES.len());
    assert_eq!(surface.tasks.len(), REQUIRED_SEMANTIC_CLOSURE_TASKS.len());
    assert_eq!(
        surface.outputs.len(),
        REQUIRED_SEMANTIC_CLOSURE_OUTPUTS.len()
    );
    assert_eq!(surface.proofs.len(), REQUIRED_SEMANTIC_CLOSURE_PROOFS.len());
}

#[test]
fn p01_024_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_semantic_closure_surface(VALID);
    let golden = fs::read_to_string("goldens/p01/valid_semantic_closure.receipt")
        .expect("semantic closure golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
