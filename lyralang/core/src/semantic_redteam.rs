use crate::k0_hash::stable_hash_label;

pub const LYRA_P01_SEMANTIC_REDTEAM_CARRIER: &str = "LYRA-P01-SEMANTIC-REDTEAM-CARRIER v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticRedTeamScenarioDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub targets: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub rejects: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticRollbackPathDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub authority: &'static str,
    pub scenarios: &'static [&'static str],
    pub proofs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticRedTeamProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub scenarios: &'static [&'static str],
    pub rollbacks: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRALANG_SEMANTIC_REDTEAM_SCENARIOS: &[SemanticRedTeamScenarioDescriptor] = &[
    SemanticRedTeamScenarioDescriptor {
        id: "canonical_symbol_drift_attack",
        kind: "canonical_symbol",
        path: "fixtures/p01/semantic_redteam_inputs/invalid_canonical_symbol_drift.lyra",
        targets: &["canonical_symbols", "redteam"],
        commands: &["lyra-p01-semantic-redteam-check", "lyra-p01-atom-check"],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        rejects: &["symbol_identity_drift", "canonical_surface_rewrite"],
        status: "artifact_emitted",
    },
    SemanticRedTeamScenarioDescriptor {
        id: "semantic_atom_mutation_attack",
        kind: "semantic_atom",
        path: "fixtures/p01/semantic_redteam_inputs/invalid_semantic_atom_mutation.lyra",
        targets: &["semantic_atoms", "redteam"],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-falsification-check",
        ],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        rejects: &["atom_kind_mutation", "unreceipted_atom_change"],
        status: "artifact_emitted",
    },
    SemanticRedTeamScenarioDescriptor {
        id: "core_ir_upgrade_bypass_attack",
        kind: "core_ir",
        path: "fixtures/p01/semantic_redteam_inputs/invalid_core_ir_upgrade_bypass.lyra",
        targets: &["core_ir", "rollback"],
        commands: &["lyra-p01-semantic-redteam-check", "lyra-p01-ir-check"],
        receipts: &[
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0039_core_ir_reuse.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        rejects: &["unbounded_ir_upgrade", "compatibility_gate_bypass"],
        status: "artifact_emitted",
    },
    SemanticRedTeamScenarioDescriptor {
        id: "receipt_replay_poisoning_attack",
        kind: "replay_poisoning",
        path: "fixtures/p01/semantic_redteam_inputs/invalid_receipt_replay_poisoning.lyra",
        targets: &["receipt_replay", "rollback"],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-replay-check",
        ],
        receipts: &[
            "receipts/p01/pass_0041_semantic_bedrock_receipts.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        rejects: &["orphan_receipt_replay", "receipt_chain_poisoning"],
        status: "artifact_emitted",
    },
    SemanticRedTeamScenarioDescriptor {
        id: "remote_semantic_truth_rewrite_attack",
        kind: "remote_truth",
        path: "fixtures/p01/semantic_redteam_inputs/invalid_remote_truth_rewrite.lyra",
        targets: &["canonical_symbols", "semantic_atoms", "core_ir"],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-deployment-check",
        ],
        receipts: &[
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        rejects: &["remote_truth_rewrite", "network_required"],
        status: "artifact_emitted",
    },
    SemanticRedTeamScenarioDescriptor {
        id: "phase_closure_fraud_attack",
        kind: "closure_fraud",
        path: "fixtures/p01/semantic_redteam_inputs/invalid_phase_closure_claim.lyra",
        targets: &["redteam", "rollback"],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-economics-check",
        ],
        receipts: &[
            "receipts/p01/pass_0051_semantic_economics.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        rejects: &["phase_closure", "global_complete"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_ROLLBACK_PATHS: &[SemanticRollbackPathDescriptor] = &[
    SemanticRollbackPathDescriptor {
        id: "canonical_symbol_receipt_rollback",
        kind: "canonical_symbol",
        path: "products/p01/semantic_redteam_canonical_symbol_rollback.lyra",
        authority: "semantic_constitution",
        scenarios: &["canonical_symbol_drift_attack"],
        proofs: &["semantic_rollback_authority_proof", "receipt_binding_proof"],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &["lyra-p01-semantic-redteam-check", "lyra-p01-atom-check"],
        status: "artifact_emitted",
    },
    SemanticRollbackPathDescriptor {
        id: "semantic_atom_state_rollback",
        kind: "semantic_atom",
        path: "products/p01/semantic_atom_state_rollback.lyra",
        authority: "receipt_chain",
        scenarios: &["semantic_atom_mutation_attack"],
        proofs: &[
            "semantic_rollback_authority_proof",
            "adversarial_semantic_rejection_proof",
        ],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-falsification-check",
        ],
        status: "artifact_emitted",
    },
    SemanticRollbackPathDescriptor {
        id: "core_ir_upgrade_rollback",
        kind: "core_ir",
        path: "products/p01/core_ir_upgrade_rollback.lyra",
        authority: "package_release_law",
        scenarios: &["core_ir_upgrade_bypass_attack"],
        proofs: &["semantic_rollback_authority_proof", "receipt_binding_proof"],
        receipts: &[
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-ir-check",
            "lyra-p01-semantic-packaging-check",
        ],
        status: "artifact_emitted",
    },
    SemanticRollbackPathDescriptor {
        id: "semantic_replay_witness_rollback",
        kind: "replay_witness",
        path: "products/p01/semantic_replay_witness_rollback.lyra",
        authority: "replay_witness",
        scenarios: &["receipt_replay_poisoning_attack"],
        proofs: &[
            "receipt_binding_proof",
            "remote_truth_rewrite_rejection_proof",
        ],
        receipts: &[
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-replay-check",
        ],
        status: "artifact_emitted",
    },
    SemanticRollbackPathDescriptor {
        id: "control_plane_frontier_rollback",
        kind: "control_plane",
        path: "ops/p01/control/semantic_redteam_law.v1.lyra",
        authority: "control_plane",
        scenarios: &[
            "phase_closure_fraud_attack",
            "remote_semantic_truth_rewrite_attack",
        ],
        proofs: &[
            "semantic_redteam_coverage_proof",
            "remote_truth_rewrite_rejection_proof",
        ],
        receipts: &[
            "receipts/p01/pass_0051_semantic_economics.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-economics-check",
        ],
        status: "artifact_emitted",
    },
    SemanticRollbackPathDescriptor {
        id: "challenge_review_rollback",
        kind: "challenge_review",
        path: "examples/p01/redteam/challenge_review_rollback.lyra",
        authority: "challenge_right",
        scenarios: &[
            "canonical_symbol_drift_attack",
            "semantic_atom_mutation_attack",
            "core_ir_upgrade_bypass_attack",
            "receipt_replay_poisoning_attack",
            "remote_semantic_truth_rewrite_attack",
            "phase_closure_fraud_attack",
        ],
        proofs: &[
            "semantic_redteam_coverage_proof",
            "adversarial_semantic_rejection_proof",
            "remote_truth_rewrite_rejection_proof",
            "p01_phase_open",
        ],
        receipts: &[
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-falsification-check",
        ],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_REDTEAM_PROOFS: &[SemanticRedTeamProofDescriptor] = &[
    SemanticRedTeamProofDescriptor {
        id: "semantic_redteam_coverage_proof",
        scope: "redteam",
        scenarios: &[
            "canonical_symbol_drift_attack",
            "semantic_atom_mutation_attack",
            "core_ir_upgrade_bypass_attack",
            "receipt_replay_poisoning_attack",
            "remote_semantic_truth_rewrite_attack",
            "phase_closure_fraud_attack",
        ],
        rollbacks: &[
            "challenge_review_rollback",
            "control_plane_frontier_rollback",
        ],
        receipts: &["receipts/p01/pass_0052_semantic_redteam.receipt"],
        commands: &["lyra-p01-semantic-redteam-check"],
        forbids: &[
            "phase_closure",
            "global_complete",
            "unreceipted_rollback",
            "remote_truth_rewrite",
            "challenge_bypass",
        ],
        status: "artifact_emitted",
    },
    SemanticRedTeamProofDescriptor {
        id: "semantic_rollback_authority_proof",
        scope: "rollback",
        scenarios: &[
            "canonical_symbol_drift_attack",
            "semantic_atom_mutation_attack",
            "core_ir_upgrade_bypass_attack",
        ],
        rollbacks: &[
            "canonical_symbol_receipt_rollback",
            "semantic_atom_state_rollback",
            "core_ir_upgrade_rollback",
        ],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &["lyra-p01-semantic-redteam-check", "lyra-p01-ir-check"],
        forbids: &[
            "phase_closure",
            "global_complete",
            "unreceipted_rollback",
            "remote_truth_rewrite",
            "challenge_bypass",
        ],
        status: "artifact_emitted",
    },
    SemanticRedTeamProofDescriptor {
        id: "receipt_binding_proof",
        scope: "receipt",
        scenarios: &[
            "canonical_symbol_drift_attack",
            "core_ir_upgrade_bypass_attack",
            "receipt_replay_poisoning_attack",
        ],
        rollbacks: &[
            "canonical_symbol_receipt_rollback",
            "core_ir_upgrade_rollback",
            "semantic_replay_witness_rollback",
        ],
        receipts: &[
            "receipts/p01/pass_0041_semantic_bedrock_receipts.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-replay-check",
        ],
        forbids: &[
            "phase_closure",
            "global_complete",
            "unreceipted_rollback",
            "remote_truth_rewrite",
            "challenge_bypass",
        ],
        status: "artifact_emitted",
    },
    SemanticRedTeamProofDescriptor {
        id: "adversarial_semantic_rejection_proof",
        scope: "adversarial",
        scenarios: &[
            "semantic_atom_mutation_attack",
            "core_ir_upgrade_bypass_attack",
            "receipt_replay_poisoning_attack",
        ],
        rollbacks: &["semantic_atom_state_rollback", "challenge_review_rollback"],
        receipts: &[
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-falsification-check",
        ],
        forbids: &[
            "phase_closure",
            "global_complete",
            "unreceipted_rollback",
            "remote_truth_rewrite",
            "challenge_bypass",
        ],
        status: "artifact_emitted",
    },
    SemanticRedTeamProofDescriptor {
        id: "remote_truth_rewrite_rejection_proof",
        scope: "remote_truth",
        scenarios: &[
            "remote_semantic_truth_rewrite_attack",
            "receipt_replay_poisoning_attack",
        ],
        rollbacks: &[
            "semantic_replay_witness_rollback",
            "control_plane_frontier_rollback",
            "challenge_review_rollback",
        ],
        receipts: &[
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-redteam-check",
            "lyra-p01-semantic-deployment-check",
        ],
        forbids: &[
            "phase_closure",
            "global_complete",
            "unreceipted_rollback",
            "remote_truth_rewrite",
            "challenge_bypass",
        ],
        status: "artifact_emitted",
    },
    SemanticRedTeamProofDescriptor {
        id: "p01_phase_open",
        scope: "phase",
        scenarios: &[
            "phase_closure_fraud_attack",
            "remote_semantic_truth_rewrite_attack",
        ],
        rollbacks: &[
            "control_plane_frontier_rollback",
            "challenge_review_rollback",
        ],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0032_semantic_objects.receipt",
            "receipts/p01/pass_0033_semantic_identity.receipt",
            "receipts/p01/pass_0034_reference_semantics.receipt",
            "receipts/p01/pass_0035_symbolic_equality.receipt",
            "receipts/p01/pass_0036_error_challenge_evidence.receipt",
            "receipts/p01/pass_0037_semantic_serialization_hashing.receipt",
            "receipts/p01/pass_0038_semantic_adversarial_corpus.receipt",
            "receipts/p01/pass_0039_core_ir_reuse.receipt",
            "receipts/p01/pass_0040_semantic_atom_reference.receipt",
            "receipts/p01/pass_0041_semantic_bedrock_receipts.receipt",
            "receipts/p01/pass_0042_formal_semantic_constitution.receipt",
            "receipts/p01/pass_0043_canonical_data_model.receipt",
            "receipts/p01/pass_0044_semantic_core_engine.receipt",
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
        ],
        commands: &["lyra-p01-semantic-redteam-check"],
        forbids: &[
            "phase_closure",
            "global_complete",
            "unreceipted_rollback",
            "remote_truth_rewrite",
            "challenge_bypass",
        ],
        status: "blocked",
    },
];

pub fn semantic_redteam_scenario_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_REDTEAM_SCENARIOS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_rollback_path_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ROLLBACK_PATHS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_redteam_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_REDTEAM_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_redteam_scenario_descriptor(
    id: &str,
) -> Option<&'static SemanticRedTeamScenarioDescriptor> {
    LYRALANG_SEMANTIC_REDTEAM_SCENARIOS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_rollback_path_descriptor(
    id: &str,
) -> Option<&'static SemanticRollbackPathDescriptor> {
    LYRALANG_SEMANTIC_ROLLBACK_PATHS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_redteam_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticRedTeamProofDescriptor> {
    LYRALANG_SEMANTIC_REDTEAM_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_redteam_scenario_signature(item: &SemanticRedTeamScenarioDescriptor) -> String {
    format!(
        "scenario:{}|kind:{}|path:{}|targets:{}|commands:{}|rejects:{}|receipts:{}|status:{}",
        item.id,
        item.kind,
        item.path,
        sorted_join(item.targets),
        sorted_join(item.commands),
        sorted_join(item.rejects),
        sorted_join(item.receipts),
        item.status
    )
}

pub fn semantic_rollback_path_signature(item: &SemanticRollbackPathDescriptor) -> String {
    format!("rollback:{}|kind:{}|path:{}|authority:{}|scenarios:{}|proofs:{}|receipts:{}|commands:{}|status:{}", item.id, item.kind, item.path, item.authority, sorted_join(item.scenarios), sorted_join(item.proofs), sorted_join(item.receipts), sorted_join(item.commands), item.status)
}

pub fn semantic_redteam_proof_signature(item: &SemanticRedTeamProofDescriptor) -> String {
    format!(
        "proof:{}|scope:{}|scenarios:{}|rollbacks:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
        item.id,
        item.scope,
        sorted_join(item.scenarios),
        sorted_join(item.rollbacks),
        sorted_join(item.receipts),
        sorted_join(item.commands),
        sorted_join(item.forbids),
        item.status
    )
}

pub fn semantic_redteam_scenario_digest(item: &SemanticRedTeamScenarioDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_redteam.scenario",
        &semantic_redteam_scenario_signature(item),
    )
}
pub fn semantic_rollback_path_digest(item: &SemanticRollbackPathDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_redteam.rollback",
        &semantic_rollback_path_signature(item),
    )
}
pub fn semantic_redteam_proof_digest(item: &SemanticRedTeamProofDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_redteam.proof",
        &semantic_redteam_proof_signature(item),
    )
}

