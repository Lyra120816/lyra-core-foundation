use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticInterfaceCommandDescriptor {
    pub id: &'static str,
    pub binary: &'static str,
    pub surface: &'static str,
    pub input: &'static str,
    pub output: &'static str,
    pub receipts: &'static [&'static str],
    pub roles: &'static [&'static str],
    pub targets: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticInterfaceWorkflowDescriptor {
    pub id: &'static str,
    pub order: &'static str,
    pub commands: &'static [&'static str],
    pub targets: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticInterfaceExampleDescriptor {
    pub id: &'static str,
    pub path: &'static str,
    pub commands: &'static [&'static str],
    pub expected_receipts: &'static [&'static str],
    pub expected_verdict: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticInterfaceProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub commands: &'static [&'static str],
    pub workflows: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticInterfaceArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub artifact_kind: &'static str,
    pub commands: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P01_SEMANTIC_INTERFACE_CARRIER: &str = "lyra.p01.semantic_interface.carrier.v1";

pub const LYRALANG_SEMANTIC_INTERFACE_COMMANDS: &[SemanticInterfaceCommandDescriptor] = &[
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_atoms", binary: "src/bin/lyra-p01-atom-check.rs", surface: "LYRA-P01-SEMANTIC-ATOMS v1", input: "fixtures/p01/semantic_atom_inputs/valid_semantic_atoms.lyra", output: "receipts/p01/pass_0030_semantic_atoms.receipt", receipts: &["receipts/p01/pass_0030_semantic_atoms.receipt"], roles: &["developer", "operator"], targets: &["semantic_atoms"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_core_ir", binary: "src/bin/lyra-p01-ir-check.rs", surface: "LYRA-P01-CORE-IR v1", input: "fixtures/p01/core_ir_inputs/valid_core_ir.lyra", output: "receipts/p01/pass_0031_core_ir.receipt", receipts: &["receipts/p01/pass_0031_core_ir.receipt"], roles: &["developer", "operator"], targets: &["core_ir"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_objects", binary: "src/bin/lyra-p01-object-check.rs", surface: "LYRA-P01-SEMANTIC-OBJECTS v1", input: "fixtures/p01/semantic_object_inputs/valid_semantic_objects.lyra", output: "receipts/p01/pass_0032_semantic_objects.receipt", receipts: &["receipts/p01/pass_0032_semantic_objects.receipt"], roles: &["developer"], targets: &["semantic_objects"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_identity", binary: "src/bin/lyra-p01-identity-check.rs", surface: "LYRA-P01-SEMANTIC-IDENTITY v1", input: "fixtures/p01/semantic_identity_inputs/valid_semantic_identity.lyra", output: "receipts/p01/pass_0033_semantic_identity.receipt", receipts: &["receipts/p01/pass_0033_semantic_identity.receipt"], roles: &["developer", "proof_auditor"], targets: &["semantic_identity"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_reference_semantics", binary: "src/bin/lyra-p01-reference-semantics-check.rs", surface: "LYRA-P01-REFERENCE-SEMANTICS v1", input: "fixtures/p01/reference_semantics_inputs/valid_reference_semantics.lyra", output: "receipts/p01/pass_0034_reference_semantics.receipt", receipts: &["receipts/p01/pass_0034_reference_semantics.receipt"], roles: &["developer", "operator"], targets: &["reference_semantics"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_symbolic_equality", binary: "src/bin/lyra-p01-symbolic-equality-check.rs", surface: "LYRA-P01-SYMBOLIC-EQUALITY v1", input: "fixtures/p01/symbolic_equality_inputs/valid_symbolic_equality.lyra", output: "receipts/p01/pass_0035_symbolic_equality.receipt", receipts: &["receipts/p01/pass_0035_symbolic_equality.receipt"], roles: &["developer", "proof_auditor"], targets: &["symbolic_equality"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_error_challenge_evidence", binary: "src/bin/lyra-p01-error-challenge-evidence-check.rs", surface: "LYRA-P01-ERROR-CHALLENGE-EVIDENCE v1", input: "fixtures/p01/error_challenge_evidence_inputs/valid_error_challenge_evidence.lyra", output: "receipts/p01/pass_0036_error_challenge_evidence.receipt", receipts: &["receipts/p01/pass_0036_error_challenge_evidence.receipt"], roles: &["developer", "red_team"], targets: &["error_challenge_evidence"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_serialization_hashing", binary: "src/bin/lyra-p01-semantic-serialization-hashing-check.rs", surface: "LYRA-P01-SEMANTIC-SERIALIZATION-HASHING v1", input: "fixtures/p01/semantic_serialization_hashing_inputs/valid_semantic_serialization_hashing.lyra", output: "receipts/p01/pass_0037_semantic_serialization_hashing.receipt", receipts: &["receipts/p01/pass_0037_semantic_serialization_hashing.receipt"], roles: &["developer", "proof_auditor"], targets: &["semantic_serialization"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_adversarial_corpus", binary: "src/bin/lyra-p01-semantic-adversarial-corpus-check.rs", surface: "LYRA-P01-SEMANTIC-ADVERSARIAL-CORPUS v1", input: "fixtures/p01/semantic_adversarial_corpus_inputs/valid_semantic_adversarial_corpus.lyra", output: "receipts/p01/pass_0038_semantic_adversarial_corpus.receipt", receipts: &["receipts/p01/pass_0038_semantic_adversarial_corpus.receipt"], roles: &["developer", "red_team"], targets: &["adversarial_corpus"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_core_ir_reuse", binary: "src/bin/lyra-p01-core-ir-reuse-check.rs", surface: "LYRA-P01-CORE-IR-REUSE v1", input: "fixtures/p01/core_ir_reuse_inputs/valid_core_ir_reuse.lyra", output: "receipts/p01/pass_0039_core_ir_reuse.receipt", receipts: &["receipts/p01/pass_0039_core_ir_reuse.receipt"], roles: &["developer", "operator"], targets: &["core_ir_reuse", "core_ir"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_atom_reference", binary: "src/bin/lyra-p01-semantic-atom-reference-check.rs", surface: "LYRA-P01-SEMANTIC-ATOM-REFERENCE v1", input: "fixtures/p01/semantic_atom_reference_inputs/valid_semantic_atom_reference.lyra", output: "receipts/p01/pass_0040_semantic_atom_reference.receipt", receipts: &["receipts/p01/pass_0040_semantic_atom_reference.receipt"], roles: &["developer", "operator"], targets: &["atom_reference", "semantic_atoms"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_bedrock_receipts", binary: "src/bin/lyra-p01-semantic-bedrock-receipts-check.rs", surface: "LYRA-P01-SEMANTIC-BEDROCK-RECEIPTS v1", input: "fixtures/p01/semantic_bedrock_receipts_inputs/valid_semantic_bedrock_receipts.lyra", output: "receipts/p01/pass_0041_semantic_bedrock_receipts.receipt", receipts: &["receipts/p01/pass_0041_semantic_bedrock_receipts.receipt"], roles: &["developer", "proof_auditor"], targets: &["bedrock_receipts"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_formal_semantic_constitution", binary: "src/bin/lyra-p01-formal-semantic-constitution-check.rs", surface: "LYRA-P01-FORMAL-SEMANTIC-CONSTITUTION v1", input: "fixtures/p01/formal_semantic_constitution_inputs/valid_formal_semantic_constitution.lyra", output: "receipts/p01/pass_0042_formal_semantic_constitution.receipt", receipts: &["receipts/p01/pass_0042_formal_semantic_constitution.receipt"], roles: &["developer", "proof_auditor"], targets: &["formal_constitution"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_canonical_data_model", binary: "src/bin/lyra-p01-canonical-data-model-check.rs", surface: "LYRA-P01-CANONICAL-DATA-MODEL v1", input: "fixtures/p01/canonical_data_model_inputs/valid_canonical_data_model.lyra", output: "receipts/p01/pass_0043_canonical_data_model.receipt", receipts: &["receipts/p01/pass_0043_canonical_data_model.receipt"], roles: &["developer", "operator"], targets: &["canonical_data_model"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_core_engine", binary: "src/bin/lyra-p01-semantic-core-engine-check.rs", surface: "LYRA-P01-SEMANTIC-CORE-ENGINE v1", input: "fixtures/p01/semantic_core_engine_inputs/valid_semantic_core_engine.lyra", output: "receipts/p01/pass_0044_semantic_core_engine.receipt", receipts: &["receipts/p01/pass_0044_semantic_core_engine.receipt"], roles: &["developer", "operator"], targets: &["semantic_core_engine", "core_ir"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_falsification", binary: "src/bin/lyra-p01-semantic-falsification-check.rs", surface: "LYRA-P01-SEMANTIC-FALSIFICATION-CORPUS v1", input: "fixtures/p01/semantic_falsification_inputs/valid_semantic_falsification.lyra", output: "receipts/p01/pass_0045_semantic_falsification.receipt", receipts: &["receipts/p01/pass_0045_semantic_falsification.receipt"], roles: &["developer", "red_team"], targets: &["falsification", "semantic_atoms", "core_ir"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_replay", binary: "src/bin/lyra-p01-semantic-replay-check.rs", surface: "LYRA-P01-SEMANTIC-REPLAY-WITNESS v1", input: "fixtures/p01/semantic_replay_inputs/valid_semantic_replay.lyra", output: "receipts/p01/pass_0046_semantic_replay.receipt", receipts: &["receipts/p01/pass_0046_semantic_replay.receipt"], roles: &["developer", "proof_auditor"], targets: &["replay"], status: "artifact_emitted" },
    SemanticInterfaceCommandDescriptor { id: "validate_semantic_interface", binary: "src/bin/lyra-p01-semantic-interface-check.rs", surface: "LYRA-P01-SEMANTIC-INTERFACE v1", input: "fixtures/p01/semantic_interface_inputs/valid_semantic_interface.lyra", output: "receipts/p01/pass_0047_semantic_interface.receipt", receipts: &["receipts/p01/pass_0047_semantic_interface.receipt"], roles: &["developer", "operator", "proof_auditor"], targets: &["canonical_symbols", "semantic_atoms", "core_ir"], status: "artifact_emitted" },
];

pub const LYRALANG_SEMANTIC_INTERFACE_WORKFLOWS: &[SemanticInterfaceWorkflowDescriptor] = &[
    SemanticInterfaceWorkflowDescriptor {
        id: "developer_local_semantic_check",
        order: "001",
        commands: &[
            "validate_semantic_atoms",
            "validate_core_ir",
            "validate_semantic_objects",
            "validate_semantic_identity",
        ],
        targets: &[
            "semantic_atoms",
            "core_ir",
            "semantic_objects",
            "semantic_identity",
        ],
        examples: &["semantic_interface_review"],
        forbids: &[
            "network_required",
            "manual_only",
            "interface_drift_accepted",
        ],
        status: "execution_proven",
    },
    SemanticInterfaceWorkflowDescriptor {
        id: "operator_core_ir_review",
        order: "002",
        commands: &[
            "validate_core_ir",
            "validate_core_ir_reuse",
            "validate_semantic_core_engine",
        ],
        targets: &["core_ir", "core_ir_reuse", "semantic_core_engine"],
        examples: &[
            "core_ir_operator_review",
            "semantic_core_engine_operator_review",
        ],
        forbids: &["unreceipted_output", "ambient_randomness"],
        status: "execution_proven",
    },
    SemanticInterfaceWorkflowDescriptor {
        id: "negative_corpus_review",
        order: "003",
        commands: &[
            "validate_error_challenge_evidence",
            "validate_semantic_adversarial_corpus",
            "validate_semantic_falsification",
        ],
        targets: &[
            "error_challenge_evidence",
            "adversarial_corpus",
            "falsification",
        ],
        examples: &["semantic_falsification_operator_review"],
        forbids: &["negative_acceptance", "corpus_drift_accepted"],
        status: "execution_proven",
    },
    SemanticInterfaceWorkflowDescriptor {
        id: "replay_receipt_audit",
        order: "004",
        commands: &[
            "validate_semantic_bedrock_receipts",
            "validate_semantic_replay",
        ],
        targets: &["bedrock_receipts", "replay"],
        examples: &["semantic_replay_operator_review"],
        forbids: &["mutable_replay", "missing_receipt"],
        status: "execution_proven",
    },
    SemanticInterfaceWorkflowDescriptor {
        id: "full_p01_semantic_frontier_review",
        order: "005",
        commands: &[
            "validate_semantic_atoms",
            "validate_core_ir",
            "validate_semantic_objects",
            "validate_semantic_identity",
            "validate_reference_semantics",
            "validate_symbolic_equality",
            "validate_error_challenge_evidence",
            "validate_semantic_serialization_hashing",
            "validate_semantic_adversarial_corpus",
            "validate_core_ir_reuse",
            "validate_semantic_atom_reference",
            "validate_semantic_bedrock_receipts",
            "validate_formal_semantic_constitution",
            "validate_canonical_data_model",
            "validate_semantic_core_engine",
            "validate_semantic_falsification",
            "validate_semantic_replay",
            "validate_semantic_interface",
        ],
        targets: &["canonical_symbols", "semantic_atoms", "core_ir"],
        examples: &[
            "semantic_interface_review",
            "semantic_falsification_operator_review",
            "semantic_replay_operator_review",
        ],
        forbids: &["phase_closed", "manual_only", "network_required"],
        status: "execution_proven",
    },
];

pub const LYRALANG_SEMANTIC_INTERFACE_EXAMPLES: &[SemanticInterfaceExampleDescriptor] = &[
    SemanticInterfaceExampleDescriptor {
        id: "semantic_interface_review",
        path: "examples/p01/operator/semantic_interface_review.lyra",
        commands: &["validate_semantic_interface"],
        expected_receipts: &["receipts/p01/pass_0047_semantic_interface.receipt"],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    SemanticInterfaceExampleDescriptor {
        id: "core_ir_operator_review",
        path: "examples/p01/operator/core_ir_review.lyra",
        commands: &["validate_core_ir"],
        expected_receipts: &["receipts/p01/pass_0031_core_ir.receipt"],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    SemanticInterfaceExampleDescriptor {
        id: "semantic_core_engine_operator_review",
        path: "examples/p01/operator/semantic_core_engine_review.lyra",
        commands: &["validate_semantic_core_engine"],
        expected_receipts: &["receipts/p01/pass_0044_semantic_core_engine.receipt"],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    SemanticInterfaceExampleDescriptor {
        id: "semantic_falsification_operator_review",
        path: "examples/p01/operator/semantic_falsification_review.lyra",
        commands: &["validate_semantic_falsification"],
        expected_receipts: &["receipts/p01/pass_0045_semantic_falsification.receipt"],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    SemanticInterfaceExampleDescriptor {
        id: "semantic_replay_operator_review",
        path: "examples/p01/operator/semantic_replay_review.lyra",
        commands: &["validate_semantic_replay"],
        expected_receipts: &["receipts/p01/pass_0046_semantic_replay.receipt"],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    SemanticInterfaceExampleDescriptor {
        id: "negative_interface_rejection_review",
        path: "fixtures/p01/semantic_interface_inputs/invalid_manual_only_interface.lyra",
        commands: &["validate_semantic_interface"],
        expected_receipts: &["receipts/p01/pass_0047_semantic_interface.receipt"],
        expected_verdict: "rejected",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_INTERFACE_PROOFS: &[SemanticInterfaceProofDescriptor] = &[
    SemanticInterfaceProofDescriptor {
        id: "command_manifest_coverage",
        scope: "command",
        commands: &[
            "validate_semantic_atoms",
            "validate_core_ir",
            "validate_semantic_objects",
            "validate_semantic_identity",
            "validate_reference_semantics",
            "validate_symbolic_equality",
            "validate_error_challenge_evidence",
            "validate_semantic_serialization_hashing",
            "validate_semantic_adversarial_corpus",
            "validate_core_ir_reuse",
            "validate_semantic_atom_reference",
            "validate_semantic_bedrock_receipts",
            "validate_formal_semantic_constitution",
            "validate_canonical_data_model",
            "validate_semantic_core_engine",
            "validate_semantic_falsification",
            "validate_semantic_replay",
            "validate_semantic_interface",
        ],
        workflows: &[
            "developer_local_semantic_check",
            "full_p01_semantic_frontier_review",
        ],
        examples: &["semantic_interface_review"],
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
        ],
        forbids: &["missing_command", "unknown_binary", "network_required"],
        status: "execution_proven",
    },
    SemanticInterfaceProofDescriptor {
        id: "workflow_ordering_determinism",
        scope: "workflow",
        commands: &["validate_semantic_interface"],
        workflows: &[
            "developer_local_semantic_check",
            "operator_core_ir_review",
            "negative_corpus_review",
            "replay_receipt_audit",
            "full_p01_semantic_frontier_review",
        ],
        examples: &["semantic_interface_review"],
        receipts: &["receipts/p01/pass_0047_semantic_interface.receipt"],
        forbids: &["duplicate_order", "unbound_command"],
        status: "execution_proven",
    },
    SemanticInterfaceProofDescriptor {
        id: "example_receipt_binding",
        scope: "example",
        commands: &[
            "validate_semantic_interface",
            "validate_semantic_falsification",
            "validate_semantic_replay",
        ],
        workflows: &["negative_corpus_review", "replay_receipt_audit"],
        examples: &[
            "semantic_interface_review",
            "core_ir_operator_review",
            "semantic_core_engine_operator_review",
            "semantic_falsification_operator_review",
            "semantic_replay_operator_review",
            "negative_interface_rejection_review",
        ],
        receipts: &[
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0044_semantic_core_engine.receipt",
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0047_semantic_interface.receipt",
        ],
        forbids: &["unreceipted_example", "wrong_verdict"],
        status: "execution_proven",
    },
    SemanticInterfaceProofDescriptor {
        id: "negative_rejection_interface",
        scope: "challenge",
        commands: &[
            "validate_semantic_interface",
            "validate_semantic_falsification",
        ],
        workflows: &["negative_corpus_review"],
        examples: &["negative_interface_rejection_review"],
        receipts: &["receipts/p01/pass_0047_semantic_interface.receipt"],
        forbids: &[
            "manual_only",
            "network_required",
            "interface_drift_accepted",
        ],
        status: "execution_proven",
    },
    SemanticInterfaceProofDescriptor {
        id: "p01_phase_open",
        scope: "phase",
        commands: &["validate_semantic_interface", "validate_semantic_replay"],
        workflows: &["full_p01_semantic_frontier_review"],
        examples: &["semantic_interface_review"],
        receipts: &["receipts/p01/pass_0047_semantic_interface.receipt"],
        forbids: &["phase_closed", "global_complete"],
        status: "working_slice",
    },
];

pub const LYRALANG_SEMANTIC_INTERFACE_ARTIFACTS: &[SemanticInterfaceArtifactDescriptor] = &[
    SemanticInterfaceArtifactDescriptor {
        id: "deterministic_interface_report",
        owner_root: "k0",
        path: "k0/determinism/src/semantic_interface.rs",
        artifact_kind: "deterministic_report",
        commands: &["validate_semantic_interface"],
        status: "artifact_emitted",
    },
    SemanticInterfaceArtifactDescriptor {
        id: "lyralang_interface_registry",
        owner_root: "lyralang",
        path: "lyralang/core/src/semantic_interface.rs",
        artifact_kind: "registry",
        commands: &["validate_semantic_interface"],
        status: "artifact_emitted",
    },
    SemanticInterfaceArtifactDescriptor {
        id: "interface_model_contract",
        owner_root: "interfaces",
        path: "interfaces/p01/src/semantic_interface_model.rs",
        artifact_kind: "model",
        commands: &["validate_semantic_interface"],
        status: "artifact_emitted",
    },
    SemanticInterfaceArtifactDescriptor {
        id: "interface_validator",
        owner_root: "ops",
        path: "ops/p01/src/semantic_interface.rs",
        artifact_kind: "validator",
        commands: &["validate_semantic_interface"],
        status: "artifact_emitted",
    },
    SemanticInterfaceArtifactDescriptor {
        id: "interface_operator_binary",
        owner_root: "src",
        path: "src/bin/lyra-p01-semantic-interface-check.rs",
        artifact_kind: "operator_binary",
        commands: &["validate_semantic_interface"],
        status: "artifact_emitted",
    },
    SemanticInterfaceArtifactDescriptor {
        id: "operator_review_example",
        owner_root: "examples",
        path: "examples/p01/operator/semantic_interface_review.lyra",
        artifact_kind: "operator_example",
        commands: &["validate_semantic_interface"],
        status: "artifact_emitted",
    },
    SemanticInterfaceArtifactDescriptor {
        id: "product_inspection_surface",
        owner_root: "products",
        path: "products/p01/semantic_interface_inspection_surface.lyra",
        artifact_kind: "inspection_surface",
        commands: &["validate_semantic_interface"],
        status: "artifact_emitted",
    },
    SemanticInterfaceArtifactDescriptor {
        id: "interface_tests",
        owner_root: "tests",
        path: "tests/p01_semantic_interface_tests.rs",
        artifact_kind: "test_suite",
        commands: &["validate_semantic_interface"],
        status: "artifact_emitted",
    },
];

pub fn semantic_interface_command_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_INTERFACE_COMMANDS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_interface_workflow_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_INTERFACE_WORKFLOWS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_interface_example_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_INTERFACE_EXAMPLES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_interface_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_INTERFACE_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_interface_artifact_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_INTERFACE_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_interface_command_descriptor(
    id: &str,
) -> Option<&'static SemanticInterfaceCommandDescriptor> {
    LYRALANG_SEMANTIC_INTERFACE_COMMANDS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_interface_workflow_descriptor(
    id: &str,
) -> Option<&'static SemanticInterfaceWorkflowDescriptor> {
    LYRALANG_SEMANTIC_INTERFACE_WORKFLOWS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_interface_example_descriptor(
    id: &str,
) -> Option<&'static SemanticInterfaceExampleDescriptor> {
    LYRALANG_SEMANTIC_INTERFACE_EXAMPLES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_interface_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticInterfaceProofDescriptor> {
    LYRALANG_SEMANTIC_INTERFACE_PROOFS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_interface_artifact_descriptor(
    id: &str,
) -> Option<&'static SemanticInterfaceArtifactDescriptor> {
    LYRALANG_SEMANTIC_INTERFACE_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_interface_command_signature(item: &SemanticInterfaceCommandDescriptor) -> String {
    format!("command:{}|binary:{}|surface:{}|input:{}|output:{}|receipts:{}|roles:{}|targets:{}|status:{}", item.id, item.binary, item.surface, item.input, item.output, item.receipts.join(","), item.roles.join(","), item.targets.join(","), item.status)
}
pub fn semantic_interface_workflow_signature(item: &SemanticInterfaceWorkflowDescriptor) -> String {
    format!(
        "workflow:{}|order:{}|commands:{}|targets:{}|examples:{}|forbids:{}|status:{}",
        item.id,
        item.order,
        item.commands.join(","),
        item.targets.join(","),
        item.examples.join(","),
        item.forbids.join(","),
        item.status
    )
}
pub fn semantic_interface_example_signature(item: &SemanticInterfaceExampleDescriptor) -> String {
    format!(
        "example:{}|path:{}|commands:{}|expected_receipts:{}|expected_verdict:{}|status:{}",
        item.id,
        item.path,
        item.commands.join(","),
        item.expected_receipts.join(","),
        item.expected_verdict,
        item.status
    )
}
pub fn semantic_interface_proof_signature(item: &SemanticInterfaceProofDescriptor) -> String {
    format!(
        "proof:{}|scope:{}|commands:{}|workflows:{}|examples:{}|receipts:{}|forbids:{}|status:{}",
        item.id,
        item.scope,
        item.commands.join(","),
        item.workflows.join(","),
        item.examples.join(","),
        item.receipts.join(","),
        item.forbids.join(","),
        item.status
    )
}
pub fn semantic_interface_artifact_signature(item: &SemanticInterfaceArtifactDescriptor) -> String {
    format!(
        "artifact:{}|owner:{}|path:{}|kind:{}|commands:{}|status:{}",
        item.id,
        item.owner_root,
        item.path,
        item.artifact_kind,
        item.commands.join(","),
        item.status
    )
}

pub fn semantic_interface_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SEMANTIC_INTERFACE_COMMANDS {
        rows.push(semantic_interface_command_signature(item));
    }
    for item in LYRALANG_SEMANTIC_INTERFACE_WORKFLOWS {
        rows.push(semantic_interface_workflow_signature(item));
    }
    for item in LYRALANG_SEMANTIC_INTERFACE_EXAMPLES {
        rows.push(semantic_interface_example_signature(item));
    }
    for item in LYRALANG_SEMANTIC_INTERFACE_PROOFS {
        rows.push(semantic_interface_proof_signature(item));
    }
    for item in LYRALANG_SEMANTIC_INTERFACE_ARTIFACTS {
        rows.push(semantic_interface_artifact_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_interface_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_interface.registry",
        &semantic_interface_registry_signature(),
    )
}
pub fn semantic_interface_command_digest(id: &str) -> Option<String> {
    semantic_interface_command_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_interface.command_descriptor",
            &semantic_interface_command_signature(item),
        )
    })
}
pub fn semantic_interface_workflow_digest(id: &str) -> Option<String> {
    semantic_interface_workflow_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_interface.workflow_descriptor",
            &semantic_interface_workflow_signature(item),
        )
    })
}
pub fn semantic_interface_example_digest(id: &str) -> Option<String> {
    semantic_interface_example_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_interface.example_descriptor",
            &semantic_interface_example_signature(item),
        )
    })
}
pub fn semantic_interface_proof_digest(id: &str) -> Option<String> {
    semantic_interface_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_interface.proof_descriptor",
            &semantic_interface_proof_signature(item),
        )
    })
}
pub fn semantic_interface_artifact_digest(id: &str) -> Option<String> {
    semantic_interface_artifact_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_interface.artifact_descriptor",
            &semantic_interface_artifact_signature(item),
        )
    })
}

