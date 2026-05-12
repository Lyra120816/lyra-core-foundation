use std::fs;
use std::path::Path;

const RECEIPT_TO_CONTRACT: &[(&str, &str)] = &[
    (
        "receipts/p01/pass_0030_semantic_atoms.receipt",
        "interfaces/p01/contracts/semantic_atoms.v1.lyra",
    ),
    (
        "receipts/p01/pass_0031_core_ir.receipt",
        "interfaces/p01/contracts/core_ir.v1.lyra",
    ),
    (
        "receipts/p01/pass_0032_semantic_objects.receipt",
        "interfaces/p01/contracts/semantic_objects.v1.lyra",
    ),
    (
        "receipts/p01/pass_0033_semantic_identity.receipt",
        "interfaces/p01/contracts/semantic_identity.v1.lyra",
    ),
    (
        "receipts/p01/pass_0034_reference_semantics.receipt",
        "interfaces/p01/contracts/reference_semantics.v1.lyra",
    ),
    (
        "receipts/p01/pass_0035_symbolic_equality.receipt",
        "interfaces/p01/contracts/symbolic_equality.v1.lyra",
    ),
    (
        "receipts/p01/pass_0036_error_challenge_evidence.receipt",
        "interfaces/p01/contracts/error_challenge_evidence.v1.lyra",
    ),
    (
        "receipts/p01/pass_0037_semantic_serialization_hashing.receipt",
        "interfaces/p01/contracts/semantic_serialization_hashing.v1.lyra",
    ),
    (
        "receipts/p01/pass_0038_semantic_adversarial_corpus.receipt",
        "interfaces/p01/contracts/semantic_adversarial_corpus.v1.lyra",
    ),
    (
        "receipts/p01/pass_0039_core_ir_reuse.receipt",
        "interfaces/p01/contracts/core_ir_reuse.v1.lyra",
    ),
    (
        "receipts/p01/pass_0040_semantic_atom_reference.receipt",
        "interfaces/p01/contracts/semantic_atom_reference.v1.lyra",
    ),
    (
        "receipts/p01/pass_0041_semantic_bedrock_receipts.receipt",
        "interfaces/p01/contracts/semantic_bedrock_receipts.v1.lyra",
    ),
    (
        "receipts/p01/pass_0042_formal_semantic_constitution.receipt",
        "interfaces/p01/contracts/formal_semantic_constitution.v1.lyra",
    ),
    (
        "receipts/p01/pass_0043_canonical_data_model.receipt",
        "interfaces/p01/contracts/canonical_data_model.v1.lyra",
    ),
    (
        "receipts/p01/pass_0044_semantic_core_engine.receipt",
        "interfaces/p01/contracts/semantic_core_engine.v1.lyra",
    ),
    (
        "receipts/p01/pass_0045_semantic_falsification.receipt",
        "interfaces/p01/contracts/semantic_falsification.v1.lyra",
    ),
    (
        "receipts/p01/pass_0046_semantic_replay.receipt",
        "interfaces/p01/contracts/semantic_replay.v1.lyra",
    ),
    (
        "receipts/p01/pass_0047_semantic_interface.receipt",
        "interfaces/p01/contracts/semantic_interface.v1.lyra",
    ),
    (
        "receipts/p01/pass_0048_semantic_packaging.receipt",
        "interfaces/p01/contracts/semantic_packaging.v1.lyra",
    ),
    (
        "receipts/p01/pass_0049_semantic_deployment.receipt",
        "interfaces/p01/contracts/semantic_deployment.v1.lyra",
    ),
    (
        "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        "interfaces/p01/contracts/semantic_ecosystem.v1.lyra",
    ),
    (
        "receipts/p01/pass_0051_semantic_economics.receipt",
        "interfaces/p01/contracts/semantic_economics.v1.lyra",
    ),
    (
        "receipts/p01/pass_0052_semantic_redteam.receipt",
        "interfaces/p01/contracts/semantic_redteam.v1.lyra",
    ),
    (
        "receipts/p01/pass_0053_semantic_closure.receipt",
        "interfaces/p01/contracts/semantic_closure.v1.lyra",
    ),
    (
        "receipts/p01/pass_0054_semantic_dependency_matrix.receipt",
        "interfaces/p01/contracts/semantic_dependency_matrix.v1.lyra",
    ),
    (
        "receipts/p01/pass_0055_semantic_proof_family.receipt",
        "interfaces/p01/contracts/semantic_proof_family.v1.lyra",
    ),
];

