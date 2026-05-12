use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_packaging::deterministic_packaging_suite_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_packaging_model::{
    DistributionCheck, PackageUnit, PackagingProof, PackagingSurface, ReleaseBundle,
};

pub const P00_PACKAGING_CONTRACT: &str = "LYRA-P00-PACKAGING-SURFACE v1";

pub const REQUIRED_PACKAGING_RULES: &[&str] = &[
    "package_manifest_required",
    "release_bundle_required",
    "offline_distribution_required",
    "command_grouping_required",
    "artifact_bound_packaging_required",
    "receipt_bound_packaging_required",
    "no_network_required_packaging",
    "phase_open_until_packaging_proven",
];

pub const REQUIRED_PACKAGE_UNITS: &[&str] = &[
    "p00_validator_suite",
    "p00_control_surfaces",
    "p00_negative_corpus",
    "p00_receipt_chain",
    "p00_operator_examples",
];

pub const REQUIRED_RELEASE_BUNDLES: &[&str] = &[
    "p00_local_truth_gate_bundle",
    "p00_red_team_bundle",
    "p00_operator_review_bundle",
];

pub const REQUIRED_DISTRIBUTION_CHECKS: &[&str] = &[
    "offline_installable",
    "manifest_hash_stable",
    "artifact_paths_bound",
    "receipt_paths_bound",
    "no_remote_fetch",
    "command_set_complete",
];

pub const REQUIRED_PACKAGING_PROOFS: &[&str] = &[
    "package_manifest_coverage",
    "release_bundle_determinism",
    "offline_distribution_gate",
    "p00_phase_open",
];

const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_PACKAGE_KINDS: &[&str] = &[
    "binary_group",
    "control_plane",
    "corpus",
    "receipt_set",
    "examples",
];
const ALLOWED_PACKAGE_OWNERS: &[&str] = &[
    "src",
    "ops",
    "interfaces",
    "fixtures",
    "goldens",
    "receipts",
    "examples",
];
const ALLOWED_CHECK_SCOPES: &[&str] = &["package", "bundle", "distribution", "phase"];
const ALLOWED_PROOF_SCOPES: &[&str] = &["package", "bundle", "distribution", "phase"];

const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p00-validate",
    "lyra-p00-authority-check",
    "lyra-p00-identity-check",
    "lyra-p00-enforcement-check",
    "lyra-p00-delivery-check",
    "lyra-p00-challenge-check",
    "lyra-p00-control-check",
    "lyra-p00-owner-root-check",
    "lyra-p00-benchmark-evidence-check",
    "lyra-p00-public-interest-check",
    "lyra-p00-canon-compliance-check",
    "lyra-p00-acceptance-check",
    "lyra-p00-formal-semantics-check",
    "lyra-p00-canonical-model-check",
    "lyra-p00-engine-check",
    "lyra-p00-falsification-check",
    "lyra-p00-replay-check",
    "lyra-p00-interface-check",
    "lyra-p00-packaging-check",
];

