use std::fs;
use std::path::Path;

use lyra_phase0::p01::{
    parse_semantic_deployment_surface, validate_semantic_deployment_surface,
    P01_SEMANTIC_DEPLOYMENT_CONTRACT, REQUIRED_SEMANTIC_COMPLIANCE_HOOKS,
    REQUIRED_SEMANTIC_DEPLOYMENT_PROOFS, REQUIRED_SEMANTIC_DEPLOYMENT_RULES,
    REQUIRED_SEMANTIC_DEPLOYMENT_TARGETS, REQUIRED_SEMANTIC_RELEASE_EVIDENCE,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_deployment_inputs/valid_semantic_deployment.lyra");

#[test]
fn p01_020_contract_binds_deployment_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_deployment.v1.lyra")
        .expect("P01-020 semantic deployment contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-DEPLOYMENT-CONTRACT v1"));
    assert!(contract.contains("task=P01-020"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-DEPLOYMENT-HOOKS v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0049_semantic_deployment.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-deployment-check.rs"));
    assert!(Path::new("ops/p01/control/semantic_deployment_law.v1.lyra").exists());
}

#[test]
fn p01_020_valid_surface_matches_required_inventory() {
    let surface = parse_semantic_deployment_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P01_SEMANTIC_DEPLOYMENT_CONTRACT);
    assert_eq!(
        surface.rules.len(),
        REQUIRED_SEMANTIC_DEPLOYMENT_RULES.len()
    );
    assert_eq!(
        surface.targets.len(),
        REQUIRED_SEMANTIC_DEPLOYMENT_TARGETS.len()
    );
    assert_eq!(
        surface.hooks.len(),
        REQUIRED_SEMANTIC_COMPLIANCE_HOOKS.len()
    );
    assert_eq!(
        surface.evidence.len(),
        REQUIRED_SEMANTIC_RELEASE_EVIDENCE.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_SEMANTIC_DEPLOYMENT_PROOFS.len()
    );
}

#[test]
fn p01_020_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_semantic_deployment_surface(VALID);
    let golden = fs::read_to_string("goldens/p01/valid_semantic_deployment.receipt")
        .expect("semantic deployment golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
