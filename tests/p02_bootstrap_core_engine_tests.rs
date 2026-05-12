use lyra_phase0::p02::{
    parse_bootstrap_core_engine_surface, validate_bootstrap_core_engine_surface, ErrorCode,
    REQUIRED_BOOTSTRAP_CORE_ENGINE_ARTIFACTS, REQUIRED_BOOTSTRAP_CORE_ENGINE_PROOFS,
    REQUIRED_BOOTSTRAP_CORE_ENGINE_RULES, REQUIRED_BOOTSTRAP_CORE_ENGINE_TRANSITIONS,
    REQUIRED_BOOTSTRAP_CORE_ENGINE_UNITS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p02/bootstrap_core_engine_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_bootstrap_core_engine() {
    let input = fixture("valid_bootstrap_core_engine.lyra");
    let parsed = parse_bootstrap_core_engine_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-015");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_RULES.len()
    );
    assert_eq!(
        parsed.units.len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_UNITS.len()
    );
    assert_eq!(
        parsed.transitions.len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_TRANSITIONS.len()
    );
    assert_eq!(
        parsed.artifacts.len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_ARTIFACTS.len()
    );
    assert_eq!(
        parsed.proofs.len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_PROOFS.len()
    );
    assert!(parsed
        .unit_by_id("bootstrap_authority_ingest_engine")
        .is_some());
    assert!(parsed
        .proof_by_id("p02_bootstrap_core_engine_parity_proof")
        .is_some());
    let (verdict, receipt) = validate_bootstrap_core_engine_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_duplicate_and_invalid_bootstrap_core_engine() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingEngineRule),
        ("invalid_missing_unit.lyra", ErrorCode::MissingEngineUnit),
        ("invalid_duplicate_unit.lyra", ErrorCode::DuplicateEntry),
        ("invalid_bad_unit_root.lyra", ErrorCode::InvalidOwnerRoot),
        (
            "invalid_bad_unit_order.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_missing_transition.lyra",
            ErrorCode::MissingTransitionBinding,
        ),
        (
            "invalid_bad_transition_endpoint.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_forked_carrier.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_missing_artifact.lyra",
            ErrorCode::MissingDeliveryArtifact,
        ),
        (
            "invalid_bad_artifact_path.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        ("invalid_missing_proof.lyra", ErrorCode::MissingEngineProof),
        (
            "invalid_bad_proof_unit.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_bad_proof_receipt.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_bad_status.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_core_engine_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_bootstrap_core_engine_claims() {
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
            "invalid_probabilistic_engine.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_placeholder_engine.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        ("invalid_engine_drift.lyra", ErrorCode::EngineDriftAccepted),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_core_engine_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
