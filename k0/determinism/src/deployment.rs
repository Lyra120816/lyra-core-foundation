use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentTargetReport {
    pub id: String,
    pub kind: String,
    pub environment: String,
    pub artifact_count: usize,
    pub command_count: usize,
    pub receipt_count: usize,
    pub target_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplianceHookReport {
    pub id: String,
    pub scope: String,
    pub target: String,
    pub require_count: usize,
    pub evidence_count: usize,
    pub receipt_count: usize,
    pub hook_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentSuiteReport {
    pub target_count: usize,
    pub hook_count: usize,
    pub evidence_count: usize,
    pub proof_count: usize,
    pub target_reports: Vec<DeploymentTargetReport>,
    pub hook_reports: Vec<ComplianceHookReport>,
    pub suite_hash: String,
}

pub fn deterministic_deployment_suite_report(
    targets: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )],
    hooks: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )],
    evidence_count: usize,
    proof_count: usize,
) -> DeploymentSuiteReport {
    let mut sorted_targets = targets.to_vec();
    sorted_targets.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });
    let mut sorted_hooks = hooks.to_vec();
    sorted_hooks.sort_by(|left, right| {
        left.2
            .cmp(&right.2)
            .then(left.0.cmp(&right.0))
            .then(left.1.cmp(&right.1))
    });

    let mut target_reports = Vec::new();
    let mut hook_reports = Vec::new();
    let mut preimage = format!(
        "targets:{}|hooks:{}|evidence:{}|proofs:{}",
        sorted_targets.len(),
        sorted_hooks.len(),
        evidence_count,
        proof_count
    );

    for (id, kind, environment, mut artifacts, mut commands, mut receipts) in sorted_targets {
        artifacts.sort();
        commands.sort();
        receipts.sort();
        let target_preimage = format!(
            "target:{}|kind:{}|environment:{}|artifacts:{}|commands:{}|receipts:{}",
            id,
            kind,
            environment,
            artifacts.join(","),
            commands.join(","),
            receipts.join(",")
        );
        let target_hash = stable_hash_label("lyra.p00.deployment.target", &target_preimage);
        preimage.push('|');
        preimage.push_str(&target_preimage);
        target_reports.push(DeploymentTargetReport {
            id,
            kind,
            environment,
            artifact_count: artifacts.len(),
            command_count: commands.len(),
            receipt_count: receipts.len(),
            target_hash,
        });
    }

    for (id, scope, target, mut requires, mut evidence, mut receipts) in sorted_hooks {
        requires.sort();
        evidence.sort();
        receipts.sort();
        let hook_preimage = format!(
            "hook:{}|scope:{}|target:{}|requires:{}|evidence:{}|receipts:{}",
            id,
            scope,
            target,
            requires.join(","),
            evidence.join(","),
            receipts.join(",")
        );
        let hook_hash = stable_hash_label("lyra.p00.deployment.hook", &hook_preimage);
        preimage.push('|');
        preimage.push_str(&hook_preimage);
        hook_reports.push(ComplianceHookReport {
            id,
            scope,
            target,
            require_count: requires.len(),
            evidence_count: evidence.len(),
            receipt_count: receipts.len(),
            hook_hash,
        });
    }

    DeploymentSuiteReport {
        target_count: target_reports.len(),
        hook_count: hook_reports.len(),
        evidence_count,
        proof_count,
        target_reports,
        hook_reports,
        suite_hash: stable_hash_label("lyra.p00.deployment.suite", &preimage),
    }
}
