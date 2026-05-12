use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_BOOTSTRAP_EVIDENCE_EMISSION_CARRIER: &str =
    "LYRA-P02-BOOTSTRAP-EVIDENCE-EMISSION-CARRIER v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapEvidenceFixtureDescriptor {
    pub id: &'static str,
    pub fixture_kind: &'static str,
    pub path: &'static str,
    pub expected_verdict: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapTargetMatrixReportDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub target_class: &'static str,
    pub proof_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapChallengeReceiptDescriptor {
    pub id: &'static str,
    pub suite_id: &'static str,
    pub surface_ref: &'static str,
    pub truth_effect: &'static str,
}

pub const LYRALANG_BOOTSTRAP_EVIDENCE_FIXTURES: &[BootstrapEvidenceFixtureDescriptor] = &[
    BootstrapEvidenceFixtureDescriptor { id: "fixture_extinction_positive", fixture_kind: "extinction_ledger_positive", path: "fixtures/p02/bootstrap_extinction_inputs/valid_bootstrap_extinction_ledger.lyra", expected_verdict: "accepted" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_extinction_negative_missing_rule", fixture_kind: "extinction_ledger_negative", path: "fixtures/p02/bootstrap_extinction_inputs/invalid_missing_rule.lyra", expected_verdict: "rejected" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_extinction_negative_duplicate_entry", fixture_kind: "extinction_ledger_negative", path: "fixtures/p02/bootstrap_extinction_inputs/invalid_duplicate_entry.lyra", expected_verdict: "rejected" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_target_matrix_positive", fixture_kind: "target_matrix_positive", path: "fixtures/p02/bootstrap_target_matrix_inputs/valid_bootstrap_target_matrix.lyra", expected_verdict: "accepted" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_target_matrix_negative_missing_target", fixture_kind: "target_matrix_negative", path: "fixtures/p02/bootstrap_target_matrix_inputs/invalid_missing_target.lyra", expected_verdict: "rejected" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_target_matrix_negative_bad_family", fixture_kind: "target_matrix_negative", path: "fixtures/p02/bootstrap_target_matrix_inputs/invalid_bad_proof_family.lyra", expected_verdict: "rejected" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_challenge_positive", fixture_kind: "bootstrap_challenge_positive", path: "fixtures/p02/host_boundary_challenge_inputs/valid_host_boundary_challenge_suites.lyra", expected_verdict: "accepted" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_challenge_negative_unreceipted", fixture_kind: "bootstrap_challenge_negative", path: "fixtures/p02/host_boundary_challenge_inputs/invalid_unreceipted_suite.lyra", expected_verdict: "rejected" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_challenge_negative_foreign_ownership", fixture_kind: "bootstrap_challenge_negative", path: "fixtures/p02/host_boundary_challenge_inputs/invalid_foreign_ownership.lyra", expected_verdict: "rejected" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_replacement_positive", fixture_kind: "seed_replacement_positive", path: "fixtures/p02/seed_runtime_replacement_milestones_inputs/valid_seed_runtime_replacement_milestones.lyra", expected_verdict: "accepted" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_replacement_negative_missing_handoff", fixture_kind: "seed_replacement_negative", path: "fixtures/p02/seed_runtime_replacement_milestones_inputs/invalid_missing_handoff.lyra", expected_verdict: "rejected" },
    BootstrapEvidenceFixtureDescriptor { id: "fixture_emission_negative_malformed", fixture_kind: "evidence_emission_malformed", path: "fixtures/p02/bootstrap_evidence_emission_inputs/invalid_duplicate_report.lyra", expected_verdict: "rejected" },
];

pub const LYRALANG_BOOTSTRAP_TARGET_REPORTS: &[BootstrapTargetMatrixReportDescriptor] = &[
    BootstrapTargetMatrixReportDescriptor {
        id: "report_linux_x86_64",
        target_id: "target_linux_x86_64",
        target_class: "linux",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_linux_aarch64",
        target_id: "target_linux_aarch64",
        target_class: "linux",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_windows_x86_64",
        target_id: "target_windows_x86_64",
        target_class: "windows",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_windows_aarch64",
        target_id: "target_windows_aarch64",
        target_class: "windows",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_android_aarch64",
        target_id: "target_android_aarch64",
        target_class: "mobile",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_ios_aarch64",
        target_id: "target_ios_aarch64",
        target_class: "mobile",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_wasm32_wasi",
        target_id: "target_wasm32_wasi",
        target_class: "wasm",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_wasm32_unknown",
        target_id: "target_wasm32_unknown",
        target_class: "wasm",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_baremetal_x86_64",
        target_id: "target_baremetal_x86_64",
        target_class: "baremetal",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_baremetal_aarch64",
        target_id: "target_baremetal_aarch64",
        target_class: "baremetal",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_baremetal_riscv64",
        target_id: "target_baremetal_riscv64",
        target_class: "baremetal",
        proof_count: 5,
    },
    BootstrapTargetMatrixReportDescriptor {
        id: "report_host_tooling_quarantine",
        target_id: "target_host_tooling_quarantine",
        target_class: "other",
        proof_count: 5,
    },
];

