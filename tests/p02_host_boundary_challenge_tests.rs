use lyra_phase0::p02::{
    parse_host_boundary_challenge_surface, validate_host_boundary_challenge_surface, ErrorCode,
    REQUIRED_HOST_BOUNDARY_PROBES, REQUIRED_HOST_BOUNDARY_RECEIPTS, REQUIRED_HOST_BOUNDARY_RULES,
    REQUIRED_HOST_BOUNDARY_SUITES,
};
fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/host_boundary_challenge_inputs/{name}"
    ))
    .expect("fixture must exist")
}
#[test]
fn accepts_valid_host_boundary_challenge_suites() {
    let input = fixture("valid_host_boundary_challenge_suites.lyra");
    let parsed = parse_host_boundary_challenge_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-005");
    assert_eq!(parsed.rules.len(), REQUIRED_HOST_BOUNDARY_RULES.len());
    assert_eq!(parsed.suites.len(), REQUIRED_HOST_BOUNDARY_SUITES.len());
    assert_eq!(parsed.probes.len(), REQUIRED_HOST_BOUNDARY_PROBES.len());
    assert_eq!(parsed.receipts.len(), REQUIRED_HOST_BOUNDARY_RECEIPTS.len());
    assert!(parsed
        .probe_for_surface("surface:rust_bootstrap_compiler")
        .is_some());
    assert!(parsed
        .suites
        .iter()
        .all(|suite| suite.binds_boundary_surface()));
    let (verdict, receipt) = validate_host_boundary_challenge_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}
#[test]
fn rejects_missing_duplicate_and_invalid_host_boundary_rows() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_suite.lyra",
            ErrorCode::MissingClosureOutputGate,
        ),
        (
            "invalid_duplicate_suite.lyra",
            ErrorCode::DuplicateClosureOutputGate,
        ),
        ("invalid_bad_owner_root.lyra", ErrorCode::InvalidOwnerRoot),
        (
            "invalid_missing_probe.lyra",
            ErrorCode::MissingChallengeFixture,
        ),
        (
            "invalid_unknown_probe_suite.lyra",
            ErrorCode::InvalidChallengeFixture,
        ),
        (
            "invalid_unknown_surface_ref.lyra",
            ErrorCode::InvalidChallengeFixture,
        ),
        (
            "invalid_missing_evidence.lyra",
            ErrorCode::MissingEvidenceBinding,
        ),
        (
            "invalid_unreceipted_suite.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_bad_expected_rejection.lyra",
            ErrorCode::InvalidChallengeFixture,
        ),
        (
            "invalid_bad_containment_gate.lyra",
            ErrorCode::InvalidChallengeFixture,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        (
            "invalid_unledgered_surface_accepted.lyra",
            ErrorCode::NegativeFixtureAccepted,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_host_boundary_challenge_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
#[test]
fn rejects_forbidden_host_boundary_claims_and_status_drift() {
    for (name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        ("invalid_ambient_time.lyra", ErrorCode::AmbientTimeAllowed),
        (
            "invalid_placeholder_suite.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_foreign_ownership.lyra",
            ErrorCode::RootOwnershipViolation,
        ),
        (
            "invalid_bad_status.lyra",
            ErrorCode::UnsupportedClosureStatus,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_host_boundary_challenge_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
