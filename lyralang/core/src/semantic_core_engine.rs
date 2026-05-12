use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticCoreEngineUnitDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub input_model: &'static str,
    pub output_model: &'static str,
    pub stage_order: &'static str,
    pub engine_law: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticCoreEngineTransitionDescriptor {
    pub id: &'static str,
    pub from_unit: &'static str,
    pub to_unit: &'static str,
    pub transition_law: &'static str,
    pub carry: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticCoreEngineArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub artifact_kind: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticCoreEngineProofDescriptor {
    pub id: &'static str,
    pub units: &'static [&'static str],
    pub transitions: &'static [&'static str],
    pub artifacts: &'static [&'static str],
    pub fixture: &'static str,
    pub golden: &'static str,
    pub receipt: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticCoreEngineError {
    UnknownUnit,
    UnknownTransition,
    UnknownArtifact,
    UnknownProof,
}

pub const LYRA_P01_SEMANTIC_CORE_ENGINE_CARRIER: &str = "lyra_p01_semantic_core_engine";

pub const LYRALANG_SEMANTIC_CORE_ENGINE_UNITS: &[SemanticCoreEngineUnitDescriptor] = &[
    SemanticCoreEngineUnitDescriptor {
        id: "canonical_symbol_ingest_engine",
        owner_root: "lyralang",
        input_model: "operator_surface_bytes",
        output_model: "canonical_symbol_model",
        stage_order: "001",
        engine_law: "canonical_lines_then_symbol_identity",
        status: "artifact_emitted",
    },
    SemanticCoreEngineUnitDescriptor {
        id: "semantic_atom_binding_engine",
        owner_root: "lyralang",
        input_model: "canonical_symbol_model",
        output_model: "semantic_atom_model",
        stage_order: "002",
        engine_law: "closed_atom_registry_binding",
        status: "artifact_emitted",
    },
    SemanticCoreEngineUnitDescriptor {
        id: "core_ir_lowering_engine",
        owner_root: "lyralang",
        input_model: "semantic_atom_model",
        output_model: "core_ir_term_model",
        stage_order: "003",
        engine_law: "canonical_atom_to_core_ir_term_lowering",
        status: "artifact_emitted",
    },
    SemanticCoreEngineUnitDescriptor {
        id: "core_ir_form_engine",
        owner_root: "interfaces",
        input_model: "core_ir_term_model",
        output_model: "core_ir_form_model",
        stage_order: "004",
        engine_law: "text_binary_ir_form_parity",
        status: "artifact_emitted",
    },
    SemanticCoreEngineUnitDescriptor {
        id: "semantic_object_emission_engine",
        owner_root: "lyralang",
        input_model: "core_ir_form_model",
        output_model: "semantic_object_model",
        stage_order: "005",
        engine_law: "semantic_object_from_core_ir_digest",
        status: "artifact_emitted",
    },
    SemanticCoreEngineUnitDescriptor {
        id: "semantic_identity_digest_engine",
        owner_root: "k0",
        input_model: "semantic_object_model",
        output_model: "semantic_identity_model",
        stage_order: "006",
        engine_law: "fnv1a128_identity_digest_commit",
        status: "execution_proven",
    },
    SemanticCoreEngineUnitDescriptor {
        id: "symbolic_equality_normal_form_engine",
        owner_root: "lyralang",
        input_model: "semantic_identity_model",
        output_model: "symbolic_equality_model",
        stage_order: "007",
        engine_law: "total_symbolic_normal_form_rewrite",
        status: "execution_proven",
    },
    SemanticCoreEngineUnitDescriptor {
        id: "semantic_receipt_commit_engine",
        owner_root: "k0",
        input_model: "symbolic_equality_model",
        output_model: "semantic_receipt_model",
        stage_order: "008",
        engine_law: "phase_receipt_commit_verdict_parity",
        status: "execution_proven",
    },
    SemanticCoreEngineUnitDescriptor {
        id: "semantic_replay_witness_engine",
        owner_root: "ops",
        input_model: "semantic_receipt_model",
        output_model: "p01_replay_witness",
        stage_order: "009",
        engine_law: "receipt_bound_replay_witness_projection",
        status: "working_slice",
    },
];

