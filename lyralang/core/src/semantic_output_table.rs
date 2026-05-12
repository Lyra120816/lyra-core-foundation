use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticOutputTableArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub role: &'static str,
}

pub const LYRA_P01_SEMANTIC_OUTPUT_TABLE_CARRIER: &str =
    "LYRA-P01-SEMANTIC-OUTPUT-TABLE-CARRIER v1";

pub const LYRALANG_SEMANTIC_OUTPUT_TABLE_ARTIFACTS: &[SemanticOutputTableArtifactDescriptor] = &[
    SemanticOutputTableArtifactDescriptor {
        id: "semantic_output_table_model",
        owner_root: "interfaces",
        path: "interfaces/p01/src/semantic_output_table_model.rs",
        role: "typed output contract model",
    },
    SemanticOutputTableArtifactDescriptor {
        id: "semantic_output_table_validator",
        owner_root: "ops",
        path: "ops/p01/src/semantic_output_table.rs",
        role: "offline semantic output validator",
    },
    SemanticOutputTableArtifactDescriptor {
        id: "semantic_output_table_report",
        owner_root: "k0",
        path: "k0/determinism/src/semantic_output_table.rs",
        role: "canonical deterministic output report",
    },
    SemanticOutputTableArtifactDescriptor {
        id: "semantic_output_table_surface",
        owner_root: "ops",
        path: "ops/p01/closure/semantic_output_table.lyra",
        role: "emitted audience output table",
    },
    SemanticOutputTableArtifactDescriptor {
        id: "semantic_output_table_fixture",
        owner_root: "fixtures",
        path: "fixtures/p01/semantic_output_table_inputs/valid_semantic_output_table.lyra",
        role: "positive validation fixture",
    },
    SemanticOutputTableArtifactDescriptor {
        id: "semantic_output_table_golden",
        owner_root: "goldens",
        path: "goldens/p01/valid_semantic_output_table.receipt",
        role: "accepted golden receipt",
    },
    SemanticOutputTableArtifactDescriptor {
        id: "semantic_output_table_receipt",
        owner_root: "receipts",
        path: "receipts/p01/pass_0057_semantic_output_table.receipt",
        role: "pass receipt binding",
    },
];

pub fn semantic_output_table_artifact_ids() -> Vec<&'static str> {
    let mut ids = LYRALANG_SEMANTIC_OUTPUT_TABLE_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    ids.sort();
    ids
}
pub fn semantic_output_table_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_OUTPUT_TABLE_ARTIFACTS.iter().all(|item| {
        !item.id.is_empty()
            && !item.owner_root.is_empty()
            && !item.path.is_empty()
            && !item.role.is_empty()
    })
}
pub fn semantic_output_table_registry_signature() -> String {
    let mut entries = LYRALANG_SEMANTIC_OUTPUT_TABLE_ARTIFACTS
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
pub fn semantic_output_table_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic.output.table.registry",
        &semantic_output_table_registry_signature(),
    )
}
