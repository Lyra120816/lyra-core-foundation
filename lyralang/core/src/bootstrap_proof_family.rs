use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapProofFamilyArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub role: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapProofFamilyDescriptor {
    pub id: &'static str,
    pub family_kind: &'static str,
    pub scope: &'static str,
    pub receipts: &'static [&'static str],
    pub covers: &'static [&'static str],
    pub proofs: &'static [&'static str],
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapProofReceiptDescriptor {
    pub id: &'static str,
    pub family: &'static str,
    pub path: &'static str,
    pub covers: &'static [&'static str],
    pub verdict: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapProofPathDescriptor {
    pub id: &'static str,
    pub family: &'static str,
    pub path_kind: &'static str,
    pub entry_receipts: &'static [&'static str],
    pub challenge_receipts: &'static [&'static str],
    pub rollback_receipts: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_PROOF_FAMILY_CARRIER: &str =
    "LYRA-P02-BOOTSTRAP-PROOF-FAMILY-CARRIER v1";

pub const LYRALANG_BOOTSTRAP_PROOF_FAMILY_ARTIFACTS: &[BootstrapProofFamilyArtifactDescriptor] = &[
    BootstrapProofFamilyArtifactDescriptor {
        id: "bootstrap_proof_family_report",
        owner_root: "k0",
        path: "k0/determinism/src/bootstrap_proof_family.rs",
        role: "canonical deterministic proof-family report",
    },
    BootstrapProofFamilyArtifactDescriptor {
        id: "bootstrap_proof_family_model",
        owner_root: "interfaces",
        path: "interfaces/p02/src/bootstrap_proof_family_model.rs",
        role: "typed proof-family surface model",
    },
    BootstrapProofFamilyArtifactDescriptor {
        id: "bootstrap_proof_family_validator",
        owner_root: "ops",
        path: "ops/p02/src/bootstrap_proof_family.rs",
        role: "offline proof-family validator",
    },
    BootstrapProofFamilyArtifactDescriptor {
        id: "bootstrap_proof_family_contract",
        owner_root: "interfaces",
        path: "interfaces/p02/contracts/bootstrap_proof_family.v1.lyra",
        role: "versioned proof-family contract",
    },
    BootstrapProofFamilyArtifactDescriptor {
        id: "bootstrap_proof_family_surface",
        owner_root: "ops",
        path: "ops/p02/closure/p02_x02_proof_family_gate.v1.lyra",
        role: "emitted proof-family table",
    },
    BootstrapProofFamilyArtifactDescriptor {
        id: "bootstrap_proof_family_manifest",
        owner_root: "products",
        path: "products/p02/bootstrap_proof_family_manifest.v1.lyra",
        role: "operator-facing proof-family manifest",
    },
    BootstrapProofFamilyArtifactDescriptor {
        id: "bootstrap_proof_family_shell",
        owner_root: "shells",
        path: "shells/p02/bootstrap_proof_family_shell.v1.lyra",
        role: "operator command surface",
    },
    BootstrapProofFamilyArtifactDescriptor {
        id: "bootstrap_proof_family_receipt",
        owner_root: "receipts",
        path: "receipts/p02/pass_0084_bootstrap_proof_family.receipt",
        role: "pass receipt binding",
    },
];

