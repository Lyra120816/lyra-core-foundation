use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalDataModelDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub owner_root: &'static str,
    pub source_task: &'static str,
    pub schema_ref: &'static str,
    pub canonical_order: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalSchemaDescriptor {
    pub id: &'static str,
    pub model_ref: &'static str,
    pub fields: &'static [&'static str],
    pub required: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalFieldDescriptor {
    pub id: &'static str,
    pub model_ref: &'static str,
    pub kind: &'static str,
    pub order: &'static str,
    pub normalization: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalModelBridgeDescriptor {
    pub id: &'static str,
    pub from_model: &'static str,
    pub to_model: &'static str,
    pub carrier: &'static str,
    pub receipt_ref: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalDataProofDescriptor {
    pub id: &'static str,
    pub models: &'static [&'static str],
    pub schemas: &'static [&'static str],
    pub fields: &'static [&'static str],
    pub bridges: &'static [&'static str],
    pub fixture: &'static str,
    pub golden: &'static str,
    pub receipt: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalDataModelError {
    UnknownModel,
    UnknownSchema,
    UnknownField,
    UnknownBridge,
    UnknownProof,
}

pub const LYRA_P01_CANONICAL_DATA_CARRIER: &str = "lyra_p01_semantic_core";

pub const LYRALANG_CANONICAL_DATA_MODELS: &[CanonicalDataModelDescriptor] = &[
    CanonicalDataModelDescriptor {
        id: "canonical_symbol_model",
        scope: "symbol",
        owner_root: "lyralang",
        source_task: "P01-001",
        schema_ref: "canonical_symbol_schema",
        canonical_order: &["symbol_id", "symbol_namespace", "symbol_kind"],
        status: "artifact_emitted",
    },
    CanonicalDataModelDescriptor {
        id: "semantic_atom_model",
        scope: "atom",
        owner_root: "lyralang",
        source_task: "P01-001",
        schema_ref: "semantic_atom_schema",
        canonical_order: &["atom_family", "atom_name", "atom_capability"],
        status: "artifact_emitted",
    },
    CanonicalDataModelDescriptor {
        id: "core_ir_term_model",
        scope: "ir",
        owner_root: "lyralang",
        source_task: "P01-002",
        schema_ref: "core_ir_term_schema",
        canonical_order: &["ir_kind", "ir_children", "ir_effects"],
        status: "artifact_emitted",
    },
    CanonicalDataModelDescriptor {
        id: "core_ir_form_model",
        scope: "ir",
        owner_root: "interfaces",
        source_task: "P01-002",
        schema_ref: "core_ir_form_schema",
        canonical_order: &["ir_kind", "ir_capabilities", "ir_receipt_ref"],
        status: "artifact_emitted",
    },
    CanonicalDataModelDescriptor {
        id: "semantic_object_model",
        scope: "object",
        owner_root: "lyralang",
        source_task: "P01-003",
        schema_ref: "semantic_object_schema",
        canonical_order: &["object_id", "object_kind", "object_digest"],
        status: "artifact_emitted",
    },
    CanonicalDataModelDescriptor {
        id: "semantic_identity_model",
        scope: "identity",
        owner_root: "k0",
        source_task: "P01-004",
        schema_ref: "semantic_identity_schema",
        canonical_order: &["identity_digest", "object_digest", "receipt_hash"],
        status: "execution_proven",
    },
    CanonicalDataModelDescriptor {
        id: "symbolic_equality_model",
        scope: "equality",
        owner_root: "lyralang",
        source_task: "P01-006",
        schema_ref: "symbolic_equality_schema",
        canonical_order: &["equality_normal_form", "identity_digest", "receipt_verdict"],
        status: "execution_proven",
    },
    CanonicalDataModelDescriptor {
        id: "semantic_receipt_model",
        scope: "receipt",
        owner_root: "k0",
        source_task: "P01-012",
        schema_ref: "semantic_receipt_schema",
        canonical_order: &["receipt_hash", "receipt_verdict", "ir_receipt_ref"],
        status: "execution_proven",
    },
];

