use lyra_phase0::p00::{
    P00_DEPLOYMENT_CONTRACT, REQUIRED_COMPLIANCE_HOOKS, REQUIRED_DEPLOYMENT_PROOFS,
    REQUIRED_DEPLOYMENT_RULES, REQUIRED_DEPLOYMENT_TARGETS, REQUIRED_RELEASE_EVIDENCE,
};

#[test]
fn deployment_contract_names_required_surface() {
    let contract = std::fs::read_to_string("interfaces/p00/contracts/deployment_hooks.v1.lyra")
        .expect("contract must exist");
    assert!(contract.contains(P00_DEPLOYMENT_CONTRACT));
    assert!(contract.contains("task=P00-020"));
    for required in REQUIRED_DEPLOYMENT_RULES
        .iter()
        .chain(REQUIRED_DEPLOYMENT_TARGETS.iter())
        .chain(REQUIRED_COMPLIANCE_HOOKS.iter())
        .chain(REQUIRED_RELEASE_EVIDENCE.iter())
        .chain(REQUIRED_DEPLOYMENT_PROOFS.iter())
    {
        assert!(contract.contains(required), "contract missing {required}");
    }
}

#[test]
fn deployment_control_surfaces_advance_frontier() {
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
