use lyra_phase0::p02::{
    parse_bootstrap_inventory_surface, validate_bootstrap_inventory_surface, ErrorCode,
    REQUIRED_BOOTSTRAP_INVENTORY_RULES, REQUIRED_BOOTSTRAP_SURFACES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p02/bootstrap_inventory_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_bootstrap_surface_inventory() {
    let input = fixture("valid_bootstrap_surface_inventory.lyra");
    let parsed =
        parse_bootstrap_inventory_surface(&input).expect("valid P02 bootstrap inventory parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-001");
    assert_eq!(parsed.rules.len(), REQUIRED_BOOTSTRAP_INVENTORY_RULES.len());
    assert_eq!(parsed.surfaces.len(), REQUIRED_BOOTSTRAP_SURFACES.len());
    assert!(parsed.temporary_surfaces().count() > 0);
    assert!(parsed.observer_surfaces().count() > 0);
    assert!(parsed.bounded_permanent_surfaces().count() > 0);
    assert!(parsed.forbidden_surfaces().count() > 0);
    let (verdict, receipt) = validate_bootstrap_inventory_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_duplicate_and_invalid_inventory_rows() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_surface.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_duplicate_surface.lyra",
            ErrorCode::DuplicateClosureOutputGate,
        ),
        (
            "invalid_bad_classification.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_invalid_owner_root.lyra",
            ErrorCode::InvalidOwnerRoot,
        ),
        (
            "invalid_temporary_without_retirement_ref.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_unreceipted_surface.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_bootstrap_inventory_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_bad_classification_law_and_forbidden_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_observer_truth_influence.lyra",
            ErrorCode::AmbientAuthority,
        ),
        (
            "invalid_bounded_permanent_truth_owner.lyra",
            ErrorCode::RootOwnershipViolation,
        ),
        (
            "invalid_forbidden_active.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_missing_observer_class.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_truth.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_inventory.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_bootstrap_inventory_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
