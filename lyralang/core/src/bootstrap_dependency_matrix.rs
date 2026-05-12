use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyNodeDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub status: &'static str,
    pub depends: &'static [&'static str],
    pub unblocks: &'static [&'static str],
    pub owner_root: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyBlockerDescriptor {
    pub id: &'static str,
    pub target: &'static str,
    pub severity: &'static str,
    pub blocks: &'static [&'static str],
    pub requires: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapParallelLaneDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub tasks: &'static [&'static str],
    pub depends: &'static [&'static str],
    pub parallel_safe: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyProofDescriptor {
    pub id: &'static str,
    pub nodes: &'static [&'static str],
    pub blockers: &'static [&'static str],
    pub lanes: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub permits: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_DEPENDENCY_MATRIX_CARRIER: &str =
    "lyra.p02.bootstrap_dependency_matrix.v1";

pub const LYRALANG_BOOTSTRAP_DEPENDENCY_NODES: &[BootstrapDependencyNodeDescriptor] = &[
    BootstrapDependencyNodeDescriptor {
        id: "P02-001",
        kind: "inventory",
        status: "bounded_closed",
        depends: &[],
        unblocks: &["P02-002", "P02-X01"],
        owner_root: "interfaces/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-002",
        kind: "extinction_ledger",
        status: "bounded_closed",
        depends: &["P02-001"],
        unblocks: &["P02-003", "P02-X05"],
        owner_root: "k0/determinism",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-003",
        kind: "seed_runtime_contracts",
        status: "bounded_closed",
        depends: &["P02-002"],
        unblocks: &["P02-004"],
        owner_root: "interfaces/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-004",
        kind: "session_rituals",
        status: "bounded_closed",
        depends: &["P02-003"],
        unblocks: &["P02-005"],
        owner_root: "shells/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-005",
        kind: "host_boundary_challenge",
        status: "bounded_closed",
        depends: &["P02-004"],
        unblocks: &["P02-006"],
        owner_root: "k0/determinism",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-006",
        kind: "target_matrix",
        status: "bounded_closed",
        depends: &["P02-005"],
        unblocks: &["P02-007", "P02-X01"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-007",
        kind: "truth_cleanup",
        status: "bounded_closed",
        depends: &["P02-006"],
        unblocks: &["P02-008"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-008",
        kind: "emergency_fallback",
        status: "bounded_closed",
        depends: &["P02-007"],
        unblocks: &["P02-009"],
        owner_root: "k0/determinism",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-009",
        kind: "seed_runtime_replacement",
        status: "bounded_closed",
        depends: &["P02-003", "P02-008"],
        unblocks: &["P02-010", "P02-X05"],
        owner_root: "k0/determinism",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-010",
        kind: "evidence_emission",
        status: "bounded_closed",
        depends: &["P02-009"],
        unblocks: &["P02-011"],
        owner_root: "k0/determinism",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-011",
        kind: "operator_handoff",
        status: "bounded_closed",
        depends: &["P02-010"],
        unblocks: &["P02-012"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-012",
        kind: "foreign_surface_closure",
        status: "bounded_closed",
        depends: &["P02-002", "P02-009", "P02-011"],
        unblocks: &["P02-013", "P02-X05"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-013",
        kind: "formal_semantics",
        status: "bounded_closed",
        depends: &["P02-012"],
        unblocks: &["P02-014"],
        owner_root: "lyralang/core",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-014",
        kind: "canonical_model",
        status: "bounded_closed",
        depends: &["P02-013"],
        unblocks: &["P02-015"],
        owner_root: "interfaces/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-015",
        kind: "core_engine",
        status: "bounded_closed",
        depends: &["P02-013", "P02-014"],
        unblocks: &["P02-016", "P02-X03"],
        owner_root: "k0/determinism",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-016",
        kind: "falsification",
        status: "bounded_closed",
        depends: &["P02-015"],
        unblocks: &["P02-017", "P02-X02"],
        owner_root: "k0/determinism",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-017",
        kind: "replay",
        status: "bounded_closed",
        depends: &["P02-010", "P02-016"],
        unblocks: &["P02-018", "P02-X02", "P02-X03"],
        owner_root: "k0/determinism",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-018",
        kind: "operator_interface",
        status: "bounded_closed",
        depends: &["P02-017"],
        unblocks: &["P02-019", "P02-X04"],
        owner_root: "shells/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-019",
        kind: "packaging",
        status: "bounded_closed",
        depends: &["P02-018"],
        unblocks: &["P02-020"],
        owner_root: "products/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-020",
        kind: "deployment",
        status: "bounded_closed",
        depends: &["P02-019"],
        unblocks: &["P02-021", "P02-X04"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-021",
        kind: "ecosystem",
        status: "bounded_closed",
        depends: &["P02-020"],
        unblocks: &["P02-022"],
        owner_root: "docs/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-022",
        kind: "economics",
        status: "bounded_closed",
        depends: &["P02-021"],
        unblocks: &["P02-023", "P02-X04"],
        owner_root: "products/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-023",
        kind: "redteam",
        status: "bounded_closed",
        depends: &["P02-005", "P02-008", "P02-012", "P02-022"],
        unblocks: &["P02-024", "P02-X02"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-024",
        kind: "closure_gate",
        status: "bounded_closed",
        depends: &["P02-023"],
        unblocks: &["P02-X01"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-X01",
        kind: "dependency_matrix",
        status: "artifact_emitted",
        depends: &[
            "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
            "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
            "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
        ],
        unblocks: &["P02-X02"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-X02",
        kind: "proof_family_table",
        status: "blocked",
        depends: &["P02-X01", "P02-016", "P02-017", "P02-023", "P02-024"],
        unblocks: &["P02-X03"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-X03",
        kind: "benchmark_pack",
        status: "blocked",
        depends: &["P02-X02", "P02-015", "P02-017"],
        unblocks: &["P02-X04"],
        owner_root: "ops/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-X04",
        kind: "output_table",
        status: "blocked",
        depends: &["P02-X03", "P02-018", "P02-020", "P02-022"],
        unblocks: &["P02-X05"],
        owner_root: "products/p02",
    },
    BootstrapDependencyNodeDescriptor {
        id: "P02-X05",
        kind: "retirement_supersession",
        status: "blocked",
        depends: &["P02-X04", "P02-002", "P02-009", "P02-012"],
        unblocks: &[],
        owner_root: "ops/p02",
    },
];

pub const LYRALANG_BOOTSTRAP_DEPENDENCY_BLOCKERS: &[BootstrapDependencyBlockerDescriptor] = &[
    BootstrapDependencyBlockerDescriptor {
        id: "x02_waits_for_dependency_matrix",
        target: "P02-X02",
        severity: "hard",
        blocks: &["P02-X02"],
        requires: &["P02-X01"],
        status: "active",
    },
    BootstrapDependencyBlockerDescriptor {
        id: "x03_waits_for_proof_family",
        target: "P02-X03",
        severity: "hard",
        blocks: &["P02-X03"],
        requires: &["P02-X02", "P02-015", "P02-017"],
        status: "active",
    },
    BootstrapDependencyBlockerDescriptor {
        id: "x04_waits_for_benchmark_pack",
        target: "P02-X04",
        severity: "hard",
        blocks: &["P02-X04"],
        requires: &["P02-X03", "P02-018", "P02-020", "P02-022"],
        status: "active",
    },
    BootstrapDependencyBlockerDescriptor {
        id: "x05_waits_for_output_table",
        target: "P02-X05",
        severity: "hard",
        blocks: &["P02-X05"],
        requires: &["P02-X04", "P02-002", "P02-009", "P02-012"],
        status: "active",
    },
    BootstrapDependencyBlockerDescriptor {
        id: "global_closure_denied_until_x05",
        target: "P02",
        severity: "hard",
        blocks: &["P02-X05"],
        requires: &["P02-X01", "P02-X02", "P02-X03", "P02-X04", "P02-X05"],
        status: "active",
    },
    BootstrapDependencyBlockerDescriptor {
        id: "parallel_lane_requires_receipts",
        target: "P02-X01",
        severity: "hard",
        blocks: &["P02-X02"],
        requires: &["P02-010", "P02-017", "P02-024"],
        status: "active",
    },
    BootstrapDependencyBlockerDescriptor {
        id: "host_extinction_blocks_retirement",
        target: "P02-X05",
        severity: "hard",
        blocks: &["P02-X05"],
        requires: &["P02-002", "P02-009", "P02-012", "P02-023"],
        status: "active",
    },
    BootstrapDependencyBlockerDescriptor {
        id: "economics_capture_blocks_output_table",
        target: "P02-X04",
        severity: "hard",
        blocks: &["P02-X04"],
        requires: &["P02-022", "P02-023"],
        status: "active",
    },
];

pub const LYRALANG_BOOTSTRAP_PARALLEL_LANES: &[BootstrapParallelLaneDescriptor] = &[
    BootstrapParallelLaneDescriptor {
        id: "lane_bootstrap_trust_core",
        scope: "bootstrap_trust",
        tasks: &["P02-001", "P02-006", "P02-013", "P02-014", "P02-015"],
        depends: &["P02-001"],
        parallel_safe: "yes",
        status: "available_after_primary_gate",
    },
    BootstrapParallelLaneDescriptor {
        id: "lane_seed_runtime_replacement",
        scope: "seed_runtime_law",
        tasks: &["P02-003", "P02-009", "P02-012", "P02-X05"],
        depends: &["P02-X01", "P02-X04"],
        parallel_safe: "no",
        status: "serialized_by_retirement_law",
    },
    BootstrapParallelLaneDescriptor {
        id: "lane_host_extinction",
        scope: "host_extinction_framework",
        tasks: &["P02-002", "P02-005", "P02-008", "P02-023", "P02-X05"],
        depends: &["P02-X01", "P02-X04"],
        parallel_safe: "no",
        status: "serialized_by_rollback_law",
    },
    BootstrapParallelLaneDescriptor {
        id: "lane_evidence_and_replay",
        scope: "evidence_replay",
        tasks: &["P02-010", "P02-016", "P02-017", "P02-X02", "P02-X03"],
        depends: &["P02-X01"],
        parallel_safe: "yes",
        status: "available_after_dependency_matrix",
    },
    BootstrapParallelLaneDescriptor {
        id: "lane_packaging_public_surface",
        scope: "distribution_value",
        tasks: &["P02-019", "P02-020", "P02-021", "P02-022", "P02-X04"],
        depends: &["P02-X01", "P02-X03"],
        parallel_safe: "yes",
        status: "available_after_benchmark_pack",
    },
    BootstrapParallelLaneDescriptor {
        id: "lane_x_outputs_serial",
        scope: "extension_outputs",
        tasks: &["P02-X01", "P02-X02", "P02-X03", "P02-X04", "P02-X05"],
        depends: &["P02-024"],
        parallel_safe: "no",
        status: "strict_serial_x01_to_x05",
    },
];

pub const LYRALANG_BOOTSTRAP_DEPENDENCY_PROOFS: &[BootstrapDependencyProofDescriptor] = &[
    BootstrapDependencyProofDescriptor {
        id: "matrix_primary_node_coverage_proof",
        nodes: &[
            "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
            "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
            "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
        ],
        blockers: &["global_closure_denied_until_x05"],
        lanes: &["lane_bootstrap_trust_core", "lane_evidence_and_replay"],
        receipts: &[
            "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
            "receipts/p02/pass_0062_bootstrap_session_rituals.receipt",
            "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
            "receipts/p02/pass_0064_bootstrap_target_matrix.receipt",
            "receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt",
            "receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt",
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
            "receipts/p02/pass_0069_operator_handoff_automation.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
            "receipts/p02/pass_0072_bootstrap_canonical_model.receipt",
            "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
            "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-closure-check",
            "lyra-p02-bootstrap-dependency-matrix-check",
        ],
        permits: &["p02_x01_artifact_emitted"],
        forbids: &["global_closure", "dependency_skip"],
        status: "artifact_emitted",
    },
    BootstrapDependencyProofDescriptor {
        id: "matrix_x_output_serial_proof",
        nodes: &["P02-X01", "P02-X02", "P02-X03", "P02-X04", "P02-X05"],
        blockers: &[
            "x02_waits_for_dependency_matrix",
            "x03_waits_for_proof_family",
            "x04_waits_for_benchmark_pack",
            "x05_waits_for_output_table",
        ],
        lanes: &["lane_x_outputs_serial"],
        receipts: &["receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt"],
        commands: &["lyra-p02-bootstrap-dependency-matrix-check"],
        permits: &["next_frontier_p02_x02"],
        forbids: &["parallel_x_outputs", "global_closure"],
        status: "artifact_emitted",
    },
    BootstrapDependencyProofDescriptor {
        id: "matrix_parallel_lane_safety_proof",
        nodes: &[
            "P02-010", "P02-016", "P02-017", "P02-019", "P02-020", "P02-021", "P02-022",
        ],
        blockers: &[
            "parallel_lane_requires_receipts",
            "economics_capture_blocks_output_table",
        ],
        lanes: &["lane_evidence_and_replay", "lane_packaging_public_surface"],
        receipts: &[
            "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-evidence-emission-check",
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-dependency-matrix-check",
        ],
        permits: &["bounded_parallel_after_receipts"],
        forbids: &[
            "unreceipted_parallelization",
            "ambient_network",
            "global_closure",
        ],
        status: "artifact_emitted",
    },
    BootstrapDependencyProofDescriptor {
        id: "matrix_host_seed_retirement_blocker_proof",
        nodes: &[
            "P02-002", "P02-003", "P02-005", "P02-008", "P02-009", "P02-012", "P02-023", "P02-X05",
        ],
        blockers: &[
            "host_extinction_blocks_retirement",
            "x05_waits_for_output_table",
        ],
        lanes: &["lane_seed_runtime_replacement", "lane_host_extinction"],
        receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
            "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-extinction-check",
            "lyra-p02-seed-runtime-replacement-check",
            "lyra-p02-foreign-surface-closure-check",
            "lyra-p02-bootstrap-redteam-check",
            "lyra-p02-bootstrap-dependency-matrix-check",
        ],
        permits: &["retirement_waits_for_x05"],
        forbids: &["host_extinction_bypass", "global_closure"],
        status: "artifact_emitted",
    },
    BootstrapDependencyProofDescriptor {
        id: "matrix_public_surface_blocker_proof",
        nodes: &["P02-019", "P02-020", "P02-021", "P02-022", "P02-X04"],
        blockers: &[
            "economics_capture_blocks_output_table",
            "x04_waits_for_benchmark_pack",
        ],
        lanes: &["lane_packaging_public_surface"],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-dependency-matrix-check",
        ],
        permits: &["public_surface_after_benchmark_pack"],
        forbids: &["capture_default", "global_closure"],
        status: "artifact_emitted",
    },
    BootstrapDependencyProofDescriptor {
        id: "matrix_next_frontier_proof",
        nodes: &["P02-X01", "P02-X02"],
        blockers: &["x02_waits_for_dependency_matrix"],
        lanes: &["lane_x_outputs_serial"],
        receipts: &["receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt"],
        commands: &["lyra-p02-bootstrap-dependency-matrix-check"],
        permits: &["next_frontier_p02_x02"],
        forbids: &["skip_to_x03", "global_closure", "phase_closed"],
        status: "artifact_emitted",
    },
];

fn node_preimage(item: &BootstrapDependencyNodeDescriptor) -> String {
    format!(
        "node:{}|kind:{}|status:{}|depends:{}|unblocks:{}|owner_root:{}",
        item.id,
        item.kind,
        item.status,
        item.depends.join(","),
        item.unblocks.join(","),
        item.owner_root
    )
}
fn blocker_preimage(item: &BootstrapDependencyBlockerDescriptor) -> String {
    format!(
        "blocker:{}|target:{}|severity:{}|blocks:{}|requires:{}|status:{}",
        item.id,
        item.target,
        item.severity,
        item.blocks.join(","),
        item.requires.join(","),
        item.status
    )
}
fn lane_preimage(item: &BootstrapParallelLaneDescriptor) -> String {
    format!(
        "lane:{}|scope:{}|tasks:{}|depends:{}|parallel_safe:{}|status:{}",
        item.id,
        item.scope,
        item.tasks.join(","),
        item.depends.join(","),
        item.parallel_safe,
        item.status
    )
}
fn proof_preimage(item: &BootstrapDependencyProofDescriptor) -> String {
    format!("proof:{}|nodes:{}|blockers:{}|lanes:{}|receipts:{}|commands:{}|permits:{}|forbids:{}|status:{}", item.id, item.nodes.join(","), item.blockers.join(","), item.lanes.join(","), item.receipts.join(","), item.commands.join(","), item.permits.join(","), item.forbids.join(","), item.status)
}

pub fn bootstrap_dependency_node_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_DEPENDENCY_NODES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_dependency_blocker_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_DEPENDENCY_BLOCKERS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_parallel_lane_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PARALLEL_LANES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_dependency_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_DEPENDENCY_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_dependency_node_descriptor(
    id: &str,
) -> Option<&'static BootstrapDependencyNodeDescriptor> {
    LYRALANG_BOOTSTRAP_DEPENDENCY_NODES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_dependency_blocker_descriptor(
    id: &str,
) -> Option<&'static BootstrapDependencyBlockerDescriptor> {
    LYRALANG_BOOTSTRAP_DEPENDENCY_BLOCKERS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_parallel_lane_descriptor(
    id: &str,
) -> Option<&'static BootstrapParallelLaneDescriptor> {
    LYRALANG_BOOTSTRAP_PARALLEL_LANES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_dependency_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapDependencyProofDescriptor> {
    LYRALANG_BOOTSTRAP_DEPENDENCY_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_dependency_node_digest(id: &str) -> Option<String> {
    bootstrap_dependency_node_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_dependency_matrix.node_descriptor",
            &node_preimage(item),
        )
    })
}
pub fn bootstrap_dependency_blocker_digest(id: &str) -> Option<String> {
    bootstrap_dependency_blocker_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_dependency_matrix.blocker_descriptor",
            &blocker_preimage(item),
        )
    })
}
pub fn bootstrap_parallel_lane_digest(id: &str) -> Option<String> {
    bootstrap_parallel_lane_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_dependency_matrix.lane_descriptor",
            &lane_preimage(item),
        )
    })
}
pub fn bootstrap_dependency_proof_digest(id: &str) -> Option<String> {
    bootstrap_dependency_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_dependency_matrix.proof_descriptor",
            &proof_preimage(item),
        )
    })
}

