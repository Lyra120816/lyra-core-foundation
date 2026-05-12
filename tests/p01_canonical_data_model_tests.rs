use lyra_phase0::p01::{
    canonical_data_model_descriptor, canonical_data_model_digest, canonical_data_model_ids,
    canonical_data_model_registry_hash, canonical_data_model_registry_signature,
    canonical_data_models_have_schema_refs, canonical_data_no_forbidden_descriptor_claims,
    canonical_data_proof_descriptor, canonical_data_proof_digest, canonical_data_proof_ids,
    canonical_data_proofs_bind_artifacts, canonical_field_descriptor, canonical_field_digest,
    canonical_field_ids, canonical_fields_bind_known_models, canonical_model_bridge_descriptor,
    canonical_model_bridge_digest, canonical_model_bridge_ids,
    canonical_model_bridges_bind_one_carrier, canonical_schema_descriptor, canonical_schema_digest,
    canonical_schema_ids, canonical_schemas_bind_known_models_and_fields,
    parse_canonical_data_model_surface, validate_canonical_data_model_surface, ErrorCode,
    REQUIRED_P01_CANONICAL_DATA_MODELS, REQUIRED_P01_CANONICAL_DATA_MODEL_RULES,
    REQUIRED_P01_CANONICAL_DATA_PROOFS, REQUIRED_P01_CANONICAL_FIELDS,
    REQUIRED_P01_CANONICAL_MODEL_BRIDGES, REQUIRED_P01_CANONICAL_SCHEMAS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/canonical_data_model_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_canonical_data_model_surface() {
    let input = fixture("valid_canonical_data_model.lyra");
    let parsed =
        parse_canonical_data_model_surface(&input).expect("valid canonical data model parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-014");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_P01_CANONICAL_DATA_MODEL_RULES.len()
    );
    assert_eq!(
        parsed.models.len(),
        REQUIRED_P01_CANONICAL_DATA_MODELS.len()
    );
    assert_eq!(parsed.schemas.len(), REQUIRED_P01_CANONICAL_SCHEMAS.len());
    assert_eq!(parsed.fields.len(), REQUIRED_P01_CANONICAL_FIELDS.len());
    assert_eq!(
        parsed.bridges.len(),
        REQUIRED_P01_CANONICAL_MODEL_BRIDGES.len()
    );
    assert_eq!(
        parsed.proofs.len(),
        REQUIRED_P01_CANONICAL_DATA_PROOFS.len()
    );
    let (verdict, receipt) = validate_canonical_data_model_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_binds_models_schemas_fields_bridges_and_proofs() {
    let signature = canonical_data_model_registry_signature();
    assert!(signature.contains("model:canonical_symbol_model"));
    assert!(signature.contains("schema:canonical_symbol_schema"));
    assert!(signature.contains("field:symbol_id"));
    assert!(signature.contains("bridge:symbols_to_atoms"));
    assert!(signature.contains("proof:p01_canonical_data_parity_proof"));
    assert_eq!(
        canonical_data_model_ids().len(),
        REQUIRED_P01_CANONICAL_DATA_MODELS.len()
    );
    assert_eq!(
        canonical_schema_ids().len(),
        REQUIRED_P01_CANONICAL_SCHEMAS.len()
    );
    assert_eq!(
        canonical_field_ids().len(),
        REQUIRED_P01_CANONICAL_FIELDS.len()
    );
    assert_eq!(
        canonical_model_bridge_ids().len(),
        REQUIRED_P01_CANONICAL_MODEL_BRIDGES.len()
    );
    assert_eq!(
        canonical_data_proof_ids().len(),
        REQUIRED_P01_CANONICAL_DATA_PROOFS.len()
    );
    assert!(canonical_data_model_registry_hash().starts_with("fnv1a128:"));
}

#[test]
fn descriptors_bind_one_core_and_real_artifacts() {
    assert!(canonical_data_models_have_schema_refs());
    assert!(canonical_schemas_bind_known_models_and_fields());
    assert!(canonical_fields_bind_known_models());
    assert!(canonical_model_bridges_bind_one_carrier());
    assert!(canonical_data_proofs_bind_artifacts());
    assert!(canonical_data_no_forbidden_descriptor_claims());

    for id in canonical_data_model_ids() {
        let descriptor = canonical_data_model_descriptor(id).expect("model descriptor exists");
        assert!(["lyralang", "interfaces", "k0", "ops"].contains(&descriptor.owner_root));
        assert!(canonical_data_model_digest(id)
            .expect("model digest")
            .starts_with("fnv1a128:"));
    }
    for id in canonical_schema_ids() {
        let descriptor = canonical_schema_descriptor(id).expect("schema descriptor exists");
        assert!(canonical_data_model_descriptor(descriptor.model_ref).is_some());
        assert!(canonical_schema_digest(id)
            .expect("schema digest")
            .starts_with("fnv1a128:"));
    }
    for id in canonical_field_ids() {
        let descriptor = canonical_field_descriptor(id).expect("field descriptor exists");
        assert!(descriptor.order.len() == 3);
        assert!(canonical_field_digest(id)
            .expect("field digest")
            .starts_with("fnv1a128:"));
    }
    for id in canonical_model_bridge_ids() {
        let descriptor = canonical_model_bridge_descriptor(id).expect("bridge descriptor exists");
        assert_eq!(descriptor.carrier, "lyra_p01_semantic_core");
        assert!(
            std::fs::metadata(descriptor.receipt_ref).is_ok(),
            "missing {}",
            descriptor.receipt_ref
        );
        assert!(canonical_model_bridge_digest(id)
            .expect("bridge digest")
            .starts_with("fnv1a128:"));
    }
    for id in canonical_data_proof_ids() {
        let descriptor = canonical_data_proof_descriptor(id).expect("proof descriptor exists");
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
        assert!(canonical_data_proof_digest(id)
            .expect("proof digest")
            .starts_with("fnv1a128:"));
    }
}

#[test]
fn rejects_required_canonical_data_gaps_and_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_model.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_schema.lyra",
            ErrorCode::MissingSchemaBinding,
        ),
        ("invalid_missing_field.lyra", ErrorCode::MissingFieldBinding),
        (
            "invalid_missing_bridge.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_proof.lyra",
            ErrorCode::MissingSemanticProof,
        ),
        ("invalid_duplicate_model.lyra", ErrorCode::DuplicateEntry),
        (
            "invalid_model_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_schema_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_field_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_bridge_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_proof_descriptor_drift.lyra",
            ErrorCode::SemanticDriftAccepted,
        ),
        ("invalid_wrong_task.lyra", ErrorCode::InvalidTask),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_forked_carrier_claim.lyra",
            ErrorCode::SemanticDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_canonical_data_model_surface(&input);
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
fn rejects_forbidden_canonical_data_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_field.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_model.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_canonical_data_model_surface(&input);
        assert!(!verdict.accepted, "{fixture_name} should reject");
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} expected {:?}, got {:?}",
            expected,
            verdict.errors
        );
    }
}
