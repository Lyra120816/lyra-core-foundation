use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticBenchmarkPackArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub role: &'static str,
}

pub const LYRA_P01_SEMANTIC_BENCHMARK_PACK_CARRIER: &str =
    "LYRA-P01-SEMANTIC-BENCHMARK-PACK-CARRIER v1";

pub const LYRALANG_SEMANTIC_BENCHMARK_PACK_ARTIFACTS: &[SemanticBenchmarkPackArtifactDescriptor] =
    &[
        SemanticBenchmarkPackArtifactDescriptor {
            id: "semantic_benchmark_pack_model",
            owner_root: "interfaces",
            path: "interfaces/p01/src/semantic_benchmark_pack_model.rs",
            role: "typed contract model",
        },
        SemanticBenchmarkPackArtifactDescriptor {
            id: "semantic_benchmark_pack_validator",
            owner_root: "ops",
            path: "ops/p01/src/semantic_benchmark_pack.rs",
            role: "offline validator",
        },
        SemanticBenchmarkPackArtifactDescriptor {
            id: "semantic_benchmark_pack_report",
            owner_root: "k0",
            path: "k0/determinism/src/semantic_benchmark_pack.rs",
            role: "canonical deterministic report",
        },
        SemanticBenchmarkPackArtifactDescriptor {
            id: "semantic_benchmark_pack_surface",
            owner_root: "ops",
            path: "ops/p01/closure/semantic_benchmark_pack.lyra",
            role: "emitted benchmark target pack",
        },
        SemanticBenchmarkPackArtifactDescriptor {
            id: "semantic_benchmark_pack_fixture",
            owner_root: "fixtures",
            path: "fixtures/p01/semantic_benchmark_pack_inputs/valid_semantic_benchmark_pack.lyra",
            role: "positive validation fixture",
        },
        SemanticBenchmarkPackArtifactDescriptor {
            id: "semantic_benchmark_pack_golden",
            owner_root: "goldens",
            path: "goldens/p01/valid_semantic_benchmark_pack.receipt",
            role: "accepted golden receipt",
        },
        SemanticBenchmarkPackArtifactDescriptor {
            id: "semantic_benchmark_pack_receipt",
            owner_root: "receipts",
            path: "receipts/p01/pass_0056_semantic_benchmark_pack.receipt",
            role: "pass receipt binding",
        },
    ];

pub fn semantic_benchmark_pack_artifact_ids() -> Vec<&'static str> {
    let mut ids = LYRALANG_SEMANTIC_BENCHMARK_PACK_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    ids.sort();
    ids
}
pub fn semantic_benchmark_pack_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_BENCHMARK_PACK_ARTIFACTS
        .iter()
        .all(|item| {
            !item.id.is_empty()
                && !item.owner_root.is_empty()
                && !item.path.is_empty()
                && !item.role.is_empty()
        })
}
pub fn semantic_benchmark_pack_registry_signature() -> String {
    let mut entries = LYRALANG_SEMANTIC_BENCHMARK_PACK_ARTIFACTS
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
pub fn semantic_benchmark_pack_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic.benchmark.pack.registry",
        &semantic_benchmark_pack_registry_signature(),
    )
}
