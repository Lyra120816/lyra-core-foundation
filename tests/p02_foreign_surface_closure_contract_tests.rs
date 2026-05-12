use lyra_phase0::p02::{
    foreign_bootstrap_surface_ids, foreign_surface_all_have_challenges,
    foreign_surface_all_have_closure_laws, foreign_surface_all_truth_neutral,
    foreign_surface_challenge_ids, foreign_surface_closure_law_ids,
    foreign_surface_closure_receipt_paths_are_local, foreign_surface_closure_registry_hash,
    foreign_surface_closure_registry_signature, LYRA_P02_FOREIGN_SURFACE_CLOSURE_CARRIER,
    REQUIRED_FOREIGN_BOOTSTRAP_SURFACES,
};

#[test]
fn lyralang_foreign_surface_closure_registry_is_complete() {
    assert_eq!(
        foreign_bootstrap_surface_ids().len(),
        REQUIRED_FOREIGN_BOOTSTRAP_SURFACES.len()
    );
    assert_eq!(
        foreign_surface_challenge_ids().len(),
        REQUIRED_FOREIGN_BOOTSTRAP_SURFACES.len()
    );
    assert_eq!(
        foreign_surface_closure_law_ids().len(),
        REQUIRED_FOREIGN_BOOTSTRAP_SURFACES.len()
    );
    assert!(foreign_surface_all_truth_neutral());
    assert!(foreign_surface_all_have_challenges());
    assert!(foreign_surface_all_have_closure_laws());
    assert!(foreign_surface_closure_receipt_paths_are_local());
    assert!(foreign_surface_closure_registry_hash().starts_with("fnv1a128:"));
    assert!(foreign_surface_closure_registry_signature()
        .starts_with(LYRA_P02_FOREIGN_SURFACE_CLOSURE_CARRIER));
}
