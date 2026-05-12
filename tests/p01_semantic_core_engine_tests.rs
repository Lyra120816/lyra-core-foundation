use lyra_phase0::p01::{
    parse_semantic_core_engine_surface, semantic_core_engine_artifact_descriptor,
    semantic_core_engine_artifact_digest, semantic_core_engine_artifact_ids,
    semantic_core_engine_artifacts_bind_paths, semantic_core_engine_no_forbidden_descriptor_claims,
    semantic_core_engine_proof_descriptor, semantic_core_engine_proof_digest,
    semantic_core_engine_proof_ids, semantic_core_engine_proofs_bind_registry,
    semantic_core_engine_registry_hash, semantic_core_engine_registry_signature,
    semantic_core_engine_transition_descriptor, semantic_core_engine_transition_digest,
    semantic_core_engine_transition_ids, semantic_core_engine_transitions_bind_known_units,
    semantic_core_engine_unit_descriptor, semantic_core_engine_unit_digest,
    semantic_core_engine_unit_ids, semantic_core_engine_units_have_stable_order,
    validate_semantic_core_engine_surface, ErrorCode, REQUIRED_SEMANTIC_CORE_ENGINE_ARTIFACTS,
    REQUIRED_SEMANTIC_CORE_ENGINE_PROOFS, REQUIRED_SEMANTIC_CORE_ENGINE_RULES,
    REQUIRED_SEMANTIC_CORE_ENGINE_TRANSITIONS, REQUIRED_SEMANTIC_CORE_ENGINE_UNITS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/semantic_core_engine_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_core_engine_surface() {
    let input = fixture("valid_semantic_core_engine.lyra");
    let parsed =
        parse_semantic_core_engine_surface(&input).expect("valid semantic core engine parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-015");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_RULES.len()
    );
    assert_eq!(
        parsed.units.len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_UNITS.len()
    );
    assert_eq!(
        parsed.transitions.len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_TRANSITIONS.len()
    );
    assert_eq!(
        parsed.artifacts.len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_ARTIFACTS.len()
    );
    assert_eq!(
        parsed.proofs.len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_PROOFS.len()
    );
    let (verdict, receipt) = validate_semantic_core_engine_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_binds_units_transitions_artifacts_and_proofs() {
    let signature = semantic_core_engine_registry_signature();
    assert!(signature.contains("unit:canonical_symbol_ingest_engine"));
    assert!(signature.contains("transition:receipt_commit_to_replay_witness"));
    assert!(signature.contains("artifact:engine_operator"));
    assert!(signature.contains("proof:p01_semantic_core_engine_parity_proof"));
    assert_eq!(
        semantic_core_engine_unit_ids().len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_UNITS.len()
    );
    assert_eq!(
        semantic_core_engine_transition_ids().len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_TRANSITIONS.len()
    );
    assert_eq!(
        semantic_core_engine_artifact_ids().len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_ARTIFACTS.len()
    );
    assert_eq!(
        semantic_core_engine_proof_ids().len(),
        REQUIRED_SEMANTIC_CORE_ENGINE_PROOFS.len()
    );
    assert!(semantic_core_engine_registry_hash().starts_with("fnv1a128:"));
}

#[test]
fn descriptors_bind_one_engine_and_real_artifacts() {
    assert!(semantic_core_engine_units_have_stable_order());
    assert!(semantic_core_engine_transitions_bind_known_units());
    assert!(semantic_core_engine_artifacts_bind_paths());
    assert!(semantic_core_engine_proofs_bind_registry());
    assert!(semantic_core_engine_no_forbidden_descriptor_claims());

    for id in semantic_core_engine_unit_ids() {
        let descriptor = semantic_core_engine_unit_descriptor(id).expect("unit descriptor exists");
        assert!(descriptor.stage_order.len() == 3);
        assert!(semantic_core_engine_unit_digest(id)
            .expect("unit digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_core_engine_transition_ids() {
        let descriptor =
            semantic_core_engine_transition_descriptor(id).expect("transition descriptor exists");
        assert_eq!(descriptor.carry, "single_carrier_state");
        assert!(semantic_core_engine_transition_digest(id)
            .expect("transition digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_core_engine_artifact_ids() {
        let descriptor =
            semantic_core_engine_artifact_descriptor(id).expect("artifact descriptor exists");
        assert!(
            std::fs::metadata(descriptor.path).is_ok(),
            "missing {}",
            descriptor.path
        );
        assert!(semantic_core_engine_artifact_digest(id)
            .expect("artifact digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_core_engine_proof_ids() {
        let descriptor =
            semantic_core_engine_proof_descriptor(id).expect("proof descriptor exists");
        assert!(
            std::fs::metadata(descriptor.fixture).is_ok(),
            "missing {}",
            descriptor.fixture
        );
        assert!(
            std::fs::metadata(descriptor.golden).is_ok(),
            "missing {}",
            descriptor.golden
        );
        assert!(
            std::fs::metadata(descriptor.receipt).is_ok(),
            "missing {}",
            descriptor.receipt
        );
        assert!(semantic_core_engine_proof_digest(id)
            .expect("proof digest")
            .starts_with("fnv1a128:"));
    }
}

#[test]
fn rejects_required_semantic_core_engine_gaps_and_drift() {
    for (fixture_name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingEngineRule),
        (
            "invalid_missing_engine_unit.lyra",
            ErrorCode::MissingEngineUnit,
        ),
        (
            "invalid_missing_transition.lyra",
            ErrorCode::MissingTransitionBinding,
        ),
        (
            "invalid_missing_artifact.lyra",
            ErrorCode::MissingDeliveryArtifact,
        ),
        ("invalid_missing_proof.lyra", ErrorCode::MissingEngineProof),
        ("invalid_duplicate_unit.lyra", ErrorCode::DuplicateEntry),
        (
            "invalid_unit_descriptor_drift.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_transition_descriptor_drift.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_artifact_descriptor_drift.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_proof_descriptor_drift.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        ("invalid_wrong_task.lyra", ErrorCode::InvalidTask),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_unstable_order.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        (
            "invalid_unknown_model_binding.lyra",
            ErrorCode::EngineDriftAccepted,
        ),
        ("invalid_receipt_path.lyra", ErrorCode::EngineDriftAccepted),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_core_engine_surface(&input);
        assert!(!verdict.accepted, "{fixture_name} should reject");
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} expected {:?}, got {:?}",
            expected,
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_core_engine_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_engine.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_engine.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_core_engine_surface(&input);
        assert!(!verdict.accepted, "{fixture_name} should reject");
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} expected {:?}, got {:?}",
            expected,
            verdict.errors
        );
    }
}
