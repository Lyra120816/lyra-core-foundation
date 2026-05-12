use lyra_phase0::p00::{parse_identity_law_surface, validate_identity_law_surface};

const VALID: &str = include_str!("../fixtures/p00/identity_law_inputs/valid_identity_law.lyra");
const INVALID_PHASE_ID: &str =
    include_str!("../fixtures/p00/identity_law_inputs/invalid_phase_id.lyra");
const INVALID_TASK_ID: &str =
    include_str!("../fixtures/p00/identity_law_inputs/invalid_task_id.lyra");
const INVALID_PREFIX_MISMATCH: &str =
    include_str!("../fixtures/p00/identity_law_inputs/invalid_prefix_mismatch.lyra");
const INVALID_UNKNOWN_SUPERSESSION: &str =
    include_str!("../fixtures/p00/identity_law_inputs/invalid_unknown_supersession.lyra");
const INVALID_DUPLICATE_IDENTITY: &str =
    include_str!("../fixtures/p00/identity_law_inputs/invalid_duplicate_identity.lyra");
const INVALID_OWNER_ROOT: &str =
    include_str!("../fixtures/p00/identity_law_inputs/invalid_owner_root.lyra");
const INVALID_PLACEHOLDER_IDENTITY: &str =
    include_str!("../fixtures/p00/identity_law_inputs/invalid_placeholder_identity.lyra");
const GOLDEN_VALID_RECEIPT: &str = include_str!("../goldens/p00/valid_identity_law.receipt");

#[test]
fn valid_identity_law_is_accepted_and_receipted() {
    let (verdict, receipt) = validate_identity_law_surface(VALID);
    assert!(verdict.accepted, "{}", verdict.canonical_text());
    assert_eq!(receipt.to_text(), GOLDEN_VALID_RECEIPT);
}

#[test]
fn valid_identity_law_parses_phase_task_and_rules() {
    let parsed = parse_identity_law_surface(VALID).expect("valid identity law must parse");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-003");
    assert_eq!(parsed.status, "working_slice");
    assert_eq!(
        parsed
            .phase_by_id("P00")
            .expect("phase P00")
            .owner_roots
            .as_slice(),
        ["interfaces", "k0", "ops"]
    );
    assert_eq!(
        parsed.task_by_id("P00-003").expect("task P00-003").phase,
        "P00"
    );
    assert_eq!(parsed.rule_value("identity_uniqueness"), Some("required"));
}

#[test]
fn invalid_phase_identity_is_rejected() {
    let (verdict, _receipt) = validate_identity_law_surface(INVALID_PHASE_ID);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("invalid_phase_identity"));
}

#[test]
fn invalid_task_identity_is_rejected() {
    let (verdict, _receipt) = validate_identity_law_surface(INVALID_TASK_ID);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("invalid_task_identity"));
}

#[test]
fn phase_task_prefix_mismatch_is_rejected() {
    let (verdict, _receipt) = validate_identity_law_surface(INVALID_PREFIX_MISMATCH);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("identity_prefix_mismatch"));
}

#[test]
fn unknown_supersession_target_is_rejected() {
    let (verdict, _receipt) = validate_identity_law_surface(INVALID_UNKNOWN_SUPERSESSION);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("unknown_supersession_target"));
}

#[test]
fn duplicate_identity_is_rejected() {
    let (verdict, _receipt) = validate_identity_law_surface(INVALID_DUPLICATE_IDENTITY);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("duplicate_identity"));
}

#[test]
fn invalid_owner_root_is_rejected() {
    let (verdict, _receipt) = validate_identity_law_surface(INVALID_OWNER_ROOT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("invalid_owner_root"));
}

#[test]
fn placeholder_identity_language_is_rejected() {
    let (verdict, _receipt) = validate_identity_law_surface(INVALID_PLACEHOLDER_IDENTITY);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("forbidden_token"));
}

#[test]
fn identity_receipts_are_reproducible_for_identical_input() {
    let (_left_verdict, left_receipt) = validate_identity_law_surface(VALID);
    let (_right_verdict, right_receipt) = validate_identity_law_surface(VALID);
    assert_eq!(left_receipt, right_receipt);
}
