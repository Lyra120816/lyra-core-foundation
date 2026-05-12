use std::{fs, path::Path};

use lyra_phase0::p02::{
    parse_bootstrap_closure_surface, validate_bootstrap_closure_surface,
    P02_BOOTSTRAP_CLOSURE_CONTRACT, REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS,
    REQUIRED_BOOTSTRAP_CLOSURE_PROOFS, REQUIRED_BOOTSTRAP_CLOSURE_RULES,
    REQUIRED_BOOTSTRAP_CLOSURE_TASKS,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/valid_bootstrap_closure.lyra");

#[test]
fn p02_024_contract_binds_bootstrap_closure_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p02/contracts/bootstrap_closure.v1.lyra")
        .expect("contract readable");
    assert!(contract.contains(P02_BOOTSTRAP_CLOSURE_CONTRACT));
    assert!(contract
        .contains("surface=fixtures/p02/bootstrap_closure_inputs/valid_bootstrap_closure.lyra"));
    assert!(contract.contains("receipt=receipts/p02/pass_0082_bootstrap_closure.receipt"));
    assert!(contract.contains("next_frontier=P02-X01"));
    assert!(Path::new("ops/p02/control/bootstrap_closure_gate_law.v1.lyra").exists());
    assert!(Path::new("products/p02/bootstrap_closure_manifest.v1.lyra").exists());
    assert!(Path::new("shells/p02/bootstrap_closure_shell.v1.lyra").exists());
}

#[test]
fn p02_024_valid_fixture_covers_required_sets() {
    let surface = parse_bootstrap_closure_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.phase, "P02");
    assert_eq!(surface.task, "P02-024");
    assert_eq!(surface.bounded_closure, "true");
    assert_eq!(surface.global_closure, "false");
    assert_eq!(surface.next_frontier, "P02-X01");
    assert_eq!(surface.rules.len(), REQUIRED_BOOTSTRAP_CLOSURE_RULES.len());
    assert_eq!(surface.tasks.len(), REQUIRED_BOOTSTRAP_CLOSURE_TASKS.len());
    assert_eq!(
        surface.outputs.len(),
        REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS.len()
    );
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_BOOTSTRAP_CLOSURE_PROOFS.len()
    );
}

#[test]
fn p02_024_golden_receipt_matches_current_validator() {
    let (_verdict, receipt) = validate_bootstrap_closure_surface(VALID);
    let golden =
        fs::read_to_string("goldens/p02/valid_bootstrap_closure.receipt").expect("golden readable");
    assert_eq!(receipt.to_text(), golden);
}
