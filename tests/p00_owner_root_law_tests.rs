use lyra_phase0::p00::{
    validate_owner_root_law_surface, ErrorCode, REQUIRED_OWNER_ROOTS, REQUIRED_OWNER_ROOT_RULES,
};

const VALID: &str = include_str!("../fixtures/p00/owner_root_law_inputs/valid_owner_root_law.lyra");
const INVALID_MISSING_K1: &str =
    include_str!("../fixtures/p00/owner_root_law_inputs/invalid_missing_k1_root.lyra");
const INVALID_DUPLICATE_ROOT: &str =
    include_str!("../fixtures/p00/owner_root_law_inputs/invalid_duplicate_owner_root.lyra");
const INVALID_UNKNOWN_ROOT: &str =
    include_str!("../fixtures/p00/owner_root_law_inputs/invalid_unknown_owner_root.lyra");
const INVALID_MISPLACED_LOGIC: &str =
    include_str!("../fixtures/p00/owner_root_law_inputs/invalid_misplaced_logic.lyra");
const INVALID_CONTROL_PLANE_CORE: &str =
    include_str!("../fixtures/p00/owner_root_law_inputs/invalid_control_plane_core_ownership.lyra");
const INVALID_UNBOUNDED_PLATFORM: &str =
    include_str!("../fixtures/p00/owner_root_law_inputs/invalid_unbounded_platform_root.lyra");
const INVALID_PRODUCT_CORE: &str =
    include_str!("../fixtures/p00/owner_root_law_inputs/invalid_product_core_ownership.lyra");
const INVALID_MISSING_ACTIVE_RESPONSIBILITY: &str = include_str!(
    "../fixtures/p00/owner_root_law_inputs/invalid_missing_active_responsibility.lyra"
);
const INVALID_RESERVED_RESPONSIBILITY: &str = include_str!(
    "../fixtures/p00/owner_root_law_inputs/invalid_reserved_root_with_responsibility.lyra"
);
const INVALID_PLACEHOLDER_LANGUAGE: &str =
    include_str!("../fixtures/p00/owner_root_law_inputs/invalid_placeholder_root_language.lyra");

fn assert_rejects_with(input: &str, expected: ErrorCode) {
    let (verdict, receipt) = validate_owner_root_law_surface(input);
    assert!(
        !verdict.accepted,
        "fixture unexpectedly accepted\n{}",
        receipt.to_text()
    );
    assert!(
        verdict.errors.iter().any(|error| error.code == expected),
        "expected {:?}, got {:?}",
        expected,
        verdict.errors
    );
    assert!(receipt.to_text().contains("verdict=REJECTED"));
}

#[test]
fn valid_owner_root_law_accepts_and_receipts() {
    let (verdict, receipt) = validate_owner_root_law_surface(VALID);
    assert!(verdict.accepted, "unexpected errors: {:?}", verdict.errors);
    assert!(receipt.to_text().contains("LYRA-P00-RECEIPT v1"));
    assert!(receipt.to_text().contains("verdict=ACCEPTED"));
}

#[test]
fn valid_owner_root_law_binds_every_required_rule_and_root() {
    for rule in REQUIRED_OWNER_ROOT_RULES {
        assert!(
            VALID.contains(&format!("rule:{rule}=required")),
            "missing rule {rule}"
        );
    }
    for root in REQUIRED_OWNER_ROOTS {
        assert!(
            VALID.contains(&format!("root:{root}=")),
            "missing owner root {root}"
        );
    }
}

#[test]
fn missing_required_owner_root_rejected() {
    assert_rejects_with(INVALID_MISSING_K1, ErrorCode::MissingOwnerRootBinding);
}

#[test]
fn duplicate_owner_root_rejected() {
    assert_rejects_with(INVALID_DUPLICATE_ROOT, ErrorCode::DuplicateOwnerRootBinding);
}

#[test]
fn unknown_owner_root_rejected() {
    assert_rejects_with(INVALID_UNKNOWN_ROOT, ErrorCode::InvalidOwnerRootBinding);
}

#[test]
fn misplaced_logic_rejected() {
    assert_rejects_with(INVALID_MISPLACED_LOGIC, ErrorCode::MisplacedOwnerRoot);
}

#[test]
fn control_plane_core_ownership_rejected() {
    assert_rejects_with(
        INVALID_CONTROL_PLANE_CORE,
        ErrorCode::RootOwnershipViolation,
    );
}

#[test]
fn unbounded_platform_root_rejected() {
    assert_rejects_with(INVALID_UNBOUNDED_PLATFORM, ErrorCode::PlatformRootUnbounded);
}

#[test]
fn product_core_ownership_rejected() {
    assert_rejects_with(INVALID_PRODUCT_CORE, ErrorCode::ProductRootCoreOwnership);
}

#[test]
fn missing_active_responsibility_rejected() {
    assert_rejects_with(
        INVALID_MISSING_ACTIVE_RESPONSIBILITY,
        ErrorCode::MissingRootResponsibility,
    );
}

#[test]
fn reserved_root_responsibility_rejected() {
    assert_rejects_with(
        INVALID_RESERVED_RESPONSIBILITY,
        ErrorCode::InvalidRootResponsibility,
    );
}

#[test]
fn placeholder_root_language_rejected() {
    assert_rejects_with(INVALID_PLACEHOLDER_LANGUAGE, ErrorCode::PlaceholderAllowed);
}
