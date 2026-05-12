use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapEconomicsFrameDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub covers: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapPublicInterestOutputDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub constituencies: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub proofs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub rejects: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapEconomicsProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub frames: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_ECONOMICS_CARRIER: &str = "lyra.p02.bootstrap_economics.carrier.v1";

pub const LYRALANG_BOOTSTRAP_ECONOMICS_FRAMES: &[BootstrapEconomicsFrameDescriptor] = &[
    BootstrapEconomicsFrameDescriptor {
        id: "bootstrap_trust_public_value_frame",
        kind: "platform_value",
        path: "docs/p02/bootstrap_trust_public_value_frame.v1.lyra",
        covers: &[
            "bootstrap_trust",
            "public_interest",
            "non_extractive_access",
        ],
        outputs: &["bootstrap_public_casebook", "public_interest_receipt_pack"],
        receipts: &[
            "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsFrameDescriptor {
        id: "seed_runtime_sovereignty_value_frame",
        kind: "sovereignty_value",
        path: "docs/p02/seed_runtime_sovereignty_value_frame.v1.lyra",
        covers: &["seed_runtime_law", "offline_access", "operator_cost"],
        outputs: &[
            "operator_cost_benefit_sheet",
            "non_extractive_bootstrap_access_model",
        ],
        receipts: &[
            "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsFrameDescriptor {
        id: "host_extinction_anti_capture_frame",
        kind: "anti_capture",
        path: "docs/p02/host_extinction_anti_capture_frame.v1.lyra",
        covers: &["host_extinction_framework", "anti_capture", "stewardship"],
        outputs: &[
            "enterprise_non_capture_review_flow",
            "negative_capture_rejection",
        ],
        receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsFrameDescriptor {
        id: "offline_distribution_access_frame",
        kind: "public_access",
        path: "docs/p02/offline_distribution_access_frame.v1.lyra",
        covers: &["offline_access", "bootstrap_trust", "public_interest"],
        outputs: &[
            "non_extractive_bootstrap_access_model",
            "public_interest_receipt_pack",
        ],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsFrameDescriptor {
        id: "enterprise_adoption_non_capture_frame",
        kind: "enterprise_non_capture",
        path: "docs/p02/enterprise_adoption_non_capture_frame.v1.lyra",
        covers: &[
            "enterprise_adoption",
            "anti_capture",
            "host_extinction_framework",
        ],
        outputs: &[
            "enterprise_non_capture_review_flow",
            "negative_capture_rejection",
        ],
        receipts: &[
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsFrameDescriptor {
        id: "operator_cost_rebuild_frame",
        kind: "operator_cost",
        path: "products/p02/bootstrap_economics_manifest.v1.lyra",
        covers: &["operator_cost", "seed_runtime_law", "offline_access"],
        outputs: &["operator_cost_benefit_sheet", "bootstrap_public_casebook"],
        receipts: &[
            "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsFrameDescriptor {
        id: "public_review_stewardship_frame",
        kind: "stewardship",
        path: "products/p02/bootstrap_economics_inspection_surface.v1.lyra",
        covers: &["stewardship", "public_interest", "bootstrap_trust"],
        outputs: &[
            "public_interest_receipt_pack",
            "phase_open_economics_review",
        ],
        receipts: &[
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_PUBLIC_INTEREST_OUTPUTS: &[BootstrapPublicInterestOutputDescriptor] =
    &[
        BootstrapPublicInterestOutputDescriptor {
            id: "bootstrap_public_casebook",
            kind: "casebook",
            path: "examples/p02/economics/bootstrap_public_casebook.v1.lyra",
            constituencies: &["public", "operator", "steward"],
            commands: &[
                "lyra-p02-bootstrap-economics-check",
                "lyra-p02-bootstrap-inventory-check",
                "lyra-p02-bootstrap-ecosystem-check",
            ],
            proofs: &["economics_coverage_proof", "public_benefit_binding_proof"],
            receipts: &[
                "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
                "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
                "receipts/p02/pass_0080_bootstrap_economics.receipt",
            ],
            rejects: &["unreceipted_public_claim", "remote_service"],
            status: "artifact_emitted",
        },
        BootstrapPublicInterestOutputDescriptor {
            id: "operator_cost_benefit_sheet",
            kind: "cost_sheet",
            path: "examples/p02/economics/operator_cost_benefit_sheet.v1.lyra",
            constituencies: &["operator", "developer", "enterprise"],
            commands: &[
                "lyra-p02-bootstrap-economics-check",
                "lyra-p02-seed-runtime-replacement-check",
                "lyra-p02-bootstrap-deployment-check",
            ],
            proofs: &[
                "public_benefit_binding_proof",
                "non_extractive_access_proof",
            ],
            receipts: &[
                "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
                "receipts/p02/pass_0078_bootstrap_deployment.receipt",
                "receipts/p02/pass_0080_bootstrap_economics.receipt",
            ],
            rejects: &["unbounded_host_cost", "extractive_default"],
            status: "artifact_emitted",
        },
        BootstrapPublicInterestOutputDescriptor {
            id: "non_extractive_bootstrap_access_model",
            kind: "access_model",
            path: "products/p02/bootstrap_economics_manifest.v1.lyra",
            constituencies: &["public", "community", "steward", "operator"],
            commands: &[
                "lyra-p02-bootstrap-economics-check",
                "lyra-p02-bootstrap-packaging-check",
                "lyra-p02-bootstrap-deployment-check",
            ],
            proofs: &[
                "non_extractive_access_proof",
                "ecosystem_economics_bridge_proof",
            ],
            receipts: &[
                "receipts/p02/pass_0077_bootstrap_packaging.receipt",
                "receipts/p02/pass_0078_bootstrap_deployment.receipt",
                "receipts/p02/pass_0080_bootstrap_economics.receipt",
            ],
            rejects: &["paywall_default", "remote_fetch"],
            status: "artifact_emitted",
        },
        BootstrapPublicInterestOutputDescriptor {
            id: "enterprise_non_capture_review_flow",
            kind: "review_flow",
            path: "examples/p02/economics/enterprise_non_capture_review_flow.v1.lyra",
            constituencies: &["enterprise", "operator", "steward"],
            commands: &[
                "lyra-p02-bootstrap-economics-check",
                "lyra-p02-bootstrap-deployment-check",
                "lyra-p02-operator-handoff-automation-check",
            ],
            proofs: &[
                "anti_capture_receipt_proof",
                "ecosystem_economics_bridge_proof",
            ],
            receipts: &[
                "receipts/p02/pass_0069_operator_handoff_automation.receipt",
                "receipts/p02/pass_0078_bootstrap_deployment.receipt",
                "receipts/p02/pass_0080_bootstrap_economics.receipt",
            ],
            rejects: &["capture", "compliance_bypass"],
            status: "artifact_emitted",
        },
        BootstrapPublicInterestOutputDescriptor {
            id: "negative_capture_rejection",
            kind: "negative",
            path: "fixtures/p02/bootstrap_economics_inputs/invalid_capture_allowed.lyra",
            constituencies: &["public", "steward", "enterprise"],
            commands: &[
                "lyra-p02-bootstrap-economics-check",
                "lyra-p02-bootstrap-falsification-check",
            ],
            proofs: &["anti_capture_receipt_proof"],
            receipts: &[
                "receipts/p02/pass_0074_bootstrap_falsification.receipt",
                "receipts/p02/pass_0080_bootstrap_economics.receipt",
            ],
            rejects: &["capture_allowed", "economics_drift_accepted"],
            status: "artifact_emitted",
        },
        BootstrapPublicInterestOutputDescriptor {
            id: "public_interest_receipt_pack",
            kind: "receipt_pack",
            path: "receipts/p02/bootstrap_economics/bootstrap_public_interest_receipt_pack.receipt",
            constituencies: &["public", "community", "steward"],
            commands: &[
                "lyra-p02-bootstrap-economics-check",
                "lyra-p02-bootstrap-replay-check",
                "lyra-p02-bootstrap-ecosystem-check",
            ],
            proofs: &[
                "public_benefit_binding_proof",
                "ecosystem_economics_bridge_proof",
            ],
            receipts: &[
                "receipts/p02/pass_0075_bootstrap_replay.receipt",
                "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
                "receipts/p02/pass_0080_bootstrap_economics.receipt",
            ],
            rejects: &["unreceipted_value_claim", "public_interest_drift"],
            status: "artifact_emitted",
        },
        BootstrapPublicInterestOutputDescriptor {
            id: "phase_open_economics_review",
            kind: "review_flow",
            path: "examples/p02/economics/phase_open_economics_review.v1.lyra",
            constituencies: &["operator", "steward"],
            commands: &[
                "lyra-p02-bootstrap-economics-check",
                "lyra-p02-bootstrap-ecosystem-check",
            ],
            proofs: &["p02_phase_open", "ecosystem_economics_bridge_proof"],
            receipts: &[
                "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
                "receipts/p02/pass_0080_bootstrap_economics.receipt",
            ],
            rejects: &["phase_closure", "global_complete"],
            status: "blocked",
        },
    ];

pub const LYRALANG_BOOTSTRAP_ECONOMICS_PROOFS: &[BootstrapEconomicsProofDescriptor] = &[
    BootstrapEconomicsProofDescriptor {
        id: "economics_coverage_proof",
        scope: "economics",
        frames: &[
            "bootstrap_trust_public_value_frame",
            "seed_runtime_sovereignty_value_frame",
            "host_extinction_anti_capture_frame",
            "offline_distribution_access_frame",
        ],
        outputs: &[
            "bootstrap_public_casebook",
            "operator_cost_benefit_sheet",
            "non_extractive_bootstrap_access_model",
        ],
        receipts: &[
            "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
            "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        commands: &["lyra-p02-bootstrap-economics-check"],
        forbids: &[
            "missing_bootstrap_trust_value",
            "missing_seed_runtime_value",
            "capture",
            "phase_closure",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsProofDescriptor {
        id: "public_benefit_binding_proof",
        scope: "public_interest",
        frames: &[
            "bootstrap_trust_public_value_frame",
            "operator_cost_rebuild_frame",
            "public_review_stewardship_frame",
        ],
        outputs: &[
            "bootstrap_public_casebook",
            "operator_cost_benefit_sheet",
            "public_interest_receipt_pack",
        ],
        receipts: &[
            "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-ecosystem-check",
        ],
        forbids: &[
            "public_interest_drift",
            "extractive_default",
            "phase_closure",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsProofDescriptor {
        id: "non_extractive_access_proof",
        scope: "access",
        frames: &[
            "seed_runtime_sovereignty_value_frame",
            "offline_distribution_access_frame",
        ],
        outputs: &[
            "non_extractive_bootstrap_access_model",
            "operator_cost_benefit_sheet",
        ],
        receipts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-deployment-check",
        ],
        forbids: &["extractive_default", "remote_fetch", "phase_closure"],
        status: "artifact_emitted",
    },
    BootstrapEconomicsProofDescriptor {
        id: "anti_capture_receipt_proof",
        scope: "anti_capture",
        frames: &[
            "host_extinction_anti_capture_frame",
            "enterprise_adoption_non_capture_frame",
        ],
        outputs: &[
            "enterprise_non_capture_review_flow",
            "negative_capture_rejection",
        ],
        receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-falsification-check",
            "lyra-p02-foreign-surface-closure-check",
        ],
        forbids: &["capture", "extractive_default", "global_complete"],
        status: "artifact_emitted",
    },
    BootstrapEconomicsProofDescriptor {
        id: "ecosystem_economics_bridge_proof",
        scope: "ecosystem_bridge",
        frames: &[
            "enterprise_adoption_non_capture_frame",
            "public_review_stewardship_frame",
            "offline_distribution_access_frame",
        ],
        outputs: &[
            "enterprise_non_capture_review_flow",
            "public_interest_receipt_pack",
            "phase_open_economics_review",
        ],
        receipts: &[
            "receipts/p02/pass_0078_bootstrap_deployment.receipt",
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-economics-check",
            "lyra-p02-bootstrap-ecosystem-check",
            "lyra-p02-bootstrap-deployment-check",
        ],
        forbids: &[
            "capture",
            "ecosystem_drift_accepted",
            "economics_drift_accepted",
            "phase_closure",
        ],
        status: "artifact_emitted",
    },
    BootstrapEconomicsProofDescriptor {
        id: "p02_phase_open",
        scope: "phase",
        frames: &[
            "public_review_stewardship_frame",
            "enterprise_adoption_non_capture_frame",
        ],
        outputs: &["phase_open_economics_review"],
        receipts: &[
            "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
            "receipts/p02/pass_0080_bootstrap_economics.receipt",
        ],
        commands: &["lyra-p02-bootstrap-economics-check"],
        forbids: &["phase_closure", "global_complete", "capture"],
        status: "blocked",
    },
];

pub fn bootstrap_economics_frame_descriptor(
    id: &str,
) -> Option<&'static BootstrapEconomicsFrameDescriptor> {
    LYRALANG_BOOTSTRAP_ECONOMICS_FRAMES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_public_interest_output_descriptor(
    id: &str,
) -> Option<&'static BootstrapPublicInterestOutputDescriptor> {
    LYRALANG_BOOTSTRAP_PUBLIC_INTEREST_OUTPUTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_economics_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapEconomicsProofDescriptor> {
    LYRALANG_BOOTSTRAP_ECONOMICS_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_economics_frame_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_ECONOMICS_FRAMES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_public_interest_output_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PUBLIC_INTEREST_OUTPUTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_economics_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_ECONOMICS_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_economics_frame_digest(id: &str) -> Option<String> {
    bootstrap_economics_frame_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_economics.frame",
            &bootstrap_economics_frame_signature(item),
        )
    })
}
pub fn bootstrap_public_interest_output_digest(id: &str) -> Option<String> {
    bootstrap_public_interest_output_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_economics.output",
            &bootstrap_public_interest_output_signature(item),
        )
    })
}
pub fn bootstrap_economics_proof_digest(id: &str) -> Option<String> {
    bootstrap_economics_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_economics.proof",
            &bootstrap_economics_proof_signature(item),
        )
    })
}

pub fn bootstrap_economics_frame_signature(item: &BootstrapEconomicsFrameDescriptor) -> String {
    format!(
        "frame:{}|kind:{}|path:{}|covers:{}|outputs:{}|receipts:{}|status:{}",
        item.id,
        item.kind,
        item.path,
        item.covers.join(","),
        item.outputs.join(","),
        item.receipts.join(","),
        item.status
    )
}

pub fn bootstrap_public_interest_output_signature(
    item: &BootstrapPublicInterestOutputDescriptor,
) -> String {
    format!("output:{}|kind:{}|path:{}|constituencies:{}|commands:{}|proofs:{}|receipts:{}|rejects:{}|status:{}", item.id, item.kind, item.path, item.constituencies.join(","), item.commands.join(","), item.proofs.join(","), item.receipts.join(","), item.rejects.join(","), item.status)
}

pub fn bootstrap_economics_proof_signature(item: &BootstrapEconomicsProofDescriptor) -> String {
    format!(
        "proof:{}|scope:{}|frames:{}|outputs:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
        item.id,
        item.scope,
        item.frames.join(","),
        item.outputs.join(","),
        item.receipts.join(","),
        item.commands.join(","),
        item.forbids.join(","),
        item.status
    )
}

pub fn bootstrap_economics_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_BOOTSTRAP_ECONOMICS_FRAMES {
        rows.push(bootstrap_economics_frame_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_PUBLIC_INTEREST_OUTPUTS {
        rows.push(bootstrap_public_interest_output_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_ECONOMICS_PROOFS {
        rows.push(bootstrap_economics_proof_signature(item));
    }
    rows.sort();
    format!(
        "carrier:{}
{}",
        LYRA_P02_BOOTSTRAP_ECONOMICS_CARRIER,
        rows.join(
            "
"
        )
    )
}

pub fn bootstrap_economics_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_economics.registry",
        &bootstrap_economics_registry_signature(),
    )
}
pub fn bootstrap_economics_carrier_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_economics.carrier",
        &bootstrap_economics_registry_hash(),
    )
}

pub fn bootstrap_economics_frames_bind_outputs() -> bool {
    LYRALANG_BOOTSTRAP_ECONOMICS_FRAMES.iter().all(|frame| {
        !frame.outputs.is_empty()
            && frame
                .outputs
                .iter()
                .all(|output| bootstrap_public_interest_output_descriptor(output).is_some())
    })
}

pub fn bootstrap_economics_outputs_bind_proofs() -> bool {
    LYRALANG_BOOTSTRAP_PUBLIC_INTEREST_OUTPUTS
        .iter()
        .all(|output| {
            !output.proofs.is_empty()
                && output
                    .proofs
                    .iter()
                    .all(|proof| bootstrap_economics_proof_descriptor(proof).is_some())
        })
}

pub fn bootstrap_economics_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_ECONOMICS_PROOFS.iter().all(|proof| {
        !proof.frames.is_empty()
            && !proof.outputs.is_empty()
            && !proof.receipts.is_empty()
            && !proof.commands.is_empty()
            && proof
                .frames
                .iter()
                .all(|frame| bootstrap_economics_frame_descriptor(frame).is_some())
            && proof
                .outputs
                .iter()
                .all(|output| bootstrap_public_interest_output_descriptor(output).is_some())
    })
}

pub fn bootstrap_economics_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_ECONOMICS_FRAMES
        .iter()
        .all(|frame| allowed_artifact_path(frame.path))
        && LYRALANG_BOOTSTRAP_PUBLIC_INTEREST_OUTPUTS
            .iter()
            .all(|output| allowed_artifact_path(output.path))
}

pub fn bootstrap_economics_no_forbidden_descriptor_claims() -> bool {
    let lowered = bootstrap_economics_registry_signature().to_ascii_lowercase();
    ![
        "network required",
        "cloud required",
        "online required",
        "remote service required",
        "remote fetch",
        "capture allowed",
        "extractive default",
        "paywall default",
        "economics drift accepted",
        "phase closed",
        "global complete",
        "todo",
        "placeholder",
        "best effort",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

pub fn bootstrap_economics_receipts_cover_p02_001_through_p02_022() -> bool {
    let signature = bootstrap_economics_registry_signature();
    [
        "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
        "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
        "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
        "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
        "receipts/p02/pass_0069_operator_handoff_automation.receipt",
        "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        "receipts/p02/pass_0075_bootstrap_replay.receipt",
        "receipts/p02/pass_0077_bootstrap_packaging.receipt",
        "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        "receipts/p02/pass_0079_bootstrap_ecosystem.receipt",
        "receipts/p02/pass_0080_bootstrap_economics.receipt",
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