pub const LYRALANG_CANONICAL_SCHEMAS: &[CanonicalSchemaDescriptor] = &[
    CanonicalSchemaDescriptor {
        id: "canonical_symbol_schema",
        model_ref: "canonical_symbol_model",
        fields: &["symbol_id", "symbol_namespace", "symbol_kind"],
        required: &["symbol_id", "symbol_kind"],
        forbids: &["ambient_default", "host_order", "random_identity"],
        status: "artifact_emitted",
    },
    CanonicalSchemaDescriptor {
        id: "semantic_atom_schema",
        model_ref: "semantic_atom_model",
        fields: &["atom_family", "atom_name", "atom_capability"],
        required: &["atom_family", "atom_name"],
        forbids: &["open_world_atom", "network_lookup", "implicit_probability"],
        status: "artifact_emitted",
    },
    CanonicalSchemaDescriptor {
        id: "core_ir_term_schema",
        model_ref: "core_ir_term_model",
        fields: &["ir_kind", "ir_children", "ir_effects"],
        required: &["ir_kind"],
        forbids: &["foreign_ast_owner", "implicit_effect", "ambient_time"],
        status: "artifact_emitted",
    },
    CanonicalSchemaDescriptor {
        id: "core_ir_form_schema",
        model_ref: "core_ir_form_model",
        fields: &["ir_kind", "ir_capabilities", "ir_receipt_ref"],
        required: &["ir_kind", "ir_receipt_ref"],
        forbids: &[
            "unstable_encoding",
            "missing_version",
            "unreceipted_upgrade",
        ],
        status: "artifact_emitted",
    },
    CanonicalSchemaDescriptor {
        id: "semantic_object_schema",
        model_ref: "semantic_object_model",
        fields: &["object_id", "object_kind", "object_digest"],
        required: &["object_id", "object_digest"],
        forbids: &["unhashed_object", "unordered_members", "empty_object"],
        status: "artifact_emitted",
    },
    CanonicalSchemaDescriptor {
        id: "semantic_identity_schema",
        model_ref: "semantic_identity_model",
        fields: &["identity_digest", "object_digest", "receipt_hash"],
        required: &["identity_digest", "receipt_hash"],
        forbids: &[
            "digest_collision_acceptance",
            "unchecked_alias",
            "host_path_identity",
        ],
        status: "execution_proven",
    },
    CanonicalSchemaDescriptor {
        id: "symbolic_equality_schema",
        model_ref: "symbolic_equality_model",
        fields: &["equality_normal_form", "identity_digest", "receipt_verdict"],
        required: &["equality_normal_form", "identity_digest"],
        forbids: &[
            "noncanonical_equivalence",
            "side_effecting_equality",
            "weighted_equality",
        ],
        status: "execution_proven",
    },
    CanonicalSchemaDescriptor {
        id: "semantic_receipt_schema",
        model_ref: "semantic_receipt_model",
        fields: &["receipt_hash", "receipt_verdict", "ir_receipt_ref"],
        required: &["receipt_hash", "receipt_verdict"],
        forbids: &["missing_contract", "missing_law", "verdict_drift"],
        status: "execution_proven",
    },
];

