use lyra_phase0::p00::{
    P00_CANONICAL_MODEL_CONTRACT, REQUIRED_CANONICAL_MODELS, REQUIRED_CANONICAL_MODEL_PROOFS,
    REQUIRED_CANONICAL_MODEL_RULES, REQUIRED_FIELD_BINDINGS, REQUIRED_MODEL_BINDINGS,
    REQUIRED_SCHEMA_BINDINGS,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/canonical_model.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const CLI: &str = include_str!("../src/bin/lyra-p00-canonical-model-check.rs");
const LIB: &str = include_str!("../src/lib.rs");

#[test]
fn canonical_model_contract_binds_runtime_header_and_required_models() {
    assert!(CONTRACT.contains(P00_CANONICAL_MODEL_CONTRACT));
    for rule in REQUIRED_CANONICAL_MODEL_RULES {
        assert!(
            CONTRACT.contains(&format!("required_rule={rule}")),
            "contract missing canonical model rule {rule}"
        );
    }
    for model in REQUIRED_CANONICAL_MODELS {
        assert!(
            CONTRACT.contains(&format!("required_model={model}")),
            "contract missing model {model}"
        );
    }
    for schema in REQUIRED_SCHEMA_BINDINGS {
        assert!(
            CONTRACT.contains(&format!("required_schema={schema}")),
            "contract missing schema {schema}"
        );
    }
    for field in REQUIRED_FIELD_BINDINGS {
        assert!(
            CONTRACT.contains(&format!("required_field={field}")),
            "contract missing field {field}"
        );
    }
    for binding in REQUIRED_MODEL_BINDINGS {
        assert!(
            CONTRACT.contains(&format!("required_binding={binding}")),
            "contract missing binding {binding}"
        );
    }
    for proof in REQUIRED_CANONICAL_MODEL_PROOFS {
        assert!(
            CONTRACT.contains(&format!("proof={proof}")),
            "contract missing proof {proof}"
        );
    }
}

#[test]
fn canonical_model_cli_and_exports_are_wired() {
    assert!(CLI.contains("validate_canonical_model_surface"));
    assert!(LIB.contains("p00_canonical_model_law"));
    assert!(LIB.contains("p00_canonical_model"));
}

#[test]
fn control_plane_advances_to_p00_014_without_closing_phase() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("status=closure_outputs_artifact_emitted"));
    assert!(TRUTH_SNAPSHOT.contains("closed=false"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
