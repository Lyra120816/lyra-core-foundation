use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputAudienceReport {
    pub id: String,
    pub output_count: usize,
    pub artifact_count: usize,
    pub receipt_count: usize,
    pub status: String,
    pub audience_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputArtifactReport {
    pub id: String,
    pub audience: String,
    pub artifact_kind: String,
    pub path: String,
    pub status: String,
    pub artifact_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputContractReport {
    pub id: String,
    pub surface: String,
    pub path: String,
    pub status: String,
    pub contract_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedGapReport {
    pub id: String,
    pub blocker: String,
    pub next_frontier: String,
    pub owner_root: String,
    pub status: String,
    pub gap_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputTableReport {
    pub audience_count: usize,
    pub artifact_count: usize,
    pub receipt_count: usize,
    pub contract_count: usize,
    pub unresolved_gap_count: usize,
    pub developer_artifact_count: usize,
    pub operator_artifact_count: usize,
    pub product_artifact_count: usize,
    pub enterprise_artifact_count: usize,
    pub public_interest_artifact_count: usize,
    pub audience_reports: Vec<OutputAudienceReport>,
    pub artifact_reports: Vec<OutputArtifactReport>,
    pub receipt_reports: Vec<OutputReceiptReport>,
    pub contract_reports: Vec<OutputContractReport>,
    pub gap_reports: Vec<UnresolvedGapReport>,
    pub table_hash: String,
}

pub fn deterministic_output_table_report(
    audiences: &[(String, Vec<String>, Vec<String>, Vec<String>, String)],
    artifacts: &[(String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
    contracts: &[(String, String, String, String)],
    gaps: &[(String, String, String, String, String)],
) -> OutputTableReport {
    let mut sorted_audiences = audiences.to_vec();
    sorted_audiences.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_artifacts = artifacts.to_vec();
    sorted_artifacts.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_receipts = receipts.to_vec();
    sorted_receipts.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_contracts = contracts.to_vec();
    sorted_contracts.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_gaps = gaps.to_vec();
    sorted_gaps.sort_by(|left, right| left.0.cmp(&right.0));

    let mut audience_reports = Vec::new();
    let mut artifact_reports = Vec::new();
    let mut receipt_reports = Vec::new();
    let mut contract_reports = Vec::new();
    let mut gap_reports = Vec::new();
    let mut developer_artifact_count = 0usize;
    let mut operator_artifact_count = 0usize;
    let mut product_artifact_count = 0usize;
    let mut enterprise_artifact_count = 0usize;
    let mut public_interest_artifact_count = 0usize;
    let mut preimage = format!(
        "audiences:{}|artifacts:{}|receipts:{}|contracts:{}|gaps:{}",
        sorted_audiences.len(),
        sorted_artifacts.len(),
        sorted_receipts.len(),
        sorted_contracts.len(),
        sorted_gaps.len()
    );

    for (id, mut outputs, mut artifacts, mut receipts, status) in sorted_audiences {
        outputs.sort();
        artifacts.sort();
        receipts.sort();
        let audience_preimage = format!(
            "audience:{}|outputs:{}|artifacts:{}|receipts:{}|status:{}",
            id,
            outputs.join(","),
            artifacts.join(","),
            receipts.join(","),
            status
        );
        let audience_hash = stable_hash_label("lyra.p00.output_table.audience", &audience_preimage);
        preimage.push('|');
        preimage.push_str(&audience_preimage);
        audience_reports.push(OutputAudienceReport {
            id,
            output_count: outputs.len(),
            artifact_count: artifacts.len(),
            receipt_count: receipts.len(),
            status,
            audience_hash,
        });
    }

    for (id, audience, artifact_kind, path, status) in sorted_artifacts {
        match audience.as_str() {
            "developer" => developer_artifact_count += 1,
            "operator" => operator_artifact_count += 1,
            "product" => product_artifact_count += 1,
            "enterprise" => enterprise_artifact_count += 1,
            "public_interest" => public_interest_artifact_count += 1,
            _ => {}
        }
        let artifact_preimage = format!(
            "artifact:{id}|audience:{audience}|kind:{artifact_kind}|path:{path}|status:{status}"
        );
        let artifact_hash = stable_hash_label("lyra.p00.output_table.artifact", &artifact_preimage);
        preimage.push('|');
        preimage.push_str(&artifact_preimage);
        artifact_reports.push(OutputArtifactReport {
            id,
            audience,
            artifact_kind,
            path,
            status,
            artifact_hash,
        });
    }

    for (id, path, target, status) in sorted_receipts {
        let receipt_preimage = format!("receipt:{id}|path:{path}|target:{target}|status:{status}");
        let receipt_hash = stable_hash_label("lyra.p00.output_table.receipt", &receipt_preimage);
        preimage.push('|');
        preimage.push_str(&receipt_preimage);
        receipt_reports.push(OutputReceiptReport {
            id,
            path,
            target,
            status,
            receipt_hash,
        });
    }

    for (id, surface, path, status) in sorted_contracts {
        let contract_preimage =
            format!("contract:{id}|surface:{surface}|path:{path}|status:{status}");
        let contract_hash = stable_hash_label("lyra.p00.output_table.contract", &contract_preimage);
        preimage.push('|');
        preimage.push_str(&contract_preimage);
        contract_reports.push(OutputContractReport {
            id,
            surface,
            path,
            status,
            contract_hash,
        });
    }

    for (id, blocker, next_frontier, owner_root, status) in sorted_gaps {
        let gap_preimage = format!(
            "gap:{id}|blocker:{blocker}|next:{next_frontier}|owner:{owner_root}|status:{status}"
        );
        let gap_hash = stable_hash_label("lyra.p00.output_table.gap", &gap_preimage);
        preimage.push('|');
        preimage.push_str(&gap_preimage);
        gap_reports.push(UnresolvedGapReport {
            id,
            blocker,
            next_frontier,
            owner_root,
            status,
            gap_hash,
        });
    }

    OutputTableReport {
        audience_count: audience_reports.len(),
        artifact_count: artifact_reports.len(),
        receipt_count: receipt_reports.len(),
        contract_count: contract_reports.len(),
        unresolved_gap_count: gap_reports.len(),
        developer_artifact_count,
        operator_artifact_count,
        product_artifact_count,
        enterprise_artifact_count,
        public_interest_artifact_count,
        audience_reports,
        artifact_reports,
        receipt_reports,
        contract_reports,
        gap_reports,
        table_hash: stable_hash_label("lyra.p00.output_table.table", &preimage),
    }
}
