use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapBenchmarkFamilyReport {
    pub id: String,
    pub family_kind: String,
    pub target_count: usize,
    pub proof_count: usize,
    pub status: String,
    pub family_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapBenchmarkTargetReport {
    pub id: String,
    pub family: String,
    pub metric: String,
    pub unit: String,
    pub threshold: String,
    pub command: String,
    pub fixture: String,
    pub golden: String,
    pub receipt: String,
    pub expected: String,
    pub status: String,
    pub target_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapBenchmarkEvidenceReport {
    pub id: String,
    pub family: String,
    pub target_count: usize,
    pub artifact_count: usize,
    pub proof_receipt_count: usize,
    pub status: String,
    pub evidence_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapBenchmarkPackReport {
    pub family_count: usize,
    pub target_count: usize,
    pub evidence_count: usize,
    pub throughput_target_count: usize,
    pub latency_target_count: usize,
    pub correctness_target_count: usize,
    pub stability_target_count: usize,
    pub adversarial_target_count: usize,
    pub rollback_target_count: usize,
    pub family_reports: Vec<BootstrapBenchmarkFamilyReport>,
    pub target_reports: Vec<BootstrapBenchmarkTargetReport>,
    pub evidence_reports: Vec<BootstrapBenchmarkEvidenceReport>,
    pub pack_hash: String,
}

pub fn deterministic_bootstrap_benchmark_pack_report(
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
) -> BootstrapBenchmarkPackReport {
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
    let mut adversarial_target_count = 0usize;
    let mut rollback_target_count = 0usize;
    let mut rows = Vec::new();

    for (id, family_kind, mut target_ids, mut proofs, status) in sorted_families {
        target_ids.sort();
        target_ids.dedup();
        proofs.sort();
        proofs.dedup();
        let preimage = format!(
            "family:{}|kind:{}|targets:{}|proofs:{}|status:{}",
            id,
            family_kind,
            target_ids.join(","),
            proofs.join(","),
            status
        );
        let family_hash = stable_hash_label("lyra.p02.bootstrap_benchmark_pack.family", &preimage);
        rows.push(format!("family:{id}|hash:{family_hash}"));
        family_reports.push(BootstrapBenchmarkFamilyReport {
            id,
            family_kind,
            target_count: target_ids.len(),
            proof_count: proofs.len(),
            status,
            family_hash,
        });
    }

    for (
        id,
        family,
        metric,
        unit,
        threshold,
        command,
        fixture,
        golden,
        receipt,
        expected,
        status,
    ) in sorted_targets
    {
        match family.as_str() {
            "throughput" => throughput_target_count += 1,
            "latency" => latency_target_count += 1,
            "correctness" => correctness_target_count += 1,
            "stability" => stability_target_count += 1,
            "adversarial" => adversarial_target_count += 1,
            "rollback" => rollback_target_count += 1,
            _ => {}
        }
        let preimage = format!("target:{id}|family:{family}|metric:{metric}|unit:{unit}|threshold:{threshold}|command:{command}|fixture:{fixture}|golden:{golden}|receipt:{receipt}|expected:{expected}|status:{status}");
        let target_hash = stable_hash_label("lyra.p02.bootstrap_benchmark_pack.target", &preimage);
        rows.push(format!("target:{id}|hash:{target_hash}"));
        target_reports.push(BootstrapBenchmarkTargetReport {
            id,
            family,
            metric,
            unit,
            threshold,
            command,
            fixture,
            golden,
            receipt,
            expected,
            status,
            target_hash,
        });
    }

    for (id, family, mut target_ids, mut artifacts, mut proof_receipts, status) in sorted_evidence {
        target_ids.sort();
        target_ids.dedup();
        artifacts.sort();
        artifacts.dedup();
        proof_receipts.sort();
        proof_receipts.dedup();
        let preimage = format!(
            "evidence:{}|family:{}|targets:{}|artifacts:{}|receipts:{}|status:{}",
            id,
            family,
            target_ids.join(","),
            artifacts.join(","),
            proof_receipts.join(","),
            status
        );
        let evidence_hash =
            stable_hash_label("lyra.p02.bootstrap_benchmark_pack.evidence", &preimage);
        rows.push(format!("evidence:{id}|hash:{evidence_hash}"));
        evidence_reports.push(BootstrapBenchmarkEvidenceReport {
            id,
            family,
            target_count: target_ids.len(),
            artifact_count: artifacts.len(),
            proof_receipt_count: proof_receipts.len(),
            status,
            evidence_hash,
        });
    }

    rows.sort();
    BootstrapBenchmarkPackReport {
        family_count: family_reports.len(),
        target_count: target_reports.len(),
        evidence_count: evidence_reports.len(),
        throughput_target_count,
        latency_target_count,
        correctness_target_count,
        stability_target_count,
        adversarial_target_count,
        rollback_target_count,
        family_reports,
        target_reports,
        evidence_reports,
        pack_hash: stable_hash_label("lyra.p02.bootstrap_benchmark_pack.pack", &rows.join("\n")),
    }
}