pub const LYRALANG_BOOTSTRAP_CHALLENGE_RECEIPTS: &[BootstrapChallengeReceiptDescriptor] = &[
    BootstrapChallengeReceiptDescriptor {
        id: "challenge_receipt_no_ambient_network_import",
        suite_id: "suite_no_ambient_network_import",
        surface_ref: "surface:git_repository_transport",
        truth_effect: "none_without_local_replay",
    },
    BootstrapChallengeReceiptDescriptor {
        id: "challenge_receipt_no_ambient_time_truth",
        suite_id: "suite_no_ambient_time_truth",
        surface_ref: "surface:external_wall_clock",
        truth_effect: "none_without_local_replay",
    },
    BootstrapChallengeReceiptDescriptor {
        id: "challenge_receipt_no_hidden_randomness_truth",
        suite_id: "suite_no_hidden_randomness_truth",
        surface_ref: "surface:host_operating_system",
        truth_effect: "none_without_local_replay",
    },
    BootstrapChallengeReceiptDescriptor {
        id: "challenge_receipt_no_unledgered_host_surface",
        suite_id: "suite_no_unledgered_host_surface",
        surface_ref: "surface:host_filesystem",
        truth_effect: "none_without_local_replay",
    },
    BootstrapChallengeReceiptDescriptor {
        id: "challenge_receipt_no_foreign_semantic_ownership",
        suite_id: "suite_no_foreign_semantic_ownership",
        surface_ref: "surface:rust_bootstrap_compiler",
        truth_effect: "none_without_local_replay",
    },
    BootstrapChallengeReceiptDescriptor {
        id: "challenge_receipt_operator_truth_containment",
        suite_id: "suite_operator_truth_containment",
        surface_ref: "surface:operator_shell_terminal",
        truth_effect: "none_without_local_replay",
    },
    BootstrapChallengeReceiptDescriptor {
        id: "challenge_receipt_foreign_runtime_quarantine",
        suite_id: "suite_foreign_runtime_quarantine",
        surface_ref: "surface:rust_std_runtime",
        truth_effect: "none_without_local_replay",
    },
];

pub fn bootstrap_evidence_fixture_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_EVIDENCE_FIXTURES
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_target_report_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_TARGET_REPORTS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_challenge_receipt_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CHALLENGE_RECEIPTS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_evidence_fixture_descriptor(
    id: &str,
) -> Option<&'static BootstrapEvidenceFixtureDescriptor> {
    LYRALANG_BOOTSTRAP_EVIDENCE_FIXTURES
        .iter()
        .find(|x| x.id == id)
}
pub fn bootstrap_target_report_descriptor(
    id: &str,
) -> Option<&'static BootstrapTargetMatrixReportDescriptor> {
    LYRALANG_BOOTSTRAP_TARGET_REPORTS
        .iter()
        .find(|x| x.id == id)
}
pub fn bootstrap_challenge_receipt_descriptor(
    id: &str,
) -> Option<&'static BootstrapChallengeReceiptDescriptor> {
    LYRALANG_BOOTSTRAP_CHALLENGE_RECEIPTS
        .iter()
        .find(|x| x.id == id)
}
pub fn bootstrap_evidence_fixtures_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_EVIDENCE_FIXTURES
        .iter()
        .all(|x| x.path.starts_with("fixtures/p02/"))
}
pub fn bootstrap_target_reports_all_pending_proof_counts() -> bool {
    LYRALANG_BOOTSTRAP_TARGET_REPORTS
        .iter()
        .all(|x| x.proof_count == 5)
}
pub fn bootstrap_challenge_receipts_truth_neutral() -> bool {
    LYRALANG_BOOTSTRAP_CHALLENGE_RECEIPTS
        .iter()
        .all(|x| x.truth_effect == "none_without_local_replay")
}

pub fn bootstrap_evidence_registry_hash() -> String {
    let fixture_part = LYRALANG_BOOTSTRAP_EVIDENCE_FIXTURES
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}",
                x.id, x.fixture_kind, x.path, x.expected_verdict
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let report_part = LYRALANG_BOOTSTRAP_TARGET_REPORTS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}",
                x.id, x.target_id, x.target_class, x.proof_count
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let challenge_part = LYRALANG_BOOTSTRAP_CHALLENGE_RECEIPTS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}",
                x.id, x.suite_id, x.surface_ref, x.truth_effect
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    stable_hash_label(
        "lyra.p02.bootstrap_evidence.registry",
        &format!("{fixture_part}|{report_part}|{challenge_part}"),
    )
}

pub fn bootstrap_evidence_registry_signature() -> String {
    format!(
        "{}:{}:{}:{}",
        LYRA_P02_BOOTSTRAP_EVIDENCE_EMISSION_CARRIER,
        LYRALANG_BOOTSTRAP_EVIDENCE_FIXTURES.len(),
        LYRALANG_BOOTSTRAP_TARGET_REPORTS.len(),
        LYRALANG_BOOTSTRAP_CHALLENGE_RECEIPTS.len()
    )
}