const FORBIDDEN_PACKAGING_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::PackagingNetworkDependency),
    ("cloud required", ErrorCode::PackagingNetworkDependency),
    ("online required", ErrorCode::PackagingNetworkDependency),
    ("remote fetch", ErrorCode::PackagingNetworkDependency),
    ("package drift accepted", ErrorCode::PackagingDriftAccepted),
    ("release drift accepted", ErrorCode::PackagingDriftAccepted),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_packaging_surface(input: &str) -> Result<PackagingSurface, Vec<ValidationError>> {
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
            "no packaging surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_PACKAGING_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_PACKAGING_CONTRACT}"),
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
                    "packaging rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(package_id) = left.strip_prefix("package:") {
            if !is_symbolic_name(package_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPackageUnit,
                    format!("line:{line_number:03}"),
                    format!("invalid package identity {package_id}"),
                ));
                continue;
            }
            if !seen_packages.insert(package_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicatePackageUnit,
                    format!("package:{package_id}"),
                    "package identity must be unique",
                ));
                continue;
            }
            match parse_package(line_number, package_id, value) {
                Ok(package) => packages.push(package),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(bundle_id) = left.strip_prefix("bundle:") {
            if !is_symbolic_name(bundle_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReleaseBundle,
                    format!("line:{line_number:03}"),
                    format!("invalid bundle identity {bundle_id}"),
                ));
                continue;
            }
            if !seen_bundles.insert(bundle_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReleaseBundle,
                    format!("bundle:{bundle_id}"),
                    "bundle identity must be unique",
                ));
                continue;
            }
            match parse_bundle(line_number, bundle_id, value) {
                Ok(bundle) => bundles.push(bundle),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(check_id) = left.strip_prefix("check:") {
            if !is_symbolic_name(check_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDistributionCheck,
                    format!("line:{line_number:03}"),
                    format!("invalid distribution check identity {check_id}"),
                ));
                continue;
            }
            if !seen_checks.insert(check_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDistributionCheck,
                    format!("check:{check_id}"),
                    "distribution check identity must be unique",
                ));
                continue;
            }
            match parse_check(line_number, check_id, value) {
                Ok(check) => checks.push(check),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPackagingProof,
                    format!("line:{line_number:03}"),
                    format!("invalid packaging proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicatePackagingProof,
                    format!("proof:{proof_id}"),
                    "packaging proof identity must be unique",
                ));
                continue;
            }
            match parse_proof(line_number, proof_id, value) {
                Ok(proof) => proofs.push(proof),
                Err(error) => errors.push(error),
            }
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
                format!("unknown packaging key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(PackagingSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        packages,
        bundles,
        checks,
        proofs,
    })
}

pub fn validate_packaging_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_packaging_surface(input) {
        Ok(surface) => errors.extend(validate_packaging_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }

    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_receipt(input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_packaging_model(surface: &PackagingSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "packaging law must bind to P00",
        ));
    }
    if surface.task != "P00-019" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "packaging law must bind to P00-019",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported packaging status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_packages(surface, &mut errors);
    require_bundles(surface, &mut errors);
    require_checks(surface, &mut errors);
    require_proofs(surface, &mut errors);
    validate_package_units(surface, &mut errors);
    validate_bundle_bindings(surface, &mut errors);
    validate_check_bindings(surface, &mut errors);
    validate_proof_bindings(surface, &mut errors);
    validate_packaging_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_package(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<PackageUnit, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidPackageUnit,
            format!("line:{line_number:03}"),
            "package fields must be key:value segments",
        )
    })?;
    let kind = required_field(&fields, "kind", ErrorCode::InvalidPackageUnit, line_number)?;
    let owner = required_field(&fields, "owner", ErrorCode::InvalidPackageUnit, line_number)?;
    let artifacts = split_csv(&required_field(
        &fields,
        "artifacts",
        ErrorCode::InvalidPackageUnit,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidPackageUnit,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidPackageUnit,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidPackageUnit,
        line_number,
    )?;
    Ok(PackageUnit {
        line_number,
        id: id.to_string(),
        kind,
        owner,
        artifacts,
        commands,
        receipts,
        status,
    })
}

fn parse_bundle(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReleaseBundle, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidReleaseBundle,
            format!("line:{line_number:03}"),
            "bundle fields must be key:value segments",
        )
    })?;
    let order = required_field(
        &fields,
        "order",
        ErrorCode::InvalidReleaseBundle,
        line_number,
    )?;
    let packages = split_csv(&required_field(
        &fields,
        "packages",
        ErrorCode::InvalidReleaseBundle,
        line_number,
    )?);
    let artifacts = split_csv(&required_field(
        &fields,
        "artifacts",
        ErrorCode::InvalidReleaseBundle,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidReleaseBundle,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidReleaseBundle,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidReleaseBundle,
        line_number,
    )?;
    Ok(ReleaseBundle {
        line_number,
        id: id.to_string(),
        order,
        packages,
        artifacts,
        receipts,
        forbids,
        status,
    })
}

