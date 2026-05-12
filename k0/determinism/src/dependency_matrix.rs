use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyNodeReport {
    pub id: String,
    pub node_kind: String,
    pub dependency_count: usize,
    pub unlock_count: usize,
    pub owner_root_count: usize,
    pub receipt_count: usize,
    pub status: String,
    pub node_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockerBindingReport {
    pub id: String,
    pub blocked_by_count: usize,
    pub unblock_count: usize,
    pub receipt_count: usize,
    pub status: String,
    pub blocker_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelLaneReport {
    pub id: String,
    pub lane_kind: String,
    pub frontier_count: usize,
    pub after_count: usize,
    pub parallel_count: usize,
    pub receipt_count: usize,
    pub status: String,
    pub lane_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyMatrixReport {
    pub dependency_count: usize,
    pub blocker_count: usize,
    pub lane_count: usize,
    pub open_blocker_count: usize,
    pub emitted_output_count: usize,
    pub node_reports: Vec<DependencyNodeReport>,
    pub blocker_reports: Vec<BlockerBindingReport>,
    pub lane_reports: Vec<ParallelLaneReport>,
    pub matrix_hash: String,
}

pub fn deterministic_dependency_matrix_report(
    dependencies: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    blockers: &[(
        String,
        Vec<String>,
        String,
        Vec<String>,
        Vec<String>,
        String,
    )],
    lanes: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
) -> DependencyMatrixReport {
    let mut sorted_dependencies = dependencies.to_vec();
    sorted_dependencies.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_blockers = blockers.to_vec();
    sorted_blockers.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_lanes = lanes.to_vec();
    sorted_lanes.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));

    let mut node_reports = Vec::new();
    let mut blocker_reports = Vec::new();
    let mut lane_reports = Vec::new();
    let mut open_blocker_count = 0usize;
    let mut emitted_output_count = 0usize;
    let mut preimage = format!(
        "dependencies:{}|blockers:{}|lanes:{}",
        sorted_dependencies.len(),
        sorted_blockers.len(),
        sorted_lanes.len()
    );

    for (id, node_kind, mut roots, mut depends, mut unlocks, mut receipts, status) in
        sorted_dependencies
    {
        roots.sort();
        depends.sort();
        unlocks.sort();
        receipts.sort();
        if id.starts_with("P00-X") && status == "artifact_emitted" {
            emitted_output_count += 1;
        }
        let node_preimage = format!(
            "node:{}|kind:{}|roots:{}|depends:{}|unlocks:{}|receipts:{}|status:{}",
            id,
            node_kind,
            roots.join(","),
            depends.join(","),
            unlocks.join(","),
            receipts.join(","),
            status
        );
        let node_hash = stable_hash_label("lyra.p00.dependency.node", &node_preimage);
        preimage.push('|');
        preimage.push_str(&node_preimage);
        node_reports.push(DependencyNodeReport {
            id,
            node_kind,
            dependency_count: depends.len(),
            unlock_count: unlocks.len(),
            owner_root_count: roots.len(),
            receipt_count: receipts.len(),
            status,
            node_hash,
        });
    }

    for (id, mut blocked_by, reason, mut unblocks, mut receipts, status) in sorted_blockers {
        blocked_by.sort();
        unblocks.sort();
        receipts.sort();
        if status == "blocked" {
            open_blocker_count += 1;
        }
        let blocker_preimage = format!(
            "blocker:{}|blocked_by:{}|reason:{}|unblocks:{}|receipts:{}|status:{}",
            id,
            blocked_by.join(","),
            reason,
            unblocks.join(","),
            receipts.join(","),
            status
        );
        let blocker_hash = stable_hash_label("lyra.p00.dependency.blocker", &blocker_preimage);
        preimage.push('|');
        preimage.push_str(&blocker_preimage);
        blocker_reports.push(BlockerBindingReport {
            id,
            blocked_by_count: blocked_by.len(),
            unblock_count: unblocks.len(),
            receipt_count: receipts.len(),
            status,
            blocker_hash,
        });
    }

    for (id, lane_kind, mut frontiers, mut after, mut parallel, mut receipts, status) in
        sorted_lanes
    {
        frontiers.sort();
        after.sort();
        parallel.sort();
        receipts.sort();
        let lane_preimage = format!(
            "lane:{}|kind:{}|frontiers:{}|after:{}|parallel:{}|receipts:{}|status:{}",
            id,
            lane_kind,
            frontiers.join(","),
            after.join(","),
            parallel.join(","),
            receipts.join(","),
            status
        );
        let lane_hash = stable_hash_label("lyra.p00.dependency.lane", &lane_preimage);
        preimage.push('|');
        preimage.push_str(&lane_preimage);
        lane_reports.push(ParallelLaneReport {
            id,
            lane_kind,
            frontier_count: frontiers.len(),
            after_count: after.len(),
            parallel_count: parallel.len(),
            receipt_count: receipts.len(),
            status,
            lane_hash,
        });
    }

    DependencyMatrixReport {
        dependency_count: node_reports.len(),
        blocker_count: blocker_reports.len(),
        lane_count: lane_reports.len(),
        open_blocker_count,
        emitted_output_count,
        node_reports,
        blocker_reports,
        lane_reports,
        matrix_hash: stable_hash_label("lyra.p00.dependency.matrix", &preimage),
    }
}
