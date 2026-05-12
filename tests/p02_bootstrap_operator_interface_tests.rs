use lyra_phase0::p02::{validate_bootstrap_operator_interface_surface, ErrorCode};

const VALID: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/valid_bootstrap_operator_interface.lyra"
);
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_missing_rule.lyra");
const INVALID_BAD_TASK: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_bad_task.lyra");
const INVALID_BAD_STATUS: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_bad_status.lyra");
const INVALID_MISSING_COMMAND: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_missing_command.lyra"
);
const INVALID_DUPLICATE_COMMAND: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_duplicate_command.lyra"
);
const INVALID_UNKNOWN_COMMAND_REFERENCE: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_unknown_command_reference.lyra"
);
const INVALID_BAD_ROLE: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_bad_role.lyra");
const INVALID_BAD_TARGET: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_bad_target.lyra");
const INVALID_MISSING_GATE: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_missing_gate.lyra");
const INVALID_DUPLICATE_GATE: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_duplicate_gate.lyra");
const INVALID_BAD_GATE_DECISION: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_bad_gate_decision.lyra"
);
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_missing_proof.lyra");
const INVALID_PROOF_UNBOUND_GATE: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_proof_unbound_gate.lyra"
);
const INVALID_BAD_ARTIFACT_OWNER: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_bad_artifact_owner.lyra"
);
const INVALID_NETWORK_REQUIRED: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_network_required.lyra"
);
const INVALID_MANUAL_ONLY: &str =
    include_str!("../fixtures/p02/bootstrap_operator_interface_inputs/invalid_manual_only.lyra");
const INVALID_PROBABILISTIC_INTERFACE: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_probabilistic_interface.lyra"
);
const INVALID_AMBIENT_TIME_INTERFACE: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_ambient_time_interface.lyra"
);
const INVALID_PHASE_CLOSURE_CLAIM: &str = include_str!(
    "../fixtures/p02/bootstrap_operator_interface_inputs/invalid_phase_closure_claim.lyra"
);

fn assert_rejects_with(input: &str, expected: ErrorCode) {
    let (verdict, receipt) = validate_bootstrap_operator_interface_surface(input);
    assert!(
        !verdict.accepted,
        "input unexpectedly accepted with receipt {}",
        receipt.receipt_hash
    );
    assert!(
        verdict.errors.iter().any(|error| error.code == expected),
        "expected {:?}, got {:?}",
        expected,
        verdict.errors
    );
}

#[test]
fn valid_bootstrap_operator_interface_is_accepted() {
    let (verdict, receipt) = validate_bootstrap_operator_interface_surface(VALID);
    assert!(
        verdict.accepted,
        "valid bootstrap operator interface rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.verdict.status_token(), "ACCEPTED");
}

#[test]
fn rejects_missing_rule() {
    assert_rejects_with(INVALID_MISSING_RULE, ErrorCode::MissingInterfaceRule);
}
#[test]
fn rejects_bad_task() {
    assert_rejects_with(INVALID_BAD_TASK, ErrorCode::InvalidTask);
}
#[test]
fn rejects_bad_status() {
    assert_rejects_with(INVALID_BAD_STATUS, ErrorCode::UnsupportedEvidenceClaim);
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
fn rejects_unknown_command_reference() {
    assert_rejects_with(
        INVALID_UNKNOWN_COMMAND_REFERENCE,
        ErrorCode::InterfaceProofUnbound,
    );
}
#[test]
fn rejects_bad_role() {
    assert_rejects_with(INVALID_BAD_ROLE, ErrorCode::InvalidInterfaceCommand);
}
#[test]
fn rejects_bad_target() {
    assert_rejects_with(INVALID_BAD_TARGET, ErrorCode::InvalidInterfaceCommand);
}
#[test]
fn rejects_missing_gate() {
    assert_rejects_with(INVALID_MISSING_GATE, ErrorCode::MissingReviewGate);
}
#[test]
fn rejects_duplicate_gate() {
    assert_rejects_with(INVALID_DUPLICATE_GATE, ErrorCode::DuplicateReviewGate);
}
#[test]
fn rejects_bad_gate_decision() {
    assert_rejects_with(INVALID_BAD_GATE_DECISION, ErrorCode::InvalidReviewGate);
}
#[test]
fn rejects_missing_proof() {
    assert_rejects_with(INVALID_MISSING_PROOF, ErrorCode::MissingInterfaceProof);
}
#[test]
fn rejects_unbound_gate() {
    assert_rejects_with(INVALID_PROOF_UNBOUND_GATE, ErrorCode::InterfaceProofUnbound);
}
#[test]
fn rejects_bad_artifact_owner() {
    assert_rejects_with(
        INVALID_BAD_ARTIFACT_OWNER,
        ErrorCode::InvalidDeliveryArtifact,
    );
}
#[test]
fn rejects_network_required() {
    assert_rejects_with(
        INVALID_NETWORK_REQUIRED,
        ErrorCode::InterfaceNetworkDependency,
    );
}
#[test]
fn rejects_manual_only() {
    assert_rejects_with(INVALID_MANUAL_ONLY, ErrorCode::ManualOnlyInterface);
}
#[test]
fn rejects_probabilistic_interface() {
    assert_rejects_with(
        INVALID_PROBABILISTIC_INTERFACE,
        ErrorCode::ProbabilisticTruthAllowed,
    );
}
#[test]
fn rejects_ambient_time_interface() {
    assert_rejects_with(
        INVALID_AMBIENT_TIME_INTERFACE,
        ErrorCode::AmbientTimeAllowed,
    );
}
#[test]
fn rejects_phase_closure_claim() {
    assert_rejects_with(
        INVALID_PHASE_CLOSURE_CLAIM,
        ErrorCode::UnsupportedGlobalClosure,
    );
}
