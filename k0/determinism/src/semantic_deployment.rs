use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDeploymentTargetReport {
    pub id: String,
    pub kind: String,
    pub environment: String,
    pub artifact_count: usize,
    pub command_count: usize,
    pub receipt_count: usize,
    pub forbid_count: usize,
    pub target_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticComplianceHookReport {
    pub id: String,
    pub scope: String,
    pub target: String,
    pub require_count: usize,
    pub evidence_count: usize,
    pub receipt_count: usize,
    pub hook_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReleaseEvidenceReport {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub target_count: usize,
    pub hook_count: usize,
    pub receipt_count: usize,
    pub command_count: usize,
    pub evidence_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDeploymentProofReport {
    pub id: String,
    pub scope: String,
    pub target_count: usize,
    pub hook_count: usize,
    pub evidence_count: usize,
    pub receipt_count: usize,
    pub command_count: usize,
    pub forbid_count: usize,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDeploymentSuiteReport {
    pub target_count: usize,
    pub hook_count: usize,
    pub evidence_count: usize,
    pub proof_count: usize,
    pub targets: Vec<SemanticDeploymentTargetReport>,
    pub hooks: Vec<SemanticComplianceHookReport>,
    pub evidence: Vec<SemanticReleaseEvidenceReport>,
    pub proofs: Vec<SemanticDeploymentProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_deployment_suite_report(
    targets: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    hooks: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    evidence: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    proofs: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
) -> SemanticDeploymentSuiteReport {
    let mut target_reports: Vec<_> = targets.iter().map(|item| {
        let preimage = format!("target:{}|kind:{}|environment:{}|artifacts:{}|commands:{}|receipts:{}|forbids:{}|status:{}", item.0, item.1, item.2, sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), item.7);
        SemanticDeploymentTargetReport { id: item.0.clone(), kind: item.1.clone(), environment: item.2.clone(), artifact_count: sorted_count(&item.3), command_count: sorted_count(&item.4), receipt_count: sorted_count(&item.5), forbid_count: sorted_count(&item.6), target_hash: stable_hash_label("lyra.p01.semantic_deployment.target", &preimage) }
    }).collect();
    target_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut hook_reports: Vec<_> = hooks
        .iter()
        .map(|item| {
            let preimage = format!(
                "hook:{}|scope:{}|target:{}|requires:{}|evidence:{}|receipts:{}|status:{}",
                item.0,
                item.1,
                item.2,
                sorted_join(&item.3),
                sorted_join(&item.4),
                sorted_join(&item.5),
                item.6
            );
            SemanticComplianceHookReport {
                id: item.0.clone(),
                scope: item.1.clone(),
                target: item.2.clone(),
                require_count: sorted_count(&item.3),
                evidence_count: sorted_count(&item.4),
                receipt_count: sorted_count(&item.5),
                hook_hash: stable_hash_label("lyra.p01.semantic_deployment.hook", &preimage),
            }
        })
        .collect();
    hook_reports.sort_by(|left, right| left.target.cmp(&right.target).then(left.id.cmp(&right.id)));

    let mut evidence_reports: Vec<_> = evidence
        .iter()
        .map(|item| {
            let preimage = format!(
                "evidence:{}|kind:{}|path:{}|targets:{}|hooks:{}|receipts:{}|commands:{}|status:{}",
                item.0,
                item.1,
                item.2,
                sorted_join(&item.3),
                sorted_join(&item.4),
                sorted_join(&item.5),
                sorted_join(&item.6),
                item.7
            );
            SemanticReleaseEvidenceReport {
                id: item.0.clone(),
                kind: item.1.clone(),
                path: item.2.clone(),
                target_count: sorted_count(&item.3),
                hook_count: sorted_count(&item.4),
                receipt_count: sorted_count(&item.5),
                command_count: sorted_count(&item.6),
                evidence_hash: stable_hash_label(
                    "lyra.p01.semantic_deployment.evidence",
                    &preimage,
                ),
            }
        })
        .collect();
    evidence_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<_> = proofs.iter().map(|item| {
        let preimage = format!("proof:{}|scope:{}|targets:{}|hooks:{}|evidence:{}|receipts:{}|commands:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), sorted_join(&item.7), item.8);
        SemanticDeploymentProofReport { id: item.0.clone(), scope: item.1.clone(), target_count: sorted_count(&item.2), hook_count: sorted_count(&item.3), evidence_count: sorted_count(&item.4), receipt_count: sorted_count(&item.5), command_count: sorted_count(&item.6), forbid_count: sorted_count(&item.7), proof_hash: stable_hash_label("lyra.p01.semantic_deployment.proof", &preimage) }
    }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_rows = Vec::new();
    for item in &target_reports {
        suite_rows.push(format!("target:{}|hash:{}", item.id, item.target_hash));
    }
    for item in &hook_reports {
        suite_rows.push(format!("hook:{}|hash:{}", item.id, item.hook_hash));
    }
    for item in &evidence_reports {
        suite_rows.push(format!("evidence:{}|hash:{}", item.id, item.evidence_hash));
    }
    for item in &proof_reports {
        suite_rows.push(format!("proof:{}|hash:{}", item.id, item.proof_hash));
    }
    suite_rows.sort();

    SemanticDeploymentSuiteReport {
        target_count: target_reports.len(),
        hook_count: hook_reports.len(),
        evidence_count: evidence_reports.len(),
        proof_count: proof_reports.len(),
        targets: target_reports,
        hooks: hook_reports,
        evidence: evidence_reports,
        proofs: proof_reports,
        suite_hash: stable_hash_label("lyra.p01.semantic_deployment.suite", &suite_rows.join("\n")),
    }
}

fn sorted_join(items: &[String]) -> String {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy.join(",")
}
fn sorted_count(items: &[String]) -> usize {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy.len()
}
