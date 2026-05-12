const CONTRACT: &str = include_str!("../interfaces/p02/contracts/bootstrap_truth_cleanup.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");
const CLEANUP: &str =
    include_str!("../ops/p02/truth_cleanup/bootstrap_truth_cleanup_frontier_rules.v1.lyra");
const OPERATOR_SURFACE: &str =
    include_str!("../shells/p02/bootstrap_truth_cleanup_operator_surface.lyra");
const PRODUCT_SURFACE: &str =
    include_str!("../products/p02/bootstrap_truth_cleanup_inspection_surface.lyra");
#[test]
fn truth_cleanup_contract_names_p02_007() {
    assert!(CONTRACT.contains("LYRA-P02-BOOTSTRAP-TRUTH-CLEANUP-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P02-007"));
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
fn emitted_truth_cleanup_binds_all_target_cleanup_and_frontier_rules() {
    for needle in [
        "cleanup:cleanup_linux_x86_64",
        "cleanup:cleanup_baremetal_riscv64",
        "frontier:frontier_linux_x86_64",
        "frontier:frontier_host_tooling_quarantine",
        "proven_action:seal_execution_receipt",
        "retired_action:bind_retirement_receipt",
        "truth_update:mark_target_closed",
        "rollback_path:rollback_to_target_matrix",
        "closure_claim:phase_open",
        "target_matrix_receipt=receipts/p02/pass_0064_bootstrap_target_matrix.receipt",
    ] {
        assert!(CLEANUP.contains(needle), "cleanup missing {needle}");
    }
    assert!(OPERATOR_SURFACE.contains("lyra-p02-truth-cleanup-check"));
    assert!(PRODUCT_SURFACE.contains("cleanup_count=12"));
    assert!(PRODUCT_SURFACE.contains("frontier_count=12"));
    assert!(!CLEANUP.contains("global_closure=true"));
}
