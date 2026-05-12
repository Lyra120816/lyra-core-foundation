use std::{fs, path::Path};

use lyra_phase0::p02::{
    parse_bootstrap_dependency_matrix_surface, validate_bootstrap_dependency_matrix_surface,
    P02_BOOTSTRAP_DEPENDENCY_MATRIX_CONTRACT, REQUIRED_BOOTSTRAP_DEPENDENCY_BLOCKERS,
    REQUIRED_BOOTSTRAP_DEPENDENCY_NODES, REQUIRED_BOOTSTRAP_DEPENDENCY_PROOFS,
    REQUIRED_BOOTSTRAP_DEPENDENCY_RULES, REQUIRED_BOOTSTRAP_PARALLEL_LANES,
};

const VALID: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/valid_bootstrap_dependency_matrix.lyra"
);

#[test]
fn p02_x01_contract_binds_dependency_matrix_fixture_and_receipt() {
    let contract =
        fs::read_to_string("interfaces/p02/contracts/bootstrap_dependency_matrix.v1.lyra")
            .expect("contract readable");
    assert!(contract.contains(P02_BOOTSTRAP_DEPENDENCY_MATRIX_CONTRACT));
    assert!(contract.contains("surface=fixtures/p02/bootstrap_dependency_matrix_inputs/valid_bootstrap_dependency_matrix.lyra"));
    assert!(contract.contains("receipt=receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt"));
    assert!(contract.contains("next_frontier=P02-X02"));
    assert!(Path::new("ops/p02/closure/p02_x01_dependency_matrix_gate.v1.lyra").exists());
    assert!(Path::new("products/p02/bootstrap_dependency_matrix_manifest.v1.lyra").exists());
    assert!(Path::new("shells/p02/bootstrap_dependency_matrix_shell.v1.lyra").exists());
}

#[test]
fn p02_x01_valid_fixture_covers_required_sets() {
    let surface = parse_bootstrap_dependency_matrix_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.phase, "P02");
    assert_eq!(surface.task, "P02-X01");
    assert_eq!(surface.global_closure, "false");
    assert_eq!(surface.next_frontier, "P02-X02");
    assert_eq!(
        surface.rules.len(),
        REQUIRED_BOOTSTRAP_DEPENDENCY_RULES.len()
    );
    assert_eq!(
        surface.nodes.len(),
        REQUIRED_BOOTSTRAP_DEPENDENCY_NODES.len()
    );
    assert_eq!(
        surface.blockers.len(),
        REQUIRED_BOOTSTRAP_DEPENDENCY_BLOCKERS.len()
    );
    assert_eq!(surface.lanes.len(), REQUIRED_BOOTSTRAP_PARALLEL_LANES.len());
    assert_eq!(
        surface.proofs.len(),
        REQUIRED_BOOTSTRAP_DEPENDENCY_PROOFS.len()
    );
}

#[test]
fn p02_x01_golden_receipt_matches_current_validator() {
    let (_verdict, receipt) = validate_bootstrap_dependency_matrix_surface(VALID);
    let golden = fs::read_to_string("goldens/p02/valid_bootstrap_dependency_matrix.receipt")
        .expect("golden readable");
    assert_eq!(receipt.to_text(), golden);
}
