use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_packaging::deterministic_bootstrap_packaging_suite_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_packaging::{
    bootstrap_packaging_artifacts_bind_paths, bootstrap_packaging_bundle_descriptor,
    bootstrap_packaging_bundle_digest, bootstrap_packaging_bundles_bind_registry,
    bootstrap_packaging_carrier_signature, bootstrap_packaging_check_descriptor,
    bootstrap_packaging_check_digest, bootstrap_packaging_checks_bind_registry,
    bootstrap_packaging_no_forbidden_descriptor_claims, bootstrap_packaging_package_descriptor,
    bootstrap_packaging_package_digest, bootstrap_packaging_proof_descriptor,
    bootstrap_packaging_proof_digest, bootstrap_packaging_proofs_bind_registry,
    bootstrap_packaging_registry_hash, bootstrap_packaging_units_cover_p02_001_through_p02_019,
};
use crate::p02_bootstrap_packaging_model::{
    BootstrapDistributionCheckBinding, BootstrapPackageUnitBinding, BootstrapPackagingProofBinding,
    BootstrapPackagingSurface, BootstrapReleaseBundleBinding,
};

pub const P02_BOOTSTRAP_PACKAGING_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-PACKAGING v1";

pub const REQUIRED_BOOTSTRAP_PACKAGING_RULES: &[&str] = &[
    "bootstrap_package_manifest_required",
    "bootstrap_release_bundle_required",
    "bootstrap_product_surface_required",
    "bootstrap_seed_runtime_package_required",
    "bootstrap_host_extinction_package_required",
    "bootstrap_replay_witness_package_required",
    "bootstrap_cli_package_required",
    "offline_distribution_required",
    "artifact_hash_required",
    "receipt_chain_required",
    "no_remote_fetch",
    "no_unreceipted_package",
    "no_package_drift_acceptance",
    "no_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_PACKAGE_UNITS: &[&str] = &[
    "bootstrap_trust_binaries",
    "bootstrap_contract_model_set",
    "bootstrap_control_law_set",
    "bootstrap_positive_negative_corpus",
    "bootstrap_packaging_goldens",
    "bootstrap_receipt_chain",
    "bootstrap_product_surfaces",
    "bootstrap_operator_examples",
    "bootstrap_shell_surfaces",
];
pub const REQUIRED_BOOTSTRAP_RELEASE_BUNDLES: &[&str] = &[
    "p02_local_bootstrap_package_bundle",
    "p02_seed_runtime_package_bundle",
    "p02_host_extinction_package_bundle",
    "p02_offline_distribution_bundle",
];
pub const REQUIRED_BOOTSTRAP_DISTRIBUTION_CHECKS: &[&str] = &[
    "offline_installable",
    "manifest_hash_stable",
    "artifact_paths_bound",
    "receipt_paths_bound",
    "no_remote_fetch_check",
    "command_set_complete",
    "product_surface_bound",
];
pub const REQUIRED_BOOTSTRAP_PACKAGING_PROOFS: &[&str] = &[
    "bootstrap_package_manifest_coverage",
    "bootstrap_release_bundle_determinism",
    "offline_distribution_gate",
    "product_surface_binding",
    "p02_phase_open",
];

