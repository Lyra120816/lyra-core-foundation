use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_BOOTSTRAP_PACKAGING_CARRIER: &str = "lyralang.bootstrap_packaging.p02_019.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapPackageUnitDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub owner_root: &'static str,
    pub artifacts: &'static [&'static str],
    pub commands: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapReleaseBundleDescriptor {
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
pub struct BootstrapDistributionCheckDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub target: &'static str,
    pub requires: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapPackagingProofDescriptor {
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

pub const LYRALANG_BOOTSTRAP_PACKAGING_PACKAGES: &[BootstrapPackageUnitDescriptor] = &[
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_trust_binaries",
        kind: "binary_group",
        owner_root: "src",
        artifacts: &[
            "src/bin/lyra-p02-bootstrap-packaging-check.rs",
            "src/bin/lyra-p02-bootstrap-interface-check.rs",
            "src/bin/lyra-p02-bootstrap-replay-check.rs",
        ],
        commands: &[
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-interface-check",
            "lyra-p02-bootstrap-replay-check",
        ],
        receipts: &["receipts/p02/pass_0077_bootstrap_packaging.receipt"],
        status: "artifact_emitted",
    },
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_contract_model_set",
        kind: "contract_set",
        owner_root: "interfaces",
        artifacts: &[
            "interfaces/p02/contracts/bootstrap_packaging.v1.lyra",
            "interfaces/p02/src/bootstrap_packaging_model.rs",
        ],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_contract_model_set.receipt"],
        status: "artifact_emitted",
    },
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_control_law_set",
        kind: "control_plane",
        owner_root: "ops",
        artifacts: &[
            "ops/p02/packaging/bootstrap_packaging.v1.lyra",
            "ops/p02/src/bootstrap_packaging.rs",
            "ops/p02/control/frontier_lock.v1.lyra",
            "ops/p02/control/truth_snapshot.v1.lyra",
            "ops/p02/control/blocker_index.v1.lyra",
        ],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_control_law_set.receipt"],
        status: "artifact_emitted",
    },
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_positive_negative_corpus",
        kind: "corpus",
        owner_root: "fixtures",
        artifacts: &[
            "fixtures/p02/bootstrap_packaging_inputs/valid_bootstrap_packaging.lyra",
            "fixtures/p02/bootstrap_packaging_inputs/invalid_missing_rule.lyra",
            "fixtures/p02/bootstrap_packaging_inputs/invalid_network_required.lyra",
            "fixtures/p02/bootstrap_packaging_inputs/invalid_phase_closure_claim.lyra",
        ],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_positive_negative_corpus.receipt"],
        status: "artifact_emitted",
    },
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_packaging_goldens",
        kind: "golden_set",
        owner_root: "goldens",
        artifacts: &[
            "goldens/p02/valid_bootstrap_packaging.receipt",
            "goldens/p02/bootstrap_packaging_suite_report.v1.lyra",
        ],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_packaging_goldens.receipt"],
        status: "artifact_emitted",
    },
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_receipt_chain",
        kind: "receipt_set",
        owner_root: "receipts",
        artifacts: &[
            "receipts/p02/pass_0077_bootstrap_packaging.receipt",
            "receipts/p02/bootstrap_packaging/bootstrap_manifest_chain.receipt",
            "receipts/p02/bootstrap_packaging/bootstrap_distribution_gate.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-replay-check",
            "lyra-p02-bootstrap-packaging-check",
        ],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_receipt_chain.receipt"],
        status: "artifact_emitted",
    },
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_product_surfaces",
        kind: "product_surface",
        owner_root: "products",
        artifacts: &[
            "products/p02/bootstrap_package_manifest.v1.lyra",
            "products/p02/bootstrap_release_bundle_manifest.v1.lyra",
            "products/p02/bootstrap_operator_distribution_readme.v1.lyra",
        ],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_product_surfaces.receipt"],
        status: "artifact_emitted",
    },
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_operator_examples",
        kind: "example_set",
        owner_root: "examples",
        artifacts: &[
            "examples/p02/bootstrap_packaging/local_install_example.lyra",
            "examples/p02/bootstrap_packaging/seed_runtime_package_example.lyra",
            "examples/p02/bootstrap_packaging/host_extinction_package_example.lyra",
            "examples/p02/bootstrap_packaging/replay_bundle_example.lyra",
        ],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_operator_examples.receipt"],
        status: "artifact_emitted",
    },
    BootstrapPackageUnitDescriptor {
        id: "bootstrap_shell_surfaces",
        kind: "shell_surface",
        owner_root: "shells",
        artifacts: &[
            "shells/p02/bootstrap_packaging_shell.v1.lyra",
            "shells/p02/bootstrap_operator_interface_shell.v1.lyra",
        ],
        commands: &[
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-interface-check",
        ],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_shell_surfaces.receipt"],
        status: "artifact_emitted",
    },
];
pub const LYRALANG_BOOTSTRAP_PACKAGING_BUNDLES: &[BootstrapReleaseBundleDescriptor] = &[
    BootstrapReleaseBundleDescriptor {
        id: "p02_local_bootstrap_package_bundle",
        order: "001",
        packages: &[
            "bootstrap_trust_binaries",
            "bootstrap_contract_model_set",
            "bootstrap_control_law_set",
            "bootstrap_shell_surfaces",
        ],
        artifacts: &["products/p02/bootstrap_package_manifest.v1.lyra"],
        receipts: &["receipts/p02/bootstrap_packaging/p02_local_bootstrap_package_bundle.receipt"],
        checks: &[
            "offline_installable",
            "manifest_hash_stable",
            "command_set_complete",
        ],
        forbids: &["remote_fetch", "ambient_time", "probabilistic_truth"],
        status: "artifact_emitted",
    },
    BootstrapReleaseBundleDescriptor {
        id: "p02_seed_runtime_package_bundle",
        order: "002",
        packages: &[
            "bootstrap_trust_binaries",
            "bootstrap_control_law_set",
            "bootstrap_receipt_chain",
            "bootstrap_product_surfaces",
        ],
        artifacts: &["products/p02/bootstrap_release_bundle_manifest.v1.lyra"],
        receipts: &["receipts/p02/bootstrap_packaging/p02_seed_runtime_package_bundle.receipt"],
        checks: &[
            "artifact_paths_bound",
            "receipt_paths_bound",
            "command_set_complete",
        ],
        forbids: &[
            "seed_runtime_unreceipted",
            "network_dependency",
            "package_drift",
        ],
        status: "artifact_emitted",
    },
    BootstrapReleaseBundleDescriptor {
        id: "p02_host_extinction_package_bundle",
        order: "003",
        packages: &[
            "bootstrap_control_law_set",
            "bootstrap_positive_negative_corpus",
            "bootstrap_receipt_chain",
            "bootstrap_operator_examples",
        ],
        artifacts: &["examples/p02/bootstrap_packaging/host_extinction_package_example.lyra"],
        receipts: &["receipts/p02/bootstrap_packaging/p02_host_extinction_package_bundle.receipt"],
        checks: &[
            "artifact_paths_bound",
            "receipt_paths_bound",
            "no_remote_fetch_check",
        ],
        forbids: &[
            "host_dependency_hidden",
            "foreign_surface_untracked",
            "phase_closure_claim",
        ],
        status: "artifact_emitted",
    },
    BootstrapReleaseBundleDescriptor {
        id: "p02_offline_distribution_bundle",
        order: "004",
        packages: &[
            "bootstrap_trust_binaries",
            "bootstrap_contract_model_set",
            "bootstrap_control_law_set",
            "bootstrap_positive_negative_corpus",
            "bootstrap_packaging_goldens",
            "bootstrap_receipt_chain",
            "bootstrap_product_surfaces",
            "bootstrap_operator_examples",
            "bootstrap_shell_surfaces",
        ],
        artifacts: &[
            "products/p02/bootstrap_release_bundle_manifest.v1.lyra",
            "goldens/p02/bootstrap_packaging_suite_report.v1.lyra",
        ],
        receipts: &["receipts/p02/bootstrap_packaging/p02_offline_distribution_bundle.receipt"],
        checks: &[
            "offline_installable",
            "manifest_hash_stable",
            "artifact_paths_bound",
            "receipt_paths_bound",
            "no_remote_fetch_check",
            "command_set_complete",
            "product_surface_bound",
        ],
        forbids: &[
            "remote_fetch",
            "unreceipted_package",
            "package_drift",
            "phase_closure_claim",
        ],
        status: "artifact_emitted",
    },
];
pub const LYRALANG_BOOTSTRAP_PACKAGING_CHECKS: &[BootstrapDistributionCheckDescriptor] = &[
    BootstrapDistributionCheckDescriptor {
        id: "offline_installable",
        scope: "distribution",
        target: "p02_offline_distribution_bundle",
        requires: &[
            "p02_offline_distribution_bundle",
            "bootstrap_shell_surfaces",
        ],
        forbids: &["remote_fetch", "network_dependency"],
        receipts: &["receipts/p02/bootstrap_packaging/offline_installable.receipt"],
        status: "artifact_emitted",
    },
    BootstrapDistributionCheckDescriptor {
        id: "manifest_hash_stable",
        scope: "bundle",
        target: "p02_local_bootstrap_package_bundle",
        requires: &[
            "bootstrap_contract_model_set",
            "bootstrap_packaging_goldens",
        ],
        forbids: &["manifest_drift", "unstable_order"],
        receipts: &["receipts/p02/bootstrap_packaging/manifest_hash_stable.receipt"],
        status: "artifact_emitted",
    },
    BootstrapDistributionCheckDescriptor {
        id: "artifact_paths_bound",
        scope: "package",
        target: "bootstrap_control_law_set",
        requires: &["bootstrap_control_law_set", "bootstrap_product_surfaces"],
        forbids: &["unknown_artifact", "path_escape"],
        receipts: &["receipts/p02/bootstrap_packaging/artifact_paths_bound.receipt"],
        status: "artifact_emitted",
    },
    BootstrapDistributionCheckDescriptor {
        id: "receipt_paths_bound",
        scope: "package",
        target: "bootstrap_receipt_chain",
        requires: &["bootstrap_receipt_chain", "bootstrap_packaging_goldens"],
        forbids: &["orphan_receipt", "receipt_hash_mismatch"],
        receipts: &["receipts/p02/bootstrap_packaging/receipt_paths_bound.receipt"],
        status: "artifact_emitted",
    },
    BootstrapDistributionCheckDescriptor {
        id: "no_remote_fetch_check",
        scope: "distribution",
        target: "p02_offline_distribution_bundle",
        requires: &["bootstrap_trust_binaries", "bootstrap_product_surfaces"],
        forbids: &["remote_fetch", "cloud_dependency"],
        receipts: &["receipts/p02/bootstrap_packaging/no_remote_fetch_check.receipt"],
        status: "artifact_emitted",
    },
    BootstrapDistributionCheckDescriptor {
        id: "command_set_complete",
        scope: "package",
        target: "bootstrap_trust_binaries",
        requires: &["bootstrap_trust_binaries", "bootstrap_shell_surfaces"],
        forbids: &["manual_only_path", "missing_cli"],
        receipts: &["receipts/p02/bootstrap_packaging/command_set_complete.receipt"],
        status: "artifact_emitted",
    },
    BootstrapDistributionCheckDescriptor {
        id: "product_surface_bound",
        scope: "product",
        target: "bootstrap_product_surfaces",
        requires: &["bootstrap_product_surfaces", "bootstrap_operator_examples"],
        forbids: &["docs_only_product", "unreceipted_product"],
        receipts: &["receipts/p02/bootstrap_packaging/product_surface_bound.receipt"],
        status: "artifact_emitted",
    },
];
pub const LYRALANG_BOOTSTRAP_PACKAGING_PROOFS: &[BootstrapPackagingProofDescriptor] = &[
    BootstrapPackagingProofDescriptor {
        id: "bootstrap_package_manifest_coverage",
        scope: "package",
        packages: &[
            "bootstrap_trust_binaries",
            "bootstrap_contract_model_set",
            "bootstrap_control_law_set",
            "bootstrap_product_surfaces",
        ],
        bundles: &["p02_local_bootstrap_package_bundle"],
        checks: &["manifest_hash_stable", "artifact_paths_bound"],
        receipts: &["receipts/p02/bootstrap_packaging/bootstrap_package_manifest_coverage.receipt"],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        forbids: &["manifest_drift", "unknown_artifact"],
        status: "artifact_emitted",
    },
    BootstrapPackagingProofDescriptor {
        id: "bootstrap_release_bundle_determinism",
        scope: "bundle",
        packages: &["bootstrap_receipt_chain", "bootstrap_packaging_goldens"],
        bundles: &["p02_offline_distribution_bundle"],
        checks: &["manifest_hash_stable", "receipt_paths_bound"],
        receipts: &[
            "receipts/p02/bootstrap_packaging/bootstrap_release_bundle_determinism.receipt",
        ],
        commands: &[
            "lyra-p02-bootstrap-packaging-check",
            "lyra-p02-bootstrap-replay-check",
        ],
        forbids: &["unstable_order", "receipt_hash_mismatch"],
        status: "artifact_emitted",
    },
    BootstrapPackagingProofDescriptor {
        id: "offline_distribution_gate",
        scope: "distribution",
        packages: &[
            "bootstrap_trust_binaries",
            "bootstrap_shell_surfaces",
            "bootstrap_product_surfaces",
        ],
        bundles: &["p02_offline_distribution_bundle"],
        checks: &[
            "offline_installable",
            "no_remote_fetch_check",
            "command_set_complete",
        ],
        receipts: &["receipts/p02/bootstrap_packaging/offline_distribution_gate.receipt"],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        forbids: &["remote_fetch", "cloud_dependency"],
        status: "artifact_emitted",
    },
    BootstrapPackagingProofDescriptor {
        id: "product_surface_binding",
        scope: "product",
        packages: &["bootstrap_product_surfaces", "bootstrap_operator_examples"],
        bundles: &[
            "p02_seed_runtime_package_bundle",
            "p02_host_extinction_package_bundle",
        ],
        checks: &["product_surface_bound", "artifact_paths_bound"],
        receipts: &["receipts/p02/bootstrap_packaging/product_surface_binding.receipt"],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        forbids: &["docs_only_product", "unreceipted_product"],
        status: "artifact_emitted",
    },
    BootstrapPackagingProofDescriptor {
        id: "p02_phase_open",
        scope: "phase",
        packages: &[
            "bootstrap_trust_binaries",
            "bootstrap_contract_model_set",
            "bootstrap_control_law_set",
            "bootstrap_positive_negative_corpus",
            "bootstrap_packaging_goldens",
            "bootstrap_receipt_chain",
            "bootstrap_product_surfaces",
            "bootstrap_operator_examples",
            "bootstrap_shell_surfaces",
        ],
        bundles: &[
            "p02_local_bootstrap_package_bundle",
            "p02_seed_runtime_package_bundle",
            "p02_host_extinction_package_bundle",
            "p02_offline_distribution_bundle",
        ],
        checks: &[
            "offline_installable",
            "manifest_hash_stable",
            "artifact_paths_bound",
            "receipt_paths_bound",
            "no_remote_fetch_check",
            "command_set_complete",
            "product_surface_bound",
        ],
        receipts: &["receipts/p02/pass_0077_bootstrap_packaging.receipt"],
        commands: &["lyra-p02-bootstrap-packaging-check"],
        forbids: &["phase_closure_claim", "global_complete"],
        status: "artifact_emitted",
    },
];

