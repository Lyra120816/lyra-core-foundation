use lyra_phase0::p01::{
    deterministic_semantic_ecosystem_suite_report, semantic_ecosystem_artifacts_bind_paths,
    semantic_ecosystem_docs_bind_examples, semantic_ecosystem_examples_bind_proofs,
    semantic_ecosystem_no_forbidden_descriptor_claims, semantic_ecosystem_proofs_bind_registry,
    semantic_ecosystem_receipts_cover_p01_001_through_p01_021, validate_semantic_ecosystem_surface,
    ErrorCode, REQUIRED_SEMANTIC_ECOSYSTEM_DOCS, REQUIRED_SEMANTIC_ECOSYSTEM_EXAMPLES,
    REQUIRED_SEMANTIC_ECOSYSTEM_PROOFS,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/valid_semantic_ecosystem.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_missing_rule.lyra");
const INVALID_MISSING_DOC: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_missing_doc.lyra");
const INVALID_DUPLICATE_DOC: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_duplicate_doc.lyra");
const INVALID_UNKNOWN_EXAMPLE_REFERENCE: &str = include_str!(
    "../fixtures/p01/semantic_ecosystem_inputs/invalid_unknown_example_reference.lyra"
);
const INVALID_MISSING_EXAMPLE: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_missing_example.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_missing_proof.lyra");
const INVALID_UNBOUND_PROOF_REFERENCE: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_NETWORK_REQUIRED: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_network_required.lyra");
const INVALID_ECOSYSTEM_DRIFT: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_ecosystem_drift.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_phase_closure_claim.lyra");
const INVALID_DOCUMENTATION_ALONE: &str = include_str!(
    "../fixtures/p01/semantic_ecosystem_inputs/invalid_documentation_alone_claim.lyra"
);
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_missing_receipt_binding.lyra");
const INVALID_UNKNOWN_COMMAND: &str =
    include_str!("../fixtures/p01/semantic_ecosystem_inputs/invalid_unknown_command.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_semantic_ecosystem_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_semantic_ecosystem_surface() {
    let (verdict, receipt) = validate_semantic_ecosystem_surface(VALID);
    assert!(
        verdict.accepted,
        "valid semantic ecosystem rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn semantic_ecosystem_deterministic_report_is_stable_and_counted() {
    let docs = vec![
        (
            "z_doc".to_string(),
            "public".to_string(),
            "docs/p01/z.lyra".to_string(),
            vec!["core_ir".to_string()],
            vec!["z_example".to_string()],
            vec!["receipts/p01/z.receipt".to_string()],
            "artifact_emitted".to_string(),
        ),
        (
            "a_doc".to_string(),
            "operator".to_string(),
            "docs/p01/a.lyra".to_string(),
            vec!["semantic_atoms".to_string()],
            vec!["z_example".to_string()],
            vec!["receipts/p01/a.receipt".to_string()],
            "artifact_emitted".to_string(),
        ),
    ];
    let examples = vec![(
        "z_example".to_string(),
        "review".to_string(),
        "examples/p01/z.lyra".to_string(),
        vec!["lyra-p01-semantic-ecosystem-check".to_string()],
        vec!["z_proof".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["drift".to_string()],
        "artifact_emitted".to_string(),
    )];
    let proofs = vec![(
        "z_proof".to_string(),
        "docs".to_string(),
        vec!["z_doc".to_string()],
        vec!["z_example".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["lyra-p01-semantic-ecosystem-check".to_string()],
        vec!["phase_closure".to_string()],
        "artifact_emitted".to_string(),
    )];
    let report = deterministic_semantic_ecosystem_suite_report(&docs, &examples, &proofs);
    assert_eq!(report.doc_count, 2);
    assert_eq!(report.example_count, 1);
    assert_eq!(report.proof_count, 1);
    assert_eq!(report.doc_reports[0].id, "a_doc");
    assert!(report.suite_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_ecosystem_gaps() {
    for (input, expected) in [
        (INVALID_MISSING_RULE, ErrorCode::MissingEcosystemRule),
        (INVALID_MISSING_DOC, ErrorCode::MissingEcosystemDoc),
        (INVALID_MISSING_EXAMPLE, ErrorCode::MissingEcosystemExample),
        (INVALID_MISSING_PROOF, ErrorCode::MissingEcosystemProof),
    ] {
        assert_rejects_with(input, expected);
    }
}

#[test]
fn rejects_duplicate_unbound_and_invalid_ecosystem_bindings() {
    assert_rejects_with(INVALID_DUPLICATE_DOC, ErrorCode::DuplicateEcosystemDoc);
    assert_rejects_with(
        INVALID_UNKNOWN_EXAMPLE_REFERENCE,
        ErrorCode::InvalidEcosystemDoc,
    );
    assert_rejects_with(
        INVALID_UNBOUND_PROOF_REFERENCE,
        ErrorCode::EcosystemProofUnbound,
    );
    assert_rejects_with(INVALID_MISSING_RECEIPT, ErrorCode::InvalidEcosystemDoc);
    assert_rejects_with(INVALID_UNKNOWN_COMMAND, ErrorCode::InvalidEcosystemExample);
}

#[test]
fn rejects_network_drift_docs_alone_and_closure_claims() {
    assert_rejects_with(
        INVALID_NETWORK_REQUIRED,
        ErrorCode::EcosystemNetworkDependency,
    );
    assert_rejects_with(INVALID_ECOSYSTEM_DRIFT, ErrorCode::EcosystemDriftAccepted);
    assert_rejects_with(INVALID_DOCUMENTATION_ALONE, ErrorCode::EcosystemDocsOnly);
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn semantic_ecosystem_descriptor_registry_is_bound() {
    assert!(semantic_ecosystem_docs_bind_examples());
    assert!(semantic_ecosystem_examples_bind_proofs());
    assert!(semantic_ecosystem_proofs_bind_registry());
    assert!(semantic_ecosystem_artifacts_bind_paths());
    assert!(semantic_ecosystem_no_forbidden_descriptor_claims());
    assert!(semantic_ecosystem_receipts_cover_p01_001_through_p01_021());
}

#[test]
fn required_semantic_ecosystem_inventory_counts_are_bound() {
    assert_eq!(REQUIRED_SEMANTIC_ECOSYSTEM_DOCS.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_ECOSYSTEM_EXAMPLES.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_ECOSYSTEM_PROOFS.len(), 6);
}