pub fn semantic_interface_workflows_bind_known_commands() -> bool {
    LYRALANG_SEMANTIC_INTERFACE_WORKFLOWS
        .iter()
        .all(|workflow| {
            workflow
                .commands
                .iter()
                .all(|id| semantic_interface_command_descriptor(id).is_some())
                && workflow
                    .examples
                    .iter()
                    .all(|id| semantic_interface_example_descriptor(id).is_some())
        })
}

pub fn semantic_interface_examples_bind_known_commands() -> bool {
    LYRALANG_SEMANTIC_INTERFACE_EXAMPLES.iter().all(|example| {
        example
            .commands
            .iter()
            .all(|id| semantic_interface_command_descriptor(id).is_some())
            && !example.expected_receipts.is_empty()
    })
}

pub fn semantic_interface_proofs_bind_registry() -> bool {
    LYRALANG_SEMANTIC_INTERFACE_PROOFS.iter().all(|proof| {
        proof
            .commands
            .iter()
            .all(|id| semantic_interface_command_descriptor(id).is_some())
            && proof
                .workflows
                .iter()
                .all(|id| semantic_interface_workflow_descriptor(id).is_some())
            && proof
                .examples
                .iter()
                .all(|id| semantic_interface_example_descriptor(id).is_some())
            && !proof.receipts.is_empty()
            && !proof.forbids.is_empty()
    })
}