pub const LYRALANG_BOOTSTRAP_PROOF_FAMILIES: &[BootstrapProofFamilyDescriptor] = &[
    BootstrapProofFamilyDescriptor {
        id: "happy_path",
        family_kind: "happy_path",
        scope: "P02",
        receipts: &[
            "receipt_happy_p02_001",
            "receipt_happy_p02_002",
            "receipt_happy_p02_003",
            "receipt_happy_p02_004",
            "receipt_happy_p02_005",
            "receipt_happy_p02_006",
            "receipt_happy_p02_007",
            "receipt_happy_p02_008",
            "receipt_happy_p02_009",
            "receipt_happy_p02_010",
            "receipt_happy_p02_011",
            "receipt_happy_p02_012",
            "receipt_happy_p02_013",
            "receipt_happy_p02_014",
            "receipt_happy_p02_015",
            "receipt_happy_p02_016",
            "receipt_happy_p02_017",
            "receipt_happy_p02_018",
            "receipt_happy_p02_019",
            "receipt_happy_p02_020",
            "receipt_happy_p02_021",
            "receipt_happy_p02_022",
            "receipt_happy_p02_023",
            "receipt_happy_p02_024",
            "receipt_happy_p02_x01",
        ],
        covers: &[
            "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
            "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
            "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
            "P02-X01",
        ],
        proofs: &[
            "primary_receipts",
            "closure_gate_receipt",
            "dependency_matrix_receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapProofFamilyDescriptor {
        id: "negative_path",
        family_kind: "negative_path",
        scope: "P02",
        receipts: &[
            "receipt_negative_truth_cleanup_rejection",
            "receipt_negative_host_boundary_rejection",
            "receipt_negative_falsification_rejection",
            "receipt_negative_redteam_rejection",
            "receipt_negative_dependency_matrix_rejection",
        ],
        covers: &[
            "P02-005", "P02-007", "P02-010", "P02-016", "P02-017", "P02-023", "P02-X01",
        ],
        proofs: &[
            "rejection_corpus",
            "falsification_receipts",
            "negative_fixture_matrix",
        ],
        status: "artifact_emitted",
    },
    BootstrapProofFamilyDescriptor {
        id: "adversarial_path",
        family_kind: "adversarial_path",
        scope: "P02",
        receipts: &[
            "receipt_adversarial_host_boundary_challenge",
            "receipt_adversarial_emergency_fallback_challenge",
            "receipt_adversarial_foreign_surface_challenge",
            "receipt_adversarial_economics_capture_challenge",
            "receipt_adversarial_redteam_attack_challenge",
        ],
        covers: &["P02-005", "P02-008", "P02-012", "P02-022", "P02-023"],
        proofs: &[
            "host_boundary_challenge",
            "capture_challenge",
            "redteam_attack_table",
        ],
        status: "artifact_emitted",
    },
    BootstrapProofFamilyDescriptor {
        id: "rollback_path",
        family_kind: "rollback_path",
        scope: "P02",
        receipts: &[
            "receipt_rollback_extinction_rollback",
            "receipt_rollback_seed_contract_rollback",
            "receipt_rollback_foreign_surface_rollback",
            "receipt_rollback_replay_rollback",
            "receipt_rollback_redteam_rollback",
            "receipt_rollback_closure_gate_rollback",
            "receipt_rollback_dependency_matrix_rollback",
        ],
        covers: &[
            "P02-002", "P02-003", "P02-009", "P02-012", "P02-017", "P02-023", "P02-024", "P02-X01",
        ],
        proofs: &[
            "rollback_law",
            "replay_receipts",
            "host_extinction_blockers",
        ],
        status: "artifact_emitted",
    },
    BootstrapProofFamilyDescriptor {
        id: "dependency_path",
        family_kind: "dependency_path",
        scope: "P02",
        receipts: &[
            "receipt_dependency_evidence_emission",
            "receipt_dependency_replay_bridge",
            "receipt_dependency_closure_gate",
            "receipt_dependency_dependency_matrix",
            "receipt_dependency_packaging_deployment",
            "receipt_dependency_economics_redteam",
        ],
        covers: &[
            "P02-010", "P02-017", "P02-019", "P02-020", "P02-022", "P02-023", "P02-024", "P02-X01",
        ],
        proofs: &[
            "dependency_matrix_graph",
            "parallel_lane_check",
            "next_frontier_proof",
        ],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_PROOF_RECEIPTS: &[BootstrapProofReceiptDescriptor] = &[
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_001",
        family: "happy_path",
        path: "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
        covers: &["P02-001"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_002",
        family: "happy_path",
        path: "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
        covers: &["P02-002"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_003",
        family: "happy_path",
        path: "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
        covers: &["P02-003"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_004",
        family: "happy_path",
        path: "receipts/p02/pass_0062_bootstrap_session_rituals.receipt",
        covers: &["P02-004"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_005",
        family: "happy_path",
        path: "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
        covers: &["P02-005"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_006",
        family: "happy_path",
        path: "receipts/p02/pass_0064_bootstrap_target_matrix.receipt",
        covers: &["P02-006"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_007",
        family: "happy_path",
        path: "receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt",
        covers: &["P02-007"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_008",
        family: "happy_path",
        path: "receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt",
        covers: &["P02-008"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_009",
        family: "happy_path",
        path: "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        covers: &["P02-009"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_010",
        family: "happy_path",
        path: "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
        covers: &["P02-010"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_011",
        family: "happy_path",
        path: "receipts/p02/pass_0069_operator_handoff_automation.receipt",
        covers: &["P02-011"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_012",
        family: "happy_path",
        path: "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        covers: &["P02-012"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_013",
        family: "happy_path",
        path: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
        covers: &["P02-013"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_014",
        family: "happy_path",
        path: "receipts/p02/pass_0072_bootstrap_canonical_model.receipt",
        covers: &["P02-014"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_015",
        family: "happy_path",
        path: "receipts/p02/pass_0073_bootstrap_core_engine.receipt",
        covers: &["P02-015"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_016",
        family: "happy_path",
        path: "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        covers: &["P02-016"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_017",
        family: "happy_path",
        path: "receipts/p02/pass_0075_bootstrap_replay.receipt",
        covers: &["P02-017"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_018",
        family: "happy_path",
        path: "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
        covers: &["P02-018"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_019",
        family: "happy_path",
        path: "receipts/p02/pass_0077_bootstrap_packaging.receipt",
        covers: &["P02-019"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_020",
        family: "happy_path",
        path: "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        covers: &["P02-020"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_021",
        family: "happy_path",
        path: "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        covers: &["P02-021"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_022",
        family: "happy_path",
        path: "receipts/p02/pass_0080_bootstrap_economics.receipt",
        covers: &["P02-022"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_023",
        family: "happy_path",
        path: "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        covers: &["P02-023"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_024",
        family: "happy_path",
        path: "receipts/p02/pass_0082_bootstrap_closure.receipt",
        covers: &["P02-024"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_happy_p02_x01",
        family: "happy_path",
        path: "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        covers: &["P02-X01"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_negative_truth_cleanup_rejection",
        family: "negative_path",
        path: "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
        covers: &["P02-007", "P02-010"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_negative_host_boundary_rejection",
        family: "negative_path",
        path: "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        covers: &["P02-005", "P02-016"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_negative_falsification_rejection",
        family: "negative_path",
        path: "receipts/p02/pass_0075_bootstrap_replay.receipt",
        covers: &["P02-016", "P02-017"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_negative_redteam_rejection",
        family: "negative_path",
        path: "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        covers: &["P02-023"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_negative_dependency_matrix_rejection",
        family: "negative_path",
        path: "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        covers: &["P02-X01"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_adversarial_host_boundary_challenge",
        family: "adversarial_path",
        path: "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
        covers: &["P02-005"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_adversarial_emergency_fallback_challenge",
        family: "adversarial_path",
        path: "receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt",
        covers: &["P02-008"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_adversarial_foreign_surface_challenge",
        family: "adversarial_path",
        path: "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        covers: &["P02-012"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_adversarial_economics_capture_challenge",
        family: "adversarial_path",
        path: "receipts/p02/pass_0080_bootstrap_economics.receipt",
        covers: &["P02-022"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_adversarial_redteam_attack_challenge",
        family: "adversarial_path",
        path: "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        covers: &["P02-023"],
        verdict: "rejected_expected",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_rollback_extinction_rollback",
        family: "rollback_path",
        path: "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
        covers: &["P02-002"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_rollback_seed_contract_rollback",
        family: "rollback_path",
        path: "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        covers: &["P02-003", "P02-009"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_rollback_foreign_surface_rollback",
        family: "rollback_path",
        path: "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        covers: &["P02-012"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_rollback_replay_rollback",
        family: "rollback_path",
        path: "receipts/p02/pass_0075_bootstrap_replay.receipt",
        covers: &["P02-017"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_rollback_redteam_rollback",
        family: "rollback_path",
        path: "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        covers: &["P02-023"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_rollback_closure_gate_rollback",
        family: "rollback_path",
        path: "receipts/p02/pass_0082_bootstrap_closure.receipt",
        covers: &["P02-024"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_rollback_dependency_matrix_rollback",
        family: "rollback_path",
        path: "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        covers: &["P02-X01"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_dependency_evidence_emission",
        family: "dependency_path",
        path: "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
        covers: &["P02-010"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_dependency_replay_bridge",
        family: "dependency_path",
        path: "receipts/p02/pass_0075_bootstrap_replay.receipt",
        covers: &["P02-017"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_dependency_closure_gate",
        family: "dependency_path",
        path: "receipts/p02/pass_0082_bootstrap_closure.receipt",
        covers: &["P02-024"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_dependency_dependency_matrix",
        family: "dependency_path",
        path: "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        covers: &["P02-X01"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_dependency_packaging_deployment",
        family: "dependency_path",
        path: "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        covers: &["P02-019", "P02-020"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapProofReceiptDescriptor {
        id: "receipt_dependency_economics_redteam",
        family: "dependency_path",
        path: "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        covers: &["P02-022", "P02-023"],
        verdict: "accepted",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_PROOF_PATHS: &[BootstrapProofPathDescriptor] = &[
    BootstrapProofPathDescriptor {
        id: "bootstrap_happy_primary_chain",
        family: "happy_path",
        path_kind: "happy_path",
        entry_receipts: &[
            "receipt_happy_p02_001",
            "receipt_happy_p02_013",
            "receipt_happy_p02_015",
        ],
        challenge_receipts: &["receipt_happy_p02_016", "receipt_happy_p02_023"],
        rollback_receipts: &["receipt_happy_p02_024", "receipt_happy_p02_x01"],
        status: "artifact_emitted",
    },
    BootstrapProofPathDescriptor {
        id: "bootstrap_negative_rejection_chain",
        family: "negative_path",
        path_kind: "negative_path",
        entry_receipts: &[
            "receipt_negative_truth_cleanup_rejection",
            "receipt_negative_host_boundary_rejection",
        ],
        challenge_receipts: &[
            "receipt_negative_falsification_rejection",
            "receipt_negative_redteam_rejection",
        ],
        rollback_receipts: &["receipt_negative_dependency_matrix_rejection"],
        status: "artifact_emitted",
    },
    BootstrapProofPathDescriptor {
        id: "bootstrap_adversarial_host_chain",
        family: "adversarial_path",
        path_kind: "adversarial_path",
        entry_receipts: &[
            "receipt_adversarial_host_boundary_challenge",
            "receipt_adversarial_emergency_fallback_challenge",
        ],
        challenge_receipts: &[
            "receipt_adversarial_foreign_surface_challenge",
            "receipt_adversarial_economics_capture_challenge",
        ],
        rollback_receipts: &["receipt_adversarial_redteam_attack_challenge"],
        status: "artifact_emitted",
    },
    BootstrapProofPathDescriptor {
        id: "bootstrap_rollback_replay_chain",
        family: "rollback_path",
        path_kind: "rollback_path",
        entry_receipts: &[
            "receipt_rollback_extinction_rollback",
            "receipt_rollback_seed_contract_rollback",
        ],
        challenge_receipts: &[
            "receipt_rollback_foreign_surface_rollback",
            "receipt_rollback_redteam_rollback",
        ],
        rollback_receipts: &[
            "receipt_rollback_replay_rollback",
            "receipt_rollback_closure_gate_rollback",
            "receipt_rollback_dependency_matrix_rollback",
        ],
        status: "artifact_emitted",
    },
    BootstrapProofPathDescriptor {
        id: "bootstrap_dependency_matrix_chain",
        family: "dependency_path",
        path_kind: "dependency_path",
        entry_receipts: &[
            "receipt_dependency_evidence_emission",
            "receipt_dependency_replay_bridge",
        ],
        challenge_receipts: &[
            "receipt_dependency_economics_redteam",
            "receipt_dependency_packaging_deployment",
        ],
        rollback_receipts: &[
            "receipt_dependency_closure_gate",
            "receipt_dependency_dependency_matrix",
        ],
        status: "artifact_emitted",
    },
];

fn family_preimage(item: &BootstrapProofFamilyDescriptor) -> String {
    format!(
        "family:{}|kind:{}|scope:{}|receipts:{}|covers:{}|proofs:{}|status:{}",
        item.id,
        item.family_kind,
        item.scope,
        item.receipts.join(","),
        item.covers.join(","),
        item.proofs.join(","),
        item.status
    )
}
fn receipt_preimage(item: &BootstrapProofReceiptDescriptor) -> String {
    format!(
        "receipt:{}|family:{}|path:{}|covers:{}|verdict:{}|status:{}",
        item.id,
        item.family,
        item.path,
        item.covers.join(","),
        item.verdict,
        item.status
    )
}
fn path_preimage(item: &BootstrapProofPathDescriptor) -> String {
    format!(
        "path:{}|family:{}|kind:{}|entries:{}|challenges:{}|rollbacks:{}|status:{}",
        item.id,
        item.family,
        item.path_kind,
        item.entry_receipts.join(","),
        item.challenge_receipts.join(","),
        item.rollback_receipts.join(","),
        item.status
    )
}
fn artifact_preimage(item: &BootstrapProofFamilyArtifactDescriptor) -> String {
    format!(
        "artifact:{}|owner_root:{}|path:{}|role:{}",
        item.id, item.owner_root, item.path, item.role
    )
}

pub fn bootstrap_proof_family_artifact_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PROOF_FAMILY_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_proof_family_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PROOF_FAMILIES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_proof_receipt_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PROOF_RECEIPTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_proof_path_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PROOF_PATHS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_proof_family_descriptor(
    id: &str,
) -> Option<&'static BootstrapProofFamilyDescriptor> {
    LYRALANG_BOOTSTRAP_PROOF_FAMILIES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_proof_receipt_descriptor(
    id: &str,
) -> Option<&'static BootstrapProofReceiptDescriptor> {
    LYRALANG_BOOTSTRAP_PROOF_RECEIPTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_proof_path_descriptor(id: &str) -> Option<&'static BootstrapProofPathDescriptor> {
    LYRALANG_BOOTSTRAP_PROOF_PATHS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_proof_family_digest(id: &str) -> Option<String> {
    bootstrap_proof_family_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_proof_family.family_descriptor",
            &family_preimage(item),
        )
    })
}
pub fn bootstrap_proof_receipt_digest(id: &str) -> Option<String> {
    bootstrap_proof_receipt_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_proof_family.receipt_descriptor",
            &receipt_preimage(item),
        )
    })
}
pub fn bootstrap_proof_path_digest(id: &str) -> Option<String> {
    bootstrap_proof_path_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_proof_family.path_descriptor",
            &path_preimage(item),
        )
    })
}

pub fn bootstrap_proof_family_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_PROOF_FAMILY_ARTIFACTS
        .iter()
        .all(|item| {
            !item.id.is_empty()
                && !item.owner_root.is_empty()
                && !item.path.is_empty()
                && !item.role.is_empty()
        })
}
pub fn bootstrap_proof_family_receipts_bind_families() -> bool {
    LYRALANG_BOOTSTRAP_PROOF_RECEIPTS.iter().all(|receipt| {
        bootstrap_proof_family_descriptor(receipt.family).is_some()
            && receipt.path.starts_with("receipts/p02/")
            && receipt.path.ends_with(".receipt")
            && !receipt.covers.is_empty()
    })
}
pub fn bootstrap_proof_family_paths_bind_receipts() -> bool {
    LYRALANG_BOOTSTRAP_PROOF_PATHS.iter().all(|path| {
        bootstrap_proof_family_descriptor(path.family).is_some()
            && path.family == path.path_kind
            && !path.entry_receipts.is_empty()
            && !path.challenge_receipts.is_empty()
            && !path.rollback_receipts.is_empty()
            && path
                .entry_receipts
                .iter()
                .chain(path.challenge_receipts.iter())
                .chain(path.rollback_receipts.iter())
                .all(|id| bootstrap_proof_receipt_descriptor(id).is_some())
    })
}
pub fn bootstrap_proof_family_families_bind_receipts() -> bool {
    LYRALANG_BOOTSTRAP_PROOF_FAMILIES.iter().all(|family| {
        family.scope == "P02"
            && family.id == family.family_kind
            && family.receipts.len() >= 3
            && !family.covers.is_empty()
            && family.receipts.iter().all(|id| {
                bootstrap_proof_receipt_descriptor(id)
                    .map(|receipt| receipt.family == family.id)
                    .unwrap_or(false)
            })
    })
}
pub fn bootstrap_proof_family_receipts_cover_p02_001_through_p02_x01() -> bool {
    let required = &[
        "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
        "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
        "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
        "P02-X01",
    ];
    required.iter().all(|target| {
        LYRALANG_BOOTSTRAP_PROOF_RECEIPTS
            .iter()
            .any(|receipt| receipt.covers.iter().any(|cover| cover == target))
    })
}
pub fn bootstrap_proof_family_no_forbidden_descriptor_claims() -> bool {
    let forbidden = [
        "network_required",
        "remote_service_required",
        "docs_only",
        "unreceipted",
        "global_closure",
        "phase_closed",
    ];
    let blob = [
        LYRALANG_BOOTSTRAP_PROOF_FAMILY_ARTIFACTS
            .iter()
            .map(artifact_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
        LYRALANG_BOOTSTRAP_PROOF_FAMILIES
            .iter()
            .map(family_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
        LYRALANG_BOOTSTRAP_PROOF_RECEIPTS
            .iter()
            .map(receipt_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
        LYRALANG_BOOTSTRAP_PROOF_PATHS
            .iter()
            .map(path_preimage)
            .collect::<Vec<_>>()
            .join(
                "
",
            ),
    ]
    .join(
        "
",
    );
    forbidden.iter().all(|token| !blob.contains(token))
}
pub fn bootstrap_proof_family_registry_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_proof_family.registry",
        &[
            LYRALANG_BOOTSTRAP_PROOF_FAMILY_ARTIFACTS
                .iter()
                .map(artifact_preimage)
                .collect::<Vec<_>>()
                .join(
                    "
",
                ),
            LYRALANG_BOOTSTRAP_PROOF_FAMILIES
                .iter()
                .map(family_preimage)
                .collect::<Vec<_>>()
                .join(
                    "
",
                ),
            LYRALANG_BOOTSTRAP_PROOF_RECEIPTS
                .iter()
                .map(receipt_preimage)
                .collect::<Vec<_>>()
                .join(
                    "
",
                ),
            LYRALANG_BOOTSTRAP_PROOF_PATHS
                .iter()
                .map(path_preimage)
                .collect::<Vec<_>>()
                .join(
                    "
",
                ),
        ]
        .join(
            "
",
        ),
    )
}
pub fn bootstrap_proof_family_registry_hash() -> String {
    bootstrap_proof_family_registry_signature()
}
pub fn bootstrap_proof_family_carrier_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_proof_family.carrier",
        LYRA_P02_BOOTSTRAP_PROOF_FAMILY_CARRIER,
    )
}
