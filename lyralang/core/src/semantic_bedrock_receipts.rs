use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticBedrockReceiptDescriptor {
    pub id: &'static str,
    pub task: &'static str,
    pub surface: &'static str,
    pub path: &'static str,
    pub expected_hash: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticBedrockAnchorDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub module: &'static str,
    pub contract: &'static str,
    pub law: &'static str,
    pub receipt_ref: &'static str,
    pub core_ref: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticBedrockParityFixtureDescriptor {
    pub id: &'static str,
    pub positive: &'static str,
    pub negative: &'static str,
    pub receipt_ref: &'static str,
    pub golden: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticBedrockGateDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub law: &'static str,
    pub evidence: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticBedrockReceiptError {
    UnknownReceipt { id: String },
    UnknownAnchor { id: String },
    UnknownFixture { id: String },
    UnknownGate { id: String },
}

pub const LYRA_P01_SEMANTIC_CORE_REF: &str = "lyra_p01_semantic_core";

pub const LYRALANG_SEMANTIC_BEDROCK_RECEIPTS: &[SemanticBedrockReceiptDescriptor] = &[
    SemanticBedrockReceiptDescriptor {
        id: "receipt_semantic_atoms",
        task: "P01-001",
        surface: "LYRA-P01-SEMANTIC-ATOMS v1",
        path: "receipts/p01/pass_0030_semantic_atoms.receipt",
        expected_hash: "core_atoms_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_core_ir",
        task: "P01-002",
        surface: "LYRA-P01-CORE-IR v1",
        path: "receipts/p01/pass_0031_core_ir.receipt",
        expected_hash: "core_ir_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_semantic_objects",
        task: "P01-003",
        surface: "LYRA-P01-SEMANTIC-OBJECTS v1",
        path: "receipts/p01/pass_0032_semantic_objects.receipt",
        expected_hash: "semantic_objects_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_semantic_identity",
        task: "P01-004",
        surface: "LYRA-P01-SEMANTIC-IDENTITY v1",
        path: "receipts/p01/pass_0033_semantic_identity.receipt",
        expected_hash: "semantic_identity_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_reference_semantics",
        task: "P01-005",
        surface: "LYRA-P01-REFERENCE-SEMANTICS v1",
        path: "receipts/p01/pass_0034_reference_semantics.receipt",
        expected_hash: "reference_semantics_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_symbolic_equality",
        task: "P01-006",
        surface: "LYRA-P01-SYMBOLIC-EQUALITY v1",
        path: "receipts/p01/pass_0035_symbolic_equality.receipt",
        expected_hash: "symbolic_equality_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_error_challenge_evidence",
        task: "P01-007",
        surface: "LYRA-P01-ERROR-CHALLENGE-EVIDENCE v1",
        path: "receipts/p01/pass_0036_error_challenge_evidence.receipt",
        expected_hash: "error_challenge_evidence_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_semantic_serialization_hashing",
        task: "P01-008",
        surface: "LYRA-P01-SEMANTIC-SERIALIZATION-HASHING v1",
        path: "receipts/p01/pass_0037_semantic_serialization_hashing.receipt",
        expected_hash: "semantic_serialization_hashing_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_semantic_adversarial_corpus",
        task: "P01-009",
        surface: "LYRA-P01-SEMANTIC-ADVERSARIAL-CORPUS v1",
        path: "receipts/p01/pass_0038_semantic_adversarial_corpus.receipt",
        expected_hash: "semantic_adversarial_corpus_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_core_ir_reuse",
        task: "P01-010",
        surface: "LYRA-P01-CORE-IR-REUSE v1",
        path: "receipts/p01/pass_0039_core_ir_reuse.receipt",
        expected_hash: "core_ir_reuse_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_semantic_atom_reference",
        task: "P01-011",
        surface: "LYRA-P01-SEMANTIC-ATOM-REFERENCE v1",
        path: "receipts/p01/pass_0040_semantic_atom_reference.receipt",
        expected_hash: "semantic_atom_reference_receipt_hash_bound",
        status: "artifact_emitted",
    },
    SemanticBedrockReceiptDescriptor {
        id: "receipt_semantic_bedrock_receipts",
        task: "P01-012",
        surface: "LYRA-P01-SEMANTIC-BEDROCK-RECEIPTS v1",
        path: "receipts/p01/pass_0041_semantic_bedrock_receipts.receipt",
        expected_hash: "semantic_bedrock_receipts_self_hash_bound",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_BEDROCK_ANCHORS: &[SemanticBedrockAnchorDescriptor] = &[
    SemanticBedrockAnchorDescriptor {
        id: "semantic_atoms_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_semantic_atoms",
        contract: "interfaces/p01/contracts/semantic_atoms.v1.lyra",
        law: "ops/p01/control/semantic_atoms_law.v1.lyra",
        receipt_ref: "receipt_semantic_atoms",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "core_ir_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_core_ir",
        contract: "interfaces/p01/contracts/core_ir.v1.lyra",
        law: "ops/p01/control/core_ir_law.v1.lyra",
        receipt_ref: "receipt_core_ir",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "semantic_objects_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_semantic_objects",
        contract: "interfaces/p01/contracts/semantic_objects.v1.lyra",
        law: "ops/p01/control/semantic_objects_law.v1.lyra",
        receipt_ref: "receipt_semantic_objects",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "semantic_identity_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_semantic_identity",
        contract: "interfaces/p01/contracts/semantic_identity.v1.lyra",
        law: "ops/p01/control/semantic_identity_law.v1.lyra",
        receipt_ref: "receipt_semantic_identity",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "reference_semantics_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_reference_semantics",
        contract: "interfaces/p01/contracts/reference_semantics.v1.lyra",
        law: "ops/p01/control/reference_semantics_law.v1.lyra",
        receipt_ref: "receipt_reference_semantics",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "symbolic_equality_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_symbolic_equality",
        contract: "interfaces/p01/contracts/symbolic_equality.v1.lyra",
        law: "ops/p01/control/symbolic_equality_law.v1.lyra",
        receipt_ref: "receipt_symbolic_equality",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "error_challenge_evidence_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_error_challenge_evidence",
        contract: "interfaces/p01/contracts/error_challenge_evidence.v1.lyra",
        law: "ops/p01/control/error_challenge_evidence_law.v1.lyra",
        receipt_ref: "receipt_error_challenge_evidence",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "semantic_serialization_hashing_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_semantic_serialization_hashing",
        contract: "interfaces/p01/contracts/semantic_serialization_hashing.v1.lyra",
        law: "ops/p01/control/semantic_serialization_hashing_law.v1.lyra",
        receipt_ref: "receipt_semantic_serialization_hashing",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "semantic_adversarial_corpus_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_semantic_adversarial_corpus",
        contract: "interfaces/p01/contracts/semantic_adversarial_corpus.v1.lyra",
        law: "ops/p01/control/semantic_adversarial_corpus_law.v1.lyra",
        receipt_ref: "receipt_semantic_adversarial_corpus",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "core_ir_reuse_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_core_ir_reuse",
        contract: "interfaces/p01/contracts/core_ir_reuse.v1.lyra",
        law: "ops/p01/control/core_ir_reuse_law.v1.lyra",
        receipt_ref: "receipt_core_ir_reuse",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    SemanticBedrockAnchorDescriptor {
        id: "semantic_atom_reference_core_anchor",
        owner_root: "lyralang",
        module: "lyralang_semantic_atom_reference",
        contract: "interfaces/p01/contracts/semantic_atom_reference.v1.lyra",
        law: "ops/p01/control/semantic_atom_reference_law.v1.lyra",
        receipt_ref: "receipt_semantic_atom_reference",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_BEDROCK_PARITY_FIXTURES: &[SemanticBedrockParityFixtureDescriptor] = &[
    SemanticBedrockParityFixtureDescriptor { id: "semantic_atoms_receipt_parity", positive: "fixtures/p01/semantic_atom_inputs/valid_semantic_atoms.lyra", negative: "fixtures/p01/semantic_atom_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_semantic_atoms", golden: "goldens/p01/valid_semantic_atoms.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "core_ir_receipt_parity", positive: "fixtures/p01/core_ir_inputs/valid_core_ir_forms.lyra", negative: "fixtures/p01/core_ir_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_core_ir", golden: "goldens/p01/valid_core_ir_forms.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "semantic_objects_receipt_parity", positive: "fixtures/p01/semantic_object_inputs/valid_semantic_objects.lyra", negative: "fixtures/p01/semantic_object_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_semantic_objects", golden: "goldens/p01/valid_semantic_objects.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "semantic_identity_receipt_parity", positive: "fixtures/p01/semantic_identity_inputs/valid_semantic_identity.lyra", negative: "fixtures/p01/semantic_identity_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_semantic_identity", golden: "goldens/p01/valid_semantic_identity.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "reference_semantics_receipt_parity", positive: "fixtures/p01/reference_semantics_inputs/valid_reference_semantics.lyra", negative: "fixtures/p01/reference_semantics_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_reference_semantics", golden: "goldens/p01/valid_reference_semantics.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "symbolic_equality_receipt_parity", positive: "fixtures/p01/symbolic_equality_inputs/valid_symbolic_equality.lyra", negative: "fixtures/p01/symbolic_equality_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_symbolic_equality", golden: "goldens/p01/valid_symbolic_equality.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "error_challenge_evidence_receipt_parity", positive: "fixtures/p01/error_challenge_evidence_inputs/valid_error_challenge_evidence.lyra", negative: "fixtures/p01/error_challenge_evidence_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_error_challenge_evidence", golden: "goldens/p01/valid_error_challenge_evidence.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "semantic_serialization_hashing_receipt_parity", positive: "fixtures/p01/semantic_serialization_hashing_inputs/valid_semantic_serialization_hashing.lyra", negative: "fixtures/p01/semantic_serialization_hashing_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_semantic_serialization_hashing", golden: "goldens/p01/valid_semantic_serialization_hashing.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "semantic_adversarial_corpus_receipt_parity", positive: "fixtures/p01/semantic_adversarial_corpus_inputs/valid_semantic_adversarial_corpus.lyra", negative: "fixtures/p01/semantic_adversarial_corpus_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_semantic_adversarial_corpus", golden: "goldens/p01/valid_semantic_adversarial_corpus.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "core_ir_reuse_receipt_parity", positive: "fixtures/p01/core_ir_reuse_inputs/valid_core_ir_reuse.lyra", negative: "fixtures/p01/core_ir_reuse_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_core_ir_reuse", golden: "goldens/p01/valid_core_ir_reuse.receipt", status: "artifact_emitted" },
    SemanticBedrockParityFixtureDescriptor { id: "semantic_atom_reference_receipt_parity", positive: "fixtures/p01/semantic_atom_reference_inputs/valid_semantic_atom_reference.lyra", negative: "fixtures/p01/semantic_atom_reference_inputs/invalid_missing_receipt.lyra", receipt_ref: "receipt_semantic_atom_reference", golden: "goldens/p01/valid_semantic_atom_reference.receipt", status: "artifact_emitted" },
];

pub const LYRALANG_SEMANTIC_BEDROCK_GATES: &[SemanticBedrockGateDescriptor] = &[
    SemanticBedrockGateDescriptor {
        id: "receipt_chain_complete_gate",
        scope: "receipt_chain",
        law: "all_admitted_p01_receipts_must_be_present",
        evidence: "semantic_bedrock_receipts_cover_core_chain",
        status: "execution_proven",
    },
    SemanticBedrockGateDescriptor {
        id: "one_core_anchor_gate",
        scope: "core_anchor",
        law: "all_receipts_must_bind_lyra_p01_semantic_core",
        evidence: "semantic_bedrock_anchors_point_to_one_core",
        status: "execution_proven",
    },
    SemanticBedrockGateDescriptor {
        id: "parity_fixture_gate",
        scope: "parity_fixtures",
        law: "positive_and_negative_receipt_fixtures_must_exist_per_frontier",
        evidence: "semantic_bedrock_parity_fixtures_cover_receipts",
        status: "execution_proven",
    },
    SemanticBedrockGateDescriptor {
        id: "no_forked_bedrock_gate",
        scope: "semantic_core",
        law: "forked_semantic_core_claims_are_rejected",
        evidence: "semantic_bedrock_no_forked_core_claims",
        status: "execution_proven",
    },
];

pub fn semantic_bedrock_receipt_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_BEDROCK_RECEIPTS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_bedrock_anchor_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_BEDROCK_ANCHORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_bedrock_parity_fixture_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_BEDROCK_PARITY_FIXTURES
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_bedrock_gate_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_BEDROCK_GATES
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_bedrock_receipt_descriptor(
    id: &str,
) -> Option<&'static SemanticBedrockReceiptDescriptor> {
    LYRALANG_SEMANTIC_BEDROCK_RECEIPTS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_bedrock_anchor_descriptor(
    id: &str,
) -> Option<&'static SemanticBedrockAnchorDescriptor> {
    LYRALANG_SEMANTIC_BEDROCK_ANCHORS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_bedrock_parity_fixture_descriptor(
    id: &str,
) -> Option<&'static SemanticBedrockParityFixtureDescriptor> {
    LYRALANG_SEMANTIC_BEDROCK_PARITY_FIXTURES
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_bedrock_gate_descriptor(
    id: &str,
) -> Option<&'static SemanticBedrockGateDescriptor> {
    LYRALANG_SEMANTIC_BEDROCK_GATES
        .iter()
        .find(|item| item.id == id)
}

pub fn canonical_semantic_bedrock_receipt_signature(
    item: &SemanticBedrockReceiptDescriptor,
) -> String {
    format!(
        "receipt:{}|task:{}|surface:{}|path:{}|expected:{}|status:{}",
        item.id, item.task, item.surface, item.path, item.expected_hash, item.status
    )
}

pub fn canonical_semantic_bedrock_anchor_signature(
    item: &SemanticBedrockAnchorDescriptor,
) -> String {
    format!(
        "anchor:{}|owner:{}|module:{}|contract:{}|law:{}|receipt:{}|core:{}|status:{}",
        item.id,
        item.owner_root,
        item.module,
        item.contract,
        item.law,
        item.receipt_ref,
        item.core_ref,
        item.status
    )
}

pub fn canonical_semantic_bedrock_parity_fixture_signature(
    item: &SemanticBedrockParityFixtureDescriptor,
) -> String {
    format!(
        "fixture:{}|positive:{}|negative:{}|receipt:{}|golden:{}|status:{}",
        item.id, item.positive, item.negative, item.receipt_ref, item.golden, item.status
    )
}

pub fn canonical_semantic_bedrock_gate_signature(item: &SemanticBedrockGateDescriptor) -> String {
    format!(
        "gate:{}|scope:{}|law:{}|evidence:{}|status:{}",
        item.id, item.scope, item.law, item.evidence, item.status
    )
}

pub fn canonical_semantic_bedrock_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SEMANTIC_BEDROCK_RECEIPTS {
        rows.push(canonical_semantic_bedrock_receipt_signature(item));
    }
    for item in LYRALANG_SEMANTIC_BEDROCK_ANCHORS {
        rows.push(canonical_semantic_bedrock_anchor_signature(item));
    }
    for item in LYRALANG_SEMANTIC_BEDROCK_PARITY_FIXTURES {
        rows.push(canonical_semantic_bedrock_parity_fixture_signature(item));
    }
    for item in LYRALANG_SEMANTIC_BEDROCK_GATES {
        rows.push(canonical_semantic_bedrock_gate_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn canonical_semantic_bedrock_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_bedrock_receipts.registry",
        &canonical_semantic_bedrock_registry_signature(),
    )
}

pub fn semantic_bedrock_receipt_digest(id: &str) -> Option<String> {
    semantic_bedrock_receipt_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_bedrock_receipts.receipt",
            &canonical_semantic_bedrock_receipt_signature(item),
        )
    })
}

pub fn semantic_bedrock_anchor_digest(id: &str) -> Option<String> {
    semantic_bedrock_anchor_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_bedrock_receipts.anchor",
            &canonical_semantic_bedrock_anchor_signature(item),
        )
    })
}

pub fn semantic_bedrock_parity_fixture_digest(id: &str) -> Option<String> {
    semantic_bedrock_parity_fixture_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_bedrock_receipts.fixture",
            &canonical_semantic_bedrock_parity_fixture_signature(item),
        )
    })
}

