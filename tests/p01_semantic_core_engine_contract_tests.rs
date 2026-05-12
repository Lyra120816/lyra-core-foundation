use lyra_phase0::p01::{
    P01_SEMANTIC_CORE_ENGINE_CONTRACT, REQUIRED_SEMANTIC_CORE_ENGINE_ARTIFACTS,
    REQUIRED_SEMANTIC_CORE_ENGINE_PROOFS, REQUIRED_SEMANTIC_CORE_ENGINE_RULES,
    REQUIRED_SEMANTIC_CORE_ENGINE_TRANSITIONS, REQUIRED_SEMANTIC_CORE_ENGINE_UNITS,
};

#[test]
fn p01_015_contract_binds_surface_fixture_golden_receipt_and_operator() {
    let contract = std::fs::read_to_string("interfaces/p01/contracts/semantic_core_engine.v1.lyra")
        .expect("P01-015 semantic core engine contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-CORE-ENGINE-CONTRACT v1"));
    assert!(contract.contains("task=P01-015"));
    assert!(contract.contains(P01_SEMANTIC_CORE_ENGINE_CONTRACT));
    assert!(contract.contains(
        "fixture=fixtures/p01/semantic_core_engine_inputs/valid_semantic_core_engine.lyra"
    ));
    assert!(contract.contains("golden=goldens/p01/valid_semantic_core_engine.receipt"));
    assert!(contract.contains("receipt=receipts/p01/pass_0044_semantic_core_engine.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-core-engine-check.rs"));
}

#[test]
fn p01_015_required_sets_are_broad_enough_for_engine_slice() {
    assert!(REQUIRED_SEMANTIC_CORE_ENGINE_RULES.len() >= 14);
    assert!(REQUIRED_SEMANTIC_CORE_ENGINE_UNITS.len() >= 9);
    assert!(REQUIRED_SEMANTIC_CORE_ENGINE_TRANSITIONS.len() >= 8);
    assert!(REQUIRED_SEMANTIC_CORE_ENGINE_ARTIFACTS.len() >= 8);
    assert!(REQUIRED_SEMANTIC_CORE_ENGINE_PROOFS.len() >= 6);
}
