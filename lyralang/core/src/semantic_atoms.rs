#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoreAtomDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub canonical_name: &'static str,
    pub identity_law: &'static str,
    pub equality_law: &'static str,
    pub normalization_law: &'static str,
    pub serialization_law: &'static str,
}

pub const LYRALANG_CORE_ATOM_DESCRIPTORS: &[CoreAtomDescriptor] = &[
    CoreAtomDescriptor {
        id: "symbol",
        kind: "symbol",
        canonical_name: "lyra.symbol",
        identity_law: "kind_id_version",
        equality_law: "canonical_byte_identity",
        normalization_law: "symbol_path_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
    CoreAtomDescriptor {
        id: "value",
        kind: "value",
        canonical_name: "lyra.value",
        identity_law: "literal_kind_payload",
        equality_law: "literal_payload_identity",
        normalization_law: "literal_form_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
    CoreAtomDescriptor {
        id: "type",
        kind: "type",
        canonical_name: "lyra.type",
        identity_law: "type_constructor_signature",
        equality_law: "normalized_type_identity",
        normalization_law: "type_constructor_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
    CoreAtomDescriptor {
        id: "effect",
        kind: "effect",
        canonical_name: "lyra.effect",
        identity_law: "effect_scope_signature",
        equality_law: "effect_scope_identity",
        normalization_law: "effect_order_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
    CoreAtomDescriptor {
        id: "capability",
        kind: "capability",
        canonical_name: "lyra.capability",
        identity_law: "capability_scope_signature",
        equality_law: "capability_scope_identity",
        normalization_law: "capability_scope_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
    CoreAtomDescriptor {
        id: "proof",
        kind: "proof",
        canonical_name: "lyra.proof",
        identity_law: "proof_term_signature",
        equality_law: "proof_term_identity",
        normalization_law: "proof_term_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
    CoreAtomDescriptor {
        id: "receipt",
        kind: "receipt",
        canonical_name: "lyra.receipt",
        identity_law: "receipt_chain_signature",
        equality_law: "receipt_chain_identity",
        normalization_law: "receipt_chain_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
    CoreAtomDescriptor {
        id: "resource",
        kind: "resource",
        canonical_name: "lyra.resource",
        identity_law: "resource_scope_signature",
        equality_law: "resource_scope_identity",
        normalization_law: "resource_scope_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
    CoreAtomDescriptor {
        id: "law",
        kind: "law",
        canonical_name: "lyra.law",
        identity_law: "law_scope_signature",
        equality_law: "law_scope_identity",
        normalization_law: "law_scope_canonicalization",
        serialization_law: "text_binary_ir_parity",
    },
];

pub fn core_atom_ids() -> Vec<&'static str> {
    LYRALANG_CORE_ATOM_DESCRIPTORS
        .iter()
        .map(|atom| atom.id)
        .collect()
}
pub fn core_atom_descriptor(id: &str) -> Option<CoreAtomDescriptor> {
    LYRALANG_CORE_ATOM_DESCRIPTORS
        .iter()
        .copied()
        .find(|atom| atom.id == id)
}
pub fn is_core_atom_id(id: &str) -> bool {
    core_atom_descriptor(id).is_some()
}

pub fn canonical_atom_signature(descriptor: CoreAtomDescriptor) -> String {
    format!(
        "atom:{}|kind:{}|canonical:{}|identity:{}|equality:{}|normalization:{}|serialization:{}",
        descriptor.id,
        descriptor.kind,
        descriptor.canonical_name,
        descriptor.identity_law,
        descriptor.equality_law,
        descriptor.normalization_law,
        descriptor.serialization_law,
    )
}

pub fn canonical_registry_signature() -> String {
    let mut signatures: Vec<String> = LYRALANG_CORE_ATOM_DESCRIPTORS
        .iter()
        .copied()
        .map(canonical_atom_signature)
        .collect();
    signatures.sort();
    signatures.join("\n")
}