pub const LYRALANG_SEMANTIC_CORE_ENGINE_TRANSITIONS: &[SemanticCoreEngineTransitionDescriptor] = &[
    SemanticCoreEngineTransitionDescriptor {
        id: "ingest_to_atom_binding",
        from_unit: "canonical_symbol_ingest_engine",
        to_unit: "semantic_atom_binding_engine",
        transition_law: "canonical_symbol_to_closed_atom",
        carry: "single_carrier_state",
        status: "artifact_emitted",
    },
    SemanticCoreEngineTransitionDescriptor {
        id: "atom_binding_to_ir_lowering",
        from_unit: "semantic_atom_binding_engine",
        to_unit: "core_ir_lowering_engine",
        transition_law: "closed_atom_to_ir_term",
        carry: "single_carrier_state",
        status: "artifact_emitted",
    },
    SemanticCoreEngineTransitionDescriptor {
        id: "ir_lowering_to_ir_form",
        from_unit: "core_ir_lowering_engine",
        to_unit: "core_ir_form_engine",
        transition_law: "term_to_form_text_binary_parity",
        carry: "single_carrier_state",
        status: "artifact_emitted",
    },
    SemanticCoreEngineTransitionDescriptor {
        id: "ir_form_to_object_emission",
        from_unit: "core_ir_form_engine",
        to_unit: "semantic_object_emission_engine",
        transition_law: "ir_form_to_object_digest",
        carry: "single_carrier_state",
        status: "artifact_emitted",
    },
    SemanticCoreEngineTransitionDescriptor {
        id: "object_emission_to_identity_digest",
        from_unit: "semantic_object_emission_engine",
        to_unit: "semantic_identity_digest_engine",
        transition_law: "object_digest_to_identity_digest",
        carry: "single_carrier_state",
        status: "execution_proven",
    },
    SemanticCoreEngineTransitionDescriptor {
        id: "identity_digest_to_equality_normal_form",
        from_unit: "semantic_identity_digest_engine",
        to_unit: "symbolic_equality_normal_form_engine",
        transition_law: "identity_to_equality_normal_form",
        carry: "single_carrier_state",
        status: "execution_proven",
    },
    SemanticCoreEngineTransitionDescriptor {
        id: "equality_normal_form_to_receipt_commit",
        from_unit: "symbolic_equality_normal_form_engine",
        to_unit: "semantic_receipt_commit_engine",
        transition_law: "normal_form_to_receipt_verdict",
        carry: "single_carrier_state",
        status: "execution_proven",
    },
    SemanticCoreEngineTransitionDescriptor {
        id: "receipt_commit_to_replay_witness",
        from_unit: "semantic_receipt_commit_engine",
        to_unit: "semantic_replay_witness_engine",
        transition_law: "receipt_to_replay_projection",
        carry: "single_carrier_state",
        status: "working_slice",
    },
];

