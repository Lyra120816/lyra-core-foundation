use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCoreEngineUnitDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub input_model: &'static str,
    pub output_model: &'static str,
    pub stage_order: &'static str,
    pub engine_law: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCoreEngineTransitionDescriptor {
    pub id: &'static str,
    pub from_unit: &'static str,
    pub to_unit: &'static str,
    pub transition_law: &'static str,
    pub carry: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCoreEngineArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub artifact_kind: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCoreEngineProofDescriptor {
    pub id: &'static str,
    pub units: &'static [&'static str],
    pub transitions: &'static [&'static str],
    pub artifacts: &'static [&'static str],
    pub fixture: &'static str,
    pub golden: &'static str,
    pub receipt: &'static str,
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_CORE_ENGINE_CARRIER: &str =
    "lyra.p02.bootstrap_core_engine.carrier.v1";

pub const LYRALANG_BOOTSTRAP_CORE_ENGINE_UNITS: &[BootstrapCoreEngineUnitDescriptor] = &[
    BootstrapCoreEngineUnitDescriptor {
        id: "bootstrap_authority_ingest_engine",
        owner_root: "k0",
        input_model: "operator_surface_bytes",
        output_model: "bootstrap_trust_model",
        stage_order: "001",
        engine_law: "canonical_bootstrap_authority_ingest",
        status: "execution_proven",
    },
    BootstrapCoreEngineUnitDescriptor {
        id: "seed_runtime_law_binding_engine",
        owner_root: "lyralang",
        input_model: "bootstrap_trust_model",
        output_model: "seed_runtime_law_model",
        stage_order: "002",
        engine_law: "seed_runtime_law_model_binding",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineUnitDescriptor {
        id: "host_surface_inventory_engine",
        owner_root: "ops",
        input_model: "seed_runtime_law_model",
        output_model: "host_extinction_model",
        stage_order: "003",
        engine_law: "host_surface_inventory_to_extinction_state",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineUnitDescriptor {
        id: "foreign_boundary_projection_engine",
        owner_root: "interfaces",
        input_model: "host_extinction_model",
        output_model: "foreign_surface_boundary_model",
        stage_order: "004",
        engine_law: "visible_bounded_challengeable_surface_projection",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineUnitDescriptor {
        id: "operator_handoff_capture_engine",
        owner_root: "ops",
        input_model: "foreign_surface_boundary_model",
        output_model: "operator_handoff_model",
        stage_order: "005",
        engine_law: "offline_operator_capture_truth_neutrality",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineUnitDescriptor {
        id: "emergency_fallback_freeze_engine",
        owner_root: "k0",
        input_model: "operator_handoff_model",
        output_model: "emergency_fallback_model",
        stage_order: "006",
        engine_law: "deterministic_fallback_freeze_and_recovery_gate",
        status: "execution_proven",
    },
    BootstrapCoreEngineUnitDescriptor {
        id: "bootstrap_receipt_commit_engine",
        owner_root: "k0",
        input_model: "emergency_fallback_model",
        output_model: "bootstrap_engine_receipt_model",
        stage_order: "007",
        engine_law: "canonical_engine_receipt_commit",
        status: "working_slice",
    },
];

pub const LYRALANG_BOOTSTRAP_CORE_ENGINE_TRANSITIONS: &[BootstrapCoreEngineTransitionDescriptor] =
    &[
        BootstrapCoreEngineTransitionDescriptor {
            id: "authority_ingest_to_seed_law",
            from_unit: "bootstrap_authority_ingest_engine",
            to_unit: "seed_runtime_law_binding_engine",
            transition_law: "trust_model_to_seed_runtime_law",
            carry: "single_carrier_state",
            status: "artifact_emitted",
        },
        BootstrapCoreEngineTransitionDescriptor {
            id: "seed_law_to_host_inventory",
            from_unit: "seed_runtime_law_binding_engine",
            to_unit: "host_surface_inventory_engine",
            transition_law: "seed_runtime_to_host_extinction_model",
            carry: "single_carrier_state",
            status: "artifact_emitted",
        },
        BootstrapCoreEngineTransitionDescriptor {
            id: "host_inventory_to_boundary_projection",
            from_unit: "host_surface_inventory_engine",
            to_unit: "foreign_boundary_projection_engine",
            transition_law: "host_extinction_to_foreign_boundary",
            carry: "single_carrier_state",
            status: "artifact_emitted",
        },
        BootstrapCoreEngineTransitionDescriptor {
            id: "boundary_projection_to_handoff_capture",
            from_unit: "foreign_boundary_projection_engine",
            to_unit: "operator_handoff_capture_engine",
            transition_law: "foreign_boundary_to_operator_handoff",
            carry: "single_carrier_state",
            status: "artifact_emitted",
        },
        BootstrapCoreEngineTransitionDescriptor {
            id: "handoff_capture_to_fallback_freeze",
            from_unit: "operator_handoff_capture_engine",
            to_unit: "emergency_fallback_freeze_engine",
            transition_law: "handoff_to_emergency_fallback_freeze",
            carry: "single_carrier_state",
            status: "artifact_emitted",
        },
        BootstrapCoreEngineTransitionDescriptor {
            id: "fallback_freeze_to_receipt_commit",
            from_unit: "emergency_fallback_freeze_engine",
            to_unit: "bootstrap_receipt_commit_engine",
            transition_law: "fallback_to_bootstrap_engine_receipt",
            carry: "single_carrier_state",
            status: "working_slice",
        },
    ];

pub const LYRALANG_BOOTSTRAP_CORE_ENGINE_ARTIFACTS: &[BootstrapCoreEngineArtifactDescriptor] = &[
    BootstrapCoreEngineArtifactDescriptor {
        id: "engine_contract",
        owner_root: "interfaces",
        path: "interfaces/p02/contracts/bootstrap_core_engine.v1.lyra",
        artifact_kind: "contract",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineArtifactDescriptor {
        id: "engine_law",
        owner_root: "ops",
        path: "ops/p02/core_engine/bootstrap_core_engine_manifest.v1.lyra",
        artifact_kind: "law_manifest",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineArtifactDescriptor {
        id: "engine_operator",
        owner_root: "shells",
        path: "shells/p02/bootstrap_core_engine_operator_surface.lyra",
        artifact_kind: "operator_surface",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineArtifactDescriptor {
        id: "valid_engine_fixture",
        owner_root: "fixtures",
        path: "fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra",
        artifact_kind: "valid_fixture",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineArtifactDescriptor {
        id: "golden_engine_receipt",
        owner_root: "goldens",
        path: "goldens/p02/valid_bootstrap_core_engine.receipt",
        artifact_kind: "golden_receipt",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineArtifactDescriptor {
        id: "execution_engine_receipt",
        owner_root: "receipts",
        path: "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
        artifact_kind: "execution_receipt",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineArtifactDescriptor {
        id: "inspection_surface",
        owner_root: "products",
        path: "products/p02/bootstrap_core_engine_inspection_surface.lyra",
        artifact_kind: "inspection_surface",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineArtifactDescriptor {
        id: "deterministic_suite_report",
        owner_root: "receipts",
        path: "receipts/p02/bootstrap_core_engine/bootstrap_core_engine_suite.report",
        artifact_kind: "suite_report",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_CORE_ENGINE_PROOFS: &[BootstrapCoreEngineProofDescriptor] = &[
    BootstrapCoreEngineProofDescriptor {
        id: "unit_order_proof",
        units: &[
            "bootstrap_authority_ingest_engine",
            "seed_runtime_law_binding_engine",
            "host_surface_inventory_engine",
            "foreign_boundary_projection_engine",
            "operator_handoff_capture_engine",
            "emergency_fallback_freeze_engine",
            "bootstrap_receipt_commit_engine",
        ],
        transitions: &[
            "authority_ingest_to_seed_law",
            "seed_law_to_host_inventory",
            "host_inventory_to_boundary_projection",
            "boundary_projection_to_handoff_capture",
            "handoff_capture_to_fallback_freeze",
            "fallback_freeze_to_receipt_commit",
        ],
        artifacts: &["engine_contract", "engine_law"],
        fixture: "fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra",
        golden: "goldens/p02/valid_bootstrap_core_engine.receipt",
        receipt: "receipts/p02/bootstrap_core_engine/unit_order.receipt",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineProofDescriptor {
        id: "model_binding_proof",
        units: &[
            "bootstrap_authority_ingest_engine",
            "seed_runtime_law_binding_engine",
            "host_surface_inventory_engine",
            "foreign_boundary_projection_engine",
            "operator_handoff_capture_engine",
            "emergency_fallback_freeze_engine",
        ],
        transitions: &[
            "authority_ingest_to_seed_law",
            "seed_law_to_host_inventory",
            "host_inventory_to_boundary_projection",
            "boundary_projection_to_handoff_capture",
            "handoff_capture_to_fallback_freeze",
        ],
        artifacts: &["engine_contract", "deterministic_suite_report"],
        fixture: "fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra",
        golden: "goldens/p02/valid_bootstrap_core_engine.receipt",
        receipt: "receipts/p02/bootstrap_core_engine/model_binding.receipt",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineProofDescriptor {
        id: "transition_totality_proof",
        units: &[
            "bootstrap_authority_ingest_engine",
            "seed_runtime_law_binding_engine",
            "host_surface_inventory_engine",
            "foreign_boundary_projection_engine",
            "operator_handoff_capture_engine",
            "emergency_fallback_freeze_engine",
            "bootstrap_receipt_commit_engine",
        ],
        transitions: &[
            "authority_ingest_to_seed_law",
            "seed_law_to_host_inventory",
            "host_inventory_to_boundary_projection",
            "boundary_projection_to_handoff_capture",
            "handoff_capture_to_fallback_freeze",
            "fallback_freeze_to_receipt_commit",
        ],
        artifacts: &["engine_law", "deterministic_suite_report"],
        fixture: "fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra",
        golden: "goldens/p02/valid_bootstrap_core_engine.receipt",
        receipt: "receipts/p02/bootstrap_core_engine/transition_totality.receipt",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineProofDescriptor {
        id: "artifact_binding_proof",
        units: &[
            "bootstrap_authority_ingest_engine",
            "seed_runtime_law_binding_engine",
            "host_surface_inventory_engine",
            "foreign_boundary_projection_engine",
            "operator_handoff_capture_engine",
            "emergency_fallback_freeze_engine",
            "bootstrap_receipt_commit_engine",
        ],
        transitions: &[
            "authority_ingest_to_seed_law",
            "seed_law_to_host_inventory",
            "host_inventory_to_boundary_projection",
            "boundary_projection_to_handoff_capture",
            "handoff_capture_to_fallback_freeze",
            "fallback_freeze_to_receipt_commit",
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
        fixture: "fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra",
        golden: "goldens/p02/valid_bootstrap_core_engine.receipt",
        receipt: "receipts/p02/bootstrap_core_engine/artifact_binding.receipt",
        status: "artifact_emitted",
    },
    BootstrapCoreEngineProofDescriptor {
        id: "receipt_commit_proof",
        units: &["bootstrap_receipt_commit_engine"],
        transitions: &["fallback_freeze_to_receipt_commit"],
        artifacts: &["golden_engine_receipt", "execution_engine_receipt"],
        fixture: "fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra",
        golden: "goldens/p02/valid_bootstrap_core_engine.receipt",
        receipt: "receipts/p02/bootstrap_core_engine/receipt_commit.receipt",
        status: "working_slice",
    },
    BootstrapCoreEngineProofDescriptor {
        id: "p02_bootstrap_core_engine_parity_proof",
        units: &[
            "bootstrap_authority_ingest_engine",
            "seed_runtime_law_binding_engine",
            "host_surface_inventory_engine",
            "foreign_boundary_projection_engine",
            "operator_handoff_capture_engine",
            "emergency_fallback_freeze_engine",
            "bootstrap_receipt_commit_engine",
        ],
        transitions: &[
            "authority_ingest_to_seed_law",
            "seed_law_to_host_inventory",
            "host_inventory_to_boundary_projection",
            "boundary_projection_to_handoff_capture",
            "handoff_capture_to_fallback_freeze",
            "fallback_freeze_to_receipt_commit",
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
        fixture: "fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra",
        golden: "goldens/p02/valid_bootstrap_core_engine.receipt",
        receipt: "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
        status: "working_slice",
    },
];

pub fn bootstrap_core_engine_unit_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_UNITS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_core_engine_transition_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_TRANSITIONS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_core_engine_artifact_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_core_engine_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_core_engine_unit_descriptor(
    id: &str,
) -> Option<&'static BootstrapCoreEngineUnitDescriptor> {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_UNITS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_core_engine_transition_descriptor(
    id: &str,
) -> Option<&'static BootstrapCoreEngineTransitionDescriptor> {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_TRANSITIONS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_core_engine_artifact_descriptor(
    id: &str,
) -> Option<&'static BootstrapCoreEngineArtifactDescriptor> {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_core_engine_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapCoreEngineProofDescriptor> {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_core_engine_unit_digest(id: &str) -> Option<String> {
    bootstrap_core_engine_unit_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_core_engine.unit.descriptor",
            &format!(
                "{}|{}|{}|{}|{}|{}|{}",
                item.id,
                item.owner_root,
                item.input_model,
                item.output_model,
                item.stage_order,
                item.engine_law,
                item.status
            ),
        )
    })
}
pub fn bootstrap_core_engine_transition_digest(id: &str) -> Option<String> {
    bootstrap_core_engine_transition_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_core_engine.transition.descriptor",
            &format!(
                "{}|{}|{}|{}|{}|{}",
                item.id, item.from_unit, item.to_unit, item.transition_law, item.carry, item.status
            ),
        )
    })
}
pub fn bootstrap_core_engine_artifact_digest(id: &str) -> Option<String> {
    bootstrap_core_engine_artifact_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_core_engine.artifact.descriptor",
            &format!(
                "{}|{}|{}|{}|{}",
                item.id, item.owner_root, item.path, item.artifact_kind, item.status
            ),
        )
    })
}
pub fn bootstrap_core_engine_proof_digest(id: &str) -> Option<String> {
    bootstrap_core_engine_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_core_engine.proof.descriptor",
            &format!(
                "{}|{}|{}|{}|{}|{}|{}|{}",
                item.id,
                item.units.join(","),
                item.transitions.join(","),
                item.artifacts.join(","),
                item.fixture,
                item.golden,
                item.receipt,
                item.status
            ),
        )
    })
}

pub fn bootstrap_core_engine_units_have_stable_order() -> bool {
    let mut orders = LYRALANG_BOOTSTRAP_CORE_ENGINE_UNITS
        .iter()
        .map(|item| item.stage_order)
        .collect::<Vec<_>>();
    let original = orders.clone();
    orders.sort();
    orders == original
        && orders.iter().all(|item| {
            item.len() == 3
                && item.as_bytes().iter().all(|byte| byte.is_ascii_digit())
                && *item != "000"
        })
}
pub fn bootstrap_core_engine_transitions_bind_known_units() -> bool {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_TRANSITIONS
        .iter()
        .all(|transition| {
            LYRALANG_BOOTSTRAP_CORE_ENGINE_UNITS
                .iter()
                .any(|unit| unit.id == transition.from_unit)
                && LYRALANG_BOOTSTRAP_CORE_ENGINE_UNITS
                    .iter()
                    .any(|unit| unit.id == transition.to_unit)
        })
}
pub fn bootstrap_core_engine_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_ARTIFACTS
        .iter()
        .all(|artifact| !artifact.path.is_empty() && !artifact.path.contains(".."))
}
pub fn bootstrap_core_engine_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_CORE_ENGINE_PROOFS.iter().all(|proof| {
        proof.units.iter().all(|id| {
            LYRALANG_BOOTSTRAP_CORE_ENGINE_UNITS
                .iter()
                .any(|unit| unit.id == *id)
        }) && proof.transitions.iter().all(|id| {
            LYRALANG_BOOTSTRAP_CORE_ENGINE_TRANSITIONS
                .iter()
                .any(|transition| transition.id == *id)
        }) && proof.artifacts.iter().all(|id| {
            LYRALANG_BOOTSTRAP_CORE_ENGINE_ARTIFACTS
                .iter()
                .any(|artifact| artifact.id == *id)
        }) && proof.fixture.starts_with("fixtures/p02/")
            && proof.golden.starts_with("goldens/p02/")
            && proof.receipt.starts_with("receipts/p02/")
    })
}
pub fn bootstrap_core_engine_no_forbidden_descriptor_claims() -> bool {
    let text = format!("{LYRALANG_BOOTSTRAP_CORE_ENGINE_UNITS:?}{LYRALANG_BOOTSTRAP_CORE_ENGINE_TRANSITIONS:?}{LYRALANG_BOOTSTRAP_CORE_ENGINE_ARTIFACTS:?}{LYRALANG_BOOTSTRAP_CORE_ENGINE_PROOFS:?}").to_ascii_lowercase();
    ![
        "network required",
        "cloud required",
        "online required",
        "probabilistic engine",
        "hidden randomness",
        "ambient time",
        "placeholder",
        "phase closed",
        "global complete",
        "forked carrier",
    ]
    .iter()
    .any(|token| text.contains(token))
}

pub fn bootstrap_core_engine_registry_hash() -> String {
    let preimage = format!(
        "units={}|transitions={}|artifacts={}|proofs={}",
        bootstrap_core_engine_unit_ids().join(","),
        bootstrap_core_engine_transition_ids().join(","),
        bootstrap_core_engine_artifact_ids().join(","),
        bootstrap_core_engine_proof_ids().join(",")
    );
    stable_hash_label("lyra.p02.bootstrap_core_engine.registry", &preimage)
}
pub fn bootstrap_core_engine_registry_signature() -> String {
    format!(
        "{}:{}",
        LYRA_P02_BOOTSTRAP_CORE_ENGINE_CARRIER,
        bootstrap_core_engine_registry_hash()
    )
}
