const CONTRACT: &str = include_str!("../interfaces/p02/contracts/host_boundary_challenge.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");
const HOST_BOUNDARY_SURFACE: &str =
    include_str!("../ops/p02/host_boundary/host_boundary_challenge_suites.v1.lyra");
const OPERATOR_SURFACE: &str =
    include_str!("../shells/p02/host_boundary_challenge_operator_surface.lyra");
const PRODUCT_SURFACE: &str =
    include_str!("../products/p02/host_boundary_challenge_inspection_surface.lyra");
#[test]
fn host_boundary_contract_names_p02_005() {
    assert!(CONTRACT.contains("LYRA-P02-HOST-BOUNDARY-CHALLENGE-SUITES v1"));
    assert!(CONTRACT.contains("task=P02-005"));
    for suite in [
        "suite:suite_no_ambient_network_import",
        "suite:suite_no_ambient_time_truth",
        "suite:suite_no_hidden_randomness_truth",
        "suite:suite_no_unledgered_host_surface",
        "suite:suite_no_foreign_semantic_ownership",
        "suite:suite_operator_truth_containment",
        "suite:suite_foreign_runtime_quarantine",
    ] {
        assert!(CONTRACT.contains(suite), "contract missing {suite}");
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
fn emitted_host_boundary_surface_binds_all_foreign_surfaces() {
    for needle in [
        "probe:probe_rust_bootstrap_compiler",
        "probe:probe_unbounded_network_bootstrap_fetch",
        "probe:probe_external_wall_clock",
        "probe:probe_operator_shell_terminal",
        "expected_error:ambient_network_allowed",
        "containment_gate:gate_no_foreign_runtime_import",
        "session_receipt=receipts/p02/pass_0062_bootstrap_session_rituals.receipt",
    ] {
        assert!(
            HOST_BOUNDARY_SURFACE.contains(needle),
            "host boundary surface missing {needle}"
        );
    }
    assert!(OPERATOR_SURFACE.contains("lyra-p02-host-boundary-check"));
    assert!(PRODUCT_SURFACE.contains("suite_count=7"));
    assert!(PRODUCT_SURFACE.contains("probe_count=17"));
    assert!(!HOST_BOUNDARY_SURFACE.contains("global_closure=true"));
}
