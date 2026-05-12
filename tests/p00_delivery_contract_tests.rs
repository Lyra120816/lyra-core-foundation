use lyra_phase0::p00::{
    P00_DELIVERY_PROTOCOL_CONTRACT, REQUIRED_DELIVERY_ARTIFACTS, REQUIRED_DELIVERY_PROOF_FAMILIES,
    REQUIRED_DELIVERY_RULES,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/delivery_protocol.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn delivery_contract_binds_runtime_header_rules_and_proof_families() {
    assert!(CONTRACT.contains(P00_DELIVERY_PROTOCOL_CONTRACT));
    for rule in REQUIRED_DELIVERY_RULES {
        assert!(
            CONTRACT.contains(rule),
            "contract missing delivery rule {rule}"
        );
    }
    for family in REQUIRED_DELIVERY_PROOF_FAMILIES {
        assert!(
            CONTRACT.contains(family),
            "contract missing proof family {family}"
        );
    }
}

#[test]
fn delivery_contract_binds_required_artifacts() {
    for artifact in REQUIRED_DELIVERY_ARTIFACTS {
        assert!(
            CONTRACT.contains(artifact.id),
            "contract missing artifact id {}",
            artifact.id
        );
        assert!(
            CONTRACT.contains(artifact.path),
            "contract missing artifact path {}",
            artifact.path
        );
    }
}

#[test]
fn delivery_control_surfaces_are_advanced_without_closure() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK.contains("allowed_claim=retirement_supersession_artifact_emitted"));
    assert!(TRUTH_SNAPSHOT.contains("closed=false"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
