use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofFamilyReport {
    pub id: String,
    pub family_kind: String,
    pub receipt_count: usize,
    pub coverage_count: usize,
    pub proof_count: usize,
    pub status: String,
    pub family_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofReceiptReport {
    pub id: String,
    pub family: String,
    pub path: String,
    pub coverage_count: usize,
    pub verdict: String,
    pub status: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPathReport {
    pub id: String,
    pub family: String,
    pub path_kind: String,
    pub entry_receipt_count: usize,
    pub challenge_receipt_count: usize,
    pub rollback_receipt_count: usize,
    pub status: String,
    pub path_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofFamilyTableReport {
    pub family_count: usize,
    pub receipt_count: usize,
    pub path_count: usize,
    pub happy_path_receipt_count: usize,
    pub negative_path_receipt_count: usize,
    pub adversarial_path_receipt_count: usize,
    pub rollback_path_receipt_count: usize,
    pub family_reports: Vec<ProofFamilyReport>,
    pub receipt_reports: Vec<ProofReceiptReport>,
    pub path_reports: Vec<ProofPathReport>,
    pub table_hash: String,
}

pub fn deterministic_proof_family_table_report(
    families: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    receipts: &[(String, String, String, Vec<String>, String, String)],
    paths: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
) -> ProofFamilyTableReport {
    let mut sorted_families = families.to_vec();
    sorted_families.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_receipts = receipts.to_vec();
    sorted_receipts.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_paths = paths.to_vec();
    sorted_paths.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut family_reports = Vec::new();
    let mut receipt_reports = Vec::new();
    let mut path_reports = Vec::new();
    let mut happy_path_receipt_count = 0usize;
    let mut negative_path_receipt_count = 0usize;
    let mut adversarial_path_receipt_count = 0usize;
    let mut rollback_path_receipt_count = 0usize;
    let mut preimage = format!(
        "families:{}|receipts:{}|paths:{}",
        sorted_families.len(),
        sorted_receipts.len(),
        sorted_paths.len()
    );
    for (id, family_kind, mut family_receipts, mut covers, mut proofs, status) in sorted_families {
        family_receipts.sort();
        covers.sort();
        proofs.sort();
        let family_preimage = format!(
            "family:{}|kind:{}|receipts:{}|covers:{}|proofs:{}|status:{}",
            id,
            family_kind,
            family_receipts.join(","),
            covers.join(","),
            proofs.join(","),
            status
        );
        let family_hash = stable_hash_label("lyra.p00.proof_family.family", &family_preimage);
        preimage.push('|');
        preimage.push_str(&family_preimage);
        family_reports.push(ProofFamilyReport {
            id,
            family_kind,
            receipt_count: family_receipts.len(),
            coverage_count: covers.len(),
            proof_count: proofs.len(),
            status,
            family_hash,
        });
    }
    for (id, family, path, mut covers, verdict, status) in sorted_receipts {
        covers.sort();
        match family.as_str() {
            "happy_path" => happy_path_receipt_count += 1,
            "negative_path" => negative_path_receipt_count += 1,
            "adversarial_path" => adversarial_path_receipt_count += 1,
            "rollback_path" => rollback_path_receipt_count += 1,
            _ => {}
        }
        let receipt_preimage = format!(
            "receipt:{}|family:{}|path:{}|covers:{}|verdict:{}|status:{}",
            id,
            family,
            path,
            covers.join(","),
            verdict,
            status
        );
        let receipt_hash = stable_hash_label("lyra.p00.proof_family.receipt", &receipt_preimage);
        preimage.push('|');
        preimage.push_str(&receipt_preimage);
        receipt_reports.push(ProofReceiptReport {
            id,
            family,
            path,
            coverage_count: covers.len(),
            verdict,
            status,
            receipt_hash,
        });
    }
    for (
        id,
        family,
        path_kind,
        mut entry_receipts,
        mut challenge_receipts,
        mut rollback_receipts,
        status,
    ) in sorted_paths
    {
        entry_receipts.sort();
        challenge_receipts.sort();
        rollback_receipts.sort();
        let path_preimage = format!(
            "path:{}|family:{}|kind:{}|entries:{}|challenges:{}|rollbacks:{}|status:{}",
            id,
            family,
            path_kind,
            entry_receipts.join(","),
            challenge_receipts.join(","),
            rollback_receipts.join(","),
            status
        );
        let path_hash = stable_hash_label("lyra.p00.proof_family.path", &path_preimage);
        preimage.push('|');
        preimage.push_str(&path_preimage);
        path_reports.push(ProofPathReport {
            id,
            family,
            path_kind,
            entry_receipt_count: entry_receipts.len(),
            challenge_receipt_count: challenge_receipts.len(),
            rollback_receipt_count: rollback_receipts.len(),
            status,
            path_hash,
        });
    }
    ProofFamilyTableReport {
        family_count: family_reports.len(),
        receipt_count: receipt_reports.len(),
        path_count: path_reports.len(),
        happy_path_receipt_count,
        negative_path_receipt_count,
        adversarial_path_receipt_count,
        rollback_path_receipt_count,
        family_reports,
        receipt_reports,
        path_reports,
        table_hash: stable_hash_label("lyra.p00.proof_family.table", &preimage),
    }
}
