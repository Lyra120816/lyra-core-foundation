use lyra_phase0::p00::{P00_IDENTITY_LAW_CONTRACT, REQUIRED_IDENTITY_RULES};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/identity_law.v1.lyra");
const VALID: &str = include_str!("../fixtures/p00/identity_law_inputs/valid_identity_law.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn identity_contract_names_the_runtime_surface_header() {
    assert!(CONTRACT.contains("LYRA-P00-IDENTITY-LAW-CONTRACT v1"));
    assert!(CONTRACT.contains(P00_IDENTITY_LAW_CONTRACT));
    assert!(CONTRACT.contains("required_task_chain=P00-001,P00-002,P00-003"));
}

#[test]
fn valid_identity_fixture_carries_every_required_rule() {
    for rule in REQUIRED_IDENTITY_RULES {
        let expected = format!("rule:{rule}=required");
        assert!(VALID.contains(&expected), "missing {expected}");
    }
}

#[test]
fn control_surfaces_are_advanced_to_current_frontier_without_closure() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
