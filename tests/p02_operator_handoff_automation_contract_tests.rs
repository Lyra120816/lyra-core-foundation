use lyra_phase0::p02::{
    operator_handoff_all_channels_offline, operator_handoff_all_targets_truth_neutral,
    operator_handoff_capture_channel_ids, operator_handoff_gates_bind_paths,
    operator_handoff_registry_hash, operator_handoff_registry_signature,
    operator_handoff_target_ids, operator_handoff_truth_gate_ids, operator_handoff_workflow_ids,
    LYRA_P02_OPERATOR_HANDOFF_AUTOMATION_CARRIER, REQUIRED_OPERATOR_HANDOFF_CAPTURE_CHANNELS,
    REQUIRED_OPERATOR_HANDOFF_GATES, REQUIRED_OPERATOR_HANDOFF_TARGETS,
    REQUIRED_OPERATOR_HANDOFF_WORKFLOWS,
};

#[test]
fn lyralang_operator_handoff_registry_is_complete() {
    assert_eq!(
        operator_handoff_workflow_ids().len(),
        REQUIRED_OPERATOR_HANDOFF_WORKFLOWS.len()
    );
    assert_eq!(
        operator_handoff_capture_channel_ids().len(),
        REQUIRED_OPERATOR_HANDOFF_CAPTURE_CHANNELS.len()
    );
    assert_eq!(
        operator_handoff_target_ids().len(),
        REQUIRED_OPERATOR_HANDOFF_TARGETS.len()
    );
    assert_eq!(
        operator_handoff_truth_gate_ids().len(),
        REQUIRED_OPERATOR_HANDOFF_GATES.len()
    );
    assert!(operator_handoff_all_channels_offline());
    assert!(operator_handoff_all_targets_truth_neutral());
    assert!(operator_handoff_gates_bind_paths());
    assert!(operator_handoff_registry_hash().starts_with("fnv1a128:"));
    assert!(operator_handoff_registry_signature()
        .starts_with(LYRA_P02_OPERATOR_HANDOFF_AUTOMATION_CARRIER));
}