pub const LYRALANG_CANONICAL_FIELDS: &[CanonicalFieldDescriptor] = &[
    CanonicalFieldDescriptor {
        id: "symbol_id",
        model_ref: "canonical_symbol_model",
        kind: "symbol",
        order: "001",
        normalization: "utf8_trimmed_symbolic_identifier",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "symbol_namespace",
        model_ref: "canonical_symbol_model",
        kind: "symbol",
        order: "002",
        normalization: "namespace_path_segments_sorted_and_joined",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "symbol_kind",
        model_ref: "canonical_symbol_model",
        kind: "enum",
        order: "003",
        normalization: "one_of_symbol_value_type_effect_capability_proof_receipt_resource_law",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "atom_family",
        model_ref: "semantic_atom_model",
        kind: "enum",
        order: "001",
        normalization: "closed_family_from_p01_semantic_atom_registry",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "atom_name",
        model_ref: "semantic_atom_model",
        kind: "symbol",
        order: "002",
        normalization: "atom_name_lowercase_symbolic_identifier",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "atom_capability",
        model_ref: "semantic_atom_model",
        kind: "capability",
        order: "003",
        normalization: "capability_atom_must_reference_closed_capability_set",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "ir_kind",
        model_ref: "core_ir_term_model",
        kind: "enum",
        order: "001",
        normalization: "closed_core_ir_kind_from_p01_core_ir_registry",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "ir_children",
        model_ref: "core_ir_term_model",
        kind: "list",
        order: "002",
        normalization: "children_sorted_by_canonical_identity_then_digest",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "ir_effects",
        model_ref: "core_ir_term_model",
        kind: "list",
        order: "003",
        normalization: "effect_atoms_sorted_by_atom_digest",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "ir_capabilities",
        model_ref: "core_ir_form_model",
        kind: "list",
        order: "002",
        normalization: "capability_atoms_sorted_by_atom_digest",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "ir_receipt_ref",
        model_ref: "core_ir_form_model",
        kind: "path",
        order: "003",
        normalization: "receipts_p01_path_to_bound_core_ir_receipt",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "object_id",
        model_ref: "semantic_object_model",
        kind: "symbol",
        order: "001",
        normalization: "object_id_lowercase_symbolic_identifier",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "object_kind",
        model_ref: "semantic_object_model",
        kind: "enum",
        order: "002",
        normalization: "one_of_module_package_program_world_plan_trace_proof",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "object_digest",
        model_ref: "semantic_object_model",
        kind: "hash",
        order: "003",
        normalization: "fnv1a128_over_canonical_object_preimage",
        status: "artifact_emitted",
    },
    CanonicalFieldDescriptor {
        id: "identity_digest",
        model_ref: "semantic_identity_model",
        kind: "hash",
        order: "001",
        normalization: "fnv1a128_over_identity_domain_and_canonical_preimage",
        status: "execution_proven",
    },
    CanonicalFieldDescriptor {
        id: "equality_normal_form",
        model_ref: "symbolic_equality_model",
        kind: "object",
        order: "001",
        normalization: "canonical_normal_form_after_total_symbolic_rewrite",
        status: "execution_proven",
    },
    CanonicalFieldDescriptor {
        id: "receipt_hash",
        model_ref: "semantic_receipt_model",
        kind: "hash",
        order: "001",
        normalization: "fnv1a128_over_phase_receipt_preimage",
        status: "execution_proven",
    },
    CanonicalFieldDescriptor {
        id: "receipt_verdict",
        model_ref: "semantic_receipt_model",
        kind: "verdict",
        order: "002",
        normalization: "accepted_or_rejected_only_with_sorted_error_rows",
        status: "execution_proven",
    },
];

pub const LYRALANG_CANONICAL_MODEL_BRIDGES: &[CanonicalModelBridgeDescriptor] = &[
    CanonicalModelBridgeDescriptor {
        id: "symbols_to_atoms",
        from_model: "canonical_symbol_model",
        to_model: "semantic_atom_model",
        carrier: "lyra_p01_semantic_core",
        receipt_ref: "receipts/p01/pass_0030_semantic_atoms.receipt",
        status: "execution_proven",
    },
    CanonicalModelBridgeDescriptor {
        id: "atoms_to_core_ir",
        from_model: "semantic_atom_model",
        to_model: "core_ir_term_model",
        carrier: "lyra_p01_semantic_core",
        receipt_ref: "receipts/p01/pass_0031_core_ir.receipt",
        status: "execution_proven",
    },
    CanonicalModelBridgeDescriptor {
        id: "core_ir_to_objects",
        from_model: "core_ir_term_model",
        to_model: "semantic_object_model",
        carrier: "lyra_p01_semantic_core",
        receipt_ref: "receipts/p01/pass_0032_semantic_objects.receipt",
        status: "execution_proven",
    },
    CanonicalModelBridgeDescriptor {
        id: "objects_to_identity",
        from_model: "semantic_object_model",
        to_model: "semantic_identity_model",
        carrier: "lyra_p01_semantic_core",
        receipt_ref: "receipts/p01/pass_0033_semantic_identity.receipt",
        status: "execution_proven",
    },
    CanonicalModelBridgeDescriptor {
        id: "equality_to_receipts",
        from_model: "symbolic_equality_model",
        to_model: "semantic_receipt_model",
        carrier: "lyra_p01_semantic_core",
        receipt_ref: "receipts/p01/pass_0035_symbolic_equality.receipt",
        status: "execution_proven",
    },
    CanonicalModelBridgeDescriptor {
        id: "receipts_to_proofs",
        from_model: "semantic_receipt_model",
        to_model: "core_ir_form_model",
        carrier: "lyra_p01_semantic_core",
        receipt_ref: "receipts/p01/pass_0042_formal_semantic_constitution.receipt",
        status: "execution_proven",
    },
];

