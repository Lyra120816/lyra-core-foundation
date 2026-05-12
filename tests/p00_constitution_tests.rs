use lyra_phase0::p00::{parse_surface, validate_constitution_surface};

const VALID: &str = include_str!("../fixtures/p00/constitutional_inputs/valid_people_first.lyra");
const INVALID_MISSING_DETERMINISM: &str =
    include_str!("../fixtures/p00/constitutional_inputs/invalid_missing_determinism.lyra");
const INVALID_PLACEHOLDER: &str =
    include_str!("../fixtures/p00/constitutional_inputs/invalid_placeholder_allowed.lyra");
const INVALID_NETWORK: &str =
    include_str!("../fixtures/p00/constitutional_inputs/invalid_ambient_network.lyra");
const INVALID_FAKE_CLOSURE: &str =
    include_str!("../fixtures/p00/constitutional_inputs/invalid_fake_closure.lyra");
const GOLDEN_VALID_RECEIPT: &str = include_str!("../goldens/p00/valid_people_first.receipt");

#[test]
fn valid_people_first_constitution_is_accepted() {
    let (verdict, receipt) = validate_constitution_surface(VALID);
    assert!(verdict.accepted, "{}", verdict.canonical_text());
    assert_eq!(receipt.to_text(), GOLDEN_VALID_RECEIPT);
}

#[test]
fn valid_people_first_parse_is_stable() {
    let parsed = parse_surface(VALID).expect("valid surface must parse");
    assert_eq!(parsed.scalar_value("phase"), Some("P00"));
    assert_eq!(parsed.scalar_value("task"), Some("P00-001"));
    assert!(parsed.has_value("principle", "determinism"));
    assert!(parsed.has_value("owner_root", "k0"));
}

#[test]
fn missing_determinism_is_rejected() {
    let (verdict, _receipt) = validate_constitution_surface(INVALID_MISSING_DETERMINISM);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("missing_required_principle"));
    assert!(text.contains("principle:determinism"));
}

#[test]
fn placeholder_permission_is_rejected() {
    let (verdict, _receipt) = validate_constitution_surface(INVALID_PLACEHOLDER);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("placeholder_allowed"));
}

#[test]
fn ambient_network_permission_is_rejected() {
    let (verdict, _receipt) = validate_constitution_surface(INVALID_NETWORK);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("ambient_network_allowed"));
}

#[test]
fn fake_global_closure_is_rejected() {
    let (verdict, _receipt) = validate_constitution_surface(INVALID_FAKE_CLOSURE);
    let text = verdict.canonical_text();
    assert!(!verdict.accepted);
    assert!(text.contains("fake_closure_claim"));
    assert!(text.contains("unsupported_global_closure"));
}

#[test]
fn receipts_are_reproducible_for_identical_input() {
    let (_left_verdict, left_receipt) = validate_constitution_surface(VALID);
    let (_right_verdict, right_receipt) = validate_constitution_surface(VALID);
    assert_eq!(left_receipt, right_receipt);
}
