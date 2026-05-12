use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticDeploymentTargetDescriptor {
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
pub struct SemanticComplianceHookDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub target: &'static str,
    pub requires: &'static [&'static str],
    pub evidence: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticReleaseEvidenceDescriptor {
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
pub struct SemanticDeploymentProofDescriptor {
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

pub const LYRA_P01_SEMANTIC_DEPLOYMENT_CARRIER: &str = "lyra.p01.semantic_deployment.carrier.v1";

pub const LYRALANG_SEMANTIC_DEPLOYMENT_TARGETS: &[SemanticDeploymentTargetDescriptor] = &[
    SemanticDeploymentTargetDescriptor {
        id: "semantic_local_workstation_deployment",
        kind: "workstation",
        environment: "offline",
        artifacts: &[
            "src/bin/lyra-p01-semantic-deployment-check.rs",
            "ops/p01/src/semantic_deployment.rs",
            "interfaces/p01/contracts/semantic_deployment.v1.lyra",
        ],
        commands: &[
            "lyra-p01-semantic-deployment-check",
            "lyra-p01-semantic-packaging-check",
        ],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        forbids: &["network_required", "remote_service", "ambient_randomness"],
        status: "artifact_emitted",
    },
    SemanticDeploymentTargetDescriptor {
        id: "semantic_airgap_archive_deployment",
        kind: "archive",
        environment: "airgap",
        artifacts: &[
            "goldens/p01/valid_semantic_deployment.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "products/p01/semantic_deployment_manifest.lyra",
        ],
        commands: &["lyra-p01-semantic-deployment-check"],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        forbids: &["network_required", "remote_service", "unreceipted_archive"],
        status: "artifact_emitted",
    },
    SemanticDeploymentTargetDescriptor {
        id: "semantic_sovereign_site_review",
        kind: "site",
        environment: "sovereign",
        artifacts: &[
            "products/p01/semantic_deployment_manifest.lyra",
            "products/p01/semantic_deployment_inspection_surface.lyra",
            "examples/p01/operator/semantic_deployment_review.lyra",
        ],
        commands: &["lyra-p01-semantic-deployment-check"],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        forbids: &["network_required", "remote_service", "cloud_dependency"],
        status: "artifact_emitted",
    },
    SemanticDeploymentTargetDescriptor {
        id: "semantic_enterprise_operator_review",
        kind: "review",
        environment: "offline",
        artifacts: &[
            "docs/p01/semantic_deployment_guide.lyra",
            "examples/p01/operator/semantic_deployment_review.lyra",
            "products/p01/semantic_deployment_manifest.lyra",
        ],
        commands: &[
            "lyra-p01-semantic-deployment-check",
            "lyra-p01-semantic-interface-check",
            "lyra-p01-semantic-packaging-check",
        ],
        receipts: &[
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        forbids: &["network_required", "remote_service", "manual_only_release"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_DEPLOYMENT_HOOKS: &[SemanticComplianceHookDescriptor] = &[
    SemanticComplianceHookDescriptor {
        id: "semantic_artifact_inventory_check",
        scope: "target",
        target: "semantic_local_workstation_deployment",
        requires: &["artifact_paths_bound", "owner_root_bound"],
        evidence: &["semantic_deployment_manifest", "artifact_hash_manifest"],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        status: "artifact_emitted",
    },
    SemanticComplianceHookDescriptor {
        id: "semantic_receipt_chain_gate",
        scope: "release",
        target: "P01",
        requires: &["receipt_chain_present", "packaging_receipt_present"],
        evidence: &["offline_install_receipt", "command_matrix_receipt"],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        status: "artifact_emitted",
    },
    SemanticComplianceHookDescriptor {
        id: "semantic_negative_corpus_gate",
        scope: "compliance",
        target: "semantic_enterprise_operator_review",
        requires: &["negative_corpus_present", "falsification_receipt_present"],
        evidence: &["operator_review_record", "artifact_hash_manifest"],
        receipts: &[
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        status: "artifact_emitted",
    },
    SemanticComplianceHookDescriptor {
        id: "semantic_offline_install_gate",
        scope: "release",
        target: "semantic_airgap_archive_deployment",
        requires: &["offline_artifacts_present", "no_remote_fetch"],
        evidence: &["offline_install_receipt", "semantic_deployment_manifest"],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        status: "artifact_emitted",
    },
    SemanticComplianceHookDescriptor {
        id: "semantic_rollout_replay_gate",
        scope: "rollback",
        target: "semantic_sovereign_site_review",
        requires: &["replay_witness_present", "rollback_path_rehearsed"],
        evidence: &["rollback_rehearsal_receipt", "command_matrix_receipt"],
        receipts: &[
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        status: "artifact_emitted",
    },
    SemanticComplianceHookDescriptor {
        id: "semantic_phase_open_gate",
        scope: "phase",
        target: "P01",
        requires: &["closure_claim_denied", "remaining_tasks_declared"],
        evidence: &["operator_review_record", "semantic_deployment_manifest"],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        status: "blocked",
    },
];

pub const LYRALANG_SEMANTIC_DEPLOYMENT_EVIDENCE: &[SemanticReleaseEvidenceDescriptor] = &[
    SemanticReleaseEvidenceDescriptor {
        id: "semantic_deployment_manifest",
        kind: "manifest",
        path: "products/p01/semantic_deployment_manifest.lyra",
        targets: &[
            "semantic_local_workstation_deployment",
            "semantic_airgap_archive_deployment",
            "semantic_sovereign_site_review",
            "semantic_enterprise_operator_review",
        ],
        hooks: &[
            "semantic_artifact_inventory_check",
            "semantic_offline_install_gate",
            "semantic_phase_open_gate",
        ],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        commands: &["lyra-p01-semantic-deployment-check"],
        status: "artifact_emitted",
    },
    SemanticReleaseEvidenceDescriptor {
        id: "artifact_hash_manifest",
        kind: "manifest",
        path: "products/p01/semantic_deployment_inspection_surface.lyra",
        targets: &[
            "semantic_local_workstation_deployment",
            "semantic_enterprise_operator_review",
        ],
        hooks: &[
            "semantic_artifact_inventory_check",
            "semantic_negative_corpus_gate",
        ],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        commands: &["lyra-p01-semantic-deployment-check"],
        status: "artifact_emitted",
    },
    SemanticReleaseEvidenceDescriptor {
        id: "command_matrix_receipt",
        kind: "matrix",
        path: "receipts/p01/pass_0049_semantic_deployment.receipt",
        targets: &[
            "semantic_local_workstation_deployment",
            "semantic_airgap_archive_deployment",
            "semantic_enterprise_operator_review",
        ],
        hooks: &[
            "semantic_receipt_chain_gate",
            "semantic_rollout_replay_gate",
        ],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        commands: &[
            "lyra-p01-semantic-deployment-check",
            "lyra-p01-semantic-packaging-check",
        ],
        status: "artifact_emitted",
    },
    SemanticReleaseEvidenceDescriptor {
        id: "operator_review_record",
        kind: "record",
        path: "examples/p01/operator/semantic_deployment_review.lyra",
        targets: &[
            "semantic_sovereign_site_review",
            "semantic_enterprise_operator_review",
        ],
        hooks: &["semantic_negative_corpus_gate", "semantic_phase_open_gate"],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        commands: &["lyra-p01-semantic-deployment-check"],
        status: "artifact_emitted",
    },
    SemanticReleaseEvidenceDescriptor {
        id: "rollback_rehearsal_receipt",
        kind: "rehearsal",
        path: "receipts/p01/pass_0049_semantic_deployment.receipt",
        targets: &[
            "semantic_airgap_archive_deployment",
            "semantic_sovereign_site_review",
        ],
        hooks: &["semantic_rollout_replay_gate"],
        receipts: &[
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-deployment-check",
            "lyra-p01-semantic-replay-check",
        ],
        status: "artifact_emitted",
    },
    SemanticReleaseEvidenceDescriptor {
        id: "offline_install_receipt",
        kind: "receipt",
        path: "receipts/p01/pass_0049_semantic_deployment.receipt",
        targets: &[
            "semantic_local_workstation_deployment",
            "semantic_airgap_archive_deployment",
        ],
        hooks: &[
            "semantic_receipt_chain_gate",
            "semantic_offline_install_gate",
        ],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        commands: &["lyra-p01-semantic-deployment-check"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_DEPLOYMENT_PROOFS: &[SemanticDeploymentProofDescriptor] = &[
    SemanticDeploymentProofDescriptor {
        id: "semantic_target_coverage",
        scope: "target",
        targets: &[
            "semantic_local_workstation_deployment",
            "semantic_airgap_archive_deployment",
            "semantic_sovereign_site_review",
            "semantic_enterprise_operator_review",
        ],
        hooks: &[
            "semantic_artifact_inventory_check",
            "semantic_offline_install_gate",
        ],
        evidence: &["semantic_deployment_manifest", "artifact_hash_manifest"],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        commands: &["lyra-p01-semantic-deployment-check"],
        forbids: &["missing_target", "unowned_artifact_path"],
        status: "artifact_emitted",
    },
    SemanticDeploymentProofDescriptor {
        id: "semantic_offline_deployment_gate",
        scope: "release",
        targets: &[
            "semantic_local_workstation_deployment",
            "semantic_airgap_archive_deployment",
        ],
        hooks: &[
            "semantic_receipt_chain_gate",
            "semantic_offline_install_gate",
        ],
        evidence: &["offline_install_receipt", "semantic_deployment_manifest"],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-deployment-check",
            "lyra-p01-semantic-packaging-check",
        ],
        forbids: &["network_dependency", "cloud_dependency", "remote_service"],
        status: "artifact_emitted",
    },
    SemanticDeploymentProofDescriptor {
        id: "semantic_compliance_hook_binding",
        scope: "compliance",
        targets: &[
            "semantic_enterprise_operator_review",
            "semantic_sovereign_site_review",
        ],
        hooks: &[
            "semantic_artifact_inventory_check",
            "semantic_negative_corpus_gate",
            "semantic_rollout_replay_gate",
        ],
        evidence: &[
            "artifact_hash_manifest",
            "operator_review_record",
            "rollback_rehearsal_receipt",
        ],
        receipts: &[
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        commands: &["lyra-p01-semantic-deployment-check"],
        forbids: &["compliance_hook_bypass"],
        status: "artifact_emitted",
    },
    SemanticDeploymentProofDescriptor {
        id: "semantic_release_evidence_replay",
        scope: "rollback",
        targets: &[
            "semantic_airgap_archive_deployment",
            "semantic_sovereign_site_review",
        ],
        hooks: &[
            "semantic_receipt_chain_gate",
            "semantic_rollout_replay_gate",
        ],
        evidence: &[
            "command_matrix_receipt",
            "rollback_rehearsal_receipt",
            "offline_install_receipt",
        ],
        receipts: &[
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-deployment-check",
            "lyra-p01-semantic-replay-check",
        ],
        forbids: &["unreceipted_rollback", "release_drift"],
        status: "artifact_emitted",
    },
    SemanticDeploymentProofDescriptor {
        id: "semantic_packaging_bridge",
        scope: "release",
        targets: &[
            "semantic_local_workstation_deployment",
            "semantic_airgap_archive_deployment",
        ],
        hooks: &[
            "semantic_receipt_chain_gate",
            "semantic_offline_install_gate",
        ],
        evidence: &["offline_install_receipt", "command_matrix_receipt"],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-deployment-check",
            "lyra-p01-semantic-packaging-check",
        ],
        forbids: &["package_drift", "unreceipted_package_action"],
        status: "artifact_emitted",
    },
    SemanticDeploymentProofDescriptor {
        id: "p01_phase_open",
        scope: "phase",
        targets: &["semantic_enterprise_operator_review"],
        hooks: &["semantic_phase_open_gate"],
        evidence: &["operator_review_record", "semantic_deployment_manifest"],
        receipts: &["receipts/p01/pass_0049_semantic_deployment.receipt"],
        commands: &["lyra-p01-semantic-deployment-check"],
        forbids: &["phase_closure", "global_complete"],
        status: "blocked",
    },
];

pub fn semantic_deployment_target_descriptor(
    id: &str,
) -> Option<&'static SemanticDeploymentTargetDescriptor> {
    LYRALANG_SEMANTIC_DEPLOYMENT_TARGETS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_deployment_hook_descriptor(
    id: &str,
) -> Option<&'static SemanticComplianceHookDescriptor> {
    LYRALANG_SEMANTIC_DEPLOYMENT_HOOKS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_deployment_evidence_descriptor(
    id: &str,
) -> Option<&'static SemanticReleaseEvidenceDescriptor> {
    LYRALANG_SEMANTIC_DEPLOYMENT_EVIDENCE
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_deployment_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticDeploymentProofDescriptor> {
    LYRALANG_SEMANTIC_DEPLOYMENT_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_deployment_target_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_DEPLOYMENT_TARGETS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_deployment_hook_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_DEPLOYMENT_HOOKS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_deployment_evidence_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_DEPLOYMENT_EVIDENCE
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_deployment_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_DEPLOYMENT_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_deployment_target_signature(item: &SemanticDeploymentTargetDescriptor) -> String {
    format!("target:{}|kind:{}|environment:{}|artifacts:{}|commands:{}|receipts:{}|forbids:{}|status:{}", item.id, item.kind, item.environment, join(item.artifacts), join(item.commands), join(item.receipts), join(item.forbids), item.status)
}
pub fn semantic_deployment_hook_signature(item: &SemanticComplianceHookDescriptor) -> String {
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
pub fn semantic_deployment_evidence_signature(item: &SemanticReleaseEvidenceDescriptor) -> String {
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
pub fn semantic_deployment_proof_signature(item: &SemanticDeploymentProofDescriptor) -> String {
    format!("proof:{}|scope:{}|targets:{}|hooks:{}|evidence:{}|receipts:{}|commands:{}|forbids:{}|status:{}", item.id, item.scope, join(item.targets), join(item.hooks), join(item.evidence), join(item.receipts), join(item.commands), join(item.forbids), item.status)
}

pub fn semantic_deployment_target_digest(id: &str) -> Option<String> {
    semantic_deployment_target_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_deployment.target",
            &semantic_deployment_target_signature(item),
        )
    })
}
pub fn semantic_deployment_hook_digest(id: &str) -> Option<String> {
    semantic_deployment_hook_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_deployment.hook",
            &semantic_deployment_hook_signature(item),
        )
    })
}
pub fn semantic_deployment_evidence_digest(id: &str) -> Option<String> {
    semantic_deployment_evidence_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_deployment.evidence",
            &semantic_deployment_evidence_signature(item),
        )
    })
}
pub fn semantic_deployment_proof_digest(id: &str) -> Option<String> {
    semantic_deployment_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_deployment.proof",
            &semantic_deployment_proof_signature(item),
        )
    })
}

pub fn semantic_deployment_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SEMANTIC_DEPLOYMENT_TARGETS {
        rows.push(semantic_deployment_target_signature(item));
    }
    for item in LYRALANG_SEMANTIC_DEPLOYMENT_HOOKS {
        rows.push(semantic_deployment_hook_signature(item));
    }
    for item in LYRALANG_SEMANTIC_DEPLOYMENT_EVIDENCE {
        rows.push(semantic_deployment_evidence_signature(item));
    }
    for item in LYRALANG_SEMANTIC_DEPLOYMENT_PROOFS {
        rows.push(semantic_deployment_proof_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_deployment_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_deployment.registry",
        &semantic_deployment_registry_signature(),
    )
}

pub fn semantic_deployment_artifacts_bind_paths() -> bool {
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
    ];
    LYRALANG_SEMANTIC_DEPLOYMENT_TARGETS.iter().all(|target| {
        target.artifacts.iter().all(|path| {
            !path.contains("..") && allowed.iter().any(|prefix| path.starts_with(prefix))
        })
    })
}