pub const LYRALANG_CANONICAL_DATA_PROOFS: &[CanonicalDataProofDescriptor] = &[
    CanonicalDataProofDescriptor {
        id: "symbol_model_proof",
        models: &["canonical_symbol_model"],
        schemas: &["canonical_symbol_schema"],
        fields: &["symbol_id", "symbol_namespace", "symbol_kind"],
        bridges: &["symbols_to_atoms"],
        fixture: "fixtures/p01/canonical_data_model_inputs/valid_canonical_data_model.lyra",
        golden: "goldens/p01/valid_canonical_data_model.receipt",
        receipt: "receipts/p01/pass_0043_canonical_data_model.receipt",
        status: "execution_proven",
    },
    CanonicalDataProofDescriptor {
        id: "atom_model_proof",
        models: &["semantic_atom_model"],
        schemas: &["semantic_atom_schema"],
        fields: &["atom_family", "atom_name", "atom_capability"],
        bridges: &["symbols_to_atoms", "atoms_to_core_ir"],
        fixture: "fixtures/p01/canonical_data_model_cases/semantic_atom_model_case.lyra",
        golden: "goldens/p01/valid_canonical_data_model.receipt",
        receipt: "receipts/p01/pass_0043_canonical_data_model.receipt",
        status: "execution_proven",
    },
    CanonicalDataProofDescriptor {
        id: "core_ir_model_proof",
        models: &["core_ir_term_model", "core_ir_form_model"],
        schemas: &["core_ir_term_schema", "core_ir_form_schema"],
        fields: &[
            "ir_kind",
            "ir_children",
            "ir_effects",
            "ir_capabilities",
            "ir_receipt_ref",
        ],
        bridges: &["atoms_to_core_ir", "core_ir_to_objects"],
        fixture: "fixtures/p01/canonical_data_model_cases/core_ir_model_case.lyra",
        golden: "goldens/p01/valid_canonical_data_model.receipt",
        receipt: "receipts/p01/pass_0043_canonical_data_model.receipt",
        status: "execution_proven",
    },
    CanonicalDataProofDescriptor {
        id: "object_identity_model_proof",
        models: &["semantic_object_model", "semantic_identity_model"],
        schemas: &["semantic_object_schema", "semantic_identity_schema"],
        fields: &[
            "object_id",
            "object_kind",
            "object_digest",
            "identity_digest",
        ],
        bridges: &["core_ir_to_objects", "objects_to_identity"],
        fixture: "fixtures/p01/canonical_data_model_cases/object_identity_model_case.lyra",
        golden: "goldens/p01/valid_canonical_data_model.receipt",
        receipt: "receipts/p01/pass_0043_canonical_data_model.receipt",
        status: "execution_proven",
    },
    CanonicalDataProofDescriptor {
        id: "equality_receipt_model_proof",
        models: &["symbolic_equality_model", "semantic_receipt_model"],
        schemas: &["symbolic_equality_schema", "semantic_receipt_schema"],
        fields: &["equality_normal_form", "receipt_hash", "receipt_verdict"],
        bridges: &["equality_to_receipts", "receipts_to_proofs"],
        fixture: "fixtures/p01/canonical_data_model_cases/equality_receipt_model_case.lyra",
        golden: "goldens/p01/valid_canonical_data_model.receipt",
        receipt: "receipts/p01/pass_0043_canonical_data_model.receipt",
        status: "execution_proven",
    },
    CanonicalDataProofDescriptor {
        id: "p01_canonical_data_parity_proof",
        models: &[
            "canonical_symbol_model",
            "semantic_atom_model",
            "core_ir_term_model",
            "core_ir_form_model",
            "semantic_object_model",
            "semantic_identity_model",
            "symbolic_equality_model",
            "semantic_receipt_model",
        ],
        schemas: &[
            "canonical_symbol_schema",
            "semantic_atom_schema",
            "core_ir_term_schema",
            "core_ir_form_schema",
            "semantic_object_schema",
            "semantic_identity_schema",
            "symbolic_equality_schema",
            "semantic_receipt_schema",
        ],
        fields: &[
            "symbol_id",
            "symbol_namespace",
            "symbol_kind",
            "atom_family",
            "atom_name",
            "atom_capability",
            "ir_kind",
            "ir_children",
            "ir_effects",
            "ir_capabilities",
            "ir_receipt_ref",
            "object_id",
            "object_kind",
            "object_digest",
            "identity_digest",
            "equality_normal_form",
            "receipt_hash",
            "receipt_verdict",
        ],
        bridges: &[
            "symbols_to_atoms",
            "atoms_to_core_ir",
            "core_ir_to_objects",
            "objects_to_identity",
            "equality_to_receipts",
            "receipts_to_proofs",
        ],
        fixture: "fixtures/p01/canonical_data_model_cases/full_parity_model_case.lyra",
        golden: "goldens/p01/valid_canonical_data_model.receipt",
        receipt: "receipts/p01/pass_0043_canonical_data_model.receipt",
        status: "execution_proven",
    },
];

