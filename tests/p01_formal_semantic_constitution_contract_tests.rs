use lyra_phase0::p01::{
    P01_FORMAL_SEMANTIC_CONSTITUTION_CONTRACT, REQUIRED_FORMAL_SEMANTIC_CONSTITUTION_RULES,
    REQUIRED_FORMAL_SEMANTIC_DOMAINS, REQUIRED_FORMAL_SEMANTIC_INVARIANTS,
    REQUIRED_FORMAL_SEMANTIC_LAWS, REQUIRED_FORMAL_SEMANTIC_PROOFS,
};

#[test]
fn p01_013_contract_binds_surface_fixture_golden_receipt_and_operator() {
    let contract =
        std::fs::read_to_string("interfaces/p01/contracts/formal_semantic_constitution.v1.lyra")
            .expect("P01-013 formal semantic constitution contract must exist");

    assert!(contract.starts_with("LYRA-P01-FORMAL-SEMANTIC-CONSTITUTION-CONTRACT v1"));
    assert!(contract.contains("task=P01-013"));
    assert!(contract.contains(P01_FORMAL_SEMANTIC_CONSTITUTION_CONTRACT));
    assert!(contract.contains("fixture=fixtures/p01/formal_semantic_constitution_inputs/valid_formal_semantic_constitution.lyra"));
    assert!(contract.contains("golden=goldens/p01/valid_formal_semantic_constitution.receipt"));
    assert!(
        contract.contains("receipt=receipts/p01/pass_0042_formal_semantic_constitution.receipt")
    );
    assert!(contract.contains("operator=src/bin/lyra-p01-formal-semantic-constitution-check.rs"));
}

#[test]
fn p01_013_required_sets_are_broad_enough_for_constitution_slice() {
    assert!(REQUIRED_FORMAL_SEMANTIC_CONSTITUTION_RULES.len() >= 13);
    assert!(REQUIRED_FORMAL_SEMANTIC_DOMAINS.len() >= 8);
    assert!(REQUIRED_FORMAL_SEMANTIC_LAWS.len() >= 8);
    assert!(REQUIRED_FORMAL_SEMANTIC_INVARIANTS.len() >= 6);
    assert!(REQUIRED_FORMAL_SEMANTIC_PROOFS.len() >= 6);
}
