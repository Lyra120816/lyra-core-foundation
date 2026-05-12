use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticObjectDescriptor {
    pub id: &'static str,
    pub primary_atom: &'static str,
    pub owner_root: &'static str,
    pub object_kind: &'static str,
    pub canonical_path: &'static str,
    pub required_parent: &'static str,
    pub ir_form: &'static str,
    pub serialization_law: &'static str,
    pub comparison_law: &'static str,
    pub lifecycle_law: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticObjectError {
    EmptyField { field: &'static str },
    InvalidSymbolicName { field: &'static str, value: String },
    DuplicateField { field: String },
    UnknownObjectKind { kind: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectRecord {
    pub object_id: String,
    pub kind: String,
    pub name: String,
    pub version: String,
    pub fields: BTreeMap<String, String>,
}

impl SemanticObjectRecord {
    pub fn new(
        object_id: &str,
        kind: &str,
        name: &str,
        version: &str,
    ) -> Result<Self, SemanticObjectError> {
        validate_symbolic_name("object_id", object_id)?;
        validate_symbolic_name("kind", kind)?;
        validate_symbolic_name("name", name)?;
        validate_symbolic_name("version", version)?;
        if semantic_object_descriptor(kind).is_none() {
            return Err(SemanticObjectError::UnknownObjectKind {
                kind: kind.to_string(),
            });
        }
        Ok(Self {
            object_id: object_id.to_string(),
            kind: kind.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            fields: BTreeMap::new(),
        })
    }

    pub fn with_field(mut self, key: &str, value: &str) -> Result<Self, SemanticObjectError> {
        validate_symbolic_name("field_key", key)?;
        validate_symbolic_name("field_value", value)?;
        if self
            .fields
            .insert(key.to_string(), value.to_string())
            .is_some()
        {
            return Err(SemanticObjectError::DuplicateField {
                field: key.to_string(),
            });
        }
        Ok(self)
    }
}

pub const LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS: &[SemanticObjectDescriptor] = &[
    SemanticObjectDescriptor {
        id: "module",
        primary_atom: "symbol",
        owner_root: "lyralang",
        object_kind: "module_object",
        canonical_path: "lyra.object.module",
        required_parent: "package",
        ir_form: "text_ir",
        serialization_law: "semantic_object_text_ir",
        comparison_law: "canonical_object_identity",
        lifecycle_law: "declared_loaded_checked",
    },
    SemanticObjectDescriptor {
        id: "package",
        primary_atom: "resource",
        owner_root: "interfaces",
        object_kind: "package_object",
        canonical_path: "lyra.object.package",
        required_parent: "program",
        ir_form: "text_ir",
        serialization_law: "semantic_object_text_ir",
        comparison_law: "canonical_object_identity",
        lifecycle_law: "declared_loaded_checked",
    },
    SemanticObjectDescriptor {
        id: "program",
        primary_atom: "symbol",
        owner_root: "lyralang",
        object_kind: "program_object",
        canonical_path: "lyra.object.program",
        required_parent: "world",
        ir_form: "text_ir",
        serialization_law: "semantic_object_text_ir",
        comparison_law: "canonical_object_identity",
        lifecycle_law: "declared_loaded_checked",
    },
    SemanticObjectDescriptor {
        id: "world",
        primary_atom: "resource",
        owner_root: "k0",
        object_kind: "world_object",
        canonical_path: "lyra.object.world",
        required_parent: "none",
        ir_form: "text_ir",
        serialization_law: "semantic_object_text_ir",
        comparison_law: "canonical_object_identity",
        lifecycle_law: "declared_loaded_checked",
    },
    SemanticObjectDescriptor {
        id: "plan",
        primary_atom: "effect",
        owner_root: "lyralang",
        object_kind: "plan_object",
        canonical_path: "lyra.object.plan",
        required_parent: "world",
        ir_form: "text_ir",
        serialization_law: "semantic_object_text_ir",
        comparison_law: "canonical_object_identity",
        lifecycle_law: "declared_loaded_checked",
    },
    SemanticObjectDescriptor {
        id: "trace",
        primary_atom: "receipt",
        owner_root: "k0",
        object_kind: "trace_object",
        canonical_path: "lyra.object.trace",
        required_parent: "plan",
        ir_form: "text_ir",
        serialization_law: "semantic_object_text_ir",
        comparison_law: "canonical_object_identity",
        lifecycle_law: "declared_loaded_checked",
    },
    SemanticObjectDescriptor {
        id: "proof",
        primary_atom: "proof",
        owner_root: "interfaces",
        object_kind: "proof_object",
        canonical_path: "lyra.object.proof",
        required_parent: "trace",
        ir_form: "text_ir",
        serialization_law: "semantic_object_text_ir",
        comparison_law: "canonical_object_identity",
        lifecycle_law: "declared_loaded_checked",
    },
];

pub fn semantic_object_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_object_descriptor(id: &str) -> Option<SemanticObjectDescriptor> {
    LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS
        .iter()
        .copied()
        .find(|item| item.id == id)
}

pub fn canonical_semantic_object_signature(descriptor: SemanticObjectDescriptor) -> String {
    format!(
        "semantic_object:{}|atom:{}|owner:{}|kind:{}|path:{}|parent:{}|ir:{}|serialization:{}|comparison:{}|lifecycle:{}",
        descriptor.id, descriptor.primary_atom, descriptor.owner_root, descriptor.object_kind, descriptor.canonical_path, descriptor.required_parent, descriptor.ir_form, descriptor.serialization_law, descriptor.comparison_law, descriptor.lifecycle_law,
    )
}

pub fn canonical_semantic_object_registry_signature() -> String {
    let mut signatures: Vec<String> = LYRALANG_SEMANTIC_OBJECT_DESCRIPTORS
        .iter()
        .copied()
        .map(canonical_semantic_object_signature)
        .collect();
    signatures.sort();
    signatures.join("\n")
}

pub fn canonical_semantic_object_text(
    record: &SemanticObjectRecord,
) -> Result<String, SemanticObjectError> {
    validate_symbolic_name("object_id", &record.object_id)?;
    validate_symbolic_name("kind", &record.kind)?;
    validate_symbolic_name("name", &record.name)?;
    validate_symbolic_name("version", &record.version)?;
    let descriptor = semantic_object_descriptor(&record.kind).ok_or_else(|| {
        SemanticObjectError::UnknownObjectKind {
            kind: record.kind.clone(),
        }
    })?;
    let mut lines = Vec::new();
    lines.push("LYRA-SEMANTIC-OBJECT v1".to_string());
    lines.push(format!("atom={}", descriptor.primary_atom));
    lines.push(format!("comparison={}", descriptor.comparison_law));
    lines.push(format!("id={}", record.object_id));
    lines.push(format!("ir={}", descriptor.ir_form));
    lines.push(format!("kind={}", record.kind));
    lines.push(format!("lifecycle={}", descriptor.lifecycle_law));
    lines.push(format!("name={}", record.name));
    lines.push(format!("owner={}", descriptor.owner_root));
    lines.push(format!("parent={}", descriptor.required_parent));
    lines.push(format!("path={}", descriptor.canonical_path));
    lines.push(format!("serialization={}", descriptor.serialization_law));
    lines.push(format!("version={}", record.version));
    for (key, value) in &record.fields {
        validate_symbolic_name("field_key", key)?;
        validate_symbolic_name("field_value", value)?;
        lines.push(format!("field:{key}={value}"));
    }
    let header = lines.remove(0);
    lines.sort();
    let mut output = String::new();
    output.push_str(&header);
    output.push('\n');
    for line in lines {
        output.push_str(&line);
        output.push('\n');
    }
    Ok(output)
}

pub fn semantic_object_parent_kind(id: &str) -> Option<&'static str> {
    semantic_object_descriptor(id).map(|item| item.required_parent)
}
pub fn is_semantic_object_id(id: &str) -> bool {
    semantic_object_descriptor(id).is_some()
}

fn validate_symbolic_name(field: &'static str, value: &str) -> Result<(), SemanticObjectError> {
    if value.is_empty() {
        return Err(SemanticObjectError::EmptyField { field });
    }
    let valid = value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'.'
    });
    if valid {
        Ok(())
    } else {
        Err(SemanticObjectError::InvalidSymbolicName {
            field,
            value: value.to_string(),
        })
    }
}
