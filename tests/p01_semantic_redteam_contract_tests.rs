use std::fs;
use std::path::Path;

use lyra_phase0::p01::{
    parse_semantic_redteam_surface, validate_semantic_redteam_surface,
    P01_SEMANTIC_REDTEAM_CONTRACT, REQUIRED_SEMANTIC_REDTEAM_PROOFS,
    REQUIRED_SEMANTIC_REDTEAM_RULES, REQUIRED_SEMANTIC_REDTEAM_SCENARIOS,
    REQUIRED_SEMANTIC_ROLLBACK_PATHS,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_redteam_inputs/valid_semantic_redteam.lyra");

#[test]
fn p01_023_contract_binds_semantic_redteam_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_redteam.v1.lyra")
        .expect("P01-023 semantic redteam contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-REDTEAM-CONTRACT v1"));
    assert!(contract.contains("task=P01-023"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-REDTEAM-ROLLBACK v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0052_semantic_redteam.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-redteam-check.rs"));
    assert!(Path::new("ops/p01/control/semantic_redteam_law.v1.lyra").exists());
}

#[test]
fn p01_023_valid_surface_matches_required_inventory() {
    let surface = parse_semantic_redteam_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P01_SEMANTIC_REDTEAM_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_SEMANTIC_REDTEAM_RULES.len());
    assert_eq!(
        surface.scenarios.len(),
        REQUIRED_SEMANTIC_REDTEAM_SCENARIOS.len()
    );
    assert_eq!(
        surface.rollbacks.len(),
        REQUIRED_SEMANTIC_ROLLBACK_PATHS.len()
    );
    assert_eq!(surface.proofs.len(), REQUIRED_SEMANTIC_REDTEAM_PROOFS.len());
}

#[test]
fn p01_023_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_semantic_redteam_surface(VALID);
    let golden = fs::read_to_string("goldens/p01/valid_semantic_redteam.receipt")
        .expect("semantic redteam golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
