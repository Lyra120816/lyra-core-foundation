const CONTRACT: &str =
    include_str!("../interfaces/p02/contracts/bootstrap_surface_inventory.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");
const INVENTORY: &str = include_str!("../ops/p02/inventory/bootstrap_surface_inventory.v1.lyra");
const OPERATOR_SURFACE: &str =
    include_str!("../shells/p02/bootstrap_inventory_operator_surface.lyra");

#[test]
fn bootstrap_inventory_contract_names_p02_001() {
    assert!(CONTRACT.contains("LYRA-P02-BOOTSTRAP-SURFACE-INVENTORY v1"));
    assert!(CONTRACT.contains("task=P02-001"));
    for surface in [
        "surface:rust_bootstrap_compiler",
        "surface:cargo_build_driver",
        "surface:host_operating_system",
        "surface:physical_cpu_instruction_set",
        "surface:unbounded_network_bootstrap_fetch",
    ] {
        assert!(CONTRACT.contains(surface), "contract missing {surface}");
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
fn emitted_inventory_binds_all_classifications_and_operator_surface() {
    for needle in [
        "classification:temporary",
        "classification:observer",
        "classification:bounded_permanent",
        "classification:forbidden",
        "retirement_ref:P02-002",
        "retirement_ref:P02-009",
        "retirement_ref:bounded_by_target_descriptor",
        "retirement_ref:forbidden_surface_no_import",
    ] {
        assert!(INVENTORY.contains(needle), "inventory missing {needle}");
    }
    assert!(OPERATOR_SURFACE.contains("lyra-p02-bootstrap-inventory-check"));
    assert!(!INVENTORY.contains("global_closure=true"));
}
