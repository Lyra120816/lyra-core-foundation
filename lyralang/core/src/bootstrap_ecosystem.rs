use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapEcosystemDocDescriptor {
    pub id: &'static str,
    pub audience: &'static str,
    pub path: &'static str,
    pub covers: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapEcosystemExampleDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub commands: &'static [&'static str],
    pub proofs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub rejects: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapEcosystemProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub docs: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_ECOSYSTEM_CARRIER: &str = "lyra.p02.bootstrap_ecosystem.carrier.v1";

pub const LYRALANG_BOOTSTRAP_ECOSYSTEM_DOCS: &[BootstrapEcosystemDocDescriptor] = &[
    BootstrapEcosystemDocDescriptor {
        id: "bootstrap_trust_operator_guide",
        audience: "operator",
        path: "docs/p02/bootstrap_trust_operator_guide.v1.lyra",
        covers: &["bootstrap_trust", "seed_runtime_law"],
        examples: &["bootstrap_trust_walkthrough", "phase_open_ecosystem_review"],
        receipts: &["receipts/p02/pass_0079_bootstrap_ecosystem.receipt"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemDocDescriptor {
        id: "seed_runtime_law_developer_reference",
        audience: "developer",
        path: "docs/p02/seed_runtime_law_developer_reference.v1.lyra",
        covers: &["seed_runtime_law", "bootstrap_trust"],
        examples: &[
            "seed_runtime_replacement_flow",
            "negative_bootstrap_doc_drift_rejection",
        ],
        receipts: &[
            "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEcosystemDocDescriptor {
        id: "host_extinction_contributor_onboarding",
        audience: "contributor",
        path: "docs/p02/host_extinction_contributor_onboarding.v1.lyra",
        covers: &["host_extinction_framework", "bootstrap_trust"],
        examples: &[
            "host_extinction_review",
            "enterprise_deployment_to_ecosystem_handoff",
        ],
        receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEcosystemDocDescriptor {
        id: "offline_bootstrap_distribution_reference",
        audience: "steward",
        path: "docs/p02/offline_bootstrap_distribution_reference.v1.lyra",
        covers: &[
            "bootstrap_trust",
            "seed_runtime_law",
            "host_extinction_framework",
        ],
        examples: &[
            "offline_airgap_bootstrap_review",
            "bootstrap_trust_walkthrough",
        ],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEcosystemDocDescriptor {
        id: "enterprise_bootstrap_adoption_guide",
        audience: "enterprise",
        path: "docs/p02/enterprise_bootstrap_adoption_guide.v1.lyra",
        covers: &["bootstrap_trust", "host_extinction_framework"],
        examples: &[
            "enterprise_deployment_to_ecosystem_handoff",
            "phase_open_ecosystem_review",
        ],
        receipts: &[
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEcosystemDocDescriptor {
        id: "public_bootstrap_review_reference",
        audience: "public",
        path: "docs/p02/public_bootstrap_review_reference.v1.lyra",
        covers: &[
            "bootstrap_trust",
            "host_extinction_framework",
            "seed_runtime_law",
        ],
        examples: &[
            "negative_bootstrap_doc_drift_rejection",
            "phase_open_ecosystem_review",
        ],
        receipts: &[
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEcosystemDocDescriptor {
        id: "bootstrap_deployment_ecosystem_walkthrough",
        audience: "operator",
        path: "docs/p02/bootstrap_deployment_ecosystem_walkthrough.v1.lyra",
        covers: &[
            "bootstrap_trust",
            "seed_runtime_law",
            "host_extinction_framework",
        ],
        examples: &[
            "enterprise_deployment_to_ecosystem_handoff",
            "offline_airgap_bootstrap_review",
        ],
        receipts: &[
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_ECOSYSTEM_EXAMPLES: &[BootstrapEcosystemExampleDescriptor] = &[
    BootstrapEcosystemExampleDescriptor {
        id: "bootstrap_trust_walkthrough",
        kind: "walkthrough",
        path: "examples/p02/ecosystem/bootstrap_trust_walkthrough.v1.lyra",
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-inventory-check",
            "lyra-p02-bootstrap-deployment-check",
        ],
        proofs: &["bootstrap_docs_coverage_proof", "receipt_binding_proof"],
        receipts: &[
            "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        rejects: &["unreceipted_doc", "remote_service"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemExampleDescriptor {
        id: "seed_runtime_replacement_flow",
        kind: "extension_flow",
        path: "examples/p02/ecosystem/seed_runtime_replacement_flow.v1.lyra",
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-seed-runtime-replacement-check",
            "lyra-p02-bootstrap-formal-semantics-check",
        ],
        proofs: &["bootstrap_docs_coverage_proof", "executable_examples_proof"],
        receipts: &[
            "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        rejects: &["seed_runtime_drift", "host_owned_semantics"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemExampleDescriptor {
        id: "host_extinction_review",
        kind: "review",
        path: "examples/p02/ecosystem/host_extinction_review.v1.lyra",
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-extinction-check",
            "lyra-p02-foreign-surface-closure-check",
        ],
        proofs: &[
            "bootstrap_docs_coverage_proof",
            "deployment_ecosystem_bridge_proof",
        ],
        receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        rejects: &["foreign_surface_untracked", "silent_host_dependency"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemExampleDescriptor {
        id: "offline_airgap_bootstrap_review",
        kind: "corpus_flow",
        path: "examples/p02/ecosystem/offline_airgap_bootstrap_review.v1.lyra",
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-replay-check",
        ],
        proofs: &["offline_distribution_proof", "receipt_binding_proof"],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        rejects: &["network_required", "remote_fetch"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemExampleDescriptor {
        id: "enterprise_deployment_to_ecosystem_handoff",
        kind: "handoff",
        path: "examples/p02/ecosystem/enterprise_deployment_to_ecosystem_handoff.v1.lyra",
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-operator-handoff-automation-check",
        ],
        proofs: &[
            "deployment_ecosystem_bridge_proof",
            "executable_examples_proof",
        ],
        receipts: &[
            "receipts/p02/pass_0069_operator_handoff_automation.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        rejects: &["manual_only", "unbounded_enterprise_review"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemExampleDescriptor {
        id: "negative_bootstrap_doc_drift_rejection",
        kind: "negative",
        path: "fixtures/p02/bootstrap_ecosystem_inputs/invalid_ecosystem_drift.lyra",
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-falsification-check",
        ],
        proofs: &["receipt_binding_proof", "executable_examples_proof"],
        receipts: &[
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        rejects: &["ecosystem_drift_accepted", "documentation_alone"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemExampleDescriptor {
        id: "phase_open_ecosystem_review",
        kind: "review",
        path: "examples/p02/ecosystem/phase_open_ecosystem_review.v1.lyra",
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-deployment-check",
        ],
        proofs: &["p02_phase_open", "deployment_ecosystem_bridge_proof"],
        receipts: &[
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        rejects: &["phase_closure", "global_complete"],
        status: "blocked",
    },
];

pub const LYRALANG_BOOTSTRAP_ECOSYSTEM_PROOFS: &[BootstrapEcosystemProofDescriptor] = &[
    BootstrapEcosystemProofDescriptor {
        id: "bootstrap_docs_coverage_proof",
        scope: "docs",
        docs: &[
            "bootstrap_trust_operator_guide",
            "seed_runtime_law_developer_reference",
            "host_extinction_contributor_onboarding",
            "offline_bootstrap_distribution_reference",
            "enterprise_bootstrap_adoption_guide",
            "public_bootstrap_review_reference",
            "bootstrap_deployment_ecosystem_walkthrough",
        ],
        examples: &[
            "bootstrap_trust_walkthrough",
            "seed_runtime_replacement_flow",
            "host_extinction_review",
        ],
        receipts: &[
            "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
            "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        commands: &["lyra-p02-bootstrap-ecosystem-check"],
        forbids: &[
            "missing_bootstrap_trust_doc",
            "missing_seed_runtime_doc",
            "missing_host_extinction_doc",
        ],
        status: "artifact_emitted",
    },
    BootstrapEcosystemProofDescriptor {
        id: "executable_examples_proof",
        scope: "examples",
        docs: &[
            "bootstrap_trust_operator_guide",
            "seed_runtime_law_developer_reference",
            "host_extinction_contributor_onboarding",
        ],
        examples: &[
            "bootstrap_trust_walkthrough",
            "seed_runtime_replacement_flow",
            "host_extinction_review",
            "enterprise_deployment_to_ecosystem_handoff",
            "negative_bootstrap_doc_drift_rejection",
        ],
        receipts: &[
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-falsification-check",
        ],
        forbids: &["manual_only", "documentation_alone"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemProofDescriptor {
        id: "receipt_binding_proof",
        scope: "receipt",
        docs: &[
            "offline_bootstrap_distribution_reference",
            "public_bootstrap_review_reference",
        ],
        examples: &[
            "offline_airgap_bootstrap_review",
            "negative_bootstrap_doc_drift_rejection",
        ],
        receipts: &[
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-packaging-check",
        ],
        forbids: &["unreceipted_doc", "unreceipted_example"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemProofDescriptor {
        id: "offline_distribution_proof",
        scope: "distribution",
        docs: &[
            "offline_bootstrap_distribution_reference",
            "bootstrap_deployment_ecosystem_walkthrough",
        ],
        examples: &[
            "offline_airgap_bootstrap_review",
            "bootstrap_trust_walkthrough",
        ],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-deployment-check",
        ],
        forbids: &["network_required", "remote_fetch", "cloud_dependency"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemProofDescriptor {
        id: "deployment_ecosystem_bridge_proof",
        scope: "deployment",
        docs: &[
            "enterprise_bootstrap_adoption_guide",
            "bootstrap_deployment_ecosystem_walkthrough",
            "host_extinction_contributor_onboarding",
        ],
        examples: &[
            "enterprise_deployment_to_ecosystem_handoff",
            "host_extinction_review",
            "phase_open_ecosystem_review",
        ],
        receipts: &[
            "receipts/p02/pass_0069_operator_handoff_automation.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-deployment-check",
            "lyra-p02-operator-handoff-automation-check",
        ],
        forbids: &["deployment_drift", "ecosystem_drift_accepted"],
        status: "artifact_emitted",
    },
    BootstrapEcosystemProofDescriptor {
        id: "p02_phase_open",
        scope: "phase",
        docs: &[
            "public_bootstrap_review_reference",
            "enterprise_bootstrap_adoption_guide",
        ],
        examples: &["phase_open_ecosystem_review"],
        receipts: &[
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        ],
        commands: &["lyra-p02-bootstrap-ecosystem-check"],
        forbids: &["phase_closure", "global_complete"],
        status: "blocked",
    },
];

pub fn bootstrap_ecosystem_doc_descriptor(
    id: &str,
) -> Option<&'static BootstrapEcosystemDocDescriptor> {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_DOCS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_ecosystem_example_descriptor(
    id: &str,
) -> Option<&'static BootstrapEcosystemExampleDescriptor> {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_EXAMPLES
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_ecosystem_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapEcosystemProofDescriptor> {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_ecosystem_doc_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_DOCS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_ecosystem_example_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_EXAMPLES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_ecosystem_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_ecosystem_doc_digest(id: &str) -> Option<String> {
    bootstrap_ecosystem_doc_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_ecosystem.doc",
            &bootstrap_ecosystem_doc_signature(item),
        )
    })
}
pub fn bootstrap_ecosystem_example_digest(id: &str) -> Option<String> {
    bootstrap_ecosystem_example_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_ecosystem.example",
            &bootstrap_ecosystem_example_signature(item),
        )
    })
}
pub fn bootstrap_ecosystem_proof_digest(id: &str) -> Option<String> {
    bootstrap_ecosystem_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_ecosystem.proof",
            &bootstrap_ecosystem_proof_signature(item),
        )
    })
}

pub fn bootstrap_ecosystem_doc_signature(item: &BootstrapEcosystemDocDescriptor) -> String {
    format!(
        "doc:{}|audience:{}|path:{}|covers:{}|examples:{}|receipts:{}|status:{}",
        item.id,
        item.audience,
        item.path,
        item.covers.join(","),
        item.examples.join(","),
        item.receipts.join(","),
        item.status
    )
}

pub fn bootstrap_ecosystem_example_signature(item: &BootstrapEcosystemExampleDescriptor) -> String {
    format!(
        "example:{}|kind:{}|path:{}|commands:{}|proofs:{}|receipts:{}|rejects:{}|status:{}",
        item.id,
        item.kind,
        item.path,
        item.commands.join(","),
        item.proofs.join(","),
        item.receipts.join(","),
        item.rejects.join(","),
        item.status
    )
}

pub fn bootstrap_ecosystem_proof_signature(item: &BootstrapEcosystemProofDescriptor) -> String {
    format!(
        "proof:{}|scope:{}|docs:{}|examples:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
        item.id,
        item.scope,
        item.docs.join(","),
        item.examples.join(","),
        item.receipts.join(","),
        item.commands.join(","),
        item.forbids.join(","),
        item.status
    )
}

pub fn bootstrap_ecosystem_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_BOOTSTRAP_ECOSYSTEM_DOCS {
        rows.push(bootstrap_ecosystem_doc_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_ECOSYSTEM_EXAMPLES {
        rows.push(bootstrap_ecosystem_example_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_ECOSYSTEM_PROOFS {
        rows.push(bootstrap_ecosystem_proof_signature(item));
    }
    rows.sort();
    format!(
        "carrier:{}\n{}",
        LYRA_P02_BOOTSTRAP_ECOSYSTEM_CARRIER,
        rows.join("\n")
    )
}

pub fn bootstrap_ecosystem_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_ecosystem.registry",
        &bootstrap_ecosystem_registry_signature(),
    )
}

pub fn bootstrap_ecosystem_carrier_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_ecosystem.carrier",
        &bootstrap_ecosystem_registry_hash(),
    )
}

pub fn bootstrap_ecosystem_docs_bind_examples() -> bool {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_DOCS.iter().all(|doc| {
        !doc.examples.is_empty()
            && doc
                .examples
                .iter()
                .all(|example| bootstrap_ecosystem_example_descriptor(example).is_some())
    })
}

pub fn bootstrap_ecosystem_examples_bind_proofs() -> bool {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_EXAMPLES.iter().all(|example| {
        !example.proofs.is_empty()
            && example
                .proofs
                .iter()
                .all(|proof| bootstrap_ecosystem_proof_descriptor(proof).is_some())
    })
}

pub fn bootstrap_ecosystem_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_PROOFS.iter().all(|proof| {
        !proof.docs.is_empty()
            && !proof.examples.is_empty()
            && !proof.receipts.is_empty()
            && !proof.commands.is_empty()
            && proof
                .docs
                .iter()
                .all(|doc| bootstrap_ecosystem_doc_descriptor(doc).is_some())
            && proof
                .examples
                .iter()
                .all(|example| bootstrap_ecosystem_example_descriptor(example).is_some())
    })
}

pub fn bootstrap_ecosystem_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_ECOSYSTEM_DOCS
        .iter()
        .all(|doc| allowed_artifact_path(doc.path))
        && LYRALANG_BOOTSTRAP_ECOSYSTEM_EXAMPLES
            .iter()
            .all(|example| allowed_artifact_path(example.path))
}

pub fn bootstrap_ecosystem_no_forbidden_descriptor_claims() -> bool {
    let lowered = bootstrap_ecosystem_registry_signature().to_ascii_lowercase();
    ![
        "network required",
        "cloud required",
        "online required",
        "remote service required",
        "remote fetch",
        "documentation alone",
        "ecosystem drift accepted",
        "phase closed",
        "global complete",
        "todo",
        "placeholder",
        "best effort",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

pub fn bootstrap_ecosystem_receipts_cover_p02_001_through_p02_021() -> bool {
    let signature = bootstrap_ecosystem_registry_signature();
    [
        "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
        "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
        "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
        "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        "receipts/p02/pass_0069_operator_handoff_automation.receipt",
        "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        "receipts/p02/pass_0075_bootstrap_replay.receipt",
        "receipts/p02/pass_0077_bootstrap_packaging.receipt",
        "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
    ]
    .iter()
    .all(|needle| signature.contains(needle))
}

fn allowed_artifact_path(path: &str) -> bool {
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
