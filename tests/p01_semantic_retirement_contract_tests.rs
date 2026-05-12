const CONTRACT: &str =
    include_str!("../interfaces/p01/contracts/semantic_retirement_supersession.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p01/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p01/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p01/control/blocker_index.v1.lyra");
const RETIREMENT_LAW: &str =
    include_str!("../ops/p01/closure/semantic_retirement_supersession_law.lyra");

#[test]
fn semantic_retirement_contract_names_p01_x05() {
    assert!(CONTRACT.contains("LYRA-P01-SEMANTIC-RETIREMENT-SUPERSESSION v1"));
    assert!(CONTRACT.contains("task=P01-X05"));
    for surface in [
        "surface:rust_semantic_bootstrap_crate",
        "surface:p01_semantic_cli_checks",
        "surface:p01_semantic_text_contracts",
        "surface:p01_semantic_control_plane",
        "surface:p01_semantic_core_ir_carrier",
        "surface:p01_semantic_output_table",
    ] {
        assert!(CONTRACT.contains(surface), "contract missing {surface}");
    }
}

#[test]
fn control_files_advance_to_semantic_retirement_and_keep_truthful_local_blocker() {
    assert!(CONTROL_FRONTIER.contains("current_task=P01-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P01-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P02"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P01-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_closure_outputs=none"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P01-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02"));
}

#[test]
fn emitted_semantic_retirement_law_binds_surfaces_gates_supersessions_and_receipts() {
    for needle in [
        "surface:rust_semantic_bootstrap_crate",
        "gate:gate_rust_semantic_bootstrap_crate",
        "supersession:supersede_rust_semantic_bootstrap_crate",
        "receipt:receipt_semantic_retirement_supersession",
    ] {
        assert!(
            RETIREMENT_LAW.contains(needle),
            "semantic retirement law missing {needle}"
        );
    }
    assert!(RETIREMENT_LAW.contains("archive:historical/superseded"));
    assert!(RETIREMENT_LAW.contains("archive:retained/active"));
    assert!(!RETIREMENT_LAW.contains("global closure true"));
}
