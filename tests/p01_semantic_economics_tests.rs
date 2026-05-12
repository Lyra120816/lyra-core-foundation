use lyra_phase0::p01::{
    deterministic_semantic_economics_suite_report, semantic_economics_artifacts_bind_paths,
    semantic_economics_frames_bind_outputs, semantic_economics_no_forbidden_descriptor_claims,
    semantic_economics_proofs_bind_registry,
    semantic_economics_receipts_cover_p01_001_through_p01_022,
    semantic_public_interest_outputs_bind_proofs, validate_semantic_economics_surface, ErrorCode,
    REQUIRED_SEMANTIC_ECONOMICS_FRAMES, REQUIRED_SEMANTIC_ECONOMICS_PROOFS,
    REQUIRED_SEMANTIC_PUBLIC_INTEREST_OUTPUTS,
};

const VALID: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/valid_semantic_economics.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_missing_rule.lyra");
const INVALID_MISSING_FRAME: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_missing_frame.lyra");
const INVALID_DUPLICATE_FRAME: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_duplicate_frame.lyra");
const INVALID_UNKNOWN_OUTPUT: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_unknown_output_reference.lyra");
const INVALID_MISSING_OUTPUT: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_missing_output.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_missing_proof.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_NETWORK_REQUIRED: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_network_required.lyra");
const INVALID_CAPTURE_ALLOWED: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_capture_allowed.lyra");
const INVALID_EXTRACTIVE_DEFAULT: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_extractive_default.lyra");
const INVALID_ECONOMICS_DRIFT: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_economics_drift.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_phase_closure_claim.lyra");
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_missing_receipt_binding.lyra");
const INVALID_UNKNOWN_COMMAND: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_unknown_command.lyra");
const INVALID_DOCS_ONLY: &str =
    include_str!("../fixtures/p01/semantic_economics_inputs/invalid_docs_only_claim.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_semantic_economics_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_semantic_economics_surface() {
    let (verdict, receipt) = validate_semantic_economics_surface(VALID);
    assert!(
        verdict.accepted,
        "valid semantic economics rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn semantic_economics_deterministic_report_is_stable_and_counted() {
    let frames = vec![
        (
            "z_frame".to_string(),
            "platform_value".to_string(),
            "docs/p01/z.lyra".to_string(),
            vec!["core_ir".to_string()],
            vec!["z_output".to_string()],
            vec!["receipts/p01/z.receipt".to_string()],
            "artifact_emitted".to_string(),
        ),
        (
            "a_frame".to_string(),
            "public_access".to_string(),
            "docs/p01/a.lyra".to_string(),
            vec!["semantic_atoms".to_string()],
            vec!["z_output".to_string()],
            vec!["receipts/p01/a.receipt".to_string()],
            "artifact_emitted".to_string(),
        ),
    ];
    let outputs = vec![(
        "z_output".to_string(),
        "casebook".to_string(),
        "examples/p01/z.lyra".to_string(),
        vec!["public".to_string()],
        vec!["lyra-p01-semantic-economics-check".to_string()],
        vec!["z_proof".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["capture_allowed".to_string()],
        "artifact_emitted".to_string(),
    )];
    let proofs = vec![(
        "z_proof".to_string(),
        "economics".to_string(),
        vec!["z_frame".to_string()],
        vec!["z_output".to_string()],
        vec!["receipts/p01/z.receipt".to_string()],
        vec!["lyra-p01-semantic-economics-check".to_string()],
        vec!["phase_closure".to_string(), "capture".to_string()],
        "artifact_emitted".to_string(),
    )];
    let report = deterministic_semantic_economics_suite_report(&frames, &outputs, &proofs);
    assert_eq!(report.frame_count, 2);
    assert_eq!(report.output_count, 1);
    assert_eq!(report.proof_count, 1);
    assert_eq!(report.frame_reports[0].id, "a_frame");
    assert!(report.suite_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_required_semantic_economics_gaps() {
    for (input, expected) in [
        (INVALID_MISSING_RULE, ErrorCode::MissingEconomicsRule),
        (INVALID_MISSING_FRAME, ErrorCode::MissingEconomicsFrame),
        (INVALID_MISSING_OUTPUT, ErrorCode::MissingEconomicsOutput),
        (INVALID_MISSING_PROOF, ErrorCode::MissingEconomicsProof),
    ] {
        assert_rejects_with(input, expected);
    }
}

#[test]
fn rejects_duplicate_unbound_and_invalid_economics_bindings() {
    assert_rejects_with(INVALID_DUPLICATE_FRAME, ErrorCode::DuplicateEconomicsFrame);
    assert_rejects_with(INVALID_UNKNOWN_OUTPUT, ErrorCode::InvalidEconomicsFrame);
    assert_rejects_with(INVALID_UNBOUND_PROOF, ErrorCode::EconomicsProofUnbound);
    assert_rejects_with(INVALID_MISSING_RECEIPT, ErrorCode::InvalidEconomicsFrame);
    assert_rejects_with(INVALID_UNKNOWN_COMMAND, ErrorCode::InvalidEconomicsOutput);
}

#[test]
fn rejects_network_capture_extraction_drift_docs_only_and_closure_claims() {
    assert_rejects_with(
        INVALID_NETWORK_REQUIRED,
        ErrorCode::EconomicsNetworkDependency,
    );
    assert_rejects_with(INVALID_CAPTURE_ALLOWED, ErrorCode::EconomicsCaptureAllowed);
    assert_rejects_with(
        INVALID_EXTRACTIVE_DEFAULT,
        ErrorCode::EconomicsExtractiveDefault,
    );
    assert_rejects_with(INVALID_ECONOMICS_DRIFT, ErrorCode::EconomicsDriftAccepted);
    assert_rejects_with(INVALID_DOCS_ONLY, ErrorCode::DocsOnlyImplementation);
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}

#[test]
fn semantic_economics_descriptor_registry_is_bound() {
    assert!(semantic_economics_frames_bind_outputs());
    assert!(semantic_public_interest_outputs_bind_proofs());
    assert!(semantic_economics_proofs_bind_registry());
    assert!(semantic_economics_artifacts_bind_paths());
    assert!(semantic_economics_no_forbidden_descriptor_claims());
    assert!(semantic_economics_receipts_cover_p01_001_through_p01_022());
}

#[test]
fn required_semantic_economics_inventory_counts_are_bound() {
    assert_eq!(REQUIRED_SEMANTIC_ECONOMICS_FRAMES.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_PUBLIC_INTEREST_OUTPUTS.len(), 6);
    assert_eq!(REQUIRED_SEMANTIC_ECONOMICS_PROOFS.len(), 6);
}
