use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticCoreSerializationFamilyDescriptor {
    pub family: &'static str,
    pub owner_root: &'static str,
    pub serializer: &'static str,
    pub hash_domain: &'static str,
    pub source_registry: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticCoreSerializationDescriptor {
    pub family: &'static str,
    pub id: &'static str,
    pub object_ref: &'static str,
    pub owner_root: &'static str,
    pub serializer: &'static str,
    pub hash_domain: &'static str,
    pub comparison_key: &'static str,
    pub source_registry: &'static str,
    pub payload_signature: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticSerializationHashingError {
    UnknownFamily { family: String },
    UnknownObjectRef { object_ref: String },
    ObjectFamilyMismatch { family: String, object_ref: String },
}

pub const LYRALANG_SEMANTIC_CORE_SERIALIZATION_FAMILIES:
    &[SemanticCoreSerializationFamilyDescriptor] = &[
    SemanticCoreSerializationFamilyDescriptor {
        family: "semantic_atom",
        owner_root: "lyralang",
        serializer: "canonical_atom_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom",
        source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "core_ir",
        owner_root: "lyralang",
        serializer: "canonical_core_ir_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.core_ir",
        source_registry: "LYRALANG_CORE_IR_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "semantic_object",
        owner_root: "mixed_owner_root",
        serializer: "canonical_semantic_object_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_object",
        source_registry: "LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "semantic_identity",
        owner_root: "k0",
        serializer: "canonical_semantic_identity_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_identity",
        source_registry: "LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "reference_literal",
        owner_root: "lyralang",
        serializer: "canonical_reference_literal_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.reference_literal",
        source_registry: "LYRALANG_REFERENCE_LITERAL_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "reference_composition",
        owner_root: "lyralang",
        serializer: "canonical_reference_composition_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.reference_composition",
        source_registry: "LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "reference_eval_seed",
        owner_root: "lyralang",
        serializer: "canonical_reference_eval_seed_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.reference_eval_seed",
        source_registry: "LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "symbolic_equality_rule",
        owner_root: "lyralang",
        serializer: "canonical_symbolic_equality_rule_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equality_rule",
        source_registry: "LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "symbolic_equivalence_class",
        owner_root: "lyralang",
        serializer: "canonical_symbolic_equivalence_class_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equivalence_class",
        source_registry: "LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "symbolic_normalization_case",
        owner_root: "lyralang",
        serializer: "canonical_symbolic_normalization_case_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_normalization_case",
        source_registry: "LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "symbolic_substitution_case",
        owner_root: "lyralang",
        serializer: "canonical_symbolic_substitution_case_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_substitution_case",
        source_registry: "LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "error_object",
        owner_root: "lyralang",
        serializer: "canonical_error_object_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.error_object",
        source_registry: "LYRALANG_ERROR_OBJECT_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "challenge_object",
        owner_root: "lyralang",
        serializer: "canonical_challenge_object_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.challenge_object",
        source_registry: "LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "evidence_object",
        owner_root: "lyralang",
        serializer: "canonical_evidence_object_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.evidence_object",
        source_registry: "LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS",
    },
    SemanticCoreSerializationFamilyDescriptor {
        family: "object_link",
        owner_root: "lyralang",
        serializer: "canonical_object_link_signature",
        hash_domain: "lyra.p01.semantic_serialization_hashing.object_link",
        source_registry: "LYRALANG_OBJECT_LINK_DESCRIPTORS",
    },
];

pub const LYRALANG_SEMANTIC_CORE_SERIALIZATION_DESCRIPTORS: &[SemanticCoreSerializationDescriptor] = &[
    SemanticCoreSerializationDescriptor { family: "challenge_object", id: "challenge_capability", object_ref: "challenge_object:challenge_capability", owner_root: "lyralang", serializer: "canonical_challenge_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.challenge_object", comparison_key: "challenge_object:challenge_capability", source_registry: "LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS", payload_signature: "challenge_object:challenge_capability|target:capability_denied|challenger:runtime|claim_ref:capability_claim|counter_evidence_ref:evidence_capability_policy|adjudication_law:policy_receipt_required|digest:fnv1a128:cfb94a02c56e3c763ba1d84f5cc3da5c|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "challenge_object", id: "challenge_parse_error", object_ref: "challenge_object:challenge_parse_error", owner_root: "lyralang", serializer: "canonical_challenge_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.challenge_object", comparison_key: "challenge_object:challenge_parse_error", source_registry: "LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS", payload_signature: "challenge_object:challenge_parse_error|target:parse_missing_token|challenger:operator|claim_ref:parser_reject|counter_evidence_ref:evidence_parser_replay|adjudication_law:replay_fixture_required|digest:fnv1a128:41374589fd123b39ebd71efb48d87ece|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "challenge_object", id: "challenge_proof", object_ref: "challenge_object:challenge_proof", owner_root: "lyralang", serializer: "canonical_challenge_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.challenge_object", comparison_key: "challenge_object:challenge_proof", source_registry: "LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS", payload_signature: "challenge_object:challenge_proof|target:proof_obligation_unmet|challenger:proof|claim_ref:proof_claim|counter_evidence_ref:evidence_proof_bundle|adjudication_law:proof_bundle_required|digest:fnv1a128:d23e6518cbe98d8505533e3bbff03deb|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "challenge_object", id: "challenge_receipt", object_ref: "challenge_object:challenge_receipt", owner_root: "lyralang", serializer: "canonical_challenge_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.challenge_object", comparison_key: "challenge_object:challenge_receipt", source_registry: "LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS", payload_signature: "challenge_object:challenge_receipt|target:receipt_mismatch|challenger:receipt|claim_ref:receipt_claim|counter_evidence_ref:evidence_receipt_chain|adjudication_law:receipt_replay_required|digest:fnv1a128:188fc5b72c444fc75143ffe9ccb41a8f|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "challenge_object", id: "challenge_type_effect", object_ref: "challenge_object:challenge_type_effect", owner_root: "lyralang", serializer: "canonical_challenge_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.challenge_object", comparison_key: "challenge_object:challenge_type_effect", source_registry: "LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS", payload_signature: "challenge_object:challenge_type_effect|target:type_effect_violation|challenger:checker|claim_ref:effect_claim|counter_evidence_ref:evidence_type_trace|adjudication_law:typed_trace_required|digest:fnv1a128:7371ba49415125b239dbb28524f72474|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "core_ir", id: "binary_ir", object_ref: "core_ir:binary_ir", owner_root: "lyralang", serializer: "canonical_core_ir_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.core_ir", comparison_key: "core_ir:binary_ir", source_registry: "LYRALANG_CORE_IR_DESCRIPTORS", payload_signature: "ir_form:binary_ir|medium:binary|version:ir_v1|header:LYRAIR01|extension:lyrairb|encoding:length_prefixed_big_endian|canonicalization:canonical_binary_frame|round_trip:binary_to_text_to_binary_identity|upgrade:explicit_version_edge_only" },
    SemanticCoreSerializationDescriptor { family: "core_ir", id: "text_ir", object_ref: "core_ir:text_ir", owner_root: "lyralang", serializer: "canonical_core_ir_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.core_ir", comparison_key: "core_ir:text_ir", source_registry: "LYRALANG_CORE_IR_DESCRIPTORS", payload_signature: "ir_form:text_ir|medium:text|version:ir_v1|header:LYRA_CORE_IR_TEXT_V1|extension:lyrair|encoding:utf8_canonical_lines|canonicalization:sorted_key_value_lines|round_trip:text_to_binary_to_text_identity|upgrade:explicit_version_edge_only" },
    SemanticCoreSerializationDescriptor { family: "error_object", id: "capability_denied", object_ref: "error_object:capability_denied", owner_root: "lyralang", serializer: "canonical_error_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.error_object", comparison_key: "error_object:capability_denied", source_registry: "LYRALANG_ERROR_OBJECT_DESCRIPTORS", payload_signature: "error_object:capability_denied|severity:reject|domain:runtime|subject:capability.fs.write|message:capability gate denied unproven write authority|evidence_ref:evidence_capability_policy|digest:fnv1a128:689646b8f77e7773dc83bc1971a91f97|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "error_object", id: "parse_missing_token", object_ref: "error_object:parse_missing_token", owner_root: "lyralang", serializer: "canonical_error_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.error_object", comparison_key: "error_object:parse_missing_token", source_registry: "LYRALANG_ERROR_OBJECT_DESCRIPTORS", payload_signature: "error_object:parse_missing_token|severity:reject|domain:parser|subject:parser.token|message:parser reached declared eof without required token|evidence_ref:evidence_parser_replay|digest:fnv1a128:5899be27dd72af3e9902cedb7ed37a34|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "error_object", id: "proof_obligation_unmet", object_ref: "error_object:proof_obligation_unmet", owner_root: "lyralang", serializer: "canonical_error_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.error_object", comparison_key: "error_object:proof_obligation_unmet", source_registry: "LYRALANG_ERROR_OBJECT_DESCRIPTORS", payload_signature: "error_object:proof_obligation_unmet|severity:reject|domain:proof|subject:obligation.normalization|message:normalization witness obligation has no proof row|evidence_ref:evidence_proof_bundle|digest:fnv1a128:fb04b537222a8a0088af2da47487ffb9|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "error_object", id: "receipt_mismatch", object_ref: "error_object:receipt_mismatch", owner_root: "lyralang", serializer: "canonical_error_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.error_object", comparison_key: "error_object:receipt_mismatch", source_registry: "LYRALANG_ERROR_OBJECT_DESCRIPTORS", payload_signature: "error_object:receipt_mismatch|severity:reject|domain:receipt|subject:receipt.chain|message:receipt replay hash differs from canonical preimage|evidence_ref:evidence_receipt_chain|digest:fnv1a128:16f5bcd1415d05760d6a3b89d4a28daf|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "error_object", id: "type_effect_violation", object_ref: "error_object:type_effect_violation", owner_root: "lyralang", serializer: "canonical_error_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.error_object", comparison_key: "error_object:type_effect_violation", source_registry: "LYRALANG_ERROR_OBJECT_DESCRIPTORS", payload_signature: "error_object:type_effect_violation|severity:reject|domain:checker|subject:effect.capability|message:declared effect exceeds admitted capability|evidence_ref:evidence_type_trace|digest:fnv1a128:b7f6b983ef57860837b8e7079f3962e9|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "evidence_object", id: "evidence_capability_policy", object_ref: "evidence_object:evidence_capability_policy", owner_root: "lyralang", serializer: "canonical_evidence_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.evidence_object", comparison_key: "evidence_object:evidence_capability_policy", source_registry: "LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS", payload_signature: "evidence_object:evidence_capability_policy|kind:policy|source:fixtures/p01/error_challenge_evidence_inputs/capability_policy.lyra|payload_digest:fnv1a128:20a86b4ca912fc90a7e0df351b13af85|witness:capability_policy_witness|digest:fnv1a128:79cfbb4057b126b2ef56829db0382acf|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "evidence_object", id: "evidence_parser_replay", object_ref: "evidence_object:evidence_parser_replay", owner_root: "lyralang", serializer: "canonical_evidence_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.evidence_object", comparison_key: "evidence_object:evidence_parser_replay", source_registry: "LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS", payload_signature: "evidence_object:evidence_parser_replay|kind:trace|source:fixtures/p01/error_challenge_evidence_inputs/parser_trace.lyra|payload_digest:fnv1a128:c231564589ea67eb86d866dda06ec3a9|witness:parser_replay_witness|digest:fnv1a128:7886f70a24a12f1448a39cf29c1ee9cb|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "evidence_object", id: "evidence_proof_bundle", object_ref: "evidence_object:evidence_proof_bundle", owner_root: "lyralang", serializer: "canonical_evidence_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.evidence_object", comparison_key: "evidence_object:evidence_proof_bundle", source_registry: "LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS", payload_signature: "evidence_object:evidence_proof_bundle|kind:proof_bundle|source:fixtures/p01/error_challenge_evidence_inputs/proof_bundle.lyra|payload_digest:fnv1a128:8a46544b8a7b8cdb70cf34d85a05cdfb|witness:proof_obligation_witness|digest:fnv1a128:adbe54e2baaff601ecb8ed749f46db35|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "evidence_object", id: "evidence_receipt_chain", object_ref: "evidence_object:evidence_receipt_chain", owner_root: "lyralang", serializer: "canonical_evidence_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.evidence_object", comparison_key: "evidence_object:evidence_receipt_chain", source_registry: "LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS", payload_signature: "evidence_object:evidence_receipt_chain|kind:receipt_chain|source:fixtures/p01/error_challenge_evidence_inputs/receipt_chain.lyra|payload_digest:fnv1a128:9691602f160155be8466b376a744bbea|witness:receipt_replay_witness|digest:fnv1a128:de2168b44cb07734098454b5b11225fd|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "evidence_object", id: "evidence_type_trace", object_ref: "evidence_object:evidence_type_trace", owner_root: "lyralang", serializer: "canonical_evidence_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.evidence_object", comparison_key: "evidence_object:evidence_type_trace", source_registry: "LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS", payload_signature: "evidence_object:evidence_type_trace|kind:trace|source:fixtures/p01/error_challenge_evidence_inputs/type_trace.lyra|payload_digest:fnv1a128:69ae7f8bc16cf05361c511a55d4d5656|witness:checker_effect_witness|digest:fnv1a128:de2141149ec1e8a146eb0f1e5fa0c46c|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "object_link", id: "challenge_parse_countered", object_ref: "object_link:challenge_parse_countered", owner_root: "lyralang", serializer: "canonical_object_link_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.object_link", comparison_key: "object_link:challenge_parse_countered", source_registry: "LYRALANG_OBJECT_LINK_DESCRIPTORS", payload_signature: "object_link:challenge_parse_countered|from:challenge_parse_error|relation:countered_by|to:evidence_parser_replay|law:challenge_counter_evidence_bound|digest:fnv1a128:97b130a349c6802b93271e74a63f107c|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "object_link", id: "challenge_parse_targets", object_ref: "object_link:challenge_parse_targets", owner_root: "lyralang", serializer: "canonical_object_link_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.object_link", comparison_key: "object_link:challenge_parse_targets", source_registry: "LYRALANG_OBJECT_LINK_DESCRIPTORS", payload_signature: "object_link:challenge_parse_targets|from:challenge_parse_error|relation:challenges|to:parse_missing_token|law:challenge_target_bound|digest:fnv1a128:bdc7f0a836c1eb00b0161141a807cf02|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "object_link", id: "error_parse_supported", object_ref: "object_link:error_parse_supported", owner_root: "lyralang", serializer: "canonical_object_link_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.object_link", comparison_key: "object_link:error_parse_supported", source_registry: "LYRALANG_OBJECT_LINK_DESCRIPTORS", payload_signature: "object_link:error_parse_supported|from:parse_missing_token|relation:supported_by|to:evidence_parser_replay|law:error_evidence_ref_matches|digest:fnv1a128:f531c81308fa1584175a44bab05968ae|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "object_link", id: "error_type_supported", object_ref: "object_link:error_type_supported", owner_root: "lyralang", serializer: "canonical_object_link_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.object_link", comparison_key: "object_link:error_type_supported", source_registry: "LYRALANG_OBJECT_LINK_DESCRIPTORS", payload_signature: "object_link:error_type_supported|from:type_effect_violation|relation:supported_by|to:evidence_type_trace|law:error_evidence_ref_matches|digest:fnv1a128:fa7192e8e3a21e09c3c72bccd3f822cc|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "object_link", id: "receipt_error_supported", object_ref: "object_link:receipt_error_supported", owner_root: "lyralang", serializer: "canonical_object_link_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.object_link", comparison_key: "object_link:receipt_error_supported", source_registry: "LYRALANG_OBJECT_LINK_DESCRIPTORS", payload_signature: "object_link:receipt_error_supported|from:receipt_mismatch|relation:supported_by|to:evidence_receipt_chain|law:receipt_replay_bound|digest:fnv1a128:8b3ede87667cdabc80abb2bdd19b6944|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "reference_composition", id: "apply", object_ref: "reference_composition:apply", owner_root: "lyralang", serializer: "canonical_reference_composition_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_composition", comparison_key: "reference_composition:apply", source_registry: "LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS", payload_signature: "reference_composition:apply|operator:compose.apply|arity:two|input_order:function_then_argument|output:application_normal_form|law:symbolic_application_seed" },
    SemanticCoreSerializationDescriptor { family: "reference_composition", id: "bind", object_ref: "reference_composition:bind", owner_root: "lyralang", serializer: "canonical_reference_composition_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_composition", comparison_key: "reference_composition:bind", source_registry: "LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS", payload_signature: "reference_composition:bind|operator:compose.bind|arity:three|input_order:symbol_value_body|output:bound_scope_normal_form|law:binding_scope_seed" },
    SemanticCoreSerializationDescriptor { family: "reference_composition", id: "identity", object_ref: "reference_composition:identity", owner_root: "lyralang", serializer: "canonical_reference_composition_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_composition", comparison_key: "reference_composition:identity", source_registry: "LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS", payload_signature: "reference_composition:identity|operator:compose.identity|arity:one|input_order:single|output:same_normal_form|law:identity_preserves_normal_form" },
    SemanticCoreSerializationDescriptor { family: "reference_composition", id: "list", object_ref: "reference_composition:list", owner_root: "lyralang", serializer: "canonical_reference_composition_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_composition", comparison_key: "reference_composition:list", source_registry: "LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS", payload_signature: "reference_composition:list|operator:compose.list|arity:many|input_order:index_ascending|output:list_normal_form|law:list_order_preserved" },
    SemanticCoreSerializationDescriptor { family: "reference_composition", id: "pair", object_ref: "reference_composition:pair", owner_root: "lyralang", serializer: "canonical_reference_composition_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_composition", comparison_key: "reference_composition:pair", source_registry: "LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS", payload_signature: "reference_composition:pair|operator:compose.pair|arity:two|input_order:left_then_right|output:pair_normal_form|law:pair_structural_evaluation" },
    SemanticCoreSerializationDescriptor { family: "reference_composition", id: "proof_step", object_ref: "reference_composition:proof_step", owner_root: "lyralang", serializer: "canonical_reference_composition_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_composition", comparison_key: "reference_composition:proof_step", source_registry: "LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS", payload_signature: "reference_composition:proof_step|operator:compose.proof_step|arity:two|input_order:claim_then_witness|output:proof_step_normal_form|law:proof_step_receipt_seed" },
    SemanticCoreSerializationDescriptor { family: "reference_composition", id: "record", object_ref: "reference_composition:record", owner_root: "lyralang", serializer: "canonical_reference_composition_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_composition", comparison_key: "reference_composition:record", source_registry: "LYRALANG_REFERENCE_COMPOSITION_DESCRIPTORS", payload_signature: "reference_composition:record|operator:compose.record|arity:many_named|input_order:key_sorted_ascii|output:record_normal_form|law:record_key_order_canonical" },
    SemanticCoreSerializationDescriptor { family: "reference_eval_seed", id: "apply_symbolic", object_ref: "reference_eval_seed:apply_symbolic", owner_root: "lyralang", serializer: "canonical_reference_eval_seed_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_eval_seed", comparison_key: "reference_eval_seed:apply_symbolic", source_registry: "LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS", payload_signature: "reference_eval_seed:apply_symbolic|input:apply(literal.symbol.core,literal.unit)|reduction:function_then_argument|expected:apply(symbol.lyra.core,unit)|law:symbolic_application_seed" },
    SemanticCoreSerializationDescriptor { family: "reference_eval_seed", id: "bind_scope", object_ref: "reference_eval_seed:bind_scope", owner_root: "lyralang", serializer: "canonical_reference_eval_seed_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_eval_seed", comparison_key: "reference_eval_seed:bind_scope", source_registry: "LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS", payload_signature: "reference_eval_seed:bind_scope|input:bind(x,literal.integer.1,literal.symbol.core)|reduction:symbol_value_body|expected:bind(x=integer.1;symbol.lyra.core)|law:binding_scope_seed" },
    SemanticCoreSerializationDescriptor { family: "reference_eval_seed", id: "list_order", object_ref: "reference_eval_seed:list_order", owner_root: "lyralang", serializer: "canonical_reference_eval_seed_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_eval_seed", comparison_key: "reference_eval_seed:list_order", source_registry: "LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS", payload_signature: "reference_eval_seed:list_order|input:list(literal.bool.false,literal.bool.true)|reduction:index_ascending|expected:list(bool.false,bool.true)|law:list_order_preserved" },
    SemanticCoreSerializationDescriptor { family: "reference_eval_seed", id: "literal_self", object_ref: "reference_eval_seed:literal_self", owner_root: "lyralang", serializer: "canonical_reference_eval_seed_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_eval_seed", comparison_key: "reference_eval_seed:literal_self", source_registry: "LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS", payload_signature: "reference_eval_seed:literal_self|input:literal.integer.1|reduction:literal_self|expected:integer.1|law:literal_identity" },
    SemanticCoreSerializationDescriptor { family: "reference_eval_seed", id: "pair_structural", object_ref: "reference_eval_seed:pair_structural", owner_root: "lyralang", serializer: "canonical_reference_eval_seed_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_eval_seed", comparison_key: "reference_eval_seed:pair_structural", source_registry: "LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS", payload_signature: "reference_eval_seed:pair_structural|input:pair(literal.integer.0,literal.integer.1)|reduction:left_then_right|expected:pair(integer.0,integer.1)|law:pair_structural_evaluation" },
    SemanticCoreSerializationDescriptor { family: "reference_eval_seed", id: "proof_step", object_ref: "reference_eval_seed:proof_step", owner_root: "lyralang", serializer: "canonical_reference_eval_seed_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_eval_seed", comparison_key: "reference_eval_seed:proof_step", source_registry: "LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS", payload_signature: "reference_eval_seed:proof_step|input:proof_step(claim.core,literal.unit)|reduction:claim_then_witness|expected:proof_step(claim.core;unit)|law:proof_step_receipt_seed" },
    SemanticCoreSerializationDescriptor { family: "reference_eval_seed", id: "record_key_sort", object_ref: "reference_eval_seed:record_key_sort", owner_root: "lyralang", serializer: "canonical_reference_eval_seed_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_eval_seed", comparison_key: "reference_eval_seed:record_key_sort", source_registry: "LYRALANG_REFERENCE_EVAL_SEED_DESCRIPTORS", payload_signature: "reference_eval_seed:record_key_sort|input:record(b:literal.integer.1,a:literal.integer.0)|reduction:key_sorted_ascii|expected:record(a=integer.0,b=integer.1)|law:record_key_order_canonical" },
    SemanticCoreSerializationDescriptor { family: "reference_literal", id: "bool_false", object_ref: "reference_literal:bool_false", owner_root: "lyralang", serializer: "canonical_reference_literal_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_literal", comparison_key: "reference_literal:bool_false", source_registry: "LYRALANG_REFERENCE_LITERAL_DESCRIPTORS", payload_signature: "reference_literal:bool_false|atom:value|canonical:literal.bool.false|normal:bool.false|evaluator:literal_self|proof:literal_identity" },
    SemanticCoreSerializationDescriptor { family: "reference_literal", id: "bool_true", object_ref: "reference_literal:bool_true", owner_root: "lyralang", serializer: "canonical_reference_literal_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_literal", comparison_key: "reference_literal:bool_true", source_registry: "LYRALANG_REFERENCE_LITERAL_DESCRIPTORS", payload_signature: "reference_literal:bool_true|atom:value|canonical:literal.bool.true|normal:bool.true|evaluator:literal_self|proof:literal_identity" },
    SemanticCoreSerializationDescriptor { family: "reference_literal", id: "integer_one", object_ref: "reference_literal:integer_one", owner_root: "lyralang", serializer: "canonical_reference_literal_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_literal", comparison_key: "reference_literal:integer_one", source_registry: "LYRALANG_REFERENCE_LITERAL_DESCRIPTORS", payload_signature: "reference_literal:integer_one|atom:value|canonical:literal.integer.1|normal:integer.1|evaluator:literal_self|proof:literal_identity" },
    SemanticCoreSerializationDescriptor { family: "reference_literal", id: "integer_zero", object_ref: "reference_literal:integer_zero", owner_root: "lyralang", serializer: "canonical_reference_literal_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_literal", comparison_key: "reference_literal:integer_zero", source_registry: "LYRALANG_REFERENCE_LITERAL_DESCRIPTORS", payload_signature: "reference_literal:integer_zero|atom:value|canonical:literal.integer.0|normal:integer.0|evaluator:literal_self|proof:literal_identity" },
    SemanticCoreSerializationDescriptor { family: "reference_literal", id: "symbol_core", object_ref: "reference_literal:symbol_core", owner_root: "lyralang", serializer: "canonical_reference_literal_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_literal", comparison_key: "reference_literal:symbol_core", source_registry: "LYRALANG_REFERENCE_LITERAL_DESCRIPTORS", payload_signature: "reference_literal:symbol_core|atom:symbol|canonical:literal.symbol.core|normal:symbol.lyra.core|evaluator:literal_self|proof:symbol_identity" },
    SemanticCoreSerializationDescriptor { family: "reference_literal", id: "text_empty", object_ref: "reference_literal:text_empty", owner_root: "lyralang", serializer: "canonical_reference_literal_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_literal", comparison_key: "reference_literal:text_empty", source_registry: "LYRALANG_REFERENCE_LITERAL_DESCRIPTORS", payload_signature: "reference_literal:text_empty|atom:value|canonical:literal.text.empty|normal:text.empty|evaluator:literal_self|proof:literal_identity" },
    SemanticCoreSerializationDescriptor { family: "reference_literal", id: "unit", object_ref: "reference_literal:unit", owner_root: "lyralang", serializer: "canonical_reference_literal_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.reference_literal", comparison_key: "reference_literal:unit", source_registry: "LYRALANG_REFERENCE_LITERAL_DESCRIPTORS", payload_signature: "reference_literal:unit|atom:value|canonical:literal.unit|normal:unit|evaluator:literal_self|proof:literal_identity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "capability", object_ref: "semantic_atom:capability", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:capability", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:capability|kind:capability|canonical:lyra.capability|identity:capability_scope_signature|equality:capability_scope_identity|normalization:capability_scope_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "effect", object_ref: "semantic_atom:effect", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:effect", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:effect|kind:effect|canonical:lyra.effect|identity:effect_scope_signature|equality:effect_scope_identity|normalization:effect_order_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "law", object_ref: "semantic_atom:law", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:law", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:law|kind:law|canonical:lyra.law|identity:law_scope_signature|equality:law_scope_identity|normalization:law_scope_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "proof", object_ref: "semantic_atom:proof", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:proof", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:proof|kind:proof|canonical:lyra.proof|identity:proof_term_signature|equality:proof_term_identity|normalization:proof_term_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "receipt", object_ref: "semantic_atom:receipt", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:receipt", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:receipt|kind:receipt|canonical:lyra.receipt|identity:receipt_chain_signature|equality:receipt_chain_identity|normalization:receipt_chain_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "resource", object_ref: "semantic_atom:resource", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:resource", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:resource|kind:resource|canonical:lyra.resource|identity:resource_scope_signature|equality:resource_scope_identity|normalization:resource_scope_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "symbol", object_ref: "semantic_atom:symbol", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:symbol", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:symbol|kind:symbol|canonical:lyra.symbol|identity:kind_id_version|equality:canonical_byte_identity|normalization:symbol_path_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "type", object_ref: "semantic_atom:type", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:type", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:type|kind:type|canonical:lyra.type|identity:type_constructor_signature|equality:normalized_type_identity|normalization:type_constructor_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_atom", id: "value", object_ref: "semantic_atom:value", owner_root: "lyralang", serializer: "canonical_atom_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_atom", comparison_key: "semantic_atom:value", source_registry: "LYRALANG_CORE_ATOM_DESCRIPTORS", payload_signature: "atom:value|kind:value|canonical:lyra.value|identity:literal_kind_payload|equality:literal_payload_identity|normalization:literal_form_canonicalization|serialization:text_binary_ir_parity" },
    SemanticCoreSerializationDescriptor { family: "semantic_identity", id: "artifact", object_ref: "semantic_identity:artifact", owner_root: "k0", serializer: "canonical_semantic_identity_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_identity", comparison_key: "semantic_identity:artifact", source_registry: "LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS", payload_signature: "semantic_identity:artifact|scope:artifact_manifest_table|material:owner_path_bytes_contract|canonicalizer:canonical_artifact_manifest_entry|digest:fnv1a128_labeled|collision:reject_equal_digest_unequal_preimage|consumer:packaging_receipts_distribution" },
    SemanticCoreSerializationDescriptor { family: "semantic_identity", id: "declaration", object_ref: "semantic_identity:declaration", owner_root: "k0", serializer: "canonical_semantic_identity_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_identity", comparison_key: "semantic_identity:declaration", source_registry: "LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS", payload_signature: "semantic_identity:declaration|scope:module_declaration_table|material:owner_symbol_type_effect|canonicalizer:sorted_declaration_fields|digest:fnv1a128_labeled|collision:reject_equal_digest_unequal_preimage|consumer:loader_checker_ir" },
    SemanticCoreSerializationDescriptor { family: "semantic_identity", id: "rewrite", object_ref: "semantic_identity:rewrite", owner_root: "k0", serializer: "canonical_semantic_identity_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_identity", comparison_key: "semantic_identity:rewrite", source_registry: "LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS", payload_signature: "semantic_identity:rewrite|scope:normalization_rewrite_table|material:lhs_rhs_law_guard|canonicalizer:normalized_rewrite_tuple|digest:fnv1a128_labeled|collision:reject_equal_digest_unequal_preimage|consumer:normalizer_evaluator_proof" },
    SemanticCoreSerializationDescriptor { family: "semantic_identity", id: "symbol", object_ref: "semantic_identity:symbol", owner_root: "k0", serializer: "canonical_semantic_identity_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_identity", comparison_key: "semantic_identity:symbol", source_registry: "LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS", payload_signature: "semantic_identity:symbol|scope:global_symbol_table|material:symbol_path|canonicalizer:lower_ascii_symbolic_path|digest:fnv1a128_labeled|collision:reject_equal_digest_unequal_preimage|consumer:lexer_parser_checker" },
    SemanticCoreSerializationDescriptor { family: "semantic_identity", id: "witness_row", object_ref: "semantic_identity:witness_row", owner_root: "k0", serializer: "canonical_semantic_identity_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_identity", comparison_key: "semantic_identity:witness_row", source_registry: "LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS", payload_signature: "semantic_identity:witness_row|scope:trace_witness_table|material:trace_index_claim_receipt|canonicalizer:monotone_witness_row_tuple|digest:fnv1a128_labeled|collision:reject_equal_digest_unequal_preimage|consumer:trace_replay_proof" },
    SemanticCoreSerializationDescriptor { family: "semantic_object", id: "module", object_ref: "semantic_object:module", owner_root: "lyralang", serializer: "canonical_semantic_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_object", comparison_key: "semantic_object:module", source_registry: "LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS", payload_signature: "semantic_object:module|atom:symbol|owner:lyralang|kind:module_object|path:lyra.object.module|parent:package|ir:text_ir|serialization:semantic_object_text_ir|comparison:canonical_object_identity|lifecycle:declared_loaded_checked" },
    SemanticCoreSerializationDescriptor { family: "semantic_object", id: "package", object_ref: "semantic_object:package", owner_root: "interfaces", serializer: "canonical_semantic_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_object", comparison_key: "semantic_object:package", source_registry: "LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS", payload_signature: "semantic_object:package|atom:resource|owner:interfaces|kind:package_object|path:lyra.object.package|parent:program|ir:text_ir|serialization:semantic_object_text_ir|comparison:canonical_object_identity|lifecycle:declared_loaded_checked" },
    SemanticCoreSerializationDescriptor { family: "semantic_object", id: "plan", object_ref: "semantic_object:plan", owner_root: "lyralang", serializer: "canonical_semantic_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_object", comparison_key: "semantic_object:plan", source_registry: "LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS", payload_signature: "semantic_object:plan|atom:effect|owner:lyralang|kind:plan_object|path:lyra.object.plan|parent:world|ir:text_ir|serialization:semantic_object_text_ir|comparison:canonical_object_identity|lifecycle:declared_loaded_checked" },
    SemanticCoreSerializationDescriptor { family: "semantic_object", id: "program", object_ref: "semantic_object:program", owner_root: "lyralang", serializer: "canonical_semantic_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_object", comparison_key: "semantic_object:program", source_registry: "LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS", payload_signature: "semantic_object:program|atom:symbol|owner:lyralang|kind:program_object|path:lyra.object.program|parent:world|ir:text_ir|serialization:semantic_object_text_ir|comparison:canonical_object_identity|lifecycle:declared_loaded_checked" },
    SemanticCoreSerializationDescriptor { family: "semantic_object", id: "proof", object_ref: "semantic_object:proof", owner_root: "interfaces", serializer: "canonical_semantic_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_object", comparison_key: "semantic_object:proof", source_registry: "LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS", payload_signature: "semantic_object:proof|atom:proof|owner:interfaces|kind:proof_object|path:lyra.object.proof|parent:trace|ir:text_ir|serialization:semantic_object_text_ir|comparison:canonical_object_identity|lifecycle:declared_loaded_checked" },
    SemanticCoreSerializationDescriptor { family: "semantic_object", id: "trace", object_ref: "semantic_object:trace", owner_root: "k0", serializer: "canonical_semantic_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_object", comparison_key: "semantic_object:trace", source_registry: "LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS", payload_signature: "semantic_object:trace|atom:receipt|owner:k0|kind:trace_object|path:lyra.object.trace|parent:plan|ir:text_ir|serialization:semantic_object_text_ir|comparison:canonical_object_identity|lifecycle:declared_loaded_checked" },
    SemanticCoreSerializationDescriptor { family: "semantic_object", id: "world", object_ref: "semantic_object:world", owner_root: "k0", serializer: "canonical_semantic_object_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.semantic_object", comparison_key: "semantic_object:world", source_registry: "LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS", payload_signature: "semantic_object:world|atom:resource|owner:k0|kind:world_object|path:lyra.object.world|parent:none|ir:text_ir|serialization:semantic_object_text_ir|comparison:canonical_object_identity|lifecycle:declared_loaded_checked" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equality_rule", id: "alpha_equivalent", object_ref: "symbolic_equality_rule:alpha_equivalent", owner_root: "lyralang", serializer: "canonical_symbolic_equality_rule_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equality_rule", comparison_key: "symbolic_equality_rule:alpha_equivalent", source_registry: "LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS", payload_signature: "equality_rule:alpha_equivalent|domain:binder|relation:bind_name_irrelevant|law:binder_scope_canonicalized|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equality_rule", id: "reflexive", object_ref: "symbolic_equality_rule:reflexive", owner_root: "lyralang", serializer: "canonical_symbolic_equality_rule_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equality_rule", comparison_key: "symbolic_equality_rule:reflexive", source_registry: "LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS", payload_signature: "equality_rule:reflexive|domain:term|relation:equal(term,term)|law:normal_form_identity|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equality_rule", id: "structural", object_ref: "symbolic_equality_rule:structural", owner_root: "lyralang", serializer: "canonical_symbolic_equality_rule_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equality_rule", comparison_key: "symbolic_equality_rule:structural", source_registry: "LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS", payload_signature: "equality_rule:structural|domain:composition|relation:equal_children_imply_equal_parent|law:canonical_child_order|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equality_rule", id: "symmetric", object_ref: "symbolic_equality_rule:symmetric", owner_root: "lyralang", serializer: "canonical_symbolic_equality_rule_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equality_rule", comparison_key: "symbolic_equality_rule:symmetric", source_registry: "LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS", payload_signature: "equality_rule:symmetric|domain:term|relation:equal(a,b)->equal(b,a)|law:normal_form_equality_symmetric|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equality_rule", id: "transitive", object_ref: "symbolic_equality_rule:transitive", owner_root: "lyralang", serializer: "canonical_symbolic_equality_rule_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equality_rule", comparison_key: "symbolic_equality_rule:transitive", source_registry: "LYRALANG_SYMBOLIC_EQUALITY_RULE_DESCRIPTORS", payload_signature: "equality_rule:transitive|domain:term|relation:equal(a,b)&equal(b,c)->equal(a,c)|law:normal_form_equality_transitive|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equivalence_class", id: "alpha_bind_class", object_ref: "symbolic_equivalence_class:alpha_bind_class", owner_root: "lyralang", serializer: "canonical_symbolic_equivalence_class_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equivalence_class", comparison_key: "symbolic_equivalence_class:alpha_bind_class", source_registry: "LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS", payload_signature: "equivalence_class:alpha_bind_class|members:bind(x=unit in symbol.x),bind(y=unit in symbol.y)|canonical:bind($0=unit in symbol.$0)|normalizer:alpha_binder_canonicalization|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equivalence_class", id: "bool_true_singleton", object_ref: "symbolic_equivalence_class:bool_true_singleton", owner_root: "lyralang", serializer: "canonical_symbolic_equivalence_class_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equivalence_class", comparison_key: "symbolic_equivalence_class:bool_true_singleton", source_registry: "LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS", payload_signature: "equivalence_class:bool_true_singleton|members:bool.true|canonical:bool.true|normalizer:literal_identity|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equivalence_class", id: "list_child_normal_class", object_ref: "symbolic_equivalence_class:list_child_normal_class", owner_root: "lyralang", serializer: "canonical_symbolic_equivalence_class_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equivalence_class", comparison_key: "symbolic_equivalence_class:list_child_normal_class", source_registry: "LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS", payload_signature: "equivalence_class:list_child_normal_class|members:list(record(b=integer.1,a=integer.0)),list(record(a=integer.0,b=integer.1))|canonical:list(record(a=integer.0,b=integer.1))|normalizer:recursive_child_normalization|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equivalence_class", id: "record_order_class", object_ref: "symbolic_equivalence_class:record_order_class", owner_root: "lyralang", serializer: "canonical_symbolic_equivalence_class_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equivalence_class", comparison_key: "symbolic_equivalence_class:record_order_class", source_registry: "LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS", payload_signature: "equivalence_class:record_order_class|members:record(b=integer.1,a=integer.0),record(a=integer.0,b=integer.1)|canonical:record(a=integer.0,b=integer.1)|normalizer:record_key_sort|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_equivalence_class", id: "unit_singleton", object_ref: "symbolic_equivalence_class:unit_singleton", owner_root: "lyralang", serializer: "canonical_symbolic_equivalence_class_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_equivalence_class", comparison_key: "symbolic_equivalence_class:unit_singleton", source_registry: "LYRALANG_SYMBOLIC_EQUIVALENCE_CLASS_DESCRIPTORS", payload_signature: "equivalence_class:unit_singleton|members:unit|canonical:unit|normalizer:literal_identity|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_normalization_case", id: "apply_child_normal", object_ref: "symbolic_normalization_case:apply_child_normal", owner_root: "lyralang", serializer: "canonical_symbolic_normalization_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_normalization_case", comparison_key: "symbolic_normalization_case:apply_child_normal", source_registry: "LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS", payload_signature: "normalization:apply_child_normal|input:apply(symbol.f,record(b=integer.1,a=integer.0))|output:apply(symbol.f,record(a=integer.0,b=integer.1))|law:normalize_before_symbolic_apply|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_normalization_case", id: "bind_alpha_normal", object_ref: "symbolic_normalization_case:bind_alpha_normal", owner_root: "lyralang", serializer: "canonical_symbolic_normalization_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_normalization_case", comparison_key: "symbolic_normalization_case:bind_alpha_normal", source_registry: "LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS", payload_signature: "normalization:bind_alpha_normal|input:bind(x=unit in symbol.x)|output:bind($0=unit in symbol.$0)|law:alpha_binder_canonicalization|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_normalization_case", id: "literal_unit_normal", object_ref: "symbolic_normalization_case:literal_unit_normal", owner_root: "lyralang", serializer: "canonical_symbolic_normalization_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_normalization_case", comparison_key: "symbolic_normalization_case:literal_unit_normal", source_registry: "LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS", payload_signature: "normalization:literal_unit_normal|input:unit|output:unit|law:literal_identity|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_normalization_case", id: "nested_record_normal", object_ref: "symbolic_normalization_case:nested_record_normal", owner_root: "lyralang", serializer: "canonical_symbolic_normalization_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_normalization_case", comparison_key: "symbolic_normalization_case:nested_record_normal", source_registry: "LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS", payload_signature: "normalization:nested_record_normal|input:list(record(b=integer.1,a=integer.0))|output:list(record(a=integer.0,b=integer.1))|law:recursive_child_normalization|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_normalization_case", id: "record_key_sort_normal", object_ref: "symbolic_normalization_case:record_key_sort_normal", owner_root: "lyralang", serializer: "canonical_symbolic_normalization_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_normalization_case", comparison_key: "symbolic_normalization_case:record_key_sort_normal", source_registry: "LYRALANG_SYMBOLIC_NORMALIZATION_CASE_DESCRIPTORS", payload_signature: "normalization:record_key_sort_normal|input:record(b=integer.1,a=integer.0)|output:record(a=integer.0,b=integer.1)|law:record_key_sort|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_substitution_case", id: "binder_shadow_guard", object_ref: "symbolic_substitution_case:binder_shadow_guard", owner_root: "lyralang", serializer: "canonical_symbolic_substitution_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_substitution_case", comparison_key: "symbolic_substitution_case:binder_shadow_guard", source_registry: "LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS", payload_signature: "substitution:binder_shadow_guard|target:x|replacement:integer.1|scope:bind(x=unit in symbol.x)|expected:bind($0=unit in symbol.$0)|law:binder_shadow_blocks_substitution|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_substitution_case", id: "capture_rejection", object_ref: "symbolic_substitution_case:capture_rejection", owner_root: "lyralang", serializer: "canonical_symbolic_substitution_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_substitution_case", comparison_key: "symbolic_substitution_case:capture_rejection", source_registry: "LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS", payload_signature: "substitution:capture_rejection|target:x|replacement:symbol.y|scope:bind(y=unit in symbol.x)|expected:reject_capture_risk|law:capture_avoidance_required|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_substitution_case", id: "substitute_pair", object_ref: "symbolic_substitution_case:substitute_pair", owner_root: "lyralang", serializer: "canonical_symbolic_substitution_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_substitution_case", comparison_key: "symbolic_substitution_case:substitute_pair", source_registry: "LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS", payload_signature: "substitution:substitute_pair|target:x|replacement:unit|scope:pair(symbol.x,bool.true)|expected:pair(unit,bool.true)|law:structural_substitution|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_substitution_case", id: "substitute_record", object_ref: "symbolic_substitution_case:substitute_record", owner_root: "lyralang", serializer: "canonical_symbolic_substitution_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_substitution_case", comparison_key: "symbolic_substitution_case:substitute_record", source_registry: "LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS", payload_signature: "substitution:substitute_record|target:x|replacement:integer.0|scope:record(b=symbol.x,a=unit)|expected:record(a=unit,b=integer.0)|law:substitute_then_normalize|status:artifact_emitted" },
    SemanticCoreSerializationDescriptor { family: "symbolic_substitution_case", id: "substitute_symbol", object_ref: "symbolic_substitution_case:substitute_symbol", owner_root: "lyralang", serializer: "canonical_symbolic_substitution_case_signature", hash_domain: "lyra.p01.semantic_serialization_hashing.symbolic_substitution_case", comparison_key: "symbolic_substitution_case:substitute_symbol", source_registry: "LYRALANG_SYMBOLIC_SUBSTITUTION_CASE_DESCRIPTORS", payload_signature: "substitution:substitute_symbol|target:x|replacement:integer.1|scope:symbol.x|expected:integer.1|law:free_symbol_replacement|status:artifact_emitted" },
];

pub fn semantic_core_serialization_family_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_CORE_SERIALIZATION_FAMILIES
        .iter()
        .map(|item| item.family)
        .collect()
}

pub fn semantic_core_object_refs() -> Vec<&'static str> {
    let mut refs: Vec<&'static str> = LYRALANG_SEMANTIC_CORE_SERIALIZATION_DESCRIPTORS
        .iter()
        .map(|item| item.object_ref)
        .collect();
    refs.sort();
    refs
}

pub fn semantic_core_serialization_family_descriptor(
    family: &str,
) -> Option<SemanticCoreSerializationFamilyDescriptor> {
    LYRALANG_SEMANTIC_CORE_SERIALIZATION_FAMILIES
        .iter()
        .copied()
        .find(|item| item.family == family)
}

pub fn semantic_core_serialization_descriptor(
    object_ref: &str,
) -> Option<SemanticCoreSerializationDescriptor> {
    LYRALANG_SEMANTIC_CORE_SERIALIZATION_DESCRIPTORS
        .iter()
        .copied()
        .find(|item| item.object_ref == object_ref)
}

pub fn semantic_core_serialization_descriptor_by_parts(
    family: &str,
    id: &str,
) -> Option<SemanticCoreSerializationDescriptor> {
    LYRALANG_SEMANTIC_CORE_SERIALIZATION_DESCRIPTORS
        .iter()
        .copied()
        .find(|item| item.family == family && item.id == id)
}

pub fn canonical_semantic_core_object_serialization_text(
    descriptor: SemanticCoreSerializationDescriptor,
) -> String {
    let payload_hash = semantic_core_payload_hash(descriptor);
    let mut lines = vec![
        format!("comparison_key={}", descriptor.comparison_key),
        format!("family={}", descriptor.family),
        format!("hash_domain={}", descriptor.hash_domain),
        format!("id={}", descriptor.id),
        format!("object_ref={}", descriptor.object_ref),
        format!("owner={}", descriptor.owner_root),
        format!("payload_hash={}", payload_hash),
        format!("serializer={}", descriptor.serializer),
        format!("source_registry={}", descriptor.source_registry),
    ];
    lines.sort();
    let mut output = String::from("LYRA-SEMANTIC-CORE-OBJECT-SERIALIZATION v1\n");
    for line in lines {
        output.push_str(&line);
        output.push('\n');
    }
    output
}

pub fn semantic_core_payload_hash(descriptor: SemanticCoreSerializationDescriptor) -> String {
    stable_hash_label(descriptor.hash_domain, descriptor.payload_signature)
}

pub fn semantic_core_record_hash(descriptor: SemanticCoreSerializationDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_serialization_hashing.record",
        &canonical_semantic_core_object_serialization_text(descriptor),
    )
}

pub fn semantic_core_object_hashes(
    object_ref: &str,
) -> Result<(String, String), SemanticSerializationHashingError> {
    let descriptor = semantic_core_serialization_descriptor(object_ref).ok_or_else(|| {
        SemanticSerializationHashingError::UnknownObjectRef {
            object_ref: object_ref.to_string(),
        }
    })?;
    Ok((
        semantic_core_payload_hash(descriptor),
        semantic_core_record_hash(descriptor),
    ))
}

pub fn canonical_semantic_core_serialization_registry_signature() -> String {
    let mut rows: Vec<String> = LYRALANG_SEMANTIC_CORE_SERIALIZATION_DESCRIPTORS.iter().copied().map(|descriptor| {
        format!(
            "object_ref:{}|family:{}|serializer:{}|hash_domain:{}|payload_hash:{}|record_hash:{}|comparison_key:{}",
            descriptor.object_ref,
            descriptor.family,
            descriptor.serializer,
            descriptor.hash_domain,
            semantic_core_payload_hash(descriptor),
            semantic_core_record_hash(descriptor),
            descriptor.comparison_key,
        )
    }).collect();
    rows.sort();
    rows.join("\n")
}

pub fn canonical_semantic_core_serialization_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_serialization_hashing.registry",
        &canonical_semantic_core_serialization_registry_signature(),
    )
}

pub fn semantic_core_serialization_round_trip_identity(
    object_ref: &str,
) -> Result<bool, SemanticSerializationHashingError> {
    let descriptor = semantic_core_serialization_descriptor(object_ref).ok_or_else(|| {
        SemanticSerializationHashingError::UnknownObjectRef {
            object_ref: object_ref.to_string(),
        }
    })?;
    let first = canonical_semantic_core_object_serialization_text(descriptor);
    let first_hash = semantic_core_record_hash(descriptor);
    let second = canonical_semantic_core_object_serialization_text(descriptor);
    let second_hash = semantic_core_record_hash(descriptor);
    Ok(first == second && first_hash == second_hash)
}

pub fn semantic_core_family_object_count(
    family: &str,
) -> Result<usize, SemanticSerializationHashingError> {
    if semantic_core_serialization_family_descriptor(family).is_none() {
        return Err(SemanticSerializationHashingError::UnknownFamily {
            family: family.to_string(),
        });
    }
    Ok(LYRALANG_SEMANTIC_CORE_SERIALIZATION_DESCRIPTORS
        .iter()
        .filter(|item| item.family == family)
        .count())
}
