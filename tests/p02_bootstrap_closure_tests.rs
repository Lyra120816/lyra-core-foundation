use lyra_phase0::p02::{
    bootstrap_closure_artifacts_bind_paths, bootstrap_closure_no_forbidden_descriptor_claims,
    bootstrap_closure_outputs_remain_open, bootstrap_closure_proofs_bind_registry,
    bootstrap_closure_receipts_cover_p02_001_through_p02_024,
    bootstrap_closure_tasks_bind_receipts, deterministic_bootstrap_closure_gate_report,
    validate_bootstrap_closure_surface, ErrorCode, REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS,
    REQUIRED_BOOTSTRAP_CLOSURE_PROOFS, REQUIRED_BOOTSTRAP_CLOSURE_TASKS,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/valid_bootstrap_closure.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_missing_rule.lyra");
const INVALID_MISSING_TASK: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_missing_task.lyra");
const INVALID_DUPLICATE_TASK: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_duplicate_task.lyra");
const INVALID_MISSING_OUTPUT: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_missing_output.lyra");
const INVALID_DUPLICATE_OUTPUT: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_duplicate_output.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_missing_proof.lyra");
const INVALID_UNBOUND_PROOF_TASK: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_unbound_proof_task.lyra");
const INVALID_UNBOUND_OUTPUT_DEPENDENCY: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_unbound_output_dependency.lyra");
const INVALID_NETWORK: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_network_required.lyra");
const INVALID_UNRECEIPTED: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_unreceipted_closure.lyra");
const INVALID_GLOBAL_TRUE: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_global_closure_true.lyra");
const INVALID_PHASE_CLOSED: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_phase_closed_claim.lyra");
const INVALID_DOCS_ONLY: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_docs_only_claim.lyra");
const INVALID_OUTPUT_PREMATURE: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_output_premature.lyra");
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_missing_receipt_binding.lyra");
const INVALID_UNKNOWN_COMMAND: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_unknown_command.lyra");
const INVALID_DESCRIPTOR_DRIFT: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_descriptor_drift.lyra");
const INVALID_MISSING_NEXT_FRONTIER: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_missing_next_frontier.lyra");
const INVALID_MISSING_PERMIT: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_missing_permit.lyra");
const INVALID_MISSING_GLOBAL_FORBID: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_missing_global_forbid.lyra");
const INVALID_MALFORMED_FIELD_MAP: &str =
    include_str!("../fixtures/p02/bootstrap_closure_inputs/invalid_malformed_field_map.lyra");

fn assert_rejects(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_bootstrap_closure_surface(input);
    assert!(!verdict.accepted);
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_bootstrap_closure_surface() {
    let (verdict, receipt) = validate_bootstrap_closure_surface(VALID);
    assert!(verdict.accepted, "{:?}", verdict.errors);
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn bootstrap_closure_deterministic_report_is_stable_and_counted() {
    let tasks = REQUIRED_BOOTSTRAP_CLOSURE_TASKS
        .iter()
        .map(|id| {
            (
                id.to_string(),
                "closure_gate".to_string(),
                vec!["receipts/p02/pass_0082_bootstrap_closure.receipt".to_string()],
                vec!["lyra-p02-bootstrap-closure-check".to_string()],
                vec!["interfaces/p02/contracts/bootstrap_closure.v1.lyra".to_string()],
                "bounded_closed".to_string(),
            )
        })
        .collect::<Vec<_>>();
    let outputs = REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS
        .iter()
        .map(|id| {
            (
                id.to_string(),
                "dependency_matrix".to_string(),
                "ops/p02/closure/p02_x01_dependency_matrix_gate.v1.lyra".to_string(),
                vec!["P02-024".to_string()],
                vec!["receipts/p02/pass_0082_bootstrap_closure.receipt".to_string()],
                "blocked".to_string(),
            )
        })
        .collect::<Vec<_>>();
    let proofs = REQUIRED_BOOTSTRAP_CLOSURE_PROOFS
        .iter()
        .map(|id| {
            (
                id.to_string(),
                "receipt_chain".to_string(),
                vec!["P02-024".to_string()],
                vec!["P02-X01".to_string()],
                vec!["receipts/p02/pass_0082_bootstrap_closure.receipt".to_string()],
                vec!["lyra-p02-bootstrap-closure-check".to_string()],
                vec!["bounded_primary_closure".to_string()],
                vec![
                    "global_closure".to_string(),
                    "unreceipted_closure".to_string(),
                ],
                "artifact_emitted".to_string(),
            )
        })
        .collect::<Vec<_>>();
    let report = deterministic_bootstrap_closure_gate_report(&tasks, &outputs, &proofs);
    assert_eq!(report.task_count, REQUIRED_BOOTSTRAP_CLOSURE_TASKS.len());
    assert_eq!(
        report.output_gate_count,
        REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS.len()
    );
    assert_eq!(report.proof_count, REQUIRED_BOOTSTRAP_CLOSURE_PROOFS.len());
    assert!(report.gate_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_bootstrap_closure_gaps() {
    assert_rejects(INVALID_MISSING_RULE, ErrorCode::MissingClosureRule);
    assert_rejects(INVALID_MISSING_TASK, ErrorCode::MissingClosureTask);
    assert_rejects(INVALID_DUPLICATE_TASK, ErrorCode::DuplicateClosureTask);
    assert_rejects(INVALID_MISSING_OUTPUT, ErrorCode::MissingClosureOutputGate);
    assert_rejects(
        INVALID_DUPLICATE_OUTPUT,
        ErrorCode::DuplicateClosureOutputGate,
    );
    assert_rejects(INVALID_MISSING_PROOF, ErrorCode::MissingClosureProof);
    assert_rejects(INVALID_UNBOUND_PROOF_TASK, ErrorCode::ClosureProofUnbound);
    assert_rejects(
        INVALID_UNBOUND_OUTPUT_DEPENDENCY,
        ErrorCode::ClosureProofUnbound,
    );
    assert_rejects(INVALID_NETWORK, ErrorCode::ClosureNetworkDependency);
    assert_rejects(INVALID_UNRECEIPTED, ErrorCode::ClosureUnreceipted);
    assert_rejects(INVALID_GLOBAL_TRUE, ErrorCode::UnsupportedGlobalClosure);
    assert_rejects(INVALID_PHASE_CLOSED, ErrorCode::UnsupportedGlobalClosure);
    assert_rejects(INVALID_DOCS_ONLY, ErrorCode::ClosureDocsOnly);
    assert_rejects(INVALID_OUTPUT_PREMATURE, ErrorCode::ClosureOutputPremature);
    assert_rejects(INVALID_MISSING_RECEIPT, ErrorCode::InvalidClosureTask);
    assert_rejects(INVALID_UNKNOWN_COMMAND, ErrorCode::InvalidClosureTask);
    assert_rejects(INVALID_DESCRIPTOR_DRIFT, ErrorCode::InvalidClosureTask);
    assert_rejects(
        INVALID_MISSING_NEXT_FRONTIER,
        ErrorCode::InvalidClosureOutputGate,
    );
    assert_rejects(INVALID_MISSING_PERMIT, ErrorCode::ClosureFormulaViolation);
    assert_rejects(
        INVALID_MISSING_GLOBAL_FORBID,
        ErrorCode::UnsupportedGlobalClosure,
    );
    assert_rejects(INVALID_MALFORMED_FIELD_MAP, ErrorCode::InvalidClosureTask);
}

#[test]
fn bootstrap_closure_descriptor_registry_is_bound() {
    assert!(bootstrap_closure_tasks_bind_receipts());
    assert!(bootstrap_closure_outputs_remain_open());
    assert!(bootstrap_closure_proofs_bind_registry());
    assert!(bootstrap_closure_artifacts_bind_paths());
    assert!(bootstrap_closure_no_forbidden_descriptor_claims());
    assert!(bootstrap_closure_receipts_cover_p02_001_through_p02_024());
}

#[test]
fn required_bootstrap_closure_inventory_counts_are_bound() {
    assert_eq!(REQUIRED_BOOTSTRAP_CLOSURE_TASKS.len(), 24);
    assert_eq!(REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS.len(), 5);
    assert_eq!(REQUIRED_BOOTSTRAP_CLOSURE_PROOFS.len(), 6);
}
