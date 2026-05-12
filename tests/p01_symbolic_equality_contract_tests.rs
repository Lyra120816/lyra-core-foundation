const CONTRACT: &str = include_str!("../interfaces/p01/contracts/symbolic_equality.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p01/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p01/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p01/control/blocker_index.v1.lyra");
const LAW: &str = include_str!("../ops/p01/control/symbolic_equality_law.v1.lyra");
const GUIDE: &str = include_str!("../docs/p01/symbolic_equality_guide.lyra");
const EXAMPLE: &str = include_str!("../examples/p01/operator/symbolic_equality_review.lyra");

#[test]
fn symbolic_equality_contract_names_p01_006() {
    assert!(CONTRACT.contains("LYRA-P01-SYMBOLIC-EQUALITY-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P01-006"));
    for item in [
        "reflexive",
        "alpha_equivalent",
        "record_order_class",
        "capture_rejection",
        "fnv1a128",
    ] {
        assert!(CONTRACT.contains(item), "contract missing {item}");
    }
}

#[test]
fn control_files_reflect_p01_boundary_at_x05() {
    assert!(FRONTIER_LOCK.contains("current_task=P01-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P01-X04"));
    assert!(FRONTIER_LOCK.contains("next_frontier=P02"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p01/pass_0058_semantic_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P01-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("P01-006"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_primary_tasks=none"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P01-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02"));
    assert!(LAW.contains("task=P01-006"));
}

#[test]
fn developer_operator_surfaces_bind_real_symbolic_equality_command_and_receipt() {
    assert!(GUIDE.contains("lyra-p01-symbolic-equality-check fixtures/p01/symbolic_equality_inputs/valid_symbolic_equality.lyra"));
    assert!(GUIDE.contains("receipts/p01/pass_0035_symbolic_equality.receipt"));
    assert!(EXAMPLE.contains("expected=accepted receipt-bound symbolic equality suite"));
}
