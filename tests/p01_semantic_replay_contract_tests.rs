use std::fs;
use std::path::Path;

use lyra_phase0::p01::{
    parse_semantic_replay_surface, validate_semantic_replay_surface, P01_SEMANTIC_REPLAY_CONTRACT,
    REQUIRED_SEMANTIC_REPLAY_ARTIFACTS, REQUIRED_SEMANTIC_REPLAY_LINKS,
    REQUIRED_SEMANTIC_REPLAY_PROOFS, REQUIRED_SEMANTIC_REPLAY_RECEIPTS,
    REQUIRED_SEMANTIC_REPLAY_RULES, REQUIRED_SEMANTIC_REPLAY_WITNESSES,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_replay_inputs/valid_semantic_replay.lyra");

#[test]
fn p01_017_contract_binds_operator_fixture_and_receipt() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_replay.v1.lyra")
        .expect("P01-017 semantic replay contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-REPLAY-CONTRACT v1"));
    assert!(contract.contains("task=P01-017"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-REPLAY-WITNESS v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0046_semantic_replay.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-replay-check.rs"));
    assert!(Path::new("ops/p01/control/semantic_replay_law.v1.lyra").exists());
}

#[test]
fn p01_017_valid_surface_matches_required_inventory() {
    let surface = parse_semantic_replay_surface(VALID).expect("valid surface parses");
    assert_eq!(surface.header, P01_SEMANTIC_REPLAY_CONTRACT);
    assert_eq!(surface.rules.len(), REQUIRED_SEMANTIC_REPLAY_RULES.len());
    assert_eq!(
        surface.receipts.len(),
        REQUIRED_SEMANTIC_REPLAY_RECEIPTS.len()
    );
    assert_eq!(
        surface.witnesses.len(),
        REQUIRED_SEMANTIC_REPLAY_WITNESSES.len()
    );
    assert_eq!(surface.links.len(), REQUIRED_SEMANTIC_REPLAY_LINKS.len());
    assert_eq!(surface.proofs.len(), REQUIRED_SEMANTIC_REPLAY_PROOFS.len());
    assert_eq!(
        surface.artifacts.len(),
        REQUIRED_SEMANTIC_REPLAY_ARTIFACTS.len()
    );
}

#[test]
fn p01_017_golden_receipt_matches_validator_output() {
    let (_verdict, receipt) = validate_semantic_replay_surface(VALID);
    let golden = fs::read_to_string("goldens/p01/valid_semantic_replay.receipt")
        .expect("semantic replay golden must exist");
    assert_eq!(receipt.to_text(), golden);
}
