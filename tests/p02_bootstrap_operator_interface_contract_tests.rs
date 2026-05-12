use lyra_phase0::p02::{
    bootstrap_operator_artifacts_bind_commands, bootstrap_operator_carrier_signature,
    bootstrap_operator_commands_cover_p02_018, bootstrap_operator_examples_bind_known_commands,
    bootstrap_operator_gates_bind_registry, bootstrap_operator_no_forbidden_descriptor_claims,
    bootstrap_operator_proofs_bind_registry, bootstrap_operator_registry_hash,
    bootstrap_operator_workflows_bind_known_commands,
    deterministic_bootstrap_operator_interface_suite_report,
    parse_bootstrap_operator_interface_surface, validate_bootstrap_operator_interface_surface,
    LYRA_P02_BOOTSTRAP_OPERATOR_INTERFACE_CARRIER, REQUIRED_BOOTSTRAP_OPERATOR_ARTIFACTS,
    REQUIRED_BOOTSTRAP_OPERATOR_COMMANDS, REQUIRED_BOOTSTRAP_OPERATOR_EXAMPLES,
    REQUIRED_BOOTSTRAP_OPERATOR_GATES, REQUIRED_BOOTSTRAP_OPERATOR_PROOFS,
    REQUIRED_BOOTSTRAP_OPERATOR_WORKFLOWS,
};

const VALID: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/valid_bootstrap_operator_interface.lyra"
);
const CONTRACT: &str =
    include_str!("../interfaces/p02/contracts/bootstrap_operator_interface.v1.lyra");

#[test]
fn contract_names_runtime_surface() {
    assert!(CONTRACT.contains("LYRA-P02-BOOTSTRAP-OPERATOR-INTERFACE v1"));
    assert!(CONTRACT.contains("task=P02-018"));
}

#[test]
fn descriptor_registry_is_closed_and_bound() {
    assert!(bootstrap_operator_workflows_bind_known_commands());
    assert!(bootstrap_operator_examples_bind_known_commands());
    assert!(bootstrap_operator_gates_bind_registry());
    assert!(bootstrap_operator_proofs_bind_registry());
    assert!(bootstrap_operator_artifacts_bind_commands());
    assert!(bootstrap_operator_commands_cover_p02_018());
    assert!(bootstrap_operator_no_forbidden_descriptor_claims());
    assert!(bootstrap_operator_carrier_signature()
        .starts_with(LYRA_P02_BOOTSTRAP_OPERATOR_INTERFACE_CARRIER));
    assert!(!bootstrap_operator_registry_hash().is_empty());
}

#[test]
fn deterministic_report_counts_match_required_tables() {
    let surface = parse_bootstrap_operator_interface_surface(VALID).expect("valid surface parses");
    let report = deterministic_bootstrap_operator_interface_suite_report(
        &surface
            .commands
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.binary.clone(),
                    item.surface.clone(),
                    item.input.clone(),
                    item.output.clone(),
                    item.receipts.clone(),
                    item.roles.clone(),
                    item.targets.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .workflows
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.order.clone(),
                    item.commands.clone(),
                    item.targets.clone(),
                    item.examples.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .examples
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.path.clone(),
                    item.commands.clone(),
                    item.expected_receipts.clone(),
                    item.expected_verdict.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .gates
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.workflow.clone(),
                    item.required_receipts.clone(),
                    item.required_examples.clone(),
                    item.decision.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .proofs
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.scope.clone(),
                    item.commands.clone(),
                    item.workflows.clone(),
                    item.examples.clone(),
                    item.gates.clone(),
                    item.receipts.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .artifacts
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.owner_root.clone(),
                    item.path.clone(),
                    item.artifact_kind.clone(),
                    item.commands.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        report.command_count,
        REQUIRED_BOOTSTRAP_OPERATOR_COMMANDS.len()
    );
    assert_eq!(
        report.workflow_count,
        REQUIRED_BOOTSTRAP_OPERATOR_WORKFLOWS.len()
    );
    assert_eq!(
        report.example_count,
        REQUIRED_BOOTSTRAP_OPERATOR_EXAMPLES.len()
    );
    assert_eq!(report.gate_count, REQUIRED_BOOTSTRAP_OPERATOR_GATES.len());
    assert_eq!(report.proof_count, REQUIRED_BOOTSTRAP_OPERATOR_PROOFS.len());
    assert_eq!(
        report.artifact_count,
        REQUIRED_BOOTSTRAP_OPERATOR_ARTIFACTS.len()
    );
    assert!(!report.suite_hash.is_empty());
}

#[test]
fn valid_surface_receipt_is_accepted() {
    let (verdict, receipt) = validate_bootstrap_operator_interface_surface(VALID);
    assert!(verdict.accepted, "receipt {receipt:?}");
    assert_eq!(receipt.verdict.status_token(), "ACCEPTED");
}