pub fn bootstrap_packaging_package_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PACKAGING_PACKAGES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_packaging_bundle_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PACKAGING_BUNDLES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_packaging_check_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PACKAGING_CHECKS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_packaging_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_PACKAGING_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_packaging_package_descriptor(
    id: &str,
) -> Option<&'static BootstrapPackageUnitDescriptor> {
    LYRALANG_BOOTSTRAP_PACKAGING_PACKAGES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_packaging_bundle_descriptor(
    id: &str,
) -> Option<&'static BootstrapReleaseBundleDescriptor> {
    LYRALANG_BOOTSTRAP_PACKAGING_BUNDLES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_packaging_check_descriptor(
    id: &str,
) -> Option<&'static BootstrapDistributionCheckDescriptor> {
    LYRALANG_BOOTSTRAP_PACKAGING_CHECKS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_packaging_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapPackagingProofDescriptor> {
    LYRALANG_BOOTSTRAP_PACKAGING_PROOFS
        .iter()
        .find(|item| item.id == id)
}

fn join_static(values: &[&'static str]) -> String {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values.join(",")
}

pub fn bootstrap_packaging_package_digest(id: &str) -> Option<String> {
    let item = bootstrap_packaging_package_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_packaging.package_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}|{}",
            item.id,
            item.kind,
            item.owner_root,
            join_static(item.artifacts),
            join_static(item.commands),
            join_static(item.receipts),
            item.status
        ),
    ))
}
pub fn bootstrap_packaging_bundle_digest(id: &str) -> Option<String> {
    let item = bootstrap_packaging_bundle_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_packaging.bundle_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            item.id,
            item.order,
            join_static(item.packages),
            join_static(item.artifacts),
            join_static(item.receipts),
            join_static(item.checks),
            join_static(item.forbids),
            item.status
        ),
    ))
}
pub fn bootstrap_packaging_check_digest(id: &str) -> Option<String> {
    let item = bootstrap_packaging_check_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_packaging.check_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}|{}",
            item.id,
            item.scope,
            item.target,
            join_static(item.requires),
            join_static(item.forbids),
            join_static(item.receipts),
            item.status
        ),
    ))
}
pub fn bootstrap_packaging_proof_digest(id: &str) -> Option<String> {
    let item = bootstrap_packaging_proof_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_packaging.proof_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            item.id,
            item.scope,
            join_static(item.packages),
            join_static(item.bundles),
            join_static(item.checks),
            join_static(item.receipts),
            join_static(item.commands),
            join_static(item.forbids),
            item.status
        ),
    ))
}