const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_PACKAGE_KINDS: &[&str] = &[
    "binary_group",
    "contract_set",
    "control_plane",
    "corpus",
    "golden_set",
    "receipt_set",
    "product_surface",
    "example_set",
    "shell_surface",
];
const ALLOWED_OWNER_ROOTS: &[&str] = &[
    "src",
    "interfaces",
    "ops",
    "fixtures",
    "goldens",
    "receipts",
    "examples",
    "products",
    "docs",
    "tests",
    "shells",
];
const ALLOWED_CHECK_SCOPES: &[&str] = &["package", "bundle", "distribution", "product", "phase"];
const ALLOWED_PROOF_SCOPES: &[&str] = &["package", "bundle", "distribution", "product", "phase"];
const FORBIDDEN_BOOTSTRAP_PACKAGING_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::PackagingNetworkDependency),
    ("cloud required", ErrorCode::PackagingNetworkDependency),
    ("online required", ErrorCode::PackagingNetworkDependency),
    (
        "remote fetch required",
        ErrorCode::PackagingNetworkDependency,
    ),
    ("package drift accepted", ErrorCode::PackagingDriftAccepted),
    ("release drift accepted", ErrorCode::PackagingDriftAccepted),
    (
        "unreceipted package action",
        ErrorCode::PackagingDriftAccepted,
    ),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_bootstrap_packaging_surface(
    input: &str,
) -> Result<BootstrapPackagingSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => {
            return Err(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "input",
                format!("{error:?}"),
            )])
        }
    };
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "empty bootstrap packaging surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P02_BOOTSTRAP_PACKAGING_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_PACKAGING_CONTRACT}"),
        ));
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut packages = Vec::new();
    let mut bundles = Vec::new();
    let mut checks = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_packages = BTreeSet::new();
    let mut seen_bundles = BTreeSet::new();
    let mut seen_checks = BTreeSet::new();
    let mut seen_proofs = BTreeSet::new();
    let mut seen_orders = BTreeSet::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
        let Some((left, value)) = line.split_once('=') else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry must contain one equals separator",
            ));
            continue;
        };
        if left.is_empty() || value.is_empty() || left != left.trim() || value != value.trim() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry sides must be non-empty and trimmed",
            ));
            continue;
        }
        if let Some(rule_name) = left.strip_prefix("rule:") {
            if !is_symbolic_name(rule_name) || !seen_rules.insert(rule_name.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingPackagingRule,
                    format!("line:{line_number:03}"),
                    "rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        match left {
            "phase" => {
                if !seen_scalars.insert(left.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateEntry,
                        format!("line:{line_number:03}"),
                        "duplicate phase",
                    ));
                }
                phase = Some(value.to_string());
            }
            "task" => {
                if !seen_scalars.insert(left.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateEntry,
                        format!("line:{line_number:03}"),
                        "duplicate task",
                    ));
                }
                task = Some(value.to_string());
            }
            "status" => {
                if !seen_scalars.insert(left.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateEntry,
                        format!("line:{line_number:03}"),
                        "duplicate status",
                    ));
                }
                status = Some(value.to_string());
            }
            "package" => {
                let fields = parse_pipe_fields(value);
                require_fields(
                    &fields,
                    &[
                        "id",
                        "kind",
                        "owner_root",
                        "artifacts",
                        "commands",
                        "receipts",
                        "status",
                    ],
                    "package",
                    line_number,
                    &mut errors,
                );
                let id = field(&fields, "id");
                if !is_symbolic_name(&id) || !seen_packages.insert(id.clone()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicatePackageUnit,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid package {id}"),
                    ));
                }
                packages.push(BootstrapPackageUnitBinding {
                    line_number,
                    id,
                    kind: field(&fields, "kind"),
                    owner_root: field(&fields, "owner_root"),
                    artifacts: list_field(&fields, "artifacts"),
                    commands: list_field(&fields, "commands"),
                    receipts: list_field(&fields, "receipts"),
                    status: field(&fields, "status"),
                });
            }
            "bundle" => {
                let fields = parse_pipe_fields(value);
                require_fields(
                    &fields,
                    &[
                        "id",
                        "order",
                        "packages",
                        "artifacts",
                        "receipts",
                        "checks",
                        "forbids",
                        "status",
                    ],
                    "bundle",
                    line_number,
                    &mut errors,
                );
                let id = field(&fields, "id");
                let order = field(&fields, "order");
                if !is_symbolic_name(&id) || !seen_bundles.insert(id.clone()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateReleaseBundle,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid bundle {id}"),
                    ));
                }
                if order.len() != 3
                    || !order.bytes().all(|byte| byte.is_ascii_digit())
                    || !seen_orders.insert(order.clone())
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidReleaseBundle,
                        format!("line:{line_number:03}"),
                        format!("invalid or duplicate bundle order {order}"),
                    ));
                }
                bundles.push(BootstrapReleaseBundleBinding {
                    line_number,
                    id,
                    order,
                    packages: list_field(&fields, "packages"),
                    artifacts: list_field(&fields, "artifacts"),
                    receipts: list_field(&fields, "receipts"),
                    checks: list_field(&fields, "checks"),
                    forbids: list_field(&fields, "forbids"),
                    status: field(&fields, "status"),
                });
            }
            "check" => {
                let fields = parse_pipe_fields(value);
                require_fields(
                    &fields,
                    &[
                        "id", "scope", "target", "requires", "forbids", "receipts", "status",
                    ],
                    "check",
                    line_number,
                    &mut errors,
                );
                let id = field(&fields, "id");
                if !is_symbolic_name(&id) || !seen_checks.insert(id.clone()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateDistributionCheck,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid check {id}"),
                    ));
                }
                checks.push(BootstrapDistributionCheckBinding {
                    line_number,
                    id,
                    scope: field(&fields, "scope"),
                    target: field(&fields, "target"),
                    requires: list_field(&fields, "requires"),
                    forbids: list_field(&fields, "forbids"),
                    receipts: list_field(&fields, "receipts"),
                    status: field(&fields, "status"),
                });
            }
            "proof" => {
                let fields = parse_pipe_fields(value);
                require_fields(
                    &fields,
                    &[
                        "id", "scope", "packages", "bundles", "checks", "receipts", "commands",
                        "forbids", "status",
                    ],
                    "proof",
                    line_number,
                    &mut errors,
                );
                let id = field(&fields, "id");
                if !is_symbolic_name(&id) || !seen_proofs.insert(id.clone()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicatePackagingProof,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid proof {id}"),
                    ));
                }
                proofs.push(BootstrapPackagingProofBinding {
                    line_number,
                    id,
                    scope: field(&fields, "scope"),
                    packages: list_field(&fields, "packages"),
                    bundles: list_field(&fields, "bundles"),
                    checks: list_field(&fields, "checks"),
                    receipts: list_field(&fields, "receipts"),
                    commands: list_field(&fields, "commands"),
                    forbids: list_field(&fields, "forbids"),
                    status: field(&fields, "status"),
                });
            }
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown entry key {left}"),
            )),
        }
    }

    if errors.is_empty() {
        match (&phase, &task, &status) {
            (Some(phase_value), Some(task_value), Some(status_value)) => {
                if phase_value != "P02" {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidPhase,
                        "phase",
                        "bootstrap packaging must bind P02",
                    ));
                }
                if task_value != "P02-019" {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidTask,
                        "task",
                        "bootstrap packaging must bind P02-019",
                    ));
                }
                if !ALLOWED_STATUSES.contains(&status_value.as_str()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::UnsupportedEvidenceClaim,
                        "status",
                        format!("unsupported status {status_value}"),
                    ));
                }
            }
            _ => {
                if phase.is_none() {
                    errors.push(ValidationError::reject(
                        ErrorCode::MissingPhase,
                        "phase",
                        "missing phase",
                    ));
                }
                if task.is_none() {
                    errors.push(ValidationError::reject(
                        ErrorCode::MissingTask,
                        "task",
                        "missing task",
                    ));
                }
                if status.is_none() {
                    errors.push(ValidationError::reject(
                        ErrorCode::UnsupportedEvidenceClaim,
                        "status",
                        "missing status",
                    ));
                }
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(BootstrapPackagingSurface {
        header,
        phase: phase.unwrap(),
        task: task.unwrap(),
        status: status.unwrap(),
        rules,
        packages,
        bundles,
        checks,
        proofs,
    })
}

