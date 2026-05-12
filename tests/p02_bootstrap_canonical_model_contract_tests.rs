use lyra_phase0::p02::{
    bootstrap_canonical_all_fields_bind_models, bootstrap_canonical_all_models_have_local_schema,
    bootstrap_canonical_all_relations_bind_models, bootstrap_canonical_all_schemas_bind_models,
    bootstrap_canonical_field_ids, bootstrap_canonical_model_ids,
    bootstrap_canonical_model_registry_hash, bootstrap_canonical_model_registry_signature,
    bootstrap_canonical_relation_ids, bootstrap_canonical_schema_ids,
    LYRA_P02_BOOTSTRAP_CANONICAL_MODEL_CARRIER, REQUIRED_BOOTSTRAP_CANONICAL_FIELDS,
    REQUIRED_BOOTSTRAP_CANONICAL_MODELS, REQUIRED_BOOTSTRAP_CANONICAL_RELATIONS,
    REQUIRED_BOOTSTRAP_CANONICAL_SCHEMAS,
};

#[test]
fn lyralang_bootstrap_canonical_model_registry_is_complete() {
    assert_eq!(
        bootstrap_canonical_model_ids().len(),
        REQUIRED_BOOTSTRAP_CANONICAL_MODELS.len()
    );
    assert_eq!(
        bootstrap_canonical_schema_ids().len(),
        REQUIRED_BOOTSTRAP_CANONICAL_SCHEMAS.len()
    );
    assert_eq!(
        bootstrap_canonical_field_ids().len(),
        REQUIRED_BOOTSTRAP_CANONICAL_FIELDS.len()
    );
    assert_eq!(
        bootstrap_canonical_relation_ids().len(),
        REQUIRED_BOOTSTRAP_CANONICAL_RELATIONS.len()
    );
    assert!(bootstrap_canonical_all_models_have_local_schema());
    assert!(bootstrap_canonical_all_schemas_bind_models());
    assert!(bootstrap_canonical_all_fields_bind_models());
    assert!(bootstrap_canonical_all_relations_bind_models());
    assert!(bootstrap_canonical_model_registry_hash().starts_with("fnv1a128:"));
    assert!(bootstrap_canonical_model_registry_signature()
        .starts_with(LYRA_P02_BOOTSTRAP_CANONICAL_MODEL_CARRIER));
}
