const CONTRACT: &str = include_str!("../interfaces/p00/contracts/output_table.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const TABLE: &str = include_str!("../ops/p00/closure/output_table.lyra");

#[test]
fn output_table_contract_names_p00_x04() {
    assert!(CONTRACT.contains("LYRA-P00-OUTPUT-TABLE v1"));
    assert!(CONTRACT.contains("task=P00-X04"));
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
fn control_files_advance_to_output_table_and_keep_phase_open() {
    assert!(CONTROL_FRONTIER.contains("current_task=P00-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P00-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P01"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_closure_outputs=none"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(!BLOCKER_INDEX.contains("blocker:P00-X04="));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}

#[test]
fn emitted_output_table_lists_required_audiences_contracts_receipts_and_gap() {
    for audience in [
        "audience:developer",
        "audience:operator",
        "audience:product",
        "audience:enterprise",
        "audience:public_interest",
    ] {
        assert!(TABLE.contains(audience));
    }
    assert!(TABLE.contains("contract:contract_output_table"));
    assert!(TABLE.contains("receipt:receipt_output_table"));
    assert!(TABLE.contains("gap:retirement_supersession_law"));
    assert!(!TABLE.contains("global closure true"));
}
