use std::fs;
use std::path::Path;

use lyra_phase0::p02::{
    parse_bootstrap_economics_surface, validate_bootstrap_economics_surface,
    P02_BOOTSTRAP_ECONOMICS_CONTRACT, REQUIRED_BOOTSTRAP_ECONOMICS_FRAMES,
    REQUIRED_BOOTSTRAP_ECONOMICS_PROOFS, REQUIRED_BOOTSTRAP_ECONOMICS_RULES,
    REQUIRED_BOOTSTRAP_PUBLIC_INTEREST_OUTPUTS,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_economics_inputs/valid_bootstrap_economics.lyra");

#[test]
fn p02_022_contract_binds_bootstrap_economics_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p02/contracts/bootstrap_economics.v1.lyra")
        .expect("P02-022 bootstrap economics contract must exist");

    assert!(contract.starts_with("LYRA-P02-BOOTSTRAP-ECONOMICS-CONTRACT v1"));
    assert!(contract.contains("task=P02-022"));
    assert!(contract.contains("surface=LYRA-P02-BOOTSTRAP-ECONOMICS-PUBLIC-INTEREST v1"));
    assert!(contract.contains("receipt=receipts/p02/pass_0080_bootstrap_economics.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p02-bootstrap-economics-check.rs"));
    assert!(Path::new("ops/p02/control/bootstrap_economics_law.v1.lyra").exists());
}

#[test]
fn p02_022_valid_surface_matches_required_inventory() {
    let surface = parse_bootstrap_economics_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P02_BOOTSTRAP_ECONOMICS_CONTRACT);
    assert_eq!(
        surface.rules.len(),
        REQUIRED_BOOTSTRAP_ECONOMICS_RULES.len()
    );
    assert_eq!(
        surface.frames.len(),
        REQUIRED_BOOTSTRAP_ECONOMICS_FRAMES.len()
    );
    assert_eq!(
        surface.outputs.len(),
        REQUIRED_BOOTSTRAP_PUBLIC_INTEREST_OUTPUTS.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_BOOTSTRAP_ECONOMICS_PROOFS.len()
    );
}

#[test]
fn p02_022_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_bootstrap_economics_surface(VALID);
    let golden = fs::read_to_string("goldens/p02/valid_bootstrap_economics.receipt")
        .expect("bootstrap economics golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