fn sorted_files(root: &str, extension: &str) -> Vec<String> {
    let mut files = fs::read_dir(root)
        .unwrap_or_else(|error| panic!("unable to read {root}: {error}"))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some(extension))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect::<Vec<_>>();
    files.sort();
    files
}

#[test]
fn every_p01_receipt_has_matching_interface_contract() {
    let receipts = sorted_files("receipts/p01", "receipt");
    let contracts = sorted_files("interfaces/p01/contracts", "lyra");
    let laws = sorted_files("ops/p01/control", "lyra")
        .into_iter()
        .filter(|path| path.ends_with("_law.v1.lyra"))
        .collect::<Vec<_>>();
    assert_eq!(
        receipts.len(),
        contracts.len(),
        "P01 receipt/contract count drift"
    );
    assert_eq!(
        receipts.len(),
        laws.len(),
        "P01 receipt/control-law count drift"
    );

    for (receipt, contract) in RECEIPT_TO_CONTRACT {
        assert!(Path::new(receipt).exists(), "missing receipt {receipt}");
        assert!(Path::new(contract).exists(), "missing contract {contract}");
        let contract_text = fs::read_to_string(contract)
            .unwrap_or_else(|error| panic!("unable to read {contract}: {error}"));
        assert!(
            contract_text.contains(receipt),
            "contract {contract} does not bind receipt {receipt}"
        );
    }
}

#[test]
fn p01_008_semantic_serialization_hashing_contract_is_bound() {
    let contract =
        fs::read_to_string("interfaces/p01/contracts/semantic_serialization_hashing.v1.lyra")
            .expect("P01-008 semantic serialization hashing contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-SERIALIZATION-HASHING-CONTRACT v1"));
    assert!(contract.contains("task=P01-008"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-SERIALIZATION-HASHING v1"));
    assert!(
        contract.contains("receipt=receipts/p01/pass_0037_semantic_serialization_hashing.receipt")
    );
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-serialization-hashing-check.rs"));
}

#[test]
fn p01_010_core_ir_reuse_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/core_ir_reuse.v1.lyra")
        .expect("P01-010 core IR reuse contract must exist");

    assert!(contract.starts_with("LYRA-P01-CORE-IR-REUSE-CONTRACT v1"));
    assert!(contract.contains("task=P01-010"));
    assert!(contract.contains("surface=LYRA-P01-CORE-IR-REUSE v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0039_core_ir_reuse.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-core-ir-reuse-check.rs"));
}

#[test]
fn p01_011_semantic_atom_reference_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_atom_reference.v1.lyra")
        .expect("P01-011 semantic atom reference contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-ATOM-REFERENCE-CONTRACT v1"));
    assert!(contract.contains("task=P01-011"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-ATOM-REFERENCE v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0040_semantic_atom_reference.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-atom-reference-check.rs"));
}

#[test]
fn p01_012_semantic_bedrock_receipts_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_bedrock_receipts.v1.lyra")
        .expect("P01-012 semantic bedrock receipts contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-BEDROCK-RECEIPTS-CONTRACT v1"));
    assert!(contract.contains("task=P01-012"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-BEDROCK-RECEIPTS v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0041_semantic_bedrock_receipts.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-bedrock-receipts-check.rs"));
}
#[test]
fn p01_013_formal_semantic_constitution_contract_is_bound() {
    let contract =
        fs::read_to_string("interfaces/p01/contracts/formal_semantic_constitution.v1.lyra")
            .expect("P01-013 formal semantic constitution contract must exist");

    assert!(contract.starts_with("LYRA-P01-FORMAL-SEMANTIC-CONSTITUTION-CONTRACT v1"));
    assert!(contract.contains("task=P01-013"));
    assert!(contract.contains("surface=LYRA-P01-FORMAL-SEMANTIC-CONSTITUTION v1"));
    assert!(
        contract.contains("receipt=receipts/p01/pass_0042_formal_semantic_constitution.receipt")
    );
    assert!(contract.contains("operator=src/bin/lyra-p01-formal-semantic-constitution-check.rs"));
}

#[test]
fn p01_014_canonical_data_model_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/canonical_data_model.v1.lyra")
        .expect("P01-014 canonical data model contract must exist");

    assert!(contract.starts_with("LYRA-P01-CANONICAL-DATA-MODEL-CONTRACT v1"));
    assert!(contract.contains("task=P01-014"));
    assert!(contract.contains("surface=LYRA-P01-CANONICAL-DATA-MODEL v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0043_canonical_data_model.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-canonical-data-model-check.rs"));
}

