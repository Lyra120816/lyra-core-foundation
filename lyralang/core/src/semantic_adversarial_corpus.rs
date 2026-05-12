use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticCollisionProbeDescriptor {
    pub id: &'static str,
    pub target_family: &'static str,
    pub left_ref: &'static str,
    pub right_ref: &'static str,
    pub collision_domain: &'static str,
    pub guard: &'static str,
    pub expected_error: &'static str,
    pub fixture_path: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticAmbiguityProbeDescriptor {
    pub id: &'static str,
    pub target_family: &'static str,
    pub ambiguous_surface: &'static str,
    pub deterministic_resolution: &'static str,
    pub guard: &'static str,
    pub expected_error: &'static str,
    pub fixture_path: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticMalformedObjectDescriptor {
    pub id: &'static str,
    pub target_family: &'static str,
    pub object_ref: &'static str,
    pub malformed_field: &'static str,
    pub rejection_law: &'static str,
    pub expected_error: &'static str,
    pub fixture_path: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticAdversarialHarnessDescriptor {
    pub id: &'static str,
    pub validator: &'static str,
    pub case_ids: &'static str,
    pub coverage: &'static str,
    pub receipt_ref: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticAdversarialCorpusError {
    UnknownCollisionProbe { id: String },
    UnknownAmbiguityProbe { id: String },
    UnknownMalformedObject { id: String },
    UnknownHarness { id: String },
}

pub const LYRALANG_SEMANTIC_COLLISION_PROBES: &[SemanticCollisionProbeDescriptor] = &[
    SemanticCollisionProbeDescriptor {
        id: "digest_alias_rewrite",
        target_family: "semantic_identity",
        left_ref: "semantic_identity:symbol_digest",
        right_ref: "semantic_identity:declaration_digest",
        collision_domain: "lyra.p01.identity.digest",
        guard: "different_preimages_must_not_share_digest",
        expected_error: "receipt_hash_mismatch",
        fixture_path:
            "fixtures/p01/semantic_adversarial_corpus_cases/collision_digest_alias_rewrite.lyra",
        status: "artifact_emitted",
    },
    SemanticCollisionProbeDescriptor {
        id: "object_hash_payload_swap",
        target_family: "semantic_serialization_hashing",
        left_ref: "semantic_atom:symbol",
        right_ref: "semantic_atom:value",
        collision_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom",
        guard: "payload_hash_binds_object_ref_and_family",
        expected_error: "receipt_hash_mismatch",
        fixture_path:
            "fixtures/p01/semantic_adversarial_corpus_cases/collision_object_hash_payload_swap.lyra",
        status: "artifact_emitted",
    },
    SemanticCollisionProbeDescriptor {
        id: "receipt_target_alias",
        target_family: "error_challenge_evidence",
        left_ref: "receipt:receipt_error_challenge_evidence",
        right_ref: "receipt:receipt_semantic_serialization_hashing",
        collision_domain: "lyra.p01.receipt.target",
        guard: "receipt_target_must_match_task",
        expected_error: "canonical_model_unbound",
        fixture_path:
            "fixtures/p01/semantic_adversarial_corpus_cases/collision_receipt_target_alias.lyra",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_AMBIGUITY_PROBES: &[SemanticAmbiguityProbeDescriptor] = &[
    SemanticAmbiguityProbeDescriptor { id: "text_ir_header_case_drift", target_family: "core_ir", ambiguous_surface: "lyrair_header_case", deterministic_resolution: "reject_header_drift", guard: "headers_are_byte_exact", expected_error: "invalid_header", fixture_path: "fixtures/p01/semantic_adversarial_corpus_cases/ambiguity_text_ir_header_case_drift.lyra", status: "artifact_emitted" },
    SemanticAmbiguityProbeDescriptor { id: "record_key_order_shadow", target_family: "symbolic_equality", ambiguous_surface: "record_order_equivalence_shadow", deterministic_resolution: "canonical_sort_then_compare", guard: "record_keys_sort_ascii_before_hash", expected_error: "canonical_model_drift_accepted", fixture_path: "fixtures/p01/semantic_adversarial_corpus_cases/ambiguity_record_key_order_shadow.lyra", status: "artifact_emitted" },
    SemanticAmbiguityProbeDescriptor { id: "alpha_bind_rename_shadow", target_family: "symbolic_equality", ambiguous_surface: "binder_rename_capture_shadow", deterministic_resolution: "reject_capture_risk", guard: "capture_risk_rejects_not_renames", expected_error: "invalid_model_binding", fixture_path: "fixtures/p01/semantic_adversarial_corpus_cases/ambiguity_alpha_bind_rename_shadow.lyra", status: "artifact_emitted" },
];

pub const LYRALANG_SEMANTIC_MALFORMED_OBJECTS: &[SemanticMalformedObjectDescriptor] = &[
    SemanticMalformedObjectDescriptor { id: "semantic_object_missing_relation", target_family: "semantic_object", object_ref: "semantic_object:program", malformed_field: "relation", rejection_law: "object_graph_requires_bound_relation", expected_error: "missing_model_binding", fixture_path: "fixtures/p01/semantic_adversarial_corpus_cases/malformed_semantic_object_missing_relation.lyra", status: "artifact_emitted" },
    SemanticMalformedObjectDescriptor { id: "binary_ir_header_drift", target_family: "core_ir", object_ref: "core_ir:binary_ir", malformed_field: "header", rejection_law: "binary_header_must_equal_magic", expected_error: "invalid_header", fixture_path: "fixtures/p01/semantic_adversarial_corpus_cases/malformed_binary_ir_header_drift.lyra", status: "artifact_emitted" },
    SemanticMalformedObjectDescriptor { id: "unknown_serializer_object_ref", target_family: "semantic_serialization_hashing", object_ref: "semantic_atom:unknown", malformed_field: "object_ref", rejection_law: "serializer_object_ref_must_exist", expected_error: "canonical_model_unbound", fixture_path: "fixtures/p01/semantic_adversarial_corpus_cases/malformed_unknown_serializer_object_ref.lyra", status: "artifact_emitted" },
    SemanticMalformedObjectDescriptor { id: "semantic_object_cycle", target_family: "semantic_object", object_ref: "semantic_object:world", malformed_field: "parent", rejection_law: "semantic_object_graph_must_be_acyclic", expected_error: "invalid_model_binding", fixture_path: "fixtures/p01/semantic_adversarial_corpus_cases/malformed_semantic_object_cycle.lyra", status: "artifact_emitted" },
    SemanticMalformedObjectDescriptor { id: "ambient_network_requirement", target_family: "reference_semantics", object_ref: "reference_eval_seed:seed_literal_unit", malformed_field: "dependency", rejection_law: "semantic_core_is_offline_only", expected_error: "ambient_network_allowed", fixture_path: "fixtures/p01/semantic_adversarial_corpus_cases/malformed_ambient_network_requirement.lyra", status: "artifact_emitted" },
];

pub const LYRALANG_SEMANTIC_ADVERSARIAL_HARNESSES: &[SemanticAdversarialHarnessDescriptor] = &[
    SemanticAdversarialHarnessDescriptor { id: "collision_guard_harness", validator: "semantic_collision_probe_validator", case_ids: "digest_alias_rewrite,object_hash_payload_swap,receipt_target_alias", coverage: "collision", receipt_ref: "receipt_semantic_adversarial_corpus", status: "execution_proven" },
    SemanticAdversarialHarnessDescriptor { id: "ambiguity_guard_harness", validator: "semantic_ambiguity_probe_validator", case_ids: "text_ir_header_case_drift,record_key_order_shadow,alpha_bind_rename_shadow", coverage: "ambiguity", receipt_ref: "receipt_semantic_adversarial_corpus", status: "execution_proven" },
    SemanticAdversarialHarnessDescriptor { id: "malformed_object_guard_harness", validator: "semantic_malformed_object_validator", case_ids: "semantic_object_missing_relation,binary_ir_header_drift,unknown_serializer_object_ref,semantic_object_cycle,ambient_network_requirement", coverage: "malformed_object", receipt_ref: "receipt_semantic_adversarial_corpus", status: "execution_proven" },
    SemanticAdversarialHarnessDescriptor { id: "full_semantic_core_adversarial_harness", validator: "semantic_core_adversarial_suite_validator", case_ids: "digest_alias_rewrite,object_hash_payload_swap,receipt_target_alias,text_ir_header_case_drift,record_key_order_shadow,alpha_bind_rename_shadow,semantic_object_missing_relation,binary_ir_header_drift,unknown_serializer_object_ref,semantic_object_cycle,ambient_network_requirement", coverage: "all_semantic_core", receipt_ref: "receipt_semantic_adversarial_corpus", status: "execution_proven" },
];

pub fn semantic_collision_probe_descriptor(
    id: &str,
) -> Option<&'static SemanticCollisionProbeDescriptor> {
    LYRALANG_SEMANTIC_COLLISION_PROBES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_ambiguity_probe_descriptor(
    id: &str,
) -> Option<&'static SemanticAmbiguityProbeDescriptor> {
    LYRALANG_SEMANTIC_AMBIGUITY_PROBES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_malformed_object_descriptor(
    id: &str,
) -> Option<&'static SemanticMalformedObjectDescriptor> {
    LYRALANG_SEMANTIC_MALFORMED_OBJECTS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_adversarial_harness_descriptor(
    id: &str,
) -> Option<&'static SemanticAdversarialHarnessDescriptor> {
    LYRALANG_SEMANTIC_ADVERSARIAL_HARNESSES
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_collision_probe_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_COLLISION_PROBES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_ambiguity_probe_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_AMBIGUITY_PROBES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_malformed_object_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_MALFORMED_OBJECTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_adversarial_harness_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ADVERSARIAL_HARNESSES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_adversarial_all_case_ids() -> Vec<&'static str> {
    let mut ids = Vec::new();
    ids.extend(semantic_collision_probe_ids());
    ids.extend(semantic_ambiguity_probe_ids());
    ids.extend(semantic_malformed_object_ids());
    ids
}

pub fn canonical_collision_probe_signature(
    descriptor: &SemanticCollisionProbeDescriptor,
) -> String {
    format!("collision_probe:{}|target_family:{}|left_ref:{}|right_ref:{}|collision_domain:{}|guard:{}|expected_error:{}|fixture:{}|status:{}", descriptor.id, descriptor.target_family, descriptor.left_ref, descriptor.right_ref, descriptor.collision_domain, descriptor.guard, descriptor.expected_error, descriptor.fixture_path, descriptor.status)
}
pub fn canonical_ambiguity_probe_signature(
    descriptor: &SemanticAmbiguityProbeDescriptor,
) -> String {
    format!("ambiguity_probe:{}|target_family:{}|ambiguous_surface:{}|deterministic_resolution:{}|guard:{}|expected_error:{}|fixture:{}|status:{}", descriptor.id, descriptor.target_family, descriptor.ambiguous_surface, descriptor.deterministic_resolution, descriptor.guard, descriptor.expected_error, descriptor.fixture_path, descriptor.status)
}
pub fn canonical_malformed_object_signature(
    descriptor: &SemanticMalformedObjectDescriptor,
) -> String {
    format!("malformed_object:{}|target_family:{}|object_ref:{}|malformed_field:{}|rejection_law:{}|expected_error:{}|fixture:{}|status:{}", descriptor.id, descriptor.target_family, descriptor.object_ref, descriptor.malformed_field, descriptor.rejection_law, descriptor.expected_error, descriptor.fixture_path, descriptor.status)
}
pub fn canonical_semantic_adversarial_harness_signature(
    descriptor: &SemanticAdversarialHarnessDescriptor,
) -> String {
    format!(
        "harness:{}|validator:{}|case_ids:{}|coverage:{}|receipt_ref:{}|status:{}",
        descriptor.id,
        descriptor.validator,
        descriptor.case_ids,
        descriptor.coverage,
        descriptor.receipt_ref,
        descriptor.status
    )
}

pub fn collision_probe_digest(id: &str) -> Result<String, SemanticAdversarialCorpusError> {
    let descriptor = semantic_collision_probe_descriptor(id).ok_or_else(|| {
        SemanticAdversarialCorpusError::UnknownCollisionProbe { id: id.to_string() }
    })?;
    Ok(stable_hash_label(
        "lyra.p01.semantic_adversarial_corpus.collision_probe",
        &canonical_collision_probe_signature(descriptor),
    ))
}
pub fn ambiguity_probe_digest(id: &str) -> Result<String, SemanticAdversarialCorpusError> {
    let descriptor = semantic_ambiguity_probe_descriptor(id).ok_or_else(|| {
        SemanticAdversarialCorpusError::UnknownAmbiguityProbe { id: id.to_string() }
    })?;
    Ok(stable_hash_label(
        "lyra.p01.semantic_adversarial_corpus.ambiguity_probe",
        &canonical_ambiguity_probe_signature(descriptor),
    ))
}
pub fn malformed_object_digest(id: &str) -> Result<String, SemanticAdversarialCorpusError> {
    let descriptor = semantic_malformed_object_descriptor(id).ok_or_else(|| {
        SemanticAdversarialCorpusError::UnknownMalformedObject { id: id.to_string() }
    })?;
    Ok(stable_hash_label(
        "lyra.p01.semantic_adversarial_corpus.malformed_object",
        &canonical_malformed_object_signature(descriptor),
    ))
}
pub fn semantic_adversarial_harness_digest(
    id: &str,
) -> Result<String, SemanticAdversarialCorpusError> {
    let descriptor = semantic_adversarial_harness_descriptor(id)
        .ok_or_else(|| SemanticAdversarialCorpusError::UnknownHarness { id: id.to_string() })?;
    Ok(stable_hash_label(
        "lyra.p01.semantic_adversarial_corpus.harness",
        &canonical_semantic_adversarial_harness_signature(descriptor),
    ))
}

pub fn canonical_semantic_adversarial_corpus_registry_signature() -> String {
    let mut lines = Vec::new();
    for item in LYRALANG_SEMANTIC_COLLISION_PROBES {
        lines.push(canonical_collision_probe_signature(item));
    }
    for item in LYRALANG_SEMANTIC_AMBIGUITY_PROBES {
        lines.push(canonical_ambiguity_probe_signature(item));
    }
    for item in LYRALANG_SEMANTIC_MALFORMED_OBJECTS {
        lines.push(canonical_malformed_object_signature(item));
    }
    for item in LYRALANG_SEMANTIC_ADVERSARIAL_HARNESSES {
        lines.push(canonical_semantic_adversarial_harness_signature(item));
    }
    lines.sort();
    lines.join("\n")
}

pub fn canonical_semantic_adversarial_corpus_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_adversarial_corpus.registry",
        &canonical_semantic_adversarial_corpus_registry_signature(),
    )
}
