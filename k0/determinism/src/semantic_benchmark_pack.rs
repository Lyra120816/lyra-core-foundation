use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBenchmarkFamilyReport {
    pub id: String,
    pub family_kind: String,
    pub target_count: usize,
    pub proof_count: usize,
    pub status: String,
    pub family_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBenchmarkTargetReport {
    pub id: String,
    pub family: String,
    pub metric: String,
    pub unit: String,
    pub threshold: String,
    pub command: String,
    pub fixture: String,
    pub golden: String,
    pub receipt: String,
    pub status: String,
    pub target_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBenchmarkEvidenceReport {
    pub id: String,
    pub family: String,
    pub target_count: usize,
    pub artifact_count: usize,
    pub proof_receipt_count: usize,
    pub status: String,
    pub evidence_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBenchmarkPackReport {
    pub family_count: usize,
    pub target_count: usize,
    pub evidence_count: usize,
    pub throughput_target_count: usize,
    pub latency_target_count: usize,
    pub correctness_target_count: usize,
    pub stability_target_count: usize,
    pub family_reports: Vec<SemanticBenchmarkFamilyReport>,
    pub target_reports: Vec<SemanticBenchmarkTargetReport>,
    pub evidence_reports: Vec<SemanticBenchmarkEvidenceReport>,
    pub pack_hash: String,
}

pub fn deterministic_semantic_benchmark_pack_report(
    families: &[(String, String, Vec<String>, Vec<String>, String)],
    targets: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    evidence: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
) -> SemanticBenchmarkPackReport {
    let mut sorted_families = families.to_vec();
    sorted_families.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_targets = targets.to_vec();
    sorted_targets.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_evidence = evidence.to_vec();
    sorted_evidence.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut family_reports = Vec::new();
    let mut target_reports = Vec::new();
    let mut evidence_reports = Vec::new();
    let mut throughput_target_count = 0usize;
    let mut latency_target_count = 0usize;
    let mut correctness_target_count = 0usize;
    let mut stability_target_count = 0usize;
    let mut preimage = format!(
        "families:{}|targets:{}|evidence:{}",
        sorted_families.len(),
        sorted_targets.len(),
        sorted_evidence.len()
    );
    for (id, family_kind, mut target_ids, mut proofs, status) in sorted_families {
        target_ids.sort();
        proofs.sort();
        let family_preimage = format!(
            "family:{}|kind:{}|targets:{}|proofs:{}|status:{}",
            id,
            family_kind,
            target_ids.join(","),
            proofs.join(","),
            status
        );
        let family_hash =
            stable_hash_label("lyra.p01.semantic.benchmark_pack.family", &family_preimage);
        preimage.push('|');
        preimage.push_str(&family_preimage);
        family_reports.push(SemanticBenchmarkFamilyReport {
            id,
            family_kind,
            target_count: target_ids.len(),
            proof_count: proofs.len(),
            status,
            family_hash,
        });
    }
    for (id, family, metric, unit, threshold, command, fixture, golden, receipt, status) in
        sorted_targets
    {
        match family.as_str() {
            "throughput" => throughput_target_count += 1,
            "latency" => latency_target_count += 1,
            "correctness" => correctness_target_count += 1,
            "stability" => stability_target_count += 1,
            _ => {}
        }
        let target_preimage = format!("target:{id}|family:{family}|metric:{metric}|unit:{unit}|threshold:{threshold}|command:{command}|fixture:{fixture}|golden:{golden}|receipt:{receipt}|status:{status}");
        let target_hash =
            stable_hash_label("lyra.p01.semantic.benchmark_pack.target", &target_preimage);
        preimage.push('|');
        preimage.push_str(&target_preimage);
        target_reports.push(SemanticBenchmarkTargetReport {
            id,
            family,
            metric,
            unit,
            threshold,
            command,
            fixture,
            golden,
            receipt,
            status,
            target_hash,
        });
    }
    for (id, family, mut target_ids, mut artifacts, mut proof_receipts, status) in sorted_evidence {
        target_ids.sort();
        artifacts.sort();
        proof_receipts.sort();
        let evidence_preimage = format!(
            "evidence:{}|family:{}|targets:{}|artifacts:{}|receipts:{}|status:{}",
            id,
            family,
            target_ids.join(","),
            artifacts.join(","),
            proof_receipts.join(","),
            status
        );
        let evidence_hash = stable_hash_label(
            "lyra.p01.semantic.benchmark_pack.evidence",
            &evidence_preimage,
        );
        preimage.push('|');
        preimage.push_str(&evidence_preimage);
        evidence_reports.push(SemanticBenchmarkEvidenceReport {
            id,
            family,
            target_count: target_ids.len(),
            artifact_count: artifacts.len(),
            proof_receipt_count: proof_receipts.len(),
            status,
            evidence_hash,
        });
    }
    SemanticBenchmarkPackReport {
        family_count: family_reports.len(),
        target_count: target_reports.len(),
        evidence_count: evidence_reports.len(),
        throughput_target_count,
        latency_target_count,
        correctness_target_count,
        stability_target_count,
        family_reports,
        target_reports,
        evidence_reports,
        pack_hash: stable_hash_label("lyra.p01.semantic.benchmark_pack.pack", &preimage),
    }
}
