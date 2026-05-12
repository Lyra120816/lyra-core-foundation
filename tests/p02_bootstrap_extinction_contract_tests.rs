const CONTRACT: &str =
    include_str!("../interfaces/p02/contracts/bootstrap_extinction_ledger.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");
const LEDGER: &str = include_str!("../ops/p02/extinction/bootstrap_extinction_ledger.v1.lyra");
const OPERATOR_SURFACE: &str =
    include_str!("../shells/p02/bootstrap_extinction_operator_surface.lyra");
const PRODUCT_SURFACE: &str =
    include_str!("../products/p02/bootstrap_extinction_inspection_surface.lyra");

#[test]
fn bootstrap_extinction_contract_names_p02_002() {
    assert!(CONTRACT.contains("LYRA-P02-BOOTSTRAP-EXTINCTION-LEDGER v1"));
    assert!(CONTRACT.contains("task=P02-002"));
    for entry in [
        "entry:rust_bootstrap_compiler",
        "entry:cargo_build_driver",
        "entry:physical_cpu_instruction_set",
        "entry:unbounded_network_bootstrap_fetch",
    ] {
        assert!(CONTRACT.contains(entry), "contract missing {entry}");
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
fn emitted_ledger_binds_every_extinction_class_and_operator_surface() {
    for needle in [
        "classification:temporary",
        "classification:observer",
        "classification:bounded_permanent",
        "classification:forbidden",
        "deletion_action:deny_import_and_delete_reference",
        "ledger_state:forbidden_no_import",
        "ledger_state:retained_by_target_descriptor",
        "inventory_receipt=receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
    ] {
        assert!(LEDGER.contains(needle), "ledger missing {needle}");
    }
    assert!(OPERATOR_SURFACE.contains("lyra-p02-bootstrap-extinction-check"));
    assert!(PRODUCT_SURFACE.contains("entry_count=17"));
    assert!(!LEDGER.contains("global_closure=true"));
}
