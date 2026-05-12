use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_packaging::deterministic_semantic_packaging_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_packaging::{
    semantic_packaging_artifacts_bind_paths, semantic_packaging_bundle_descriptor,
    semantic_packaging_bundle_digest, semantic_packaging_bundles_bind_registry,
    semantic_packaging_check_descriptor, semantic_packaging_check_digest,
    semantic_packaging_checks_bind_registry, semantic_packaging_no_forbidden_descriptor_claims,
    semantic_packaging_package_descriptor, semantic_packaging_package_digest,
    semantic_packaging_proof_descriptor, semantic_packaging_proof_digest,
    semantic_packaging_proofs_bind_registry, semantic_packaging_registry_hash,
    semantic_packaging_units_cover_p01_001_through_p01_019,
};
use crate::p01_semantic_packaging_model::{
    SemanticDistributionCheck, SemanticPackageUnit, SemanticPackagingProof,
    SemanticPackagingSurface, SemanticReleaseBundle,
};

pub const P01_SEMANTIC_PACKAGING_CONTRACT: &str = "LYRA-P01-SEMANTIC-PACKAGING v1";

pub const REQUIRED_SEMANTIC_PACKAGING_RULES: &[&str] = &[
    "semantic_package_manifest_required",
    "semantic_release_bundle_required",
    "offline_distribution_required",
    "product_surface_required",
    "command_grouping_required",
    "artifact_bound_packaging_required",
    "receipt_bound_packaging_required",
    "replay_witness_packaging_required",
    "no_network_required_packaging",
    "no_unreceipted_package",
    "no_package_drift_acceptance",
];

pub const REQUIRED_SEMANTIC_PACKAGE_UNITS: &[&str] = &[
    "semantic_core_validation_binaries",
    "semantic_contract_model_set",
    "semantic_control_law_set",
    "semantic_positive_negative_corpus",
    "semantic_packaging_goldens",
    "semantic_receipt_chain",
    "semantic_product_surfaces",
];

pub const REQUIRED_SEMANTIC_RELEASE_BUNDLES: &[&str] = &[
    "p01_local_semantic_package_bundle",
    "p01_red_team_semantic_package_bundle",
    "p01_operator_product_bundle",
    "p01_offline_distribution_bundle",
];

pub const REQUIRED_SEMANTIC_DISTRIBUTION_CHECKS: &[&str] = &[
    "offline_installable",
    "manifest_hash_stable",
    "artifact_paths_bound",
    "receipt_paths_bound",
    "no_remote_fetch",
    "command_set_complete",
    "product_surface_bound",
];

pub const REQUIRED_SEMANTIC_PACKAGING_PROOFS: &[&str] = &[
    "package_manifest_coverage",
    "release_bundle_determinism",
    "offline_distribution_gate",
    "product_surface_binding",
    "p01_phase_open",
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
];
const ALLOWED_CHECK_SCOPES: &[&str] = &["package", "bundle", "distribution", "product", "phase"];
const ALLOWED_PROOF_SCOPES: &[&str] = &["package", "bundle", "distribution", "product", "phase"];

