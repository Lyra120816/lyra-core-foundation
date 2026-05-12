use lyra_phase0::p00::{
    P00_CONTROL_SURFACES_CONTRACT, REQUIRED_CONTROL_RULES, REQUIRED_CONTROL_SURFACES,
    REQUIRED_PASS_TEMPLATE_FIELDS,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/control_surfaces.v1.lyra");
const PASS_TEMPLATE: &str = include_str!("../ops/p00/control/pass_template.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn control_contract_binds_runtime_header_rules_and_surface_ids() {
    assert!(CONTRACT.contains(P00_CONTROL_SURFACES_CONTRACT));
    for rule in REQUIRED_CONTROL_RULES {
        assert!(CONTRACT.contains(rule), "contract missing rule {rule}");
    }
    for surface in REQUIRED_CONTROL_SURFACES {
        assert!(
            CONTRACT.contains(&format!("surface:{surface}")),
            "contract missing surface {surface}"
        );
    }
}

#[test]
fn pass_template_binds_required_operator_report_fields() {
    assert!(PASS_TEMPLATE.contains("LYRA-P00-PASS-TEMPLATE v1"));
    for field in REQUIRED_PASS_TEMPLATE_FIELDS {
        assert!(
            PASS_TEMPLATE.contains(field),
            "pass template missing field {field}"
        );
    }
    assert!(PASS_TEMPLATE.contains("forbidden:global_complete"));
    assert!(PASS_TEMPLATE.contains("forbidden:placeholder"));
    assert!(PASS_TEMPLATE.contains("forbidden:no_artifact"));
}

#[test]
fn updated_control_files_bind_current_frontier_and_receipt() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("closed=false"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
