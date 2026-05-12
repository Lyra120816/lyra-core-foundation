use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineUnitReport {
    pub id: String,
    pub unit_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineTransitionReport {
    pub id: String,
    pub transition_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineArtifactReport {
    pub id: String,
    pub artifact_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineProofReport {
    pub id: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineSuiteReport {
    pub unit_count: usize,
    pub transition_count: usize,
    pub artifact_count: usize,
    pub proof_count: usize,
    pub lyralang_owned_count: usize,
    pub k0_owned_count: usize,
    pub interface_owned_count: usize,
    pub ops_owned_count: usize,
    pub unit_reports: Vec<SemanticCoreEngineUnitReport>,
    pub transition_reports: Vec<SemanticCoreEngineTransitionReport>,
    pub artifact_reports: Vec<SemanticCoreEngineArtifactReport>,
    pub proof_reports: Vec<SemanticCoreEngineProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_core_engine_suite_report(
    units: &[(String, String, String, String, String, String, String)],
    transitions: &[(String, String, String, String, String, String)],
    artifacts: &[(String, String, String, String, String)],
    proofs: &[(
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
        String,
        String,
        String,
    )],
) -> SemanticCoreEngineSuiteReport {
    let mut lyralang_owned_count = 0usize;
    let mut k0_owned_count = 0usize;
    let mut interface_owned_count = 0usize;
    let mut ops_owned_count = 0usize;

    let mut unit_reports: Vec<SemanticCoreEngineUnitReport> = units
        .iter()
        .map(|item| {
            if item.1 == "lyralang" {
                lyralang_owned_count += 1;
            }
            if item.1 == "k0" {
                k0_owned_count += 1;
            }
            if item.1 == "interfaces" {
                interface_owned_count += 1;
            }
            if item.1 == "ops" {
                ops_owned_count += 1;
            }
            let preimage = format!(
                "unit:{}|owner:{}|input:{}|output:{}|order:{}|law:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            SemanticCoreEngineUnitReport {
                id: item.0.clone(),
                unit_hash: stable_hash_label("lyra.p01.semantic_core_engine.unit", &preimage),
            }
        })
        .collect();
    unit_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut transition_reports: Vec<SemanticCoreEngineTransitionReport> = transitions
        .iter()
        .map(|item| {
            let preimage = format!(
                "transition:{}|from:{}|to:{}|law:{}|carry:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            SemanticCoreEngineTransitionReport {
                id: item.0.clone(),
                transition_hash: stable_hash_label(
                    "lyra.p01.semantic_core_engine.transition",
                    &preimage,
                ),
            }
        })
        .collect();
    transition_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut artifact_reports: Vec<SemanticCoreEngineArtifactReport> = artifacts
        .iter()
        .map(|item| {
            let preimage = format!(
                "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            SemanticCoreEngineArtifactReport {
                id: item.0.clone(),
                artifact_hash: stable_hash_label(
                    "lyra.p01.semantic_core_engine.artifact",
                    &preimage,
                ),
            }
        })
        .collect();
    artifact_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<SemanticCoreEngineProofReport> = proofs.iter().map(|item| {
        let preimage = format!("proof:{}|units:{}|transitions:{}|artifacts:{}|fixture:{}|golden:{}|receipt:{}|status:{}", item.0, sorted_join(&item.1), sorted_join(&item.2), sorted_join(&item.3), item.4, item.5, item.6, item.7);
        SemanticCoreEngineProofReport { id: item.0.clone(), proof_hash: stable_hash_label("lyra.p01.semantic_core_engine.proof", &preimage) }
    }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let preimage = format!(
        "units:{}|transitions:{}|artifacts:{}|proofs:{}|unit_hashes:{}|transition_hashes:{}|artifact_hashes:{}|proof_hashes:{}",
        unit_reports.len(), transition_reports.len(), artifact_reports.len(), proof_reports.len(),
        unit_reports.iter().map(|item| item.unit_hash.as_str()).collect::<Vec<_>>().join(","),
        transition_reports.iter().map(|item| item.transition_hash.as_str()).collect::<Vec<_>>().join(","),
        artifact_reports.iter().map(|item| item.artifact_hash.as_str()).collect::<Vec<_>>().join(","),
        proof_reports.iter().map(|item| item.proof_hash.as_str()).collect::<Vec<_>>().join(","),
    );

    SemanticCoreEngineSuiteReport {
        unit_count: unit_reports.len(),
        transition_count: transition_reports.len(),
        artifact_count: artifact_reports.len(),
        proof_count: proof_reports.len(),
        lyralang_owned_count,
        k0_owned_count,
        interface_owned_count,
        ops_owned_count,
        unit_reports,
        transition_reports,
        artifact_reports,
        proof_reports,
        suite_hash: stable_hash_label("lyra.p01.semantic_core_engine.suite", &preimage),
    }
}

fn sorted_join(values: &[String]) -> String {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values.join(",")
}
