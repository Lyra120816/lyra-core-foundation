const CONTRACT: &str =
    include_str!("../interfaces/p02/contracts/seed_runtime_replacement_milestones.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");
const MILESTONES: &str =
    include_str!("../ops/p02/seed_runtime/seed_runtime_replacement_milestones.v1.lyra");
const OPERATOR_SURFACE: &str =
    include_str!("../shells/p02/seed_runtime_replacement_operator_surface.lyra");
const PRODUCT_SURFACE: &str =
    include_str!("../products/p02/seed_runtime_replacement_inspection_surface.lyra");
#[test]
fn seed_runtime_replacement_contract_names_p02_009() {
    assert!(CONTRACT.contains("LYRA-P02-SEED-RUNTIME-REPLACEMENT-MILESTONES-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P02-009"));
    for target in [
        "target:target_linux_x86_64",
        "target:target_windows_aarch64",
        "target:target_android_aarch64",
        "target:target_wasm32_wasi",
        "target:target_baremetal_riscv64",
        "target:target_host_tooling_quarantine",
    ] {
        assert!(CONTRACT.contains(target), "contract missing {target}");
    }
}
#[test]
fn control_plane_below_phase_closure_at_current_frontier() {
    assert!(CONTROL_FRONTIER.contains("current_task=P02-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P02-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P03"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P02-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_primary_tasks=none"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("blocker:p02_primary_task_remainder"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P03"));
}
#[test]
fn emitted_replacement_milestones_bind_all_targets_and_handoffs() {
    for needle in [
        "milestone:milestone_linux_x86_64",
        "milestone:milestone_baremetal_riscv64",
        "handoff:handoff_linux_x86_64",
        "handoff:handoff_host_tooling_quarantine",
        "entry_gate:seed_runtime_contract_emitted",
        "proof_gate:native_seed_execution_receipt_required",
        "extinction_gate:delete_or_reclassify_foreign_surface_after_successor_proven",
        "fallback_ref:fallback_linux_x86_64",
        "import_gate:post_import_replay_required",
        "closure_claim:phase_open",
        "emergency_fallback_receipt=receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt",
    ] {
        assert!(
            MILESTONES.contains(needle),
            "replacement milestones missing {needle}"
        );
    }
    assert!(OPERATOR_SURFACE.contains("lyra-p02-seed-runtime-replacement-check"));
    assert!(PRODUCT_SURFACE.contains("milestone_count=12"));
    assert!(PRODUCT_SURFACE.contains("handoff_count=12"));
    assert!(!MILESTONES.contains("global_closure=true"));
}
