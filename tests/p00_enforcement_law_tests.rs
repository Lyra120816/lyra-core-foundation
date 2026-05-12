use lyra_phase0::p00::{parse_enforcement_surface, validate_enforcement_surface};

const VALID: &str =
    include_str!("../fixtures/p00/enforcement_law_inputs/valid_enforcement_law.lyra");
const INVALID_UNDERBUILD_DOCS_ONLY: &str =
    include_str!("../fixtures/p00/enforcement_law_inputs/invalid_underbuild_docs_only.lyra");
const INVALID_PLACEHOLDER_SCAFFOLD: &str =
    include_str!("../fixtures/p00/enforcement_law_inputs/invalid_placeholder_scaffold.lyra");
const INVALID_FAKE_CLOSURE_CLAIM: &str =
    include_str!("../fixtures/p00/enforcement_law_inputs/invalid_fake_closure_claim.lyra");
const INVALID_MISSING_TESTS: &str =
    include_str!("../fixtures/p00/enforcement_law_inputs/invalid_missing_tests.lyra");
const INVALID_MISSING_RECEIPTS: &str =
    include_str!("../fixtures/p00/enforcement_law_inputs/invalid_missing_receipts.lyra");
const INVALID_MISPLACED_OWNER_ROOT: &str =
    include_str!("../fixtures/p00/enforcement_law_inputs/invalid_misplaced_owner_root.lyra");
const INVALID_DUPLICATE_UNIT: &str =
    include_str!("../fixtures/p00/enforcement_law_inputs/invalid_duplicate_unit.lyra");
const GOLDEN_VALID_RECEIPT: &str = include_str!("../goldens/p00/valid_enforcement_law.receipt");

#[test]
fn valid_enforcement_law_is_accepted_and_receipted() {
    let (verdict, receipt) = validate_enforcement_surface(VALID);
    assert!(verdict.accepted, "{}", verdict.canonical_text());
    assert_eq!(receipt.to_text(), GOLDEN_VALID_RECEIPT);
}

#[test]
fn valid_enforcement_law_parses_units_rules_and_claims() {
    let parsed = parse_enforcement_surface(VALID).expect("valid enforcement law must parse");
    assert_eq!(parsed.phase, "P00");
    assert_eq!(parsed.task, "P00-004");
    assert_eq!(parsed.status, "working_slice");
    assert_eq!(parsed.rule_value("anti_underbuild"), Some("required"));
    assert_eq!(
        parsed
            .unit_by_id("p00_enforcement_law_validator")
            .expect("unit")
            .path,
        "ops/p00/src/enforcement.rs"
    );
    assert_eq!(
        parsed.claim_by_id("P00-004").expect("claim").status,
        "working_slice"
    );
}

#[test]
fn docs_only_underbuild_is_rejected() {
    let (verdict, _receipt) = validate_enforcement_surface(INVALID_UNDERBUILD_DOCS_ONLY);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("underbuild_violation") || text.contains("docs_only_implementation"));
}

#[test]
fn placeholder_scaffold_is_rejected() {
    let (verdict, _receipt) = validate_enforcement_surface(INVALID_PLACEHOLDER_SCAFFOLD);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("placeholder_allowed"));
}

#[test]
fn fake_closure_claim_is_rejected() {
    let (verdict, _receipt) = validate_enforcement_surface(INVALID_FAKE_CLOSURE_CLAIM);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("unsupported_global_closure"));
}

#[test]
fn missing_tests_are_rejected() {
    let (verdict, _receipt) = validate_enforcement_surface(INVALID_MISSING_TESTS);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_test_proof"));
}

#[test]
fn missing_receipts_are_rejected() {
    let (verdict, _receipt) = validate_enforcement_surface(INVALID_MISSING_RECEIPTS);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_receipt_proof"));
}

#[test]
fn misplaced_owner_root_is_rejected() {
    let (verdict, _receipt) = validate_enforcement_surface(INVALID_MISPLACED_OWNER_ROOT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("misplaced_owner_root"));
}

#[test]
fn duplicate_unit_is_rejected() {
    let (verdict, _receipt) = validate_enforcement_surface(INVALID_DUPLICATE_UNIT);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("duplicate_implementation_unit"));
}

#[test]
fn enforcement_receipts_are_reproducible_for_identical_input() {
    let (_left_verdict, left_receipt) = validate_enforcement_surface(VALID);
    let (_right_verdict, right_receipt) = validate_enforcement_surface(VALID);
    assert_eq!(left_receipt, right_receipt);
}
