use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureTaskReport {
    pub id: String,
    pub scope: String,
    pub receipt_count: usize,
    pub command_count: usize,
    pub evidence_count: usize,
    pub status: String,
    pub task_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureOutputGateReport {
    pub id: String,
    pub output_kind: String,
    pub path: String,
    pub dependency_count: usize,
    pub receipt_count: usize,
    pub status: String,
    pub output_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureGateReport {
    pub task_count: usize,
    pub output_gate_count: usize,
    pub proof_count: usize,
    pub bounded_task_count: usize,
    pub open_output_count: usize,
    pub task_reports: Vec<ClosureTaskReport>,
    pub output_gate_reports: Vec<ClosureOutputGateReport>,
    pub gate_hash: String,
}

pub fn deterministic_closure_gate_report(
    tasks: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    outputs: &[(String, String, String, Vec<String>, Vec<String>, String)],
    proof_count: usize,
) -> ClosureGateReport {
    let mut sorted_tasks = tasks.to_vec();
    sorted_tasks.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_outputs = outputs.to_vec();
    sorted_outputs.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });

    let mut task_reports = Vec::new();
    let mut output_gate_reports = Vec::new();
    let mut bounded_task_count = 0usize;
    let mut open_output_count = 0usize;
    let mut preimage = format!(
        "tasks:{}|outputs:{}|proofs:{}",
        sorted_tasks.len(),
        sorted_outputs.len(),
        proof_count
    );

    for (id, scope, mut receipts, mut commands, mut evidence, status) in sorted_tasks {
        receipts.sort();
        commands.sort();
        evidence.sort();
        if status == "bounded_closed" {
            bounded_task_count += 1;
        }
        let task_preimage = format!(
            "task:{}|scope:{}|receipts:{}|commands:{}|evidence:{}|status:{}",
            id,
            scope,
            receipts.join(","),
            commands.join(","),
            evidence.join(","),
            status
        );
        let task_hash = stable_hash_label("lyra.p00.closure.task", &task_preimage);
        preimage.push('|');
        preimage.push_str(&task_preimage);
        task_reports.push(ClosureTaskReport {
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
        receipts.sort();
        if status == "blocked" || status == "working_slice" || status == "artifact_emitted" {
            open_output_count += 1;
        }
        let output_preimage = format!(
            "output:{}|kind:{}|path:{}|depends:{}|receipts:{}|status:{}",
            id,
            output_kind,
            path,
            depends.join(","),
            receipts.join(","),
            status
        );
        let output_hash = stable_hash_label("lyra.p00.closure.output", &output_preimage);
        preimage.push('|');
        preimage.push_str(&output_preimage);
        output_gate_reports.push(ClosureOutputGateReport {
            id,
            output_kind,
            path,
            dependency_count: depends.len(),
            receipt_count: receipts.len(),
            status,
            output_hash,
        });
    }

    ClosureGateReport {
        task_count: task_reports.len(),
        output_gate_count: output_gate_reports.len(),
        proof_count,
        bounded_task_count,
        open_output_count,
        task_reports,
        output_gate_reports,
        gate_hash: stable_hash_label("lyra.p00.closure.gate", &preimage),
    }
}
