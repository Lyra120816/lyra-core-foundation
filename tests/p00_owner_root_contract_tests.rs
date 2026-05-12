use lyra_phase0::p00::{
    P00_OWNER_ROOT_LAW_CONTRACT, REQUIRED_OWNER_ROOTS, REQUIRED_OWNER_ROOT_RULES,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/owner_root_law.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const CLI: &str = include_str!("../src/bin/lyra-p00-owner-root-check.rs");
const LIB: &str = include_str!("../src/lib.rs");

#[test]
fn owner_root_contract_binds_runtime_header_rules_and_roots() {
    assert!(CONTRACT.contains(P00_OWNER_ROOT_LAW_CONTRACT));
    for rule in REQUIRED_OWNER_ROOT_RULES {
        assert!(CONTRACT.contains(rule), "contract missing rule {rule}");
    }
    for root in REQUIRED_OWNER_ROOTS {
        assert!(
            CONTRACT.contains(&format!("root:{root}=")),
            "contract missing root {root}"
        );
    }
}

#[test]
fn owner_root_cli_and_exports_are_wired() {
    assert!(CLI.contains("validate_owner_root_law_surface"));
    assert!(LIB.contains("p00_owner_root"));
    assert!(LIB.contains("p00_owner_root_model"));
}

#[test]
fn control_files_bind_p00_008_and_next_frontier() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("latest_finished_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
