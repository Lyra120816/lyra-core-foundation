use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputAudienceReport {
    pub id: String,
    pub output_count: usize,
    pub artifact_count: usize,
    pub receipt_count: usize,
    pub status: String,
    pub audience_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputArtifactReport {
    pub id: String,
    pub audience: String,
    pub artifact_kind: String,
    pub owner_root: String,
    pub path: String,
    pub status: String,
    pub artifact_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputContractReport {
    pub id: String,
    pub surface: String,
    pub path: String,
    pub status: String,
    pub contract_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputGapReport {
    pub id: String,
    pub blocker: String,
    pub next_frontier: String,
    pub owner_root: String,
    pub status: String,
    pub gap_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputTableReport {
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
    pub audience_reports: Vec<BootstrapOutputAudienceReport>,
    pub artifact_reports: Vec<BootstrapOutputArtifactReport>,
    pub receipt_reports: Vec<BootstrapOutputReceiptReport>,
    pub contract_reports: Vec<BootstrapOutputContractReport>,
    pub gap_reports: Vec<BootstrapOutputGapReport>,
    pub table_hash: String,
}

pub fn deterministic_bootstrap_output_table_report(
    audiences: &[(String, Vec<String>, Vec<String>, Vec<String>, String)],
    artifacts: &[(String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
    contracts: &[(String, String, String, String)],
    gaps: &[(String, String, String, String, String)],
) -> BootstrapOutputTableReport {
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
    let mut rows = Vec::new();

    for (id, mut outputs, mut artifacts, mut receipts, status) in sorted_audiences {
        outputs.sort();
        outputs.dedup();
        artifacts.sort();
        artifacts.dedup();
        receipts.sort();
        receipts.dedup();
        let preimage = format!(
            "audience:{}|outputs:{}|artifacts:{}|receipts:{}|status:{}",
            id,
            outputs.join(","),
            artifacts.join(","),
            receipts.join(","),
            status
        );
        let audience_hash =
            stable_hash_label("lyra.p02.bootstrap_output_table.audience", &preimage);
        rows.push(format!("audience:{id}|hash:{audience_hash}"));
        audience_reports.push(BootstrapOutputAudienceReport {
            id,
            output_count: outputs.len(),
            artifact_count: artifacts.len(),
            receipt_count: receipts.len(),
            status,
            audience_hash,
        });
    }

    for (id, audience, artifact_kind, owner_root, path, status) in sorted_artifacts {
        match audience.as_str() {
            "developer" => developer_artifact_count += 1,
            "operator" => operator_artifact_count += 1,
            "product" => product_artifact_count += 1,
            "enterprise" => enterprise_artifact_count += 1,
            "public_interest" => public_interest_artifact_count += 1,
            _ => {}
        }
        let preimage = format!("artifact:{id}|audience:{audience}|kind:{artifact_kind}|owner:{owner_root}|path:{path}|status:{status}");
        let artifact_hash =
            stable_hash_label("lyra.p02.bootstrap_output_table.artifact", &preimage);
        rows.push(format!("artifact:{id}|hash:{artifact_hash}"));
        artifact_reports.push(BootstrapOutputArtifactReport {
            id,
            audience,
            artifact_kind,
            owner_root,
            path,
            status,
            artifact_hash,
        });
    }

    for (id, path, target, status) in sorted_receipts {
        let preimage = format!("receipt:{id}|path:{path}|target:{target}|status:{status}");
        let receipt_hash = stable_hash_label("lyra.p02.bootstrap_output_table.receipt", &preimage);
        rows.push(format!("receipt:{id}|hash:{receipt_hash}"));
        receipt_reports.push(BootstrapOutputReceiptReport {
            id,
            path,
            target,
            status,
            receipt_hash,
        });
    }

    for (id, surface, path, status) in sorted_contracts {
        let preimage = format!("contract:{id}|surface:{surface}|path:{path}|status:{status}");
        let contract_hash =
            stable_hash_label("lyra.p02.bootstrap_output_table.contract", &preimage);
        rows.push(format!("contract:{id}|hash:{contract_hash}"));
        contract_reports.push(BootstrapOutputContractReport {
            id,
            surface,
            path,
            status,
            contract_hash,
        });
    }

    for (id, blocker, next_frontier, owner_root, status) in sorted_gaps {
        let preimage = format!(
            "gap:{id}|blocker:{blocker}|next:{next_frontier}|owner:{owner_root}|status:{status}"
        );
        let gap_hash = stable_hash_label("lyra.p02.bootstrap_output_table.gap", &preimage);
        rows.push(format!("gap:{id}|hash:{gap_hash}"));
        gap_reports.push(BootstrapOutputGapReport {
            id,
            blocker,
            next_frontier,
            owner_root,
            status,
            gap_hash,
        });
    }

    rows.sort();
    BootstrapOutputTableReport {
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
        table_hash: stable_hash_label("lyra.p02.bootstrap_output_table.table", &rows.join("\n")),
    }
}
