use lyra_phase0::p00::{P00_CONSTITUTION_CONTRACT, P00_GOVERNANCE_REQUIREMENTS};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/constitution_surface.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn contract_surface_binds_runtime_header_and_requirements() {
    assert!(CONTRACT.contains(P00_CONSTITUTION_CONTRACT));
    for requirement in P00_GOVERNANCE_REQUIREMENTS {
        assert!(
            CONTRACT.contains(requirement.name),
            "contract does not bind runtime requirement {}:{}",
            requirement.namespace,
            requirement.name
        );
    }
}

#[test]
fn frontier_lock_names_current_frontier_and_previous_slice() {
    assert!(FRONTIER_LOCK.contains("phase=P00"));
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("current_work_package=P00-closure-outputs"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK.contains("allowed_claim=retirement_supersession_artifact_emitted"));
    assert!(FRONTIER_LOCK.contains("rejected_claim=global_complete"));
}

#[test]
fn truth_snapshot_refuses_phase_closure() {
    assert!(TRUTH_SNAPSHOT.contains("status=closure_outputs_artifact_emitted"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("global_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_primary_tasks=none"));
}

#[test]
fn blocker_index_keeps_next_frontiers_explicit() {
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
