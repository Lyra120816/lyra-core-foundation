use crate::k0_hash::stable_hash_label;
use crate::lyralang_semantic_atoms::{core_atom_descriptor, core_atom_ids};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticAtomReferenceLibraryDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub registry_ref: &'static str,
    pub atom_ids: &'static str,
    pub library_path: &'static str,
    pub export_contract: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticAtomReferenceExampleDescriptor {
    pub id: &'static str,
    pub library_ref: &'static str,
    pub atom_id: &'static str,
    pub example_path: &'static str,
    pub expected_inspection: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticAtomInspectionToolDescriptor {
    pub id: &'static str,
    pub binary: &'static str,
    pub input_contract: &'static str,
    pub output_contract: &'static str,
    pub fixture_path: &'static str,
    pub receipt_ref: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticAtomReferenceGateDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub law: &'static str,
    pub evidence: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticAtomReferenceError {
    UnknownLibrary { id: String },
    UnknownExample { id: String },
    UnknownTool { id: String },
    UnknownGate { id: String },
    UnknownAtom { id: String },
}

pub const SEMANTIC_ATOM_REFERENCE_ATOM_IDS: &str =
    "symbol,value,type,effect,capability,proof,receipt,resource,law";

pub const LYRALANG_SEMANTIC_ATOM_REFERENCE_LIBRARIES: &[SemanticAtomReferenceLibraryDescriptor] = &[
    SemanticAtomReferenceLibraryDescriptor {
        id: "core_atom_reference_library",
        owner_root: "lyralang",
        registry_ref: "lyralang_semantic_atoms",
        atom_ids: SEMANTIC_ATOM_REFERENCE_ATOM_IDS,
        library_path: "lyralang/core/reference/semantic_atoms.v1.lyra",
        export_contract: "all_core_atoms_exported_without_semantic_rewrite",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceLibraryDescriptor {
        id: "interface_atom_reference_library",
        owner_root: "interfaces",
        registry_ref: "interfaces_p01_semantic_atom_model",
        atom_ids: SEMANTIC_ATOM_REFERENCE_ATOM_IDS,
        library_path: "interfaces/p01/contracts/semantic_atom_reference.v1.lyra",
        export_contract: "interface_contract_binds_reference_registry",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceLibraryDescriptor {
        id: "operator_atom_reference_library",
        owner_root: "products",
        registry_ref: "products_p01_semantic_atom_reference_surface",
        atom_ids: SEMANTIC_ATOM_REFERENCE_ATOM_IDS,
        library_path: "products/p01/semantic_atom_reference_inspection_surface.lyra",
        export_contract: "operator_surface_exposes_reference_only_views",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_ATOM_REFERENCE_EXAMPLES: &[SemanticAtomReferenceExampleDescriptor] = &[
    SemanticAtomReferenceExampleDescriptor {
        id: "symbol_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "symbol",
        example_path: "examples/p01/semantic_atom_reference/symbol_atom_reference.lyra",
        expected_inspection: "atom_symbol_canonical_lyra_symbol",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceExampleDescriptor {
        id: "value_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "value",
        example_path: "examples/p01/semantic_atom_reference/value_atom_reference.lyra",
        expected_inspection: "atom_value_canonical_lyra_value",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceExampleDescriptor {
        id: "type_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "type",
        example_path: "examples/p01/semantic_atom_reference/type_atom_reference.lyra",
        expected_inspection: "atom_type_canonical_lyra_type",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceExampleDescriptor {
        id: "effect_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "effect",
        example_path: "examples/p01/semantic_atom_reference/effect_atom_reference.lyra",
        expected_inspection: "atom_effect_canonical_lyra_effect",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceExampleDescriptor {
        id: "capability_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "capability",
        example_path: "examples/p01/semantic_atom_reference/capability_atom_reference.lyra",
        expected_inspection: "atom_capability_canonical_lyra_capability",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceExampleDescriptor {
        id: "proof_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "proof",
        example_path: "examples/p01/semantic_atom_reference/proof_atom_reference.lyra",
        expected_inspection: "atom_proof_canonical_lyra_proof",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceExampleDescriptor {
        id: "receipt_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "receipt",
        example_path: "examples/p01/semantic_atom_reference/receipt_atom_reference.lyra",
        expected_inspection: "atom_receipt_canonical_lyra_receipt",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceExampleDescriptor {
        id: "resource_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "resource",
        example_path: "examples/p01/semantic_atom_reference/resource_atom_reference.lyra",
        expected_inspection: "atom_resource_canonical_lyra_resource",
        status: "artifact_emitted",
    },
    SemanticAtomReferenceExampleDescriptor {
        id: "law_atom_reference_example",
        library_ref: "core_atom_reference_library",
        atom_id: "law",
        example_path: "examples/p01/semantic_atom_reference/law_atom_reference.lyra",
        expected_inspection: "atom_law_canonical_lyra_law",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_ATOM_INSPECTION_TOOLS: &[SemanticAtomInspectionToolDescriptor] = &[
    SemanticAtomInspectionToolDescriptor {
        id: "semantic_atom_reference_cli",
        binary: "src/bin/lyra-p01-semantic-atom-reference-check.rs",
        input_contract: "LYRA-P01-SEMANTIC-ATOM-REFERENCE v1",
        output_contract: "LYRA-P01-RECEIPT v1",
        fixture_path:
            "fixtures/p01/semantic_atom_reference_inputs/valid_semantic_atom_reference.lyra",
        receipt_ref: "receipt_semantic_atom_reference",
        status: "execution_proven",
    },
    SemanticAtomInspectionToolDescriptor {
        id: "semantic_atom_reference_index",
        binary: "lyralang/core/reference/semantic_atoms.v1.lyra",
        input_contract: "lyralang_semantic_atoms",
        output_contract: "all_atoms_sorted_by_id",
        fixture_path: "lyralang/core/reference/semantic_atoms.v1.lyra",
        receipt_ref: "receipt_semantic_atom_reference",
        status: "artifact_emitted",
    },
    SemanticAtomInspectionToolDescriptor {
        id: "semantic_atom_reference_product_surface",
        binary: "products/p01/semantic_atom_reference_inspection_surface.lyra",
        input_contract: "products_p01_semantic_atom_reference_surface",
        output_contract: "operator_inspection_without_semantic_mutation",
        fixture_path: "products/p01/semantic_atom_reference_inspection_surface.lyra",
        receipt_ref: "receipt_semantic_atom_reference",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_ATOM_REFERENCE_GATES: &[SemanticAtomReferenceGateDescriptor] = &[
    SemanticAtomReferenceGateDescriptor {
        id: "all_atoms_exported_gate",
        scope: "reference_libraries",
        law: "all_core_atoms_must_be_exported",
        evidence: "semantic_atom_reference_all_atoms_exported",
        status: "execution_proven",
    },
    SemanticAtomReferenceGateDescriptor {
        id: "examples_cover_atoms_gate",
        scope: "examples",
        law: "one_reference_example_per_core_atom",
        evidence: "semantic_atom_reference_examples_cover_all_atoms",
        status: "execution_proven",
    },
    SemanticAtomReferenceGateDescriptor {
        id: "inspection_is_read_only_gate",
        scope: "inspection_tools",
        law: "inspection_must_not_rewrite_semantics",
        evidence: "inspection_outputs_bind_canonical_atom_descriptor",
        status: "execution_proven",
    },
];

pub fn semantic_atom_reference_library_descriptor(
    id: &str,
) -> Option<&'static SemanticAtomReferenceLibraryDescriptor> {
    LYRALANG_SEMANTIC_ATOM_REFERENCE_LIBRARIES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_atom_reference_example_descriptor(
    id: &str,
) -> Option<&'static SemanticAtomReferenceExampleDescriptor> {
    LYRALANG_SEMANTIC_ATOM_REFERENCE_EXAMPLES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_atom_inspection_tool_descriptor(
    id: &str,
) -> Option<&'static SemanticAtomInspectionToolDescriptor> {
    LYRALANG_SEMANTIC_ATOM_INSPECTION_TOOLS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_atom_reference_gate_descriptor(
    id: &str,
) -> Option<&'static SemanticAtomReferenceGateDescriptor> {
    LYRALANG_SEMANTIC_ATOM_REFERENCE_GATES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_atom_reference_library_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ATOM_REFERENCE_LIBRARIES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_atom_reference_example_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ATOM_REFERENCE_EXAMPLES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_atom_inspection_tool_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ATOM_INSPECTION_TOOLS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_atom_reference_gate_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ATOM_REFERENCE_GATES
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_atom_reference_library_exports_atom(library_id: &str, atom_id: &str) -> bool {
    semantic_atom_reference_library_descriptor(library_id)
        .map(|library| {
            library.atom_ids.split(',').any(|item| item == atom_id)
                && core_atom_descriptor(atom_id).is_some()
        })
        .unwrap_or(false)
}

pub fn semantic_atom_reference_all_atoms_exported() -> bool {
    let mut exported = LYRALANG_SEMANTIC_ATOM_REFERENCE_LIBRARIES
        .iter()
        .flat_map(|library| library.atom_ids.split(','))
        .filter(|atom_id| core_atom_descriptor(atom_id).is_some())
        .collect::<Vec<_>>();
    exported.sort();
    exported.dedup();
    let mut required = core_atom_ids();
    required.sort();
    exported == required
}

pub fn semantic_atom_reference_examples_cover_all_atoms() -> bool {
    let mut example_atoms = LYRALANG_SEMANTIC_ATOM_REFERENCE_EXAMPLES
        .iter()
        .map(|example| example.atom_id)
        .collect::<Vec<_>>();
    example_atoms.sort();
    example_atoms.dedup();
    let mut required = core_atom_ids();
    required.sort();
    example_atoms == required
}

pub fn canonical_semantic_atom_reference_library_signature(
    descriptor: &SemanticAtomReferenceLibraryDescriptor,
) -> String {
    format!(
        "library:{}|owner:{}|registry:{}|atoms:{}|path:{}|export:{}|status:{}",
        descriptor.id,
        descriptor.owner_root,
        descriptor.registry_ref,
        descriptor.atom_ids,
        descriptor.library_path,
        descriptor.export_contract,
        descriptor.status
    )
}
pub fn canonical_semantic_atom_reference_example_signature(
    descriptor: &SemanticAtomReferenceExampleDescriptor,
) -> String {
    format!(
        "example:{}|library:{}|atom:{}|path:{}|expected:{}|status:{}",
        descriptor.id,
        descriptor.library_ref,
        descriptor.atom_id,
        descriptor.example_path,
        descriptor.expected_inspection,
        descriptor.status
    )
}
pub fn canonical_semantic_atom_inspection_tool_signature(
    descriptor: &SemanticAtomInspectionToolDescriptor,
) -> String {
    format!(
        "tool:{}|binary:{}|input:{}|output:{}|fixture:{}|receipt:{}|status:{}",
        descriptor.id,
        descriptor.binary,
        descriptor.input_contract,
        descriptor.output_contract,
        descriptor.fixture_path,
        descriptor.receipt_ref,
        descriptor.status
    )
}
pub fn canonical_semantic_atom_reference_gate_signature(
    descriptor: &SemanticAtomReferenceGateDescriptor,
) -> String {
    format!(
        "gate:{}|scope:{}|law:{}|evidence:{}|status:{}",
        descriptor.id, descriptor.scope, descriptor.law, descriptor.evidence, descriptor.status
    )
}

pub fn semantic_atom_reference_library_digest(
    id: &str,
) -> Result<String, SemanticAtomReferenceError> {
    let descriptor = semantic_atom_reference_library_descriptor(id)
        .ok_or_else(|| SemanticAtomReferenceError::UnknownLibrary { id: id.to_string() })?;
    Ok(stable_hash_label(
        "lyra.p01.semantic_atom_reference.library",
        &canonical_semantic_atom_reference_library_signature(descriptor),
    ))
}
pub fn semantic_atom_reference_example_digest(
    id: &str,
) -> Result<String, SemanticAtomReferenceError> {
    let descriptor = semantic_atom_reference_example_descriptor(id)
        .ok_or_else(|| SemanticAtomReferenceError::UnknownExample { id: id.to_string() })?;
    Ok(stable_hash_label(
        "lyra.p01.semantic_atom_reference.example",
        &canonical_semantic_atom_reference_example_signature(descriptor),
    ))
}
pub fn semantic_atom_inspection_tool_digest(
    id: &str,
) -> Result<String, SemanticAtomReferenceError> {
    let descriptor = semantic_atom_inspection_tool_descriptor(id)
        .ok_or_else(|| SemanticAtomReferenceError::UnknownTool { id: id.to_string() })?;
    Ok(stable_hash_label(
        "lyra.p01.semantic_atom_reference.tool",
        &canonical_semantic_atom_inspection_tool_signature(descriptor),
    ))
}
pub fn semantic_atom_reference_gate_digest(id: &str) -> Result<String, SemanticAtomReferenceError> {
    let descriptor = semantic_atom_reference_gate_descriptor(id)
        .ok_or_else(|| SemanticAtomReferenceError::UnknownGate { id: id.to_string() })?;
    Ok(stable_hash_label(
        "lyra.p01.semantic_atom_reference.gate",
        &canonical_semantic_atom_reference_gate_signature(descriptor),
    ))
}

pub fn canonical_semantic_atom_reference_registry_signature() -> String {
    let mut lines = Vec::new();
    for descriptor in LYRALANG_SEMANTIC_ATOM_REFERENCE_LIBRARIES {
        lines.push(canonical_semantic_atom_reference_library_signature(
            descriptor,
        ));
    }
    for descriptor in LYRALANG_SEMANTIC_ATOM_REFERENCE_EXAMPLES {
        lines.push(canonical_semantic_atom_reference_example_signature(
            descriptor,
        ));
    }
    for descriptor in LYRALANG_SEMANTIC_ATOM_INSPECTION_TOOLS {
        lines.push(canonical_semantic_atom_inspection_tool_signature(
            descriptor,
        ));
    }
    for descriptor in LYRALANG_SEMANTIC_ATOM_REFERENCE_GATES {
        lines.push(canonical_semantic_atom_reference_gate_signature(descriptor));
    }
    lines.sort();
    lines.join("\n")
}

pub fn canonical_semantic_atom_reference_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_atom_reference.registry",
        &canonical_semantic_atom_reference_registry_signature(),
    )
}
