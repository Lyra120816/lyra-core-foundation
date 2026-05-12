use lyra_phase0::p00::{
    P00_REDTEAM_ROLLBACK_CONTRACT, REQUIRED_REDTEAM_PROOFS, REQUIRED_REDTEAM_RULES,
    REQUIRED_REDTEAM_SCENARIOS, REQUIRED_ROLLBACK_PATHS,
};

#[test]
fn redteam_contract_names_required_surface() {
    let contract = std::fs::read_to_string("interfaces/p00/contracts/redteam_rollback.v1.lyra")
        .expect("contract must exist");
    assert!(contract.contains(P00_REDTEAM_ROLLBACK_CONTRACT));
    assert!(contract.contains("task=P00-023"));
    for required in REQUIRED_REDTEAM_RULES
        .iter()
        .chain(REQUIRED_REDTEAM_SCENARIOS.iter())
        .chain(REQUIRED_ROLLBACK_PATHS.iter())
        .chain(REQUIRED_REDTEAM_PROOFS.iter())
    {
        assert!(contract.contains(required), "contract missing {required}");
    }
}

#[test]
fn redteam_control_surfaces_advance_frontier() {
    let frontier =
        std::fs::read_to_string("ops/p00/control/frontier_lock.v1.lyra").expect("frontier exists");
    let truth =
        std::fs::read_to_string("ops/p00/control/truth_snapshot.v1.lyra").expect("truth exists");
    let blockers =
        std::fs::read_to_string("ops/p00/control/blocker_index.v1.lyra").expect("blockers exists");
    assert!(frontier.contains("current_task=P00-X05"));
    assert!(frontier.contains("next_frontier=P01"));
    assert!(truth.contains("current_frontier=P00-X05"));
    assert!(truth.contains("latest_receipt=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(blockers.contains("current_frontier=P00-X05"));
    assert!(blockers.contains("next_immediate_frontier=P01"));
}
