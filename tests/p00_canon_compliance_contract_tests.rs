use lyra_phase0::p00::{
    P00_CANON_COMPLIANCE_CONTRACT, REQUIRED_CANON_RULES, REQUIRED_CANON_SOURCES,
    REQUIRED_CANON_VALIDATIONS, REQUIRED_ROADMAP_BINDINGS,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/canon_compliance.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const CLI: &str = include_str!("../src/bin/lyra-p00-canon-compliance-check.rs");
const LIB: &str = include_str!("../src/lib.rs");

#[test]
fn canon_compliance_contract_binds_runtime_header_rules_sources_and_validations() {
    assert!(CONTRACT.contains(P00_CANON_COMPLIANCE_CONTRACT));
    for rule in REQUIRED_CANON_RULES {
        assert!(
            CONTRACT.contains(&format!("rule:{rule}=")),
            "contract missing canon rule {rule}"
        );
    }
    for source in REQUIRED_CANON_SOURCES {
        assert!(
            CONTRACT.contains(&format!("source:{source}=")),
            "contract missing canon source {source}"
        );
    }
    for task in REQUIRED_ROADMAP_BINDINGS {
        assert!(
            CONTRACT.contains(&format!("roadmap:{task}=")),
            "contract missing roadmap binding {task}"
        );
    }
    for validation in REQUIRED_CANON_VALIDATIONS {
        assert!(
            CONTRACT.contains(&format!("validation:{validation}=")),
            "contract missing validation {validation}"
        );
    }
}

#[test]
fn canon_compliance_cli_and_exports_are_wired() {
    assert!(CLI.contains("validate_canon_compliance_surface"));
    assert!(LIB.contains("p00_canon_compliance"));
    assert!(LIB.contains("p00_canon_compliance_model"));
}

#[test]
fn control_plane_advances_to_p00_011_without_closing_phase() {
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