pub fn semantic_deployment_hooks_bind_targets() -> bool {
    LYRALANG_SEMANTIC_DEPLOYMENT_HOOKS.iter().all(|hook| {
        hook.target == "P01" || semantic_deployment_target_descriptor(hook.target).is_some()
    })
}

pub fn semantic_deployment_evidence_bind_registry() -> bool {
    LYRALANG_SEMANTIC_DEPLOYMENT_EVIDENCE.iter().all(|item| {
        item.targets
            .iter()
            .all(|target| semantic_deployment_target_descriptor(target).is_some())
            && item
                .hooks
                .iter()
                .all(|hook| semantic_deployment_hook_descriptor(hook).is_some())
    })
}

pub fn semantic_deployment_proofs_bind_registry() -> bool {
    LYRALANG_SEMANTIC_DEPLOYMENT_PROOFS.iter().all(|proof| {
        proof
            .targets
            .iter()
            .all(|target| semantic_deployment_target_descriptor(target).is_some())
            && proof
                .hooks
                .iter()
                .all(|hook| semantic_deployment_hook_descriptor(hook).is_some())
            && proof
                .evidence
                .iter()
                .all(|item| semantic_deployment_evidence_descriptor(item).is_some())
    })
}

pub fn semantic_deployment_receipts_cover_p01_001_through_p01_020() -> bool {
    let mut receipts = Vec::new();
    for target in LYRALANG_SEMANTIC_DEPLOYMENT_TARGETS {
        receipts.extend_from_slice(target.receipts);
    }
    for hook in LYRALANG_SEMANTIC_DEPLOYMENT_HOOKS {
        receipts.extend_from_slice(hook.receipts);
    }
    for item in LYRALANG_SEMANTIC_DEPLOYMENT_EVIDENCE {
        receipts.extend_from_slice(item.receipts);
    }
    for proof in LYRALANG_SEMANTIC_DEPLOYMENT_PROOFS {
        receipts.extend_from_slice(proof.receipts);
    }
    [
        "receipts/p01/pass_0045_semantic_falsification.receipt",
        "receipts/p01/pass_0046_semantic_replay.receipt",
        "receipts/p01/pass_0047_semantic_interface.receipt",
        "receipts/p01/pass_0048_semantic_packaging.receipt",
        "receipts/p01/pass_0049_semantic_deployment.receipt",
    ]
    .iter()
    .all(|required| receipts.contains(required))
}

pub fn semantic_deployment_no_forbidden_descriptor_claims() -> bool {
    let lowered = semantic_deployment_registry_signature().to_ascii_lowercase();
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