pub fn canonical_data_model_ids() -> Vec<&'static str> {
    LYRALANG_CANONICAL_DATA_MODELS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn canonical_schema_ids() -> Vec<&'static str> {
    LYRALANG_CANONICAL_SCHEMAS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn canonical_field_ids() -> Vec<&'static str> {
    LYRALANG_CANONICAL_FIELDS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn canonical_model_bridge_ids() -> Vec<&'static str> {
    LYRALANG_CANONICAL_MODEL_BRIDGES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn canonical_data_proof_ids() -> Vec<&'static str> {
    LYRALANG_CANONICAL_DATA_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn canonical_data_model_descriptor(id: &str) -> Option<&'static CanonicalDataModelDescriptor> {
    LYRALANG_CANONICAL_DATA_MODELS
        .iter()
        .find(|item| item.id == id)
}
pub fn canonical_schema_descriptor(id: &str) -> Option<&'static CanonicalSchemaDescriptor> {
    LYRALANG_CANONICAL_SCHEMAS.iter().find(|item| item.id == id)
}
pub fn canonical_field_descriptor(id: &str) -> Option<&'static CanonicalFieldDescriptor> {
    LYRALANG_CANONICAL_FIELDS.iter().find(|item| item.id == id)
}
pub fn canonical_model_bridge_descriptor(
    id: &str,
) -> Option<&'static CanonicalModelBridgeDescriptor> {
    LYRALANG_CANONICAL_MODEL_BRIDGES
        .iter()
        .find(|item| item.id == id)
}
pub fn canonical_data_proof_descriptor(id: &str) -> Option<&'static CanonicalDataProofDescriptor> {
    LYRALANG_CANONICAL_DATA_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn canonical_data_model_signature(item: &CanonicalDataModelDescriptor) -> String {
    format!(
        "model:{}|scope:{}|owner:{}|source:{}|schema:{}|order:{}|status:{}",
        item.id,
        item.scope,
        item.owner_root,
        item.source_task,
        item.schema_ref,
        item.canonical_order.join(","),
        item.status
    )
}

pub fn canonical_schema_signature(item: &CanonicalSchemaDescriptor) -> String {
    format!(
        "schema:{}|model:{}|fields:{}|required:{}|forbids:{}|status:{}",
        item.id,
        item.model_ref,
        item.fields.join(","),
        item.required.join(","),
        item.forbids.join(","),
        item.status
    )
}

pub fn canonical_field_signature(item: &CanonicalFieldDescriptor) -> String {
    format!(
        "field:{}|model:{}|kind:{}|order:{}|normalization:{}|status:{}",
        item.id, item.model_ref, item.kind, item.order, item.normalization, item.status
    )
}

pub fn canonical_model_bridge_signature(item: &CanonicalModelBridgeDescriptor) -> String {
    format!(
        "bridge:{}|from:{}|to:{}|carrier:{}|receipt:{}|status:{}",
        item.id, item.from_model, item.to_model, item.carrier, item.receipt_ref, item.status
    )
}

pub fn canonical_data_proof_signature(item: &CanonicalDataProofDescriptor) -> String {
    format!("proof:{}|models:{}|schemas:{}|fields:{}|bridges:{}|fixture:{}|golden:{}|receipt:{}|status:{}", item.id, item.models.join(","), item.schemas.join(","), item.fields.join(","), item.bridges.join(","), item.fixture, item.golden, item.receipt, item.status)
}