pub fn semantic_redteam_registry_signature() -> String {
    let mut rows = Vec::new();
    for scenario in LYRALANG_SEMANTIC_REDTEAM_SCENARIOS {
        rows.push(format!(
            "scenario:{}|{}",
            scenario.id,
            semantic_redteam_scenario_digest(scenario)
        ));
    }
    for rollback in LYRALANG_SEMANTIC_ROLLBACK_PATHS {
        rows.push(format!(
            "rollback:{}|{}",
            rollback.id,
            semantic_rollback_path_digest(rollback)
        ));
    }
    for proof in LYRALANG_SEMANTIC_REDTEAM_PROOFS {
        rows.push(format!(
            "proof:{}|{}",
            proof.id,
            semantic_redteam_proof_digest(proof)
        ));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_redteam_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_redteam.registry",
        &semantic_redteam_registry_signature(),
    )
}

pub fn semantic_redteam_scenarios_bind_rollbacks() -> bool {
    let scenario_ids = semantic_redteam_scenario_ids();
    LYRALANG_SEMANTIC_ROLLBACK_PATHS.iter().all(|rollback| {
        rollback
            .scenarios
            .iter()
            .all(|id| scenario_ids.contains(id))
    })
}

pub fn semantic_redteam_rollbacks_bind_proofs() -> bool {
    let proof_ids = semantic_redteam_proof_ids();
    LYRALANG_SEMANTIC_ROLLBACK_PATHS
        .iter()
        .all(|rollback| rollback.proofs.iter().all(|id| proof_ids.contains(id)))
}

