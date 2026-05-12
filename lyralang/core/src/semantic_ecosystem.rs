use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticEcosystemDocDescriptor {
    pub id: &'static str,
    pub audience: &'static str,
    pub path: &'static str,
    pub covers: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticEcosystemExampleDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub commands: &'static [&'static str],
    pub proofs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub rejects: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticEcosystemProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub docs: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P01_SEMANTIC_ECOSYSTEM_CARRIER: &str = "lyra.p01.semantic_ecosystem.carrier.v1";

pub const LYRALANG_SEMANTIC_ECOSYSTEM_DOCS: &[SemanticEcosystemDocDescriptor] = &[
    SemanticEcosystemDocDescriptor {
        id: "semantic_symbol_operator_guide",
        audience: "operator",
        path: "docs/p01/semantic_symbol_operator_guide.lyra",
        covers: &["canonical_symbols", "semantic_atoms", "core_ir"],
        examples: &[
            "canonical_symbol_walkthrough",
            "offline_operator_semantic_review",
        ],
        receipts: &["receipts/p01/pass_0050_semantic_ecosystem.receipt"],
        status: "artifact_emitted",
    },
    SemanticEcosystemDocDescriptor {
        id: "semantic_atom_developer_reference",
        audience: "developer",
        path: "docs/p01/semantic_atom_developer_reference.lyra",
        covers: &["semantic_atoms", "canonical_symbols"],
        examples: &[
            "semantic_atom_extension_flow",
            "negative_semantic_doc_drift_rejection",
        ],
        receipts: &["receipts/p01/pass_0050_semantic_ecosystem.receipt"],
        status: "artifact_emitted",
    },
    SemanticEcosystemDocDescriptor {
        id: "core_ir_ecosystem_walkthrough",
        audience: "developer",
        path: "docs/p01/core_ir_ecosystem_walkthrough.lyra",
        covers: &["core_ir", "canonical_symbols"],
        examples: &["core_ir_receipt_review", "deployment_to_ecosystem_handoff"],
        receipts: &["receipts/p01/pass_0050_semantic_ecosystem.receipt"],
        status: "artifact_emitted",
    },
    SemanticEcosystemDocDescriptor {
        id: "canonical_semantics_contributor_onboarding",
        audience: "contributor",
        path: "docs/p01/canonical_semantics_contributor_onboarding.lyra",
        covers: &["canonical_symbols", "semantic_atoms", "core_ir"],
        examples: &[
            "canonical_symbol_walkthrough",
            "semantic_atom_extension_flow",
        ],
        receipts: &["receipts/p01/pass_0050_semantic_ecosystem.receipt"],
        status: "artifact_emitted",
    },
    SemanticEcosystemDocDescriptor {
        id: "offline_distribution_reference",
        audience: "steward",
        path: "docs/p01/offline_semantic_distribution_reference.lyra",
        covers: &["canonical_symbols", "semantic_atoms", "core_ir"],
        examples: &[
            "offline_operator_semantic_review",
            "deployment_to_ecosystem_handoff",
        ],
        receipts: &[
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        status: "artifact_emitted",
    },
    SemanticEcosystemDocDescriptor {
        id: "public_semantic_review_reference",
        audience: "public",
        path: "docs/p01/public_semantic_review_reference.lyra",
        covers: &["canonical_symbols", "semantic_atoms", "core_ir"],
        examples: &[
            "negative_semantic_doc_drift_rejection",
            "core_ir_receipt_review",
        ],
        receipts: &["receipts/p01/pass_0050_semantic_ecosystem.receipt"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_ECOSYSTEM_EXAMPLES: &[SemanticEcosystemExampleDescriptor] = &[
    SemanticEcosystemExampleDescriptor {
        id: "canonical_symbol_walkthrough",
        kind: "walkthrough",
        path: "examples/p01/ecosystem/canonical_symbol_walkthrough.lyra",
        commands: &["lyra-p01-semantic-ecosystem-check", "lyra-p01-atom-check"],
        proofs: &["docs_coverage_proof", "executable_examples_proof"],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        rejects: &["unstable_symbol_order", "unreceipted_symbol_claim"],
        status: "artifact_emitted",
    },
    SemanticEcosystemExampleDescriptor {
        id: "semantic_atom_extension_flow",
        kind: "extension_flow",
        path: "examples/p01/ecosystem/semantic_atom_extension_flow.lyra",
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-semantic-atom-reference-check",
        ],
        proofs: &["docs_coverage_proof", "executable_examples_proof"],
        receipts: &[
            "receipts/p01/pass_0040_semantic_atom_reference.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        rejects: &["unbound_atom_family", "descriptor_drift"],
        status: "artifact_emitted",
    },
    SemanticEcosystemExampleDescriptor {
        id: "core_ir_receipt_review",
        kind: "review",
        path: "examples/p01/ecosystem/core_ir_receipt_review.lyra",
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-ir-check",
            "lyra-p01-semantic-replay-check",
        ],
        proofs: &["receipt_binding_proof", "executable_examples_proof"],
        receipts: &[
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        rejects: &["receipt_hash_mismatch", "orphan_ir_claim"],
        status: "artifact_emitted",
    },
    SemanticEcosystemExampleDescriptor {
        id: "offline_operator_semantic_review",
        kind: "review",
        path: "examples/p01/ecosystem/offline_operator_semantic_review.lyra",
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-semantic-interface-check",
            "lyra-p01-semantic-deployment-check",
        ],
        proofs: &["offline_distribution_proof", "deployment_bridge_proof"],
        receipts: &[
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        rejects: &["remote_service_required", "unreceipted_operator_step"],
        status: "artifact_emitted",
    },
    SemanticEcosystemExampleDescriptor {
        id: "negative_semantic_doc_drift_rejection",
        kind: "negative",
        path: "fixtures/p01/semantic_ecosystem_inputs/invalid_ecosystem_drift.lyra",
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-semantic-falsification-check",
        ],
        proofs: &["receipt_binding_proof", "p01_phase_open"],
        receipts: &[
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        rejects: &["ecosystem_drift_accepted", "phase_closure"],
        status: "artifact_emitted",
    },
    SemanticEcosystemExampleDescriptor {
        id: "deployment_to_ecosystem_handoff",
        kind: "handoff",
        path: "examples/p01/ecosystem/deployment_to_ecosystem_handoff.lyra",
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-semantic-packaging-check",
            "lyra-p01-semantic-deployment-check",
        ],
        proofs: &["deployment_bridge_proof", "offline_distribution_proof"],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        rejects: &["deployment_drift_accepted", "network_required"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_ECOSYSTEM_PROOFS: &[SemanticEcosystemProofDescriptor] = &[
    SemanticEcosystemProofDescriptor {
        id: "docs_coverage_proof",
        scope: "docs",
        docs: &[
            "semantic_symbol_operator_guide",
            "semantic_atom_developer_reference",
            "core_ir_ecosystem_walkthrough",
            "canonical_semantics_contributor_onboarding",
            "public_semantic_review_reference",
        ],
        examples: &[
            "canonical_symbol_walkthrough",
            "semantic_atom_extension_flow",
            "core_ir_receipt_review",
            "negative_semantic_doc_drift_rejection",
        ],
        receipts: &["receipts/p01/pass_0050_semantic_ecosystem.receipt"],
        commands: &["lyra-p01-semantic-ecosystem-check"],
        forbids: &["phase_closure", "global_complete"],
        status: "artifact_emitted",
    },
    SemanticEcosystemProofDescriptor {
        id: "executable_examples_proof",
        scope: "examples",
        docs: &[
            "semantic_symbol_operator_guide",
            "canonical_semantics_contributor_onboarding",
        ],
        examples: &[
            "canonical_symbol_walkthrough",
            "semantic_atom_extension_flow",
            "core_ir_receipt_review",
            "offline_operator_semantic_review",
        ],
        receipts: &["receipts/p01/pass_0050_semantic_ecosystem.receipt"],
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-atom-check",
            "lyra-p01-ir-check",
        ],
        forbids: &["phase_closure", "global_complete"],
        status: "artifact_emitted",
    },
    SemanticEcosystemProofDescriptor {
        id: "receipt_binding_proof",
        scope: "receipt",
        docs: &[
            "core_ir_ecosystem_walkthrough",
            "public_semantic_review_reference",
        ],
        examples: &[
            "core_ir_receipt_review",
            "negative_semantic_doc_drift_rejection",
        ],
        receipts: &[
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-semantic-replay-check",
        ],
        forbids: &["phase_closure", "global_complete"],
        status: "artifact_emitted",
    },
    SemanticEcosystemProofDescriptor {
        id: "offline_distribution_proof",
        scope: "distribution",
        docs: &[
            "offline_distribution_reference",
            "semantic_symbol_operator_guide",
        ],
        examples: &[
            "offline_operator_semantic_review",
            "deployment_to_ecosystem_handoff",
        ],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-semantic-packaging-check",
            "lyra-p01-semantic-deployment-check",
        ],
        forbids: &["phase_closure", "global_complete"],
        status: "artifact_emitted",
    },
    SemanticEcosystemProofDescriptor {
        id: "deployment_bridge_proof",
        scope: "deployment",
        docs: &[
            "offline_distribution_reference",
            "core_ir_ecosystem_walkthrough",
        ],
        examples: &[
            "offline_operator_semantic_review",
            "deployment_to_ecosystem_handoff",
        ],
        receipts: &[
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-ecosystem-check",
            "lyra-p01-semantic-deployment-check",
        ],
        forbids: &["phase_closure", "global_complete"],
        status: "artifact_emitted",
    },
    SemanticEcosystemProofDescriptor {
        id: "p01_phase_open",
        scope: "phase",
        docs: &[
            "public_semantic_review_reference",
            "canonical_semantics_contributor_onboarding",
        ],
        examples: &[
            "negative_semantic_doc_drift_rejection",
            "deployment_to_ecosystem_handoff",
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
        ],
        commands: &["lyra-p01-semantic-ecosystem-check"],
        forbids: &["phase_closure", "global_complete"],
        status: "blocked",
    },
];

pub fn semantic_ecosystem_doc_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ECOSYSTEM_DOCS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_ecosystem_example_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ECOSYSTEM_EXAMPLES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_ecosystem_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ECOSYSTEM_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_ecosystem_doc_descriptor(
    id: &str,
) -> Option<&'static SemanticEcosystemDocDescriptor> {
    LYRALANG_SEMANTIC_ECOSYSTEM_DOCS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_ecosystem_example_descriptor(
    id: &str,
) -> Option<&'static SemanticEcosystemExampleDescriptor> {
    LYRALANG_SEMANTIC_ECOSYSTEM_EXAMPLES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_ecosystem_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticEcosystemProofDescriptor> {
    LYRALANG_SEMANTIC_ECOSYSTEM_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_ecosystem_doc_signature(item: &SemanticEcosystemDocDescriptor) -> String {
    format!(
        "doc:{}|audience:{}|path:{}|covers:{}|examples:{}|receipts:{}|status:{}",
        item.id,
        item.audience,
        item.path,
        sorted_join(item.covers),
        sorted_join(item.examples),
        sorted_join(item.receipts),
        item.status
    )
}

pub fn semantic_ecosystem_example_signature(item: &SemanticEcosystemExampleDescriptor) -> String {
    format!(
        "example:{}|kind:{}|path:{}|commands:{}|proofs:{}|receipts:{}|rejects:{}|status:{}",
        item.id,
        item.kind,
        item.path,
        sorted_join(item.commands),
        sorted_join(item.proofs),
        sorted_join(item.receipts),
        sorted_join(item.rejects),
        item.status
    )
}

pub fn semantic_ecosystem_proof_signature(item: &SemanticEcosystemProofDescriptor) -> String {
    format!(
        "proof:{}|scope:{}|docs:{}|examples:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
        item.id,
        item.scope,
        sorted_join(item.docs),
        sorted_join(item.examples),
        sorted_join(item.receipts),
        sorted_join(item.commands),
        sorted_join(item.forbids),
        item.status
    )
}

pub fn semantic_ecosystem_doc_digest(item: &SemanticEcosystemDocDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_ecosystem.doc",
        &semantic_ecosystem_doc_signature(item),
    )
}
pub fn semantic_ecosystem_example_digest(item: &SemanticEcosystemExampleDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_ecosystem.example",
        &semantic_ecosystem_example_signature(item),
    )
}
pub fn semantic_ecosystem_proof_digest(item: &SemanticEcosystemProofDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_ecosystem.proof",
        &semantic_ecosystem_proof_signature(item),
    )
}

pub fn semantic_ecosystem_registry_signature() -> String {
    let mut rows = Vec::new();
    for doc in LYRALANG_SEMANTIC_ECOSYSTEM_DOCS {
        rows.push(format!(
            "doc:{}|{}",
            doc.id,
            semantic_ecosystem_doc_digest(doc)
        ));
    }
    for example in LYRALANG_SEMANTIC_ECOSYSTEM_EXAMPLES {
        rows.push(format!(
            "example:{}|{}",
            example.id,
            semantic_ecosystem_example_digest(example)
        ));
    }
    for proof in LYRALANG_SEMANTIC_ECOSYSTEM_PROOFS {
        rows.push(format!(
            "proof:{}|{}",
            proof.id,
            semantic_ecosystem_proof_digest(proof)
        ));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_ecosystem_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_ecosystem.registry",
        &semantic_ecosystem_registry_signature(),
    )
}

pub fn semantic_ecosystem_docs_bind_examples() -> bool {
    let example_ids = semantic_ecosystem_example_ids();
    LYRALANG_SEMANTIC_ECOSYSTEM_DOCS
        .iter()
        .all(|doc| doc.examples.iter().all(|id| example_ids.contains(id)))
}

pub fn semantic_ecosystem_examples_bind_proofs() -> bool {
    let proof_ids = semantic_ecosystem_proof_ids();
    LYRALANG_SEMANTIC_ECOSYSTEM_EXAMPLES
        .iter()
        .all(|example| example.proofs.iter().all(|id| proof_ids.contains(id)))
}

pub fn semantic_ecosystem_proofs_bind_registry() -> bool {
    let doc_ids = semantic_ecosystem_doc_ids();
    let example_ids = semantic_ecosystem_example_ids();
    LYRALANG_SEMANTIC_ECOSYSTEM_PROOFS.iter().all(|proof| {
        proof.docs.iter().all(|id| doc_ids.contains(id))
            && proof.examples.iter().all(|id| example_ids.contains(id))
    })
}

pub fn semantic_ecosystem_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_ECOSYSTEM_DOCS
        .iter()
        .all(|doc| doc.path.starts_with("docs/p01/") || doc.path.starts_with("products/p01/"))
        && LYRALANG_SEMANTIC_ECOSYSTEM_EXAMPLES.iter().all(|example| {
            example.path.starts_with("examples/p01/") || example.path.starts_with("fixtures/p01/")
        })
}

pub fn semantic_ecosystem_receipts_cover_p01_001_through_p01_021() -> bool {
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
    ];
    let mut receipts = Vec::new();
    for doc in LYRALANG_SEMANTIC_ECOSYSTEM_DOCS {
        receipts.extend_from_slice(doc.receipts);
    }
    for example in LYRALANG_SEMANTIC_ECOSYSTEM_EXAMPLES {
        receipts.extend_from_slice(example.receipts);
    }
    for proof in LYRALANG_SEMANTIC_ECOSYSTEM_PROOFS {
        receipts.extend_from_slice(proof.receipts);
    }
    required
        .iter()
        .all(|needle| receipts.iter().any(|receipt| receipt.contains(needle)))
}

pub fn semantic_ecosystem_no_forbidden_descriptor_claims() -> bool {
    let signature = semantic_ecosystem_registry_signature().to_ascii_lowercase();
    !signature.contains("network required")
        && !signature.contains("cloud required")
        && !signature.contains("online required")
        && !signature.contains("remote fetch")
        && !signature.contains("phase closed")
        && !signature.contains("global complete")
}

fn sorted_join(items: &[&'static str]) -> String {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy.join(",")
}
