const CONTRACT: &str = include_str!("../interfaces/p01/contracts/semantic_proof_family.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p01/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p01/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p01/control/blocker_index.v1.lyra");
const PROOF_TABLE: &str = include_str!("../ops/p01/closure/semantic_proof_family_table.lyra");

#[test]
fn semantic_proof_family_contract_names_p01_x02() {
    assert!(CONTRACT.contains("LYRA-P01-SEMANTIC-PROOF-FAMILY-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P01-X02"));
    assert!(CONTRACT.contains("surface=LYRA-P01-SEMANTIC-PROOF-FAMILY-TABLE v1"));
    assert!(CONTRACT.contains("receipt=receipts/p01/pass_0055_semantic_proof_family.receipt"));
    assert!(CONTRACT.contains("next_frontier=P01-X03"));
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
    assert!(TRUTH_SNAPSHOT.contains("P01-X02"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P01-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02"));
}

#[test]
fn emitted_proof_table_lists_all_required_semantic_proof_families_without_global_closure() {
    for id in [
        "proof_family:happy_path",
        "proof_family:negative_path",
        "proof_family:adversarial_path",
        "proof_family:rollback_path",
    ] {
        assert!(PROOF_TABLE.contains(id));
    }
    for id in [
        "path:semantic_happy_primary_chain",
        "path:semantic_negative_rejection_chain",
        "path:semantic_adversarial_challenge_chain",
        "path:semantic_rollback_replay_chain",
    ] {
        assert!(PROOF_TABLE.contains(id));
    }
    assert!(!PROOF_TABLE.contains("global_closure:true"));
}