pub fn canonical_data_model_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_CANONICAL_DATA_MODELS {
        rows.push(canonical_data_model_signature(item));
    }
    for item in LYRALANG_CANONICAL_SCHEMAS {
        rows.push(canonical_schema_signature(item));
    }
    for item in LYRALANG_CANONICAL_FIELDS {
        rows.push(canonical_field_signature(item));
    }
    for item in LYRALANG_CANONICAL_MODEL_BRIDGES {
        rows.push(canonical_model_bridge_signature(item));
    }
    for item in LYRALANG_CANONICAL_DATA_PROOFS {
        rows.push(canonical_data_proof_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn canonical_data_model_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.canonical_data_model.registry",
        &canonical_data_model_registry_signature(),
    )
}
pub fn canonical_data_model_digest(id: &str) -> Option<String> {
    canonical_data_model_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.canonical_data_model.model",
            &canonical_data_model_signature(item),
        )
    })
}
pub fn canonical_schema_digest(id: &str) -> Option<String> {
    canonical_schema_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.canonical_data_model.schema",
            &canonical_schema_signature(item),
        )
    })
}
pub fn canonical_field_digest(id: &str) -> Option<String> {
    canonical_field_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.canonical_data_model.field",
            &canonical_field_signature(item),
        )
    })
}
pub fn canonical_model_bridge_digest(id: &str) -> Option<String> {
    canonical_model_bridge_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.canonical_data_model.bridge",
            &canonical_model_bridge_signature(item),
        )
    })
}
pub fn canonical_data_proof_digest(id: &str) -> Option<String> {
    canonical_data_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.canonical_data_model.proof",
            &canonical_data_proof_signature(item),
        )
    })
}

pub fn canonical_data_models_have_schema_refs() -> bool {
    LYRALANG_CANONICAL_DATA_MODELS
        .iter()
        .all(|model| canonical_schema_descriptor(model.schema_ref).is_some())
}

pub fn canonical_schemas_bind_known_models_and_fields() -> bool {
    LYRALANG_CANONICAL_SCHEMAS.iter().all(|schema| {
        canonical_data_model_descriptor(schema.model_ref).is_some()
            && schema
                .fields
                .iter()
                .all(|field| canonical_field_descriptor(field).is_some())
            && schema
                .required
                .iter()
                .all(|field| schema.fields.contains(field))
    })
}

pub fn canonical_fields_bind_known_models() -> bool {
    LYRALANG_CANONICAL_FIELDS
        .iter()
        .all(|field| canonical_data_model_descriptor(field.model_ref).is_some())
}

pub fn canonical_model_bridges_bind_one_carrier() -> bool {
    LYRALANG_CANONICAL_MODEL_BRIDGES.iter().all(|bridge| {
        bridge.carrier == LYRA_P01_CANONICAL_DATA_CARRIER
            && canonical_data_model_descriptor(bridge.from_model).is_some()
            && canonical_data_model_descriptor(bridge.to_model).is_some()
            && bridge.receipt_ref.starts_with("receipts/p01/")
            && bridge.receipt_ref.ends_with(".receipt")
    })
}

pub fn canonical_data_proofs_bind_artifacts() -> bool {
    LYRALANG_CANONICAL_DATA_PROOFS.iter().all(|proof| {
        proof
            .models
            .iter()
            .all(|id| canonical_data_model_descriptor(id).is_some())
            && proof
                .schemas
                .iter()
                .all(|id| canonical_schema_descriptor(id).is_some())
            && proof
                .fields
                .iter()
                .all(|id| canonical_field_descriptor(id).is_some())
            && proof
                .bridges
                .iter()
                .all(|id| canonical_model_bridge_descriptor(id).is_some())
            && proof.fixture.ends_with(".lyra")
            && proof.golden.ends_with(".receipt")
            && proof.receipt.ends_with(".receipt")
    })
}

pub fn canonical_data_no_forbidden_descriptor_claims() -> bool {
    let lower = canonical_data_model_registry_signature().to_ascii_lowercase();
    !(lower.contains("probabilistic")
        || lower.contains("stochastic")
        || lower.contains("hidden randomness")
        || lower.contains("network required")
        || lower.contains("placeholder")
        || lower.contains("todo")
        || lower.contains("phase closed"))
}
