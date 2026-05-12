use lyra_phase0::p00::{parse_delivery_surface, validate_delivery_surface};

const VALID: &str =
    include_str!("../fixtures/p00/delivery_protocol_inputs/valid_delivery_protocol.lyra");
const INVALID_MISSING_ARTIFACT: &str =
    include_str!("../fixtures/p00/delivery_protocol_inputs/invalid_missing_artifact.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p00/delivery_protocol_inputs/invalid_missing_proof.lyra");
const INVALID_CLOSURE_BEFORE_RECEIPT: &str =
    include_str!("../fixtures/p00/delivery_protocol_inputs/invalid_closure_before_receipt.lyra");
const INVALID_DOCS_ONLY_DELIVERY: &str =
    include_str!("../fixtures/p00/delivery_protocol_inputs/invalid_docs_only_delivery.lyra");
const INVALID_MISSING_COMMAND_RECORD: &str =
    include_str!("../fixtures/p00/delivery_protocol_inputs/invalid_missing_command_record.lyra");
const INVALID_UNKNOWN_EVIDENCE_PATH: &str =
    include_str!("../fixtures/p00/delivery_protocol_inputs/invalid_unknown_evidence_path.lyra");
const INVALID_DUPLICATE_ARTIFACT: &str =
    include_str!("../fixtures/p00/delivery_protocol_inputs/invalid_duplicate_artifact.lyra");
const GOLDEN_VALID_RECEIPT: &str = include_str!("../goldens/p00/valid_delivery_protocol.receipt");

#[test]
fn valid_delivery_protocol_is_accepted_and_receipted() {
    let (verdict, receipt) = validate_delivery_surface(VALID);
    assert!(verdict.accepted, "{}", verdict.canonical_text());
    assert_eq!(receipt.to_text(), GOLDEN_VALID_RECEIPT);
}

#[test]
fn valid_delivery_protocol_parses_artifacts_proofs_and_claims() {
    let parsed = parse_delivery_surface(VALID).expect("valid delivery protocol must parse");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-005");
    assert_eq!(parsed.status, "working_slice");
    assert_eq!(parsed.rule_value("artifact_only_pass"), Some("required"));
    assert_eq!(
        parsed
            .artifact_by_id("p00_delivery_runtime")
            .expect("artifact")
            .path,
        "ops/p00/src/delivery.rs"
    );
    assert_eq!(
        parsed
            .proof_by_id("execution_receipts")
            .expect("proof")
            .family,
        "p00-execution-receipts"
    );
    assert_eq!(
        parsed.claim_by_id("p00_005").expect("claim").status,
        "working_slice"
    );
}

#[test]
fn missing_required_artifact_is_rejected() {
    let (verdict, _receipt) = validate_delivery_surface(INVALID_MISSING_ARTIFACT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_delivery_artifact"));
}

#[test]
fn missing_required_proof_family_is_rejected() {
    let (verdict, _receipt) = validate_delivery_surface(INVALID_MISSING_PROOF);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_proof_binding"));
}

#[test]
fn closure_before_receipt_is_rejected() {
    let (verdict, _receipt) = validate_delivery_surface(INVALID_CLOSURE_BEFORE_RECEIPT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("closure_before_receipt") || text.contains("unsupported_global_closure"));
}

#[test]
fn docs_only_delivery_is_rejected() {
    let (verdict, _receipt) = validate_delivery_surface(INVALID_DOCS_ONLY_DELIVERY);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("underbuild_violation") || text.contains("docs_only_implementation"));
}

#[test]
fn missing_command_record_is_rejected() {
    let (verdict, _receipt) = validate_delivery_surface(INVALID_MISSING_COMMAND_RECORD);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_command_record"));
}

#[test]
fn unknown_evidence_path_is_rejected() {
    let (verdict, _receipt) = validate_delivery_surface(INVALID_UNKNOWN_EVIDENCE_PATH);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("unknown_evidence_path"));
}

#[test]
fn duplicate_artifact_is_rejected() {
    let (verdict, _receipt) = validate_delivery_surface(INVALID_DUPLICATE_ARTIFACT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("duplicate_delivery_artifact"));
}

#[test]
fn delivery_receipts_are_reproducible_for_identical_input() {
    let (_left_verdict, left_receipt) = validate_delivery_surface(VALID);
    let (_right_verdict, right_receipt) = validate_delivery_surface(VALID);
    assert_eq!(left_receipt, right_receipt);
}
