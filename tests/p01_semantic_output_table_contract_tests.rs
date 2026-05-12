const CONTRACT: &str = include_str!("../interfaces/p01/contracts/semantic_output_table.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p01/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p01/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p01/control/blocker_index.v1.lyra");
const TABLE: &str = include_str!("../ops/p01/closure/semantic_output_table.lyra");

#[test]
fn semantic_output_table_contract_names_p01_x04() {
    assert!(CONTRACT.contains("LYRA-P01-SEMANTIC-OUTPUT-TABLE-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P01-X04"));
    for audience in [
        "audience:developer",
        "audience:operator",
        "audience:product",
        "audience:enterprise",
        "audience:public_interest",
    ] {
        assert!(CONTRACT.contains(audience));
    }
}

#[test]
fn control_files_reflect_p01_boundary_at_x05() {
    assert!(CONTROL_FRONTIER.contains("current_task=P01-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P01-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P02"));
    assert!(CONTROL_FRONTIER
        .contains("truth_bound=receipts/p01/pass_0058_semantic_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P01-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_closure_outputs=none"));
    assert!(TRUTH_SNAPSHOT.contains("P01-X04"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P01-X05"));
    assert!(!BLOCKER_INDEX.contains("blocker:P01-X04="));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02"));
}

#[test]
fn emitted_semantic_output_table_lists_required_audiences_contracts_receipts_and_gap() {
    for audience in [
        "audience:developer",
        "audience:operator",
        "audience:product",
        "audience:enterprise",
        "audience:public_interest",
    ] {
        assert!(TABLE.contains(audience));
    }
    assert!(TABLE.contains("contract:contract_semantic_output_table"));
    assert!(TABLE.contains("receipt:receipt_semantic_output_table"));
    assert!(TABLE.contains("gap:semantic_retirement_supersession_law"));
    assert!(!TABLE.contains("global closure true"));
}
