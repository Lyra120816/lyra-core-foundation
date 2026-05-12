use crate::k0_hash::stable_hash_label;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapEmergencyFallbackDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub target_class: &'static str,
    pub failure_state: &'static str,
    pub fallback_action: &'static str,
    pub rollback_path: &'static str,
    pub closure_claim: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapEmergencyRollbackDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub trigger: &'static str,
    pub to_state: &'static str,
    pub replay_gate: &'static str,
    pub frontier_decision: &'static str,
}
pub const LYRA_P02_BOOTSTRAP_EMERGENCY_FALLBACK_CARRIER: &str =
    "lyralang.bootstrap_emergency_fallback.v1";
pub const LYRALANG_BOOTSTRAP_EMERGENCY_FALLBACKS: &[BootstrapEmergencyFallbackDescriptor] = &[
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_linux_x86_64",
        target_id: "target_linux_x86_64",
        target_class: "linux",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_linux_aarch64",
        target_id: "target_linux_aarch64",
        target_class: "linux",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_windows_x86_64",
        target_id: "target_windows_x86_64",
        target_class: "windows",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_windows_aarch64",
        target_id: "target_windows_aarch64",
        target_class: "windows",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_android_aarch64",
        target_id: "target_android_aarch64",
        target_class: "mobile",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_ios_aarch64",
        target_id: "target_ios_aarch64",
        target_class: "mobile",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_wasm32_wasi",
        target_id: "target_wasm32_wasi",
        target_class: "wasm",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_wasm32_unknown",
        target_id: "target_wasm32_unknown",
        target_class: "wasm",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_baremetal_x86_64",
        target_id: "target_baremetal_x86_64",
        target_class: "baremetal",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_baremetal_aarch64",
        target_id: "target_baremetal_aarch64",
        target_class: "baremetal",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_baremetal_riscv64",
        target_id: "target_baremetal_riscv64",
        target_class: "baremetal",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
    BootstrapEmergencyFallbackDescriptor {
        id: "fallback_host_tooling_quarantine",
        target_id: "target_host_tooling_quarantine",
        target_class: "other",
        failure_state: "pending_local_validation",
        fallback_action: "enter_bounded_failure_quarantine",
        rollback_path: "rollback_to_truth_cleanup",
        closure_claim: "phase_open",
    },
];
pub const LYRALANG_BOOTSTRAP_EMERGENCY_ROLLBACKS: &[BootstrapEmergencyRollbackDescriptor] = &[
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_linux_x86_64",
        target_id: "target_linux_x86_64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_linux_aarch64",
        target_id: "target_linux_aarch64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_windows_x86_64",
        target_id: "target_windows_x86_64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_windows_aarch64",
        target_id: "target_windows_aarch64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_android_aarch64",
        target_id: "target_android_aarch64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_ios_aarch64",
        target_id: "target_ios_aarch64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_wasm32_wasi",
        target_id: "target_wasm32_wasi",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_wasm32_unknown",
        target_id: "target_wasm32_unknown",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_baremetal_x86_64",
        target_id: "target_baremetal_x86_64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_baremetal_aarch64",
        target_id: "target_baremetal_aarch64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_baremetal_riscv64",
        target_id: "target_baremetal_riscv64",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
    BootstrapEmergencyRollbackDescriptor {
        id: "rollback_host_tooling_quarantine",
        target_id: "target_host_tooling_quarantine",
        trigger: "incomplete_target_lane",
        to_state: "bounded_failure_quarantine",
        replay_gate: "post_rollback_replay_required",
        frontier_decision: "hold_until_target_proven_or_retired",
    },
];
pub fn bootstrap_emergency_fallback_ids() -> Vec<&'static str> {
    let mut ids: Vec<_> = LYRALANG_BOOTSTRAP_EMERGENCY_FALLBACKS
        .iter()
        .map(|x| x.id)
        .collect();
    ids.sort();
    ids
}
pub fn bootstrap_emergency_rollback_ids() -> Vec<&'static str> {
    let mut ids: Vec<_> = LYRALANG_BOOTSTRAP_EMERGENCY_ROLLBACKS
        .iter()
        .map(|x| x.id)
        .collect();
    ids.sort();
    ids
}
pub fn bootstrap_emergency_fallback_binds_target(target: &str) -> bool {
    LYRALANG_BOOTSTRAP_EMERGENCY_FALLBACKS
        .iter()
        .any(|x| x.target_id == target)
}
pub fn bootstrap_emergency_rollback_binds_target(target: &str) -> bool {
    LYRALANG_BOOTSTRAP_EMERGENCY_ROLLBACKS
        .iter()
        .any(|x| x.target_id == target)
}
pub fn bootstrap_emergency_fallback_no_phase_closure_claims() -> bool {
    LYRALANG_BOOTSTRAP_EMERGENCY_FALLBACKS
        .iter()
        .all(|x| x.closure_claim == "phase_open")
}
pub fn bootstrap_emergency_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_BOOTSTRAP_EMERGENCY_FALLBACKS {
        rows.push(format!(
            "fallback:{}:{}:{}:{}:{}:{}:{}",
            item.id,
            item.target_id,
            item.target_class,
            item.failure_state,
            item.fallback_action,
            item.rollback_path,
            item.closure_claim
        ));
    }
    for item in LYRALANG_BOOTSTRAP_EMERGENCY_ROLLBACKS {
        rows.push(format!(
            "rollback:{}:{}:{}:{}:{}:{}",
            item.id,
            item.target_id,
            item.trigger,
            item.to_state,
            item.replay_gate,
            item.frontier_decision
        ));
    }
    rows.sort();
    rows.join("|")
}
pub fn bootstrap_emergency_registry_hash() -> String {
    stable_hash_label(
        "lyralang.p02.bootstrap_emergency_fallback.registry",
        &bootstrap_emergency_registry_signature(),
    )
}