fn parse_check(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<DistributionCheck, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidDistributionCheck,
            format!("line:{line_number:03}"),
            "check fields must be key:value segments",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidDistributionCheck,
        line_number,
    )?;
    let target = required_field(
        &fields,
        "target",
        ErrorCode::InvalidDistributionCheck,
        line_number,
    )?;
    let requires = split_csv(&required_field(
        &fields,
        "requires",
        ErrorCode::InvalidDistributionCheck,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidDistributionCheck,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidDistributionCheck,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidDistributionCheck,
        line_number,
    )?;
    Ok(DistributionCheck {
        line_number,
        id: id.to_string(),
        scope,
        target,
        requires,
        forbids,
        receipts,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<PackagingProof, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidPackagingProof,
            format!("line:{line_number:03}"),
            "proof fields must be key:value segments",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidPackagingProof,
        line_number,
    )?;
    let packages = split_csv(&required_field(
        &fields,
        "packages",
        ErrorCode::InvalidPackagingProof,
        line_number,
    )?);
    let bundles = split_csv(&required_field(
        &fields,
        "bundles",
        ErrorCode::InvalidPackagingProof,
        line_number,
    )?);
    let checks = split_csv(&required_field(
        &fields,
        "checks",
        ErrorCode::InvalidPackagingProof,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidPackagingProof,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidPackagingProof,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidPackagingProof,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidPackagingProof,
        line_number,
    )?;
    Ok(PackagingProof {
        line_number,
        id: id.to_string(),
        scope,
        packages,
        bundles,
        checks,
        receipts,
        commands,
        forbids,
        status,
    })
}

fn require_rules(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_PACKAGING_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("blocked_until_proven") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingPackagingRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingPackagingRule,
                format!("rule:{rule}"),
                "required packaging rule missing",
            )),
        }
    }
}

fn require_packages(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_PACKAGE_UNITS {
        if surface.package_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPackageUnit,
                format!("package:{id}"),
                "required package unit missing",
            ));
        }
    }
}

fn require_bundles(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_RELEASE_BUNDLES {
        if surface.bundle_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReleaseBundle,
                format!("bundle:{id}"),
                "required release bundle missing",
            ));
        }
    }
}

fn require_checks(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_DISTRIBUTION_CHECKS {
        if surface.check_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDistributionCheck,
                format!("check:{id}"),
                "required distribution check missing",
            ));
        }
    }
}

fn require_proofs(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_PACKAGING_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPackagingProof,
                format!("proof:{id}"),
                "required packaging proof missing",
            ));
        }
    }
}

fn validate_package_units(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    for package in &surface.packages {
        if !ALLOWED_PACKAGE_KINDS.contains(&package.kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackageUnit,
                package.canonical_identity(),
                format!("invalid package kind {}", package.kind),
            ));
        }
        if !ALLOWED_PACKAGE_OWNERS.contains(&package.owner.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackageUnit,
                package.canonical_identity(),
                format!("invalid package owner {}", package.owner),
            ));
        }
        if !ALLOWED_STATUSES.contains(&package.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackageUnit,
                package.canonical_identity(),
                format!("invalid package status {}", package.status),
            ));
        }
        if package.artifacts.is_empty() || package.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackageUnit,
                package.canonical_identity(),
                "packages must bind artifacts and receipts",
            ));
        }
        for command in &package.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPackageUnit,
                    package.canonical_identity(),
                    format!("unknown package command {command}"),
                ));
            }
        }
    }
}

fn validate_bundle_bindings(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    let mut orders = BTreeSet::new();
    for bundle in &surface.bundles {
        if !orders.insert(bundle.order.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseBundle,
                bundle.canonical_identity(),
                format!("duplicate bundle order {}", bundle.order),
            ));
        }
        if !ALLOWED_STATUSES.contains(&bundle.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseBundle,
                bundle.canonical_identity(),
                format!("invalid bundle status {}", bundle.status),
            ));
        }
        for package in &bundle.packages {
            if surface.package_by_id(package).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReleaseBundle,
                    bundle.canonical_identity(),
                    format!("unknown bundle package {package}"),
                ));
            }
        }
        if bundle.artifacts.is_empty() || bundle.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseBundle,
                bundle.canonical_identity(),
                "bundles must bind artifacts and receipts",
            ));
        }
        if !bundle
            .forbids
            .iter()
            .any(|item| item == "network_required" || item == "remote_fetch")
        {
            errors.push(ValidationError::reject(
                ErrorCode::PackagingNetworkDependency,
                bundle.canonical_identity(),
                "bundle must explicitly forbid network-required distribution",
            ));
        }
    }
}

