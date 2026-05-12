use lyra_phase0::p00::{
    P00_PUBLIC_INTEREST_LAW_CONTRACT, REQUIRED_ANTI_EXTRACTIVE_DUTIES,
    REQUIRED_PARTICIPATION_CONSTITUENCIES, REQUIRED_PUBLIC_INTEREST_RULES,
    REQUIRED_PUBLIC_INTEREST_SAFEGUARDS,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/public_interest_law.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const CLI: &str = include_str!("../src/bin/lyra-p00-public-interest-check.rs");
const LIB: &str = include_str!("../src/lib.rs");

#[test]
fn public_interest_contract_binds_runtime_header_rules_and_required_surfaces() {
    assert!(CONTRACT.contains(P00_PUBLIC_INTEREST_LAW_CONTRACT));
    for rule in REQUIRED_PUBLIC_INTEREST_RULES {
        assert!(
            CONTRACT.contains(rule),
            "contract missing public-interest rule {rule}"
        );
    }
    for safeguard in REQUIRED_PUBLIC_INTEREST_SAFEGUARDS {
        assert!(
            CONTRACT.contains(&format!("safeguard:{safeguard}=")),
            "contract missing safeguard {safeguard}"
        );
    }
    for constituency in REQUIRED_PARTICIPATION_CONSTITUENCIES {
        assert!(
            CONTRACT.contains(&format!("constituency:{constituency}")),
            "contract missing constituency {constituency}"
        );
    }
    for duty in REQUIRED_ANTI_EXTRACTIVE_DUTIES {
        assert!(
            CONTRACT.contains(&format!("duty:{duty}=")),
            "contract missing duty {duty}"
        );
    }
}

#[test]
fn public_interest_cli_and_exports_are_wired() {
    assert!(CLI.contains("validate_public_interest_law_surface"));
    assert!(LIB.contains("p00_public_interest"));
    assert!(LIB.contains("p00_public_interest_model"));
}

#[test]
fn control_plane_advances_to_p00_010_without_closing_phase() {
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
