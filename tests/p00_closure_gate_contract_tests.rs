use lyra_phase0::p00::{
    P00_CLOSURE_GATE_CONTRACT, REQUIRED_CLOSURE_OUTPUTS, REQUIRED_CLOSURE_PROOFS,
    REQUIRED_CLOSURE_RULES, REQUIRED_CLOSURE_TASKS,
};

#[test]
fn closure_gate_contract_names_required_surface() {
    let contract = std::fs::read_to_string("interfaces/p00/contracts/closure_gate.v1.lyra")
        .expect("contract must exist");
    assert!(contract.contains(P00_CLOSURE_GATE_CONTRACT));
    assert!(contract.contains("task=P00-024"));
    assert!(contract.contains("bounded_closure=true"));
    assert!(contract.contains("global_closure=false"));
    assert!(contract.contains("next_frontier=P00-X01"));
    for required in REQUIRED_CLOSURE_RULES
        .iter()
        .chain(REQUIRED_CLOSURE_TASKS.iter())
        .chain(REQUIRED_CLOSURE_OUTPUTS.iter())
        .chain(REQUIRED_CLOSURE_PROOFS.iter())
    {
        assert!(contract.contains(required), "contract missing {required}");
    }
}

#[test]
fn closure_gate_control_surfaces_advance_to_closure_outputs() {
    let frontier =
        std::fs::read_to_string("ops/p00/control/frontier_lock.v1.lyra").expect("frontier exists");
    let truth =
        std::fs::read_to_string("ops/p00/control/truth_snapshot.v1.lyra").expect("truth exists");
    let blockers =
        std::fs::read_to_string("ops/p00/control/blocker_index.v1.lyra").expect("blockers exists");
    assert!(frontier.contains("current_task=P00-X05"));
    assert!(frontier.contains("next_frontier=P01"));
    assert!(truth.contains("current_frontier=P00-X05"));
    assert!(truth.contains("bounded_closure=true"));
    assert!(truth.contains("global_closure=false"));
    assert!(truth.contains("remaining_primary_tasks=none"));
    assert!(truth.contains("remaining_closure_outputs=none"));
    assert!(blockers.contains("current_frontier=P00-X05"));
    assert!(blockers.contains("next_immediate_frontier=P01"));
}