pub fn semantic_interface_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_INTERFACE_ARTIFACTS
        .iter()
        .all(|artifact| {
            !artifact.path.contains("..")
                && artifact.path.starts_with(artifact.owner_root)
                && artifact
                    .commands
                    .iter()
                    .all(|id| semantic_interface_command_descriptor(id).is_some())
        })
}

pub fn semantic_interface_commands_cover_p01_001_through_p01_018() -> bool {
    let expected = [
        "validate_semantic_atoms",
        "validate_core_ir",
        "validate_semantic_objects",
        "validate_semantic_identity",
        "validate_reference_semantics",
        "validate_symbolic_equality",
        "validate_error_challenge_evidence",
        "validate_semantic_serialization_hashing",
        "validate_semantic_adversarial_corpus",
        "validate_core_ir_reuse",
        "validate_semantic_atom_reference",
        "validate_semantic_bedrock_receipts",
        "validate_formal_semantic_constitution",
        "validate_canonical_data_model",
        "validate_semantic_core_engine",
        "validate_semantic_falsification",
        "validate_semantic_replay",
        "validate_semantic_interface",
    ];
    expected
        .iter()
        .all(|id| semantic_interface_command_descriptor(id).is_some())
}

pub fn semantic_interface_no_forbidden_descriptor_claims() -> bool {
    !semantic_interface_registry_signature().contains("manual only")
        && !semantic_interface_registry_signature().contains("network required")
        && !semantic_interface_registry_signature().contains("phase closed")
        && !semantic_interface_registry_signature().contains("interface drift accepted")
}
