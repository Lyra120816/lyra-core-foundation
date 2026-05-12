use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticRetirementArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub role: &'static str,
}

pub const LYRA_P01_SEMANTIC_RETIREMENT_CARRIER: &str = "LYRA-P01-SEMANTIC-RETIREMENT-CARRIER v1";

pub const LYRALANG_SEMANTIC_RETIREMENT_ARTIFACTS: &[SemanticRetirementArtifactDescriptor] = &[
    SemanticRetirementArtifactDescriptor {
        id: "semantic_retirement_model",
        owner_root: "interfaces",
        path: "interfaces/p01/src/semantic_retirement_model.rs",
        role: "typed semantic retirement contract model",
    },
    SemanticRetirementArtifactDescriptor {
        id: "semantic_retirement_validator",
        owner_root: "ops",
        path: "ops/p01/src/semantic_retirement.rs",
        role: "offline semantic retirement validator",
    },
    SemanticRetirementArtifactDescriptor {
        id: "semantic_retirement_report",
        owner_root: "k0",
        path: "k0/determinism/src/semantic_retirement.rs",
        role: "canonical deterministic semantic retirement report",
    },
    SemanticRetirementArtifactDescriptor {
        id: "semantic_retirement_carrier",
        owner_root: "lyralang",
        path: "lyralang/core/src/semantic_retirement.rs",
        role: "native migration carrier registry",
    },
    SemanticRetirementArtifactDescriptor {
        id: "semantic_retirement_surface",
        owner_root: "ops",
        path: "ops/p01/closure/semantic_retirement_supersession_law.lyra",
        role: "emitted P01 retirement law",
    },
    SemanticRetirementArtifactDescriptor {
        id: "semantic_retirement_fixture",
        owner_root: "fixtures",
        path: "fixtures/p01/semantic_retirement_inputs/valid_semantic_retirement_supersession.lyra",
        role: "positive validation fixture",
    },
    SemanticRetirementArtifactDescriptor {
        id: "semantic_retirement_golden",
        owner_root: "goldens",
        path: "goldens/p01/valid_semantic_retirement_supersession.receipt",
        role: "accepted golden receipt",
    },
    SemanticRetirementArtifactDescriptor {
        id: "semantic_retirement_receipt",
        owner_root: "receipts",
        path: "receipts/p01/pass_0058_semantic_retirement_supersession.receipt",
        role: "pass receipt binding",
    },
];

pub fn semantic_retirement_artifact_ids() -> Vec<&'static str> {
    let mut ids = LYRALANG_SEMANTIC_RETIREMENT_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    ids.sort();
    ids
}
pub fn semantic_retirement_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_RETIREMENT_ARTIFACTS.iter().all(|item| {
        !item.id.is_empty()
            && !item.owner_root.is_empty()
            && !item.path.is_empty()
            && !item.role.is_empty()
    })
}
pub fn semantic_retirement_registry_signature() -> String {
    let mut entries = LYRALANG_SEMANTIC_RETIREMENT_ARTIFACTS
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
pub fn semantic_retirement_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic.retirement.registry",
        &semantic_retirement_registry_signature(),
    )
}
