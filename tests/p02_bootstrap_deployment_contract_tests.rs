use std::fs;
use std::path::Path;

use lyra_phase0::p02::{
    parse_bootstrap_deployment_surface, validate_bootstrap_deployment_surface,
    P02_BOOTSTRAP_DEPLOYMENT_CONTRACT, REQUIRED_BOOTSTRAP_COMPLIANCE_HOOKS,
    REQUIRED_BOOTSTRAP_DEPLOYMENT_PROOFS, REQUIRED_BOOTSTRAP_DEPLOYMENT_RULES,
    REQUIRED_BOOTSTRAP_DEPLOYMENT_TARGETS, REQUIRED_BOOTSTRAP_RELEASE_EVIDENCE,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_deployment_inputs/valid_bootstrap_deployment.lyra");

#[test]
fn p02_020_contract_binds_deployment_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p02/contracts/bootstrap_deployment.v1.lyra")
        .expect("P02-020 bootstrap deployment contract must exist");
    assert!(contract.starts_with("LYRA-P02-BOOTSTRAP-DEPLOYMENT-CONTRACT v1"));
    assert!(contract.contains("task=P02-020"));
    assert!(contract.contains("surface=LYRA-P02-BOOTSTRAP-DEPLOYMENT-HOOKS v1"));
    assert!(contract.contains("receipt=receipts/p02/pass_0078_bootstrap_deployment.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p02-bootstrap-deployment-check.rs"));
    assert!(Path::new("ops/p02/deployment/bootstrap_deployment.v1.lyra").exists());
    assert!(Path::new("products/p02/bootstrap_deployment_manifest.v1.lyra").exists());
}

#[test]
fn p02_020_valid_surface_matches_required_inventory() {
    let surface = parse_bootstrap_deployment_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P02_BOOTSTRAP_DEPLOYMENT_CONTRACT);
    assert_eq!(
        surface.rules.len(),
        REQUIRED_BOOTSTRAP_DEPLOYMENT_RULES.len()
    );
    assert_eq!(
        surface.targets.len(),
        REQUIRED_BOOTSTRAP_DEPLOYMENT_TARGETS.len()
    );
    assert_eq!(
        surface.hooks.len(),
        REQUIRED_BOOTSTRAP_COMPLIANCE_HOOKS.len()
    );
    assert_eq!(
        surface.evidence.len(),
        REQUIRED_BOOTSTRAP_RELEASE_EVIDENCE.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_BOOTSTRAP_DEPLOYMENT_PROOFS.len()
    );
}

#[test]
fn p02_020_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_bootstrap_deployment_surface(VALID);
    let golden = fs::read_to_string("goldens/p02/valid_bootstrap_deployment.receipt")
        .expect("bootstrap deployment golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
