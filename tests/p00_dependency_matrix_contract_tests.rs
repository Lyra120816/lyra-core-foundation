const CONTRACT: &str = include_str!("../interfaces/p00/contracts/dependency_matrix.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const MATRIX: &str = include_str!("../ops/p00/closure/dependency_matrix.lyra");

#[test]
fn dependency_matrix_contract_names_p00_x01() {
    assert!(CONTRACT.contains("LYRA-P00-DEPENDENCY-MATRIX v1"));
    assert!(CONTRACT.contains("task=P00-X01"));
    assert!(CONTRACT.contains("dependency:P00-024"));
    assert!(CONTRACT.contains("blocker:P00-X02"));
    assert!(CONTRACT.contains("lane:closure_output_chain"));
}

#[test]
fn control_files_advance_to_dependency_matrix_and_keep_phase_open() {
    assert!(CONTROL_FRONTIER.contains("current_task=P00-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P00-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P01"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_closure_outputs=none"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}

#[test]
fn emitted_matrix_lists_all_closure_outputs_without_global_closure() {
    for id in ["P00-X01", "P00-X02", "P00-X03", "P00-X04", "P00-X05"] {
        assert!(MATRIX.contains(&format!("dependency:{id}")));
    }
    assert!(MATRIX.contains("blocker:P00-GLOBAL"));
    assert!(!MATRIX.contains("global closure true"));
}
