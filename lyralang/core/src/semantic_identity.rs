use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticIdentityDescriptor {
    pub domain: &'static str,
    pub scope: &'static str,
    pub material: &'static str,
    pub canonicalizer: &'static str,
    pub digest: &'static str,
    pub collision_law: &'static str,
    pub consumer: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticIdentityError {
    EmptyField {
        field: &'static str,
    },
    InvalidSymbolicName {
        field: &'static str,
        value: String,
    },
    UnknownDomain {
        domain: String,
    },
    DescriptorDrift {
        field: &'static str,
        expected: String,
        actual: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIdentityMaterial {
    pub domain: String,
    pub id: String,
    pub owner: String,
    pub payload: String,
    pub normalization: String,
}

impl SemanticIdentityMaterial {
    pub fn new(
        domain: &str,
        id: &str,
        owner: &str,
        payload: &str,
        normalization: &str,
    ) -> Result<Self, SemanticIdentityError> {
        validate_symbolic_name("domain", domain)?;
        validate_symbolic_name("id", id)?;
        validate_symbolic_name("owner", owner)?;
        validate_symbolic_name("payload", payload)?;
        validate_symbolic_name("normalization", normalization)?;
        let Some(descriptor) = semantic_identity_descriptor(domain) else {
            return Err(SemanticIdentityError::UnknownDomain {
                domain: domain.to_string(),
            });
        };
        if descriptor.canonicalizer != normalization {
            return Err(SemanticIdentityError::DescriptorDrift {
                field: "normalization",
                expected: descriptor.canonicalizer.to_string(),
                actual: normalization.to_string(),
            });
        }
        Ok(Self {
            domain: domain.to_string(),
            id: id.to_string(),
            owner: owner.to_string(),
            payload: payload.to_string(),
            normalization: normalization.to_string(),
        })
    }
}

pub const LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS: &[SemanticIdentityDescriptor] = &[
    SemanticIdentityDescriptor {
        domain: "symbol",
        scope: "global_symbol_table",
        material: "symbol_path",
        canonicalizer: "lower_ascii_symbolic_path",
        digest: "fnv1a128_labeled",
        collision_law: "reject_equal_digest_unequal_preimage",
        consumer: "lexer_parser_checker",
    },
    SemanticIdentityDescriptor {
        domain: "declaration",
        scope: "module_declaration_table",
        material: "owner_symbol_type_effect",
        canonicalizer: "sorted_declaration_fields",
        digest: "fnv1a128_labeled",
        collision_law: "reject_equal_digest_unequal_preimage",
        consumer: "loader_checker_ir",
    },
    SemanticIdentityDescriptor {
        domain: "rewrite",
        scope: "normalization_rewrite_table",
        material: "lhs_rhs_law_guard",
        canonicalizer: "normalized_rewrite_tuple",
        digest: "fnv1a128_labeled",
        collision_law: "reject_equal_digest_unequal_preimage",
        consumer: "normalizer_evaluator_proof",
    },
    SemanticIdentityDescriptor {
        domain: "witness_row",
        scope: "trace_witness_table",
        material: "trace_index_claim_receipt",
        canonicalizer: "monotone_witness_row_tuple",
        digest: "fnv1a128_labeled",
        collision_law: "reject_equal_digest_unequal_preimage",
        consumer: "trace_replay_proof",
    },
    SemanticIdentityDescriptor {
        domain: "artifact",
        scope: "artifact_manifest_table",
        material: "owner_path_bytes_contract",
        canonicalizer: "canonical_artifact_manifest_entry",
        digest: "fnv1a128_labeled",
        collision_law: "reject_equal_digest_unequal_preimage",
        consumer: "packaging_receipts_distribution",
    },
];

pub fn semantic_identity_domains() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS
        .iter()
        .map(|item| item.domain)
        .collect()
}
pub fn semantic_identity_descriptor(domain: &str) -> Option<SemanticIdentityDescriptor> {
    LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS
        .iter()
        .copied()
        .find(|item| item.domain == domain)
}
pub fn is_semantic_identity_domain(domain: &str) -> bool {
    semantic_identity_descriptor(domain).is_some()
}

pub fn canonical_semantic_identity_signature(descriptor: SemanticIdentityDescriptor) -> String {
    format!("semantic_identity:{}|scope:{}|material:{}|canonicalizer:{}|digest:{}|collision:{}|consumer:{}", descriptor.domain, descriptor.scope, descriptor.material, descriptor.canonicalizer, descriptor.digest, descriptor.collision_law, descriptor.consumer)
}

pub fn canonical_semantic_identity_registry_signature() -> String {
    let mut signatures: Vec<String> = LYRALANG_SEMANTIC_IDENTITY_DESCRIPTORS
        .iter()
        .copied()
        .map(canonical_semantic_identity_signature)
        .collect();
    signatures.sort();
    signatures.join("\n")
}

pub fn canonical_identity_preimage(
    material: &SemanticIdentityMaterial,
) -> Result<String, SemanticIdentityError> {
    let descriptor = semantic_identity_descriptor(&material.domain).ok_or_else(|| {
        SemanticIdentityError::UnknownDomain {
            domain: material.domain.clone(),
        }
    })?;
    if descriptor.canonicalizer != material.normalization {
        return Err(SemanticIdentityError::DescriptorDrift {
            field: "normalization",
            expected: descriptor.canonicalizer.to_string(),
            actual: material.normalization.clone(),
        });
    }
    validate_symbolic_name("id", &material.id)?;
    validate_symbolic_name("owner", &material.owner)?;
    validate_symbolic_name("payload", &material.payload)?;
    Ok(format!("LYRA-SEMANTIC-IDENTITY-PREIMAGE v1\ndomain={}\nid={}\nowner={}\npayload={}\nnormalization={}\nmaterial={}\nscope={}\n", material.domain, material.id, material.owner, material.payload, material.normalization, descriptor.material, descriptor.scope))
}

pub fn canonical_identity_digest(
    material: &SemanticIdentityMaterial,
) -> Result<String, SemanticIdentityError> {
    let preimage = canonical_identity_preimage(material)?;
    Ok(stable_hash_label(
        &format!("lyra.p01.semantic_identity.{}", material.domain),
        &preimage,
    ))
}

pub fn canonical_identity_digest_from_parts(
    domain: &str,
    id: &str,
    owner: &str,
    payload: &str,
    normalization: &str,
) -> Result<String, SemanticIdentityError> {
    let material = SemanticIdentityMaterial::new(domain, id, owner, payload, normalization)?;
    canonical_identity_digest(&material)
}

fn validate_symbolic_name(field: &'static str, value: &str) -> Result<(), SemanticIdentityError> {
    if value.is_empty() {
        return Err(SemanticIdentityError::EmptyField { field });
    }
    let valid = value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'.'
    });
    if valid {
        Ok(())
    } else {
        Err(SemanticIdentityError::InvalidSymbolicName {
            field,
            value: value.to_string(),
        })
    }
}
