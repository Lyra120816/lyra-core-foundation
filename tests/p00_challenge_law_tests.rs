use lyra_phase0::p00::{parse_challenge_law_surface, validate_challenge_law_surface};

const VALID: &str = include_str!("../fixtures/p00/challenge_law_inputs/valid_challenge_law.lyra");
const INVALID_MISSING_RED_TEAM_REVIEW: &str =
    include_str!("../fixtures/p00/challenge_law_inputs/invalid_missing_red_team_review.lyra");
const INVALID_MISSING_CHALLENGE_RIGHT: &str =
    include_str!("../fixtures/p00/challenge_law_inputs/invalid_missing_challenge_right.lyra");
const INVALID_ROLLBACK_WITHOUT_RECEIPT: &str =
    include_str!("../fixtures/p00/challenge_law_inputs/invalid_rollback_without_receipt.lyra");
const INVALID_AMENDMENT_BYPASSES_AUTHORITY: &str =
    include_str!("../fixtures/p00/challenge_law_inputs/invalid_amendment_bypasses_authority.lyra");
const INVALID_ARCHIVE_PRIMARY_AMENDMENT: &str =
    include_str!("../fixtures/p00/challenge_law_inputs/invalid_archive_primary_amendment.lyra");
const INVALID_RETALIATION_ALLOWED: &str =
    include_str!("../fixtures/p00/challenge_law_inputs/invalid_retaliation_allowed.lyra");
const INVALID_UNBOUNDED_AMENDMENT: &str =
    include_str!("../fixtures/p00/challenge_law_inputs/invalid_unbounded_amendment.lyra");
const INVALID_DUPLICATE_REVIEW_GATE: &str =
    include_str!("../fixtures/p00/challenge_law_inputs/invalid_duplicate_review_gate.lyra");
const GOLDEN_VALID_RECEIPT: &str = include_str!("../goldens/p00/valid_challenge_law.receipt");

#[test]
fn valid_challenge_law_is_accepted_and_receipted() {
    let (verdict, receipt) = validate_challenge_law_surface(VALID);
    assert!(verdict.accepted, "{}", verdict.canonical_text());
    assert_eq!(receipt.to_text(), GOLDEN_VALID_RECEIPT);
}

#[test]
fn valid_challenge_law_parses_review_challenge_rollback_and_amendment() {
    let parsed = parse_challenge_law_surface(VALID).expect("valid challenge law must parse");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-006");
    assert_eq!(parsed.status, "working_slice");
    assert_eq!(
        parsed.rule_value("red_team_review_required"),
        Some("required")
    );
    assert_eq!(
        parsed
            .review_by_id("frontier_red_team")
            .expect("review")
            .required_before,
        "frontier_advance"
    );
    assert_eq!(
        parsed
            .challenge_by_id("operator_challenge")
            .expect("challenge")
            .protection,
        "non_retaliation"
    );
    assert_eq!(
        parsed
            .rollback_by_id("frontier_rollback")
            .expect("rollback")
            .target,
        "previous_frontier"
    );
    assert_eq!(
        parsed
            .amendment_by_id("constitutional_amendment")
            .expect("amendment")
            .scope,
        "constitution"
    );
}

#[test]
fn missing_red_team_review_is_rejected() {
    let (verdict, _receipt) = validate_challenge_law_surface(INVALID_MISSING_RED_TEAM_REVIEW);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_review_gate"));
}

#[test]
fn missing_challenge_right_is_rejected() {
    let (verdict, _receipt) = validate_challenge_law_surface(INVALID_MISSING_CHALLENGE_RIGHT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_challenge_right"));
}

#[test]
fn rollback_without_receipt_is_rejected() {
    let (verdict, _receipt) = validate_challenge_law_surface(INVALID_ROLLBACK_WITHOUT_RECEIPT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("rollback_without_receipt"));
}

#[test]
fn amendment_authority_bypass_is_rejected() {
    let (verdict, _receipt) = validate_challenge_law_surface(INVALID_AMENDMENT_BYPASSES_AUTHORITY);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("amendment_authority_bypass"));
}

#[test]
fn archive_primary_amendment_is_rejected() {
    let (verdict, _receipt) = validate_challenge_law_surface(INVALID_ARCHIVE_PRIMARY_AMENDMENT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("archive_primary_amendment"));
}

#[test]
fn retaliation_allowed_is_rejected() {
    let (verdict, _receipt) = validate_challenge_law_surface(INVALID_RETALIATION_ALLOWED);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("retaliation_allowed"));
}

#[test]
fn unbounded_amendment_is_rejected() {
    let (verdict, _receipt) = validate_challenge_law_surface(INVALID_UNBOUNDED_AMENDMENT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("unbounded_amendment"));
}

#[test]
fn duplicate_review_gate_is_rejected() {
    let (verdict, _receipt) = validate_challenge_law_surface(INVALID_DUPLICATE_REVIEW_GATE);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("duplicate_review_gate"));
}

#[test]
fn challenge_receipts_are_reproducible_for_identical_input() {
    let (_left_verdict, left_receipt) = validate_challenge_law_surface(VALID);
    let (_right_verdict, right_receipt) = validate_challenge_law_surface(VALID);
    assert_eq!(left_receipt, right_receipt);
}
