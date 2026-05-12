use lyra_phase0::p02::{
    bootstrap_dependency_blockers_bind_required_nodes,
    bootstrap_dependency_matrix_artifacts_bind_paths,
    bootstrap_dependency_matrix_no_forbidden_descriptor_claims,
    bootstrap_dependency_matrix_receipts_cover_p02_001_through_p02_x01,
    bootstrap_dependency_nodes_bind_owner_roots, bootstrap_dependency_proofs_bind_registry,
    bootstrap_parallel_lanes_bind_existing_nodes, deterministic_bootstrap_dependency_matrix_report,
    validate_bootstrap_dependency_matrix_surface, ErrorCode,
    REQUIRED_BOOTSTRAP_DEPENDENCY_BLOCKERS, REQUIRED_BOOTSTRAP_DEPENDENCY_NODES,
    REQUIRED_BOOTSTRAP_DEPENDENCY_PROOFS, REQUIRED_BOOTSTRAP_PARALLEL_LANES,
};

const VALID: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/valid_bootstrap_dependency_matrix.lyra"
);
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_missing_rule.lyra");
const INVALID_MISSING_NODE: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_missing_node.lyra");
const INVALID_DUPLICATE_NODE: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_duplicate_node.lyra");
const INVALID_MISSING_BLOCKER: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_missing_blocker.lyra");
const INVALID_DUPLICATE_BLOCKER: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_duplicate_blocker.lyra"
);
const INVALID_MISSING_LANE: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_missing_lane.lyra");
const INVALID_DUPLICATE_LANE: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_duplicate_lane.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_missing_proof.lyra");
const INVALID_UNBOUND_NODE: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_unbound_node_dependency.lyra"
);
const INVALID_UNBOUND_BLOCKER: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_unbound_blocker_requirement.lyra"
);
const INVALID_UNBOUND_LANE: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_unbound_lane_task.lyra"
);
const INVALID_UNBOUND_PROOF: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_unbound_proof_lane.lyra"
);
const INVALID_GLOBAL: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_global_closure_true.lyra"
);
const INVALID_NEXT: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_wrong_next_frontier.lyra"
);
const INVALID_NETWORK: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_network_required.lyra"
);
const INVALID_DOCS_ONLY: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_docs_only.lyra");
const INVALID_RECEIPTLESS: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_receiptless_parallel.lyra"
);
const INVALID_OWNER_ROOT: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_owner_root.lyra");
const INVALID_PARALLEL_X: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_parallel_x_outputs.lyra"
);
const INVALID_COMMAND: &str =
    include_str!("../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_missing_command.lyra");
const INVALID_DRIFT: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_descriptor_drift.lyra"
);
const INVALID_RECEIPT_PATH: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_missing_receipt_path.lyra"
);
const INVALID_MALFORMED: &str = include_str!(
    "../fixtures/p02/bootstrap_dependency_matrix_inputs/invalid_malformed_field_map.lyra"
);

