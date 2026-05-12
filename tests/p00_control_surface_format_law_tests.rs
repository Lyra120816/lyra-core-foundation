use lyra_phase0::p00::{parse_control_surface_format_law, validate_control_surface_format_law};

const VALID: &str =
    include_str!("../fixtures/p00/control_surface_inputs/valid_control_surfaces.lyra");
const INVALID_MISSING_FRONTIER_LOCK: &str =
    include_str!("../fixtures/p00/control_surface_inputs/invalid_missing_frontier_lock.lyra");
const INVALID_MISSING_TRUTH_SNAPSHOT_BINDING: &str = include_str!(
    "../fixtures/p00/control_surface_inputs/invalid_missing_truth_snapshot_binding.lyra"
);
const INVALID_MISSING_PASS_TEMPLATE: &str =
    include_str!("../fixtures/p00/control_surface_inputs/invalid_missing_pass_template.lyra");
const INVALID_MISSING_BLOCKER_BINDING: &str =
    include_str!("../fixtures/p00/control_surface_inputs/invalid_missing_blocker_binding.lyra");
const INVALID_UNSTABLE_FIELD: &str =
    include_str!("../fixtures/p00/control_surface_inputs/invalid_unstable_field.lyra");
const INVALID_MANUAL_ONLY_SURFACE: &str =
    include_str!("../fixtures/p00/control_surface_inputs/invalid_manual_only_surface.lyra");
const INVALID_DUPLICATE_CONTROL_SURFACE: &str =
    include_str!("../fixtures/p00/control_surface_inputs/invalid_duplicate_control_surface.lyra");
const INVALID_UNKNOWN_CLAIM_SURFACE: &str =
    include_str!("../fixtures/p00/control_surface_inputs/invalid_unknown_claim_surface.lyra");
const INVALID_GLOBAL_COMPLETE_CLAIM: &str =
    include_str!("../fixtures/p00/control_surface_inputs/invalid_global_complete_claim.lyra");
const GOLDEN_VALID_RECEIPT: &str = include_str!("../goldens/p00/valid_control_surfaces.receipt");

#[test]
fn valid_control_surface_format_law_is_accepted_and_receipted() {
    let (verdict, receipt) = validate_control_surface_format_law(VALID);
    assert!(verdict.accepted, "{}", verdict.canonical_text());
    assert_eq!(receipt.to_text(), GOLDEN_VALID_RECEIPT);
}

#[test]
fn valid_control_surface_format_law_parses_all_base_surfaces() {
    let parsed =
        parse_control_surface_format_law(VALID).expect("valid control surface law must parse");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-007");
    assert_eq!(parsed.status, "working_slice");
    assert_eq!(
        parsed.rule_value("frontier_lock_required"),
        Some("required")
    );
    assert_eq!(
        parsed
            .surface_by_id("frontier_lock")
            .expect("frontier lock")
            .schema,
        "ops/p00/control/frontier_lock.v1.lyra"
    );
    assert_eq!(
        parsed
            .field_by_id("truth_snapshot.closed")
            .expect("truth closed")
            .value,
        "false"
    );
    assert_eq!(
        parsed
            .template_by_id("pass_template")
            .expect("template")
            .path,
        "ops/p00/control/pass_template.v1.lyra"
    );
    assert_eq!(
        parsed
            .claim_by_id("control_surface_format_law")
            .expect("claim")
            .status,
        "working_slice"
    );
}

#[test]
fn missing_frontier_lock_is_rejected() {
    let (verdict, _receipt) = validate_control_surface_format_law(INVALID_MISSING_FRONTIER_LOCK);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_control_surface"));
}

#[test]
fn missing_truth_snapshot_receipt_binding_is_rejected() {
    let (verdict, _receipt) =
        validate_control_surface_format_law(INVALID_MISSING_TRUTH_SNAPSHOT_BINDING);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_control_field"));
}

#[test]
fn missing_pass_template_is_rejected() {
    let (verdict, _receipt) = validate_control_surface_format_law(INVALID_MISSING_PASS_TEMPLATE);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_control_surface"));
    assert!(text.contains("missing_pass_template"));
}

#[test]
fn missing_blocker_binding_is_rejected() {
    let (verdict, _receipt) = validate_control_surface_format_law(INVALID_MISSING_BLOCKER_BINDING);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_control_field"));
}

#[test]
fn unstable_field_is_rejected() {
    let (verdict, _receipt) = validate_control_surface_format_law(INVALID_UNSTABLE_FIELD);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("control_surface_drift"));
}

#[test]
fn manual_only_control_surface_is_rejected() {
    let (verdict, _receipt) = validate_control_surface_format_law(INVALID_MANUAL_ONLY_SURFACE);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("invalid_control_surface"));
}

#[test]
fn duplicate_control_surface_is_rejected() {
    let (verdict, _receipt) =
        validate_control_surface_format_law(INVALID_DUPLICATE_CONTROL_SURFACE);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("duplicate_control_surface"));
}

#[test]
fn unknown_claim_surface_is_rejected() {
    let (verdict, _receipt) = validate_control_surface_format_law(INVALID_UNKNOWN_CLAIM_SURFACE);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("unknown_evidence_path"));
}

#[test]
fn global_complete_claim_is_rejected() {
    let (verdict, _receipt) = validate_control_surface_format_law(INVALID_GLOBAL_COMPLETE_CLAIM);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(
        text.contains("unsupported_global_closure") || text.contains("unsupported_closure_status")
    );
}

#[test]
fn control_surface_receipts_are_reproducible_for_identical_input() {
    let (_left_verdict, left_receipt) = validate_control_surface_format_law(VALID);
    let (_right_verdict, right_receipt) = validate_control_surface_format_law(VALID);
    assert_eq!(left_receipt, right_receipt);
}