pub const LYRALANG_SEMANTIC_CORE_ENGINE_ARTIFACTS: &[SemanticCoreEngineArtifactDescriptor] = &[
    SemanticCoreEngineArtifactDescriptor {
        id: "engine_contract",
        owner_root: "interfaces",
        path: "interfaces/p01/contracts/semantic_core_engine.v1.lyra",
        artifact_kind: "contract",
        status: "artifact_emitted",
    },
    SemanticCoreEngineArtifactDescriptor {
        id: "engine_law",
        owner_root: "ops",
        path: "ops/p01/control/semantic_core_engine_law.v1.lyra",
        artifact_kind: "law",
        status: "artifact_emitted",
    },
    SemanticCoreEngineArtifactDescriptor {
        id: "engine_operator",
        owner_root: "src",
        path: "src/bin/lyra-p01-semantic-core-engine-check.rs",
        artifact_kind: "binary",
        status: "artifact_emitted",
    },
    SemanticCoreEngineArtifactDescriptor {
        id: "valid_engine_fixture",
        owner_root: "fixtures",
        path: "fixtures/p01/semantic_core_engine_inputs/valid_semantic_core_engine.lyra",
        artifact_kind: "fixture",
        status: "artifact_emitted",
    },
    SemanticCoreEngineArtifactDescriptor {
        id: "golden_engine_receipt",
        owner_root: "goldens",
        path: "goldens/p01/valid_semantic_core_engine.receipt",
        artifact_kind: "golden",
        status: "artifact_emitted",
    },
    SemanticCoreEngineArtifactDescriptor {
        id: "execution_engine_receipt",
        owner_root: "receipts",
        path: "receipts/p01/pass_0044_semantic_core_engine.receipt",
        artifact_kind: "receipt",
        status: "artifact_emitted",
    },
    SemanticCoreEngineArtifactDescriptor {
        id: "inspection_surface",
        owner_root: "products",
        path: "products/p01/semantic_core_engine_inspection_surface.lyra",
        artifact_kind: "inspection",
        status: "artifact_emitted",
    },
    SemanticCoreEngineArtifactDescriptor {
        id: "deterministic_suite_report",
        owner_root: "k0",
        path: "k0/determinism/src/semantic_core_engine.rs",
        artifact_kind: "report",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_CORE_ENGINE_PROOFS: &[SemanticCoreEngineProofDescriptor] = &[
    SemanticCoreEngineProofDescriptor {
        id: "unit_order_proof",
        units: &[
            "canonical_symbol_ingest_engine",
            "semantic_atom_binding_engine",
            "core_ir_lowering_engine",
            "core_ir_form_engine",
            "semantic_object_emission_engine",
            "semantic_identity_digest_engine",
            "symbolic_equality_normal_form_engine",
            "semantic_receipt_commit_engine",
            "semantic_replay_witness_engine",
        ],
        transitions: &["ingest_to_atom_binding", "atom_binding_to_ir_lowering"],
        artifacts: &["engine_contract", "engine_law"],
        fixture: "fixtures/p01/semantic_core_engine_cases/unit_order_case.lyra",
        golden: "goldens/p01/valid_semantic_core_engine.receipt",
        receipt: "receipts/p01/pass_0044_semantic_core_engine.receipt",
        status: "artifact_emitted",
    },
    SemanticCoreEngineProofDescriptor {
        id: "model_binding_proof",
        units: &[
            "canonical_symbol_ingest_engine",
            "semantic_atom_binding_engine",
            "core_ir_lowering_engine",
            "core_ir_form_engine",
        ],
        transitions: &["atom_binding_to_ir_lowering", "ir_lowering_to_ir_form"],
        artifacts: &["valid_engine_fixture", "inspection_surface"],
        fixture: "fixtures/p01/semantic_core_engine_cases/model_binding_case.lyra",
        golden: "goldens/p01/valid_semantic_core_engine.receipt",
        receipt: "receipts/p01/pass_0044_semantic_core_engine.receipt",
        status: "artifact_emitted",
    },
    SemanticCoreEngineProofDescriptor {
        id: "transition_totality_proof",
        units: &[
            "canonical_symbol_ingest_engine",
            "semantic_atom_binding_engine",
            "core_ir_lowering_engine",
            "core_ir_form_engine",
            "semantic_object_emission_engine",
            "semantic_identity_digest_engine",
            "symbolic_equality_normal_form_engine",
            "semantic_receipt_commit_engine",
            "semantic_replay_witness_engine",
        ],
        transitions: &[
            "ingest_to_atom_binding",
            "atom_binding_to_ir_lowering",
            "ir_lowering_to_ir_form",
            "ir_form_to_object_emission",
            "object_emission_to_identity_digest",
            "identity_digest_to_equality_normal_form",
            "equality_normal_form_to_receipt_commit",
            "receipt_commit_to_replay_witness",
        ],
        artifacts: &["deterministic_suite_report"],
        fixture: "fixtures/p01/semantic_core_engine_cases/transition_totality_case.lyra",
        golden: "goldens/p01/valid_semantic_core_engine.receipt",
        receipt: "receipts/p01/pass_0044_semantic_core_engine.receipt",
        status: "execution_proven",
    },
    SemanticCoreEngineProofDescriptor {
        id: "artifact_binding_proof",
        units: &[
            "semantic_object_emission_engine",
            "semantic_identity_digest_engine",
        ],
        transitions: &[
            "ir_form_to_object_emission",
            "object_emission_to_identity_digest",
        ],
        artifacts: &[
            "engine_contract",
            "engine_law",
            "engine_operator",
            "valid_engine_fixture",
            "golden_engine_receipt",
            "execution_engine_receipt",
            "inspection_surface",
            "deterministic_suite_report",
        ],
        fixture: "fixtures/p01/semantic_core_engine_cases/artifact_binding_case.lyra",
        golden: "goldens/p01/valid_semantic_core_engine.receipt",
        receipt: "receipts/p01/pass_0044_semantic_core_engine.receipt",
        status: "artifact_emitted",
    },
    SemanticCoreEngineProofDescriptor {
        id: "receipt_commit_proof",
        units: &[
            "symbolic_equality_normal_form_engine",
            "semantic_receipt_commit_engine",
            "semantic_replay_witness_engine",
        ],
        transitions: &[
            "equality_normal_form_to_receipt_commit",
            "receipt_commit_to_replay_witness",
        ],
        artifacts: &["golden_engine_receipt", "execution_engine_receipt"],
        fixture: "fixtures/p01/semantic_core_engine_cases/receipt_commit_case.lyra",
        golden: "goldens/p01/valid_semantic_core_engine.receipt",
        receipt: "receipts/p01/pass_0044_semantic_core_engine.receipt",
        status: "execution_proven",
    },
    SemanticCoreEngineProofDescriptor {
        id: "p01_semantic_core_engine_parity_proof",
        units: &[
            "canonical_symbol_ingest_engine",
            "semantic_atom_binding_engine",
            "core_ir_lowering_engine",
            "core_ir_form_engine",
            "semantic_object_emission_engine",
            "semantic_identity_digest_engine",
            "symbolic_equality_normal_form_engine",
            "semantic_receipt_commit_engine",
            "semantic_replay_witness_engine",
        ],
        transitions: &[
            "ingest_to_atom_binding",
            "atom_binding_to_ir_lowering",
            "ir_lowering_to_ir_form",
            "ir_form_to_object_emission",
            "object_emission_to_identity_digest",
            "identity_digest_to_equality_normal_form",
            "equality_normal_form_to_receipt_commit",
            "receipt_commit_to_replay_witness",
        ],
        artifacts: &[
            "engine_contract",
            "engine_law",
            "engine_operator",
            "valid_engine_fixture",
            "golden_engine_receipt",
            "execution_engine_receipt",
            "inspection_surface",
            "deterministic_suite_report",
        ],
        fixture: "fixtures/p01/semantic_core_engine_cases/full_engine_parity_case.lyra",
        golden: "goldens/p01/valid_semantic_core_engine.receipt",
        receipt: "receipts/p01/pass_0044_semantic_core_engine.receipt",
        status: "execution_proven",
    },
];

pub fn semantic_core_engine_unit_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_CORE_ENGINE_UNITS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_core_engine_transition_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_CORE_ENGINE_TRANSITIONS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_core_engine_artifact_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_CORE_ENGINE_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_core_engine_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_CORE_ENGINE_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_core_engine_unit_descriptor(
    id: &str,
) -> Option<&'static SemanticCoreEngineUnitDescriptor> {
    LYRALANG_SEMANTIC_CORE_ENGINE_UNITS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_core_engine_transition_descriptor(
    id: &str,
) -> Option<&'static SemanticCoreEngineTransitionDescriptor> {
    LYRALANG_SEMANTIC_CORE_ENGINE_TRANSITIONS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_core_engine_artifact_descriptor(
    id: &str,
) -> Option<&'static SemanticCoreEngineArtifactDescriptor> {
    LYRALANG_SEMANTIC_CORE_ENGINE_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_core_engine_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticCoreEngineProofDescriptor> {
    LYRALANG_SEMANTIC_CORE_ENGINE_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_core_engine_unit_signature(item: &SemanticCoreEngineUnitDescriptor) -> String {
    format!(
        "unit:{}|owner:{}|input:{}|output:{}|order:{}|law:{}|status:{}",
        item.id,
        item.owner_root,
        item.input_model,
        item.output_model,
        item.stage_order,
        item.engine_law,
        item.status
    )
}

pub fn semantic_core_engine_transition_signature(
    item: &SemanticCoreEngineTransitionDescriptor,
) -> String {
    format!(
        "transition:{}|from:{}|to:{}|law:{}|carry:{}|status:{}",
        item.id, item.from_unit, item.to_unit, item.transition_law, item.carry, item.status
    )
}

pub fn semantic_core_engine_artifact_signature(
    item: &SemanticCoreEngineArtifactDescriptor,
) -> String {
    format!(
        "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
        item.id, item.owner_root, item.path, item.artifact_kind, item.status
    )
}

pub fn semantic_core_engine_proof_signature(item: &SemanticCoreEngineProofDescriptor) -> String {
    format!(
        "proof:{}|units:{}|transitions:{}|artifacts:{}|fixture:{}|golden:{}|receipt:{}|status:{}",
        item.id,
        item.units.join(","),
        item.transitions.join(","),
        item.artifacts.join(","),
        item.fixture,
        item.golden,
        item.receipt,
        item.status
    )
}

pub fn semantic_core_engine_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SEMANTIC_CORE_ENGINE_UNITS {
        rows.push(semantic_core_engine_unit_signature(item));
    }
    for item in LYRALANG_SEMANTIC_CORE_ENGINE_TRANSITIONS {
        rows.push(semantic_core_engine_transition_signature(item));
    }
    for item in LYRALANG_SEMANTIC_CORE_ENGINE_ARTIFACTS {
        rows.push(semantic_core_engine_artifact_signature(item));
    }
    for item in LYRALANG_SEMANTIC_CORE_ENGINE_PROOFS {
        rows.push(semantic_core_engine_proof_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_core_engine_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_core_engine.registry",
        &semantic_core_engine_registry_signature(),
    )
}
pub fn semantic_core_engine_unit_digest(id: &str) -> Option<String> {
    semantic_core_engine_unit_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_core_engine.unit",
            &semantic_core_engine_unit_signature(item),
        )
    })
}
pub fn semantic_core_engine_transition_digest(id: &str) -> Option<String> {
    semantic_core_engine_transition_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_core_engine.transition",
            &semantic_core_engine_transition_signature(item),
        )
    })
}
pub fn semantic_core_engine_artifact_digest(id: &str) -> Option<String> {
    semantic_core_engine_artifact_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_core_engine.artifact",
            &semantic_core_engine_artifact_signature(item),
        )
    })
}
pub fn semantic_core_engine_proof_digest(id: &str) -> Option<String> {
    semantic_core_engine_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_core_engine.proof",
            &semantic_core_engine_proof_signature(item),
        )
    })
}