pub fn bootstrap_dependency_node_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_dependency_matrix.node_registry",
        &LYRALANG_BOOTSTRAP_DEPENDENCY_NODES
            .iter()
            .map(node_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
    )
}
pub fn bootstrap_dependency_blocker_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_dependency_matrix.blocker_registry",
        &LYRALANG_BOOTSTRAP_DEPENDENCY_BLOCKERS
            .iter()
            .map(blocker_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
    )
}
pub fn bootstrap_parallel_lane_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_dependency_matrix.lane_registry",
        &LYRALANG_BOOTSTRAP_PARALLEL_LANES
            .iter()
            .map(lane_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
    )
}
pub fn bootstrap_dependency_proof_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_dependency_matrix.proof_registry",
        &LYRALANG_BOOTSTRAP_DEPENDENCY_PROOFS
            .iter()
            .map(proof_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

pub fn bootstrap_dependency_matrix_registry_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_dependency_matrix.registry_signature",
        &[
            bootstrap_dependency_node_signature(),
            bootstrap_dependency_blocker_signature(),
            bootstrap_parallel_lane_signature(),
            bootstrap_dependency_proof_signature(),
        ]
        .join("\n"),
    )
}

pub fn bootstrap_dependency_matrix_registry_hash() -> String {
    bootstrap_dependency_matrix_registry_signature()
}
pub fn bootstrap_dependency_matrix_carrier_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_dependency_matrix.carrier",
        LYRA_P02_BOOTSTRAP_DEPENDENCY_MATRIX_CARRIER,
    )
}

