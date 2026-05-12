use lyra_phase0::p00::{
    P00_REPLAY_WITNESS_CONTRACT, REQUIRED_RECEIPT_CHAIN_LINKS, REQUIRED_REPLAY_PROOFS,
    REQUIRED_REPLAY_RECEIPTS, REQUIRED_REPLAY_RULES, REQUIRED_REPLAY_WITNESSES,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/replay_witness.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn replay_contract_binds_runtime_header_and_required_rules() {
    assert!(CONTRACT.contains(P00_REPLAY_WITNESS_CONTRACT));
    for item in REQUIRED_REPLAY_RULES {
        assert!(
            CONTRACT.contains(item),
            "contract misses replay rule {item}"
        );
    }
}

#[test]
fn replay_contract_names_all_required_runtime_families() {
    assert_eq!(REQUIRED_REPLAY_RECEIPTS.len(), 17);
    assert_eq!(REQUIRED_REPLAY_WITNESSES.len(), 6);
    assert_eq!(REQUIRED_RECEIPT_CHAIN_LINKS.len(), 16);
    assert_eq!(REQUIRED_REPLAY_PROOFS.len(), 4);
}

#[test]
fn control_surfaces_advance_to_p00_017_without_phase_closure() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