const FORBIDDEN_PACKAGING_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::PackagingNetworkDependency),
    ("cloud required", ErrorCode::PackagingNetworkDependency),
    ("online required", ErrorCode::PackagingNetworkDependency),
    ("remote fetch", ErrorCode::PackagingNetworkDependency),
    ("package drift accepted", ErrorCode::PackagingDriftAccepted),
    ("release drift accepted", ErrorCode::PackagingDriftAccepted),
    (
        "unreceipted package action",
        ErrorCode::PackagingDriftAccepted,
    ),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_packaging_surface(
    input: &str,
) -> Result<SemanticPackagingSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => {
            return Err(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "byte-stream",
                format!("{error:?}"),
            )])
        }
    };
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "line:000",
            "no semantic packaging lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_PACKAGING_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_PACKAGING_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
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

    for (offset, line) in lines.iter().enumerate().skip(1) {
        let line_number = offset + 1;
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
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "semantic packaging rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if left == "package" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id",
                    "kind",
                    "owner",
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
                    format!("duplicate or invalid semantic package {id}"),
                ));
            }
            packages.push(SemanticPackageUnit {
                line_number,
                id,
                kind: field(&fields, "kind"),
                owner_root: field(&fields, "owner"),
                artifacts: list_field(&fields, "artifacts"),
                commands: list_field(&fields, "commands"),
                receipts: list_field(&fields, "receipts"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "bundle" {
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
            if !is_symbolic_name(&id) || !seen_bundles.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReleaseBundle,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic bundle {id}"),
                ));
            }
            bundles.push(SemanticReleaseBundle {
                line_number,
                id,
                order: field(&fields, "order"),
                packages: list_field(&fields, "packages"),
                artifacts: list_field(&fields, "artifacts"),
                receipts: list_field(&fields, "receipts"),
                checks: list_field(&fields, "checks"),
                forbids: list_field(&fields, "forbids"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "check" {
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
                    format!("duplicate or invalid semantic distribution check {id}"),
                ));
            }
            checks.push(SemanticDistributionCheck {
                line_number,
                id,
                scope: field(&fields, "scope"),
                target: field(&fields, "target"),
                requires: list_field(&fields, "requires"),
                forbids: list_field(&fields, "forbids"),
                receipts: list_field(&fields, "receipts"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "proof" {
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
                    format!("duplicate or invalid semantic packaging proof {id}"),
                ));
            }
            proofs.push(SemanticPackagingProof {
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
            continue;
        }
        if !seen_scalars.insert(left.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate scalar {left}"),
            ));
            continue;
        }
        match left {
            "phase" => phase = Some(value.to_string()),
            "task" => task = Some(value.to_string()),
            "status" => status = Some(value.to_string()),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown semantic packaging key {left}"),
            )),
        }
    }
    if phase.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingPhase,
            "surface",
            "missing phase",
        ));
    }
    if task.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingTask,
            "surface",
            "missing task",
        ));
    }
    if status.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            "surface",
            "missing status",
        ));
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticPackagingSurface {
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

pub fn validate_semantic_packaging_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_semantic_packaging_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_semantic_packaging(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_packaging(
    surface: &SemanticPackagingSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-019" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-019 got {}", surface.task),
        ));
    }
    validate_status("surface", "P01-019", 0, &surface.status, errors);

    for rule in REQUIRED_SEMANTIC_PACKAGING_RULES {
        match surface.rules.get(*rule) {
            Some(value) if value == "required" || value == "forbidden" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingPackagingRule,
                format!("rule:{rule}"),
                format!("expected required/forbidden got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingPackagingRule,
                format!("rule:{rule}"),
                "missing semantic packaging rule",
            )),
        }
    }

    require_ids(
        "package",
        REQUIRED_SEMANTIC_PACKAGE_UNITS,
        surface
            .packages
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingPackageUnit,
        errors,
    );
    require_ids(
        "bundle",
        REQUIRED_SEMANTIC_RELEASE_BUNDLES,
        surface
            .bundles
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingReleaseBundle,
        errors,
    );
    require_ids(
        "check",
        REQUIRED_SEMANTIC_DISTRIBUTION_CHECKS,
        surface.checks.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingDistributionCheck,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_SEMANTIC_PACKAGING_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingPackagingProof,
        errors,
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
    let mut bundle_orders = BTreeSet::new();

    for package in &surface.packages {
        validate_status(
            "package",
            &package.id,
            package.line_number,
            &package.status,
            errors,
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
                    "package {} invalid owner {}",
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
            if artifact.contains("..") || !artifact.starts_with(&package.owner_root) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPackageUnit,
                    format!("line:{:03}", package.line_number),
                    format!(
                        "package {} artifact path/root mismatch {artifact}",
                        package.id
                    ),
                ));
            }
        }
        for command in &package.commands {
            if !command.starts_with("lyra-p01-") || command.contains(' ') {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPackageUnit,
                    format!("line:{:03}", package.line_number),
                    format!("package {} invalid command {command}", package.id),
                ));
            }
        }
        for receipt in &package.receipts {
            validate_receipt_path(
                receipt,
                package.line_number,
                ErrorCode::InvalidPackageUnit,
                errors,
            );
        }
        if let Some(descriptor) = semantic_packaging_package_descriptor(&package.id) {
            let digest = semantic_packaging_package_digest(&package.id).unwrap_or_default();
            if descriptor.kind != package.kind
                || descriptor.owner_root != package.owner_root
                || descriptor.artifacts != package.artifacts.as_slice()
                || descriptor.commands != package.commands.as_slice()
                || descriptor.receipts != package.receipts.as_slice()
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingDriftAccepted,
                    format!("line:{:03}", package.line_number),
                    format!("package {} descriptor drift", package.id),
                ));
            }
        }
    }

    for bundle in &surface.bundles {
        validate_status(
            "bundle",
            &bundle.id,
            bundle.line_number,
            &bundle.status,
            errors,
        );
        if bundle.order.len() != 3
            || !bundle.order.chars().all(|ch| ch.is_ascii_digit())
            || !bundle_orders.insert(bundle.order.clone())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseBundle,
                format!("line:{:03}", bundle.line_number),
                format!("bundle {} order is invalid", bundle.id),
            ));
        }
        if bundle.artifacts.is_empty()
            || bundle.receipts.is_empty()
            || bundle.checks.is_empty()
            || bundle.forbids.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseBundle,
                format!("line:{:03}", bundle.line_number),
                format!(
                    "bundle {} requires artifacts, receipts, checks, and forbidden claims",
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
                errors,
            );
        }
        for receipt in &bundle.receipts {
            validate_receipt_path(
                receipt,
                bundle.line_number,
                ErrorCode::InvalidReleaseBundle,
                errors,
            );
        }
        if let Some(descriptor) = semantic_packaging_bundle_descriptor(&bundle.id) {
            let digest = semantic_packaging_bundle_digest(&bundle.id).unwrap_or_default();
            if descriptor.order != bundle.order
                || descriptor.packages != bundle.packages.as_slice()
                || descriptor.checks != bundle.checks.as_slice()
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingDriftAccepted,
                    format!("line:{:03}", bundle.line_number),
                    format!("bundle {} descriptor drift", bundle.id),
                ));
            }
        }
    }

    for check in &surface.checks {
        validate_status("check", &check.id, check.line_number, &check.status, errors);
        if !ALLOWED_CHECK_SCOPES.contains(&check.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                format!("line:{:03}", check.line_number),
                format!("check {} invalid scope {}", check.id, check.scope),
            ));
        }
        let target_known = package_ids.contains(check.target.as_str())
            || bundle_ids.contains(check.target.as_str())
            || check.target == "p01_semantic_distribution";
        if !target_known {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                format!("line:{:03}", check.line_number),
                format!("check {} unknown target {}", check.id, check.target),
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
        for receipt in &check.receipts {
            validate_receipt_path(
                receipt,
                check.line_number,
                ErrorCode::InvalidDistributionCheck,
                errors,
            );
        }
        if semantic_packaging_check_descriptor(&check.id).is_some()
            && semantic_packaging_check_digest(&check.id)
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
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
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
                errors,
            );
        }
        for command in &proof.commands {
            if !command.starts_with("lyra-p01-") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPackagingProof,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} invalid command {command}", proof.id),
                ));
            }
        }
        if semantic_packaging_proof_descriptor(&proof.id).is_some()
            && semantic_packaging_proof_digest(&proof.id)
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

    if !semantic_packaging_bundles_bind_registry()
        || !semantic_packaging_checks_bind_registry()
        || !semantic_packaging_proofs_bind_registry()
        || !semantic_packaging_artifacts_bind_paths()
        || !semantic_packaging_units_cover_p01_001_through_p01_019()
        || !semantic_packaging_no_forbidden_descriptor_claims()
        || semantic_packaging_registry_hash().is_empty()
    {
        errors.push(ValidationError::reject(
            ErrorCode::PackagingDriftAccepted,
            "lyralang",
            "semantic packaging registry failed binding checks",
        ));
    }

    let report = deterministic_semantic_packaging_suite_report(
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
            "semantic packaging report drift",
        ));
    }
}

fn scan_forbidden_text(input: &str, errors: &mut Vec<ValidationError>) {
    let lowered = input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_PACKAGING_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden-text",
                format!("forbidden semantic packaging text {needle}"),
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
            ErrorCode::UnsupportedClosureStatus,
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
    if !path.starts_with("receipts/p01/") || !path.ends_with(".receipt") || path.contains("..") {
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
                format!("missing semantic packaging {kind} {id}"),
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
