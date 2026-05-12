use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_BOOTSTRAP_CANONICAL_MODEL_CARRIER: &str =
    "lyra.p02.bootstrap_canonical_model.carrier.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCanonicalModelDescriptor {
    pub id: &'static str,
    pub domain_id: &'static str,
    pub schema_path: &'static str,
    pub hash_policy: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCanonicalSchemaDescriptor {
    pub id: &'static str,
    pub model_id: &'static str,
    pub contract_path: &'static str,
    pub encoding: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCanonicalFieldDescriptor {
    pub id: &'static str,
    pub model_id: &'static str,
    pub field_type: &'static str,
    pub order: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCanonicalRelationDescriptor {
    pub id: &'static str,
    pub from_model: &'static str,
    pub to_model: &'static str,
    pub relation_kind: &'static str,
}

pub const LYRALANG_BOOTSTRAP_CANONICAL_MODELS: &[BootstrapCanonicalModelDescriptor] = &[
    BootstrapCanonicalModelDescriptor {
        id: "bootstrap_trust_model",
        domain_id: "bootstrap_trust",
        schema_path: "interfaces/p02/contracts/bootstrap_trust_model.v1.lyra",
        hash_policy: "stable_hash_label",
    },
    BootstrapCanonicalModelDescriptor {
        id: "seed_runtime_law_model",
        domain_id: "seed_runtime_law",
        schema_path: "interfaces/p02/contracts/seed_runtime_law_model.v1.lyra",
        hash_policy: "stable_hash_label",
    },
    BootstrapCanonicalModelDescriptor {
        id: "host_extinction_model",
        domain_id: "host_extinction_framework",
        schema_path: "interfaces/p02/contracts/host_extinction_model.v1.lyra",
        hash_policy: "stable_hash_label",
    },
    BootstrapCanonicalModelDescriptor {
        id: "foreign_surface_boundary_model",
        domain_id: "foreign_surface_boundary",
        schema_path: "interfaces/p02/contracts/foreign_surface_boundary_model.v1.lyra",
        hash_policy: "stable_hash_label",
    },
    BootstrapCanonicalModelDescriptor {
        id: "operator_handoff_model",
        domain_id: "operator_handoff_truth",
        schema_path: "interfaces/p02/contracts/operator_handoff_model.v1.lyra",
        hash_policy: "stable_hash_label",
    },
    BootstrapCanonicalModelDescriptor {
        id: "emergency_fallback_model",
        domain_id: "emergency_fallback_safety",
        schema_path: "interfaces/p02/contracts/emergency_fallback_model.v1.lyra",
        hash_policy: "stable_hash_label",
    },
];

pub const LYRALANG_BOOTSTRAP_CANONICAL_SCHEMAS: &[BootstrapCanonicalSchemaDescriptor] = &[
    BootstrapCanonicalSchemaDescriptor {
        id: "bootstrap_trust_schema",
        model_id: "bootstrap_trust_model",
        contract_path: "interfaces/p02/contracts/bootstrap_trust_model.v1.lyra",
        encoding: "canonical_lyra_kv",
    },
    BootstrapCanonicalSchemaDescriptor {
        id: "seed_runtime_law_schema",
        model_id: "seed_runtime_law_model",
        contract_path: "interfaces/p02/contracts/seed_runtime_law_model.v1.lyra",
        encoding: "canonical_lyra_kv",
    },
    BootstrapCanonicalSchemaDescriptor {
        id: "host_extinction_schema",
        model_id: "host_extinction_model",
        contract_path: "interfaces/p02/contracts/host_extinction_model.v1.lyra",
        encoding: "canonical_lyra_kv",
    },
    BootstrapCanonicalSchemaDescriptor {
        id: "foreign_surface_boundary_schema",
        model_id: "foreign_surface_boundary_model",
        contract_path: "interfaces/p02/contracts/foreign_surface_boundary_model.v1.lyra",
        encoding: "canonical_lyra_kv",
    },
    BootstrapCanonicalSchemaDescriptor {
        id: "operator_handoff_schema",
        model_id: "operator_handoff_model",
        contract_path: "interfaces/p02/contracts/operator_handoff_model.v1.lyra",
        encoding: "canonical_lyra_kv",
    },
    BootstrapCanonicalSchemaDescriptor {
        id: "emergency_fallback_schema",
        model_id: "emergency_fallback_model",
        contract_path: "interfaces/p02/contracts/emergency_fallback_model.v1.lyra",
        encoding: "canonical_lyra_kv",
    },
];

pub const LYRALANG_BOOTSTRAP_CANONICAL_FIELDS: &[BootstrapCanonicalFieldDescriptor] = &[
    BootstrapCanonicalFieldDescriptor {
        id: "trust_receipt_hash",
        model_id: "bootstrap_trust_model",
        field_type: "hash",
        order: "001",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "trust_authority_floor",
        model_id: "bootstrap_trust_model",
        field_type: "symbol",
        order: "002",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "seed_runtime_owner",
        model_id: "seed_runtime_law_model",
        field_type: "owner_root",
        order: "001",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "seed_runtime_replacement_state",
        model_id: "seed_runtime_law_model",
        field_type: "enum",
        order: "002",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "host_surface_id",
        model_id: "host_extinction_model",
        field_type: "symbol",
        order: "001",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "host_extinction_gate",
        model_id: "host_extinction_model",
        field_type: "receipt_path",
        order: "002",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "foreign_surface_id",
        model_id: "foreign_surface_boundary_model",
        field_type: "symbol",
        order: "001",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "foreign_challenge_suite",
        model_id: "foreign_surface_boundary_model",
        field_type: "receipt_path",
        order: "002",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "handoff_capture_channel",
        model_id: "operator_handoff_model",
        field_type: "symbol",
        order: "001",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "handoff_truth_gate",
        model_id: "operator_handoff_model",
        field_type: "receipt_path",
        order: "002",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "fallback_freeze_gate",
        model_id: "emergency_fallback_model",
        field_type: "receipt_path",
        order: "001",
    },
    BootstrapCanonicalFieldDescriptor {
        id: "fallback_recovery_state",
        model_id: "emergency_fallback_model",
        field_type: "enum",
        order: "002",
    },
];

pub const LYRALANG_BOOTSTRAP_CANONICAL_RELATIONS: &[BootstrapCanonicalRelationDescriptor] = &[
    BootstrapCanonicalRelationDescriptor {
        id: "trust_model_to_seed_runtime",
        from_model: "bootstrap_trust_model",
        to_model: "seed_runtime_law_model",
        relation_kind: "receipt_enables",
    },
    BootstrapCanonicalRelationDescriptor {
        id: "seed_runtime_to_host_extinction",
        from_model: "seed_runtime_law_model",
        to_model: "host_extinction_model",
        relation_kind: "replacement_gates",
    },
    BootstrapCanonicalRelationDescriptor {
        id: "host_extinction_to_foreign_surface",
        from_model: "host_extinction_model",
        to_model: "foreign_surface_boundary_model",
        relation_kind: "boundary_exposes",
    },
    BootstrapCanonicalRelationDescriptor {
        id: "foreign_surface_to_handoff",
        from_model: "foreign_surface_boundary_model",
        to_model: "operator_handoff_model",
        relation_kind: "challenge_feeds",
    },
    BootstrapCanonicalRelationDescriptor {
        id: "handoff_to_fallback",
        from_model: "operator_handoff_model",
        to_model: "emergency_fallback_model",
        relation_kind: "freeze_controls",
    },
    BootstrapCanonicalRelationDescriptor {
        id: "fallback_to_trust_model",
        from_model: "emergency_fallback_model",
        to_model: "bootstrap_trust_model",
        relation_kind: "recovery_rechecks",
    },
];

pub fn bootstrap_canonical_model_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CANONICAL_MODELS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_canonical_schema_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CANONICAL_SCHEMAS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_canonical_field_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CANONICAL_FIELDS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_canonical_relation_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CANONICAL_RELATIONS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_canonical_all_models_have_local_schema() -> bool {
    LYRALANG_BOOTSTRAP_CANONICAL_MODELS
        .iter()
        .all(|x| x.schema_path.starts_with("interfaces/p02/contracts/"))
}
pub fn bootstrap_canonical_all_schemas_bind_models() -> bool {
    LYRALANG_BOOTSTRAP_CANONICAL_SCHEMAS.iter().all(|schema| {
        LYRALANG_BOOTSTRAP_CANONICAL_MODELS
            .iter()
            .any(|model| model.id == schema.model_id)
    })
}
pub fn bootstrap_canonical_all_fields_bind_models() -> bool {
    LYRALANG_BOOTSTRAP_CANONICAL_FIELDS.iter().all(|field| {
        LYRALANG_BOOTSTRAP_CANONICAL_MODELS
            .iter()
            .any(|model| model.id == field.model_id)
    })
}
pub fn bootstrap_canonical_all_relations_bind_models() -> bool {
    LYRALANG_BOOTSTRAP_CANONICAL_RELATIONS
        .iter()
        .all(|relation| {
            LYRALANG_BOOTSTRAP_CANONICAL_MODELS
                .iter()
                .any(|model| model.id == relation.from_model)
                && LYRALANG_BOOTSTRAP_CANONICAL_MODELS
                    .iter()
                    .any(|model| model.id == relation.to_model)
        })
}

pub fn bootstrap_canonical_model_registry_hash() -> String {
    let preimage = format!(
        "models={}|schemas={}|fields={}|relations={}",
        bootstrap_canonical_model_ids().join(","),
        bootstrap_canonical_schema_ids().join(","),
        bootstrap_canonical_field_ids().join(","),
        bootstrap_canonical_relation_ids().join(",")
    );
    stable_hash_label("lyra.p02.bootstrap_canonical_model.registry", &preimage)
}
pub fn bootstrap_canonical_model_registry_signature() -> String {
    format!(
        "{}:{}",
        LYRA_P02_BOOTSTRAP_CANONICAL_MODEL_CARRIER,
        bootstrap_canonical_model_registry_hash()
    )
}
