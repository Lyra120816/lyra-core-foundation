use lyra_phase0::p00::{
    validate_engine_surface, P00_ENGINE_CONTRACT, REQUIRED_ENGINE_PROOFS, REQUIRED_ENGINE_RULES,
    REQUIRED_ENGINE_TRANSITIONS, REQUIRED_ENGINE_UNITS,
};

const VALID: &str =
    include_str!("../fixtures/p00/deterministic_engine_inputs/valid_deterministic_engine.lyra");
const CONTRACT: &str = include_str!("../interfaces/p00/contracts/deterministic_engine.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn deterministic_engine_contract_names_required_entities() {
    assert!(CONTRACT.contains("LYRA-P00-DETERMINISTIC-ENGINE-CONTRACT v1"));
    for unit in REQUIRED_ENGINE_UNITS {
        assert!(
            CONTRACT.contains(unit),
            "contract missing engine unit {unit}"
        );
    }
    for transition in REQUIRED_ENGINE_TRANSITIONS {
        assert!(
            CONTRACT.contains(transition),
            "contract missing transition {transition}"
        );
    }
    for proof in REQUIRED_ENGINE_PROOFS {
        assert!(CONTRACT.contains(proof), "contract missing proof {proof}");
    }
}

#[test]
fn deterministic_engine_surface_header_matches_runtime_contract() {
    assert_eq!(P00_ENGINE_CONTRACT, "LYRA-P00-DETERMINISTIC-ENGINE v1");
    let (verdict, _receipt) = validate_engine_surface(VALID);
    assert!(
        verdict.accepted,
        "valid engine contract fixture rejected: {:?}",
        verdict.errors
    );
    for rule in REQUIRED_ENGINE_RULES {
        assert!(
            VALID.contains(&format!("rule:{rule}=")),
            "fixture missing rule {rule}"
        );
    }
}

#[test]
fn deterministic_engine_control_surfaces_are_advanced_to_p00_015() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("P00-015"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