fn allowed_path(path: &str) -> bool {
    path.starts_with("k0/")
        || path.starts_with("k1/")
        || path.starts_with("interfaces/")
        || path.starts_with("lyralang/")
        || path.starts_with("ops/")
        || path.starts_with("products/")
        || path.starts_with("shells/")
        || path.starts_with("docs/")
        || path.starts_with("examples/")
        || path.starts_with("fixtures/")
        || path.starts_with("goldens/")
        || path.starts_with("receipts/")
        || path.starts_with("src/bin/")
        || path.starts_with("tests/")
}

pub fn bootstrap_dependency_nodes_bind_owner_roots() -> bool {
    LYRALANG_BOOTSTRAP_DEPENDENCY_NODES.iter().all(|node| {
        allowed_path(node.owner_root) && !node.kind.is_empty() && !node.status.is_empty()
    })
}

pub fn bootstrap_dependency_blockers_bind_required_nodes() -> bool {
    LYRALANG_BOOTSTRAP_DEPENDENCY_BLOCKERS
        .iter()
        .all(|blocker| {
            !blocker.blocks.is_empty()
                && !blocker.requires.is_empty()
                && blocker
                    .blocks
                    .iter()
                    .all(|id| bootstrap_dependency_node_descriptor(id).is_some())
                && blocker
                    .requires
                    .iter()
                    .all(|id| bootstrap_dependency_node_descriptor(id).is_some())
                && (bootstrap_dependency_node_descriptor(blocker.target).is_some()
                    || blocker.target == "P02")
        })
}

