use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_OPERATOR_HANDOFF_AUTOMATION_CARRIER: &str =
    "lyra.p02.operator_handoff_automation.carrier.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffWorkflowDescriptor {
    pub id: &'static str,
    pub stage: &'static str,
    pub trigger: &'static str,
    pub output_set: &'static str,
    pub truth_effect: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffCaptureChannelDescriptor {
    pub id: &'static str,
    pub medium: &'static str,
    pub boundary: &'static str,
    pub network_allowed: bool,
    pub operator_ack_required: bool,
    pub capture_hash_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffTargetDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub target_class: &'static str,
    pub capture_channel: &'static str,
    pub pre_import_gate: &'static str,
    pub truth_effect: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffTruthGateDescriptor {
    pub id: &'static str,
    pub gate_class: &'static str,
    pub required_before: &'static str,
    pub rejects: &'static str,
    pub evidence_path: &'static str,
}

pub const LYRALANG_OPERATOR_HANDOFF_WORKFLOWS: &[OperatorHandoffWorkflowDescriptor] = &[
    OperatorHandoffWorkflowDescriptor {
        id: "workflow_preflight_capture_manifest",
        stage: "preflight",
        trigger: "operator_explicit",
        output_set: "canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffWorkflowDescriptor {
        id: "workflow_digest_pairing",
        stage: "canonicalization",
        trigger: "imported_artifact_detected",
        output_set: "digest_pairing_report",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffWorkflowDescriptor {
        id: "workflow_operator_ack_seal",
        stage: "ack_seal",
        trigger: "operator_explicit",
        output_set: "operator_ack_seal",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffWorkflowDescriptor {
        id: "workflow_challenge_binding",
        stage: "challenge_binding",
        trigger: "operator_explicit",
        output_set: "host_boundary_challenge_binding",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffWorkflowDescriptor {
        id: "workflow_local_replay_quarantine",
        stage: "replay_quarantine",
        trigger: "local_replay_completed",
        output_set: "quarantined_replay_receipt",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffWorkflowDescriptor {
        id: "workflow_truth_snapshot_closeout",
        stage: "closeout",
        trigger: "local_validation_receipt_present",
        output_set: "truth_snapshot_candidate",
        truth_effect: "none_without_local_replay",
    },
];

pub const LYRALANG_OPERATOR_HANDOFF_CAPTURE_CHANNELS: &[OperatorHandoffCaptureChannelDescriptor] =
    &[
        OperatorHandoffCaptureChannelDescriptor {
            id: "channel_airgap_media",
            medium: "airgap_media",
            boundary: "external_proof_capture",
            network_allowed: false,
            operator_ack_required: true,
            capture_hash_required: true,
        },
        OperatorHandoffCaptureChannelDescriptor {
            id: "channel_local_filesystem_drop",
            medium: "local_filesystem",
            boundary: "external_proof_capture",
            network_allowed: false,
            operator_ack_required: true,
            capture_hash_required: true,
        },
        OperatorHandoffCaptureChannelDescriptor {
            id: "channel_terminal_paste_digest",
            medium: "operator_terminal",
            boundary: "external_proof_capture",
            network_allowed: false,
            operator_ack_required: true,
            capture_hash_required: true,
        },
        OperatorHandoffCaptureChannelDescriptor {
            id: "channel_external_drive_manifest",
            medium: "external_drive",
            boundary: "external_proof_capture",
            network_allowed: false,
            operator_ack_required: true,
            capture_hash_required: true,
        },
    ];

pub const LYRALANG_OPERATOR_HANDOFF_TARGET_HANDOFFS: &[OperatorHandoffTargetDescriptor] = &[
    OperatorHandoffTargetDescriptor {
        id: "handoff_linux_x86_64",
        target_id: "target_linux_x86_64",
        target_class: "linux",
        capture_channel: "channel_local_filesystem_drop",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_linux_aarch64",
        target_id: "target_linux_aarch64",
        target_class: "linux",
        capture_channel: "channel_airgap_media",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_windows_x86_64",
        target_id: "target_windows_x86_64",
        target_class: "windows",
        capture_channel: "channel_external_drive_manifest",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_windows_aarch64",
        target_id: "target_windows_aarch64",
        target_class: "windows",
        capture_channel: "channel_external_drive_manifest",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_android_aarch64",
        target_id: "target_android_aarch64",
        target_class: "mobile",
        capture_channel: "channel_airgap_media",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_ios_aarch64",
        target_id: "target_ios_aarch64",
        target_class: "mobile",
        capture_channel: "channel_airgap_media",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_wasm32_wasi",
        target_id: "target_wasm32_wasi",
        target_class: "wasm",
        capture_channel: "channel_terminal_paste_digest",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_wasm32_unknown",
        target_id: "target_wasm32_unknown",
        target_class: "wasm",
        capture_channel: "channel_terminal_paste_digest",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_baremetal_x86_64",
        target_id: "target_baremetal_x86_64",
        target_class: "baremetal",
        capture_channel: "channel_airgap_media",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_baremetal_aarch64",
        target_id: "target_baremetal_aarch64",
        target_class: "baremetal",
        capture_channel: "channel_airgap_media",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_baremetal_riscv64",
        target_id: "target_baremetal_riscv64",
        target_class: "baremetal",
        capture_channel: "channel_airgap_media",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
    OperatorHandoffTargetDescriptor {
        id: "handoff_host_tooling_quarantine",
        target_id: "target_host_tooling_quarantine",
        target_class: "other",
        capture_channel: "channel_local_filesystem_drop",
        pre_import_gate: "gate_canonical_capture_manifest",
        truth_effect: "none_without_local_replay",
    },
];

pub const LYRALANG_OPERATOR_HANDOFF_TRUTH_GATES: &[OperatorHandoffTruthGateDescriptor] = &[
    OperatorHandoffTruthGateDescriptor {
        id: "gate_canonical_capture_manifest",
        gate_class: "pre_import",
        required_before: "workflow_preflight_capture_manifest",
        rejects: "uncanonical_manifest",
        evidence_path: "ops/p02/handoff/canonical_capture_manifest.gate.lyra",
    },
    OperatorHandoffTruthGateDescriptor {
        id: "gate_operator_acknowledgement",
        gate_class: "pre_import",
        required_before: "workflow_operator_ack_seal",
        rejects: "missing_operator_ack",
        evidence_path: "ops/p02/handoff/operator_acknowledgement.gate.lyra",
    },
    OperatorHandoffTruthGateDescriptor {
        id: "gate_digest_pairing",
        gate_class: "pre_import",
        required_before: "workflow_digest_pairing",
        rejects: "digest_mismatch",
        evidence_path: "ops/p02/handoff/digest_pairing.gate.lyra",
    },
    OperatorHandoffTruthGateDescriptor {
        id: "gate_challenge_suite_binding",
        gate_class: "import",
        required_before: "workflow_challenge_binding",
        rejects: "unbound_challenge_suite",
        evidence_path: "ops/p02/handoff/challenge_suite_binding.gate.lyra",
    },
    OperatorHandoffTruthGateDescriptor {
        id: "gate_local_replay_quarantine",
        gate_class: "import",
        required_before: "workflow_local_replay_quarantine",
        rejects: "truth_promotion_before_replay",
        evidence_path: "ops/p02/handoff/local_replay_quarantine.gate.lyra",
    },
    OperatorHandoffTruthGateDescriptor {
        id: "gate_truth_snapshot_update",
        gate_class: "post_import",
        required_before: "workflow_truth_snapshot_closeout",
        rejects: "truth_snapshot_drift",
        evidence_path: "ops/p02/handoff/truth_snapshot_update.gate.lyra",
    },
];

pub fn operator_handoff_workflow_ids() -> Vec<&'static str> {
    LYRALANG_OPERATOR_HANDOFF_WORKFLOWS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn operator_handoff_capture_channel_ids() -> Vec<&'static str> {
    LYRALANG_OPERATOR_HANDOFF_CAPTURE_CHANNELS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn operator_handoff_target_ids() -> Vec<&'static str> {
    LYRALANG_OPERATOR_HANDOFF_TARGET_HANDOFFS
        .iter()
        .map(|x| x.target_id)
        .collect()
}
pub fn operator_handoff_truth_gate_ids() -> Vec<&'static str> {
    LYRALANG_OPERATOR_HANDOFF_TRUTH_GATES
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn operator_handoff_workflow_descriptor(
    id: &str,
) -> Option<&'static OperatorHandoffWorkflowDescriptor> {
    LYRALANG_OPERATOR_HANDOFF_WORKFLOWS
        .iter()
        .find(|x| x.id == id)
}
pub fn operator_handoff_channel_descriptor(
    id: &str,
) -> Option<&'static OperatorHandoffCaptureChannelDescriptor> {
    LYRALANG_OPERATOR_HANDOFF_CAPTURE_CHANNELS
        .iter()
        .find(|x| x.id == id)
}
pub fn operator_handoff_target_descriptor(
    target_id: &str,
) -> Option<&'static OperatorHandoffTargetDescriptor> {
    LYRALANG_OPERATOR_HANDOFF_TARGET_HANDOFFS
        .iter()
        .find(|x| x.target_id == target_id)
}
pub fn operator_handoff_truth_gate_descriptor(
    id: &str,
) -> Option<&'static OperatorHandoffTruthGateDescriptor> {
    LYRALANG_OPERATOR_HANDOFF_TRUTH_GATES
        .iter()
        .find(|x| x.id == id)
}
pub fn operator_handoff_all_channels_offline() -> bool {
    LYRALANG_OPERATOR_HANDOFF_CAPTURE_CHANNELS
        .iter()
        .all(|x| !x.network_allowed && x.operator_ack_required && x.capture_hash_required)
}
pub fn operator_handoff_all_targets_truth_neutral() -> bool {
    LYRALANG_OPERATOR_HANDOFF_TARGET_HANDOFFS
        .iter()
        .all(|x| x.truth_effect == "none_without_local_replay")
}
pub fn operator_handoff_gates_bind_paths() -> bool {
    LYRALANG_OPERATOR_HANDOFF_TRUTH_GATES
        .iter()
        .all(|x| x.evidence_path.starts_with("ops/p02/handoff/"))
}

pub fn operator_handoff_registry_hash() -> String {
    let workflow_part = LYRALANG_OPERATOR_HANDOFF_WORKFLOWS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.stage, x.trigger, x.output_set, x.truth_effect
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let channel_part = LYRALANG_OPERATOR_HANDOFF_CAPTURE_CHANNELS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                x.id,
                x.medium,
                x.boundary,
                x.network_allowed,
                x.operator_ack_required,
                x.capture_hash_required
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let target_part = LYRALANG_OPERATOR_HANDOFF_TARGET_HANDOFFS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                x.id,
                x.target_id,
                x.target_class,
                x.capture_channel,
                x.pre_import_gate,
                x.truth_effect
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let gate_part = LYRALANG_OPERATOR_HANDOFF_TRUTH_GATES
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.gate_class, x.required_before, x.rejects, x.evidence_path
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    stable_hash_label(
        "lyra.p02.operator_handoff.registry",
        &format!("{workflow_part}|{channel_part}|{target_part}|{gate_part}"),
    )
}

pub fn operator_handoff_registry_signature() -> String {
    format!(
        "{}:{}:{}:{}:{}",
        LYRA_P02_OPERATOR_HANDOFF_AUTOMATION_CARRIER,
        LYRALANG_OPERATOR_HANDOFF_WORKFLOWS.len(),
        LYRALANG_OPERATOR_HANDOFF_CAPTURE_CHANNELS.len(),
        LYRALANG_OPERATOR_HANDOFF_TARGET_HANDOFFS.len(),
        LYRALANG_OPERATOR_HANDOFF_TRUTH_GATES.len()
    )
}
