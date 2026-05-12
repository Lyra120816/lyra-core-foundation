const CONTRACT: &str = include_str!("../interfaces/p00/contracts/retirement_supersession.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const RETIREMENT_LAW: &str = include_str!("../ops/p00/closure/retirement_supersession_law.lyra");

#[test]
fn retirement_contract_names_p00_x05() {
    assert!(CONTRACT.contains("LYRA-P00-RETIREMENT-SUPERSESSION v1"));
    assert!(CONTRACT.contains("task=P00-X05"));
    for surface in [
        "surface:rust_bootstrap_crate",
        "surface:p00_cli_checks",
        "surface:p00_text_contracts",
        "surface:p00_control_plane",
        "surface:p00_receipt_format",
        "surface:p00_hash_algorithm",
    ] {
        assert!(CONTRACT.contains(surface), "contract missing {surface}");
    }
}

#[test]
fn control_files_advance_to_retirement_and_keep_truthful_global_blocker() {
    assert!(CONTROL_FRONTIER.contains("current_task=P00-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P00-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P01"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_closure_outputs=none"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}

#[test]
fn emitted_retirement_law_binds_surfaces_gates_supersessions_and_receipts() {
    for needle in [
        "surface:rust_bootstrap_crate",
        "gate:gate_rust_bootstrap_crate",
        "supersession:supersede_rust_bootstrap_crate",
        "receipt:receipt_retirement_supersession",
    ] {
        assert!(
            RETIREMENT_LAW.contains(needle),
            "retirement law missing {needle}"
        );
    }
    assert!(RETIREMENT_LAW.contains("archive:historical/superseded"));
    assert!(RETIREMENT_LAW.contains("archive:retained/active"));
    assert!(!RETIREMENT_LAW.contains("global closure true"));
}
