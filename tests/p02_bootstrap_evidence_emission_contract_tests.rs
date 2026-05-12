const CONTRACT: &str =
    include_str!("../interfaces/p02/contracts/bootstrap_evidence_emission.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p02/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p02/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p02/control/blocker_index.v1.lyra");
const EVIDENCE: &str = include_str!("../ops/p02/evidence/bootstrap_evidence_emission.v1.lyra");
const EXTINCTION_FIXTURE_MANIFEST: &str =
    include_str!("../ops/p02/evidence/extinction_ledger_fixture_manifest.v1.lyra");
const TARGET_REPORTS: &str =
    include_str!("../ops/p02/evidence/bootstrap_target_matrix_reports.v1.lyra");
const OPERATOR_SURFACE: &str =
    include_str!("../shells/p02/bootstrap_evidence_emission_operator_surface.lyra");
const PRODUCT_SURFACE: &str =
    include_str!("../products/p02/bootstrap_evidence_emission_inspection_surface.lyra");

#[test]
fn evidence_emission_contract_names_p02_010() {
    assert!(CONTRACT.contains("LYRA-P02-BOOTSTRAP-EVIDENCE-EMISSION-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P02-010"));
    for target in [
        "target:target_linux_x86_64",
        "target:target_windows_aarch64",
        "target:target_android_aarch64",
        "target:target_wasm32_wasi",
        "target:target_baremetal_riscv64",
        "target:target_host_tooling_quarantine",
    ] {
        assert!(CONTRACT.contains(target), "contract missing {target}");
    }
    assert!(CONTRACT.contains("challenge_suite:suite_no_ambient_network_import"));
    assert!(CONTRACT.contains("challenge_suite:suite_foreign_runtime_quarantine"));
}

#[test]
fn control_plane_below_phase_closure_at_current_frontier() {
    assert!(CONTROL_FRONTIER.contains("current_task=P02-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P02-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P03"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P02-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_primary_tasks=none"));
    assert!(BLOCKER_INDEX.contains("blocker:p02_primary_task_remainder"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P03"));
}

#[test]
fn emitted_evidence_binds_all_required_report_and_receipt_families() {
    for needle in [
        "fixture:fixture_extinction_positive",
        "fixture:fixture_target_matrix_positive",
        "fixture:fixture_challenge_positive",
        "fixture:fixture_emission_negative_malformed",
        "target_report:report_linux_x86_64",
        "target_report:report_baremetal_riscv64",
        "challenge_receipt:challenge_receipt_no_ambient_network_import",
        "challenge_receipt:challenge_receipt_foreign_runtime_quarantine",
        "truth_effect:none_without_local_replay",
        "receipt:receipt_bootstrap_evidence_emission",
    ] {
        assert!(
            EVIDENCE.contains(needle),
            "evidence surface missing {needle}"
        );
    }
    assert!(OPERATOR_SURFACE.contains("lyra-p02-bootstrap-evidence-emission-check"));
    assert!(PRODUCT_SURFACE.contains("fixture_count=12"));
    assert!(PRODUCT_SURFACE.contains("target_report_count=12"));
    assert!(PRODUCT_SURFACE.contains("challenge_receipt_count=7"));
    assert!(EXTINCTION_FIXTURE_MANIFEST.contains("LYRA-P02-EXTINCTION-LEDGER-FIXTURE-MANIFEST v1"));
    assert!(TARGET_REPORTS.contains("LYRA-P02-BOOTSTRAP-TARGET-MATRIX-REPORTS v1"));
    assert!(TARGET_REPORTS.contains("target_report:report_host_tooling_quarantine"));
    assert!(!EVIDENCE.contains("global_closure=true"));
}
