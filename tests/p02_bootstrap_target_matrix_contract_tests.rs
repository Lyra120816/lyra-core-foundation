const CONTRACT: &str = include_str!("../interfaces/p02/contracts/bootstrap_target_matrix.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");
const TARGET_MATRIX: &str =
    include_str!("../ops/p02/target_matrix/bootstrap_target_matrix.v1.lyra");
const OPERATOR_SURFACE: &str =
    include_str!("../shells/p02/bootstrap_target_matrix_operator_surface.lyra");
const PRODUCT_SURFACE: &str =
    include_str!("../products/p02/bootstrap_target_matrix_inspection_surface.lyra");
#[test]
fn target_matrix_contract_names_p02_006() {
    assert!(CONTRACT.contains("LYRA-P02-BOOTSTRAP-TARGET-MATRIX-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P02-006"));
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
    assert!(CONTROL_FRONTIER.contains("current_task=P02-019"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P02-018"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P03"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P02-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_primary_tasks=P02-020"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("blocker:p02_primary_task_remainder"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02-020"));
}
#[test]
fn emitted_target_matrix_binds_all_target_classes_and_pending_local_validation() {
    for needle in [
        "target_class:linux",
        "target_class:windows",
        "target_class:mobile",
        "target_class:wasm",
        "target_class:baremetal",
        "target_class:other",
        "proof_family:canonical_io",
        "proof_family:deterministic_replay",
        "proof_family:host_boundary",
        "proof_family:receipt_chain",
        "proof_family:rollback_lane",
        "host_boundary_receipt=receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
    ] {
        assert!(
            TARGET_MATRIX.contains(needle),
            "target matrix missing {needle}"
        );
    }
    assert!(OPERATOR_SURFACE.contains("lyra-p02-target-matrix-check"));
    assert!(PRODUCT_SURFACE.contains("target_count=12"));
    assert!(PRODUCT_SURFACE.contains("proof_count=60"));
    assert!(!TARGET_MATRIX.contains("global_closure=true"));
}
