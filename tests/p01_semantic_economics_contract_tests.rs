use std::fs;
use std::path::Path;

use lyra_phase0::p01::{
    parse_semantic_economics_surface, validate_semantic_economics_surface,
    P01_SEMANTIC_ECONOMICS_CONTRACT, REQUIRED_SEMANTIC_ECONOMICS_FRAMES,
    REQUIRED_SEMANTIC_ECONOMICS_PROOFS, REQUIRED_SEMANTIC_ECONOMICS_RULES,
    REQUIRED_SEMANTIC_PUBLIC_INTEREST_OUTPUTS,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/valid_semantic_economics.lyra");

#[test]
fn p01_022_contract_binds_economics_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_economics.v1.lyra")
        .expect("P01-022 semantic economics contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-ECONOMICS-CONTRACT v1"));
    assert!(contract.contains("task=P01-022"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-ECONOMICS-PUBLIC-INTEREST v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0051_semantic_economics.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-economics-check.rs"));
    assert!(Path::new("ops/p01/control/semantic_economics_law.v1.lyra").exists());
}

#[test]
fn p01_022_valid_surface_matches_required_inventory() {
    let surface = parse_semantic_economics_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P01_SEMANTIC_ECONOMICS_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_SEMANTIC_ECONOMICS_RULES.len());
    assert_eq!(
        surface.frames.len(),
        REQUIRED_SEMANTIC_ECONOMICS_FRAMES.len()
    );
    assert_eq!(
        surface.outputs.len(),
        REQUIRED_SEMANTIC_PUBLIC_INTEREST_OUTPUTS.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_SEMANTIC_ECONOMICS_PROOFS.len()
    );
}

#[test]
fn p01_022_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_semantic_economics_surface(VALID);
    let golden = fs::read_to_string("goldens/p01/valid_semantic_economics.receipt")
        .expect("semantic economics golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
