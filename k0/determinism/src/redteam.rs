use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedTeamScenarioReport {
    pub id: String,
    pub scenario_kind: String,
    pub path: String,
    pub target_count: usize,
    pub command_count: usize,
    pub rejection_count: usize,
    pub receipt_count: usize,
    pub scenario_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackPathReport {
    pub id: String,
    pub rollback_kind: String,
    pub path: String,
    pub authority: String,
    pub scenario_count: usize,
    pub proof_count: usize,
    pub command_count: usize,
    pub receipt_count: usize,
    pub rollback_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedTeamRollbackSuiteReport {
    pub scenario_count: usize,
    pub rollback_count: usize,
    pub proof_count: usize,
    pub scenario_reports: Vec<RedTeamScenarioReport>,
    pub rollback_reports: Vec<RollbackPathReport>,
    pub suite_hash: String,
}

pub fn deterministic_redteam_rollback_report(
    scenarios: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )],
    rollbacks: &[(
        String,
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )],
    proof_count: usize,
) -> RedTeamRollbackSuiteReport {
    let mut sorted_scenarios = scenarios.to_vec();
    sorted_scenarios.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });
    let mut sorted_rollbacks = rollbacks.to_vec();
    sorted_rollbacks.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
            .then(left.3.cmp(&right.3))
    });

    let mut scenario_reports = Vec::new();
    let mut rollback_reports = Vec::new();
    let mut preimage = format!(
        "scenarios:{}|rollbacks:{}|proofs:{}",
        sorted_scenarios.len(),
        sorted_rollbacks.len(),
        proof_count
    );

    for (id, scenario_kind, path, mut targets, mut commands, mut rejects, mut receipts) in
        sorted_scenarios
    {
        targets.sort();
        commands.sort();
        rejects.sort();
        receipts.sort();
        let scenario_preimage = format!(
            "scenario:{}|kind:{}|path:{}|targets:{}|commands:{}|rejects:{}|receipts:{}",
            id,
            scenario_kind,
            path,
            targets.join(","),
            commands.join(","),
            rejects.join(","),
            receipts.join(",")
        );
        let scenario_hash = stable_hash_label("lyra.p00.redteam.scenario", &scenario_preimage);
        preimage.push('|');
        preimage.push_str(&scenario_preimage);
        scenario_reports.push(RedTeamScenarioReport {
            id,
            scenario_kind,
            path,
            target_count: targets.len(),
            command_count: commands.len(),
            rejection_count: rejects.len(),
            receipt_count: receipts.len(),
            scenario_hash,
        });
    }

    for (
        id,
        rollback_kind,
        path,
        authority,
        mut scenarios,
        mut proofs,
        mut receipts,
        mut commands,
    ) in sorted_rollbacks
    {
        scenarios.sort();
        proofs.sort();
        receipts.sort();
        commands.sort();
        let rollback_preimage = format!(
            "rollback:{}|kind:{}|path:{}|authority:{}|scenarios:{}|proofs:{}|receipts:{}|commands:{}",
            id,
            rollback_kind,
            path,
            authority,
            scenarios.join(","),
            proofs.join(","),
            receipts.join(","),
            commands.join(",")
        );
        let rollback_hash = stable_hash_label("lyra.p00.redteam.rollback", &rollback_preimage);
        preimage.push('|');
        preimage.push_str(&rollback_preimage);
        rollback_reports.push(RollbackPathReport {
            id,
            rollback_kind,
            path,
            authority,
            scenario_count: scenarios.len(),
            proof_count: proofs.len(),
            command_count: commands.len(),
            receipt_count: receipts.len(),
            rollback_hash,
        });
    }

    RedTeamRollbackSuiteReport {
        scenario_count: scenario_reports.len(),
        rollback_count: rollback_reports.len(),
        proof_count,
        scenario_reports,
        rollback_reports,
        suite_hash: stable_hash_label("lyra.p00.redteam.suite", &preimage),
    }
}
