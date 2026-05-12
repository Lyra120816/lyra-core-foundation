use lyra_phase0::p02::{
    bootstrap_constitutional_law_ids, bootstrap_formal_all_domains_constitutional,
    bootstrap_formal_all_laws_receipted, bootstrap_formal_all_transitions_guarded,
    bootstrap_formal_domain_ids, bootstrap_formal_invariant_ids,
    bootstrap_formal_invariants_reject_core_forbidden, bootstrap_formal_semantics_registry_hash,
    bootstrap_formal_semantics_registry_signature, bootstrap_formal_transition_ids,
    LYRA_P02_BOOTSTRAP_FORMAL_SEMANTICS_CARRIER, REQUIRED_BOOTSTRAP_CONSTITUTIONAL_LAWS,
    REQUIRED_BOOTSTRAP_FORMAL_DOMAINS, REQUIRED_BOOTSTRAP_FORMAL_INVARIANTS,
    REQUIRED_BOOTSTRAP_FORMAL_TRANSITIONS,
};

#[test]
fn lyralang_bootstrap_formal_semantics_registry_is_complete() {
    assert_eq!(
        bootstrap_formal_domain_ids().len(),
        REQUIRED_BOOTSTRAP_FORMAL_DOMAINS.len()
    );
    assert_eq!(
        bootstrap_constitutional_law_ids().len(),
        REQUIRED_BOOTSTRAP_CONSTITUTIONAL_LAWS.len()
    );
    assert_eq!(
        bootstrap_formal_transition_ids().len(),
        REQUIRED_BOOTSTRAP_FORMAL_TRANSITIONS.len()
    );
    assert_eq!(
        bootstrap_formal_invariant_ids().len(),
        REQUIRED_BOOTSTRAP_FORMAL_INVARIANTS.len()
    );
    assert!(bootstrap_formal_all_domains_constitutional());
    assert!(bootstrap_formal_all_laws_receipted());
    assert!(bootstrap_formal_all_transitions_guarded());
    assert!(bootstrap_formal_invariants_reject_core_forbidden());
    assert!(bootstrap_formal_semantics_registry_hash().starts_with("fnv1a128:"));
    assert!(bootstrap_formal_semantics_registry_signature()
        .starts_with(LYRA_P02_BOOTSTRAP_FORMAL_SEMANTICS_CARRIER));
}
