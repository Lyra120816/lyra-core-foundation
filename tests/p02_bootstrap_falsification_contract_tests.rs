use lyra_phase0::p02::{
    bootstrap_falsification_artifact_ids, bootstrap_falsification_artifacts_bind_paths,
    bootstrap_falsification_case_ids, bootstrap_falsification_harness_ids,
    bootstrap_falsification_harnesses_bind_known_cases,
    bootstrap_falsification_no_forbidden_descriptor_claims, bootstrap_falsification_proof_ids,
    bootstrap_falsification_proofs_bind_registry, bootstrap_falsification_registry_hash,
    bootstrap_falsification_registry_signature,
    bootstrap_falsification_targets_all_required_domains, bootstrap_rejection_assertion_ids,
    bootstrap_rejection_assertions_bind_known_cases, validate_bootstrap_falsification_surface,
    LYRA_P02_BOOTSTRAP_FALSIFICATION_CARRIER, P02_BOOTSTRAP_FALSIFICATION_CONTRACT,
    REQUIRED_BOOTSTRAP_FALSIFICATION_ARTIFACTS, REQUIRED_BOOTSTRAP_FALSIFICATION_CASES,
    REQUIRED_BOOTSTRAP_FALSIFICATION_HARNESSES, REQUIRED_BOOTSTRAP_FALSIFICATION_PROOFS,
    REQUIRED_BOOTSTRAP_FALSIFICATION_RULES, REQUIRED_BOOTSTRAP_REJECTION_ASSERTIONS,
};

const VALID: &str = include_str!(
    "../fixtures/p02/bootstrap_falsification_inputs/valid_bootstrap_falsification.lyra"
);
const CONTRACT: &str = include_str!("../interfaces/p02/contracts/bootstrap_falsification.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");

#[test]
fn bootstrap_falsification_contract_names_required_entities() {
    assert!(CONTRACT.contains("LYRA-P02-BOOTSTRAP-FALSIFICATION-CONTRACT v1"));
    for id in REQUIRED_BOOTSTRAP_FALSIFICATION_CASES {
        assert!(CONTRACT.contains(id), "contract missing case {id}");
    }
    for id in REQUIRED_BOOTSTRAP_FALSIFICATION_HARNESSES {
        assert!(CONTRACT.contains(id), "contract missing harness {id}");
    }
    for id in REQUIRED_BOOTSTRAP_REJECTION_ASSERTIONS {
        assert!(CONTRACT.contains(id), "contract missing assertion {id}");
    }
    for id in REQUIRED_BOOTSTRAP_FALSIFICATION_ARTIFACTS {
        assert!(CONTRACT.contains(id), "contract missing artifact {id}");
    }
    for id in REQUIRED_BOOTSTRAP_FALSIFICATION_PROOFS {
        assert!(CONTRACT.contains(id), "contract missing proof {id}");
    }
}

#[test]
fn bootstrap_falsification_surface_header_matches_runtime_contract() {
    assert_eq!(
        P02_BOOTSTRAP_FALSIFICATION_CONTRACT,
        "LYRA-P02-BOOTSTRAP-FALSIFICATION-CORPUS v1"
    );
    let (verdict, _receipt) = validate_bootstrap_falsification_surface(VALID);
    assert!(
        verdict.accepted,
        "valid bootstrap falsification contract fixture rejected: {:?}",
        verdict.errors
    );
    for rule in REQUIRED_BOOTSTRAP_FALSIFICATION_RULES {
        assert!(
            VALID.contains(&format!("rule:{rule}=")),
            "fixture missing rule {rule}"
        );
    }
}

#[test]
fn bootstrap_falsification_registry_is_complete_and_deterministic() {
    assert_eq!(
        bootstrap_falsification_case_ids().len(),
        REQUIRED_BOOTSTRAP_FALSIFICATION_CASES.len()
    );
    assert_eq!(
        bootstrap_falsification_harness_ids().len(),
        REQUIRED_BOOTSTRAP_FALSIFICATION_HARNESSES.len()
    );
    assert_eq!(
        bootstrap_rejection_assertion_ids().len(),
        REQUIRED_BOOTSTRAP_REJECTION_ASSERTIONS.len()
    );
    assert_eq!(
        bootstrap_falsification_artifact_ids().len(),
        REQUIRED_BOOTSTRAP_FALSIFICATION_ARTIFACTS.len()
    );
    assert_eq!(
        bootstrap_falsification_proof_ids().len(),
        REQUIRED_BOOTSTRAP_FALSIFICATION_PROOFS.len()
    );
    assert!(bootstrap_falsification_harnesses_bind_known_cases());
    assert!(bootstrap_rejection_assertions_bind_known_cases());
    assert!(bootstrap_falsification_artifacts_bind_paths());
    assert!(bootstrap_falsification_proofs_bind_registry());
    assert!(bootstrap_falsification_targets_all_required_domains());
    assert!(bootstrap_falsification_no_forbidden_descriptor_claims());
    assert!(bootstrap_falsification_registry_hash().starts_with("fnv1a128:"));
    assert!(bootstrap_falsification_registry_signature()
        .starts_with(LYRA_P02_BOOTSTRAP_FALSIFICATION_CARRIER));
}

#[test]
fn bootstrap_falsification_control_surfaces_align_with_package_frontier() {
    assert!(FRONTIER_LOCK.contains("current_task=P02-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P02-X04"));
    assert!(
        FRONTIER_LOCK.contains("truth_bound=receipts/p02/pass_0087_bootstrap_retirement_supersession.receipt")
    );
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P02-X05"));
    assert!(TRUTH_SNAPSHOT.contains("latest_finished_frontier=P02-X05"));
    assert!(TRUTH_SNAPSHOT.contains("P02-016"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P02-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P03"));
}
