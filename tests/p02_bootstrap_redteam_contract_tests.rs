use std::fs;
use std::path::Path;

use lyra_phase0::p02::{
    parse_bootstrap_redteam_surface, validate_bootstrap_redteam_surface,
    P02_BOOTSTRAP_REDTEAM_CONTRACT, REQUIRED_BOOTSTRAP_REDTEAM_PROOFS,
    REQUIRED_BOOTSTRAP_REDTEAM_RULES, REQUIRED_BOOTSTRAP_REDTEAM_SCENARIOS,
    REQUIRED_BOOTSTRAP_ROLLBACK_PATHS,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_redteam_inputs/valid_bootstrap_redteam.lyra");

#[test]
fn p02_023_contract_binds_bootstrap_redteam_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p02/contracts/bootstrap_redteam.v1.lyra")
        .expect("P02-023 bootstrap redteam contract must exist");

    assert!(contract.starts_with("LYRA-P02-BOOTSTRAP-REDTEAM-CONTRACT v1"));
    assert!(contract.contains("task=P02-023"));
    assert!(contract.contains("surface=LYRA-P02-BOOTSTRAP-REDTEAM-ROLLBACK-LAW v1"));
    assert!(contract.contains("receipt=receipts/p02/pass_0081_bootstrap_redteam.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p02-bootstrap-redteam-check.rs"));
    assert!(Path::new("ops/p02/control/bootstrap_redteam_rollback_law.v1.lyra").exists());
}

#[test]
fn p02_023_valid_surface_matches_required_inventory() {
    let surface = parse_bootstrap_redteam_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P02_BOOTSTRAP_REDTEAM_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_BOOTSTRAP_REDTEAM_RULES.len());
    assert_eq!(
        surface.scenarios.len(),
        REQUIRED_BOOTSTRAP_REDTEAM_SCENARIOS.len()
    );
    assert_eq!(
        surface.rollbacks.len(),
        REQUIRED_BOOTSTRAP_ROLLBACK_PATHS.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_BOOTSTRAP_REDTEAM_PROOFS.len()
    );
}

#[test]
fn p02_023_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_bootstrap_redteam_surface(VALID);
    let golden = fs::read_to_string("goldens/p02/valid_bootstrap_redteam.receipt")
        .expect("bootstrap redteam golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
