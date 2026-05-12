use lyra_phase0::p00::{
    P00_FORMAL_SEMANTICS_CONTRACT, REQUIRED_FORMAL_SEMANTIC_RULES, REQUIRED_INVARIANT_BINDINGS,
    REQUIRED_SEMANTIC_DOMAINS, REQUIRED_SEMANTIC_PROOFS, REQUIRED_SEMANTIC_RULE_BINDINGS,
    REQUIRED_TRANSITION_LAWS,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/formal_semantics.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const CLI: &str = include_str!("../src/bin/lyra-p00-formal-semantics-check.rs");
const LIB: &str = include_str!("../src/lib.rs");

#[test]
fn formal_semantics_contract_binds_runtime_header_and_required_semantics() {
    assert!(CONTRACT.contains(P00_FORMAL_SEMANTICS_CONTRACT));
    for rule in REQUIRED_FORMAL_SEMANTIC_RULES {
        assert!(
            CONTRACT.contains(&format!("rule:{rule}=")),
            "contract missing formal semantic rule {rule}"
        );
    }
    for domain in REQUIRED_SEMANTIC_DOMAINS {
        assert!(
            CONTRACT.contains(&format!("domain:{domain}=")),
            "contract missing semantic domain {domain}"
        );
    }
    for rule in REQUIRED_SEMANTIC_RULE_BINDINGS {
        assert!(
            CONTRACT.contains(&format!("semantic_rule:{rule}=")),
            "contract missing semantic rule binding {rule}"
        );
    }
    for transition in REQUIRED_TRANSITION_LAWS {
        assert!(
            CONTRACT.contains(&format!("transition:{transition}=")),
            "contract missing transition law {transition}"
        );
    }
    for invariant in REQUIRED_INVARIANT_BINDINGS {
        assert!(
            CONTRACT.contains(&format!("invariant:{invariant}=")),
            "contract missing invariant binding {invariant}"
        );
    }
    for proof in REQUIRED_SEMANTIC_PROOFS {
        assert!(
            CONTRACT.contains(&format!("proof:{proof}=")),
            "contract missing semantic proof {proof}"
        );
    }
}

#[test]
fn formal_semantics_cli_and_exports_are_wired() {
    assert!(CLI.contains("validate_formal_semantics_surface"));
    assert!(LIB.contains("p00_formal_semantics"));
    assert!(LIB.contains("p00_formal_semantics_model"));
}

#[test]
fn control_plane_advances_to_p00_013_without_closing_phase() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("status=closure_outputs_artifact_emitted"));
    assert!(TRUTH_SNAPSHOT.contains("closed=false"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