fn validate_check_bindings(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    for check in &surface.checks {
        if !ALLOWED_CHECK_SCOPES.contains(&check.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                check.canonical_identity(),
                format!("invalid distribution check scope {}", check.scope),
            ));
        }
        let target_known = surface.package_by_id(&check.target).is_some()
            || surface.bundle_by_id(&check.target).is_some()
            || check.target == "P00";
        if !target_known {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                check.canonical_identity(),
                format!("unknown distribution target {}", check.target),
            ));
        }
        if check.receipts.is_empty() || check.requires.is_empty() || check.forbids.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                check.canonical_identity(),
                "distribution checks must bind requirements, forbids, and receipts",
            ));
        }
        if !ALLOWED_STATUSES.contains(&check.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDistributionCheck,
                check.canonical_identity(),
                format!("invalid distribution check status {}", check.status),
            ));
        }
    }
}

fn validate_proof_bindings(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackagingProof,
                proof.canonical_identity(),
                format!("invalid packaging proof scope {}", proof.scope),
            ));
        }
        if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackagingProof,
                proof.canonical_identity(),
                format!("invalid proof status {}", proof.status),
            ));
        }
        for package in &proof.packages {
            if surface.package_by_id(package).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof package {package}"),
                ));
            }
        }
        for bundle in &proof.bundles {
            if surface.bundle_by_id(bundle).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof bundle {bundle}"),
                ));
            }
        }
        for check in &proof.checks {
            if surface.check_by_id(check).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof check {check}"),
                ));
            }
        }
        for command in &proof.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::PackagingProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof command {command}"),
                ));
            }
        }
        if proof.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPackagingProof,
                proof.canonical_identity(),
                "packaging proofs must bind receipts",
            ));
        }
    }
}

fn validate_packaging_report(surface: &PackagingSurface, errors: &mut Vec<ValidationError>) {
    let package_inputs: Vec<(String, String, Vec<String>, Vec<String>, Vec<String>)> = surface
        .packages
        .iter()
        .map(|package| {
            (
                package.id.clone(),
                package.kind.clone(),
                package.artifacts.clone(),
                package.commands.clone(),
                package.receipts.clone(),
            )
        })
        .collect();
    let bundle_inputs: Vec<(String, String, Vec<String>, Vec<String>, Vec<String>)> = surface
        .bundles
        .iter()
        .map(|bundle| {
            (
                bundle.id.clone(),
                bundle.order.clone(),
                bundle.packages.clone(),
                bundle.artifacts.clone(),
                bundle.receipts.clone(),
            )
        })
        .collect();
    let report = deterministic_packaging_suite_report(
        &package_inputs,
        &bundle_inputs,
        surface.checks.len(),
        surface.proofs.len(),
    );
    if report.package_count != surface.packages.len()
        || report.bundle_count != surface.bundles.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::PackagingDriftAccepted,
            "k0_packaging_report",
            "packaging report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::PackagingDriftAccepted,
            "k0_packaging_report",
            "packaging report hash must be stable fnv1a128",
        ));
    }
}

fn parse_field_map(value: &str) -> Option<BTreeMap<String, String>> {
    let mut output = BTreeMap::new();
    for segment in value.split('|') {
        let (key, val) = segment.split_once(':')?;
        if key.is_empty() || val.is_empty() || key != key.trim() || val != val.trim() {
            return None;
        }
        if output.insert(key.to_string(), val.to_string()).is_some() {
            return None;
        }
    }
    Some(output)
}

fn required_field(
    fields: &BTreeMap<String, String>,
    name: &str,
    code: ErrorCode,
    line_number: usize,
) -> Result<String, ValidationError> {
    fields
        .get(name)
        .cloned()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ValidationError::reject(
                code,
                format!("line:{line_number:03}"),
                format!("missing field {name}"),
            )
        })
}

fn split_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_PACKAGING_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden packaging token {needle}"),
            ));
        }
    }
}
