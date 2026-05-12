use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationCaseReport {
    pub id: String,
    pub target_domain: String,
    pub expected_error: String,
    pub case_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationHarnessReport {
    pub id: String,
    pub case_count: usize,
    pub harness_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRejectionAssertionReport {
    pub id: String,
    pub case_id: String,
    pub assertion_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationArtifactReport {
    pub id: String,
    pub artifact_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationProofReport {
    pub id: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationSuiteReport {
    pub case_count: usize,
    pub harness_count: usize,
    pub assertion_count: usize,
    pub artifact_count: usize,
    pub proof_count: usize,
    pub canonical_symbol_case_count: usize,
    pub semantic_atom_case_count: usize,
    pub core_ir_case_count: usize,
    pub case_reports: Vec<SemanticFalsificationCaseReport>,
    pub harness_reports: Vec<SemanticFalsificationHarnessReport>,
    pub assertion_reports: Vec<SemanticRejectionAssertionReport>,
    pub artifact_reports: Vec<SemanticFalsificationArtifactReport>,
    pub proof_reports: Vec<SemanticFalsificationProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_falsification_suite_report(
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
) -> SemanticFalsificationSuiteReport {
    let mut canonical_symbol_case_count = 0usize;
    let mut semantic_atom_case_count = 0usize;
    let mut core_ir_case_count = 0usize;

    let mut case_reports: Vec<SemanticFalsificationCaseReport> = cases
        .iter()
        .map(|item| {
            if item.1 == "canonical_symbols" {
                canonical_symbol_case_count += 1;
            }
            if item.1 == "semantic_atoms" {
                semantic_atom_case_count += 1;
            }
            if item.1 == "core_ir" {
                core_ir_case_count += 1;
            }
            let preimage = format!(
                "case:{}|target:{}|validator:{}|mutation:{}|expected:{}|fixture:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            SemanticFalsificationCaseReport {
                id: item.0.clone(),
                target_domain: item.1.clone(),
                expected_error: item.4.clone(),
                case_hash: stable_hash_label("lyra.p01.semantic_falsification.case", &preimage),
            }
        })
        .collect();
    case_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut harness_reports: Vec<SemanticFalsificationHarnessReport> = harnesses
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
            SemanticFalsificationHarnessReport {
                id: item.0.clone(),
                case_count: sorted_count(&item.2),
                harness_hash: stable_hash_label(
                    "lyra.p01.semantic_falsification.harness",
                    &preimage,
                ),
            }
        })
        .collect();
    harness_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut assertion_reports: Vec<SemanticRejectionAssertionReport> = assertions
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
            SemanticRejectionAssertionReport {
                id: item.0.clone(),
                case_id: item.1.clone(),
                assertion_hash: stable_hash_label(
                    "lyra.p01.semantic_falsification.assertion",
                    &preimage,
                ),
            }
        })
        .collect();
    assertion_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut artifact_reports: Vec<SemanticFalsificationArtifactReport> = artifacts
        .iter()
        .map(|item| {
            let preimage = format!(
                "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            SemanticFalsificationArtifactReport {
                id: item.0.clone(),
                artifact_hash: stable_hash_label(
                    "lyra.p01.semantic_falsification.artifact",
                    &preimage,
                ),
            }
        })
        .collect();
    artifact_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<SemanticFalsificationProofReport> = proofs
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
            SemanticFalsificationProofReport {
                id: item.0.clone(),
                proof_hash: stable_hash_label("lyra.p01.semantic_falsification.proof", &preimage),
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

    SemanticFalsificationSuiteReport {
        case_count: case_reports.len(),
        harness_count: harness_reports.len(),
        assertion_count: assertion_reports.len(),
        artifact_count: artifact_reports.len(),
        proof_count: proof_reports.len(),
        canonical_symbol_case_count,
        semantic_atom_case_count,
        core_ir_case_count,
        case_reports,
        harness_reports,
        assertion_reports,
        artifact_reports,
        proof_reports,
        suite_hash: stable_hash_label(
            "lyra.p01.semantic_falsification.suite",
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