pub fn semantic_core_engine_units_have_stable_order() -> bool {
    let mut seen = std::collections::BTreeSet::new();
    LYRALANG_SEMANTIC_CORE_ENGINE_UNITS.iter().all(|unit| {
        unit.stage_order.len() == 3
            && unit
                .stage_order
                .as_bytes()
                .iter()
                .all(|byte| byte.is_ascii_digit())
            && seen.insert(unit.stage_order)
    })
}

pub fn semantic_core_engine_transitions_bind_known_units() -> bool {
    LYRALANG_SEMANTIC_CORE_ENGINE_TRANSITIONS
        .iter()
        .all(|transition| {
            semantic_core_engine_unit_descriptor(transition.from_unit).is_some()
                && semantic_core_engine_unit_descriptor(transition.to_unit).is_some()
                && transition.carry == "single_carrier_state"
        })
}

pub fn semantic_core_engine_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_CORE_ENGINE_ARTIFACTS
        .iter()
        .all(|artifact| {
            !artifact.path.is_empty()
                && !artifact.path.contains("..")
                && ["lyra", "rs", "receipt"]
                    .iter()
                    .any(|suffix| artifact.path.ends_with(suffix))
        })
}

pub fn semantic_core_engine_proofs_bind_registry() -> bool {
    LYRALANG_SEMANTIC_CORE_ENGINE_PROOFS.iter().all(|proof| {
        proof
            .units
            .iter()
            .all(|id| semantic_core_engine_unit_descriptor(id).is_some())
            && proof
                .transitions
                .iter()
                .all(|id| semantic_core_engine_transition_descriptor(id).is_some())
            && proof
                .artifacts
                .iter()
                .all(|id| semantic_core_engine_artifact_descriptor(id).is_some())
            && proof.fixture.ends_with(".lyra")
            && proof.golden.ends_with(".receipt")
            && proof.receipt.ends_with(".receipt")
    })
}

pub fn semantic_core_engine_no_forbidden_descriptor_claims() -> bool {
    let lower = semantic_core_engine_registry_signature().to_ascii_lowercase();
    !(lower.contains("probabilistic")
        || lower.contains("stochastic")
        || lower.contains("hidden randomness")
        || lower.contains("network required")
        || lower.contains("placeholder")
        || lower.contains("todo")
        || lower.contains("phase closed"))
}