pub fn semantic_bedrock_gate_digest(id: &str) -> Option<String> {
    semantic_bedrock_gate_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_bedrock_receipts.gate",
            &canonical_semantic_bedrock_gate_signature(item),
        )
    })
}

pub fn semantic_bedrock_receipts_cover_core_chain() -> bool {
    let tasks = [
        "P01-001", "P01-002", "P01-003", "P01-004", "P01-005", "P01-006", "P01-007", "P01-008",
        "P01-009", "P01-010", "P01-011", "P01-012",
    ];
    tasks.iter().all(|task| {
        LYRALANG_SEMANTIC_BEDROCK_RECEIPTS
            .iter()
            .any(|item| item.task == *task)
    })
}

pub fn semantic_bedrock_anchors_point_to_one_core() -> bool {
    LYRALANG_SEMANTIC_BEDROCK_ANCHORS.iter().all(|item| {
        item.core_ref == LYRA_P01_SEMANTIC_CORE_REF
            && semantic_bedrock_receipt_descriptor(item.receipt_ref).is_some()
    })
}

pub fn semantic_bedrock_parity_fixtures_cover_receipts() -> bool {
    LYRALANG_SEMANTIC_BEDROCK_ANCHORS.iter().all(|anchor| {
        LYRALANG_SEMANTIC_BEDROCK_PARITY_FIXTURES
            .iter()
            .any(|fixture| fixture.receipt_ref == anchor.receipt_ref)
    })
}

pub fn semantic_bedrock_no_forked_core_claims() -> bool {
    LYRALANG_SEMANTIC_BEDROCK_ANCHORS
        .iter()
        .all(|item| item.core_ref == LYRA_P01_SEMANTIC_CORE_REF)
}
