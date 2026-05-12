use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_SEED_RUNTIME_REPLACEMENT_CARRIER: &str =
    "lyralang.core.p02.seed_runtime_replacement_milestones";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeedRuntimeReplacementMilestoneDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub target_class: &'static str,
    pub replacement_unit: &'static str,
    pub foreign_surface_ref: &'static str,
    pub native_successor: &'static str,
    pub entry_gate: &'static str,
    pub proof_gate: &'static str,
    pub extinction_gate: &'static str,
    pub fallback_ref: &'static str,
    pub closure_claim: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeedRuntimeReplacementHandoffDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub operator_role: &'static str,
    pub truth_effect: &'static str,
    pub import_gate: &'static str,
}

pub const LYRALANG_SEED_RUNTIME_REPLACEMENT_MILESTONES:
    &[SeedRuntimeReplacementMilestoneDescriptor] = &[
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_linux_x86_64",
        target_id: "target_linux_x86_64",
        target_class: "linux",
        replacement_unit: "native_seed_runtime_linux_x86_64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_linux_x86_64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_linux_x86_64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_linux_aarch64",
        target_id: "target_linux_aarch64",
        target_class: "linux",
        replacement_unit: "native_seed_runtime_linux_aarch64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_linux_aarch64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_linux_aarch64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_windows_x86_64",
        target_id: "target_windows_x86_64",
        target_class: "windows",
        replacement_unit: "native_seed_runtime_windows_x86_64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_windows_x86_64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_windows_x86_64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_windows_aarch64",
        target_id: "target_windows_aarch64",
        target_class: "windows",
        replacement_unit: "native_seed_runtime_windows_aarch64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_windows_aarch64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_windows_aarch64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_android_aarch64",
        target_id: "target_android_aarch64",
        target_class: "mobile",
        replacement_unit: "native_seed_runtime_android_aarch64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_android_aarch64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_android_aarch64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_ios_aarch64",
        target_id: "target_ios_aarch64",
        target_class: "mobile",
        replacement_unit: "native_seed_runtime_ios_aarch64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_ios_aarch64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_ios_aarch64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_wasm32_wasi",
        target_id: "target_wasm32_wasi",
        target_class: "wasm",
        replacement_unit: "native_seed_runtime_wasm32_wasi",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_wasm32_wasi",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_wasm32_wasi",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_wasm32_unknown",
        target_id: "target_wasm32_unknown",
        target_class: "wasm",
        replacement_unit: "native_seed_runtime_wasm32_unknown",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_wasm32_unknown",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_wasm32_unknown",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_baremetal_x86_64",
        target_id: "target_baremetal_x86_64",
        target_class: "baremetal",
        replacement_unit: "native_seed_runtime_baremetal_x86_64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_baremetal_x86_64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_baremetal_x86_64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_baremetal_aarch64",
        target_id: "target_baremetal_aarch64",
        target_class: "baremetal",
        replacement_unit: "native_seed_runtime_baremetal_aarch64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_baremetal_aarch64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_baremetal_aarch64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_baremetal_riscv64",
        target_id: "target_baremetal_riscv64",
        target_class: "baremetal",
        replacement_unit: "native_seed_runtime_baremetal_riscv64",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_baremetal_riscv64",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_baremetal_riscv64",
        closure_claim: "phase_open",
    },
    SeedRuntimeReplacementMilestoneDescriptor {
        id: "milestone_host_tooling_quarantine",
        target_id: "target_host_tooling_quarantine",
        target_class: "other",
        replacement_unit: "native_seed_runtime_host_tooling_quarantine",
        foreign_surface_ref: "rust_bootstrap_compiler",
        native_successor: "lyra_native_seed_runtime_host_tooling_quarantine",
        entry_gate: "seed_runtime_contract_emitted",
        proof_gate: "native_seed_execution_receipt_required",
        extinction_gate: "delete_or_reclassify_foreign_surface_after_successor_proven",
        fallback_ref: "fallback_host_tooling_quarantine",
        closure_claim: "phase_open",
    },
];

pub const LYRALANG_SEED_RUNTIME_REPLACEMENT_HANDOFFS: &[SeedRuntimeReplacementHandoffDescriptor] =
    &[
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_linux_x86_64",
            target_id: "target_linux_x86_64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_linux_aarch64",
            target_id: "target_linux_aarch64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_windows_x86_64",
            target_id: "target_windows_x86_64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_windows_aarch64",
            target_id: "target_windows_aarch64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_android_aarch64",
            target_id: "target_android_aarch64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_ios_aarch64",
            target_id: "target_ios_aarch64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_wasm32_wasi",
            target_id: "target_wasm32_wasi",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_wasm32_unknown",
            target_id: "target_wasm32_unknown",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_baremetal_x86_64",
            target_id: "target_baremetal_x86_64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_baremetal_aarch64",
            target_id: "target_baremetal_aarch64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_baremetal_riscv64",
            target_id: "target_baremetal_riscv64",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
        SeedRuntimeReplacementHandoffDescriptor {
            id: "handoff_host_tooling_quarantine",
            target_id: "target_host_tooling_quarantine",
            operator_role: "external_capture_only",
            truth_effect: "none_without_local_replay",
            import_gate: "post_import_replay_required",
        },
    ];

pub fn seed_runtime_replacement_milestone_ids() -> Vec<&'static str> {
    let mut ids: Vec<_> = LYRALANG_SEED_RUNTIME_REPLACEMENT_MILESTONES
        .iter()
        .map(|x| x.id)
        .collect();
    ids.sort();
    ids
}
pub fn seed_runtime_replacement_handoff_ids() -> Vec<&'static str> {
    let mut ids: Vec<_> = LYRALANG_SEED_RUNTIME_REPLACEMENT_HANDOFFS
        .iter()
        .map(|x| x.id)
        .collect();
    ids.sort();
    ids
}
pub fn seed_runtime_replacement_binds_target(target: &str) -> bool {
    LYRALANG_SEED_RUNTIME_REPLACEMENT_MILESTONES
        .iter()
        .any(|x| x.target_id == target)
}
pub fn seed_runtime_replacement_handoff_binds_target(target: &str) -> bool {
    LYRALANG_SEED_RUNTIME_REPLACEMENT_HANDOFFS
        .iter()
        .any(|x| x.target_id == target)
}
pub fn seed_runtime_replacement_no_phase_closure_claims() -> bool {
    LYRALANG_SEED_RUNTIME_REPLACEMENT_MILESTONES
        .iter()
        .all(|x| x.closure_claim == "phase_open")
}
pub fn seed_runtime_replacement_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SEED_RUNTIME_REPLACEMENT_MILESTONES {
        rows.push(format!(
            "milestone:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            item.id,
            item.target_id,
            item.target_class,
            item.replacement_unit,
            item.foreign_surface_ref,
            item.native_successor,
            item.entry_gate,
            item.proof_gate,
            item.extinction_gate,
            item.fallback_ref,
            item.closure_claim
        ));
    }
    for item in LYRALANG_SEED_RUNTIME_REPLACEMENT_HANDOFFS {
        rows.push(format!(
            "handoff:{}:{}:{}:{}:{}",
            item.id, item.target_id, item.operator_role, item.truth_effect, item.import_gate
        ));
    }
    rows.sort();
    rows.join("|")
}
pub fn seed_runtime_replacement_registry_hash() -> String {
    stable_hash_label(
        "lyralang.p02.seed_runtime_replacement.registry",
        &seed_runtime_replacement_registry_signature(),
    )
}