pub fn bootstrap_packaging_registry_signature() -> String {
    let mut lines = Vec::new();
    for id in bootstrap_packaging_package_ids() {
        lines.push(format!("package:{id}"));
    }
    for id in bootstrap_packaging_bundle_ids() {
        lines.push(format!("bundle:{id}"));
    }
    for id in bootstrap_packaging_check_ids() {
        lines.push(format!("check:{id}"));
    }
    for id in bootstrap_packaging_proof_ids() {
        lines.push(format!("proof:{id}"));
    }
    lines.sort();
    lines.join("\n")
}
pub fn bootstrap_packaging_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_packaging.registry",
        &bootstrap_packaging_registry_signature(),
    )
}
pub fn bootstrap_packaging_carrier_signature() -> String {
    format!(
        "{}|{}",
        LYRA_P02_BOOTSTRAP_PACKAGING_CARRIER,
        bootstrap_packaging_registry_hash()
    )
}

pub fn bootstrap_packaging_bundles_bind_registry() -> bool {
    let packages = bootstrap_packaging_package_ids();
    let checks = bootstrap_packaging_check_ids();
    LYRALANG_BOOTSTRAP_PACKAGING_BUNDLES.iter().all(|item| {
        item.packages
            .iter()
            .all(|package| packages.contains(package))
            && item.checks.iter().all(|check| checks.contains(check))
            && !item.artifacts.is_empty()
            && !item.receipts.is_empty()
    })
}
pub fn bootstrap_packaging_checks_bind_registry() -> bool {
    let packages = bootstrap_packaging_package_ids();
    let bundles = bootstrap_packaging_bundle_ids();
    LYRALANG_BOOTSTRAP_PACKAGING_CHECKS.iter().all(|item| {
        !item.receipts.is_empty()
            && (packages.contains(&item.target) || bundles.contains(&item.target))
    })
}
pub fn bootstrap_packaging_proofs_bind_registry() -> bool {
    let packages = bootstrap_packaging_package_ids();
    let bundles = bootstrap_packaging_bundle_ids();
    let checks = bootstrap_packaging_check_ids();
    LYRALANG_BOOTSTRAP_PACKAGING_PROOFS.iter().all(|item| {
        item.packages
            .iter()
            .all(|package| packages.contains(package))
            && item.bundles.iter().all(|bundle| bundles.contains(bundle))
            && item.checks.iter().all(|check| checks.contains(check))
            && !item.receipts.is_empty()
            && !item.commands.is_empty()
    })
}
pub fn bootstrap_packaging_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_PACKAGING_PACKAGES.iter().all(|item| {
        item.artifacts.iter().all(|path| valid_artifact_path(path))
            && item.receipts.iter().all(|path| valid_receipt_path(path))
    }) && LYRALANG_BOOTSTRAP_PACKAGING_BUNDLES.iter().all(|item| {
        item.artifacts.iter().all(|path| valid_artifact_path(path))
            && item.receipts.iter().all(|path| valid_receipt_path(path))
    }) && LYRALANG_BOOTSTRAP_PACKAGING_CHECKS
        .iter()
        .all(|item| item.receipts.iter().all(|path| valid_receipt_path(path)))
        && LYRALANG_BOOTSTRAP_PACKAGING_PROOFS
            .iter()
            .all(|item| item.receipts.iter().all(|path| valid_receipt_path(path)))
}
pub fn bootstrap_packaging_units_cover_p02_001_through_p02_019() -> bool {
    bootstrap_packaging_package_ids().len() >= 9
        && bootstrap_packaging_bundle_ids().len() >= 4
        && bootstrap_packaging_proof_ids().len() >= 5
}
pub fn bootstrap_packaging_no_forbidden_descriptor_claims() -> bool {
    let signature = bootstrap_packaging_registry_signature().to_ascii_lowercase();
    !(signature.contains("network_required")
        || signature.contains("remote_fetch_required")
        || signature.contains("package_drift_accepted")
        || signature.contains("phase_complete"))
}

fn valid_artifact_path(path: &str) -> bool {
    let allowed = [
        "src/",
        "interfaces/",
        "ops/",
        "fixtures/",
        "goldens/",
        "receipts/",
        "examples/",
        "products/",
        "docs/",
        "tests/",
        "shells/",
    ];
    !path.contains("..") && allowed.iter().any(|prefix| path.starts_with(prefix))
}
fn valid_receipt_path(path: &str) -> bool {
    !path.contains("..") && path.starts_with("receipts/p02/") && path.ends_with(".receipt")
}