pub fn bootstrap_parallel_lanes_bind_existing_nodes() -> bool {
    LYRALANG_BOOTSTRAP_PARALLEL_LANES.iter().all(|lane| {
        !lane.tasks.is_empty()
            && lane
                .tasks
                .iter()
                .all(|id| bootstrap_dependency_node_descriptor(id).is_some())
            && lane
                .depends
                .iter()
                .all(|id| bootstrap_dependency_node_descriptor(id).is_some())
    })
}

pub fn bootstrap_dependency_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_DEPENDENCY_PROOFS.iter().all(|proof| {
        !proof.nodes.is_empty()
            && !proof.receipts.is_empty()
            && proof
                .commands
                .contains(&"lyra-p02-bootstrap-dependency-matrix-check")
            && proof
                .nodes
                .iter()
                .all(|id| bootstrap_dependency_node_descriptor(id).is_some())
            && proof
                .blockers
                .iter()
                .all(|id| bootstrap_dependency_blocker_descriptor(id).is_some())
            && proof
                .lanes
                .iter()
                .all(|id| bootstrap_parallel_lane_descriptor(id).is_some())
            && proof.forbids.contains(&"global_closure")
    })
}

pub fn bootstrap_dependency_matrix_artifacts_bind_paths() -> bool {
    bootstrap_dependency_nodes_bind_owner_roots()
        && LYRALANG_BOOTSTRAP_DEPENDENCY_PROOFS
            .iter()
            .flat_map(|proof| proof.receipts.iter().copied())
            .all(allowed_path)
}

