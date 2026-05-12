use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyNodeReport {
    pub id: String,
    pub kind: String,
    pub status: String,
    pub dependency_count: usize,
    pub unblock_count: usize,
    pub owner_root: String,
    pub node_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyBlockerReport {
    pub id: String,
    pub target: String,
    pub severity: String,
    pub block_count: usize,
    pub require_count: usize,
    pub status: String,
    pub blocker_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapParallelLaneReport {
    pub id: String,
    pub scope: String,
    pub task_count: usize,
    pub dependency_count: usize,
    pub parallel_safe: String,
    pub status: String,
    pub lane_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyProofReport {
    pub id: String,
    pub node_count: usize,
    pub blocker_count: usize,
    pub lane_count: usize,
    pub receipt_count: usize,
    pub command_count: usize,
    pub status: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyMatrixReport {
    pub node_count: usize,
    pub blocker_count: usize,
    pub lane_count: usize,
    pub proof_count: usize,
    pub blocked_node_count: usize,
    pub artifact_node_count: usize,
    pub node_reports: Vec<BootstrapDependencyNodeReport>,
    pub blocker_reports: Vec<BootstrapDependencyBlockerReport>,
    pub lane_reports: Vec<BootstrapParallelLaneReport>,
    pub proof_reports: Vec<BootstrapDependencyProofReport>,
    pub matrix_hash: String,
}

pub fn deterministic_bootstrap_dependency_matrix_report(
    nodes: &[(String, String, String, Vec<String>, Vec<String>, String)],
    blockers: &[(String, String, String, Vec<String>, Vec<String>, String)],
    lanes: &[(String, String, Vec<String>, Vec<String>, String, String)],
    proofs: &[(
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
) -> BootstrapDependencyMatrixReport {
    let mut sorted_nodes = nodes.to_vec();
    sorted_nodes.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_blockers = blockers.to_vec();
    sorted_blockers.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_lanes = lanes.to_vec();
    sorted_lanes.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_proofs = proofs.to_vec();
    sorted_proofs.sort_by(|left, right| left.0.cmp(&right.0));

    let mut node_reports = Vec::new();
    let mut blocker_reports = Vec::new();
    let mut lane_reports = Vec::new();
    let mut proof_reports = Vec::new();
    let mut blocked_node_count = 0usize;
    let mut artifact_node_count = 0usize;
    let mut rows = Vec::new();

    for (id, kind, status, mut depends, mut unblocks, owner_root) in sorted_nodes {
        depends.sort();
        depends.dedup();
        unblocks.sort();
        unblocks.dedup();
        if status == "blocked" {
            blocked_node_count += 1;
        }
        if status == "artifact_emitted" {
            artifact_node_count += 1;
        }
        let preimage = format!(
            "node:{}|kind:{}|status:{}|depends:{}|unblocks:{}|owner_root:{}",
            id,
            kind,
            status,
            depends.join(","),
            unblocks.join(","),
            owner_root
        );
        let node_hash = stable_hash_label("lyra.p02.bootstrap_dependency_matrix.node", &preimage);
        rows.push(format!("node:{id}|hash:{node_hash}"));
        node_reports.push(BootstrapDependencyNodeReport {
            id,
            kind,
            status,
            dependency_count: depends.len(),
            unblock_count: unblocks.len(),
            owner_root,
            node_hash,
        });
    }

    for (id, target, severity, mut blocks, mut requires, status) in sorted_blockers {
        blocks.sort();
        blocks.dedup();
        requires.sort();
        requires.dedup();
        let preimage = format!(
            "blocker:{}|target:{}|severity:{}|blocks:{}|requires:{}|status:{}",
            id,
            target,
            severity,
            blocks.join(","),
            requires.join(","),
            status
        );
        let blocker_hash =
            stable_hash_label("lyra.p02.bootstrap_dependency_matrix.blocker", &preimage);
        rows.push(format!("blocker:{id}|hash:{blocker_hash}"));
        blocker_reports.push(BootstrapDependencyBlockerReport {
            id,
            target,
            severity,
            block_count: blocks.len(),
            require_count: requires.len(),
            status,
            blocker_hash,
        });
    }

    for (id, scope, mut tasks, mut depends, parallel_safe, status) in sorted_lanes {
        tasks.sort();
        tasks.dedup();
        depends.sort();
        depends.dedup();
        let preimage = format!(
            "lane:{}|scope:{}|tasks:{}|depends:{}|parallel_safe:{}|status:{}",
            id,
            scope,
            tasks.join(","),
            depends.join(","),
            parallel_safe,
            status
        );
        let lane_hash = stable_hash_label("lyra.p02.bootstrap_dependency_matrix.lane", &preimage);
        rows.push(format!("lane:{id}|hash:{lane_hash}"));
        lane_reports.push(BootstrapParallelLaneReport {
            id,
            scope,
            task_count: tasks.len(),
            dependency_count: depends.len(),
            parallel_safe,
            status,
            lane_hash,
        });
    }

    for (id, mut nodes, mut blockers, mut lanes, mut receipts, mut commands, status) in
        sorted_proofs
    {
        nodes.sort();
        nodes.dedup();
        blockers.sort();
        blockers.dedup();
        lanes.sort();
        lanes.dedup();
        receipts.sort();
        receipts.dedup();
        commands.sort();
        commands.dedup();
        let preimage = format!(
            "proof:{}|nodes:{}|blockers:{}|lanes:{}|receipts:{}|commands:{}|status:{}",
            id,
            nodes.join(","),
            blockers.join(","),
            lanes.join(","),
            receipts.join(","),
            commands.join(","),
            status
        );
        let proof_hash = stable_hash_label("lyra.p02.bootstrap_dependency_matrix.proof", &preimage);
        rows.push(format!("proof:{id}|hash:{proof_hash}"));
        proof_reports.push(BootstrapDependencyProofReport {
            id,
            node_count: nodes.len(),
            blocker_count: blockers.len(),
            lane_count: lanes.len(),
            receipt_count: receipts.len(),
            command_count: commands.len(),
            status,
            proof_hash,
        });
    }

    rows.sort();
    let matrix_hash = stable_hash_label(
        "lyra.p02.bootstrap_dependency_matrix.report",
        &rows.join("\n"),
    );
    BootstrapDependencyMatrixReport {
        node_count: node_reports.len(),
        blocker_count: blocker_reports.len(),
        lane_count: lane_reports.len(),
        proof_count: proof_reports.len(),
        blocked_node_count,
        artifact_node_count,
        node_reports,
        blocker_reports,
        lane_reports,
        proof_reports,
        matrix_hash,
    }
}
