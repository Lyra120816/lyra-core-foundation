use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceCommandReport {
    pub id: String,
    pub binary: String,
    pub target_count: usize,
    pub receipt_count: usize,
    pub command_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceWorkflowReport {
    pub id: String,
    pub order: String,
    pub command_count: usize,
    pub workflow_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceExampleReport {
    pub id: String,
    pub expected_verdict: String,
    pub example_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceProofReport {
    pub id: String,
    pub scope: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceArtifactReport {
    pub id: String,
    pub owner_root: String,
    pub artifact_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceSuiteReport {
    pub command_count: usize,
    pub workflow_count: usize,
    pub example_count: usize,
    pub proof_count: usize,
    pub artifact_count: usize,
    pub commands: Vec<SemanticInterfaceCommandReport>,
    pub workflows: Vec<SemanticInterfaceWorkflowReport>,
    pub examples: Vec<SemanticInterfaceExampleReport>,
    pub proofs: Vec<SemanticInterfaceProofReport>,
    pub artifacts: Vec<SemanticInterfaceArtifactReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_interface_suite_report(
    commands: &[(
        String,
        String,
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    workflows: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    examples: &[(String, String, Vec<String>, Vec<String>, String, String)],
    proofs: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    artifacts: &[(String, String, String, String, Vec<String>, String)],
) -> SemanticInterfaceSuiteReport {
    let mut command_reports: Vec<SemanticInterfaceCommandReport> = commands.iter().map(|item| {
        let preimage = format!("command:{}|binary:{}|surface:{}|input:{}|output:{}|receipts:{}|roles:{}|targets:{}|status:{}", item.0, item.1, item.2, item.3, item.4, sorted_join(&item.5), sorted_join(&item.6), sorted_join(&item.7), item.8);
        SemanticInterfaceCommandReport {
            id: item.0.clone(),
            binary: item.1.clone(),
            target_count: sorted_count(&item.7),
            receipt_count: sorted_count(&item.5),
            command_hash: stable_hash_label("lyra.p01.semantic_interface.command", &preimage),
        }
    }).collect();
    command_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut workflow_reports: Vec<SemanticInterfaceWorkflowReport> = workflows
        .iter()
        .map(|item| {
            let preimage = format!(
                "workflow:{}|order:{}|commands:{}|targets:{}|examples:{}|forbids:{}|status:{}",
                item.0,
                item.1,
                sorted_join(&item.2),
                sorted_join(&item.3),
                sorted_join(&item.4),
                sorted_join(&item.5),
                item.6
            );
            SemanticInterfaceWorkflowReport {
                id: item.0.clone(),
                order: item.1.clone(),
                command_count: sorted_count(&item.2),
                workflow_hash: stable_hash_label("lyra.p01.semantic_interface.workflow", &preimage),
            }
        })
        .collect();
    workflow_reports
        .sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));

    let mut example_reports: Vec<SemanticInterfaceExampleReport> = examples
        .iter()
        .map(|item| {
            let preimage = format!(
                "example:{}|path:{}|commands:{}|expected_receipts:{}|expected_verdict:{}|status:{}",
                item.0,
                item.1,
                sorted_join(&item.2),
                sorted_join(&item.3),
                item.4,
                item.5
            );
            SemanticInterfaceExampleReport {
                id: item.0.clone(),
                expected_verdict: item.4.clone(),
                example_hash: stable_hash_label("lyra.p01.semantic_interface.example", &preimage),
            }
        })
        .collect();
    example_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<SemanticInterfaceProofReport> = proofs.iter().map(|item| {
        let preimage = format!("proof:{}|scope:{}|commands:{}|workflows:{}|examples:{}|receipts:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), item.7);
        SemanticInterfaceProofReport { id: item.0.clone(), scope: item.1.clone(), proof_hash: stable_hash_label("lyra.p01.semantic_interface.proof", &preimage) }
    }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut artifact_reports: Vec<SemanticInterfaceArtifactReport> = artifacts
        .iter()
        .map(|item| {
            let preimage = format!(
                "artifact:{}|owner:{}|path:{}|kind:{}|commands:{}|status:{}",
                item.0,
                item.1,
                item.2,
                item.3,
                sorted_join(&item.4),
                item.5
            );
            SemanticInterfaceArtifactReport {
                id: item.0.clone(),
                owner_root: item.1.clone(),
                artifact_hash: stable_hash_label("lyra.p01.semantic_interface.artifact", &preimage),
            }
        })
        .collect();
    artifact_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_lines = Vec::new();
    for item in &command_reports {
        suite_lines.push(format!("command:{}|{}", item.id, item.command_hash));
    }
    for item in &workflow_reports {
        suite_lines.push(format!("workflow:{}|{}", item.id, item.workflow_hash));
    }
    for item in &example_reports {
        suite_lines.push(format!("example:{}|{}", item.id, item.example_hash));
    }
    for item in &proof_reports {
        suite_lines.push(format!("proof:{}|{}", item.id, item.proof_hash));
    }
    for item in &artifact_reports {
        suite_lines.push(format!("artifact:{}|{}", item.id, item.artifact_hash));
    }
    suite_lines.sort();

    SemanticInterfaceSuiteReport {
        command_count: command_reports.len(),
        workflow_count: workflow_reports.len(),
        example_count: example_reports.len(),
        proof_count: proof_reports.len(),
        artifact_count: artifact_reports.len(),
        commands: command_reports,
        workflows: workflow_reports,
        examples: example_reports,
        proofs: proof_reports,
        artifacts: artifact_reports,
        suite_hash: stable_hash_label("lyra.p01.semantic_interface.suite", &suite_lines.join("\n")),
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
