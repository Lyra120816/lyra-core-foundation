use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationCaseReport {
    pub id: String,
    pub target_domain: String,
    pub expected_error: String,
    pub case_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationHarnessReport {
    pub id: String,
    pub case_count: usize,
    pub harness_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRejectionAssertionReport {
    pub id: String,
    pub case_id: String,
    pub assertion_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationArtifactReport {
    pub id: String,
    pub artifact_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationProofReport {
    pub id: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationSuiteReport {
    pub case_count: usize,
    pub harness_count: usize,
    pub assertion_count: usize,
    pub artifact_count: usize,
    pub proof_count: usize,
    pub bootstrap_trust_case_count: usize,
    pub seed_runtime_law_case_count: usize,
    pub host_extinction_case_count: usize,
    pub foreign_boundary_case_count: usize,
    pub operator_handoff_case_count: usize,
    pub emergency_fallback_case_count: usize,
    pub receipt_commit_case_count: usize,
    pub case_reports: Vec<BootstrapFalsificationCaseReport>,
    pub harness_reports: Vec<BootstrapFalsificationHarnessReport>,
    pub assertion_reports: Vec<BootstrapRejectionAssertionReport>,
    pub artifact_reports: Vec<BootstrapFalsificationArtifactReport>,
    pub proof_reports: Vec<BootstrapFalsificationProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_bootstrap_falsification_suite_report(
    cases: &[(String, String, String, String, String, String, String)],
    harnesses: &[(String, String, Vec<String>, String, String, String)],
    assertions: &[(String, String, String, String, Vec<String>, String)],
    artifacts: &[(String, String, String, String, String)],
    proofs: &[(
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
        String,
    )],
) -> BootstrapFalsificationSuiteReport {
    let mut bootstrap_trust_case_count = 0usize;
    let mut seed_runtime_law_case_count = 0usize;
    let mut host_extinction_case_count = 0usize;
    let mut foreign_boundary_case_count = 0usize;
    let mut operator_handoff_case_count = 0usize;
    let mut emergency_fallback_case_count = 0usize;
    let mut receipt_commit_case_count = 0usize;

    let mut case_reports: Vec<BootstrapFalsificationCaseReport> = cases
        .iter()
        .map(|item| {
            match item.1.as_str() {
                "bootstrap_trust" => bootstrap_trust_case_count += 1,
                "seed_runtime_law" => seed_runtime_law_case_count += 1,
                "host_extinction" => host_extinction_case_count += 1,
                "foreign_boundary" => foreign_boundary_case_count += 1,
                "operator_handoff" => operator_handoff_case_count += 1,
                "emergency_fallback" => emergency_fallback_case_count += 1,
                "receipt_commit" => receipt_commit_case_count += 1,
                _ => {}
            }
            let preimage = format!(
                "case:{}|target:{}|validator:{}|mutation:{}|expected:{}|fixture:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            BootstrapFalsificationCaseReport {
                id: item.0.clone(),
                target_domain: item.1.clone(),
                expected_error: item.4.clone(),
                case_hash: stable_hash_label("lyra.p02.bootstrap_falsification.case", &preimage),
            }
        })
        .collect();
    case_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut harness_reports: Vec<BootstrapFalsificationHarnessReport> = harnesses
        .iter()
        .map(|item| {
            let preimage = format!(
                "harness:{}|runner:{}|cases:{}|mode:{}|receipt:{}|status:{}",
                item.0,
                item.1,
                sorted_join(&item.2),
                item.3,
                item.4,
                item.5
            );
            BootstrapFalsificationHarnessReport {
                id: item.0.clone(),
                case_count: sorted_count(&item.2),
                harness_hash: stable_hash_label(
                    "lyra.p02.bootstrap_falsification.harness",
                    &preimage,
                ),
            }
        })
        .collect();
    harness_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut assertion_reports: Vec<BootstrapRejectionAssertionReport> = assertions
        .iter()
        .map(|item| {
            let preimage = format!(
                "assertion:{}|case:{}|expected:{}|surface:{}|forbids:{}|status:{}",
                item.0,
                item.1,
                item.2,
                item.3,
                sorted_join(&item.4),
                item.5
            );
            BootstrapRejectionAssertionReport {
                id: item.0.clone(),
                case_id: item.1.clone(),
                assertion_hash: stable_hash_label(
                    "lyra.p02.bootstrap_falsification.assertion",
                    &preimage,
                ),
            }
        })
        .collect();
    assertion_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut artifact_reports: Vec<BootstrapFalsificationArtifactReport> = artifacts
        .iter()
        .map(|item| {
            let preimage = format!(
                "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            BootstrapFalsificationArtifactReport {
                id: item.0.clone(),
                artifact_hash: stable_hash_label(
                    "lyra.p02.bootstrap_falsification.artifact",
                    &preimage,
                ),
            }
        })
        .collect();
    artifact_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<BootstrapFalsificationProofReport> = proofs
        .iter()
        .map(|item| {
            let preimage = format!(
                "proof:{}|cases:{}|harnesses:{}|assertions:{}|artifacts:{}|receipt:{}|status:{}",
                item.0,
                sorted_join(&item.1),
                sorted_join(&item.2),
                sorted_join(&item.3),
                sorted_join(&item.4),
                item.5,
                item.6
            );
            BootstrapFalsificationProofReport {
                id: item.0.clone(),
                proof_hash: stable_hash_label("lyra.p02.bootstrap_falsification.proof", &preimage),
            }
        })
        .collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_lines = Vec::new();
    for item in &case_reports {
        suite_lines.push(format!("case:{}|{}", item.id, item.case_hash));
    }
    for item in &harness_reports {
        suite_lines.push(format!("harness:{}|{}", item.id, item.harness_hash));
    }
    for item in &assertion_reports {
        suite_lines.push(format!("assertion:{}|{}", item.id, item.assertion_hash));
    }
    for item in &artifact_reports {
        suite_lines.push(format!("artifact:{}|{}", item.id, item.artifact_hash));
    }
    for item in &proof_reports {
        suite_lines.push(format!("proof:{}|{}", item.id, item.proof_hash));
    }
    suite_lines.sort();

    BootstrapFalsificationSuiteReport {
        case_count: case_reports.len(),
        harness_count: harness_reports.len(),
        assertion_count: assertion_reports.len(),
        artifact_count: artifact_reports.len(),
        proof_count: proof_reports.len(),
        bootstrap_trust_case_count,
        seed_runtime_law_case_count,
        host_extinction_case_count,
        foreign_boundary_case_count,
        operator_handoff_case_count,
        emergency_fallback_case_count,
        receipt_commit_case_count,
        case_reports,
        harness_reports,
        assertion_reports,
        artifact_reports,
        proof_reports,
        suite_hash: stable_hash_label(
            "lyra.p02.bootstrap_falsification.suite",
            &suite_lines.join("\n"),
        ),
    }
}

fn sorted_join(values: &[String]) -> String {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values.join(",")
}

fn sorted_count(values: &[String]) -> usize {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values.len()
}
