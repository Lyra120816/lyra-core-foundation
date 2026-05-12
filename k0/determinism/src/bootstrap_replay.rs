use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayReceiptReport {
    pub id: String,
    pub path: String,
    pub receipt_hash: String,
    pub replay_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayWitnessReport {
    pub id: String,
    pub order: String,
    pub receipt_count: usize,
    pub witness_hash: String,
    pub replay_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayChainLinkReport {
    pub id: String,
    pub from: String,
    pub to: String,
    pub link_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayProofReport {
    pub id: String,
    pub scope: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayArtifactReport {
    pub id: String,
    pub artifact_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplaySuiteReport {
    pub receipt_count: usize,
    pub witness_count: usize,
    pub link_count: usize,
    pub proof_count: usize,
    pub artifact_count: usize,
    pub receipts: Vec<BootstrapReplayReceiptReport>,
    pub witnesses: Vec<BootstrapReplayWitnessReport>,
    pub links: Vec<BootstrapReplayChainLinkReport>,
    pub proofs: Vec<BootstrapReplayProofReport>,
    pub artifacts: Vec<BootstrapReplayArtifactReport>,
    pub suite_hash: String,
}

pub fn deterministic_bootstrap_replay_suite_report(
    receipts: &[(String, String, String, String, String, String, String)],
    witnesses: &[(
        String,
        String,
        Vec<String>,
        String,
        String,
        Vec<String>,
        String,
    )],
    links: &[(String, String, String, String, Vec<String>, String)],
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
    artifacts: &[(String, String, String, String, String)],
) -> BootstrapReplaySuiteReport {
    let mut receipt_reports: Vec<BootstrapReplayReceiptReport> = receipts.iter().map(|item| {
        let preimage = format!("receipt:{}|path:{}|input_hash:{}|canonical_hash:{}|verdict_hash:{}|receipt_hash:{}|status:{}", item.0, item.1, item.2, item.3, item.4, item.5, item.6);
        let replay_preimage = format!("{}|{}|{}", item.0, item.1, item.5);
        BootstrapReplayReceiptReport {
            id: item.0.clone(),
            path: item.1.clone(),
            receipt_hash: item.5.clone(),
            replay_hash: stable_hash_label("lyra.p02.bootstrap_replay.receipt", &format!("{preimage}\n{replay_preimage}")),
        }
    }).collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut witness_reports: Vec<BootstrapReplayWitnessReport> = witnesses
        .iter()
        .map(|item| {
            let preimage = format!(
                "witness:{}|order:{}|receipts:{}|preimage:{}|witness_hash:{}|commands:{}|status:{}",
                item.0,
                item.1,
                sorted_join(&item.2),
                item.3,
                item.4,
                sorted_join(&item.5),
                item.6
            );
            BootstrapReplayWitnessReport {
                id: item.0.clone(),
                order: item.1.clone(),
                receipt_count: sorted_count(&item.2),
                witness_hash: item.4.clone(),
                replay_hash: stable_hash_label("lyra.p02.bootstrap_replay.witness", &preimage),
            }
        })
        .collect();
    witness_reports
        .sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));

    let mut link_reports: Vec<BootstrapReplayChainLinkReport> = links
        .iter()
        .map(|item| {
            let preimage = format!(
                "link:{}|from:{}|to:{}|relation:{}|receipts:{}|status:{}",
                item.0,
                item.1,
                item.2,
                item.3,
                sorted_join(&item.4),
                item.5
            );
            BootstrapReplayChainLinkReport {
                id: item.0.clone(),
                from: item.1.clone(),
                to: item.2.clone(),
                link_hash: stable_hash_label("lyra.p02.bootstrap_replay.link", &preimage),
            }
        })
        .collect();
    link_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<BootstrapReplayProofReport> = proofs.iter().map(|item| {
        let preimage = format!("proof:{}|scope:{}|receipts:{}|witnesses:{}|links:{}|commands:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), item.7);
        BootstrapReplayProofReport { id: item.0.clone(), scope: item.1.clone(), proof_hash: stable_hash_label("lyra.p02.bootstrap_replay.proof", &preimage) }
    }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut artifact_reports: Vec<BootstrapReplayArtifactReport> = artifacts
        .iter()
        .map(|item| {
            let preimage = format!(
                "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            BootstrapReplayArtifactReport {
                id: item.0.clone(),
                artifact_hash: stable_hash_label("lyra.p02.bootstrap_replay.artifact", &preimage),
            }
        })
        .collect();
    artifact_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_lines = Vec::new();
    for item in &receipt_reports {
        suite_lines.push(format!("receipt:{}|{}", item.id, item.replay_hash));
    }
    for item in &witness_reports {
        suite_lines.push(format!("witness:{}|{}", item.id, item.replay_hash));
    }
    for item in &link_reports {
        suite_lines.push(format!("link:{}|{}", item.id, item.link_hash));
    }
    for item in &proof_reports {
        suite_lines.push(format!("proof:{}|{}", item.id, item.proof_hash));
    }
    for item in &artifact_reports {
        suite_lines.push(format!("artifact:{}|{}", item.id, item.artifact_hash));
    }
    suite_lines.sort();

    BootstrapReplaySuiteReport {
        receipt_count: receipt_reports.len(),
        witness_count: witness_reports.len(),
        link_count: link_reports.len(),
        proof_count: proof_reports.len(),
        artifact_count: artifact_reports.len(),
        receipts: receipt_reports,
        witnesses: witness_reports,
        links: link_reports,
        proofs: proof_reports,
        artifacts: artifact_reports,
        suite_hash: stable_hash_label("lyra.p02.bootstrap_replay.suite", &suite_lines.join("\n")),
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
