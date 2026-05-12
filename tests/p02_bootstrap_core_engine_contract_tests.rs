use lyra_phase0::p02::{
    bootstrap_core_engine_artifact_ids, bootstrap_core_engine_artifacts_bind_paths,
    bootstrap_core_engine_no_forbidden_descriptor_claims, bootstrap_core_engine_proof_ids,
    bootstrap_core_engine_proofs_bind_registry, bootstrap_core_engine_registry_hash,
    bootstrap_core_engine_registry_signature, bootstrap_core_engine_transition_ids,
    bootstrap_core_engine_transitions_bind_known_units, bootstrap_core_engine_unit_ids,
    bootstrap_core_engine_units_have_stable_order, LYRA_P02_BOOTSTRAP_CORE_ENGINE_CARRIER,
    REQUIRED_BOOTSTRAP_CORE_ENGINE_ARTIFACTS, REQUIRED_BOOTSTRAP_CORE_ENGINE_PROOFS,
    REQUIRED_BOOTSTRAP_CORE_ENGINE_TRANSITIONS, REQUIRED_BOOTSTRAP_CORE_ENGINE_UNITS,
};

#[test]
fn lyralang_bootstrap_core_engine_registry_is_complete() {
    assert_eq!(
        bootstrap_core_engine_unit_ids().len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_UNITS.len()
    );
    assert_eq!(
        bootstrap_core_engine_transition_ids().len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_TRANSITIONS.len()
    );
    assert_eq!(
        bootstrap_core_engine_artifact_ids().len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_ARTIFACTS.len()
    );
    assert_eq!(
        bootstrap_core_engine_proof_ids().len(),
        REQUIRED_BOOTSTRAP_CORE_ENGINE_PROOFS.len()
    );
    assert!(bootstrap_core_engine_units_have_stable_order());
    assert!(bootstrap_core_engine_transitions_bind_known_units());
    assert!(bootstrap_core_engine_artifacts_bind_paths());
    assert!(bootstrap_core_engine_proofs_bind_registry());
    assert!(bootstrap_core_engine_no_forbidden_descriptor_claims());
    assert!(bootstrap_core_engine_registry_hash().starts_with("fnv1a128:"));
    assert!(bootstrap_core_engine_registry_signature()
        .starts_with(LYRA_P02_BOOTSTRAP_CORE_ENGINE_CARRIER));
}
