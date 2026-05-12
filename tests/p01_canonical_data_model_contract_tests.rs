use lyra_phase0::p01::{
    P01_CANONICAL_DATA_MODEL_CONTRACT, REQUIRED_P01_CANONICAL_DATA_MODELS,
    REQUIRED_P01_CANONICAL_DATA_MODEL_RULES, REQUIRED_P01_CANONICAL_DATA_PROOFS,
    REQUIRED_P01_CANONICAL_FIELDS, REQUIRED_P01_CANONICAL_MODEL_BRIDGES,
    REQUIRED_P01_CANONICAL_SCHEMAS,
};

#[test]
fn p01_014_contract_binds_surface_fixture_golden_receipt_and_operator() {
    let contract = std::fs::read_to_string("interfaces/p01/contracts/canonical_data_model.v1.lyra")
        .expect("P01-014 canonical data model contract must exist");

    assert!(contract.starts_with("LYRA-P01-CANONICAL-DATA-MODEL-CONTRACT v1"));
    assert!(contract.contains("task=P01-014"));
    assert!(contract.contains(P01_CANONICAL_DATA_MODEL_CONTRACT));
    assert!(contract.contains(
        "fixture=fixtures/p01/canonical_data_model_inputs/valid_canonical_data_model.lyra"
    ));
    assert!(contract.contains("golden=goldens/p01/valid_canonical_data_model.receipt"));
    assert!(contract.contains("receipt=receipts/p01/pass_0043_canonical_data_model.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-canonical-data-model-check.rs"));
}

#[test]
fn p01_014_required_sets_are_broad_enough_for_model_slice() {
    assert!(REQUIRED_P01_CANONICAL_DATA_MODEL_RULES.len() >= 12);
    assert!(REQUIRED_P01_CANONICAL_DATA_MODELS.len() >= 8);
    assert!(REQUIRED_P01_CANONICAL_SCHEMAS.len() >= 8);
    assert!(REQUIRED_P01_CANONICAL_FIELDS.len() >= 18);
    assert!(REQUIRED_P01_CANONICAL_MODEL_BRIDGES.len() >= 6);
    assert!(REQUIRED_P01_CANONICAL_DATA_PROOFS.len() >= 6);
}
