use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticProofFamilyArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub role: &'static str,
}

pub const LYRA_P01_SEMANTIC_PROOF_FAMILY_CARRIER: &str =
    "LYRA-P01-SEMANTIC-PROOF-FAMILY-CARRIER v1";

pub const LYRALANG_SEMANTIC_PROOF_FAMILY_ARTIFACTS: &[SemanticProofFamilyArtifactDescriptor] = &[
    SemanticProofFamilyArtifactDescriptor {
        id: "semantic_proof_family_model",
        owner_root: "interfaces",
        path: "interfaces/p01/src/semantic_proof_family_model.rs",
        role: "typed contract model",
    },
    SemanticProofFamilyArtifactDescriptor {
        id: "semantic_proof_family_validator",
        owner_root: "ops",
        path: "ops/p01/src/semantic_proof_family.rs",
        role: "offline validator",
    },
    SemanticProofFamilyArtifactDescriptor {
        id: "semantic_proof_family_report",
        owner_root: "k0",
        path: "k0/determinism/src/semantic_proof_family.rs",
        role: "canonical deterministic report",
    },
    SemanticProofFamilyArtifactDescriptor {
        id: "semantic_proof_family_surface",
        owner_root: "ops",
        path: "ops/p01/closure/semantic_proof_family_table.lyra",
        role: "emitted proof family table",
    },
    SemanticProofFamilyArtifactDescriptor {
        id: "semantic_proof_family_receipt",
        owner_root: "receipts",
        path: "receipts/p01/pass_0055_semantic_proof_family.receipt",
        role: "pass receipt binding",
    },
];

pub fn semantic_proof_family_artifact_ids() -> Vec<&'static str> {
    let mut ids = LYRALANG_SEMANTIC_PROOF_FAMILY_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    ids.sort();
    ids
}
pub fn semantic_proof_family_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_PROOF_FAMILY_ARTIFACTS.iter().all(|item| {
        !item.id.is_empty()
            && !item.owner_root.is_empty()
            && !item.path.is_empty()
            && !item.role.is_empty()
    })
}
pub fn semantic_proof_family_registry_signature() -> String {
    let mut entries = LYRALANG_SEMANTIC_PROOF_FAMILY_ARTIFACTS
        .iter()
        .map(|item| {
            format!(
                "{}:{}:{}:{}",
                item.id, item.owner_root, item.path, item.role
            )
        })
        .collect::<Vec<_>>();
    entries.sort();
    entries.join("|")
}
pub fn semantic_proof_family_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic.proof.family.registry",
        &semantic_proof_family_registry_signature(),
    )
}
