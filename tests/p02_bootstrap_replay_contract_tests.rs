use std::fs;
use std::path::Path;

use lyra_phase0::p02::{
    parse_bootstrap_replay_surface, validate_bootstrap_replay_surface,
    P02_BOOTSTRAP_REPLAY_CONTRACT, REQUIRED_BOOTSTRAP_REPLAY_ARTIFACTS,
    REQUIRED_BOOTSTRAP_REPLAY_LINKS, REQUIRED_BOOTSTRAP_REPLAY_PROOFS,
    REQUIRED_BOOTSTRAP_REPLAY_RECEIPTS, REQUIRED_BOOTSTRAP_REPLAY_RULES,
    REQUIRED_BOOTSTRAP_REPLAY_WITNESSES,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/valid_bootstrap_replay.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");

#[test]
fn p02_017_contract_binds_operator_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p02/contracts/bootstrap_replay.v1.lyra")
        .expect("P02-017 bootstrap replay contract must exist");

    assert!(contract.starts_with("LYRA-P02-BOOTSTRAP-REPLAY-CONTRACT v1"));
    assert!(contract.contains("task=P02-017"));
    assert!(contract.contains("surface=LYRA-P02-BOOTSTRAP-REPLAY-WITNESS v1"));
    assert!(contract.contains("receipt=receipts/p02/pass_0075_bootstrap_replay.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p02-bootstrap-replay-check.rs"));
    assert!(Path::new("ops/p02/replay/bootstrap_replay_witnesses.v1.lyra").exists());
}

#[test]
fn p02_017_valid_surface_matches_required_inventory() {
    let surface = parse_bootstrap_replay_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P02_BOOTSTRAP_REPLAY_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_BOOTSTRAP_REPLAY_RULES.len());
    assert_eq!(
        surface.receipts.len(),
        REQUIRED_BOOTSTRAP_REPLAY_RECEIPTS.len()
    );
    assert_eq!(
        surface.witnesses.len(),
        REQUIRED_BOOTSTRAP_REPLAY_WITNESSES.len()
    );
    assert_eq!(surface.links.len(), REQUIRED_BOOTSTRAP_REPLAY_LINKS.len());
    assert_eq!(surface.proofs.len(), REQUIRED_BOOTSTRAP_REPLAY_PROOFS.len());
    assert_eq!(
        surface.artifacts.len(),
        REQUIRED_BOOTSTRAP_REPLAY_ARTIFACTS.len()
    );
}

#[test]
fn p02_017_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_bootstrap_replay_surface(VALID);
    let golden = fs::read_to_string("goldens/p02/valid_bootstrap_replay.receipt")
        .expect("bootstrap replay golden must exist");
    assert_eq!(receipt.to_text(), golden);
}

#[test]
fn bootstrap_replay_control_surfaces_align_with_package_frontier() {
    assert!(FRONTIER_LOCK.contains("current_task=P02-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P02-X04"));
    assert!(
        FRONTIER_LOCK.contains("truth_bound=receipts/p02/pass_0087_bootstrap_retirement_supersession.receipt")
    );
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P02-X05"));
    assert!(TRUTH_SNAPSHOT.contains("latest_finished_frontier=P02-X05"));
    assert!(TRUTH_SNAPSHOT.contains("P02-017"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P02-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P03"));
}
