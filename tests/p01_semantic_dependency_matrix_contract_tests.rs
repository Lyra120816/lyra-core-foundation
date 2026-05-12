const CONTRACT: &str =
    include_str!("../interfaces/p01/contracts/semantic_dependency_matrix.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p01/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p01/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p01/control/blocker_index.v1.lyra");
const MATRIX: &str = include_str!("../ops/p01/closure/semantic_dependency_matrix.lyra");

#[test]
fn semantic_dependency_matrix_contract_names_p01_x01() {
    assert!(CONTRACT.contains("LYRA-P01-SEMANTIC-DEPENDENCY-MATRIX v1"));
    assert!(CONTRACT.contains("task=P01-X01"));
    assert!(CONTRACT.contains("dependency:P01-024"));
    assert!(CONTRACT.contains("dependency:P01-X05"));
    assert!(CONTRACT.contains("blocker:P01-GLOBAL"));
    assert!(CONTRACT.contains("lane:closure_output_chain"));
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
    assert!(TRUTH_SNAPSHOT.contains("P01-X01"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P01-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02"));
}

#[test]
fn emitted_matrix_lists_all_semantic_closure_outputs_without_global_closure() {
    for id in ["P01-X01", "P01-X02", "P01-X03", "P01-X04", "P01-X05"] {
        assert!(MATRIX.contains(&format!("dependency:{id}")));
    }
    assert!(MATRIX.contains("blocker:P01-GLOBAL"));
    assert!(!MATRIX.contains("global_closure:true"));
}
