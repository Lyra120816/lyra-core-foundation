const CONTRACT: &str =
    include_str!("../interfaces/p02/contracts/bootstrap_emergency_fallback.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");
const EMERGENCY: &str =
    include_str!("../ops/p02/emergency_fallback/bootstrap_emergency_fallback_law.v1.lyra");
const OPERATOR_SURFACE: &str =
    include_str!("../shells/p02/bootstrap_emergency_fallback_operator_surface.lyra");
const PRODUCT_SURFACE: &str =
    include_str!("../products/p02/bootstrap_emergency_fallback_inspection_surface.lyra");
#[test]
fn emergency_fallback_contract_names_p02_008() {
    assert!(CONTRACT.contains("LYRA-P02-BOOTSTRAP-EMERGENCY-FALLBACK-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P02-008"));
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
    assert!(BLOCKER_INDEX.contains("blocker:p02_primary_task_remainder"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P03"));
}
#[test]
fn emitted_emergency_fallback_binds_all_target_failure_and_rollback_rules() {
    for needle in [
        "fallback:fallback_linux_x86_64",
        "fallback:fallback_baremetal_riscv64",
        "rollback:rollback_linux_x86_64",
        "rollback:rollback_host_tooling_quarantine",
        "failure_state:pending_local_validation",
        "freeze_action:freeze_truth_promotion",
        "fallback_action:enter_bounded_failure_quarantine",
        "rollback_path:rollback_to_truth_cleanup",
        "replay_gate:post_rollback_replay_required",
        "closure_claim:phase_open",
        "truth_cleanup_receipt=receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt",
    ] {
        assert!(
            EMERGENCY.contains(needle),
            "emergency fallback missing {needle}"
        );
    }
    assert!(OPERATOR_SURFACE.contains("lyra-p02-emergency-fallback-check"));
    assert!(PRODUCT_SURFACE.contains("fallback_count=12"));
    assert!(PRODUCT_SURFACE.contains("rollback_count=12"));
    assert!(!EMERGENCY.contains("global_closure=true"));
}
