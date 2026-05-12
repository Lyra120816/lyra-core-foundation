use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRedTeamScenarioReport {
    pub id: String,
    pub attack_kind: String,
    pub path: String,
    pub target_count: usize,
    pub rollback_count: usize,
    pub command_count: usize,
    pub receipt_count: usize,
    pub reject_count: usize,
    pub scenario_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRollbackPathReport {
    pub id: String,
    pub rollback_kind: String,
    pub path: String,
    pub trigger_count: usize,
    pub restore_count: usize,
    pub receipt_count: usize,
    pub command_count: usize,
    pub challenge_right_count: usize,
    pub rollback_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRedTeamProofReport {
    pub id: String,
    pub scope: String,
    pub scenario_count: usize,
    pub rollback_count: usize,
    pub receipt_count: usize,
    pub command_count: usize,
    pub forbid_count: usize,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRedTeamSuiteReport {
    pub scenario_count: usize,
    pub rollback_count: usize,
    pub proof_count: usize,
    pub scenario_reports: Vec<BootstrapRedTeamScenarioReport>,
    pub rollback_reports: Vec<BootstrapRollbackPathReport>,
    pub proof_reports: Vec<BootstrapRedTeamProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_bootstrap_redteam_suite_report(
    scenarios: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    rollbacks: &[(
        String,
        String,
        String,
        Vec<String>,
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
        String,
    )],
) -> BootstrapRedTeamSuiteReport {
    let mut scenario_reports: Vec<_> = scenarios.iter().map(|item| {
        let preimage = format!(
            "scenario:{}|kind:{}|path:{}|targets:{}|rollbacks:{}|commands:{}|receipts:{}|rejects:{}|status:{}",
            item.0, item.1, item.2, sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), sorted_join(&item.7), item.8
        );
        BootstrapRedTeamScenarioReport {
            id: item.0.clone(), attack_kind: item.1.clone(), path: item.2.clone(),
            target_count: sorted_count(&item.3), rollback_count: sorted_count(&item.4), command_count: sorted_count(&item.5),
            receipt_count: sorted_count(&item.6), reject_count: sorted_count(&item.7),
            scenario_hash: stable_hash_label("lyra.p02.bootstrap_redteam.scenario", &preimage),
        }
    }).collect();
    scenario_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut rollback_reports: Vec<_> = rollbacks.iter().map(|item| {
        let preimage = format!(
            "rollback:{}|kind:{}|path:{}|triggers:{}|restores:{}|receipts:{}|commands:{}|rights:{}|status:{}",
            item.0, item.1, item.2, sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), sorted_join(&item.7), item.8
        );
        BootstrapRollbackPathReport {
            id: item.0.clone(), rollback_kind: item.1.clone(), path: item.2.clone(),
            trigger_count: sorted_count(&item.3), restore_count: sorted_count(&item.4), receipt_count: sorted_count(&item.5),
            command_count: sorted_count(&item.6), challenge_right_count: sorted_count(&item.7),
            rollback_hash: stable_hash_label("lyra.p02.bootstrap_redteam.rollback", &preimage),
        }
    }).collect();
    rollback_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<_> = proofs.iter().map(|item| {
        let preimage = format!(
            "proof:{}|scope:{}|scenarios:{}|rollbacks:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
            item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), item.7
        );
        BootstrapRedTeamProofReport {
            id: item.0.clone(), scope: item.1.clone(), scenario_count: sorted_count(&item.2), rollback_count: sorted_count(&item.3),
            receipt_count: sorted_count(&item.4), command_count: sorted_count(&item.5), forbid_count: sorted_count(&item.6),
            proof_hash: stable_hash_label("lyra.p02.bootstrap_redteam.proof", &preimage),
        }
    }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut rows = Vec::new();
    for item in &scenario_reports {
        rows.push(format!("scenario:{}|hash:{}", item.id, item.scenario_hash));
    }
    for item in &rollback_reports {
        rows.push(format!("rollback:{}|hash:{}", item.id, item.rollback_hash));
    }
    for item in &proof_reports {
        rows.push(format!("proof:{}|hash:{}", item.id, item.proof_hash));
    }
    rows.sort();

    BootstrapRedTeamSuiteReport {
        scenario_count: scenario_reports.len(),
        rollback_count: rollback_reports.len(),
        proof_count: proof_reports.len(),
        scenario_reports,
        rollback_reports,
        proof_reports,
        suite_hash: stable_hash_label("lyra.p02.bootstrap_redteam.suite", &rows.join("\n")),
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
