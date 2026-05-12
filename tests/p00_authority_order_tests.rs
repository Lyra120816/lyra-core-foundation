use lyra_phase0::p00::{parse_authority_order_surface, validate_authority_order_surface};

const VALID: &str =
    include_str!("../fixtures/p00/authority_order_inputs/valid_authority_order.lyra");
const INVALID_MISSING_MASTER: &str =
    include_str!("../fixtures/p00/authority_order_inputs/invalid_missing_master.lyra");
const INVALID_DUPLICATE_RANK: &str =
    include_str!("../fixtures/p00/authority_order_inputs/invalid_duplicate_rank.lyra");
const INVALID_LOWER_SUPERSEDES_MASTER: &str =
    include_str!("../fixtures/p00/authority_order_inputs/invalid_lower_supersedes_master.lyra");
const INVALID_ARCHIVE_RANK: &str =
    include_str!("../fixtures/p00/authority_order_inputs/invalid_archive_rank_too_high.lyra");
const INVALID_OPERATOR_OVERRIDE: &str = include_str!(
    "../fixtures/p00/authority_order_inputs/invalid_operator_overrides_constitution.lyra"
);
const INVALID_AMBIENT_AUTHORITY: &str =
    include_str!("../fixtures/p00/authority_order_inputs/invalid_ambient_authority.lyra");
const GOLDEN_VALID_RECEIPT: &str = include_str!("../goldens/p00/valid_authority_order.receipt");

#[test]
fn valid_authority_order_is_accepted_and_receipted() {
    let (verdict, receipt) = validate_authority_order_surface(VALID);
    assert!(verdict.accepted, "{}", verdict.canonical_text());
    assert_eq!(receipt.to_text(), GOLDEN_VALID_RECEIPT);
}

#[test]
fn valid_authority_order_parses_required_precedence() {
    let parsed = parse_authority_order_surface(VALID).expect("valid authority order must parse");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-002");
    assert_eq!(parsed.status, "working_slice");
    assert_eq!(parsed.sorted_layers()[0].name, "single_file_master");
    assert_eq!(
        parsed
            .layer_by_name("archive_context")
            .expect("archive layer")
            .rank,
        70
    );
    assert_eq!(parsed.rule_value("strict_total_order"), Some("required"));
}

#[test]
fn missing_single_file_master_is_rejected() {
    let (verdict, _receipt) = validate_authority_order_surface(INVALID_MISSING_MASTER);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_master_authority"));
    assert!(text.contains("missing_authority_layer"));
}

#[test]
fn duplicate_authority_rank_is_rejected() {
    let (verdict, _receipt) = validate_authority_order_surface(INVALID_DUPLICATE_RANK);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("duplicate_authority_rank"));
}

#[test]
fn lower_authority_superseding_master_is_rejected() {
    let (verdict, _receipt) = validate_authority_order_surface(INVALID_LOWER_SUPERSEDES_MASTER);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("authority_supersession_violation"));
    assert!(text.contains("single_file_master"));
}

#[test]
fn archive_context_ranked_too_high_is_rejected() {
    let (verdict, _receipt) = validate_authority_order_surface(INVALID_ARCHIVE_RANK);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("archive_authority_too_high"));
    assert!(text.contains("archive_context"));
}

#[test]
fn operator_override_of_constitution_is_rejected() {
    let (verdict, _receipt) = validate_authority_order_surface(INVALID_OPERATOR_OVERRIDE);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(
        text.contains("operator_override_constitition")
            || text.contains("operator_override_constitution")
    );
}

#[test]
fn ambient_authority_is_rejected() {
    let (verdict, _receipt) = validate_authority_order_surface(INVALID_AMBIENT_AUTHORITY);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("ambient_authority"));
}

#[test]
fn authority_receipts_are_reproducible_for_identical_input() {
    let (_left_verdict, left_receipt) = validate_authority_order_surface(VALID);
    let (_right_verdict, right_receipt) = validate_authority_order_surface(VALID);
    assert_eq!(left_receipt, right_receipt);
}
