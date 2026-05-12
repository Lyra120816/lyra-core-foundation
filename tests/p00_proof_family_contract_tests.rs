const CONTRACT: &str = include_str!("../interfaces/p00/contracts/proof_family_table.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const TABLE: &str = include_str!("../ops/p00/closure/proof_family_table.lyra");

#[test]
fn proof_family_contract_names_p00_x02() {
    assert!(CONTRACT.contains("LYRA-P00-PROOF-FAMILY-TABLE v1"));
    assert!(CONTRACT.contains("task=P00-X02"));
    assert!(CONTRACT.contains("proof_family:happy_path"));
    assert!(CONTRACT.contains("proof_family:negative_path"));
    assert!(CONTRACT.contains("proof_family:adversarial_path"));
    assert!(CONTRACT.contains("proof_family:rollback_path"));
}

#[test]
fn control_files_advance_to_proof_family_and_keep_phase_open() {
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
fn emitted_table_lists_all_required_path_families_without_global_closure() {
    for family in [
        "happy_path",
        "negative_path",
        "adversarial_path",
        "rollback_path",
    ] {
        assert!(TABLE.contains(&format!("proof_family:{family}")));
    }
    assert!(TABLE.contains("receipt:receipt_dependency_matrix"));
    assert!(!TABLE.contains("global closure true"));
}
