use lyra_phase0::p02::{
    parse_bootstrap_extinction_ledger_surface, validate_bootstrap_extinction_ledger_surface,
    ErrorCode, REQUIRED_BOOTSTRAP_EXTINCTION_ENTRIES, REQUIRED_BOOTSTRAP_EXTINCTION_RECEIPTS,
    REQUIRED_BOOTSTRAP_EXTINCTION_RULES, REQUIRED_BOOTSTRAP_EXTINCTION_LEDGER_GATES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p02/bootstrap_extinction_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_bootstrap_extinction_ledger() {
    let input = fixture("valid_bootstrap_extinction_ledger.lyra");
    let parsed = parse_bootstrap_extinction_ledger_surface(&input)
        .expect("valid P02 bootstrap extinction ledger parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-002");
    assert_eq!(
        parsed.inventory_receipt,
        "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt"
    );
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_BOOTSTRAP_EXTINCTION_RULES.len()
    );
    assert_eq!(
        parsed.entries.len(),
        REQUIRED_BOOTSTRAP_EXTINCTION_ENTRIES.len()
    );
    assert_eq!(
        parsed.gates.len(),
        REQUIRED_BOOTSTRAP_EXTINCTION_LEDGER_GATES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_BOOTSTRAP_EXTINCTION_RECEIPTS.len()
    );
    assert!(parsed.temporary_entries().count() > 0);
    assert!(parsed.observer_entries().count() > 0);
    assert!(parsed.bounded_permanent_entries().count() > 0);
    assert!(parsed.forbidden_entries().count() > 0);
    let (verdict, receipt) = validate_bootstrap_extinction_ledger_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_duplicate_and_invalid_ledger_rows() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_entry.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_duplicate_entry.lyra",
            ErrorCode::DuplicateClosureOutputGate,
        ),
        (
            "invalid_bad_classification.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        ("invalid_bad_owner_root.lyra", ErrorCode::InvalidOwnerRoot),
        (
            "invalid_missing_gate.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_gate_mismatch.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_missing_inventory_receipt.lyra",
            ErrorCode::MissingReceiptProof,
        ),
        (
            "invalid_unreceipted_entry.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_bootstrap_extinction_ledger_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_class_law_and_forbidden_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_temporary_without_successor.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_observer_truth_successor.lyra",
            ErrorCode::AmbientAuthority,
        ),
        (
            "invalid_bounded_permanent_deletion.lyra",
            ErrorCode::RootOwnershipViolation,
        ),
        (
            "invalid_forbidden_import_allowed.lyra",
            ErrorCode::InvalidClosureOutputGate,
        ),
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_ledger.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_ambient_time_gate.lyra",
            ErrorCode::AmbientTimeAllowed,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_bootstrap_extinction_ledger_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
