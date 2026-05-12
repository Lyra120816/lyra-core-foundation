use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_BOOTSTRAP_CLOSURE_CARRIER: &str = "lyra.p02.bootstrap_closure.carrier.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapClosureTaskDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub evidence: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapClosureOutputDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub depends: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapClosureProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub tasks: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub permits: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRALANG_BOOTSTRAP_CLOSURE_TASKS: &[BootstrapClosureTaskDescriptor] = &[
    BootstrapClosureTaskDescriptor {
        id: "P02-001",
        scope: "inventory",
        receipts: &["receipts/p02/pass_0059_bootstrap_surface_inventory.receipt"],
        commands: &[
            "lyra-p02-bootstrap-inventory-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_surface_inventory.v1.lyra",
            "goldens/p02/valid_bootstrap_surface_inventory.receipt",
            "docs/p02/bootstrap_surface_inventory_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-002",
        scope: "extinction_ledger",
        receipts: &["receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt"],
        commands: &[
            "lyra-p02-bootstrap-extinction-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_extinction_ledger.v1.lyra",
            "goldens/p02/valid_bootstrap_extinction_ledger.receipt",
            "docs/p02/bootstrap_extinction_ledger_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-003",
        scope: "seed_runtime_contracts",
        receipts: &["receipts/p02/pass_0061_seed_runtime_contracts.receipt"],
        commands: &[
            "lyra-p02-bootstrap-interface-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/seed_runtime_contracts.v1.lyra",
            "goldens/p02/valid_seed_runtime_contracts.receipt",
            "docs/p02/seed_runtime_contracts_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-004",
        scope: "session_rituals",
        receipts: &["receipts/p02/pass_0062_bootstrap_session_rituals.receipt"],
        commands: &[
            "lyra-p02-bootstrap-interface-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_session_rituals.v1.lyra",
            "goldens/p02/valid_bootstrap_session_rituals.receipt",
            "docs/p02/bootstrap_session_rituals_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-005",
        scope: "host_boundary_challenge",
        receipts: &["receipts/p02/pass_0063_host_boundary_challenge_suites.receipt"],
        commands: &[
            "lyra-p02-host-boundary-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/host_boundary_challenge.v1.lyra",
            "goldens/p02/valid_host_boundary_challenge_suites.receipt",
            "docs/p02/host_boundary_challenge_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-006",
        scope: "target_matrix",
        receipts: &["receipts/p02/pass_0064_bootstrap_target_matrix.receipt"],
        commands: &[
            "lyra-p02-target-matrix-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_target_matrix.v1.lyra",
            "goldens/p02/valid_bootstrap_target_matrix.receipt",
            "docs/p02/bootstrap_target_matrix_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-007",
        scope: "truth_cleanup",
        receipts: &["receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt"],
        commands: &[
            "lyra-p02-truth-cleanup-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_truth_cleanup.v1.lyra",
            "goldens/p02/valid_bootstrap_truth_cleanup.receipt",
            "docs/p02/bootstrap_truth_cleanup_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-008",
        scope: "emergency_fallback",
        receipts: &["receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt"],
        commands: &[
            "lyra-p02-emergency-fallback-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_emergency_fallback.v1.lyra",
            "goldens/p02/valid_bootstrap_emergency_fallback.receipt",
            "docs/p02/bootstrap_emergency_fallback_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-009",
        scope: "seed_runtime_replacement",
        receipts: &["receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt"],
        commands: &[
            "lyra-p02-seed-runtime-replacement-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/seed_runtime_replacement_milestones.v1.lyra",
            "goldens/p02/valid_seed_runtime_replacement_milestones.receipt",
            "docs/p02/seed_runtime_replacement_milestones_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-010",
        scope: "evidence_emission",
        receipts: &["receipts/p02/pass_0068_bootstrap_evidence_emission.receipt"],
        commands: &[
            "lyra-p02-bootstrap-evidence-emission-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_evidence_emission.v1.lyra",
            "goldens/p02/valid_bootstrap_evidence_emission.receipt",
            "docs/p02/bootstrap_evidence_emission_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-011",
        scope: "operator_handoff",
        receipts: &["receipts/p02/pass_0069_operator_handoff_automation.receipt"],
        commands: &[
            "lyra-p02-operator-handoff-automation-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/operator_handoff_automation.v1.lyra",
            "goldens/p02/valid_operator_handoff_automation.receipt",
            "docs/p02/operator_handoff_automation_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-012",
        scope: "foreign_surface_closure",
        receipts: &["receipts/p02/pass_0070_foreign_surface_closure.receipt"],
        commands: &[
            "lyra-p02-foreign-surface-closure-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/foreign_surface_closure.v1.lyra",
            "goldens/p02/valid_foreign_surface_closure.receipt",
            "docs/p02/foreign_surface_closure_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-013",
        scope: "formal_semantics",
        receipts: &["receipts/p02/pass_0071_bootstrap_formal_semantics.receipt"],
        commands: &[
            "lyra-p02-bootstrap-formal-semantics-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_formal_semantics.v1.lyra",
            "goldens/p02/valid_bootstrap_formal_semantics.receipt",
            "docs/p02/bootstrap_formal_semantics_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-014",
        scope: "canonical_model",
        receipts: &["receipts/p02/pass_0072_bootstrap_canonical_model.receipt"],
        commands: &[
            "lyra-p02-bootstrap-canonical-model-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_canonical_model.v1.lyra",
            "goldens/p02/valid_bootstrap_canonical_model.receipt",
            "docs/p02/bootstrap_canonical_model_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-015",
        scope: "core_engine",
        receipts: &["receipts/p02/pass_0073_bootstrap_core_engine.receipt"],
        commands: &[
            "lyra-p02-bootstrap-core-engine-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_core_engine.v1.lyra",
            "goldens/p02/valid_bootstrap_core_engine.receipt",
            "docs/p02/bootstrap_core_engine_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-016",
        scope: "falsification",
        receipts: &["receipts/p02/pass_0074_bootstrap_falsification.receipt"],
        commands: &[
            "lyra-p02-bootstrap-falsification-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_falsification.v1.lyra",
            "goldens/p02/valid_bootstrap_falsification.receipt",
            "docs/p02/bootstrap_falsification_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-017",
        scope: "replay",
        receipts: &["receipts/p02/pass_0075_bootstrap_replay.receipt"],
        commands: &[
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_replay.v1.lyra",
            "goldens/p02/valid_bootstrap_replay.receipt",
            "docs/p02/bootstrap_replay_guide.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-018",
        scope: "operator_interface",
        receipts: &["receipts/p02/pass_0076_bootstrap_operator_interface.receipt"],
        commands: &[
            "lyra-p02-bootstrap-interface-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_operator_interface.v1.lyra",
            "goldens/p02/valid_bootstrap_operator_interface.receipt",
            "docs/p02/bootstrap_operator_interface.md",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-019",
        scope: "packaging",
        receipts: &["receipts/p02/pass_0077_bootstrap_packaging.receipt"],
        commands: &[
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_packaging.v1.lyra",
            "goldens/p02/valid_bootstrap_packaging.receipt",
            "products/p02/bootstrap_package_manifest.v1.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-020",
        scope: "deployment",
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_deployment.v1.lyra",
            "goldens/p02/valid_bootstrap_deployment.receipt",
            "docs/p02/bootstrap_deployment_guide.v1.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-021",
        scope: "ecosystem",
        receipts: &["receipts/p02/pass_0079_bootstrap_ecosystem.receipt"],
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_ecosystem.v1.lyra",
            "goldens/p02/valid_bootstrap_ecosystem.receipt",
            "docs/p02/bootstrap_deployment_ecosystem_walkthrough.v1.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-022",
        scope: "economics",
        receipts: &["receipts/p02/pass_0080_bootstrap_economics.receipt"],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_economics.v1.lyra",
            "goldens/p02/valid_bootstrap_economics.receipt",
            "docs/p02/bootstrap_trust_public_value_frame.v1.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-023",
        scope: "redteam",
        receipts: &["receipts/p02/pass_0081_bootstrap_redteam.receipt"],
        commands: &[
            "lyra-p02-bootstrap-redteam-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_redteam.v1.lyra",
            "goldens/p02/valid_bootstrap_redteam.receipt",
            "docs/p02/bootstrap_redteam_rollback_guide.v1.lyra",
        ],
        status: "bounded_closed",
    },
    BootstrapClosureTaskDescriptor {
        id: "P02-024",
        scope: "closure_gate",
        receipts: &["receipts/p02/pass_0082_bootstrap_closure.receipt"],
        commands: &["lyra-p02-bootstrap-closure-check"],
        evidence: &[
            "interfaces/p02/contracts/bootstrap_closure.v1.lyra",
            "goldens/p02/valid_bootstrap_closure.receipt",
            "ops/p02/control/bootstrap_closure_gate_law.v1.lyra",
        ],
        status: "bounded_closed",
    },
];

pub const LYRALANG_BOOTSTRAP_CLOSURE_OUTPUTS: &[BootstrapClosureOutputDescriptor] = &[
    BootstrapClosureOutputDescriptor {
        id: "P02-X01",
        kind: "dependency_matrix",
        path: "ops/p02/closure/p02_x01_dependency_matrix_gate.v1.lyra",
        depends: &[
            "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
            "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
            "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
        ],
        receipts: &["receipts/p02/pass_0082_bootstrap_closure.receipt"],
        status: "blocked",
    },
    BootstrapClosureOutputDescriptor {
        id: "P02-X02",
        kind: "proof_family_table",
        path: "ops/p02/closure/p02_x02_proof_family_gate.v1.lyra",
        depends: &["P02-X01", "P02-023", "P02-024"],
        receipts: &[
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
        ],
        status: "blocked",
    },
    BootstrapClosureOutputDescriptor {
        id: "P02-X03",
        kind: "benchmark_pack",
        path: "ops/p02/closure/p02_x03_benchmark_pack_gate.v1.lyra",
        depends: &["P02-X01", "P02-X02", "P02-015", "P02-017"],
        receipts: &[
            "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
        ],
        status: "blocked",
    },
    BootstrapClosureOutputDescriptor {
        id: "P02-X04",
        kind: "output_table",
        path: "ops/p02/closure/p02_x04_output_table_gate.v1.lyra",
        depends: &[
            "P02-X01", "P02-X02", "P02-X03", "P02-018", "P02-020", "P02-022",
        ],
        receipts: &[
            "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
        ],
        status: "blocked",
    },
    BootstrapClosureOutputDescriptor {
        id: "P02-X05",
        kind: "retirement_law",
        path: "ops/p02/closure/p02_x05_retirement_supersession_gate.v1.lyra",
        depends: &[
            "P02-X01", "P02-X02", "P02-X03", "P02-X04", "P02-002", "P02-009", "P02-012",
        ],
        receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
        ],
        status: "blocked",
    },
];

pub const LYRALANG_BOOTSTRAP_CLOSURE_PROOFS: &[BootstrapClosureProofDescriptor] = &[
    BootstrapClosureProofDescriptor {
        id: "bootstrap_primary_task_receipt_chain",
        scope: "receipt_chain",
        tasks: &[
            "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
            "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
            "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
        ],
        outputs: &["P02-X01"],
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
        ],
        commands: &["lyra-p02-bootstrap-closure-check"],
        permits: &["bounded_primary_closure"],
        forbids: &["global_closure", "unreceipted_closure"],
        status: "artifact_emitted",
    },
    BootstrapClosureProofDescriptor {
        id: "bootstrap_negative_corpus_receipt_chain",
        scope: "receipt_chain",
        tasks: &["P02-016", "P02-017", "P02-023", "P02-024"],
        outputs: &["P02-X02"],
        receipts: &[
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-falsification-check",
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-redteam-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        permits: &["bounded_primary_closure"],
        forbids: &["global_closure", "unreceipted_closure", "network_closure"],
        status: "artifact_emitted",
    },
    BootstrapClosureProofDescriptor {
        id: "bootstrap_redteam_rollback_receipt_chain",
        scope: "receipt_chain",
        tasks: &["P02-005", "P02-008", "P02-012", "P02-023", "P02-024"],
        outputs: &["P02-X02", "P02-X05"],
        receipts: &[
            "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
            "receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
        ],
        commands: &[
            "lyra-p02-host-boundary-check",
            "lyra-p02-emergency-fallback-check",
            "lyra-p02-foreign-surface-closure-check",
            "lyra-p02-bootstrap-redteam-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        permits: &["bounded_primary_closure"],
        forbids: &["global_closure", "unreceipted_closure", "rollback_bypass"],
        status: "artifact_emitted",
    },
    BootstrapClosureProofDescriptor {
        id: "bootstrap_control_plane_transition_proof",
        scope: "control_plane",
        tasks: &["P02-007", "P02-010", "P02-011", "P02-024"],
        outputs: &["P02-X01", "P02-X04"],
        receipts: &[
            "receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt",
            "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
            "receipts/p02/pass_0069_operator_handoff_automation.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
        ],
        commands: &[
            "lyra-p02-truth-cleanup-check",
            "lyra-p02-bootstrap-evidence-emission-check",
            "lyra-p02-operator-handoff-automation-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        permits: &["bounded_primary_closure"],
        forbids: &[
            "global_closure",
            "unreceipted_closure",
            "silent_frontier_advance",
        ],
        status: "artifact_emitted",
    },
    BootstrapClosureProofDescriptor {
        id: "bootstrap_bounded_vs_global_closure_proof",
        scope: "global_denial",
        tasks: &[
            "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
        ],
        outputs: &["P02-X01", "P02-X02", "P02-X03", "P02-X04", "P02-X05"],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
            "receipts/p02/pass_0081_bootstrap_redteam.receipt",
            "receipts/p02/pass_0082_bootstrap_closure.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-redteam-check",
            "lyra-p02-bootstrap-closure-check",
        ],
        permits: &["bounded_primary_closure"],
        forbids: &[
            "global_closure",
            "phase_output_completion",
            "unreceipted_closure",
        ],
        status: "artifact_emitted",
    },
    BootstrapClosureProofDescriptor {
        id: "bootstrap_output_gate_open_proof",
        scope: "closure_outputs",
        tasks: &["P02-024"],
        outputs: &["P02-X01", "P02-X02", "P02-X03", "P02-X04", "P02-X05"],
        receipts: &["receipts/p02/pass_0082_bootstrap_closure.receipt"],
        commands: &["lyra-p02-bootstrap-closure-check"],
        permits: &["next_frontier_p02_x01"],
        forbids: &[
            "global_closure",
            "output_gate_completion",
            "unreceipted_closure",
        ],
        status: "artifact_emitted",
    },
];

pub fn bootstrap_closure_task_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CLOSURE_TASKS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_closure_output_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CLOSURE_OUTPUTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_closure_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CLOSURE_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_closure_task_descriptor(
    id: &str,
) -> Option<&'static BootstrapClosureTaskDescriptor> {
    LYRALANG_BOOTSTRAP_CLOSURE_TASKS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_closure_output_descriptor(
    id: &str,
) -> Option<&'static BootstrapClosureOutputDescriptor> {
    LYRALANG_BOOTSTRAP_CLOSURE_OUTPUTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_closure_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapClosureProofDescriptor> {
    LYRALANG_BOOTSTRAP_CLOSURE_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_closure_task_digest(id: &str) -> Option<String> {
    bootstrap_closure_task_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_closure.task_descriptor",
            &task_preimage(item),
        )
    })
}
pub fn bootstrap_closure_output_digest(id: &str) -> Option<String> {
    bootstrap_closure_output_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_closure.output_descriptor",
            &output_preimage(item),
        )
    })
}
pub fn bootstrap_closure_proof_digest(id: &str) -> Option<String> {
    bootstrap_closure_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_closure.proof_descriptor",
            &proof_preimage(item),
        )
    })
}

pub fn bootstrap_closure_task_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_closure.task_registry",
        &LYRALANG_BOOTSTRAP_CLOSURE_TASKS
            .iter()
            .map(task_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
    )
}
pub fn bootstrap_closure_output_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_closure.output_registry",
        &LYRALANG_BOOTSTRAP_CLOSURE_OUTPUTS
            .iter()
            .map(output_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
    )
}
pub fn bootstrap_closure_proof_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_closure.proof_registry",
        &LYRALANG_BOOTSTRAP_CLOSURE_PROOFS
            .iter()
            .map(proof_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
    )
}

pub fn bootstrap_closure_registry_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_closure.registry_signature",
        &[
            bootstrap_closure_task_signature(),
            bootstrap_closure_output_signature(),
            bootstrap_closure_proof_signature(),
        ]
        .join(
            "
",
        ),
    )
}

pub fn bootstrap_closure_registry_hash() -> String {
    bootstrap_closure_registry_signature()
}
pub fn bootstrap_closure_carrier_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_closure.carrier",
        LYRA_P02_BOOTSTRAP_CLOSURE_CARRIER,
    )
}

pub fn bootstrap_closure_tasks_bind_receipts() -> bool {
    LYRALANG_BOOTSTRAP_CLOSURE_TASKS.iter().all(|task| {
        !task.receipts.is_empty()
            && task
                .receipts
                .iter()
                .all(|receipt| receipt.starts_with("receipts/p02/pass_"))
            && task.commands.contains(&"lyra-p02-bootstrap-closure-check")
            && !task.evidence.is_empty()
    })
}

pub fn bootstrap_closure_outputs_remain_open() -> bool {
    LYRALANG_BOOTSTRAP_CLOSURE_OUTPUTS.iter().all(|output| {
        matches!(
            output.status,
            "blocked" | "working_slice" | "artifact_emitted"
        ) && !output.depends.is_empty()
            && !output.receipts.is_empty()
    })
}

pub fn bootstrap_closure_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_CLOSURE_PROOFS.iter().all(|proof| {
        !proof.tasks.is_empty()
            && !proof.outputs.is_empty()
            && !proof.receipts.is_empty()
            && proof.commands.contains(&"lyra-p02-bootstrap-closure-check")
            && proof.permits.iter().any(|permit| {
                *permit == "bounded_primary_closure" || *permit == "next_frontier_p02_x01"
            })
            && proof.forbids.contains(&"global_closure")
            && proof.forbids.contains(&"unreceipted_closure")
    })
}

pub fn bootstrap_closure_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_CLOSURE_TASKS
        .iter()
        .flat_map(|task| task.evidence.iter().copied())
        .all(allowed_path)
        && LYRALANG_BOOTSTRAP_CLOSURE_OUTPUTS
            .iter()
            .all(|output| allowed_path(output.path))
}

pub fn bootstrap_closure_receipts_cover_p02_001_through_p02_024() -> bool {
    let mut required = [
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
    ];
    required.sort();
    required.iter().all(|needed| {
        LYRALANG_BOOTSTRAP_CLOSURE_TASKS
            .iter()
            .any(|task| task.receipts.contains(needed))
    })
}

pub fn bootstrap_closure_no_forbidden_descriptor_claims() -> bool {
    let lowered = [
        LYRALANG_BOOTSTRAP_CLOSURE_TASKS
            .iter()
            .map(task_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
        LYRALANG_BOOTSTRAP_CLOSURE_OUTPUTS
            .iter()
            .map(output_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
        LYRALANG_BOOTSTRAP_CLOSURE_PROOFS
            .iter()
            .map(proof_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
    ]
    .join(
        "
",
    )
    .to_ascii_lowercase();
    ![
        "network required",
        "cloud required",
        "online required",
        "remote service required",
        "remote fetch",
        "unreceipted closure allowed",
        "closure without receipt",
        "closure drift accepted",
        "global complete",
        "phase closed",
        "phase closure true",
        "global closure true",
        "closure outputs complete",
        "manual only",
        "docs only",
        "todo",
        "placeholder",
        "best effort",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

fn task_preimage(item: &BootstrapClosureTaskDescriptor) -> String {
    format!(
        "task:{}|scope:{}|receipts:{}|commands:{}|evidence:{}|status:{}",
        item.id,
        item.scope,
        item.receipts.join(","),
        item.commands.join(","),
        item.evidence.join(","),
        item.status
    )
}
fn output_preimage(item: &BootstrapClosureOutputDescriptor) -> String {
    format!(
        "output:{}|kind:{}|path:{}|depends:{}|receipts:{}|status:{}",
        item.id,
        item.kind,
        item.path,
        item.depends.join(","),
        item.receipts.join(","),
        item.status
    )
}
fn proof_preimage(item: &BootstrapClosureProofDescriptor) -> String {
    format!("proof:{}|scope:{}|tasks:{}|outputs:{}|receipts:{}|commands:{}|permits:{}|forbids:{}|status:{}", item.id, item.scope, item.tasks.join(","), item.outputs.join(","), item.receipts.join(","), item.commands.join(","), item.permits.join(","), item.forbids.join(","), item.status)
}
fn allowed_path(path: &str) -> bool {
    [
        "docs/",
        "examples/",
        "products/",
        "fixtures/",
        "receipts/",
        "ops/",
        "interfaces/",
        "src/",
        "tests/",
        "shells/",
        "goldens/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
        && !path.contains("..")
}
