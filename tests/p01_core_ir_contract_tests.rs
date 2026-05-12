const CONTRACT: &str = include_str!("../interfaces/p01/contracts/core_ir.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p01/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p01/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p01/control/blocker_index.v1.lyra");
const GUIDE: &str = include_str!("../docs/p01/core_ir_guide.lyra");
const EXAMPLE: &str = include_str!("../examples/p01/operator/core_ir_review.lyra");

#[test]
fn core_ir_contract_names_p01_002() {
    assert!(CONTRACT.contains("LYRA-P01-CORE-IR-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P01-002"));
    for item in [
        "form:text_ir",
        "form:binary_ir",
        "compatibility:forward",
        "compatibility:backward",
    ] {
        assert!(CONTRACT.contains(item), "contract missing {item}");
    }
}

#[test]
fn control_files_reflect_p01_x05_boundary_before_p02() {
    assert!(FRONTIER_LOCK.contains("current_task=P01-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P01-X04"));
    assert!(FRONTIER_LOCK.contains("next_frontier=P02"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P01-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("admitted_tasks=P01-001,P01-002"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_primary_tasks=none"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P01-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02"));
}

#[test]
fn developer_operator_surfaces_bind_real_p01_ir_command_and_receipt() {
    assert!(
        GUIDE.contains("lyra-p01-ir-check fixtures/p01/core_ir_inputs/valid_core_ir_forms.lyra")
    );
    assert!(GUIDE.contains("receipts/p01/pass_0031_core_ir.receipt"));
    assert!(
        EXAMPLE.contains("expected=accepted receipt-bound canonical text and binary ir bedrock")
    );
}
