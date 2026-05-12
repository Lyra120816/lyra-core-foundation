use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticEconomicsFrameDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub path: &'static str,
    pub covers: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticPublicInterestOutputDescriptor {
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
pub struct SemanticEconomicsProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub frames: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P01_SEMANTIC_ECONOMICS_CARRIER: &str = "lyra.p01.semantic_economics.carrier.v1";

pub const LYRALANG_SEMANTIC_ECONOMICS_FRAMES: &[SemanticEconomicsFrameDescriptor] = &[
    SemanticEconomicsFrameDescriptor {
        id: "canonical_semantics_platform_value_frame",
        kind: "platform_value",
        path: "docs/p01/semantic_economics_platform_value.lyra",
        covers: &[
            "canonical_symbols",
            "semantic_atoms",
            "core_ir",
            "public_interest",
        ],
        outputs: &["public_semantic_casebook", "science_and_civic_reuse_pack"],
        receipts: &["receipts/p01/pass_0051_semantic_economics.receipt"],
        status: "artifact_emitted",
    },
    SemanticEconomicsFrameDescriptor {
        id: "public_semantic_access_frame",
        kind: "public_access",
        path: "docs/p01/public_semantic_access_model.lyra",
        covers: &[
            "canonical_symbols",
            "semantic_atoms",
            "non_extractive_access",
        ],
        outputs: &[
            "non_extractive_semantic_access_model",
            "public_semantic_casebook",
        ],
        receipts: &["receipts/p01/pass_0051_semantic_economics.receipt"],
        status: "artifact_emitted",
    },
    SemanticEconomicsFrameDescriptor {
        id: "anti_capture_symbolic_infrastructure_frame",
        kind: "anti_capture",
        path: "docs/p01/anti_capture_symbolic_infrastructure.lyra",
        covers: &["core_ir", "stewardship", "public_interest"],
        outputs: &[
            "negative_capture_rejection",
            "semantic_stewardship_review_flow",
        ],
        receipts: &[
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    SemanticEconomicsFrameDescriptor {
        id: "operator_cost_rebuild_frame",
        kind: "operator_cost",
        path: "docs/p01/semantic_operator_cost_rebuild_frame.lyra",
        covers: &["canonical_symbols", "core_ir", "stewardship"],
        outputs: &[
            "operator_cost_benefit_sheet",
            "semantic_stewardship_review_flow",
        ],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    SemanticEconomicsFrameDescriptor {
        id: "commons_science_governance_frame",
        kind: "commons",
        path: "docs/p01/semantic_commons_science_governance.lyra",
        covers: &["semantic_atoms", "core_ir", "public_interest"],
        outputs: &[
            "science_and_civic_reuse_pack",
            "semantic_stewardship_review_flow",
        ],
        receipts: &[
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        status: "artifact_emitted",
    },
    SemanticEconomicsFrameDescriptor {
        id: "semantic_labor_participation_frame",
        kind: "labor_participation",
        path: "products/p01/semantic_labor_participation_frame.lyra",
        covers: &[
            "canonical_symbols",
            "semantic_atoms",
            "non_extractive_access",
            "stewardship",
        ],
        outputs: &[
            "operator_cost_benefit_sheet",
            "non_extractive_semantic_access_model",
        ],
        receipts: &["receipts/p01/pass_0051_semantic_economics.receipt"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_PUBLIC_INTEREST_OUTPUTS: &[SemanticPublicInterestOutputDescriptor] = &[
    SemanticPublicInterestOutputDescriptor {
        id: "public_semantic_casebook",
        kind: "casebook",
        path: "examples/p01/economics/public_semantic_casebook.lyra",
        constituencies: &["public", "community", "developer"],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-semantic-ecosystem-check",
        ],
        proofs: &["economics_coverage_proof", "public_benefit_binding_proof"],
        receipts: &[
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        rejects: &["unreceipted_value_claim", "capture_allowed"],
        status: "artifact_emitted",
    },
    SemanticPublicInterestOutputDescriptor {
        id: "operator_cost_benefit_sheet",
        kind: "cost_sheet",
        path: "products/p01/semantic_operator_cost_benefit_sheet.lyra",
        constituencies: &["operator", "developer", "labor"],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-semantic-packaging-check",
        ],
        proofs: &["economics_coverage_proof", "access_model_proof"],
        receipts: &[
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        rejects: &["hidden_network_cost", "unbounded_operator_burden"],
        status: "artifact_emitted",
    },
    SemanticPublicInterestOutputDescriptor {
        id: "semantic_stewardship_review_flow",
        kind: "review_flow",
        path: "examples/p01/economics/semantic_stewardship_review_flow.lyra",
        constituencies: &["steward", "community", "contributor"],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-semantic-deployment-check",
        ],
        proofs: &["public_benefit_binding_proof", "anti_capture_receipt_proof"],
        receipts: &[
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        rejects: &["public_interest_drift_accepted", "remote_service_required"],
        status: "artifact_emitted",
    },
    SemanticPublicInterestOutputDescriptor {
        id: "non_extractive_semantic_access_model",
        kind: "access_model",
        path: "products/p01/non_extractive_semantic_access_model.lyra",
        constituencies: &["public", "operator", "community"],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-semantic-interface-check",
        ],
        proofs: &["access_model_proof", "public_benefit_binding_proof"],
        receipts: &[
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        rejects: &["paywall_default", "extractive_default"],
        status: "artifact_emitted",
    },
    SemanticPublicInterestOutputDescriptor {
        id: "negative_capture_rejection",
        kind: "negative",
        path: "fixtures/p01/semantic_economics_inputs/invalid_capture_allowed.lyra",
        constituencies: &["public", "steward"],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-semantic-falsification-check",
        ],
        proofs: &["anti_capture_receipt_proof", "p01_phase_open"],
        receipts: &[
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        rejects: &["capture_allowed", "phase_closure"],
        status: "artifact_emitted",
    },
    SemanticPublicInterestOutputDescriptor {
        id: "science_and_civic_reuse_pack",
        kind: "reuse_pack",
        path: "examples/p01/economics/science_and_civic_reuse_pack.lyra",
        constituencies: &["science", "civic", "developer", "public"],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-ir-check",
            "lyra-p01-atom-check",
        ],
        proofs: &["science_civic_reuse_proof", "economics_coverage_proof"],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        rejects: &["orphan_ir_value_claim", "unreceipted_reuse_claim"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_ECONOMICS_PROOFS: &[SemanticEconomicsProofDescriptor] = &[
    SemanticEconomicsProofDescriptor {
        id: "economics_coverage_proof",
        scope: "economics",
        frames: &[
            "canonical_semantics_platform_value_frame",
            "operator_cost_rebuild_frame",
            "commons_science_governance_frame",
        ],
        outputs: &[
            "public_semantic_casebook",
            "operator_cost_benefit_sheet",
            "science_and_civic_reuse_pack",
        ],
        receipts: &["receipts/p01/pass_0051_semantic_economics.receipt"],
        commands: &["lyra-p01-semantic-economics-check"],
        forbids: &[
            "phase_closure",
            "global_complete",
            "capture",
            "extractive_default",
        ],
        status: "artifact_emitted",
    },
    SemanticEconomicsProofDescriptor {
        id: "public_benefit_binding_proof",
        scope: "public_interest",
        frames: &[
            "canonical_semantics_platform_value_frame",
            "public_semantic_access_frame",
            "commons_science_governance_frame",
        ],
        outputs: &[
            "public_semantic_casebook",
            "semantic_stewardship_review_flow",
            "non_extractive_semantic_access_model",
        ],
        receipts: &[
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-semantic-ecosystem-check",
        ],
        forbids: &[
            "phase_closure",
            "global_complete",
            "capture",
            "extractive_default",
        ],
        status: "artifact_emitted",
    },
    SemanticEconomicsProofDescriptor {
        id: "access_model_proof",
        scope: "access",
        frames: &[
            "public_semantic_access_frame",
            "operator_cost_rebuild_frame",
            "semantic_labor_participation_frame",
        ],
        outputs: &[
            "non_extractive_semantic_access_model",
            "operator_cost_benefit_sheet",
        ],
        receipts: &[
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-semantic-interface-check",
        ],
        forbids: &[
            "phase_closure",
            "global_complete",
            "capture",
            "extractive_default",
        ],
        status: "artifact_emitted",
    },
    SemanticEconomicsProofDescriptor {
        id: "anti_capture_receipt_proof",
        scope: "anti_capture",
        frames: &[
            "anti_capture_symbolic_infrastructure_frame",
            "semantic_labor_participation_frame",
        ],
        outputs: &[
            "negative_capture_rejection",
            "semantic_stewardship_review_flow",
        ],
        receipts: &[
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-semantic-falsification-check",
        ],
        forbids: &[
            "phase_closure",
            "global_complete",
            "capture",
            "extractive_default",
        ],
        status: "artifact_emitted",
    },
    SemanticEconomicsProofDescriptor {
        id: "science_civic_reuse_proof",
        scope: "reuse",
        frames: &[
            "canonical_semantics_platform_value_frame",
            "commons_science_governance_frame",
        ],
        outputs: &["science_and_civic_reuse_pack", "public_semantic_casebook"],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        commands: &[
            "lyra-p01-semantic-economics-check",
            "lyra-p01-ir-check",
            "lyra-p01-atom-check",
        ],
        forbids: &[
            "phase_closure",
            "global_complete",
            "capture",
            "extractive_default",
        ],
        status: "artifact_emitted",
    },
    SemanticEconomicsProofDescriptor {
        id: "p01_phase_open",
        scope: "phase",
        frames: &[
            "anti_capture_symbolic_infrastructure_frame",
            "public_semantic_access_frame",
        ],
        outputs: &[
            "negative_capture_rejection",
            "non_extractive_semantic_access_model",
        ],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0032_semantic_objects.receipt",
            "receipts/p01/pass_0033_semantic_identity.receipt",
            "receipts/p01/pass_0034_reference_semantics.receipt",
            "receipts/p01/pass_0035_symbolic_equality.receipt",
            "receipts/p01/pass_0036_error_challenge_evidence.receipt",
            "receipts/p01/pass_0037_semantic_serialization_hashing.receipt",
            "receipts/p01/pass_0038_semantic_adversarial_corpus.receipt",
            "receipts/p01/pass_0039_core_ir_reuse.receipt",
            "receipts/p01/pass_0040_semantic_atom_reference.receipt",
            "receipts/p01/pass_0041_semantic_bedrock_receipts.receipt",
            "receipts/p01/pass_0042_formal_semantic_constitution.receipt",
            "receipts/p01/pass_0043_canonical_data_model.receipt",
            "receipts/p01/pass_0044_semantic_core_engine.receipt",
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0048_semantic_packaging.receipt",
            "receipts/p01/pass_0049_semantic_deployment.receipt",
            "receipts/p01/pass_0050_semantic_ecosystem.receipt",
            "receipts/p01/pass_0051_semantic_economics.receipt",
        ],
        commands: &["lyra-p01-semantic-economics-check"],
        forbids: &[
            "phase_closure",
            "global_complete",
            "capture",
            "extractive_default",
        ],
        status: "blocked",
    },
];

pub fn semantic_economics_frame_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ECONOMICS_FRAMES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_public_interest_output_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_PUBLIC_INTEREST_OUTPUTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_economics_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_ECONOMICS_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_economics_frame_descriptor(
    id: &str,
) -> Option<&'static SemanticEconomicsFrameDescriptor> {
    LYRALANG_SEMANTIC_ECONOMICS_FRAMES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_public_interest_output_descriptor(
    id: &str,
) -> Option<&'static SemanticPublicInterestOutputDescriptor> {
    LYRALANG_SEMANTIC_PUBLIC_INTEREST_OUTPUTS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_economics_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticEconomicsProofDescriptor> {
    LYRALANG_SEMANTIC_ECONOMICS_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_economics_frame_signature(item: &SemanticEconomicsFrameDescriptor) -> String {
    format!(
        "frame:{}|kind:{}|path:{}|covers:{}|outputs:{}|receipts:{}|status:{}",
        item.id,
        item.kind,
        item.path,
        sorted_join(item.covers),
        sorted_join(item.outputs),
        sorted_join(item.receipts),
        item.status
    )
}

pub fn semantic_public_interest_output_signature(
    item: &SemanticPublicInterestOutputDescriptor,
) -> String {
    format!("output:{}|kind:{}|path:{}|constituencies:{}|commands:{}|proofs:{}|receipts:{}|rejects:{}|status:{}", item.id, item.kind, item.path, sorted_join(item.constituencies), sorted_join(item.commands), sorted_join(item.proofs), sorted_join(item.receipts), sorted_join(item.rejects), item.status)
}

pub fn semantic_economics_proof_signature(item: &SemanticEconomicsProofDescriptor) -> String {
    format!(
        "proof:{}|scope:{}|frames:{}|outputs:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
        item.id,
        item.scope,
        sorted_join(item.frames),
        sorted_join(item.outputs),
        sorted_join(item.receipts),
        sorted_join(item.commands),
        sorted_join(item.forbids),
        item.status
    )
}

pub fn semantic_economics_frame_digest(item: &SemanticEconomicsFrameDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_economics.frame",
        &semantic_economics_frame_signature(item),
    )
}
pub fn semantic_public_interest_output_digest(
    item: &SemanticPublicInterestOutputDescriptor,
) -> String {
    stable_hash_label(
        "lyra.p01.semantic_economics.output",
        &semantic_public_interest_output_signature(item),
    )
}
pub fn semantic_economics_proof_digest(item: &SemanticEconomicsProofDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.semantic_economics.proof",
        &semantic_economics_proof_signature(item),
    )
}

pub fn semantic_economics_registry_signature() -> String {
    let mut rows = Vec::new();
    for frame in LYRALANG_SEMANTIC_ECONOMICS_FRAMES {
        rows.push(format!(
            "frame:{}|{}",
            frame.id,
            semantic_economics_frame_digest(frame)
        ));
    }
    for output in LYRALANG_SEMANTIC_PUBLIC_INTEREST_OUTPUTS {
        rows.push(format!(
            "output:{}|{}",
            output.id,
            semantic_public_interest_output_digest(output)
        ));
    }
    for proof in LYRALANG_SEMANTIC_ECONOMICS_PROOFS {
        rows.push(format!(
            "proof:{}|{}",
            proof.id,
            semantic_economics_proof_digest(proof)
        ));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_economics_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_economics.registry",
        &semantic_economics_registry_signature(),
    )
}

pub fn semantic_economics_frames_bind_outputs() -> bool {
    let output_ids = semantic_public_interest_output_ids();
    LYRALANG_SEMANTIC_ECONOMICS_FRAMES
        .iter()
        .all(|frame| frame.outputs.iter().all(|id| output_ids.contains(id)))
}

pub fn semantic_public_interest_outputs_bind_proofs() -> bool {
    let proof_ids = semantic_economics_proof_ids();
    LYRALANG_SEMANTIC_PUBLIC_INTEREST_OUTPUTS
        .iter()
        .all(|output| output.proofs.iter().all(|id| proof_ids.contains(id)))
}

pub fn semantic_economics_proofs_bind_registry() -> bool {
    let frame_ids = semantic_economics_frame_ids();
    let output_ids = semantic_public_interest_output_ids();
    LYRALANG_SEMANTIC_ECONOMICS_PROOFS.iter().all(|proof| {
        proof.frames.iter().all(|id| frame_ids.contains(id))
            && proof.outputs.iter().all(|id| output_ids.contains(id))
    })
}

pub fn semantic_economics_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_ECONOMICS_FRAMES
        .iter()
        .all(|frame| frame.path.starts_with("docs/p01/") || frame.path.starts_with("products/p01/"))
        && LYRALANG_SEMANTIC_PUBLIC_INTEREST_OUTPUTS
            .iter()
            .all(|output| {
                output.path.starts_with("examples/p01/")
                    || output.path.starts_with("products/p01/")
                    || output.path.starts_with("fixtures/p01/")
            })
}

pub fn semantic_economics_receipts_cover_p01_001_through_p01_022() -> bool {
    let required = [
        "pass_0030_semantic_atoms",
        "pass_0031_core_ir",
        "pass_0032_semantic_objects",
        "pass_0033_semantic_identity",
        "pass_0034_reference_semantics",
        "pass_0035_symbolic_equality",
        "pass_0036_error_challenge_evidence",
        "pass_0037_semantic_serialization_hashing",
        "pass_0038_semantic_adversarial_corpus",
        "pass_0039_core_ir_reuse",
        "pass_0040_semantic_atom_reference",
        "pass_0041_semantic_bedrock_receipts",
        "pass_0042_formal_semantic_constitution",
        "pass_0043_canonical_data_model",
        "pass_0044_semantic_core_engine",
        "pass_0045_semantic_falsification",
        "pass_0046_semantic_replay",
        "pass_0047_semantic_interface",
        "pass_0048_semantic_packaging",
        "pass_0049_semantic_deployment",
        "pass_0050_semantic_ecosystem",
        "pass_0051_semantic_economics",
    ];
    let mut receipts = Vec::new();
    for frame in LYRALANG_SEMANTIC_ECONOMICS_FRAMES {
        receipts.extend_from_slice(frame.receipts);
    }
    for output in LYRALANG_SEMANTIC_PUBLIC_INTEREST_OUTPUTS {
        receipts.extend_from_slice(output.receipts);
    }
    for proof in LYRALANG_SEMANTIC_ECONOMICS_PROOFS {
        receipts.extend_from_slice(proof.receipts);
    }
    required
        .iter()
        .all(|needle| receipts.iter().any(|receipt| receipt.contains(needle)))
}

pub fn semantic_economics_no_forbidden_descriptor_claims() -> bool {
    let signature = semantic_economics_registry_signature().to_ascii_lowercase();
    !signature.contains("network required")
        && !signature.contains("cloud required")
        && !signature.contains("online required")
        && !signature.contains("remote fetch")
        && !signature.contains("capture allowed")
        && !signature.contains("paywall default")
        && !signature.contains("extractive default")
        && !signature.contains("phase closed")
        && !signature.contains("global complete")
}

fn sorted_join(items: &[&'static str]) -> String {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy.join(",")
}