pub fn validate_bootstrap_packaging_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let verdict = match parse_bootstrap_packaging_surface(input) {
        Ok(surface) => validate_bootstrap_packaging_model(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_packaging_model(
    surface: &BootstrapPackagingSurface,
    raw_input: &str,
) -> Verdict {
    let mut errors = Vec::new();
    scan_forbidden_text(raw_input, &mut errors);
    require_ids(
        "rule",
        REQUIRED_BOOTSTRAP_PACKAGING_RULES,
        surface.rules.keys().map(String::as_str).collect(),
        ErrorCode::MissingPackagingRule,
        &mut errors,
    );
    let package_ids: BTreeSet<&str> = surface
        .packages
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let bundle_ids: BTreeSet<&str> = surface
        .bundles
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let check_ids: BTreeSet<&str> = surface.checks.iter().map(|item| item.id.as_str()).collect();
    let proof_ids: BTreeSet<&str> = surface.proofs.iter().map(|item| item.id.as_str()).collect();
    require_ids(
        "package",
        REQUIRED_BOOTSTRAP_PACKAGE_UNITS,
        package_ids.clone(),
        ErrorCode::MissingPackageUnit,
        &mut errors,
    );
    require_ids(
        "bundle",
        REQUIRED_BOOTSTRAP_RELEASE_BUNDLES,
        bundle_ids.clone(),
        ErrorCode::MissingReleaseBundle,
        &mut errors,
    );
    require_ids(
        "check",
        REQUIRED_BOOTSTRAP_DISTRIBUTION_CHECKS,
        check_ids.clone(),
        ErrorCode::MissingDistributionCheck,
        &mut errors,
    );
    require_ids(
        "proof",
        REQUIRED_BOOTSTRAP_PACKAGING_PROOFS,
        proof_ids,
        ErrorCode::MissingPackagingProof,
        &mut errors,
    );

    for package in &surface.packages {
        validate_status(
            "package",
            &package.id,
            package.line_number,
            &package.status,
            &mut errors,
        );
        if !ALLOWED_PACKAGE_KINDS.contains(&package.kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackageUnit,
                format!("line:{:03}", package.line_number),
                format!("package {} invalid kind {}", package.id, package.kind),
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&package.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackageUnit,
                format!("line:{:03}", package.line_number),
                format!(
                    "package {} invalid owner root {}",
                    package.id, package.owner_root
                ),
            ));
        }
        if package.artifacts.is_empty()
            || package.commands.is_empty()
            || package.receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackageUnit,
                format!("line:{:03}", package.line_number),
                format!(
                    "package {} requires artifacts, commands, and receipts",
                    package.id
                ),
            ));
        }
        for artifact in &package.artifacts {
            validate_artifact_path(
                artifact,
                package.line_number,
                ErrorCode::InvalidPackageUnit,
                &mut errors,
            );
        }
        for receipt in &package.receipts {
            validate_receipt_path(
                receipt,
                package.line_number,
                ErrorCode::InvalidPackageUnit,
                &mut errors,
            );
        }
        for command in &package.commands {
            if !command.starts_with("lyra-p02-") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPackageUnit,
                    format!("line:{:03}", package.line_number),
                    format!("package {} invalid command {command}", package.id),
                ));
            }
        }
        if bootstrap_packaging_package_descriptor(&package.id).is_some()
            && bootstrap_packaging_package_digest(&package.id)
                .unwrap_or_default()
                .is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::PackagingDriftAccepted,
                format!("line:{:03}", package.line_number),
                format!("package {} descriptor digest empty", package.id),
            ));
        }
    }
    for bundle in &surface.bundles {
        validate_status(
            "bundle",
            &bundle.id,
            bundle.line_number,
            &bundle.status,
            &mut errors,
        );
        if bundle.packages.is_empty()
            || bundle.artifacts.is_empty()
            || bundle.receipts.is_empty()
            || bundle.checks.is_empty()
            || bundle.forbids.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseBundle,
                format!("line:{:03}", bundle.line_number),
                format!(
                    "bundle {} requires package, artifact, receipt, check, and forbidden bindings",
                    bundle.id
                ),
            ));
        }
        for package_id in &bundle.packages {
            if !package_ids.contains(package_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReleaseBundle,
                    format!("line:{:03}", bundle.line_number),
                    format!(
                        "bundle {} references unknown package {package_id}",
                        bundle.id
                    ),
                ));
            }
        }
        for check_id in &bundle.checks {
            if !check_ids.contains(check_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReleaseBundle,
                    format!("line:{:03}", bundle.line_number),
                    format!("bundle {} references unknown check {check_id}", bundle.id),
                ));
            }
        }
        for artifact in &bundle.artifacts {
            validate_artifact_path(
                artifact,
                bundle.line_number,
                ErrorCode::InvalidReleaseBundle,
                &mut errors,
            );
        }
        for receipt in &bundle.receipts {
            validate_receipt_path(
                receipt,
                bundle.line_number,
                ErrorCode::InvalidReleaseBundle,
                &mut errors,
            );
        }
        if bootstrap_packaging_bundle_descriptor(&bundle.id).is_some()
            && bootstrap_packaging_bundle_digest(&bundle.id)
                .unwrap_or_default()
                .is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::PackagingDriftAccepted,
                format!("line:{:03}", bundle.line_number),
                format!("bundle {} descriptor digest empty", bundle.id),
            ));
        }
    }
    for check in &surface.checks {
        validate_status(
            "check",
            &check.id,
            check.line_number,
            &check.status,
            &mut errors,
        );
        if !ALLOWED_CHECK_SCOPES.contains(&check.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                format!("line:{:03}", check.line_number),
                format!("check {} invalid scope {}", check.id, check.scope),
            ));
        }
        if check.requires.is_empty() || check.forbids.is_empty() || check.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                format!("line:{:03}", check.line_number),
                format!(
                    "check {} requires dependency, forbidden, and receipt bindings",
                    check.id
                ),
            ));
        }
        if check.scope == "package" && !package_ids.contains(check.target.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                format!("line:{:03}", check.line_number),
                format!(
                    "check {} targets unknown package {}",
                    check.id, check.target
                ),
            ));
        }
        if (check.scope == "bundle" || check.scope == "distribution")
            && !bundle_ids.contains(check.target.as_str())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                format!("line:{:03}", check.line_number),
                format!("check {} targets unknown bundle {}", check.id, check.target),
            ));
        }
        if check.scope == "product" && !package_ids.contains(check.target.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                format!("line:{:03}", check.line_number),
                format!(
                    "check {} targets unknown product package {}",
                    check.id, check.target
                ),
            ));
        }
        for dependency in &check.requires {
            if !package_ids.contains(dependency.as_str())
                && !bundle_ids.contains(dependency.as_str())
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDistributionCheck,
                    format!("line:{:03}", check.line_number),
                    format!(
                        "check {} requires unknown dependency {dependency}",
                        check.id
                    ),
                ));
            }
        }
        for receipt in &check.receipts {
            validate_receipt_path(
                receipt,
                check.line_number,
                ErrorCode::InvalidDistributionCheck,
                &mut errors,
            );
        }
        if bootstrap_packaging_check_descriptor(&check.id).is_some()
            && bootstrap_packaging_check_digest(&check.id)
                .unwrap_or_default()
                .is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::PackagingDriftAccepted,
                format!("line:{:03}", check.line_number),
                format!("check {} descriptor digest empty", check.id),
            ));
        }
    }
    for proof in &surface.proofs {
        validate_status(
            "proof",
            &proof.id,
            proof.line_number,
            &proof.status,
            &mut errors,
        );
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackagingProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} invalid scope {}", proof.id, proof.scope),
            ));
        }
        for package_id in &proof.packages {
            if !package_ids.contains(package_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown package {package_id}", proof.id),
                ));
            }
        }
        for bundle_id in &proof.bundles {
            if !bundle_ids.contains(bundle_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown bundle {bundle_id}", proof.id),
                ));
            }
        }
        for check_id in &proof.checks {
            if !check_ids.contains(check_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown check {check_id}", proof.id),
                ));
            }
        }
        if proof.receipts.is_empty() || proof.commands.is_empty() || proof.forbids.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackagingProof,
                format!("line:{:03}", proof.line_number),
                format!(
                    "proof {} needs receipts, commands, and forbidden claims",
                    proof.id
                ),
            ));
        }
        for receipt in &proof.receipts {
            validate_receipt_path(
                receipt,
                proof.line_number,
                ErrorCode::InvalidPackagingProof,
                &mut errors,
            );
        }
        for command in &proof.commands {
            if !command.starts_with("lyra-p02-") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPackagingProof,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} invalid command {command}", proof.id),
                ));
            }
        }
        if bootstrap_packaging_proof_descriptor(&proof.id).is_some()
            && bootstrap_packaging_proof_digest(&proof.id)
                .unwrap_or_default()
                .is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::PackagingDriftAccepted,
                format!("line:{:03}", proof.line_number),
                format!("proof {} descriptor digest empty", proof.id),
            ));
        }
    }

    if !bootstrap_packaging_bundles_bind_registry()
        || !bootstrap_packaging_checks_bind_registry()
        || !bootstrap_packaging_proofs_bind_registry()
        || !bootstrap_packaging_artifacts_bind_paths()
        || !bootstrap_packaging_units_cover_p02_001_through_p02_019()
        || !bootstrap_packaging_no_forbidden_descriptor_claims()
        || bootstrap_packaging_registry_hash().is_empty()
        || bootstrap_packaging_carrier_signature().is_empty()
    {
        errors.push(ValidationError::reject(
            ErrorCode::PackagingDriftAccepted,
            "lyralang",
            "bootstrap packaging registry failed binding checks",
        ));
    }

    let report = deterministic_bootstrap_packaging_suite_report(
        &surface
            .packages
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.kind.clone(),
                    item.owner_root.clone(),
                    item.artifacts.clone(),
                    item.commands.clone(),
                    item.receipts.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .bundles
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.order.clone(),
                    item.packages.clone(),
                    item.artifacts.clone(),
                    item.receipts.clone(),
                    item.checks.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .checks
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.scope.clone(),
                    item.target.clone(),
                    item.requires.clone(),
                    item.forbids.clone(),
                    item.receipts.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .proofs
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.scope.clone(),
                    item.packages.clone(),
                    item.bundles.clone(),
                    item.checks.clone(),
                    item.receipts.clone(),
                    item.commands.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
    );
    if report.package_count != surface.packages.len()
        || report.bundle_count != surface.bundles.len()
        || report.check_count != surface.checks.len()
        || report.proof_count != surface.proofs.len()
        || !report.suite_hash.starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::PackagingDriftAccepted,
            "deterministic-report",
            "bootstrap packaging report drift",
        ));
    }
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn scan_forbidden_text(input: &str, errors: &mut Vec<ValidationError>) {
    let lowered = input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_BOOTSTRAP_PACKAGING_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden-text",
                format!("forbidden bootstrap packaging text {needle}"),
            ));
        }
    }
}
fn validate_status(
    kind: &str,
    id: &str,
    line_number: usize,
    status: &str,
    errors: &mut Vec<ValidationError>,
) {
    if !ALLOWED_STATUSES.contains(&status) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            format!("{kind}:{id}:line:{line_number:03}"),
            format!("unsupported status {status}"),
        ));
    }
}
fn validate_receipt_path(
    path: &str,
    line_number: usize,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    if !path.starts_with("receipts/p02/") || !path.ends_with(".receipt") || path.contains("..") {
        errors.push(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("invalid receipt path {path}"),
        ));
    }
}
fn validate_artifact_path(
    path: &str,
    line_number: usize,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
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
    if path.contains("..") || !allowed.iter().any(|prefix| path.starts_with(prefix)) {
        errors.push(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("invalid artifact path {path}"),
        ));
    }
}
fn require_ids(
    kind: &str,
    required: &[&str],
    actual: BTreeSet<&str>,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    for id in required {
        if !actual.contains(id) {
            errors.push(ValidationError::reject(
                code,
                format!("{kind}:{id}"),
                format!("missing bootstrap packaging {kind} {id}"),
            ));
        }
    }
}
fn parse_pipe_fields(value: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        if let Some((key, item_value)) = segment.split_once(':') {
            fields.insert(key.to_string(), item_value.to_string());
        }
    }
    fields
}
fn require_fields(
    fields: &BTreeMap<String, String>,
    required: &[&str],
    kind: &str,
    line_number: usize,
    errors: &mut Vec<ValidationError>,
) {
    for item in required {
        if !fields.contains_key(*item) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("{kind} missing field {item}"),
            ));
        }
    }
}
fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    fields.get(name).cloned().unwrap_or_default()
}
fn list_field(fields: &BTreeMap<String, String>, name: &str) -> Vec<String> {
    fields
        .get(name)
        .map(|value| {
            value
                .split(',')
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default()
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
        && value.chars().next().unwrap().is_ascii_lowercase()
}
