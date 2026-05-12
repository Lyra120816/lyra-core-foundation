use lyra_phase0::p00::{
    P00_AUTHORITY_ORDER_CONTRACT, REQUIRED_AUTHORITY_LAYERS, REQUIRED_AUTHORITY_RULES,
};

const AUTHORITY_CONTRACT: &str =
    include_str!("../interfaces/p00/contracts/authority_order.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const PASS_RECEIPT: &str = include_str!("../receipts/p00/pass_0002_authority_order.receipt");

#[test]
fn authority_contract_binds_runtime_header_layers_and_rules() {
    assert!(AUTHORITY_CONTRACT.contains(P00_AUTHORITY_ORDER_CONTRACT));
    for layer in REQUIRED_AUTHORITY_LAYERS {
        assert!(
            AUTHORITY_CONTRACT.contains(layer.name),
            "missing layer {}",
            layer.name
        );
        assert!(
            AUTHORITY_CONTRACT.contains(&format!("rank:{:03}", layer.rank)),
            "missing rank {:03}",
            layer.rank
        );
    }
    for rule in REQUIRED_AUTHORITY_RULES {
        assert!(AUTHORITY_CONTRACT.contains(rule), "missing rule {rule}");
    }
}

#[test]
fn frontier_lock_keeps_current_p00_frontier_without_phase_closure() {
    assert!(FRONTIER_LOCK.contains("phase=P00"));
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK.contains("allowed_claim=retirement_supersession_artifact_emitted"));
    assert!(FRONTIER_LOCK.contains("rejected_claim=global_complete"));
}

#[test]
fn truth_snapshot_keeps_p00_open_and_current_frontier_explicit() {
    assert!(TRUTH_SNAPSHOT.contains("status=closure_outputs_artifact_emitted"));
    assert!(TRUTH_SNAPSHOT.contains("closed=false"));
    assert!(TRUTH_SNAPSHOT.contains("latest_finished_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
}

#[test]
fn blocker_index_names_next_immediate_frontier_after_challenge_law() {
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
}

#[test]
fn authority_order_receipt_remains_preserved_after_frontier_advance() {
    assert!(PASS_RECEIPT.contains("verdict=ACCEPTED"));
    assert!(PASS_RECEIPT.contains("receipt_hash=fnv1a128:"));
}
