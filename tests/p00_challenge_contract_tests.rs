use lyra_phase0::p00::{
    P00_CHALLENGE_LAW_CONTRACT, REQUIRED_AMENDMENT_GATES, REQUIRED_CHALLENGE_RIGHTS,
    REQUIRED_CHALLENGE_RULES, REQUIRED_REVIEW_GATES, REQUIRED_ROLLBACK_AUTHORITIES,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/challenge_law.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn challenge_contract_binds_runtime_header_rules_and_required_gates() {
    assert!(CONTRACT.contains(P00_CHALLENGE_LAW_CONTRACT));
    for rule in REQUIRED_CHALLENGE_RULES {
        assert!(
            CONTRACT.contains(rule),
            "contract missing challenge rule {rule}"
        );
    }
    for review in REQUIRED_REVIEW_GATES {
        assert!(
            CONTRACT.contains(review),
            "contract missing review gate {review}"
        );
    }
    for challenge in REQUIRED_CHALLENGE_RIGHTS {
        assert!(
            CONTRACT.contains(challenge),
            "contract missing challenge right {challenge}"
        );
    }
    for rollback in REQUIRED_ROLLBACK_AUTHORITIES {
        assert!(
            CONTRACT.contains(rollback),
            "contract missing rollback authority {rollback}"
        );
    }
    for amendment in REQUIRED_AMENDMENT_GATES {
        assert!(
            CONTRACT.contains(amendment),
            "contract missing amendment gate {amendment}"
        );
    }
}

#[test]
fn challenge_control_surfaces_are_advanced_without_closure() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("current_work_package=P00-closure-outputs"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK.contains("allowed_claim=retirement_supersession_artifact_emitted"));
    assert!(TRUTH_SNAPSHOT.contains("closed=false"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
