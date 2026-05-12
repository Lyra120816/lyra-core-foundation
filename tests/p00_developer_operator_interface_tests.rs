use lyra_phase0::p00::{
    parse_developer_operator_interface_surface, validate_developer_operator_interface_surface,
    ErrorCode, REQUIRED_INTERFACE_COMMANDS, REQUIRED_INTERFACE_EXAMPLES, REQUIRED_INTERFACE_PROOFS,
    REQUIRED_INTERFACE_WORKFLOWS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p00/developer_operator_interface_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_developer_operator_interface_surface() {
    let input = fixture("valid_developer_operator_interface.lyra");
    let parsed =
        parse_developer_operator_interface_surface(&input).expect("valid interface surface parses");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-018");
    assert_eq!(parsed.commands.len(), REQUIRED_INTERFACE_COMMANDS.len());
    assert_eq!(parsed.workflows.len(), REQUIRED_INTERFACE_WORKFLOWS.len());
    assert_eq!(parsed.examples.len(), REQUIRED_INTERFACE_EXAMPLES.len());
    assert_eq!(parsed.proofs.len(), REQUIRED_INTERFACE_PROOFS.len());
    let (verdict, receipt) = validate_developer_operator_interface_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.verdict.accepted, verdict.accepted);
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_required_rule() {
    let input = fixture("invalid_missing_rule.lyra");
    let (verdict, _) = validate_developer_operator_interface_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::MissingInterfaceRule));
}

#[test]
fn rejects_missing_required_command() {
    let input = fixture("invalid_missing_command.lyra");
    let (verdict, _) = validate_developer_operator_interface_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::MissingInterfaceCommand));
}

#[test]
fn rejects_duplicate_command_identity() {
    let input = fixture("invalid_duplicate_command.lyra");
    let (verdict, _) = validate_developer_operator_interface_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::DuplicateInterfaceCommand));
}

#[test]
fn rejects_unknown_workflow_command_binding() {
    let input = fixture("invalid_unknown_workflow_command.lyra");
    let (verdict, _) = validate_developer_operator_interface_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::InvalidInterfaceWorkflow));
}

#[test]
fn rejects_missing_workflow_example_and_proof() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_workflow.lyra",
            ErrorCode::MissingInterfaceWorkflow,
        ),
        (
            "invalid_missing_example.lyra",
            ErrorCode::MissingInterfaceExample,
        ),
        (
            "invalid_missing_proof.lyra",
            ErrorCode::MissingInterfaceProof,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_developer_operator_interface_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_unbound_proof_reference() {
    let input = fixture("invalid_unbound_proof_reference.lyra");
    let (verdict, _) = validate_developer_operator_interface_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::InterfaceProofUnbound));
}

#[test]
fn rejects_manual_or_network_only_interfaces() {
    for (fixture_name, expected) in [
        (
            "invalid_manual_only_interface.lyra",
            ErrorCode::ManualOnlyInterface,
        ),
        (
            "invalid_network_required.lyra",
            ErrorCode::InterfaceNetworkDependency,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_developer_operator_interface_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_phase_closure_claim() {
    let input = fixture("invalid_phase_closure_claim.lyra");
    let (verdict, _) = validate_developer_operator_interface_surface(&input);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::UnsupportedGlobalClosure));
}