pub fn semantic_redteam_proofs_bind_registry() -> bool {
    let scenario_ids = semantic_redteam_scenario_ids();
    let rollback_ids = semantic_rollback_path_ids();
    LYRALANG_SEMANTIC_REDTEAM_PROOFS.iter().all(|proof| {
        proof.scenarios.iter().all(|id| scenario_ids.contains(id))
            && proof.rollbacks.iter().all(|id| rollback_ids.contains(id))
    })
}

pub fn semantic_redteam_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_REDTEAM_SCENARIOS.iter().all(|scenario| {
        scenario.path.starts_with("fixtures/p01/")
            || scenario.path.starts_with("examples/p01/")
            || scenario.path.starts_with("products/p01/")
    }) && LYRALANG_SEMANTIC_ROLLBACK_PATHS.iter().all(|rollback| {
        rollback.path.starts_with("products/p01/")
            || rollback.path.starts_with("examples/p01/")
            || rollback.path.starts_with("ops/p01/")
            || rollback.path.starts_with("docs/p01/")
    })
}

pub fn semantic_redteam_receipts_cover_p01_001_through_p01_023() -> bool {
    let required = [
        "pass_0030_semantic_atoms",
        "pass_0031_core_ir",
        "pass_0032_semantic_objects",
        "pass_0033_semantic_identity",
        "pass_0034_reference_semantics",
        "pass_0035_symbolic_equality",
        "pass_0036_error_challenge_evidence",
        "pass_0037_semantic_serialization_hashing",
        "pass_0038_semantic_adversarial_corpus",
        "pass_0039_core_ir_reuse",
        "pass_0040_semantic_atom_reference",
        "pass_0041_semantic_bedrock_receipts",
        "pass_0042_formal_semantic_constitution",
        "pass_0043_canonical_data_model",
        "pass_0044_semantic_core_engine",
        "pass_0045_semantic_falsification",
        "pass_0046_semantic_replay",
        "pass_0047_semantic_interface",
        "pass_0048_semantic_packaging",
        "pass_0049_semantic_deployment",
        "pass_0050_semantic_ecosystem",
        "pass_0051_semantic_economics",
        "pass_0052_semantic_redteam",
    ];
    let mut receipts = Vec::new();
    for scenario in LYRALANG_SEMANTIC_REDTEAM_SCENARIOS {
        receipts.extend_from_slice(scenario.receipts);
    }
    for rollback in LYRALANG_SEMANTIC_ROLLBACK_PATHS {
        receipts.extend_from_slice(rollback.receipts);
    }
    for proof in LYRALANG_SEMANTIC_REDTEAM_PROOFS {
        receipts.extend_from_slice(proof.receipts);
    }
    required
        .iter()
        .all(|needle| receipts.iter().any(|receipt| receipt.contains(needle)))
}

pub fn semantic_redteam_no_forbidden_descriptor_claims() -> bool {
    let signature = semantic_redteam_registry_signature().to_ascii_lowercase();
    !signature.contains("network required")
        && !signature.contains("cloud required")
        && !signature.contains("online required")
        && !signature.contains("remote fetch")
        && !signature.contains("remote truth rewrite allowed")
        && !signature.contains("rollback without receipt")
        && !signature.contains("challenge bypass allowed")
        && !signature.contains("phase closed")
        && !signature.contains("global complete")
}

fn sorted_join(items: &[&'static str]) -> String {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy.join(",")
}
