use crate::k0_hash::stable_hash_label;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCleanupDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub target_class: &'static str,
    pub proven_action: &'static str,
    pub retired_action: &'static str,
    pub truth_update: &'static str,
    pub rollback_path: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapFrontierAdvanceDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub next_frontier: &'static str,
    pub closure_claim: &'static str,
}
pub const LYRA_P02_BOOTSTRAP_TRUTH_CLEANUP_CARRIER: &str = "lyralang.bootstrap_truth_cleanup.v1";
pub const LYRALANG_BOOTSTRAP_CLEANUPS: &[BootstrapCleanupDescriptor] = &[
    BootstrapCleanupDescriptor {
        id: "cleanup_linux_x86_64",
        target_id: "target_linux_x86_64",
        target_class: "linux",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_linux_aarch64",
        target_id: "target_linux_aarch64",
        target_class: "linux",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_windows_x86_64",
        target_id: "target_windows_x86_64",
        target_class: "windows",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_windows_aarch64",
        target_id: "target_windows_aarch64",
        target_class: "windows",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_android_aarch64",
        target_id: "target_android_aarch64",
        target_class: "mobile",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_ios_aarch64",
        target_id: "target_ios_aarch64",
        target_class: "mobile",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_wasm32_wasi",
        target_id: "target_wasm32_wasi",
        target_class: "wasm",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_wasm32_unknown",
        target_id: "target_wasm32_unknown",
        target_class: "wasm",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_baremetal_x86_64",
        target_id: "target_baremetal_x86_64",
        target_class: "baremetal",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_baremetal_aarch64",
        target_id: "target_baremetal_aarch64",
        target_class: "baremetal",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_baremetal_riscv64",
        target_id: "target_baremetal_riscv64",
        target_class: "baremetal",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
    BootstrapCleanupDescriptor {
        id: "cleanup_host_tooling_quarantine",
        target_id: "target_host_tooling_quarantine",
        target_class: "other",
        proven_action: "seal_execution_receipt",
        retired_action: "bind_retirement_receipt",
        truth_update: "mark_target_closed",
        rollback_path: "rollback_to_target_matrix",
    },
];
pub const LYRALANG_BOOTSTRAP_FRONTIER_ADVANCES: &[BootstrapFrontierAdvanceDescriptor] = &[
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_linux_x86_64",
        target_id: "target_linux_x86_64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_linux_aarch64",
        target_id: "target_linux_aarch64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_windows_x86_64",
        target_id: "target_windows_x86_64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_windows_aarch64",
        target_id: "target_windows_aarch64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_android_aarch64",
        target_id: "target_android_aarch64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_ios_aarch64",
        target_id: "target_ios_aarch64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_wasm32_wasi",
        target_id: "target_wasm32_wasi",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_wasm32_unknown",
        target_id: "target_wasm32_unknown",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_baremetal_x86_64",
        target_id: "target_baremetal_x86_64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_baremetal_aarch64",
        target_id: "target_baremetal_aarch64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_baremetal_riscv64",
        target_id: "target_baremetal_riscv64",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
    BootstrapFrontierAdvanceDescriptor {
        id: "frontier_host_tooling_quarantine",
        target_id: "target_host_tooling_quarantine",
        next_frontier: "P02-008",
        closure_claim: "phase_open",
    },
];
pub fn bootstrap_cleanup_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CLEANUPS.iter().map(|x| x.id).collect()
}
pub fn bootstrap_frontier_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_FRONTIER_ADVANCES
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_cleanup_covers_target(target: &str) -> bool {
    LYRALANG_BOOTSTRAP_CLEANUPS
        .iter()
        .any(|x| x.target_id == target)
}
pub fn bootstrap_frontier_binds_target(target: &str) -> bool {
    LYRALANG_BOOTSTRAP_FRONTIER_ADVANCES.iter().any(|x| {
        x.target_id == target && x.next_frontier == "P02-008" && x.closure_claim == "phase_open"
    })
}
pub fn bootstrap_truth_cleanup_registry_hash() -> String {
    let mut rows = Vec::new();
    for x in LYRALANG_BOOTSTRAP_CLEANUPS {
        rows.push(format!(
            "cleanup:{}:{}:{}:{}:{}:{}:{}",
            x.id,
            x.target_id,
            x.target_class,
            x.proven_action,
            x.retired_action,
            x.truth_update,
            x.rollback_path
        ));
    }
    for x in LYRALANG_BOOTSTRAP_FRONTIER_ADVANCES {
        rows.push(format!(
            "frontier:{}:{}:{}:{}",
            x.id, x.target_id, x.next_frontier, x.closure_claim
        ));
    }
    rows.sort();
    stable_hash_label(
        "lyra.lyralang.bootstrap_truth_cleanup.registry",
        &rows.join("|"),
    )
}
pub fn bootstrap_truth_cleanup_registry_signature() -> String {
    stable_hash_label(
        "lyra.lyralang.bootstrap_truth_cleanup.signature",
        &bootstrap_truth_cleanup_registry_hash(),
    )
}
pub fn bootstrap_truth_cleanup_no_phase_closure_claims() -> bool {
    LYRALANG_BOOTSTRAP_FRONTIER_ADVANCES
        .iter()
        .all(|x| x.closure_claim == "phase_open")
}
