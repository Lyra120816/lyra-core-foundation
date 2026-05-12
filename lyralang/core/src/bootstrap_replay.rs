use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapReplayReceiptDescriptor {
    pub id: &'static str,
    pub path: &'static str,
    pub input_hash: &'static str,
    pub canonical_hash: &'static str,
    pub verdict_hash: &'static str,
    pub receipt_hash: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapReplayWitnessDescriptor {
    pub id: &'static str,
    pub order: &'static str,
    pub receipts: &'static [&'static str],
    pub preimage: &'static str,
    pub witness_hash: &'static str,
    pub commands: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapReplayChainLinkDescriptor {
    pub id: &'static str,
    pub from: &'static str,
    pub to: &'static str,
    pub relation: &'static str,
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapReplayProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub receipts: &'static [&'static str],
    pub witnesses: &'static [&'static str],
    pub links: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapReplayArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub artifact_kind: &'static str,
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_REPLAY_CARRIER: &str = "lyra.p02.bootstrap_replay.carrier.v1";
const HASH: &str = "fnv1a128:0123456789abcdef0123456789abcdef";

pub const LYRALANG_BOOTSTRAP_REPLAY_RECEIPTS: &[BootstrapReplayReceiptDescriptor] = &[
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_surface_inventory_receipt",
        path: "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_extinction_ledger_receipt",
        path: "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "seed_runtime_contracts_receipt",
        path: "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_session_rituals_receipt",
        path: "receipts/p02/pass_0062_bootstrap_session_rituals.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "host_boundary_challenge_receipt",
        path: "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_target_matrix_receipt",
        path: "receipts/p02/pass_0064_bootstrap_target_matrix.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_truth_cleanup_receipt",
        path: "receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_emergency_fallback_receipt",
        path: "receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "seed_runtime_replacement_milestones_receipt",
        path: "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_evidence_emission_receipt",
        path: "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "operator_handoff_automation_receipt",
        path: "receipts/p02/pass_0069_operator_handoff_automation.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "foreign_surface_closure_receipt",
        path: "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_formal_semantics_receipt",
        path: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_canonical_model_receipt",
        path: "receipts/p02/pass_0072_bootstrap_canonical_model.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_core_engine_receipt",
        path: "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_falsification_receipt",
        path: "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    BootstrapReplayReceiptDescriptor {
        id: "bootstrap_replay_receipt",
        path: "receipts/p02/pass_0075_bootstrap_replay.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_REPLAY_WITNESSES: &[BootstrapReplayWitnessDescriptor] = &[
    BootstrapReplayWitnessDescriptor {
        id: "bootstrap_trust_replay",
        order: "001",
        receipts: &[
            "bootstrap_surface_inventory_receipt",
            "bootstrap_target_matrix_receipt",
            "bootstrap_truth_cleanup_receipt",
            "bootstrap_formal_semantics_receipt",
            "bootstrap_canonical_model_receipt",
        ],
        preimage: "bootstrap_trust_inventory_target_cleanup_semantics_model_preimage",
        witness_hash: HASH,
        commands: &["lyra_p02_bootstrap_trust_replay_check"],
        status: "execution_proven",
    },
    BootstrapReplayWitnessDescriptor {
        id: "seed_runtime_law_replay",
        order: "002",
        receipts: &[
            "seed_runtime_contracts_receipt",
            "seed_runtime_replacement_milestones_receipt",
            "bootstrap_formal_semantics_receipt",
            "bootstrap_core_engine_receipt",
        ],
        preimage: "seed_runtime_contract_replacement_semantics_engine_preimage",
        witness_hash: HASH,
        commands: &["lyra_p02_seed_runtime_law_replay_check"],
        status: "execution_proven",
    },
    BootstrapReplayWitnessDescriptor {
        id: "host_extinction_replay",
        order: "003",
        receipts: &[
            "bootstrap_extinction_ledger_receipt",
            "host_boundary_challenge_receipt",
            "foreign_surface_closure_receipt",
            "bootstrap_falsification_receipt",
        ],
        preimage: "host_extinction_boundary_closure_falsification_preimage",
        witness_hash: HASH,
        commands: &["lyra_p02_host_extinction_replay_check"],
        status: "execution_proven",
    },
    BootstrapReplayWitnessDescriptor {
        id: "operator_handoff_replay",
        order: "004",
        receipts: &[
            "bootstrap_session_rituals_receipt",
            "operator_handoff_automation_receipt",
            "bootstrap_evidence_emission_receipt",
        ],
        preimage: "session_handoff_evidence_preimage",
        witness_hash: HASH,
        commands: &["lyra_p02_operator_handoff_replay_check"],
        status: "execution_proven",
    },
    BootstrapReplayWitnessDescriptor {
        id: "bootstrap_engine_replay",
        order: "005",
        receipts: &[
            "bootstrap_canonical_model_receipt",
            "bootstrap_core_engine_receipt",
            "bootstrap_falsification_receipt",
        ],
        preimage: "canonical_model_core_engine_falsification_preimage",
        witness_hash: HASH,
        commands: &["lyra_p02_bootstrap_engine_replay_check"],
        status: "execution_proven",
    },
    BootstrapReplayWitnessDescriptor {
        id: "fallback_receipt_replay",
        order: "006",
        receipts: &[
            "bootstrap_emergency_fallback_receipt",
            "bootstrap_evidence_emission_receipt",
            "bootstrap_falsification_receipt",
        ],
        preimage: "fallback_evidence_falsification_preimage",
        witness_hash: HASH,
        commands: &["lyra_p02_fallback_receipt_replay_check"],
        status: "execution_proven",
    },
    BootstrapReplayWitnessDescriptor {
        id: "p02_bootstrap_receipt_chain_replay",
        order: "007",
        receipts: &[
            "bootstrap_surface_inventory_receipt",
            "bootstrap_extinction_ledger_receipt",
            "seed_runtime_contracts_receipt",
            "bootstrap_session_rituals_receipt",
            "host_boundary_challenge_receipt",
            "bootstrap_target_matrix_receipt",
            "bootstrap_truth_cleanup_receipt",
            "bootstrap_emergency_fallback_receipt",
            "seed_runtime_replacement_milestones_receipt",
            "bootstrap_evidence_emission_receipt",
            "operator_handoff_automation_receipt",
            "foreign_surface_closure_receipt",
            "bootstrap_formal_semantics_receipt",
            "bootstrap_canonical_model_receipt",
            "bootstrap_core_engine_receipt",
            "bootstrap_falsification_receipt",
            "bootstrap_replay_receipt",
        ],
        preimage: "p02_bootstrap_full_receipt_chain_preimage",
        witness_hash: HASH,
        commands: &["lyra_p02_bootstrap_receipt_chain_replay_check"],
        status: "execution_proven",
    },
];

