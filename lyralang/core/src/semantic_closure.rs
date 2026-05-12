use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticClosureTaskDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub evidence: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticClosureOutputDescriptor {
    pub id: &'static str,
    pub output_kind: &'static str,
    pub path: &'static str,
    pub depends: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticClosureProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub tasks: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub permits: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P01_SEMANTIC_CLOSURE_CARRIER: &str = "lyra.p01.semantic_closure_gate.v1";

pub const LYRALANG_SEMANTIC_CLOSURE_TASKS: &[SemanticClosureTaskDescriptor] = &[
    SemanticClosureTaskDescriptor { id: "P01-001", scope: "semantic_atoms", receipts: &["receipts/p01/pass_0030_semantic_atoms.receipt"], commands: &["lyra-p01-atom-check", "lyra-p01-semantic-closure-check"], evidence: &["lyralang/core/src/semantic_atoms.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-002", scope: "core_ir", receipts: &["receipts/p01/pass_0031_core_ir.receipt"], commands: &["lyra-p01-ir-check", "lyra-p01-semantic-closure-check"], evidence: &["lyralang/core/src/core_ir.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-003", scope: "semantic_objects", receipts: &["receipts/p01/pass_0032_semantic_objects.receipt"], commands: &["lyra-p01-object-check", "lyra-p01-semantic-closure-check"], evidence: &["lyralang/core/src/semantic_objects.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-004", scope: "semantic_identity", receipts: &["receipts/p01/pass_0033_semantic_identity.receipt"], commands: &["lyra-p01-identity-check", "lyra-p01-semantic-closure-check"], evidence: &["lyralang/core/src/semantic_identity.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-005", scope: "reference_semantics", receipts: &["receipts/p01/pass_0034_reference_semantics.receipt"], commands: &["lyra-p01-reference-semantics-check", "lyra-p01-semantic-closure-check"], evidence: &["lyralang/core/src/reference_semantics.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-006", scope: "symbolic_equality", receipts: &["receipts/p01/pass_0035_symbolic_equality.receipt"], commands: &["lyra-p01-symbolic-equality-check", "lyra-p01-semantic-closure-check"], evidence: &["lyralang/core/src/symbolic_equality.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-007", scope: "error_challenge_evidence", receipts: &["receipts/p01/pass_0036_error_challenge_evidence.receipt"], commands: &["lyra-p01-error-challenge-evidence-check", "lyra-p01-semantic-closure-check"], evidence: &["lyralang/core/src/error_challenge_evidence.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-008", scope: "serialization_hashing", receipts: &["receipts/p01/pass_0037_semantic_serialization_hashing.receipt"], commands: &["lyra-p01-semantic-serialization-hashing-check", "lyra-p01-semantic-closure-check"], evidence: &["k0/determinism/src/semantic_serialization_hashing.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-009", scope: "adversarial_corpus", receipts: &["receipts/p01/pass_0038_semantic_adversarial_corpus.receipt"], commands: &["lyra-p01-semantic-adversarial-corpus-check", "lyra-p01-semantic-closure-check"], evidence: &["fixtures/p01/semantic_adversarial_corpus_inputs/valid_semantic_adversarial_corpus.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-010", scope: "core_ir_reuse", receipts: &["receipts/p01/pass_0039_core_ir_reuse.receipt"], commands: &["lyra-p01-core-ir-reuse-check", "lyra-p01-semantic-closure-check"], evidence: &["lyralang/core/src/core_ir_reuse.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-011", scope: "atom_reference", receipts: &["receipts/p01/pass_0040_semantic_atom_reference.receipt"], commands: &["lyra-p01-semantic-atom-reference-check", "lyra-p01-semantic-closure-check"], evidence: &["docs/p01/semantic_atom_reference_guide.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-012", scope: "bedrock_receipts", receipts: &["receipts/p01/pass_0041_semantic_bedrock_receipts.receipt"], commands: &["lyra-p01-semantic-bedrock-receipts-check", "lyra-p01-semantic-closure-check"], evidence: &["docs/p01/semantic_bedrock_receipts_guide.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-013", scope: "formal_semantic_constitution", receipts: &["receipts/p01/pass_0042_formal_semantic_constitution.receipt"], commands: &["lyra-p01-formal-semantic-constitution-check", "lyra-p01-semantic-closure-check"], evidence: &["ops/p01/control/formal_semantic_constitution_law.v1.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-014", scope: "canonical_data_model", receipts: &["receipts/p01/pass_0043_canonical_data_model.receipt"], commands: &["lyra-p01-canonical-data-model-check", "lyra-p01-semantic-closure-check"], evidence: &["interfaces/p01/contracts/canonical_data_model.v1.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-015", scope: "core_engine", receipts: &["receipts/p01/pass_0044_semantic_core_engine.receipt"], commands: &["lyra-p01-semantic-core-engine-check", "lyra-p01-semantic-closure-check"], evidence: &["k0/determinism/src/semantic_core_engine.rs"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-016", scope: "falsification", receipts: &["receipts/p01/pass_0045_semantic_falsification.receipt"], commands: &["lyra-p01-semantic-falsification-check", "lyra-p01-semantic-closure-check"], evidence: &["fixtures/p01/semantic_falsification_inputs/valid_semantic_falsification.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-017", scope: "replay", receipts: &["receipts/p01/pass_0046_semantic_replay.receipt"], commands: &["lyra-p01-semantic-replay-check", "lyra-p01-semantic-closure-check"], evidence: &["fixtures/p01/semantic_replay_inputs/valid_semantic_replay.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-018", scope: "interface", receipts: &["receipts/p01/pass_0047_semantic_interface.receipt"], commands: &["lyra-p01-semantic-interface-check", "lyra-p01-semantic-closure-check"], evidence: &["products/p01/semantic_interface_inspection_surface.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-019", scope: "packaging", receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"], commands: &["lyra-p01-semantic-packaging-check", "lyra-p01-semantic-closure-check"], evidence: &["products/p01/semantic_packaging_distribution_manifest.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-020", scope: "deployment", receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"], commands: &["lyra-p01-semantic-deployment-check", "lyra-p01-semantic-closure-check"], evidence: &["products/p01/semantic_deployment_manifest.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-021", scope: "ecosystem", receipts: &["receipts/p01/pass_0050_semantic_ecosystem.receipt"], commands: &["lyra-p01-semantic-ecosystem-check", "lyra-p01-semantic-closure-check"], evidence: &["products/p01/semantic_ecosystem_manifest.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-022", scope: "economics", receipts: &["receipts/p01/pass_0051_semantic_economics.receipt"], commands: &["lyra-p01-semantic-economics-check", "lyra-p01-semantic-closure-check"], evidence: &["products/p01/semantic_economics_manifest.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-023", scope: "redteam", receipts: &["receipts/p01/pass_0052_semantic_redteam.receipt"], commands: &["lyra-p01-semantic-redteam-check", "lyra-p01-semantic-closure-check"], evidence: &["products/p01/semantic_redteam_inspection_surface.lyra"], status: "bounded_closed" },
    SemanticClosureTaskDescriptor { id: "P01-024", scope: "closure", receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"], commands: &["lyra-p01-semantic-closure-check", "lyra-p01-semantic-closure-check"], evidence: &["fixtures/p01/semantic_closure_inputs/valid_semantic_closure.lyra"], status: "bounded_closed" },
];

pub const LYRALANG_SEMANTIC_CLOSURE_OUTPUTS: &[SemanticClosureOutputDescriptor] = &[
    SemanticClosureOutputDescriptor {
        id: "P01-X01",
        output_kind: "dependency_matrix",
        path: "ops/p01/control/p01_dependency_matrix.v1.lyra",
        depends: &[
            "P01-001", "P01-002", "P01-003", "P01-004", "P01-005", "P01-006", "P01-007", "P01-008",
            "P01-009", "P01-010", "P01-011", "P01-012", "P01-013", "P01-014", "P01-015", "P01-016",
            "P01-017", "P01-018", "P01-019", "P01-020", "P01-021", "P01-022", "P01-023", "P01-024",
        ],
        receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"],
        status: "blocked",
    },
    SemanticClosureOutputDescriptor {
        id: "P01-X02",
        output_kind: "proof_family_table",
        path: "ops/p01/control/p01_proof_family_table.v1.lyra",
        depends: &["P01-X01"],
        receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"],
        status: "blocked",
    },
    SemanticClosureOutputDescriptor {
        id: "P01-X03",
        output_kind: "benchmark_pack",
        path: "products/p01/p01_semantic_benchmark_pack.lyra",
        depends: &["P01-X01", "P01-X02"],
        receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"],
        status: "blocked",
    },
    SemanticClosureOutputDescriptor {
        id: "P01-X04",
        output_kind: "output_table",
        path: "products/p01/p01_semantic_output_table.lyra",
        depends: &["P01-X01", "P01-X02", "P01-X03"],
        receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"],
        status: "blocked",
    },
    SemanticClosureOutputDescriptor {
        id: "P01-X05",
        output_kind: "retirement_law",
        path: "ops/p01/control/p01_retirement_supersession_law.v1.lyra",
        depends: &["P01-X01", "P01-X02", "P01-X03", "P01-X04"],
        receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"],
        status: "blocked",
    },
];

pub const LYRALANG_SEMANTIC_CLOSURE_PROOFS: &[SemanticClosureProofDescriptor] = &[
    SemanticClosureProofDescriptor {
        id: "semantic_primary_task_receipt_chain",
        scope: "receipt_chain",
        tasks: &[
            "P01-001", "P01-002", "P01-003", "P01-004", "P01-005", "P01-006", "P01-007", "P01-008",
            "P01-009", "P01-010", "P01-011", "P01-012", "P01-013", "P01-014", "P01-015", "P01-016",
            "P01-017", "P01-018", "P01-019", "P01-020", "P01-021", "P01-022", "P01-023", "P01-024",
        ],
        outputs: &["P01-X01"],
        receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"],
        commands: &["lyra-p01-semantic-closure-check"],
        permits: &["bounded_primary_closure"],
        forbids: &["global_closure", "unreceipted_closure"],
        status: "artifact_emitted",
    },
    SemanticClosureProofDescriptor {
        id: "semantic_negative_corpus_receipt_chain",
        scope: "receipt_chain",
        tasks: &["P01-009", "P01-016", "P01-023", "P01-024"],
        outputs: &["P01-X02"],
        receipts: &[
            "receipts/p01/pass_0038_semantic_adversarial_corpus.receipt",
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0052_semantic_redteam.receipt",
            "receipts/p01/pass_0053_semantic_closure.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-closure-check",
            "lyra-p01-semantic-falsification-check",
        ],
        permits: &["bounded_primary_closure"],
        forbids: &["global_closure", "unreceipted_closure"],
        status: "artifact_emitted",
    },
    SemanticClosureProofDescriptor {
        id: "semantic_redteam_rollback_receipt_chain",
        scope: "receipt_chain",
        tasks: &["P01-023", "P01-024"],
        outputs: &["P01-X02", "P01-X05"],
        receipts: &[
            "receipts/p01/pass_0052_semantic_redteam.receipt",
            "receipts/p01/pass_0053_semantic_closure.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-closure-check",
            "lyra-p01-semantic-redteam-check",
        ],
        permits: &["bounded_primary_closure"],
        forbids: &["global_closure", "unreceipted_closure"],
        status: "artifact_emitted",
    },
    SemanticClosureProofDescriptor {
        id: "semantic_control_plane_transition_proof",
        scope: "control_plane",
        tasks: &["P01-024"],
        outputs: &["P01-X01", "P01-X02", "P01-X03", "P01-X04", "P01-X05"],
        receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"],
        commands: &["lyra-p01-semantic-closure-check"],
        permits: &["bounded_primary_closure"],
        forbids: &["global_closure", "unreceipted_closure"],
        status: "artifact_emitted",
    },
    SemanticClosureProofDescriptor {
        id: "semantic_bounded_vs_global_closure_proof",
        scope: "global_denial",
        tasks: &["P01-024"],
        outputs: &["P01-X01", "P01-X02", "P01-X03", "P01-X04", "P01-X05"],
        receipts: &["receipts/p01/pass_0053_semantic_closure.receipt"],
        commands: &["lyra-p01-semantic-closure-check"],
        permits: &["bounded_primary_closure"],
        forbids: &["global_closure", "unreceipted_closure"],
        status: "artifact_emitted",
    },
];

pub fn semantic_closure_task_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_CLOSURE_TASKS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_closure_output_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_CLOSURE_OUTPUTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_closure_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_CLOSURE_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_closure_task_descriptor(
    id: &str,
) -> Option<&'static SemanticClosureTaskDescriptor> {
    LYRALANG_SEMANTIC_CLOSURE_TASKS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_closure_output_descriptor(
    id: &str,
) -> Option<&'static SemanticClosureOutputDescriptor> {
    LYRALANG_SEMANTIC_CLOSURE_OUTPUTS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_closure_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticClosureProofDescriptor> {
    LYRALANG_SEMANTIC_CLOSURE_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_closure_task_signature(item: &SemanticClosureTaskDescriptor) -> String {
    format!(
        "task:{}|scope:{}|receipts:{}|commands:{}|evidence:{}|status:{}",
        item.id,
        item.scope,
        sorted_join(item.receipts),
        sorted_join(item.commands),
        sorted_join(item.evidence),
        item.status
    )
}

pub fn semantic_closure_output_signature(item: &SemanticClosureOutputDescriptor) -> String {
    format!(
        "output:{}|kind:{}|path:{}|depends:{}|receipts:{}|status:{}",
        item.id,
        item.output_kind,
        item.path,
        sorted_join(item.depends),
        sorted_join(item.receipts),
        item.status
    )
}

pub fn semantic_closure_proof_signature(item: &SemanticClosureProofDescriptor) -> String {
    format!("proof:{}|scope:{}|tasks:{}|outputs:{}|receipts:{}|commands:{}|permits:{}|forbids:{}|status:{}", item.id, item.scope, sorted_join(item.tasks), sorted_join(item.outputs), sorted_join(item.receipts), sorted_join(item.commands), sorted_join(item.permits), sorted_join(item.forbids), item.status)
}

pub fn semantic_closure_task_digest(item: &SemanticClosureTaskDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_closure.task",
        &semantic_closure_task_signature(item),
    )
}
pub fn semantic_closure_output_digest(item: &SemanticClosureOutputDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_closure.output",
        &semantic_closure_output_signature(item),
    )
}
pub fn semantic_closure_proof_digest(item: &SemanticClosureProofDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_closure.proof",
        &semantic_closure_proof_signature(item),
    )
}

pub fn semantic_closure_registry_signature() -> String {
    let mut rows = Vec::new();
    for task in LYRALANG_SEMANTIC_CLOSURE_TASKS {
        rows.push(format!(
            "task:{}|{}",
            task.id,
            semantic_closure_task_digest(task)
        ));
    }
    for output in LYRALANG_SEMANTIC_CLOSURE_OUTPUTS {
        rows.push(format!(
            "output:{}|{}",
            output.id,
            semantic_closure_output_digest(output)
        ));
    }
    for proof in LYRALANG_SEMANTIC_CLOSURE_PROOFS {
        rows.push(format!(
            "proof:{}|{}",
            proof.id,
            semantic_closure_proof_digest(proof)
        ));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_closure_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_closure.registry",
        &semantic_closure_registry_signature(),
    )
}

pub fn semantic_closure_outputs_bind_known_tasks() -> bool {
    let task_ids = semantic_closure_task_ids();
    let output_ids = semantic_closure_output_ids();
    LYRALANG_SEMANTIC_CLOSURE_OUTPUTS.iter().all(|output| {
        output
            .depends
            .iter()
            .all(|id| task_ids.contains(id) || output_ids.contains(id))
    })
}

pub fn semantic_closure_proofs_bind_registry() -> bool {
    let task_ids = semantic_closure_task_ids();
    let output_ids = semantic_closure_output_ids();
    LYRALANG_SEMANTIC_CLOSURE_PROOFS.iter().all(|proof| {
        proof.tasks.iter().all(|id| task_ids.contains(id))
            && proof.outputs.iter().all(|id| output_ids.contains(id))
    })
}

pub fn semantic_closure_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_CLOSURE_TASKS.iter().all(|task| {
        task.evidence.iter().all(|path| {
            path.starts_with("lyralang/")
                || path.starts_with("k0/")
                || path.starts_with("interfaces/")
                || path.starts_with("ops/")
                || path.starts_with("fixtures/")
                || path.starts_with("docs/")
                || path.starts_with("products/")
        })
    }) && LYRALANG_SEMANTIC_CLOSURE_OUTPUTS.iter().all(|output| {
        output.path.starts_with("ops/")
            || output.path.starts_with("docs/")
            || output.path.starts_with("products/")
            || output.path.starts_with("interfaces/")
    })
}

pub fn semantic_closure_receipts_cover_p01_001_through_p01_024() -> bool {
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
        "pass_0053_semantic_closure",
    ];
    let mut receipts = Vec::new();
    for task in LYRALANG_SEMANTIC_CLOSURE_TASKS {
        receipts.extend_from_slice(task.receipts);
    }
    for output in LYRALANG_SEMANTIC_CLOSURE_OUTPUTS {
        receipts.extend_from_slice(output.receipts);
    }
    for proof in LYRALANG_SEMANTIC_CLOSURE_PROOFS {
        receipts.extend_from_slice(proof.receipts);
    }
    required
        .iter()
        .all(|needle| receipts.iter().any(|receipt| receipt.contains(needle)))
}

pub fn semantic_closure_no_forbidden_descriptor_claims() -> bool {
    let signature = semantic_closure_registry_signature().to_ascii_lowercase();
    !signature.contains("network required")
        && !signature.contains("cloud required")
        && !signature.contains("online required")
        && !signature.contains("remote fetch")
        && !signature.contains("global complete")
        && !signature.contains("phase closed")
        && !signature.contains("closure without receipt")
}

fn sorted_join(items: &[&'static str]) -> String {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy.join(",")
}
