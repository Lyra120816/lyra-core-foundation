use std::fs;

use lyra_phase0::p02::{
    bootstrap_proof_family_artifacts_bind_paths, bootstrap_proof_family_carrier_signature,
    bootstrap_proof_family_families_bind_receipts,
    bootstrap_proof_family_no_forbidden_descriptor_claims,
    bootstrap_proof_family_paths_bind_receipts, bootstrap_proof_family_receipts_bind_families,
    bootstrap_proof_family_receipts_cover_p02_001_through_p02_x01,
    bootstrap_proof_family_registry_hash, validate_bootstrap_proof_family_surface,
    P02_BOOTSTRAP_PROOF_FAMILY_CONTRACT,
};

#[test]
fn bootstrap_proof_family_contract_and_registry_are_bound() {
    let contract = fs::read_to_string("interfaces/p02/contracts/bootstrap_proof_family.v1.lyra")
        .expect("contract exists");
    assert!(contract.contains(P02_BOOTSTRAP_PROOF_FAMILY_CONTRACT));
    assert!(contract.contains("global_closure=denied"));
    assert!(bootstrap_proof_family_artifacts_bind_paths());
    assert!(bootstrap_proof_family_families_bind_receipts());
    assert!(bootstrap_proof_family_receipts_bind_families());
    assert!(bootstrap_proof_family_paths_bind_receipts());
    assert!(bootstrap_proof_family_receipts_cover_p02_001_through_p02_x01());
    assert!(bootstrap_proof_family_no_forbidden_descriptor_claims());
    assert!(bootstrap_proof_family_registry_hash().starts_with("fnv1a128:"));
    assert!(bootstrap_proof_family_carrier_signature().starts_with("fnv1a128:"));
}

#[test]
fn emitted_p02_x02_gate_accepts_without_closing_phase() {
    let surface = fs::read_to_string("ops/p02/closure/p02_x02_proof_family_gate.v1.lyra")
        .expect("surface exists");
    let (verdict, receipt) = validate_bootstrap_proof_family_surface(&surface);
    assert!(verdict.accepted, "{}", receipt.to_text());
    assert!(surface.contains("closure_scope=extended_open"));
    assert!(surface.contains("global_closure=denied"));
    assert!(surface.contains("next_frontier=P02-X03"));
}
