use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorCommandReport {
    pub id: String,
    pub binary: String,
    pub target_count: usize,
    pub receipt_count: usize,
    pub command_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorWorkflowReport {
    pub id: String,
    pub order: String,
    pub command_count: usize,
    pub workflow_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorExampleReport {
    pub id: String,
    pub expected_verdict: String,
    pub example_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorAcceptanceGateReport {
    pub id: String,
    pub workflow: String,
    pub decision: String,
    pub gate_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorProofReport {
    pub id: String,
    pub scope: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorArtifactReport {
    pub id: String,
    pub owner_root: String,
    pub artifact_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorInterfaceSuiteReport {
    pub command_count: usize,
    pub workflow_count: usize,
    pub example_count: usize,
    pub gate_count: usize,
    pub proof_count: usize,
    pub artifact_count: usize,
    pub commands: Vec<BootstrapOperatorCommandReport>,
    pub workflows: Vec<BootstrapOperatorWorkflowReport>,
    pub examples: Vec<BootstrapOperatorExampleReport>,
    pub gates: Vec<BootstrapOperatorAcceptanceGateReport>,
    pub proofs: Vec<BootstrapOperatorProofReport>,
    pub artifacts: Vec<BootstrapOperatorArtifactReport>,
    pub suite_hash: String,
}

pub fn deterministic_bootstrap_operator_interface_suite_report(
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
    gates: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        String,
        Vec<String>,
        String,
    )],
    proofs: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    artifacts: &[(String, String, String, String, Vec<String>, String)],
) -> BootstrapOperatorInterfaceSuiteReport {
    let mut command_reports: Vec<BootstrapOperatorCommandReport> = commands.iter().map(|item| {
            let preimage = format!("command:{}|binary:{}|surface:{}|input:{}|output:{}|receipts:{}|roles:{}|targets:{}|status:{}", item.0, item.1, item.2, item.3, item.4, sorted_join(&item.5), sorted_join(&item.6), sorted_join(&item.7), item.8);
            BootstrapOperatorCommandReport { id: item.0.clone(), binary: item.1.clone(), target_count: sorted_count(&item.7), receipt_count: sorted_count(&item.5), command_hash: stable_hash_label("lyra.p02.bootstrap_operator_interface.command", &preimage) }
        }).collect();
    command_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut workflow_reports: Vec<BootstrapOperatorWorkflowReport> = workflows
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
            BootstrapOperatorWorkflowReport {
                id: item.0.clone(),
                order: item.1.clone(),
                command_count: sorted_count(&item.2),
                workflow_hash: stable_hash_label(
                    "lyra.p02.bootstrap_operator_interface.workflow",
                    &preimage,
                ),
            }
        })
        .collect();
    workflow_reports
        .sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));

    let mut example_reports: Vec<BootstrapOperatorExampleReport> = examples
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
            BootstrapOperatorExampleReport {
                id: item.0.clone(),
                expected_verdict: item.4.clone(),
                example_hash: stable_hash_label(
                    "lyra.p02.bootstrap_operator_interface.example",
                    &preimage,
                ),
            }
        })
        .collect();
    example_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut gate_reports: Vec<BootstrapOperatorAcceptanceGateReport> = gates.iter().map(|item| {
            let preimage = format!("gate:{}|workflow:{}|required_receipts:{}|required_examples:{}|decision:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), item.4, sorted_join(&item.5), item.6);
            BootstrapOperatorAcceptanceGateReport { id: item.0.clone(), workflow: item.1.clone(), decision: item.4.clone(), gate_hash: stable_hash_label("lyra.p02.bootstrap_operator_interface.gate", &preimage) }
        }).collect();
    gate_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<BootstrapOperatorProofReport> = proofs.iter().map(|item| {
            let preimage = format!("proof:{}|scope:{}|commands:{}|workflows:{}|examples:{}|gates:{}|receipts:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), sorted_join(&item.7), item.8);
            BootstrapOperatorProofReport { id: item.0.clone(), scope: item.1.clone(), proof_hash: stable_hash_label("lyra.p02.bootstrap_operator_interface.proof", &preimage) }
        }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut artifact_reports: Vec<BootstrapOperatorArtifactReport> = artifacts
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
            BootstrapOperatorArtifactReport {
                id: item.0.clone(),
                owner_root: item.1.clone(),
                artifact_hash: stable_hash_label(
                    "lyra.p02.bootstrap_operator_interface.artifact",
                    &preimage,
                ),
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
    for item in &gate_reports {
        suite_lines.push(format!("gate:{}|{}", item.id, item.gate_hash));
    }
    for item in &proof_reports {
        suite_lines.push(format!("proof:{}|{}", item.id, item.proof_hash));
    }
    for item in &artifact_reports {
        suite_lines.push(format!("artifact:{}|{}", item.id, item.artifact_hash));
    }
    suite_lines.sort();

    BootstrapOperatorInterfaceSuiteReport {
        command_count: command_reports.len(),
        workflow_count: workflow_reports.len(),
        example_count: example_reports.len(),
        gate_count: gate_reports.len(),
        proof_count: proof_reports.len(),
        artifact_count: artifact_reports.len(),
        commands: command_reports,
        workflows: workflow_reports,
        examples: example_reports,
        gates: gate_reports,
        proofs: proof_reports,
        artifacts: artifact_reports,
        suite_hash: stable_hash_label(
            "lyra.p02.bootstrap_operator_interface.suite",
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
