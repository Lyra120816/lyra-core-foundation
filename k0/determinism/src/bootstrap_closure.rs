use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapClosureTaskReport {
    pub id: String,
    pub scope: String,
    pub receipt_count: usize,
    pub command_count: usize,
    pub evidence_count: usize,
    pub status: String,
    pub task_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapClosureOutputGateReport {
    pub id: String,
    pub output_kind: String,
    pub path: String,
    pub dependency_count: usize,
    pub receipt_count: usize,
    pub status: String,
    pub output_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapClosureProofReport {
    pub id: String,
    pub scope: String,
    pub task_count: usize,
    pub output_count: usize,
    pub receipt_count: usize,
    pub command_count: usize,
    pub permit_count: usize,
    pub forbid_count: usize,
    pub status: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapClosureGateReport {
    pub task_count: usize,
    pub output_gate_count: usize,
    pub proof_count: usize,
    pub bounded_task_count: usize,
    pub open_output_count: usize,
    pub task_reports: Vec<BootstrapClosureTaskReport>,
    pub output_gate_reports: Vec<BootstrapClosureOutputGateReport>,
    pub proof_reports: Vec<BootstrapClosureProofReport>,
    pub gate_hash: String,
}

pub fn deterministic_bootstrap_closure_gate_report(
    tasks: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    outputs: &[(String, String, String, Vec<String>, Vec<String>, String)],
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
) -> BootstrapClosureGateReport {
    let mut sorted_tasks = tasks.to_vec();
    sorted_tasks.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_outputs = outputs.to_vec();
    sorted_outputs.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });
    let mut sorted_proofs = proofs.to_vec();
    sorted_proofs.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));

    let mut task_reports = Vec::new();
    let mut output_gate_reports = Vec::new();
    let mut proof_reports = Vec::new();
    let mut bounded_task_count = 0usize;
    let mut open_output_count = 0usize;
    let mut rows = Vec::new();

    for (id, scope, mut receipts, mut commands, mut evidence, status) in sorted_tasks {
        receipts.sort();
        receipts.dedup();
        commands.sort();
        commands.dedup();
        evidence.sort();
        evidence.dedup();
        if status == "bounded_closed" {
            bounded_task_count += 1;
        }
        let preimage = format!(
            "task:{}|scope:{}|receipts:{}|commands:{}|evidence:{}|status:{}",
            id,
            scope,
            receipts.join(","),
            commands.join(","),
            evidence.join(","),
            status
        );
        let task_hash = stable_hash_label("lyra.p02.bootstrap_closure.task", &preimage);
        rows.push(format!("task:{id}|hash:{task_hash}"));
        task_reports.push(BootstrapClosureTaskReport {
            id,
            scope,
            receipt_count: receipts.len(),
            command_count: commands.len(),
            evidence_count: evidence.len(),
            status,
            task_hash,
        });
    }

    for (id, output_kind, path, mut depends, mut receipts, status) in sorted_outputs {
        depends.sort();
        depends.dedup();
        receipts.sort();
        receipts.dedup();
        if status == "blocked" || status == "working_slice" || status == "artifact_emitted" {
            open_output_count += 1;
        }
        let preimage = format!(
            "output:{}|kind:{}|path:{}|depends:{}|receipts:{}|status:{}",
            id,
            output_kind,
            path,
            depends.join(","),
            receipts.join(","),
            status
        );
        let output_hash = stable_hash_label("lyra.p02.bootstrap_closure.output", &preimage);
        rows.push(format!("output:{id}|hash:{output_hash}"));
        output_gate_reports.push(BootstrapClosureOutputGateReport {
            id,
            output_kind,
            path,
            dependency_count: depends.len(),
            receipt_count: receipts.len(),
            status,
            output_hash,
        });
    }

    for (
        id,
        scope,
        mut tasks,
        mut outputs,
        mut receipts,
        mut commands,
        mut permits,
        mut forbids,
        status,
    ) in sorted_proofs
    {
        tasks.sort();
        tasks.dedup();
        outputs.sort();
        outputs.dedup();
        receipts.sort();
        receipts.dedup();
        commands.sort();
        commands.dedup();
        permits.sort();
        permits.dedup();
        forbids.sort();
        forbids.dedup();
        let preimage = format!(
            "proof:{}|scope:{}|tasks:{}|outputs:{}|receipts:{}|commands:{}|permits:{}|forbids:{}|status:{}",
            id, scope, tasks.join(","), outputs.join(","), receipts.join(","), commands.join(","), permits.join(","), forbids.join(","), status
        );
        let proof_hash = stable_hash_label("lyra.p02.bootstrap_closure.proof", &preimage);
        rows.push(format!("proof:{id}|hash:{proof_hash}"));
        proof_reports.push(BootstrapClosureProofReport {
            id,
            scope,
            task_count: tasks.len(),
            output_count: outputs.len(),
            receipt_count: receipts.len(),
            command_count: commands.len(),
            permit_count: permits.len(),
            forbid_count: forbids.len(),
            status,
            proof_hash,
        });
    }

    rows.sort();
    BootstrapClosureGateReport {
        task_count: task_reports.len(),
        output_gate_count: output_gate_reports.len(),
        proof_count: proof_reports.len(),
        bounded_task_count,
        open_output_count,
        task_reports,
        output_gate_reports,
        proof_reports,
        gate_hash: stable_hash_label("lyra.p02.bootstrap_closure.gate", &rows.join("\n")),
    }
}
