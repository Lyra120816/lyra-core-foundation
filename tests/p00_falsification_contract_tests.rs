use lyra_phase0::p00::{
    validate_falsification_surface, P00_FALSIFICATION_CONTRACT, REQUIRED_FALSIFICATION_HARNESSES,
    REQUIRED_FALSIFICATION_PROOFS, REQUIRED_FALSIFICATION_RULES, REQUIRED_NEGATIVE_CASES,
    REQUIRED_REJECTION_ASSERTIONS,
};

const VALID: &str =
    include_str!("../fixtures/p00/falsification_inputs/valid_falsification_corpus.lyra");
const CONTRACT: &str = include_str!("../interfaces/p00/contracts/falsification_corpus.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn falsification_contract_names_required_entities() {
    assert!(CONTRACT.contains("LYRA-P00-FALSIFICATION-CORPUS-CONTRACT v1"));
    for case_id in REQUIRED_NEGATIVE_CASES {
        assert!(
            CONTRACT.contains(case_id),
            "contract missing negative case {case_id}"
        );
    }
    for harness in REQUIRED_FALSIFICATION_HARNESSES {
        assert!(
            CONTRACT.contains(harness),
            "contract missing harness {harness}"
        );
    }
    for assertion in REQUIRED_REJECTION_ASSERTIONS {
        assert!(
            CONTRACT.contains(assertion),
            "contract missing assertion {assertion}"
        );
    }
    for proof in REQUIRED_FALSIFICATION_PROOFS {
        assert!(CONTRACT.contains(proof), "contract missing proof {proof}");
    }
}

#[test]
fn falsification_surface_header_matches_runtime_contract() {
    assert_eq!(
        P00_FALSIFICATION_CONTRACT,
        "LYRA-P00-FALSIFICATION-CORPUS v1"
    );
    let (verdict, _receipt) = validate_falsification_surface(VALID);
    assert!(
        verdict.accepted,
        "valid falsification contract fixture rejected: {:?}",
        verdict.errors
    );
    for rule in REQUIRED_FALSIFICATION_RULES {
        assert!(
            VALID.contains(&format!("rule:{rule}=")),
            "fixture missing rule {rule}"
        );
    }
}

#[test]
fn falsification_control_surfaces_are_advanced_to_p00_016() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("P00-016"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