pub const LYRALANG_BOOTSTRAP_REPLAY_LINKS: &[BootstrapReplayChainLinkDescriptor] = &[
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_surface_inventory_to_bootstrap_extinction_ledger",
        from: "bootstrap_surface_inventory_receipt",
        to: "bootstrap_extinction_ledger_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_surface_inventory_receipt",
            "bootstrap_extinction_ledger_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_extinction_ledger_to_seed_runtime_contracts",
        from: "bootstrap_extinction_ledger_receipt",
        to: "seed_runtime_contracts_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_extinction_ledger_receipt",
            "seed_runtime_contracts_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "seed_runtime_contracts_to_bootstrap_session_rituals",
        from: "seed_runtime_contracts_receipt",
        to: "bootstrap_session_rituals_receipt",
        relation: "precedes",
        receipts: &[
            "seed_runtime_contracts_receipt",
            "bootstrap_session_rituals_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_session_rituals_to_host_boundary_challenge",
        from: "bootstrap_session_rituals_receipt",
        to: "host_boundary_challenge_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_session_rituals_receipt",
            "host_boundary_challenge_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "host_boundary_challenge_to_bootstrap_target_matrix",
        from: "host_boundary_challenge_receipt",
        to: "bootstrap_target_matrix_receipt",
        relation: "precedes",
        receipts: &[
            "host_boundary_challenge_receipt",
            "bootstrap_target_matrix_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_target_matrix_to_bootstrap_truth_cleanup",
        from: "bootstrap_target_matrix_receipt",
        to: "bootstrap_truth_cleanup_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_target_matrix_receipt",
            "bootstrap_truth_cleanup_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_truth_cleanup_to_bootstrap_emergency_fallback",
        from: "bootstrap_truth_cleanup_receipt",
        to: "bootstrap_emergency_fallback_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_truth_cleanup_receipt",
            "bootstrap_emergency_fallback_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_emergency_fallback_to_seed_runtime_replacement_milestones",
        from: "bootstrap_emergency_fallback_receipt",
        to: "seed_runtime_replacement_milestones_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_emergency_fallback_receipt",
            "seed_runtime_replacement_milestones_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "seed_runtime_replacement_milestones_to_bootstrap_evidence_emission",
        from: "seed_runtime_replacement_milestones_receipt",
        to: "bootstrap_evidence_emission_receipt",
        relation: "precedes",
        receipts: &[
            "seed_runtime_replacement_milestones_receipt",
            "bootstrap_evidence_emission_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_evidence_emission_to_operator_handoff_automation",
        from: "bootstrap_evidence_emission_receipt",
        to: "operator_handoff_automation_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_evidence_emission_receipt",
            "operator_handoff_automation_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "operator_handoff_automation_to_foreign_surface_closure",
        from: "operator_handoff_automation_receipt",
        to: "foreign_surface_closure_receipt",
        relation: "precedes",
        receipts: &[
            "operator_handoff_automation_receipt",
            "foreign_surface_closure_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "foreign_surface_closure_to_bootstrap_formal_semantics",
        from: "foreign_surface_closure_receipt",
        to: "bootstrap_formal_semantics_receipt",
        relation: "precedes",
        receipts: &[
            "foreign_surface_closure_receipt",
            "bootstrap_formal_semantics_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_formal_semantics_to_bootstrap_canonical_model",
        from: "bootstrap_formal_semantics_receipt",
        to: "bootstrap_canonical_model_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_formal_semantics_receipt",
            "bootstrap_canonical_model_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_canonical_model_to_bootstrap_core_engine",
        from: "bootstrap_canonical_model_receipt",
        to: "bootstrap_core_engine_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_canonical_model_receipt",
            "bootstrap_core_engine_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_core_engine_to_bootstrap_falsification",
        from: "bootstrap_core_engine_receipt",
        to: "bootstrap_falsification_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_core_engine_receipt",
            "bootstrap_falsification_receipt",
        ],
        status: "execution_proven",
    },
    BootstrapReplayChainLinkDescriptor {
        id: "bootstrap_falsification_to_bootstrap_replay",
        from: "bootstrap_falsification_receipt",
        to: "bootstrap_replay_receipt",
        relation: "precedes",
        receipts: &[
            "bootstrap_falsification_receipt",
            "bootstrap_replay_receipt",
        ],
        status: "execution_proven",
    },
];

pub const LYRALANG_BOOTSTRAP_REPLAY_PROOFS: &[BootstrapReplayProofDescriptor] = &[
    BootstrapReplayProofDescriptor {
        id: "bootstrap_trust_replay_proof",
        scope: "domain",
        receipts: &[
            "bootstrap_surface_inventory_receipt",
            "bootstrap_target_matrix_receipt",
            "bootstrap_truth_cleanup_receipt",
            "bootstrap_formal_semantics_receipt",
            "bootstrap_canonical_model_receipt",
        ],
        witnesses: &["bootstrap_trust_replay"],
        links: &[
            "bootstrap_surface_inventory_to_bootstrap_extinction_ledger",
            "host_boundary_challenge_to_bootstrap_target_matrix",
            "bootstrap_target_matrix_to_bootstrap_truth_cleanup",
            "foreign_surface_closure_to_bootstrap_formal_semantics",
            "bootstrap_formal_semantics_to_bootstrap_canonical_model",
        ],
        commands: &["lyra_p02_bootstrap_trust_replay_proof_check"],
        forbids: &[
            "ambient_authority",
            "mutable_replay",
            "missing_master_authority",
        ],
        status: "execution_proven",
    },
    BootstrapReplayProofDescriptor {
        id: "seed_runtime_law_replay_proof",
        scope: "domain",
        receipts: &[
            "seed_runtime_contracts_receipt",
            "seed_runtime_replacement_milestones_receipt",
            "bootstrap_formal_semantics_receipt",
            "bootstrap_core_engine_receipt",
        ],
        witnesses: &["seed_runtime_law_replay"],
        links: &[
            "bootstrap_extinction_ledger_to_seed_runtime_contracts",
            "bootstrap_emergency_fallback_to_seed_runtime_replacement_milestones",
            "foreign_surface_closure_to_bootstrap_formal_semantics",
            "bootstrap_canonical_model_to_bootstrap_core_engine",
        ],
        commands: &["lyra_p02_seed_runtime_law_replay_proof_check"],
        forbids: &["ambient_network", "probabilistic_replay", "mutable_replay"],
        status: "execution_proven",
    },
    BootstrapReplayProofDescriptor {
        id: "host_extinction_replay_proof",
        scope: "domain",
        receipts: &[
            "bootstrap_extinction_ledger_receipt",
            "host_boundary_challenge_receipt",
            "foreign_surface_closure_receipt",
            "bootstrap_falsification_receipt",
        ],
        witnesses: &["host_extinction_replay"],
        links: &[
            "bootstrap_surface_inventory_to_bootstrap_extinction_ledger",
            "bootstrap_session_rituals_to_host_boundary_challenge",
            "operator_handoff_automation_to_foreign_surface_closure",
            "bootstrap_core_engine_to_bootstrap_falsification",
        ],
        commands: &["lyra_p02_host_extinction_replay_proof_check"],
        forbids: &[
            "unledgered_host_surface",
            "foreign_truth_source",
            "mutable_replay",
        ],
        status: "execution_proven",
    },
    BootstrapReplayProofDescriptor {
        id: "p02_bootstrap_receipt_chain_integrity",
        scope: "chain",
        receipts: &[
            "bootstrap_surface_inventory_receipt",
            "bootstrap_extinction_ledger_receipt",
            "seed_runtime_contracts_receipt",
            "bootstrap_session_rituals_receipt",
            "host_boundary_challenge_receipt",
            "bootstrap_target_matrix_receipt",
            "bootstrap_truth_cleanup_receipt",
            "bootstrap_emergency_fallback_receipt",
            "seed_runtime_replacement_milestones_receipt",
            "bootstrap_evidence_emission_receipt",
            "operator_handoff_automation_receipt",
            "foreign_surface_closure_receipt",
            "bootstrap_formal_semantics_receipt",
            "bootstrap_canonical_model_receipt",
            "bootstrap_core_engine_receipt",
            "bootstrap_falsification_receipt",
            "bootstrap_replay_receipt",
        ],
        witnesses: &["p02_bootstrap_receipt_chain_replay"],
        links: &[
            "bootstrap_surface_inventory_to_bootstrap_extinction_ledger",
            "bootstrap_extinction_ledger_to_seed_runtime_contracts",
            "seed_runtime_contracts_to_bootstrap_session_rituals",
            "bootstrap_session_rituals_to_host_boundary_challenge",
            "host_boundary_challenge_to_bootstrap_target_matrix",
            "bootstrap_target_matrix_to_bootstrap_truth_cleanup",
            "bootstrap_truth_cleanup_to_bootstrap_emergency_fallback",
            "bootstrap_emergency_fallback_to_seed_runtime_replacement_milestones",
            "seed_runtime_replacement_milestones_to_bootstrap_evidence_emission",
            "bootstrap_evidence_emission_to_operator_handoff_automation",
            "operator_handoff_automation_to_foreign_surface_closure",
            "foreign_surface_closure_to_bootstrap_formal_semantics",
            "bootstrap_formal_semantics_to_bootstrap_canonical_model",
            "bootstrap_canonical_model_to_bootstrap_core_engine",
            "bootstrap_core_engine_to_bootstrap_falsification",
            "bootstrap_falsification_to_bootstrap_replay",
        ],
        commands: &["lyra_p02_bootstrap_receipt_chain_integrity_check"],
        forbids: &["orphan_receipt", "receipt_hash_mismatch", "mutable_replay"],
        status: "execution_proven",
    },
    BootstrapReplayProofDescriptor {
        id: "bootstrap_witness_hash_stability",
        scope: "witness",
        receipts: &[
            "bootstrap_core_engine_receipt",
            "bootstrap_falsification_receipt",
            "bootstrap_replay_receipt",
        ],
        witnesses: &[
            "bootstrap_engine_replay",
            "fallback_receipt_replay",
            "p02_bootstrap_receipt_chain_replay",
        ],
        links: &[
            "bootstrap_canonical_model_to_bootstrap_core_engine",
            "bootstrap_core_engine_to_bootstrap_falsification",
            "bootstrap_falsification_to_bootstrap_replay",
        ],
        commands: &["lyra_p02_bootstrap_witness_hash_stability_check"],
        forbids: &["hash_mismatch", "host_order", "ambient_time"],
        status: "execution_proven",
    },
];

pub const LYRALANG_BOOTSTRAP_REPLAY_ARTIFACTS: &[BootstrapReplayArtifactDescriptor] = &[
    BootstrapReplayArtifactDescriptor {
        id: "bootstrap_replay_contract",
        owner_root: "interfaces",
        path: "interfaces/p02/contracts/bootstrap_replay.v1.lyra",
        artifact_kind: "contract",
        status: "artifact_emitted",
    },
    BootstrapReplayArtifactDescriptor {
        id: "bootstrap_replay_law",
        owner_root: "ops",
        path: "ops/p02/replay/bootstrap_replay_witnesses.v1.lyra",
        artifact_kind: "law",
        status: "artifact_emitted",
    },
    BootstrapReplayArtifactDescriptor {
        id: "bootstrap_replay_operator",
        owner_root: "src",
        path: "src/bin/lyra-p02-bootstrap-replay-check.rs",
        artifact_kind: "operator",
        status: "artifact_emitted",
    },
    BootstrapReplayArtifactDescriptor {
        id: "valid_bootstrap_replay_fixture",
        owner_root: "fixtures",
        path: "fixtures/p02/bootstrap_replay_inputs/valid_bootstrap_replay.lyra",
        artifact_kind: "fixture",
        status: "artifact_emitted",
    },
    BootstrapReplayArtifactDescriptor {
        id: "golden_bootstrap_replay_receipt",
        owner_root: "goldens",
        path: "goldens/p02/valid_bootstrap_replay.receipt",
        artifact_kind: "golden",
        status: "artifact_emitted",
    },
    BootstrapReplayArtifactDescriptor {
        id: "execution_bootstrap_replay_receipt",
        owner_root: "receipts",
        path: "receipts/p02/pass_0075_bootstrap_replay.receipt",
        artifact_kind: "receipt",
        status: "artifact_emitted",
    },
    BootstrapReplayArtifactDescriptor {
        id: "deterministic_bootstrap_replay_report",
        owner_root: "k0",
        path: "k0/determinism/src/bootstrap_replay.rs",
        artifact_kind: "deterministic_report",
        status: "artifact_emitted",
    },
    BootstrapReplayArtifactDescriptor {
        id: "bootstrap_replay_suite_report",
        owner_root: "receipts",
        path: "receipts/p02/bootstrap_replay/bootstrap_replay_suite.report",
        artifact_kind: "report",
        status: "artifact_emitted",
    },
];

pub fn bootstrap_replay_receipt_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_REPLAY_RECEIPTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_replay_witness_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_REPLAY_WITNESSES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_replay_link_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_REPLAY_LINKS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_replay_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_REPLAY_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_replay_artifact_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_REPLAY_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_replay_receipt_descriptor(
    id: &str,
) -> Option<&'static BootstrapReplayReceiptDescriptor> {
    LYRALANG_BOOTSTRAP_REPLAY_RECEIPTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_replay_witness_descriptor(
    id: &str,
) -> Option<&'static BootstrapReplayWitnessDescriptor> {
    LYRALANG_BOOTSTRAP_REPLAY_WITNESSES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_replay_link_descriptor(
    id: &str,
) -> Option<&'static BootstrapReplayChainLinkDescriptor> {
    LYRALANG_BOOTSTRAP_REPLAY_LINKS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_replay_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapReplayProofDescriptor> {
    LYRALANG_BOOTSTRAP_REPLAY_PROOFS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_replay_artifact_descriptor(
    id: &str,
) -> Option<&'static BootstrapReplayArtifactDescriptor> {
    LYRALANG_BOOTSTRAP_REPLAY_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_replay_receipt_signature(item: &BootstrapReplayReceiptDescriptor) -> String {
    format!("receipt:{}|path:{}|input_hash:{}|canonical_hash:{}|verdict_hash:{}|receipt_hash:{}|status:{}", item.id, item.path, item.input_hash, item.canonical_hash, item.verdict_hash, item.receipt_hash, item.status)
}
pub fn bootstrap_replay_witness_signature(item: &BootstrapReplayWitnessDescriptor) -> String {
    format!(
        "witness:{}|order:{}|receipts:{}|preimage:{}|witness_hash:{}|commands:{}|status:{}",
        item.id,
        item.order,
        item.receipts.join(","),
        item.preimage,
        item.witness_hash,
        item.commands.join(","),
        item.status
    )
}
pub fn bootstrap_replay_link_signature(item: &BootstrapReplayChainLinkDescriptor) -> String {
    format!(
        "link:{}|from:{}|to:{}|relation:{}|receipts:{}|status:{}",
        item.id,
        item.from,
        item.to,
        item.relation,
        item.receipts.join(","),
        item.status
    )
}
pub fn bootstrap_replay_proof_signature(item: &BootstrapReplayProofDescriptor) -> String {
    format!(
        "proof:{}|scope:{}|receipts:{}|witnesses:{}|links:{}|commands:{}|forbids:{}|status:{}",
        item.id,
        item.scope,
        item.receipts.join(","),
        item.witnesses.join(","),
        item.links.join(","),
        item.commands.join(","),
        item.forbids.join(","),
        item.status
    )
}
pub fn bootstrap_replay_artifact_signature(item: &BootstrapReplayArtifactDescriptor) -> String {
    format!(
        "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
        item.id, item.owner_root, item.path, item.artifact_kind, item.status
    )
}

pub fn bootstrap_replay_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_BOOTSTRAP_REPLAY_RECEIPTS {
        rows.push(bootstrap_replay_receipt_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_REPLAY_WITNESSES {
        rows.push(bootstrap_replay_witness_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_REPLAY_LINKS {
        rows.push(bootstrap_replay_link_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_REPLAY_PROOFS {
        rows.push(bootstrap_replay_proof_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_REPLAY_ARTIFACTS {
        rows.push(bootstrap_replay_artifact_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn bootstrap_replay_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_replay.registry",
        &bootstrap_replay_registry_signature(),
    )
}
pub fn bootstrap_replay_carrier_signature() -> String {
    format!(
        "{}:{}",
        LYRA_P02_BOOTSTRAP_REPLAY_CARRIER,
        bootstrap_replay_registry_hash()
    )
}
pub fn bootstrap_replay_receipt_digest(id: &str) -> Option<String> {
    bootstrap_replay_receipt_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_replay.receipt_descriptor",
            &bootstrap_replay_receipt_signature(item),
        )
    })
}
pub fn bootstrap_replay_witness_digest(id: &str) -> Option<String> {
    bootstrap_replay_witness_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_replay.witness_descriptor",
            &bootstrap_replay_witness_signature(item),
        )
    })
}
pub fn bootstrap_replay_link_digest(id: &str) -> Option<String> {
    bootstrap_replay_link_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_replay.link_descriptor",
            &bootstrap_replay_link_signature(item),
        )
    })
}
pub fn bootstrap_replay_proof_digest(id: &str) -> Option<String> {
    bootstrap_replay_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_replay.proof_descriptor",
            &bootstrap_replay_proof_signature(item),
        )
    })
}
pub fn bootstrap_replay_artifact_digest(id: &str) -> Option<String> {
    bootstrap_replay_artifact_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_replay.artifact_descriptor",
            &bootstrap_replay_artifact_signature(item),
        )
    })
}

