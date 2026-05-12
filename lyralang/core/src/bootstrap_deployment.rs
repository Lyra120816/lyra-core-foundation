use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapDeploymentTargetDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub environment: &'static str,
    pub artifacts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapComplianceHookDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub target: &'static str,
    pub requires: &'static [&'static str],
    pub evidence: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapReleaseEvidenceDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub targets: &'static [&'static str],
    pub hooks: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapDeploymentProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub targets: &'static [&'static str],
    pub hooks: &'static [&'static str],
    pub evidence: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_DEPLOYMENT_CARRIER: &str = "lyra.p02.bootstrap_deployment.carrier.v1";

pub const LYRALANG_BOOTSTRAP_DEPLOYMENT_TARGETS: &[BootstrapDeploymentTargetDescriptor] = &[
    BootstrapDeploymentTargetDescriptor {
        id: "p02_local_bootstrap_deployment",
        kind: "workstation",
        environment: "offline",
        artifacts: &[
            "src/bin/lyra-p02-bootstrap-deployment-check.rs",
            "ops/p02/src/bootstrap_deployment.rs",
            "interfaces/p02/contracts/bootstrap_deployment.v1.lyra",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-interface-check",
        ],
        receipts: &[
            "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        forbids: &["network_required", "remote_service", "ambient_randomness"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentTargetDescriptor {
        id: "p02_airgap_distribution_deployment",
        kind: "archive",
        environment: "airgap",
        artifacts: &[
            "goldens/p02/valid_bootstrap_deployment.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "products/p02/bootstrap_deployment_manifest.v1.lyra",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-packaging-check",
        ],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        forbids: &["network_required", "remote_service", "unreceipted_archive"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentTargetDescriptor {
        id: "p02_sovereign_site_deployment",
        kind: "site",
        environment: "sovereign",
        artifacts: &[
            "products/p02/bootstrap_deployment_manifest.v1.lyra",
            "products/p02/bootstrap_deployment_inspection_surface.v1.lyra",
            "examples/p02/operator/bootstrap_deployment_review.v1.lyra",
        ],
        commands: &["lyra-p02-bootstrap-deployment-check"],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        forbids: &["cloud_dependency", "network_required", "remote_service"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentTargetDescriptor {
        id: "p02_enterprise_operator_deployment",
        kind: "review",
        environment: "offline",
        artifacts: &[
            "docs/p02/bootstrap_deployment_guide.v1.lyra",
            "examples/p02/operator/bootstrap_deployment_review.v1.lyra",
            "products/p02/bootstrap_deployment_manifest.v1.lyra",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-interface-check",
            "lyra-p02-bootstrap-packaging-check",
        ],
        receipts: &[
            "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        forbids: &["manual_only_release", "network_required", "remote_service"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentTargetDescriptor {
        id: "p02_host_extinction_deployment",
        kind: "extinction_lane",
        environment: "offline",
        artifacts: &[
            "ops/p02/foreign_surface_closure/foreign_surface_closure_law.v1.lyra",
            "receipts/p02/foreign_surface_closure/foreign_rust_toolchain_retirement.receipt",
            "products/p02/bootstrap_deployment_manifest.v1.lyra",
        ],
        commands: &[
            "lyra-p02-foreign-surface-closure-check",
            "lyra-p02-bootstrap-deployment-check",
        ],
        receipts: &[
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        forbids: &[
            "foreign_surface_untracked",
            "hidden_host_dependency",
            "network_required",
        ],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_DEPLOYMENT_HOOKS: &[BootstrapComplianceHookDescriptor] = &[
    BootstrapComplianceHookDescriptor {
        id: "bootstrap_artifact_inventory_gate",
        scope: "target",
        target: "p02_local_bootstrap_deployment",
        requires: &["artifact_paths_bound", "owner_root_bound"],
        evidence: &["bootstrap_deployment_manifest", "artifact_hash_manifest"],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        status: "artifact_emitted",
    },
    BootstrapComplianceHookDescriptor {
        id: "bootstrap_receipt_chain_gate",
        scope: "release",
        target: "P02",
        requires: &["packaging_receipt_present", "receipt_chain_present"],
        evidence: &["offline_install_receipt", "command_matrix_receipt"],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapComplianceHookDescriptor {
        id: "host_extinction_enterprise_gate",
        scope: "enterprise",
        target: "p02_host_extinction_deployment",
        requires: &[
            "challenge_receipts_present",
            "foreign_surface_closure_present",
        ],
        evidence: &["host_extinction_release_record", "artifact_hash_manifest"],
        receipts: &[
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapComplianceHookDescriptor {
        id: "seed_runtime_deployment_gate",
        scope: "compliance",
        target: "p02_local_bootstrap_deployment",
        requires: &[
            "replacement_milestone_present",
            "seed_runtime_contract_present",
        ],
        evidence: &[
            "seed_runtime_release_record",
            "bootstrap_deployment_manifest",
        ],
        receipts: &[
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapComplianceHookDescriptor {
        id: "offline_install_gate",
        scope: "release",
        target: "p02_airgap_distribution_deployment",
        requires: &["no_remote_fetch", "offline_artifacts_present"],
        evidence: &["offline_install_receipt", "bootstrap_deployment_manifest"],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        status: "artifact_emitted",
    },
    BootstrapComplianceHookDescriptor {
        id: "rollout_replay_gate",
        scope: "rollback",
        target: "p02_sovereign_site_deployment",
        requires: &["replay_witness_present", "rollback_path_rehearsed"],
        evidence: &["rollback_rehearsal_receipt", "command_matrix_receipt"],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapComplianceHookDescriptor {
        id: "p02_phase_open_gate",
        scope: "phase",
        target: "P02",
        requires: &["closure_claim_denied", "remaining_tasks_declared"],
        evidence: &["operator_review_record", "bootstrap_deployment_manifest"],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        status: "blocked",
    },
];

pub const LYRALANG_BOOTSTRAP_DEPLOYMENT_EVIDENCE: &[BootstrapReleaseEvidenceDescriptor] = &[
    BootstrapReleaseEvidenceDescriptor {
        id: "bootstrap_deployment_manifest",
        kind: "manifest",
        path: "products/p02/bootstrap_deployment_manifest.v1.lyra",
        targets: &[
            "p02_local_bootstrap_deployment",
            "p02_airgap_distribution_deployment",
            "p02_sovereign_site_deployment",
            "p02_enterprise_operator_deployment",
            "p02_host_extinction_deployment",
        ],
        hooks: &[
            "bootstrap_artifact_inventory_gate",
            "offline_install_gate",
            "p02_phase_open_gate",
            "seed_runtime_deployment_gate",
        ],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        commands: &["lyra-p02-bootstrap-deployment-check"],
        status: "artifact_emitted",
    },
    BootstrapReleaseEvidenceDescriptor {
        id: "artifact_hash_manifest",
        kind: "manifest",
        path: "products/p02/bootstrap_deployment_inspection_surface.v1.lyra",
        targets: &[
            "p02_local_bootstrap_deployment",
            "p02_enterprise_operator_deployment",
            "p02_host_extinction_deployment",
        ],
        hooks: &[
            "bootstrap_artifact_inventory_gate",
            "host_extinction_enterprise_gate",
        ],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        commands: &["lyra-p02-bootstrap-deployment-check"],
        status: "artifact_emitted",
    },
    BootstrapReleaseEvidenceDescriptor {
        id: "command_matrix_receipt",
        kind: "matrix",
        path: "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        targets: &[
            "p02_local_bootstrap_deployment",
            "p02_airgap_distribution_deployment",
            "p02_enterprise_operator_deployment",
        ],
        hooks: &["bootstrap_receipt_chain_gate", "rollout_replay_gate"],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-packaging-check",
        ],
        status: "artifact_emitted",
    },
    BootstrapReleaseEvidenceDescriptor {
        id: "operator_review_record",
        kind: "record",
        path: "examples/p02/operator/bootstrap_deployment_review.v1.lyra",
        targets: &[
            "p02_sovereign_site_deployment",
            "p02_enterprise_operator_deployment",
        ],
        hooks: &["p02_phase_open_gate"],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        commands: &["lyra-p02-bootstrap-deployment-check"],
        status: "artifact_emitted",
    },
    BootstrapReleaseEvidenceDescriptor {
        id: "rollback_rehearsal_receipt",
        kind: "rehearsal",
        path: "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        targets: &[
            "p02_airgap_distribution_deployment",
            "p02_sovereign_site_deployment",
        ],
        hooks: &["rollout_replay_gate"],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-replay-check",
        ],
        status: "artifact_emitted",
    },
    BootstrapReleaseEvidenceDescriptor {
        id: "offline_install_receipt",
        kind: "receipt",
        path: "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        targets: &[
            "p02_local_bootstrap_deployment",
            "p02_airgap_distribution_deployment",
        ],
        hooks: &["bootstrap_receipt_chain_gate", "offline_install_gate"],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        commands: &["lyra-p02-bootstrap-deployment-check"],
        status: "artifact_emitted",
    },
    BootstrapReleaseEvidenceDescriptor {
        id: "seed_runtime_release_record",
        kind: "record",
        path: "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        targets: &["p02_local_bootstrap_deployment"],
        hooks: &["seed_runtime_deployment_gate"],
        receipts: &[
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        commands: &[
            "lyra-p02-seed-runtime-replacement-check",
            "lyra-p02-bootstrap-deployment-check",
        ],
        status: "artifact_emitted",
    },
    BootstrapReleaseEvidenceDescriptor {
        id: "host_extinction_release_record",
        kind: "record",
        path: "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        targets: &[
            "p02_host_extinction_deployment",
            "p02_enterprise_operator_deployment",
        ],
        hooks: &["host_extinction_enterprise_gate"],
        receipts: &[
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        commands: &[
            "lyra-p02-foreign-surface-closure-check",
            "lyra-p02-bootstrap-deployment-check",
        ],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_DEPLOYMENT_PROOFS: &[BootstrapDeploymentProofDescriptor] = &[
    BootstrapDeploymentProofDescriptor {
        id: "bootstrap_target_coverage",
        scope: "target",
        targets: &[
            "p02_local_bootstrap_deployment",
            "p02_airgap_distribution_deployment",
            "p02_sovereign_site_deployment",
            "p02_enterprise_operator_deployment",
            "p02_host_extinction_deployment",
        ],
        hooks: &[
            "bootstrap_artifact_inventory_gate",
            "host_extinction_enterprise_gate",
            "seed_runtime_deployment_gate",
        ],
        evidence: &[
            "bootstrap_deployment_manifest",
            "artifact_hash_manifest",
            "host_extinction_release_record",
            "seed_runtime_release_record",
        ],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        commands: &["lyra-p02-bootstrap-deployment-check"],
        forbids: &["missing_target", "unowned_artifact_path"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentProofDescriptor {
        id: "bootstrap_offline_deployment_gate",
        scope: "release",
        targets: &[
            "p02_local_bootstrap_deployment",
            "p02_airgap_distribution_deployment",
        ],
        hooks: &["bootstrap_receipt_chain_gate", "offline_install_gate"],
        evidence: &["offline_install_receipt", "bootstrap_deployment_manifest"],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-packaging-check",
        ],
        forbids: &["cloud_dependency", "network_dependency", "remote_service"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentProofDescriptor {
        id: "bootstrap_enterprise_compliance_binding",
        scope: "enterprise",
        targets: &[
            "p02_enterprise_operator_deployment",
            "p02_host_extinction_deployment",
            "p02_sovereign_site_deployment",
        ],
        hooks: &[
            "bootstrap_artifact_inventory_gate",
            "host_extinction_enterprise_gate",
            "seed_runtime_deployment_gate",
            "rollout_replay_gate",
        ],
        evidence: &[
            "artifact_hash_manifest",
            "operator_review_record",
            "host_extinction_release_record",
            "seed_runtime_release_record",
            "rollback_rehearsal_receipt",
        ],
        receipts: &[
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-foreign-surface-closure-check",
        ],
        forbids: &["compliance_hook_bypass", "unbounded_enterprise_review"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentProofDescriptor {
        id: "bootstrap_release_evidence_replay",
        scope: "rollback",
        targets: &[
            "p02_airgap_distribution_deployment",
            "p02_sovereign_site_deployment",
        ],
        hooks: &["bootstrap_receipt_chain_gate", "rollout_replay_gate"],
        evidence: &[
            "command_matrix_receipt",
            "rollback_rehearsal_receipt",
            "offline_install_receipt",
        ],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-replay-check",
        ],
        forbids: &["release_drift", "unreceipted_rollback"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentProofDescriptor {
        id: "bootstrap_packaging_bridge",
        scope: "release",
        targets: &[
            "p02_local_bootstrap_deployment",
            "p02_airgap_distribution_deployment",
        ],
        hooks: &["bootstrap_receipt_chain_gate", "offline_install_gate"],
        evidence: &["offline_install_receipt", "command_matrix_receipt"],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-bootstrap-packaging-check",
        ],
        forbids: &["package_drift", "unreceipted_package_action"],
        status: "artifact_emitted",
    },
    BootstrapDeploymentProofDescriptor {
        id: "p02_phase_open",
        scope: "phase",
        targets: &["p02_enterprise_operator_deployment"],
        hooks: &["p02_phase_open_gate"],
        evidence: &["operator_review_record", "bootstrap_deployment_manifest"],
        receipts: &["receipts/p02/pass_0078_bootstrap_deployment.receipt"],
        commands: &["lyra-p02-bootstrap-deployment-check"],
        forbids: &["phase_closure", "global_complete"],
        status: "blocked",
    },
];

pub fn bootstrap_deployment_target_descriptor(
    id: &str,
) -> Option<&'static BootstrapDeploymentTargetDescriptor> {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_TARGETS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_deployment_hook_descriptor(
    id: &str,
) -> Option<&'static BootstrapComplianceHookDescriptor> {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_HOOKS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_deployment_evidence_descriptor(
    id: &str,
) -> Option<&'static BootstrapReleaseEvidenceDescriptor> {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_EVIDENCE
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_deployment_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapDeploymentProofDescriptor> {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_deployment_target_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_TARGETS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_deployment_hook_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_HOOKS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_deployment_evidence_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_EVIDENCE
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_deployment_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_deployment_target_signature(item: &BootstrapDeploymentTargetDescriptor) -> String {
    format!("target:{}|kind:{}|environment:{}|artifacts:{}|commands:{}|receipts:{}|forbids:{}|status:{}", item.id, item.kind, item.environment, join(item.artifacts), join(item.commands), join(item.receipts), join(item.forbids), item.status)
}
pub fn bootstrap_deployment_hook_signature(item: &BootstrapComplianceHookDescriptor) -> String {
    format!(
        "hook:{}|scope:{}|target:{}|requires:{}|evidence:{}|receipts:{}|status:{}",
        item.id,
        item.scope,
        item.target,
        join(item.requires),
        join(item.evidence),
        join(item.receipts),
        item.status
    )
}
pub fn bootstrap_deployment_evidence_signature(
    item: &BootstrapReleaseEvidenceDescriptor,
) -> String {
    format!(
        "evidence:{}|kind:{}|path:{}|targets:{}|hooks:{}|receipts:{}|commands:{}|status:{}",
        item.id,
        item.kind,
        item.path,
        join(item.targets),
        join(item.hooks),
        join(item.receipts),
        join(item.commands),
        item.status
    )
}
pub fn bootstrap_deployment_proof_signature(item: &BootstrapDeploymentProofDescriptor) -> String {
    format!("proof:{}|scope:{}|targets:{}|hooks:{}|evidence:{}|receipts:{}|commands:{}|forbids:{}|status:{}", item.id, item.scope, join(item.targets), join(item.hooks), join(item.evidence), join(item.receipts), join(item.commands), join(item.forbids), item.status)
}

pub fn bootstrap_deployment_target_digest(id: &str) -> Option<String> {
    bootstrap_deployment_target_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_deployment.target",
            &bootstrap_deployment_target_signature(item),
        )
    })
}
pub fn bootstrap_deployment_hook_digest(id: &str) -> Option<String> {
    bootstrap_deployment_hook_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_deployment.hook",
            &bootstrap_deployment_hook_signature(item),
        )
    })
}
pub fn bootstrap_deployment_evidence_digest(id: &str) -> Option<String> {
    bootstrap_deployment_evidence_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_deployment.evidence",
            &bootstrap_deployment_evidence_signature(item),
        )
    })
}
pub fn bootstrap_deployment_proof_digest(id: &str) -> Option<String> {
    bootstrap_deployment_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_deployment.proof",
            &bootstrap_deployment_proof_signature(item),
        )
    })
}

pub fn bootstrap_deployment_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_BOOTSTRAP_DEPLOYMENT_TARGETS {
        rows.push(bootstrap_deployment_target_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_DEPLOYMENT_HOOKS {
        rows.push(bootstrap_deployment_hook_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_DEPLOYMENT_EVIDENCE {
        rows.push(bootstrap_deployment_evidence_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_DEPLOYMENT_PROOFS {
        rows.push(bootstrap_deployment_proof_signature(item));
    }
    rows.sort();
    rows.join(
        "
",
    )
}

pub fn bootstrap_deployment_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_deployment.registry",
        &bootstrap_deployment_registry_signature(),
    )
}
pub fn bootstrap_deployment_carrier_signature() -> String {
    format!(
        "carrier:{}|registry:{}",
        LYRA_P02_BOOTSTRAP_DEPLOYMENT_CARRIER,
        bootstrap_deployment_registry_hash()
    )
}

pub fn bootstrap_deployment_artifacts_bind_paths() -> bool {
    let allowed = [
        "src/",
        "ops/",
        "interfaces/",
        "goldens/",
        "receipts/",
        "products/",
        "examples/",
        "docs/",
        "fixtures/",
        "tests/",
        "shells/",
    ];
    LYRALANG_BOOTSTRAP_DEPLOYMENT_TARGETS.iter().all(|target| {
        target.artifacts.iter().all(|path| {
            !path.contains("..") && allowed.iter().any(|prefix| path.starts_with(prefix))
        })
    })
}

pub fn bootstrap_deployment_hooks_bind_targets() -> bool {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_HOOKS.iter().all(|hook| {
        hook.target == "P02" || bootstrap_deployment_target_descriptor(hook.target).is_some()
    })
}

pub fn bootstrap_deployment_evidence_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_EVIDENCE.iter().all(|item| {
        item.targets
            .iter()
            .all(|target| bootstrap_deployment_target_descriptor(target).is_some())
            && item
                .hooks
                .iter()
                .all(|hook| bootstrap_deployment_hook_descriptor(hook).is_some())
    })
}

pub fn bootstrap_deployment_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_DEPLOYMENT_PROOFS.iter().all(|proof| {
        proof
            .targets
            .iter()
            .all(|target| bootstrap_deployment_target_descriptor(target).is_some())
            && proof
                .hooks
                .iter()
                .all(|hook| bootstrap_deployment_hook_descriptor(hook).is_some())
            && proof
                .evidence
                .iter()
                .all(|item| bootstrap_deployment_evidence_descriptor(item).is_some())
    })
}

pub fn bootstrap_deployment_receipts_cover_p02_001_through_p02_020() -> bool {
    let mut receipts = Vec::new();
    for target in LYRALANG_BOOTSTRAP_DEPLOYMENT_TARGETS {
        receipts.extend_from_slice(target.receipts);
    }
    for hook in LYRALANG_BOOTSTRAP_DEPLOYMENT_HOOKS {
        receipts.extend_from_slice(hook.receipts);
    }
    for item in LYRALANG_BOOTSTRAP_DEPLOYMENT_EVIDENCE {
        receipts.extend_from_slice(item.receipts);
    }
    for proof in LYRALANG_BOOTSTRAP_DEPLOYMENT_PROOFS {
        receipts.extend_from_slice(proof.receipts);
    }
    [
        "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        "receipts/p02/pass_0075_bootstrap_replay.receipt",
        "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
        "receipts/p02/pass_0077_bootstrap_packaging.receipt",
        "receipts/p02/pass_0078_bootstrap_deployment.receipt",
    ]
    .iter()
    .all(|required| receipts.contains(required))
}

pub fn bootstrap_deployment_no_forbidden_descriptor_claims() -> bool {
    let lowered = bootstrap_deployment_registry_signature().to_ascii_lowercase();
    ![
        "network required",
        "cloud required",
        "online required",
        "remote fetch",
        "remote service required",
        "deployment drift accepted",
        "release drift accepted",
        "phase closed",
        "global complete",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

fn join(items: &[&str]) -> String {
    items.join(",")
}
