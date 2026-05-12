use lyra_phase0::p00::{
    P00_ACCEPTANCE_PROOF_CONTRACT, REQUIRED_ACCEPTANCE_GOLDENS, REQUIRED_ACCEPTANCE_PROOFS,
    REQUIRED_ACCEPTANCE_RULES, REQUIRED_CHALLENGE_FIXTURES,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/acceptance_proof.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const CLI: &str = include_str!("../src/bin/lyra-p00-acceptance-check.rs");
const LIB: &str = include_str!("../src/lib.rs");

#[test]
fn acceptance_contract_binds_runtime_header_rules_goldens_fixtures_and_proofs() {
    assert!(CONTRACT.contains(P00_ACCEPTANCE_PROOF_CONTRACT));
    for rule in REQUIRED_ACCEPTANCE_RULES {
        assert!(
            CONTRACT.contains(&format!("rule:{rule}=")),
            "contract missing acceptance rule {rule}"
        );
    }
    for golden in REQUIRED_ACCEPTANCE_GOLDENS {
        assert!(
            CONTRACT.contains(&format!("golden:{golden}=")),
            "contract missing golden {golden}"
        );
    }
    for fixture in REQUIRED_CHALLENGE_FIXTURES {
        assert!(
            CONTRACT.contains(&format!("fixture:{fixture}=")),
            "contract missing challenge fixture {fixture}"
        );
    }
    for proof in REQUIRED_ACCEPTANCE_PROOFS {
        assert!(
            CONTRACT.contains(&format!("proof:{proof}=")),
            "contract missing proof {proof}"
        );
    }
}

#[test]
fn acceptance_cli_and_exports_are_wired() {
    assert!(CLI.contains("validate_acceptance_proof_surface"));
    assert!(LIB.contains("p00_acceptance"));
    assert!(LIB.contains("p00_acceptance_model"));
}

#[test]
fn control_plane_advances_to_p00_012_without_closing_phase() {
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