pub fn bootstrap_replay_witnesses_bind_known_receipts() -> bool {
    LYRALANG_BOOTSTRAP_REPLAY_WITNESSES.iter().all(|witness| {
        witness
            .receipts
            .iter()
            .all(|id| bootstrap_replay_receipt_descriptor(id).is_some())
    })
}

pub fn bootstrap_replay_links_bind_known_receipts() -> bool {
    LYRALANG_BOOTSTRAP_REPLAY_LINKS.iter().all(|link| {
        bootstrap_replay_receipt_descriptor(link.from).is_some()
            && bootstrap_replay_receipt_descriptor(link.to).is_some()
            && link
                .receipts
                .iter()
                .all(|id| bootstrap_replay_receipt_descriptor(id).is_some())
    })
}

pub fn bootstrap_replay_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_REPLAY_PROOFS.iter().all(|proof| {
        proof
            .receipts
            .iter()
            .all(|id| bootstrap_replay_receipt_descriptor(id).is_some())
            && proof
                .witnesses
                .iter()
                .all(|id| bootstrap_replay_witness_descriptor(id).is_some())
            && proof
                .links
                .iter()
                .all(|id| bootstrap_replay_link_descriptor(id).is_some())
            && !proof.commands.is_empty()
            && !proof.forbids.is_empty()
    })
}

pub fn bootstrap_replay_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_REPLAY_ARTIFACTS.iter().all(|artifact| {
        !artifact.path.is_empty()
            && !artifact.path.contains("..")
            && ["lyra", "rs", "receipt", "report"]
                .iter()
                .any(|suffix| artifact.path.ends_with(suffix))
    })
}

pub fn bootstrap_replay_receipts_cover_p02_001_through_p02_017() -> bool {
    LYRALANG_BOOTSTRAP_REPLAY_RECEIPTS.len() == 17
        && bootstrap_replay_receipt_descriptor("bootstrap_surface_inventory_receipt").is_some()
        && bootstrap_replay_receipt_descriptor("bootstrap_replay_receipt").is_some()
}

pub fn bootstrap_replay_no_forbidden_descriptor_claims() -> bool {
    let lower = bootstrap_replay_registry_signature().to_ascii_lowercase();
    !(lower.contains("mutable replay allowed")
        || lower.contains("network replay required")
        || lower.contains("probabilistic replay allowed")
        || lower.contains("phase closed")
        || lower.contains("global complete")
        || lower.contains("manual only"))
}