#[test]
fn p01_015_semantic_core_engine_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_core_engine.v1.lyra")
        .expect("P01-015 semantic core engine contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-CORE-ENGINE-CONTRACT v1"));
    assert!(contract.contains("task=P01-015"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-CORE-ENGINE v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0044_semantic_core_engine.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-core-engine-check.rs"));
}

#[test]
fn p01_016_semantic_falsification_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_falsification.v1.lyra")
        .expect("P01-016 semantic falsification contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-FALSIFICATION-CONTRACT v1"));
    assert!(contract.contains("task=P01-016"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-FALSIFICATION-CORPUS v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0045_semantic_falsification.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-falsification-check.rs"));
}

#[test]
fn p01_017_semantic_replay_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_replay.v1.lyra")
        .expect("P01-017 semantic replay contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-REPLAY-CONTRACT v1"));
    assert!(contract.contains("task=P01-017"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-REPLAY-WITNESS v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0046_semantic_replay.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-replay-check.rs"));
}

#[test]
fn p01_018_semantic_interface_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_interface.v1.lyra")
        .expect("P01-018 semantic interface contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-INTERFACE-CONTRACT v1"));
    assert!(contract.contains("task=P01-018"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-INTERFACE v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0047_semantic_interface.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-interface-check.rs"));
}

#[test]
fn p01_019_semantic_packaging_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_packaging.v1.lyra")
        .expect("P01-019 semantic packaging contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-PACKAGING-CONTRACT v1"));
    assert!(contract.contains("task=P01-019"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-PACKAGING v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0048_semantic_packaging.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-packaging-check.rs"));
}

#[test]
fn p01_020_semantic_deployment_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_deployment.v1.lyra")
        .expect("P01-020 semantic deployment contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-DEPLOYMENT-CONTRACT v1"));
    assert!(contract.contains("task=P01-020"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-DEPLOYMENT-HOOKS v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0049_semantic_deployment.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-deployment-check.rs"));
}

#[test]
fn p01_021_semantic_ecosystem_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_ecosystem.v1.lyra")
        .expect("P01-021 semantic ecosystem contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-ECOSYSTEM-CONTRACT v1"));
    assert!(contract.contains("task=P01-021"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-ECOSYSTEM-DOCS-EXAMPLES v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0050_semantic_ecosystem.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-ecosystem-check.rs"));
}

#[test]
fn p01_022_semantic_economics_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_economics.v1.lyra")
        .expect("P01-022 semantic economics contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-ECONOMICS-CONTRACT v1"));
    assert!(contract.contains("task=P01-022"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-ECONOMICS-PUBLIC-INTEREST v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0051_semantic_economics.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-economics-check.rs"));
}

#[test]
fn p01_023_semantic_redteam_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_redteam.v1.lyra")
        .expect("P01-023 semantic redteam contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-REDTEAM-CONTRACT v1"));
    assert!(contract.contains("task=P01-023"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-REDTEAM-ROLLBACK v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0052_semantic_redteam.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-redteam-check.rs"));
}

#[test]
fn p01_024_semantic_closure_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_closure.v1.lyra")
        .expect("P01-024 semantic closure contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-CLOSURE-CONTRACT v1"));
    assert!(contract.contains("task=P01-024"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-CLOSURE-GATE v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0053_semantic_closure.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-closure-check.rs"));
}

#[test]
fn p01_x01_semantic_dependency_matrix_contract_is_bound() {
    let contract =
        fs::read_to_string("interfaces/p01/contracts/semantic_dependency_matrix.v1.lyra")
            .expect("P01-X01 semantic dependency matrix contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-DEPENDENCY-MATRIX v1"));
    assert!(contract.contains("task=P01-X01"));
    assert!(contract.contains("dependency:P01-024"));
    assert!(contract.contains("blocker:P01-GLOBAL"));
    assert!(contract.contains("lane:closure_output_chain"));
}

#[test]
fn p01_x02_semantic_proof_family_contract_is_bound() {
    let contract = fs::read_to_string("interfaces/p01/contracts/semantic_proof_family.v1.lyra")
        .expect("P01-X02 semantic proof family contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-PROOF-FAMILY-CONTRACT v1"));
    assert!(contract.contains("task=P01-X02"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-PROOF-FAMILY-TABLE v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0055_semantic_proof_family.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-proof-family-check.rs"));
}
