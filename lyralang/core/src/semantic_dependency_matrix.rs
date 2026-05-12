use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticDependencyMatrixArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub role: &'static str,
}

pub const LYRA_P01_SEMANTIC_DEPENDENCY_MATRIX_CARRIER: &str =
    "LYRA-P01-SEMANTIC-DEPENDENCY-MATRIX-CARRIER v1";

pub const LYRALANG_SEMANTIC_DEPENDENCY_MATRIX_ARTIFACTS:
    &[SemanticDependencyMatrixArtifactDescriptor] = &[
    SemanticDependencyMatrixArtifactDescriptor {
        id: "semantic_dependency_matrix_model",
        owner_root: "interfaces",
        path: "interfaces/p01/src/semantic_dependency_matrix_model.rs",
        role: "typed contract model",
    },
    SemanticDependencyMatrixArtifactDescriptor {
        id: "semantic_dependency_matrix_validator",
        owner_root: "ops",
        path: "ops/p01/src/semantic_dependency_matrix.rs",
        role: "offline validator",
    },
    SemanticDependencyMatrixArtifactDescriptor {
        id: "semantic_dependency_matrix_report",
        owner_root: "k0",
        path: "k0/determinism/src/semantic_dependency_matrix.rs",
        role: "canonical deterministic report",
    },
    SemanticDependencyMatrixArtifactDescriptor {
        id: "semantic_dependency_matrix_surface",
        owner_root: "ops",
        path: "ops/p01/closure/semantic_dependency_matrix.lyra",
        role: "emitted dependency blocker parallelization map",
    },
    SemanticDependencyMatrixArtifactDescriptor {
        id: "semantic_dependency_matrix_receipt",
        owner_root: "receipts",
        path: "receipts/p01/pass_0054_semantic_dependency_matrix.receipt",
        role: "pass receipt binding",
    },
];

pub fn semantic_dependency_matrix_artifact_ids() -> Vec<&'static str> {
    let mut ids = LYRALANG_SEMANTIC_DEPENDENCY_MATRIX_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    ids.sort();
    ids
}

pub fn semantic_dependency_matrix_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_DEPENDENCY_MATRIX_ARTIFACTS
        .iter()
        .all(|item| {
            !item.id.is_empty()
                && !item.owner_root.is_empty()
                && !item.path.is_empty()
                && !item.role.is_empty()
        })
}

pub fn semantic_dependency_matrix_registry_signature() -> String {
    let mut entries = LYRALANG_SEMANTIC_DEPENDENCY_MATRIX_ARTIFACTS
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

pub fn semantic_dependency_matrix_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic.dependency.matrix.registry",
        &semantic_dependency_matrix_registry_signature(),
    )
}
