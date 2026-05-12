use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticPackageUnitDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub owner_root: &'static str,
    pub artifacts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticReleaseBundleDescriptor {
    pub id: &'static str,
    pub order: &'static str,
    pub packages: &'static [&'static str],
    pub artifacts: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub checks: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticDistributionCheckDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub target: &'static str,
    pub requires: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticPackagingProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub packages: &'static [&'static str],
    pub bundles: &'static [&'static str],
    pub checks: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P01_SEMANTIC_PACKAGING_CARRIER: &str = "lyra.p01.semantic_packaging.carrier.v1";

pub const LYRALANG_SEMANTIC_PACKAGING_PACKAGES: &[SemanticPackageUnitDescriptor] = &[
    SemanticPackageUnitDescriptor {
        id: "semantic_core_validation_binaries",
        kind: "binary_group",
        owner_root: "src",
        artifacts: &[
            "src/bin/lyra-p01-atom-check.rs",
            "src/bin/lyra-p01-ir-check.rs",
            "src/bin/lyra-p01-semantic-core-engine-check.rs",
            "src/bin/lyra-p01-semantic-falsification-check.rs",
            "src/bin/lyra-p01-semantic-replay-check.rs",
            "src/bin/lyra-p01-semantic-interface-check.rs",
            "src/bin/lyra-p01-semantic-packaging-check.rs",
        ],
        commands: &[
            "lyra-p01-atom-check",
            "lyra-p01-ir-check",
            "lyra-p01-semantic-core-engine-check",
            "lyra-p01-semantic-falsification-check",
            "lyra-p01-semantic-replay-check",
            "lyra-p01-semantic-interface-check",
            "lyra-p01-semantic-packaging-check",
        ],
        receipts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0044_semantic_core_engine.receipt",
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0048_semantic_packaging.receipt",
        ],
        status: "artifact_emitted",
    },
    SemanticPackageUnitDescriptor {
        id: "semantic_contract_model_set",
        kind: "contract_set",
        owner_root: "interfaces",
        artifacts: &[
            "interfaces/p01/src/semantic_packaging_model.rs",
            "interfaces/p01/contracts/semantic_packaging.v1.lyra",
            "interfaces/p01/contracts/semantic_interface.v1.lyra",
            "interfaces/p01/contracts/semantic_replay.v1.lyra",
        ],
        commands: &["lyra-p01-semantic-packaging-check"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticPackageUnitDescriptor {
        id: "semantic_control_law_set",
        kind: "control_plane",
        owner_root: "ops",
        artifacts: &[
            "ops/p01/src/semantic_packaging.rs",
            "ops/p01/control/semantic_packaging_law.v1.lyra",
            "ops/p01/control/frontier_lock.v1.lyra",
            "ops/p01/control/truth_snapshot.v1.lyra",
            "ops/p01/control/blocker_index.v1.lyra",
        ],
        commands: &["lyra-p01-semantic-packaging-check"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticPackageUnitDescriptor {
        id: "semantic_positive_negative_corpus",
        kind: "corpus",
        owner_root: "fixtures",
        artifacts: &[
            "fixtures/p01/semantic_packaging_inputs/valid_semantic_packaging.lyra",
            "fixtures/p01/semantic_packaging_inputs/invalid_missing_rule.lyra",
            "fixtures/p01/semantic_packaging_inputs/invalid_unknown_bundle_package.lyra",
            "fixtures/p01/semantic_packaging_inputs/invalid_network_required.lyra",
        ],
        commands: &["lyra-p01-semantic-packaging-check"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticPackageUnitDescriptor {
        id: "semantic_packaging_goldens",
        kind: "golden_set",
        owner_root: "goldens",
        artifacts: &["goldens/p01/valid_semantic_packaging.receipt"],
        commands: &["lyra-p01-semantic-packaging-check"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticPackageUnitDescriptor {
        id: "semantic_receipt_chain",
        kind: "receipt_set",
        owner_root: "receipts",
        artifacts: &[
            "receipts/p01/pass_0030_semantic_atoms.receipt",
            "receipts/p01/pass_0031_core_ir.receipt",
            "receipts/p01/pass_0044_semantic_core_engine.receipt",
            "receipts/p01/pass_0045_semantic_falsification.receipt",
            "receipts/p01/pass_0046_semantic_replay.receipt",
            "receipts/p01/pass_0047_semantic_interface.receipt",
            "receipts/p01/pass_0048_semantic_packaging.receipt",
        ],
        commands: &["lyra-p01-semantic-packaging-check"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticPackageUnitDescriptor {
        id: "semantic_product_surfaces",
        kind: "product_surface",
        owner_root: "products",
        artifacts: &[
            "products/p01/semantic_interface_inspection_surface.lyra",
            "products/p01/semantic_packaging_distribution_manifest.lyra",
            "products/p01/semantic_packaging_inspection_surface.lyra",
        ],
        commands: &["lyra-p01-semantic-packaging-check"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_PACKAGING_BUNDLES: &[SemanticReleaseBundleDescriptor] = &[
    SemanticReleaseBundleDescriptor {
        id: "p01_local_semantic_package_bundle",
        order: "001",
        packages: &[
            "semantic_core_validation_binaries",
            "semantic_contract_model_set",
            "semantic_control_law_set",
            "semantic_positive_negative_corpus",
            "semantic_packaging_goldens",
            "semantic_receipt_chain",
            "semantic_product_surfaces",
        ],
        artifacts: &["products/p01/semantic_packaging_distribution_manifest.lyra"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        checks: &[
            "offline_installable",
            "manifest_hash_stable",
            "artifact_paths_bound",
            "receipt_paths_bound",
            "command_set_complete",
            "product_surface_bound",
        ],
        forbids: &["remote_fetch", "ambient_network", "package_drift"],
        status: "artifact_emitted",
    },
    SemanticReleaseBundleDescriptor {
        id: "p01_red_team_semantic_package_bundle",
        order: "002",
        packages: &[
            "semantic_control_law_set",
            "semantic_positive_negative_corpus",
            "semantic_receipt_chain",
        ],
        artifacts: &["fixtures/p01/semantic_packaging_inputs/invalid_network_required.lyra"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        checks: &["no_remote_fetch", "receipt_paths_bound"],
        forbids: &["negative_fixture_acceptance", "unreceipted_package_action"],
        status: "artifact_emitted",
    },
    SemanticReleaseBundleDescriptor {
        id: "p01_operator_product_bundle",
        order: "003",
        packages: &[
            "semantic_core_validation_binaries",
            "semantic_contract_model_set",
            "semantic_receipt_chain",
            "semantic_product_surfaces",
        ],
        artifacts: &[
            "examples/p01/operator/semantic_packaging_review.lyra",
            "products/p01/semantic_packaging_inspection_surface.lyra",
        ],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        checks: &[
            "offline_installable",
            "product_surface_bound",
            "command_set_complete",
        ],
        forbids: &["manual_only_release", "operator_drift"],
        status: "artifact_emitted",
    },
    SemanticReleaseBundleDescriptor {
        id: "p01_offline_distribution_bundle",
        order: "004",
        packages: &[
            "semantic_core_validation_binaries",
            "semantic_contract_model_set",
            "semantic_control_law_set",
            "semantic_positive_negative_corpus",
            "semantic_packaging_goldens",
            "semantic_receipt_chain",
            "semantic_product_surfaces",
        ],
        artifacts: &["products/p01/semantic_packaging_distribution_manifest.lyra"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        checks: &[
            "offline_installable",
            "manifest_hash_stable",
            "artifact_paths_bound",
            "receipt_paths_bound",
            "no_remote_fetch",
            "command_set_complete",
            "product_surface_bound",
        ],
        forbids: &[
            "network_dependency",
            "cloud_dependency",
            "global_closure_claim",
        ],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_PACKAGING_CHECKS: &[SemanticDistributionCheckDescriptor] = &[
    SemanticDistributionCheckDescriptor {
        id: "offline_installable",
        scope: "distribution",
        target: "p01_semantic_distribution",
        requires: &[
            "semantic_core_validation_binaries",
            "semantic_contract_model_set",
        ],
        forbids: &["network_dependency", "cloud_dependency"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticDistributionCheckDescriptor {
        id: "manifest_hash_stable",
        scope: "bundle",
        target: "p01_local_semantic_package_bundle",
        requires: &["semantic_product_surfaces", "semantic_receipt_chain"],
        forbids: &["manifest_order_drift"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticDistributionCheckDescriptor {
        id: "artifact_paths_bound",
        scope: "package",
        target: "semantic_product_surfaces",
        requires: &["semantic_contract_model_set", "semantic_control_law_set"],
        forbids: &["unowned_artifact_path"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticDistributionCheckDescriptor {
        id: "receipt_paths_bound",
        scope: "package",
        target: "semantic_receipt_chain",
        requires: &["semantic_receipt_chain"],
        forbids: &["unreceipted_package_action"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticDistributionCheckDescriptor {
        id: "no_remote_fetch",
        scope: "distribution",
        target: "p01_semantic_distribution",
        requires: &["semantic_core_validation_binaries"],
        forbids: &["remote_fetch", "online_dependency"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticDistributionCheckDescriptor {
        id: "command_set_complete",
        scope: "bundle",
        target: "p01_local_semantic_package_bundle",
        requires: &["semantic_core_validation_binaries"],
        forbids: &["missing_packaging_binary"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
    SemanticDistributionCheckDescriptor {
        id: "product_surface_bound",
        scope: "product",
        target: "semantic_product_surfaces",
        requires: &["semantic_product_surfaces", "semantic_contract_model_set"],
        forbids: &["product_surface_drift"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_PACKAGING_PROOFS: &[SemanticPackagingProofDescriptor] = &[
    SemanticPackagingProofDescriptor {
        id: "package_manifest_coverage",
        scope: "package",
        packages: &[
            "semantic_core_validation_binaries",
            "semantic_contract_model_set",
            "semantic_control_law_set",
            "semantic_positive_negative_corpus",
            "semantic_packaging_goldens",
            "semantic_receipt_chain",
            "semantic_product_surfaces",
        ],
        bundles: &["p01_local_semantic_package_bundle"],
        checks: &["artifact_paths_bound", "receipt_paths_bound"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        commands: &["lyra-p01-semantic-packaging-check"],
        forbids: &["missing_package_unit", "unowned_artifact_path"],
        status: "artifact_emitted",
    },
    SemanticPackagingProofDescriptor {
        id: "release_bundle_determinism",
        scope: "bundle",
        packages: &[
            "semantic_core_validation_binaries",
            "semantic_receipt_chain",
        ],
        bundles: &[
            "p01_local_semantic_package_bundle",
            "p01_offline_distribution_bundle",
        ],
        checks: &["manifest_hash_stable", "command_set_complete"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        commands: &["lyra-p01-semantic-packaging-check"],
        forbids: &["release_order_drift"],
        status: "artifact_emitted",
    },
    SemanticPackagingProofDescriptor {
        id: "offline_distribution_gate",
        scope: "distribution",
        packages: &[
            "semantic_core_validation_binaries",
            "semantic_contract_model_set",
            "semantic_receipt_chain",
        ],
        bundles: &["p01_offline_distribution_bundle"],
        checks: &["offline_installable", "no_remote_fetch"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        commands: &["lyra-p01-semantic-packaging-check"],
        forbids: &["network_dependency", "cloud_dependency"],
        status: "artifact_emitted",
    },
    SemanticPackagingProofDescriptor {
        id: "product_surface_binding",
        scope: "product",
        packages: &["semantic_product_surfaces", "semantic_contract_model_set"],
        bundles: &["p01_operator_product_bundle"],
        checks: &["product_surface_bound"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        commands: &["lyra-p01-semantic-packaging-check"],
        forbids: &["product_surface_drift"],
        status: "artifact_emitted",
    },
    SemanticPackagingProofDescriptor {
        id: "p01_phase_open",
        scope: "phase",
        packages: &["semantic_receipt_chain"],
        bundles: &["p01_local_semantic_package_bundle"],
        checks: &["receipt_paths_bound"],
        receipts: &["receipts/p01/pass_0048_semantic_packaging.receipt"],
        commands: &["lyra-p01-semantic-packaging-check"],
        forbids: &["phase_closure_claim", "global_complete_claim"],
        status: "blocked",
    },
];

pub fn semantic_packaging_package_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_PACKAGING_PACKAGES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_packaging_bundle_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_PACKAGING_BUNDLES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_packaging_check_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_PACKAGING_CHECKS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_packaging_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_PACKAGING_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_packaging_package_descriptor(
    id: &str,
) -> Option<&'static SemanticPackageUnitDescriptor> {
    LYRALANG_SEMANTIC_PACKAGING_PACKAGES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_packaging_bundle_descriptor(
    id: &str,
) -> Option<&'static SemanticReleaseBundleDescriptor> {
    LYRALANG_SEMANTIC_PACKAGING_BUNDLES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_packaging_check_descriptor(
    id: &str,
) -> Option<&'static SemanticDistributionCheckDescriptor> {
    LYRALANG_SEMANTIC_PACKAGING_CHECKS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_packaging_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticPackagingProofDescriptor> {
    LYRALANG_SEMANTIC_PACKAGING_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_packaging_package_signature(item: &SemanticPackageUnitDescriptor) -> String {
    format!(
        "package:{}|kind:{}|owner:{}|artifacts:{}|commands:{}|receipts:{}|status:{}",
        item.id,
        item.kind,
        item.owner_root,
        item.artifacts.join(","),
        item.commands.join(","),
        item.receipts.join(","),
        item.status
    )
}
pub fn semantic_packaging_bundle_signature(item: &SemanticReleaseBundleDescriptor) -> String {
    format!(
        "bundle:{}|order:{}|packages:{}|artifacts:{}|receipts:{}|checks:{}|forbids:{}|status:{}",
        item.id,
        item.order,
        item.packages.join(","),
        item.artifacts.join(","),
        item.receipts.join(","),
        item.checks.join(","),
        item.forbids.join(","),
        item.status
    )
}
pub fn semantic_packaging_check_signature(item: &SemanticDistributionCheckDescriptor) -> String {
    format!(
        "check:{}|scope:{}|target:{}|requires:{}|forbids:{}|receipts:{}|status:{}",
        item.id,
        item.scope,
        item.target,
        item.requires.join(","),
        item.forbids.join(","),
        item.receipts.join(","),
        item.status
    )
}
pub fn semantic_packaging_proof_signature(item: &SemanticPackagingProofDescriptor) -> String {
    format!("proof:{}|scope:{}|packages:{}|bundles:{}|checks:{}|receipts:{}|commands:{}|forbids:{}|status:{}", item.id, item.scope, item.packages.join(","), item.bundles.join(","), item.checks.join(","), item.receipts.join(","), item.commands.join(","), item.forbids.join(","), item.status)
}

pub fn semantic_packaging_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SEMANTIC_PACKAGING_PACKAGES {
        rows.push(semantic_packaging_package_signature(item));
    }
    for item in LYRALANG_SEMANTIC_PACKAGING_BUNDLES {
        rows.push(semantic_packaging_bundle_signature(item));
    }
    for item in LYRALANG_SEMANTIC_PACKAGING_CHECKS {
        rows.push(semantic_packaging_check_signature(item));
    }
    for item in LYRALANG_SEMANTIC_PACKAGING_PROOFS {
        rows.push(semantic_packaging_proof_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_packaging_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_packaging.registry",
        &semantic_packaging_registry_signature(),
    )
}
pub fn semantic_packaging_package_digest(id: &str) -> Option<String> {
    semantic_packaging_package_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_packaging.package_descriptor",
            &semantic_packaging_package_signature(item),
        )
    })
}
pub fn semantic_packaging_bundle_digest(id: &str) -> Option<String> {
    semantic_packaging_bundle_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_packaging.bundle_descriptor",
            &semantic_packaging_bundle_signature(item),
        )
    })
}
pub fn semantic_packaging_check_digest(id: &str) -> Option<String> {
    semantic_packaging_check_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_packaging.check_descriptor",
            &semantic_packaging_check_signature(item),
        )
    })
}
pub fn semantic_packaging_proof_digest(id: &str) -> Option<String> {
    semantic_packaging_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_packaging.proof_descriptor",
            &semantic_packaging_proof_signature(item),
        )
    })
}

pub fn semantic_packaging_bundles_bind_registry() -> bool {
    LYRALANG_SEMANTIC_PACKAGING_BUNDLES.iter().all(|bundle| {
        bundle
            .packages
            .iter()
            .all(|id| semantic_packaging_package_descriptor(id).is_some())
            && bundle
                .checks
                .iter()
                .all(|id| semantic_packaging_check_descriptor(id).is_some())
            && !bundle.receipts.is_empty()
            && !bundle.forbids.is_empty()
    })
}

pub fn semantic_packaging_checks_bind_registry() -> bool {
    LYRALANG_SEMANTIC_PACKAGING_CHECKS.iter().all(|check| {
        let target_known = semantic_packaging_package_descriptor(check.target).is_some()
            || semantic_packaging_bundle_descriptor(check.target).is_some()
            || check.target == "p01_semantic_distribution";
        target_known
            && !check.requires.is_empty()
            && !check.receipts.is_empty()
            && !check.forbids.is_empty()
    })
}

pub fn semantic_packaging_proofs_bind_registry() -> bool {
    LYRALANG_SEMANTIC_PACKAGING_PROOFS.iter().all(|proof| {
        proof
            .packages
            .iter()
            .all(|id| semantic_packaging_package_descriptor(id).is_some())
            && proof
                .bundles
                .iter()
                .all(|id| semantic_packaging_bundle_descriptor(id).is_some())
            && proof
                .checks
                .iter()
                .all(|id| semantic_packaging_check_descriptor(id).is_some())
            && !proof.receipts.is_empty()
            && !proof.commands.is_empty()
            && !proof.forbids.is_empty()
    })
}

pub fn semantic_packaging_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_PACKAGING_PACKAGES.iter().all(|package| {
        package
            .artifacts
            .iter()
            .all(|path| !path.contains("..") && path.starts_with(package.owner_root))
            && package
                .receipts
                .iter()
                .all(|path| path.starts_with("receipts/p01/") && path.ends_with(".receipt"))
    })
}

pub fn semantic_packaging_units_cover_p01_001_through_p01_019() -> bool {
    let required = [
        "semantic_core_validation_binaries",
        "semantic_contract_model_set",
        "semantic_control_law_set",
        "semantic_positive_negative_corpus",
        "semantic_packaging_goldens",
        "semantic_receipt_chain",
        "semantic_product_surfaces",
    ];
    required
        .iter()
        .all(|id| semantic_packaging_package_descriptor(id).is_some())
}

pub fn semantic_packaging_no_forbidden_descriptor_claims() -> bool {
    let registry = semantic_packaging_registry_signature();
    !registry.contains("network required")
        && !registry.contains("cloud required")
        && !registry.contains("online required")
        && !registry.contains("package drift accepted")
        && !registry.contains("release drift accepted")
        && !registry.contains("phase closed")
        && !registry.contains("global complete")
}
