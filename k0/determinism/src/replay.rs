use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReceiptReport {
    pub order: String,
    pub receipt_id: String,
    pub receipt_hash: String,
    pub replay_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayWitnessReport {
    pub order: String,
    pub witness_id: String,
    pub witness_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplaySuiteReport {
    pub task: String,
    pub receipt_count: usize,
    pub witness_count: usize,
    pub receipts: Vec<ReplayReceiptReport>,
    pub witnesses: Vec<ReplayWitnessReport>,
    pub suite_hash: String,
}

pub fn deterministic_replay_report(
    task: &str,
    receipts: &[(&str, &str)],
    witnesses: &[(&str, &str)],
) -> ReplaySuiteReport {
    let mut ordered_receipts: Vec<(&str, &str)> = receipts.to_vec();
    ordered_receipts.sort_by(|left, right| left.0.cmp(right.0).then(left.1.cmp(right.1)));
    let mut ordered_witnesses: Vec<(&str, &str)> = witnesses.to_vec();
    ordered_witnesses.sort_by(|left, right| left.0.cmp(right.0).then(left.1.cmp(right.1)));

    let mut receipt_reports = Vec::with_capacity(ordered_receipts.len());
    for (index, (receipt_id, receipt_text)) in ordered_receipts.iter().enumerate() {
        let receipt_hash = stable_hash_label("lyra.p00.replay.receipt", receipt_text);
        let replay_preimage = format!("{task}|{receipt_id}|{receipt_hash}");
        receipt_reports.push(ReplayReceiptReport {
            order: format!("{:03}", index + 1),
            receipt_id: (*receipt_id).to_string(),
            receipt_hash,
            replay_hash: stable_hash_label("lyra.p00.replay.receipt.report", &replay_preimage),
        });
    }

    let mut witness_reports = Vec::with_capacity(ordered_witnesses.len());
    for (index, (witness_id, witness_text)) in ordered_witnesses.iter().enumerate() {
        witness_reports.push(ReplayWitnessReport {
            order: format!("{:03}", index + 1),
            witness_id: (*witness_id).to_string(),
            witness_hash: stable_hash_label("lyra.p00.replay.witness", witness_text),
        });
    }

    let mut preimage = String::new();
    preimage.push_str(task);
    preimage.push('\n');
    for report in &receipt_reports {
        preimage.push_str(&report.order);
        preimage.push('|');
        preimage.push_str(&report.receipt_id);
        preimage.push('|');
        preimage.push_str(&report.receipt_hash);
        preimage.push('|');
        preimage.push_str(&report.replay_hash);
        preimage.push('\n');
    }
    for report in &witness_reports {
        preimage.push_str(&report.order);
        preimage.push('|');
        preimage.push_str(&report.witness_id);
        preimage.push('|');
        preimage.push_str(&report.witness_hash);
        preimage.push('\n');
    }

    let suite_hash = stable_hash_label("lyra.p00.replay.suite", &preimage);
    ReplaySuiteReport {
        task: task.to_string(),
        receipt_count: receipt_reports.len(),
        witness_count: witness_reports.len(),
        receipts: receipt_reports,
        witnesses: witness_reports,
        suite_hash,
    }
}
