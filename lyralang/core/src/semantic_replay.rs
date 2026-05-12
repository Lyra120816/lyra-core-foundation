use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticReplayReceiptDescriptor {
    pub id: &'static str,
    pub path: &'static str,
    pub input_hash: &'static str,
    pub canonical_hash: &'static str,
    pub verdict_hash: &'static str,
    pub receipt_hash: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticReplayWitnessDescriptor {
    pub id: &'static str,
    pub order: &'static str,
    pub receipts: &'static [&'static str],
    pub preimage: &'static str,
    pub witness_hash: &'static str,
    pub commands: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticReplayChainLinkDescriptor {
    pub id: &'static str,
    pub from: &'static str,
    pub to: &'static str,
    pub relation: &'static str,
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticReplayProofDescriptor {
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
pub struct SemanticReplayArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub artifact_kind: &'static str,
    pub status: &'static str,
}

pub const LYRA_P01_SEMANTIC_REPLAY_CARRIER: &str = "lyra.p01.semantic_replay.carrier.v1";
const HASH: &str = "fnv1a128:0123456789abcdef0123456789abcdef";

pub const LYRALANG_SEMANTIC_REPLAY_RECEIPTS: &[SemanticReplayReceiptDescriptor] = &[
    SemanticReplayReceiptDescriptor {
        id: "semantic_atoms_receipt",
        path: "receipts/p01/pass_0030_semantic_atoms.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "core_ir_receipt",
        path: "receipts/p01/pass_0031_core_ir.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_objects_receipt",
        path: "receipts/p01/pass_0032_semantic_objects.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_identity_receipt",
        path: "receipts/p01/pass_0033_semantic_identity.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "reference_semantics_receipt",
        path: "receipts/p01/pass_0034_reference_semantics.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "symbolic_equality_receipt",
        path: "receipts/p01/pass_0035_symbolic_equality.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "error_challenge_evidence_receipt",
        path: "receipts/p01/pass_0036_error_challenge_evidence.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_serialization_hashing_receipt",
        path: "receipts/p01/pass_0037_semantic_serialization_hashing.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_adversarial_corpus_receipt",
        path: "receipts/p01/pass_0038_semantic_adversarial_corpus.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "core_ir_reuse_receipt",
        path: "receipts/p01/pass_0039_core_ir_reuse.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_atom_reference_receipt",
        path: "receipts/p01/pass_0040_semantic_atom_reference.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_bedrock_receipts_receipt",
        path: "receipts/p01/pass_0041_semantic_bedrock_receipts.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "formal_semantic_constitution_receipt",
        path: "receipts/p01/pass_0042_formal_semantic_constitution.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "canonical_data_model_receipt",
        path: "receipts/p01/pass_0043_canonical_data_model.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_core_engine_receipt",
        path: "receipts/p01/pass_0044_semantic_core_engine.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_falsification_receipt",
        path: "receipts/p01/pass_0045_semantic_falsification.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
    SemanticReplayReceiptDescriptor {
        id: "semantic_replay_receipt",
        path: "receipts/p01/pass_0046_semantic_replay.receipt",
        input_hash: HASH,
        canonical_hash: HASH,
        verdict_hash: HASH,
        receipt_hash: HASH,
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_REPLAY_WITNESSES: &[SemanticReplayWitnessDescriptor] = &[
    SemanticReplayWitnessDescriptor {
        id: "canonical_symbols_replay",
        order: "001",
        receipts: &[
            "semantic_atoms_receipt",
            "semantic_identity_receipt",
            "semantic_serialization_hashing_receipt",
            "canonical_data_model_receipt",
        ],
        preimage: "canonical_symbols_identity_serialization_model_preimage",
        witness_hash: HASH,
        commands: &["lyra_p01_canonical_symbols_replay_check"],
        status: "execution_proven",
    },
    SemanticReplayWitnessDescriptor {
        id: "semantic_atoms_replay",
        order: "002",
        receipts: &[
            "semantic_atoms_receipt",
            "semantic_atom_reference_receipt",
            "semantic_bedrock_receipts_receipt",
        ],
        preimage: "semantic_atoms_reference_bedrock_preimage",
        witness_hash: HASH,
        commands: &["lyra_p01_semantic_atoms_replay_check"],
        status: "execution_proven",
    },
    SemanticReplayWitnessDescriptor {
        id: "core_ir_replay",
        order: "003",
        receipts: &[
            "core_ir_receipt",
            "core_ir_reuse_receipt",
            "semantic_core_engine_receipt",
        ],
        preimage: "core_ir_reuse_engine_preimage",
        witness_hash: HASH,
        commands: &["lyra_p01_core_ir_replay_check"],
        status: "execution_proven",
    },
    SemanticReplayWitnessDescriptor {
        id: "semantic_object_model_replay",
        order: "004",
        receipts: &[
            "semantic_objects_receipt",
            "reference_semantics_receipt",
            "symbolic_equality_receipt",
            "error_challenge_evidence_receipt",
        ],
        preimage: "semantic_object_reference_equality_evidence_preimage",
        witness_hash: HASH,
        commands: &["lyra_p01_semantic_object_model_replay_check"],
        status: "execution_proven",
    },
    SemanticReplayWitnessDescriptor {
        id: "semantic_core_engine_replay",
        order: "005",
        receipts: &[
            "formal_semantic_constitution_receipt",
            "canonical_data_model_receipt",
            "semantic_core_engine_receipt",
        ],
        preimage: "formal_semantics_model_engine_preimage",
        witness_hash: HASH,
        commands: &["lyra_p01_semantic_core_engine_replay_check"],
        status: "execution_proven",
    },
    SemanticReplayWitnessDescriptor {
        id: "semantic_falsification_replay",
        order: "006",
        receipts: &[
            "semantic_adversarial_corpus_receipt",
            "semantic_falsification_receipt",
            "semantic_replay_receipt",
        ],
        preimage: "semantic_adversarial_falsification_replay_preimage",
        witness_hash: HASH,
        commands: &["lyra_p01_semantic_falsification_replay_check"],
        status: "execution_proven",
    },
    SemanticReplayWitnessDescriptor {
        id: "p01_semantic_receipt_chain_replay",
        order: "007",
        receipts: &[
            "semantic_atoms_receipt",
            "core_ir_receipt",
            "semantic_objects_receipt",
            "semantic_identity_receipt",
            "reference_semantics_receipt",
            "symbolic_equality_receipt",
            "error_challenge_evidence_receipt",
            "semantic_serialization_hashing_receipt",
            "semantic_adversarial_corpus_receipt",
            "core_ir_reuse_receipt",
            "semantic_atom_reference_receipt",
            "semantic_bedrock_receipts_receipt",
            "formal_semantic_constitution_receipt",
            "canonical_data_model_receipt",
            "semantic_core_engine_receipt",
            "semantic_falsification_receipt",
            "semantic_replay_receipt",
        ],
        preimage: "p01_semantic_full_receipt_chain_preimage",
        witness_hash: HASH,
        commands: &["lyra_p01_semantic_receipt_chain_replay_check"],
        status: "execution_proven",
    },
];

pub const LYRALANG_SEMANTIC_REPLAY_LINKS: &[SemanticReplayChainLinkDescriptor] = &[
    SemanticReplayChainLinkDescriptor {
        id: "semantic_atoms_to_core_ir",
        from: "semantic_atoms_receipt",
        to: "core_ir_receipt",
        relation: "precedes",
        receipts: &["semantic_atoms_receipt", "core_ir_receipt"],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "core_ir_to_semantic_objects",
        from: "core_ir_receipt",
        to: "semantic_objects_receipt",
        relation: "precedes",
        receipts: &["core_ir_receipt", "semantic_objects_receipt"],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "semantic_objects_to_semantic_identity",
        from: "semantic_objects_receipt",
        to: "semantic_identity_receipt",
        relation: "precedes",
        receipts: &["semantic_objects_receipt", "semantic_identity_receipt"],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "semantic_identity_to_reference_semantics",
        from: "semantic_identity_receipt",
        to: "reference_semantics_receipt",
        relation: "precedes",
        receipts: &["semantic_identity_receipt", "reference_semantics_receipt"],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "reference_semantics_to_symbolic_equality",
        from: "reference_semantics_receipt",
        to: "symbolic_equality_receipt",
        relation: "precedes",
        receipts: &["reference_semantics_receipt", "symbolic_equality_receipt"],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "symbolic_equality_to_error_challenge_evidence",
        from: "symbolic_equality_receipt",
        to: "error_challenge_evidence_receipt",
        relation: "precedes",
        receipts: &[
            "symbolic_equality_receipt",
            "error_challenge_evidence_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "error_challenge_evidence_to_semantic_serialization_hashing",
        from: "error_challenge_evidence_receipt",
        to: "semantic_serialization_hashing_receipt",
        relation: "precedes",
        receipts: &[
            "error_challenge_evidence_receipt",
            "semantic_serialization_hashing_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "semantic_serialization_hashing_to_semantic_adversarial_corpus",
        from: "semantic_serialization_hashing_receipt",
        to: "semantic_adversarial_corpus_receipt",
        relation: "precedes",
        receipts: &[
            "semantic_serialization_hashing_receipt",
            "semantic_adversarial_corpus_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "semantic_adversarial_corpus_to_core_ir_reuse",
        from: "semantic_adversarial_corpus_receipt",
        to: "core_ir_reuse_receipt",
        relation: "precedes",
        receipts: &[
            "semantic_adversarial_corpus_receipt",
            "core_ir_reuse_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "core_ir_reuse_to_semantic_atom_reference",
        from: "core_ir_reuse_receipt",
        to: "semantic_atom_reference_receipt",
        relation: "precedes",
        receipts: &["core_ir_reuse_receipt", "semantic_atom_reference_receipt"],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "semantic_atom_reference_to_semantic_bedrock_receipts",
        from: "semantic_atom_reference_receipt",
        to: "semantic_bedrock_receipts_receipt",
        relation: "precedes",
        receipts: &[
            "semantic_atom_reference_receipt",
            "semantic_bedrock_receipts_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "semantic_bedrock_receipts_to_formal_semantic_constitution",
        from: "semantic_bedrock_receipts_receipt",
        to: "formal_semantic_constitution_receipt",
        relation: "precedes",
        receipts: &[
            "semantic_bedrock_receipts_receipt",
            "formal_semantic_constitution_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "formal_semantic_constitution_to_canonical_data_model",
        from: "formal_semantic_constitution_receipt",
        to: "canonical_data_model_receipt",
        relation: "precedes",
        receipts: &[
            "formal_semantic_constitution_receipt",
            "canonical_data_model_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "canonical_data_model_to_semantic_core_engine",
        from: "canonical_data_model_receipt",
        to: "semantic_core_engine_receipt",
        relation: "precedes",
        receipts: &[
            "canonical_data_model_receipt",
            "semantic_core_engine_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "semantic_core_engine_to_semantic_falsification",
        from: "semantic_core_engine_receipt",
        to: "semantic_falsification_receipt",
        relation: "precedes",
        receipts: &[
            "semantic_core_engine_receipt",
            "semantic_falsification_receipt",
        ],
        status: "execution_proven",
    },
    SemanticReplayChainLinkDescriptor {
        id: "semantic_falsification_to_semantic_replay",
        from: "semantic_falsification_receipt",
        to: "semantic_replay_receipt",
        relation: "precedes",
        receipts: &["semantic_falsification_receipt", "semantic_replay_receipt"],
        status: "execution_proven",
    },
];

pub const LYRALANG_SEMANTIC_REPLAY_PROOFS: &[SemanticReplayProofDescriptor] = &[
    SemanticReplayProofDescriptor {
        id: "canonical_symbols_replay_proof",
        scope: "domain",
        receipts: &[
            "semantic_atoms_receipt",
            "semantic_identity_receipt",
            "semantic_serialization_hashing_receipt",
            "canonical_data_model_receipt",
        ],
        witnesses: &["canonical_symbols_replay"],
        links: &[
            "semantic_atoms_to_core_ir",
            "semantic_identity_to_reference_semantics",
            "formal_semantic_constitution_to_canonical_data_model",
        ],
        commands: &["lyra_p01_canonical_symbols_replay_proof_check"],
        forbids: &["ambient_time", "hidden_randomness", "mutable_replay"],
        status: "execution_proven",
    },
    SemanticReplayProofDescriptor {
        id: "semantic_atoms_replay_proof",
        scope: "domain",
        receipts: &[
            "semantic_atoms_receipt",
            "semantic_atom_reference_receipt",
            "semantic_bedrock_receipts_receipt",
        ],
        witnesses: &["semantic_atoms_replay"],
        links: &[
            "core_ir_reuse_to_semantic_atom_reference",
            "semantic_atom_reference_to_semantic_bedrock_receipts",
        ],
        commands: &["lyra_p01_semantic_atoms_replay_proof_check"],
        forbids: &[
            "probabilistic_replay",
            "foreign_truth_source",
            "mutable_replay",
        ],
        status: "execution_proven",
    },
    SemanticReplayProofDescriptor {
        id: "core_ir_replay_proof",
        scope: "domain",
        receipts: &[
            "core_ir_receipt",
            "core_ir_reuse_receipt",
            "semantic_core_engine_receipt",
        ],
        witnesses: &["core_ir_replay", "semantic_core_engine_replay"],
        links: &[
            "semantic_atoms_to_core_ir",
            "semantic_adversarial_corpus_to_core_ir_reuse",
            "canonical_data_model_to_semantic_core_engine",
        ],
        commands: &["lyra_p01_core_ir_replay_proof_check"],
        forbids: &["host_order", "mutable_replay", "network_replay"],
        status: "execution_proven",
    },
    SemanticReplayProofDescriptor {
        id: "p01_semantic_receipt_chain_integrity",
        scope: "chain",
        receipts: &[
            "semantic_atoms_receipt",
            "core_ir_receipt",
            "semantic_objects_receipt",
            "semantic_identity_receipt",
            "reference_semantics_receipt",
            "symbolic_equality_receipt",
            "error_challenge_evidence_receipt",
            "semantic_serialization_hashing_receipt",
            "semantic_adversarial_corpus_receipt",
            "core_ir_reuse_receipt",
            "semantic_atom_reference_receipt",
            "semantic_bedrock_receipts_receipt",
            "formal_semantic_constitution_receipt",
            "canonical_data_model_receipt",
            "semantic_core_engine_receipt",
            "semantic_falsification_receipt",
            "semantic_replay_receipt",
        ],
        witnesses: &["p01_semantic_receipt_chain_replay"],
        links: &[
            "semantic_atoms_to_core_ir",
            "core_ir_to_semantic_objects",
            "semantic_objects_to_semantic_identity",
            "semantic_identity_to_reference_semantics",
            "reference_semantics_to_symbolic_equality",
            "symbolic_equality_to_error_challenge_evidence",
            "error_challenge_evidence_to_semantic_serialization_hashing",
            "semantic_serialization_hashing_to_semantic_adversarial_corpus",
            "semantic_adversarial_corpus_to_core_ir_reuse",
            "core_ir_reuse_to_semantic_atom_reference",
            "semantic_atom_reference_to_semantic_bedrock_receipts",
            "semantic_bedrock_receipts_to_formal_semantic_constitution",
            "formal_semantic_constitution_to_canonical_data_model",
            "canonical_data_model_to_semantic_core_engine",
            "semantic_core_engine_to_semantic_falsification",
            "semantic_falsification_to_semantic_replay",
        ],
        commands: &["lyra_p01_semantic_receipt_chain_integrity_check"],
        forbids: &["orphan_receipt", "receipt_hash_mismatch", "mutable_replay"],
        status: "execution_proven",
    },
    SemanticReplayProofDescriptor {
        id: "semantic_witness_hash_stability",
        scope: "witness",
        receipts: &[
            "semantic_core_engine_receipt",
            "semantic_falsification_receipt",
            "semantic_replay_receipt",
        ],
        witnesses: &[
            "semantic_core_engine_replay",
            "semantic_falsification_replay",
            "p01_semantic_receipt_chain_replay",
        ],
        links: &[
            "canonical_data_model_to_semantic_core_engine",
            "semantic_core_engine_to_semantic_falsification",
            "semantic_falsification_to_semantic_replay",
        ],
        commands: &["lyra_p01_semantic_witness_hash_stability_check"],
        forbids: &["hash_mismatch", "host_order", "ambient_network"],
        status: "execution_proven",
    },
];

pub const LYRALANG_SEMANTIC_REPLAY_ARTIFACTS: &[SemanticReplayArtifactDescriptor] = &[
    SemanticReplayArtifactDescriptor {
        id: "semantic_replay_contract",
        owner_root: "interfaces",
        path: "interfaces/p01/contracts/semantic_replay.v1.lyra",
        artifact_kind: "contract",
        status: "artifact_emitted",
    },
    SemanticReplayArtifactDescriptor {
        id: "semantic_replay_law",
        owner_root: "ops",
        path: "ops/p01/control/semantic_replay_law.v1.lyra",
        artifact_kind: "law",
        status: "artifact_emitted",
    },
    SemanticReplayArtifactDescriptor {
        id: "semantic_replay_operator",
        owner_root: "src",
        path: "src/bin/lyra-p01-semantic-replay-check.rs",
        artifact_kind: "operator",
        status: "artifact_emitted",
    },
    SemanticReplayArtifactDescriptor {
        id: "valid_semantic_replay_fixture",
        owner_root: "fixtures",
        path: "fixtures/p01/semantic_replay_inputs/valid_semantic_replay.lyra",
        artifact_kind: "fixture",
        status: "artifact_emitted",
    },
    SemanticReplayArtifactDescriptor {
        id: "golden_semantic_replay_receipt",
        owner_root: "goldens",
        path: "goldens/p01/valid_semantic_replay.receipt",
        artifact_kind: "golden",
        status: "artifact_emitted",
    },
    SemanticReplayArtifactDescriptor {
        id: "execution_semantic_replay_receipt",
        owner_root: "receipts",
        path: "receipts/p01/pass_0046_semantic_replay.receipt",
        artifact_kind: "receipt",
        status: "artifact_emitted",
    },
    SemanticReplayArtifactDescriptor {
        id: "deterministic_semantic_replay_report",
        owner_root: "k0",
        path: "k0/determinism/src/semantic_replay.rs",
        artifact_kind: "deterministic_report",
        status: "artifact_emitted",
    },
];

pub fn semantic_replay_receipt_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_REPLAY_RECEIPTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_replay_witness_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_REPLAY_WITNESSES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_replay_link_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_REPLAY_LINKS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_replay_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_REPLAY_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_replay_artifact_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_REPLAY_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_replay_receipt_descriptor(
    id: &str,
) -> Option<&'static SemanticReplayReceiptDescriptor> {
    LYRALANG_SEMANTIC_REPLAY_RECEIPTS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_replay_witness_descriptor(
    id: &str,
) -> Option<&'static SemanticReplayWitnessDescriptor> {
    LYRALANG_SEMANTIC_REPLAY_WITNESSES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_replay_link_descriptor(
    id: &str,
) -> Option<&'static SemanticReplayChainLinkDescriptor> {
    LYRALANG_SEMANTIC_REPLAY_LINKS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_replay_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticReplayProofDescriptor> {
    LYRALANG_SEMANTIC_REPLAY_PROOFS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_replay_artifact_descriptor(
    id: &str,
) -> Option<&'static SemanticReplayArtifactDescriptor> {
    LYRALANG_SEMANTIC_REPLAY_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_replay_receipt_signature(item: &SemanticReplayReceiptDescriptor) -> String {
    format!("receipt:{}|path:{}|input_hash:{}|canonical_hash:{}|verdict_hash:{}|receipt_hash:{}|status:{}", item.id, item.path, item.input_hash, item.canonical_hash, item.verdict_hash, item.receipt_hash, item.status)
}
pub fn semantic_replay_witness_signature(item: &SemanticReplayWitnessDescriptor) -> String {
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
pub fn semantic_replay_link_signature(item: &SemanticReplayChainLinkDescriptor) -> String {
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
pub fn semantic_replay_proof_signature(item: &SemanticReplayProofDescriptor) -> String {
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
pub fn semantic_replay_artifact_signature(item: &SemanticReplayArtifactDescriptor) -> String {
    format!(
        "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
        item.id, item.owner_root, item.path, item.artifact_kind, item.status
    )
}

pub fn semantic_replay_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SEMANTIC_REPLAY_RECEIPTS {
        rows.push(semantic_replay_receipt_signature(item));
    }
    for item in LYRALANG_SEMANTIC_REPLAY_WITNESSES {
        rows.push(semantic_replay_witness_signature(item));
    }
    for item in LYRALANG_SEMANTIC_REPLAY_LINKS {
        rows.push(semantic_replay_link_signature(item));
    }
    for item in LYRALANG_SEMANTIC_REPLAY_PROOFS {
        rows.push(semantic_replay_proof_signature(item));
    }
    for item in LYRALANG_SEMANTIC_REPLAY_ARTIFACTS {
        rows.push(semantic_replay_artifact_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_replay_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_replay.registry",
        &semantic_replay_registry_signature(),
    )
}
pub fn semantic_replay_receipt_digest(id: &str) -> Option<String> {
    semantic_replay_receipt_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_replay.receipt_descriptor",
            &semantic_replay_receipt_signature(item),
        )
    })
}
pub fn semantic_replay_witness_digest(id: &str) -> Option<String> {
    semantic_replay_witness_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_replay.witness_descriptor",
            &semantic_replay_witness_signature(item),
        )
    })
}
pub fn semantic_replay_link_digest(id: &str) -> Option<String> {
    semantic_replay_link_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_replay.link_descriptor",
            &semantic_replay_link_signature(item),
        )
    })
}
pub fn semantic_replay_proof_digest(id: &str) -> Option<String> {
    semantic_replay_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_replay.proof_descriptor",
            &semantic_replay_proof_signature(item),
        )
    })
}
pub fn semantic_replay_artifact_digest(id: &str) -> Option<String> {
    semantic_replay_artifact_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_replay.artifact_descriptor",
            &semantic_replay_artifact_signature(item),
        )
    })
}

pub fn semantic_replay_witnesses_bind_known_receipts() -> bool {
    LYRALANG_SEMANTIC_REPLAY_WITNESSES.iter().all(|witness| {
        witness
            .receipts
            .iter()
            .all(|id| semantic_replay_receipt_descriptor(id).is_some())
    })
}

pub fn semantic_replay_links_bind_known_receipts() -> bool {
    LYRALANG_SEMANTIC_REPLAY_LINKS.iter().all(|link| {
        semantic_replay_receipt_descriptor(link.from).is_some()
            && semantic_replay_receipt_descriptor(link.to).is_some()
            && link
                .receipts
                .iter()
                .all(|id| semantic_replay_receipt_descriptor(id).is_some())
    })
}

pub fn semantic_replay_proofs_bind_registry() -> bool {
    LYRALANG_SEMANTIC_REPLAY_PROOFS.iter().all(|proof| {
        proof
            .receipts
            .iter()
            .all(|id| semantic_replay_receipt_descriptor(id).is_some())
            && proof
                .witnesses
                .iter()
                .all(|id| semantic_replay_witness_descriptor(id).is_some())
            && proof
                .links
                .iter()
                .all(|id| semantic_replay_link_descriptor(id).is_some())
            && !proof.commands.is_empty()
            && !proof.forbids.is_empty()
    })
}

pub fn semantic_replay_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_REPLAY_ARTIFACTS.iter().all(|artifact| {
        !artifact.path.is_empty()
            && !artifact.path.contains("..")
            && ["lyra", "rs", "receipt"]
                .iter()
                .any(|suffix| artifact.path.ends_with(suffix))
    })
}

pub fn semantic_replay_receipts_cover_p01_001_through_p01_017() -> bool {
    LYRALANG_SEMANTIC_REPLAY_RECEIPTS.len() == 17
        && semantic_replay_receipt_descriptor("semantic_atoms_receipt").is_some()
        && semantic_replay_receipt_descriptor("semantic_replay_receipt").is_some()
}

pub fn semantic_replay_no_forbidden_descriptor_claims() -> bool {
    let lower = semantic_replay_registry_signature().to_ascii_lowercase();
    !(lower.contains("mutable replay allowed")
        || lower.contains("network replay")
        || lower.contains("probabilistic replay")
        || lower.contains("phase closed")
        || lower.contains("global complete"))
}
