use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapRedTeamScenarioDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub targets: &'static [&'static str],
    pub rollbacks: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub rejects: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapRollbackPathDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub triggers: &'static [&'static str],
    pub restores: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub challenge_rights: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapRedTeamProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub scenarios: &'static [&'static str],
    pub rollbacks: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_REDTEAM_CARRIER: &str = "lyra.p02.bootstrap_redteam.carrier.v1";

pub const LYRALANG_BOOTSTRAP_REDTEAM_SCENARIOS: &[BootstrapRedTeamScenarioDescriptor] = &[
    BootstrapRedTeamScenarioDescriptor {
        id: "ambient_host_dependency_attack",
        kind: "host_dependency",
        path: "examples/p02/redteam/ambient_host_dependency_attack.v1.lyra",
        targets: &[
            "bootstrap_trust",
            "host_extinction_framework",
            "challenge_right",
        ],
        rollbacks: &[
            "host_dependency_quarantine_rollback",
            "extinction_ledger_reseal",
        ],
        commands: &[
            "lyra-p02-host-boundary-check",
            "lyra-p02-bootstrap-extinction-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        receipts: &[
            "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
            "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        rejects: &["ambient_host_dependency", "unledgered_foreign_surface"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamScenarioDescriptor {
        id: "remote_truth_rewrite_attack",
        kind: "remote_truth_rewrite",
        path: "examples/p02/redteam/remote_truth_rewrite_attack.v1.lyra",
        targets: &["bootstrap_trust", "remote_truth_rewrite", "receipt_replay"],
        rollbacks: &[
            "remote_import_rejection_rollback",
            "deployment_packet_replay_rollback",
        ],
        commands: &[
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        rejects: &["remote_truth_rewrite", "unreceipted_import"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamScenarioDescriptor {
        id: "seed_runtime_drift_attack",
        kind: "seed_runtime_drift",
        path: "examples/p02/redteam/seed_runtime_drift_attack.v1.lyra",
        targets: &["seed_runtime_law", "bootstrap_trust", "rollback"],
        rollbacks: &[
            "seed_runtime_last_good_restore",
            "phase_open_reassertion_rollback",
        ],
        commands: &[
            "lyra-p02-seed-runtime-replacement-check",
            "lyra-p02-bootstrap-core-engine-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        receipts: &[
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        rejects: &["seed_runtime_drift", "unreceipted_runtime_promotion"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamScenarioDescriptor {
        id: "extinction_ledger_bypass_attack",
        kind: "extinction_bypass",
        path: "examples/p02/redteam/extinction_ledger_bypass_attack.v1.lyra",
        targets: &["host_extinction_framework", "rollback", "challenge_right"],
        rollbacks: &[
            "extinction_ledger_reseal",
            "host_dependency_quarantine_rollback",
        ],
        commands: &[
            "lyra-p02-bootstrap-extinction-check",
            "lyra-p02-foreign-surface-closure-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        rejects: &["extinction_ledger_bypass", "hidden_host_helper"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamScenarioDescriptor {
        id: "deployment_receipt_replay_attack",
        kind: "receipt_replay",
        path: "examples/p02/redteam/deployment_receipt_replay_attack.v1.lyra",
        targets: &["receipt_replay", "bootstrap_trust", "rollback"],
        rollbacks: &[
            "deployment_packet_replay_rollback",
            "remote_import_rejection_rollback",
        ],
        commands: &[
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        rejects: &["stale_receipt_replay", "deployment_digest_drift"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamScenarioDescriptor {
        id: "economics_capture_attack",
        kind: "economics_capture",
        path: "examples/p02/redteam/economics_capture_attack.v1.lyra",
        targets: &["economics_capture", "public_interest", "challenge_right"],
        rollbacks: &[
            "economics_capture_reversal",
            "phase_open_reassertion_rollback",
        ],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        receipts: &[
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        rejects: &["capture", "extractive_default"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamScenarioDescriptor {
        id: "closure_premature_claim_attack",
        kind: "closure_premature",
        path: "examples/p02/redteam/closure_premature_claim_attack.v1.lyra",
        targets: &["phase_open", "rollback", "bootstrap_trust"],
        rollbacks: &[
            "phase_open_reassertion_rollback",
            "seed_runtime_last_good_restore",
        ],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        receipts: &[
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        rejects: &["phase_closure", "global_complete"],
        status: "blocked",
    },
];

pub const LYRALANG_BOOTSTRAP_ROLLBACK_PATHS: &[BootstrapRollbackPathDescriptor] = &[
    BootstrapRollbackPathDescriptor {
        id: "host_dependency_quarantine_rollback",
        kind: "quarantine",
        path: "ops/p02/redteam/host_dependency_quarantine_rollback.v1.lyra",
        triggers: &["ambient_host_dependency", "hidden_host_helper"],
        restores: &["host_extinction_framework", "bootstrap_trust"],
        receipts: &[
            "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-host-boundary-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        challenge_rights: &["operator_challenge", "deterministic_replay_challenge"],
        status: "artifact_emitted",
    },
    BootstrapRollbackPathDescriptor {
        id: "remote_import_rejection_rollback",
        kind: "import_rejection",
        path: "ops/p02/redteam/remote_import_rejection_rollback.v1.lyra",
        triggers: &["remote_truth_rewrite", "unreceipted_import"],
        restores: &["bootstrap_trust", "remote_truth_rewrite"],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        challenge_rights: &["operator_challenge", "receipt_chain_challenge"],
        status: "artifact_emitted",
    },
    BootstrapRollbackPathDescriptor {
        id: "seed_runtime_last_good_restore",
        kind: "last_good_restore",
        path: "ops/p02/redteam/seed_runtime_last_good_restore.v1.lyra",
        triggers: &["seed_runtime_drift", "unreceipted_runtime_promotion"],
        restores: &["seed_runtime_law", "bootstrap_trust"],
        receipts: &[
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-seed-runtime-replacement-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        challenge_rights: &["operator_challenge", "runtime_promotion_challenge"],
        status: "artifact_emitted",
    },
    BootstrapRollbackPathDescriptor {
        id: "extinction_ledger_reseal",
        kind: "ledger_reseal",
        path: "ops/p02/redteam/extinction_ledger_reseal.v1.lyra",
        triggers: &["extinction_ledger_bypass", "unledgered_foreign_surface"],
        restores: &["host_extinction_framework", "challenge_right"],
        receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-extinction-check",
            "lyra-p02-foreign-surface-closure-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        challenge_rights: &["operator_challenge", "foreign_surface_challenge"],
        status: "artifact_emitted",
    },
    BootstrapRollbackPathDescriptor {
        id: "deployment_packet_replay_rollback",
        kind: "packet_replay",
        path: "ops/p02/redteam/deployment_packet_replay_rollback.v1.lyra",
        triggers: &["stale_receipt_replay", "deployment_digest_drift"],
        restores: &["receipt_replay", "bootstrap_trust"],
        receipts: &[
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        challenge_rights: &["operator_challenge", "deployment_digest_challenge"],
        status: "artifact_emitted",
    },
    BootstrapRollbackPathDescriptor {
        id: "economics_capture_reversal",
        kind: "capture_reversal",
        path: "ops/p02/redteam/economics_capture_reversal.v1.lyra",
        triggers: &["capture", "extractive_default"],
        restores: &["economics_capture", "public_interest"],
        receipts: &[
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        challenge_rights: &["operator_challenge", "public_interest_challenge"],
        status: "artifact_emitted",
    },
    BootstrapRollbackPathDescriptor {
        id: "phase_open_reassertion_rollback",
        kind: "phase_open_reassertion",
        path: "ops/p02/redteam/phase_open_reassertion_rollback.v1.lyra",
        triggers: &["phase_closure", "global_complete"],
        restores: &["phase_open", "rollback"],
        receipts: &[
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        challenge_rights: &["operator_challenge", "closure_gate_challenge"],
        status: "blocked",
    },
];

pub const LYRALANG_BOOTSTRAP_REDTEAM_PROOFS: &[BootstrapRedTeamProofDescriptor] = &[
    BootstrapRedTeamProofDescriptor {
        id: "redteam_coverage_proof",
        scope: "redteam",
        scenarios: &[
            "ambient_host_dependency_attack",
            "remote_truth_rewrite_attack",
            "seed_runtime_drift_attack",
            "extinction_ledger_bypass_attack",
            "deployment_receipt_replay_attack",
            "economics_capture_attack",
            "closure_premature_claim_attack",
        ],
        rollbacks: &[
            "host_dependency_quarantine_rollback",
            "remote_import_rejection_rollback",
            "seed_runtime_last_good_restore",
        ],
        receipts: &[
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-falsification-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        forbids: &["phase_closure", "remote_truth_rewrite", "challenge_bypass"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamProofDescriptor {
        id: "rollback_receipt_binding_proof",
        scope: "rollback",
        scenarios: &[
            "seed_runtime_drift_attack",
            "deployment_receipt_replay_attack",
            "closure_premature_claim_attack",
        ],
        rollbacks: &[
            "host_dependency_quarantine_rollback",
            "remote_import_rejection_rollback",
            "seed_runtime_last_good_restore",
            "extinction_ledger_reseal",
            "deployment_packet_replay_rollback",
            "economics_capture_reversal",
            "phase_open_reassertion_rollback",
        ],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        forbids: &["unreceipted_rollback", "phase_closure", "global_complete"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamProofDescriptor {
        id: "remote_truth_rewrite_rejection_proof",
        scope: "remote_truth",
        scenarios: &[
            "remote_truth_rewrite_attack",
            "deployment_receipt_replay_attack",
        ],
        rollbacks: &[
            "remote_import_rejection_rollback",
            "deployment_packet_replay_rollback",
        ],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        forbids: &[
            "remote_truth_rewrite",
            "remote_consensus_override",
            "phase_closure",
        ],
        status: "artifact_emitted",
    },
    BootstrapRedTeamProofDescriptor {
        id: "challenge_right_enforcement_proof",
        scope: "challenge",
        scenarios: &[
            "ambient_host_dependency_attack",
            "extinction_ledger_bypass_attack",
            "economics_capture_attack",
        ],
        rollbacks: &[
            "host_dependency_quarantine_rollback",
            "extinction_ledger_reseal",
            "economics_capture_reversal",
        ],
        receipts: &[
            "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-host-boundary-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        forbids: &["challenge_bypass", "retaliation", "phase_closure"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamProofDescriptor {
        id: "economics_capture_redteam_bridge_proof",
        scope: "economics_bridge",
        scenarios: &["economics_capture_attack", "closure_premature_claim_attack"],
        rollbacks: &[
            "economics_capture_reversal",
            "phase_open_reassertion_rollback",
        ],
        receipts: &[
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-redteam-check",
        ],
        forbids: &["capture", "extractive_default", "phase_closure"],
        status: "artifact_emitted",
    },
    BootstrapRedTeamProofDescriptor {
        id: "p02_phase_open",
        scope: "phase",
        scenarios: &["closure_premature_claim_attack"],
        rollbacks: &["phase_open_reassertion_rollback"],
        receipts: &[
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        ],
        commands: &["lyra-p02-bootstrap-redteam-check"],
        forbids: &["phase_closure", "global_complete", "remote_truth_rewrite"],
        status: "blocked",
    },
];

pub fn bootstrap_redteam_scenario_descriptor(
    id: &str,
) -> Option<&'static BootstrapRedTeamScenarioDescriptor> {
    LYRALANG_BOOTSTRAP_REDTEAM_SCENARIOS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_rollback_path_descriptor(
    id: &str,
) -> Option<&'static BootstrapRollbackPathDescriptor> {
    LYRALANG_BOOTSTRAP_ROLLBACK_PATHS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_redteam_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapRedTeamProofDescriptor> {
    LYRALANG_BOOTSTRAP_REDTEAM_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_redteam_scenario_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_REDTEAM_SCENARIOS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_rollback_path_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_ROLLBACK_PATHS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_redteam_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_REDTEAM_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_redteam_scenario_digest(id: &str) -> Option<String> {
    bootstrap_redteam_scenario_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_redteam.scenario",
            &bootstrap_redteam_scenario_signature(item),
        )
    })
}
pub fn bootstrap_rollback_path_digest(id: &str) -> Option<String> {
    bootstrap_rollback_path_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_redteam.rollback",
            &bootstrap_rollback_path_signature(item),
        )
    })
}
pub fn bootstrap_redteam_proof_digest(id: &str) -> Option<String> {
    bootstrap_redteam_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_redteam.proof",
            &bootstrap_redteam_proof_signature(item),
        )
    })
}

pub fn bootstrap_redteam_scenario_signature(item: &BootstrapRedTeamScenarioDescriptor) -> String {
    format!("scenario:{}|kind:{}|path:{}|targets:{}|rollbacks:{}|commands:{}|receipts:{}|rejects:{}|status:{}", item.id, item.kind, item.path, item.targets.join(","), item.rollbacks.join(","), item.commands.join(","), item.receipts.join(","), item.rejects.join(","), item.status)
}

pub fn bootstrap_rollback_path_signature(item: &BootstrapRollbackPathDescriptor) -> String {
    format!("rollback:{}|kind:{}|path:{}|triggers:{}|restores:{}|receipts:{}|commands:{}|rights:{}|status:{}", item.id, item.kind, item.path, item.triggers.join(","), item.restores.join(","), item.receipts.join(","), item.commands.join(","), item.challenge_rights.join(","), item.status)
}

pub fn bootstrap_redteam_proof_signature(item: &BootstrapRedTeamProofDescriptor) -> String {
    format!(
        "proof:{}|scope:{}|scenarios:{}|rollbacks:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
        item.id,
        item.scope,
        item.scenarios.join(","),
        item.rollbacks.join(","),
        item.receipts.join(","),
        item.commands.join(","),
        item.forbids.join(","),
        item.status
    )
}

pub fn bootstrap_redteam_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_BOOTSTRAP_REDTEAM_SCENARIOS {
        rows.push(bootstrap_redteam_scenario_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_ROLLBACK_PATHS {
        rows.push(bootstrap_rollback_path_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_REDTEAM_PROOFS {
        rows.push(bootstrap_redteam_proof_signature(item));
    }
    rows.sort();
    format!(
        "carrier:{}
{}",
        LYRA_P02_BOOTSTRAP_REDTEAM_CARRIER,
        rows.join(
            "
"
        )
    )
}

pub fn bootstrap_redteam_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_redteam.registry",
        &bootstrap_redteam_registry_signature(),
    )
}
pub fn bootstrap_redteam_carrier_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_redteam.carrier",
        &bootstrap_redteam_registry_hash(),
    )
}

pub fn bootstrap_redteam_scenarios_bind_rollbacks() -> bool {
    LYRALANG_BOOTSTRAP_REDTEAM_SCENARIOS.iter().all(|scenario| {
        !scenario.rollbacks.is_empty()
            && scenario
                .rollbacks
                .iter()
                .all(|rollback| bootstrap_rollback_path_descriptor(rollback).is_some())
    })
}

pub fn bootstrap_redteam_rollbacks_bind_challenge_rights() -> bool {
    LYRALANG_BOOTSTRAP_ROLLBACK_PATHS.iter().all(|rollback| {
        !rollback.challenge_rights.is_empty()
            && !rollback.receipts.is_empty()
            && !rollback.commands.is_empty()
    })
}

pub fn bootstrap_redteam_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_REDTEAM_PROOFS.iter().all(|proof| {
        !proof.scenarios.is_empty()
            && !proof.rollbacks.is_empty()
            && !proof.receipts.is_empty()
            && !proof.commands.is_empty()
            && proof
                .scenarios
                .iter()
                .all(|scenario| bootstrap_redteam_scenario_descriptor(scenario).is_some())
            && proof
                .rollbacks
                .iter()
                .all(|rollback| bootstrap_rollback_path_descriptor(rollback).is_some())
    })
}

pub fn bootstrap_redteam_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_REDTEAM_SCENARIOS
        .iter()
        .all(|scenario| allowed_artifact_path(scenario.path))
        && LYRALANG_BOOTSTRAP_ROLLBACK_PATHS
            .iter()
            .all(|rollback| allowed_artifact_path(rollback.path))
}

pub fn bootstrap_redteam_no_forbidden_descriptor_claims() -> bool {
    let lowered = bootstrap_redteam_registry_signature().to_ascii_lowercase();
    ![
        "network required",
        "cloud required",
        "online required",
        "remote service required",
        "remote fetch",
        "remote truth rewrite allowed",
        "challenge bypass",
        "unreceipted rollback",
        "rollback unreceipted",
        "redteam drift accepted",
        "corpus drift accepted",
        "phase closed",
        "global complete",
        "todo",
        "placeholder",
        "best effort",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

pub fn bootstrap_redteam_receipts_cover_p02_001_through_p02_023() -> bool {
    let signature = bootstrap_redteam_registry_signature();
    [
        "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
        "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
        "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
        "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
        "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        "receipts/p02/pass_0075_bootstrap_replay.receipt",
        "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        "receipts/p02/pass_0080_bootstrap_economics.receipt",
        "receipts/p02/pass_0081_bootstrap_redteam.receipt",
    ]
    .iter()
    .all(|needle| signature.contains(needle))
}

fn allowed_artifact_path(path: &str) -> bool {
    [
        "docs/",
        "examples/",
        "products/",
        "fixtures/",
        "receipts/",
        "ops/",
        "interfaces/",
        "src/",
        "tests/",
        "shells/",
        "goldens/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
        && !path.contains("..")
}
