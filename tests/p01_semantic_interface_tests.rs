use lyra_phase0::p01::{
    deterministic_semantic_interface_suite_report, validate_semantic_interface_surface, ErrorCode,
    REQUIRED_SEMANTIC_INTERFACE_ARTIFACTS, REQUIRED_SEMANTIC_INTERFACE_COMMANDS,
    REQUIRED_SEMANTIC_INTERFACE_EXAMPLES, REQUIRED_SEMANTIC_INTERFACE_PROOFS,
    REQUIRED_SEMANTIC_INTERFACE_WORKFLOWS,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/valid_semantic_interface.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_missing_rule.lyra");
const INVALID_MISSING_COMMAND: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_missing_command.lyra");
const INVALID_DUPLICATE_COMMAND: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_duplicate_command.lyra");
const INVALID_UNKNOWN_WORKFLOW_COMMAND: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_unknown_workflow_command.lyra");
const INVALID_MISSING_WORKFLOW: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_missing_workflow.lyra");
const INVALID_MISSING_EXAMPLE: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_missing_example.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_missing_proof.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_MANUAL_ONLY: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_manual_only_interface.lyra");
const INVALID_NETWORK_REQUIRED: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_network_required.lyra");
const INVALID_INTERFACE_DRIFT: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_interface_drift.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p01/semantic_interface_inputs/invalid_phase_closure_claim.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_semantic_interface_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_semantic_interface_surface() {
    let (verdict, receipt) = validate_semantic_interface_surface(VALID);
    assert!(
        verdict.accepted,
        "valid semantic interface rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn semantic_interface_deterministic_report_is_stable_and_counted() {
    let commands = vec![
        (
            "z_command".to_string(),
            "src/bin/lyra-p01-z.rs".to_string(),
            "LYRA-P01-Z v1".to_string(),
            "fixtures/p01/z.lyra".to_string(),
            "receipts/p01/z.receipt".to_string(),
            vec!["receipts/p01/z.receipt".to_string()],
            vec!["operator".to_string()],
            vec!["core_ir".to_string()],
            "artifact_emitted".to_string(),
        ),
        (
            "a_command".to_string(),
            "src/bin/lyra-p01-a.rs".to_string(),
            "LYRA-P01-A v1".to_string(),
            "fixtures/p01/a.lyra".to_string(),
            "receipts/p01/a.receipt".to_string(),
            vec!["receipts/p01/a.receipt".to_string()],
            vec!["developer".to_string()],
            vec!["semantic_atoms".to_string()],
            "artifact_emitted".to_string(),
        ),
    ];
    let workflows = vec![(
        "a_workflow".to_string(),
        "001".to_string(),
        vec!["a_command".to_string()],
        vec!["semantic_atoms".to_string()],
        vec!["a_example".to_string()],
        vec!["manual_only".to_string()],
        "execution_proven".to_string(),
    )];
    let examples = vec![(
        "a_example".to_string(),
        "examples/p01/operator/a.lyra".to_string(),
        vec!["a_command".to_string()],
        vec!["receipts/p01/a.receipt".to_string()],
        "accepted".to_string(),
        "artifact_emitted".to_string(),
    )];
    let proofs = vec![(
        "a_proof".to_string(),
        "command".to_string(),
        vec!["a_command".to_string()],
        vec!["a_workflow".to_string()],
        vec!["a_example".to_string()],
        vec!["receipts/p01/a.receipt".to_string()],
        vec!["manual_only".to_string()],
        "execution_proven".to_string(),
    )];
    let artifacts = vec![(
        "a_artifact".to_string(),
        "k0".to_string(),
        "k0/determinism/src/semantic_interface.rs".to_string(),
        "deterministic_report".to_string(),
        vec!["a_command".to_string()],
        "artifact_emitted".to_string(),
    )];
    let left = deterministic_semantic_interface_suite_report(
        &commands, &workflows, &examples, &proofs, &artifacts,
    );
    let right = deterministic_semantic_interface_suite_report(
        &commands, &workflows, &examples, &proofs, &artifacts,
    );
    assert_eq!(left, right);
    assert_eq!(left.command_count, 2);
    assert_eq!(left.workflow_count, 1);
    assert_eq!(left.commands[0].id, "a_command");
    assert_eq!(left.workflows[0].order, "001");
}

#[test]
fn required_semantic_interface_inventory_is_broad() {
    assert_eq!(REQUIRED_SEMANTIC_INTERFACE_COMMANDS.len(), 18);
    assert_eq!(REQUIRED_SEMANTIC_INTERFACE_WORKFLOWS.len(), 5);
    assert_eq!(REQUIRED_SEMANTIC_INTERFACE_EXAMPLES.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_INTERFACE_PROOFS.len(), 5);
    assert_eq!(REQUIRED_SEMANTIC_INTERFACE_ARTIFACTS.len(), 8);
}

#[test]
fn rejects_missing_rule() {
    assert_rejects_with(INVALID_MISSING_RULE, ErrorCode::MissingInterfaceRule);
}
#[test]
fn rejects_missing_command() {
    assert_rejects_with(INVALID_MISSING_COMMAND, ErrorCode::MissingInterfaceCommand);
}
#[test]
fn rejects_duplicate_command() {
    assert_rejects_with(
        INVALID_DUPLICATE_COMMAND,
        ErrorCode::DuplicateInterfaceCommand,
    );
}
#[test]
fn rejects_unknown_workflow_command() {
    assert_rejects_with(
        INVALID_UNKNOWN_WORKFLOW_COMMAND,
        ErrorCode::InvalidInterfaceWorkflow,
    );
}
#[test]
fn rejects_missing_workflow() {
    assert_rejects_with(
        INVALID_MISSING_WORKFLOW,
        ErrorCode::MissingInterfaceWorkflow,
    );
}
#[test]
fn rejects_missing_example() {
    assert_rejects_with(INVALID_MISSING_EXAMPLE, ErrorCode::MissingInterfaceExample);
}
#[test]
fn rejects_missing_proof() {
    assert_rejects_with(INVALID_MISSING_PROOF, ErrorCode::MissingInterfaceProof);
}
#[test]
fn rejects_unbound_proof_reference() {
    assert_rejects_with(INVALID_UNBOUND_PROOF, ErrorCode::InterfaceProofUnbound);
}
#[test]
fn rejects_manual_only_interface() {
    assert_rejects_with(INVALID_MANUAL_ONLY, ErrorCode::ManualOnlyInterface);
}
#[test]
fn rejects_network_required_interface() {
    assert_rejects_with(
        INVALID_NETWORK_REQUIRED,
        ErrorCode::InterfaceNetworkDependency,
    );
}
#[test]
fn rejects_interface_drift_claim() {
    assert_rejects_with(INVALID_INTERFACE_DRIFT, ErrorCode::InterfaceDriftAccepted);
}
#[test]
fn rejects_phase_closure_claim() {
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}