fn assert_rejects(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_bootstrap_dependency_matrix_surface(input);
    assert!(!verdict.accepted);
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_bootstrap_dependency_matrix_surface() {
    let (verdict, receipt) = validate_bootstrap_dependency_matrix_surface(VALID);
    assert!(verdict.accepted, "{:?}", verdict.errors);
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn bootstrap_dependency_matrix_report_is_stable_and_counted() {
    let nodes = vec![(
        "P02-X01".to_string(),
        "dependency_matrix".to_string(),
        "artifact_emitted".to_string(),
        vec!["P02-024".to_string()],
        vec!["P02-X02".to_string()],
        "ops/p02".to_string(),
    )];
    let blockers = vec![(
        "x02_waits_for_dependency_matrix".to_string(),
        "P02-X02".to_string(),
        "hard".to_string(),
        vec!["P02-X02".to_string()],
        vec!["P02-X01".to_string()],
        "active".to_string(),
    )];
    let lanes = vec![(
        "lane_x_outputs_serial".to_string(),
        "extension_outputs".to_string(),
        vec!["P02-X01".to_string(), "P02-X02".to_string()],
        vec!["P02-024".to_string()],
        "no".to_string(),
        "strict_serial_x01_to_x05".to_string(),
    )];
    let proofs = vec![(
        "matrix_next_frontier_proof".to_string(),
        vec!["P02-X01".to_string(), "P02-X02".to_string()],
        vec!["x02_waits_for_dependency_matrix".to_string()],
        vec!["lane_x_outputs_serial".to_string()],
        vec!["receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt".to_string()],
        vec!["lyra-p02-bootstrap-dependency-matrix-check".to_string()],
        "artifact_emitted".to_string(),
    )];
    let report =
        deterministic_bootstrap_dependency_matrix_report(&nodes, &blockers, &lanes, &proofs);
    assert_eq!(report.node_count, 1);
    assert_eq!(report.blocker_count, 1);
    assert_eq!(report.lane_count, 1);
    assert_eq!(report.proof_count, 1);
    assert!(report.matrix_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_dependency_matrix_gaps_and_unsafe_claims() {
    assert_rejects(INVALID_MISSING_RULE, ErrorCode::MissingClosureRule);
    assert_rejects(INVALID_MISSING_NODE, ErrorCode::MissingClosureTask);
    assert_rejects(INVALID_DUPLICATE_NODE, ErrorCode::DuplicateClosureTask);
    assert_rejects(INVALID_MISSING_BLOCKER, ErrorCode::MissingBlockerBinding);
    assert_rejects(INVALID_DUPLICATE_BLOCKER, ErrorCode::DuplicateEntry);
    assert_rejects(INVALID_MISSING_LANE, ErrorCode::MissingBlockerBinding);
    assert_rejects(INVALID_DUPLICATE_LANE, ErrorCode::DuplicateEntry);
    assert_rejects(INVALID_MISSING_PROOF, ErrorCode::MissingClosureProof);
    assert_rejects(INVALID_UNBOUND_NODE, ErrorCode::ClosureProofUnbound);
    assert_rejects(INVALID_UNBOUND_BLOCKER, ErrorCode::ClosureProofUnbound);
    assert_rejects(INVALID_UNBOUND_LANE, ErrorCode::ClosureProofUnbound);
    assert_rejects(INVALID_UNBOUND_PROOF, ErrorCode::ClosureProofUnbound);
    assert_rejects(INVALID_GLOBAL, ErrorCode::UnsupportedGlobalClosure);
    assert_rejects(INVALID_NEXT, ErrorCode::MissingBlockerBinding);
    assert_rejects(INVALID_NETWORK, ErrorCode::ClosureNetworkDependency);
    assert_rejects(INVALID_DOCS_ONLY, ErrorCode::ClosureDocsOnly);
    assert_rejects(INVALID_RECEIPTLESS, ErrorCode::ClosureUnreceipted);
    assert_rejects(INVALID_OWNER_ROOT, ErrorCode::MisplacedOwnerRoot);
    assert_rejects(INVALID_PARALLEL_X, ErrorCode::ClosureFormulaViolation);
    assert_rejects(INVALID_COMMAND, ErrorCode::MissingCommandRecord);
    assert_rejects(INVALID_DRIFT, ErrorCode::ClosureDriftAccepted);
    assert_rejects(INVALID_RECEIPT_PATH, ErrorCode::ClosureUnreceipted);
    assert_rejects(INVALID_MALFORMED, ErrorCode::InvalidClosureTask);
}

#[test]
fn dependency_matrix_descriptor_registry_is_bound() {
    assert!(bootstrap_dependency_nodes_bind_owner_roots());
    assert!(bootstrap_dependency_blockers_bind_required_nodes());
    assert!(bootstrap_parallel_lanes_bind_existing_nodes());
    assert!(bootstrap_dependency_proofs_bind_registry());
    assert!(bootstrap_dependency_matrix_artifacts_bind_paths());
    assert!(bootstrap_dependency_matrix_no_forbidden_descriptor_claims());
    assert!(bootstrap_dependency_matrix_receipts_cover_p02_001_through_p02_x01());
}

#[test]
fn required_dependency_inventory_counts_are_bound() {
    assert_eq!(REQUIRED_BOOTSTRAP_DEPENDENCY_NODES.len(), 29);
    assert_eq!(REQUIRED_BOOTSTRAP_DEPENDENCY_BLOCKERS.len(), 8);
    assert_eq!(REQUIRED_BOOTSTRAP_PARALLEL_LANES.len(), 6);
    assert_eq!(REQUIRED_BOOTSTRAP_DEPENDENCY_PROOFS.len(), 6);
}