pub fn bootstrap_dependency_matrix_receipts_cover_p02_001_through_p02_x01() -> bool {
    let required = [
        "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
        "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
        "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
        "receipts/p02/pass_0062_bootstrap_session_rituals.receipt",
        "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
        "receipts/p02/pass_0064_bootstrap_target_matrix.receipt",
        "receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt",
        "receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt",
        "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
        "receipts/p02/pass_0069_operator_handoff_automation.receipt",
        "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
        "receipts/p02/pass_0072_bootstrap_canonical_model.receipt",
        "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
        "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        "receipts/p02/pass_0075_bootstrap_replay.receipt",
        "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
        "receipts/p02/pass_0077_bootstrap_packaging.receipt",
        "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        "receipts/p02/pass_0080_bootstrap_economics.receipt",
        "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        "receipts/p02/pass_0082_bootstrap_closure.receipt",
        "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
    ];
    required.iter().all(|needed| {
        LYRALANG_BOOTSTRAP_DEPENDENCY_PROOFS
            .iter()
            .any(|proof| proof.receipts.contains(needed))
            || LYRALANG_BOOTSTRAP_DEPENDENCY_NODES.iter().any(|node| {
                node.id == "P02-X01"
                    && *needed == "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt"
            })
    })
}

pub fn bootstrap_dependency_matrix_no_forbidden_descriptor_claims() -> bool {
    let lowered = [
        LYRALANG_BOOTSTRAP_DEPENDENCY_NODES
            .iter()
            .map(node_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_DEPENDENCY_BLOCKERS
            .iter()
            .map(blocker_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_PARALLEL_LANES
            .iter()
            .map(lane_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_DEPENDENCY_PROOFS
            .iter()
            .map(proof_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
    ]
    .join("\n")
    .to_ascii_lowercase();
    ![
        "network required",
        "cloud required",
        "online required",
        "remote service required",
        "remote fetch",
        "matrix without receipt",
        "docs only",
        "manual only",
        "todo",
        "placeholder",
        "best effort",
        "global complete",
        "phase closed",
        "global_closure=true",
        "dependency skip allowed",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}
