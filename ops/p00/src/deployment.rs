use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_deployment::deterministic_deployment_suite_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_deployment_model::{
    ComplianceHook, DeploymentProof, DeploymentSurface, DeploymentTarget, ReleaseEvidence,
};

pub const P00_DEPLOYMENT_CONTRACT: &str = "LYRA-P00-DEPLOYMENT-HOOKS v1";

pub const REQUIRED_DEPLOYMENT_RULES: &[&str] = &[
    "deployment_manifest_required",
    "offline_deployment_required",
    "compliance_hook_required",
    "release_evidence_required",
    "rollback_readiness_required",
    "receipt_bound_deployment_required",
    "no_remote_service_dependency",
    "phase_open_until_deployment_proven",
];

pub const REQUIRED_DEPLOYMENT_TARGETS: &[&str] = &[
    "local_workstation",
    "offline_archive",
    "airgap_review",
    "sovereign_site",
];

pub const REQUIRED_COMPLIANCE_HOOKS: &[&str] = &[
    "artifact_inventory_check",
    "license_publish_guard",
    "receipt_chain_check",
    "negative_corpus_gate",
    "rollback_readiness_gate",
];

pub const REQUIRED_RELEASE_EVIDENCE: &[&str] = &[
    "offline_install_receipt",
    "artifact_hash_manifest",
    "command_matrix_receipt",
    "operator_review_record",
    "rollback_rehearsal_receipt",
];

pub const REQUIRED_DEPLOYMENT_PROOFS: &[&str] = &[
    "deployment_manifest_coverage",
    "offline_deployment_gate",
    "compliance_hook_binding",
    "release_evidence_replay",
    "p00_phase_open",
];

const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_TARGET_KINDS: &[&str] = &["workstation", "archive", "review", "site"];
const ALLOWED_ENVIRONMENTS: &[&str] = &["offline", "airgap", "sovereign"];
const ALLOWED_HOOK_SCOPES: &[&str] = &["target", "release", "compliance", "rollback", "phase"];
const ALLOWED_EVIDENCE_KINDS: &[&str] = &["receipt", "manifest", "matrix", "record", "rehearsal"];
const ALLOWED_PROOF_SCOPES: &[&str] = &["target", "release", "compliance", "rollback", "phase"];

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
    "lyra-p00-deployment-check",
];

const FORBIDDEN_DEPLOYMENT_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::DeploymentNetworkDependency),
    ("cloud required", ErrorCode::DeploymentNetworkDependency),
    ("online required", ErrorCode::DeploymentNetworkDependency),
    (
        "remote service required",
        ErrorCode::DeploymentNetworkDependency,
    ),
    ("remote fetch", ErrorCode::DeploymentNetworkDependency),
    (
        "deployment drift accepted",
        ErrorCode::DeploymentDriftAccepted,
    ),
    ("release drift accepted", ErrorCode::DeploymentDriftAccepted),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_deployment_surface(input: &str) -> Result<DeploymentSurface, Vec<ValidationError>> {
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
            "no deployment surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_DEPLOYMENT_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_DEPLOYMENT_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut targets = Vec::new();
    let mut hooks = Vec::new();
    let mut evidence = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_targets = BTreeSet::new();
    let mut seen_hooks = BTreeSet::new();
    let mut seen_evidence = BTreeSet::new();
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
                    "deployment rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(target_id) = left.strip_prefix("target:") {
            if !is_symbolic_name(target_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDeploymentTarget,
                    format!("line:{line_number:03}"),
                    format!("invalid deployment target identity {target_id}"),
                ));
                continue;
            }
            if !seen_targets.insert(target_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeploymentTarget,
                    format!("target:{target_id}"),
                    "deployment target identity must be unique",
                ));
                continue;
            }
            match parse_target(line_number, target_id, value) {
                Ok(target) => targets.push(target),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(hook_id) = left.strip_prefix("hook:") {
            if !is_symbolic_name(hook_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidComplianceHook,
                    format!("line:{line_number:03}"),
                    format!("invalid compliance hook identity {hook_id}"),
                ));
                continue;
            }
            if !seen_hooks.insert(hook_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateComplianceHook,
                    format!("hook:{hook_id}"),
                    "compliance hook identity must be unique",
                ));
                continue;
            }
            match parse_hook(line_number, hook_id, value) {
                Ok(hook) => hooks.push(hook),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(evidence_id) = left.strip_prefix("evidence:") {
            if !is_symbolic_name(evidence_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReleaseEvidence,
                    format!("line:{line_number:03}"),
                    format!("invalid release evidence identity {evidence_id}"),
                ));
                continue;
            }
            if !seen_evidence.insert(evidence_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReleaseEvidence,
                    format!("evidence:{evidence_id}"),
                    "release evidence identity must be unique",
                ));
                continue;
            }
            match parse_evidence(line_number, evidence_id, value) {
                Ok(item) => evidence.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDeploymentProof,
                    format!("line:{line_number:03}"),
                    format!("invalid deployment proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeploymentProof,
                    format!("proof:{proof_id}"),
                    "deployment proof identity must be unique",
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
                format!("unknown deployment key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(DeploymentSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        targets,
        hooks,
        evidence,
        proofs,
    })
}

pub fn validate_deployment_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_deployment_surface(input) {
        Ok(surface) => errors.extend(validate_deployment_model(&surface).errors),
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

pub fn validate_deployment_model(surface: &DeploymentSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "deployment law must bind to P00",
        ));
    }
    if surface.task != "P00-020" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "deployment law must bind to P00-020",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported deployment status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_targets(surface, &mut errors);
    require_hooks(surface, &mut errors);
    require_evidence(surface, &mut errors);
    require_proofs(surface, &mut errors);
    validate_targets(surface, &mut errors);
    validate_hooks(surface, &mut errors);
    validate_evidence(surface, &mut errors);
    validate_proofs(surface, &mut errors);
    validate_deployment_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_target(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<DeploymentTarget, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidDeploymentTarget,
            format!("line:{line_number:03}"),
            "target fields must be key:value segments",
        )
    })?;
    let kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidDeploymentTarget,
        line_number,
    )?;
    let environment = required_field(
        &fields,
        "environment",
        ErrorCode::InvalidDeploymentTarget,
        line_number,
    )?;
    let artifacts = split_csv(&required_field(
        &fields,
        "artifacts",
        ErrorCode::InvalidDeploymentTarget,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidDeploymentTarget,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidDeploymentTarget,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidDeploymentTarget,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidDeploymentTarget,
        line_number,
    )?;
    Ok(DeploymentTarget {
        line_number,
        id: id.to_string(),
        kind,
        environment,
        artifacts,
        commands,
        receipts,
        forbids,
        status,
    })
}

fn parse_hook(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ComplianceHook, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidComplianceHook,
            format!("line:{line_number:03}"),
            "hook fields must be key:value segments",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidComplianceHook,
        line_number,
    )?;
    let target = required_field(
        &fields,
        "target",
        ErrorCode::InvalidComplianceHook,
        line_number,
    )?;
    let requires = split_csv(&required_field(
        &fields,
        "requires",
        ErrorCode::InvalidComplianceHook,
        line_number,
    )?);
    let evidence = split_csv(&required_field(
        &fields,
        "evidence",
        ErrorCode::InvalidComplianceHook,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidComplianceHook,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidComplianceHook,
        line_number,
    )?;
    Ok(ComplianceHook {
        line_number,
        id: id.to_string(),
        scope,
        target,
        requires,
        evidence,
        receipts,
        status,
    })
}

fn parse_evidence(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReleaseEvidence, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidReleaseEvidence,
            format!("line:{line_number:03}"),
            "evidence fields must be key:value segments",
        )
    })?;
    let kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidReleaseEvidence,
        line_number,
    )?;
    let path = required_field(
        &fields,
        "path",
        ErrorCode::InvalidReleaseEvidence,
        line_number,
    )?;
    let targets = split_csv(&required_field(
        &fields,
        "targets",
        ErrorCode::InvalidReleaseEvidence,
        line_number,
    )?);
    let hooks = split_csv(&required_field(
        &fields,
        "hooks",
        ErrorCode::InvalidReleaseEvidence,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidReleaseEvidence,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidReleaseEvidence,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidReleaseEvidence,
        line_number,
    )?;
    Ok(ReleaseEvidence {
        line_number,
        id: id.to_string(),
        kind,
        path,
        targets,
        hooks,
        receipts,
        commands,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<DeploymentProof, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidDeploymentProof,
            format!("line:{line_number:03}"),
            "proof fields must be key:value segments",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidDeploymentProof,
        line_number,
    )?;
    let targets = split_csv(&required_field(
        &fields,
        "targets",
        ErrorCode::InvalidDeploymentProof,
        line_number,
    )?);
    let hooks = split_csv(&required_field(
        &fields,
        "hooks",
        ErrorCode::InvalidDeploymentProof,
        line_number,
    )?);
    let evidence = split_csv(&required_field(
        &fields,
        "evidence",
        ErrorCode::InvalidDeploymentProof,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidDeploymentProof,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidDeploymentProof,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidDeploymentProof,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidDeploymentProof,
        line_number,
    )?;
    Ok(DeploymentProof {
        line_number,
        id: id.to_string(),
        scope,
        targets,
        hooks,
        evidence,
        receipts,
        commands,
        forbids,
        status,
    })
}

fn require_rules(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_DEPLOYMENT_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("blocked_until_proven") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentRule,
                format!("rule:{rule}"),
                "required deployment rule missing",
            )),
        }
    }
}

fn require_targets(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_DEPLOYMENT_TARGETS {
        if surface.target_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target:{id}"),
                "required deployment target missing",
            ));
        }
    }
}

fn require_hooks(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_COMPLIANCE_HOOKS {
        if surface.hook_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingComplianceHook,
                format!("hook:{id}"),
                "required compliance hook missing",
            ));
        }
    }
}

fn require_evidence(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_RELEASE_EVIDENCE {
        if surface.evidence_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReleaseEvidence,
                format!("evidence:{id}"),
                "required release evidence missing",
            ));
        }
    }
}

fn require_proofs(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_DEPLOYMENT_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentProof,
                format!("proof:{id}"),
                "required deployment proof missing",
            ));
        }
    }
}

fn validate_targets(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for target in &surface.targets {
        if !ALLOWED_TARGET_KINDS.contains(&target.kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                format!("invalid target kind {}", target.kind),
            ));
        }
        if !ALLOWED_ENVIRONMENTS.contains(&target.environment.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                format!("invalid deployment environment {}", target.environment),
            ));
        }
        if !ALLOWED_STATUSES.contains(&target.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                format!("invalid target status {}", target.status),
            ));
        }
        if target.artifacts.is_empty() || target.commands.is_empty() || target.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                "targets must bind artifacts, commands, and receipts",
            ));
        }
        if !target
            .forbids
            .iter()
            .any(|item| item == "network_required" || item == "remote_service")
        {
            errors.push(ValidationError::reject(
                ErrorCode::DeploymentNetworkDependency,
                target.canonical_identity(),
                "deployment targets must explicitly forbid remote service dependency",
            ));
        }
        for command in &target.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDeploymentTarget,
                    target.canonical_identity(),
                    format!("unknown deployment command {command}"),
                ));
            }
        }
    }
}

fn validate_hooks(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for hook in &surface.hooks {
        if !ALLOWED_HOOK_SCOPES.contains(&hook.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidComplianceHook,
                hook.canonical_identity(),
                format!("invalid hook scope {}", hook.scope),
            ));
        }
        let target_known = surface.target_by_id(&hook.target).is_some() || hook.target == "P00";
        if !target_known {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidComplianceHook,
                hook.canonical_identity(),
                format!("unknown hook target {}", hook.target),
            ));
        }
        if hook.requires.is_empty() || hook.evidence.is_empty() || hook.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidComplianceHook,
                hook.canonical_identity(),
                "hooks must bind requirements, evidence, and receipts",
            ));
        }
        if !ALLOWED_STATUSES.contains(&hook.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidComplianceHook,
                hook.canonical_identity(),
                format!("invalid hook status {}", hook.status),
            ));
        }
    }
}

fn validate_evidence(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for evidence in &surface.evidence {
        if !ALLOWED_EVIDENCE_KINDS.contains(&evidence.kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseEvidence,
                evidence.canonical_identity(),
                format!("invalid evidence kind {}", evidence.kind),
            ));
        }
        if !evidence.path.starts_with("receipts/")
            && !evidence.path.starts_with("goldens/")
            && !evidence.path.starts_with("products/")
            && !evidence.path.starts_with("examples/")
            && !evidence.path.starts_with("ops/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseEvidence,
                evidence.canonical_identity(),
                format!("invalid evidence path {}", evidence.path),
            ));
        }
        for target in &evidence.targets {
            if surface.target_by_id(target).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReleaseEvidence,
                    evidence.canonical_identity(),
                    format!("unknown evidence target {target}"),
                ));
            }
        }
        for hook in &evidence.hooks {
            if surface.hook_by_id(hook).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReleaseEvidence,
                    evidence.canonical_identity(),
                    format!("unknown evidence hook {hook}"),
                ));
            }
        }
        for command in &evidence.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReleaseEvidence,
                    evidence.canonical_identity(),
                    format!("unknown evidence command {command}"),
                ));
            }
        }
        if evidence.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseEvidence,
                evidence.canonical_identity(),
                "release evidence must bind receipts",
            ));
        }
        if !ALLOWED_STATUSES.contains(&evidence.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseEvidence,
                evidence.canonical_identity(),
                format!("invalid evidence status {}", evidence.status),
            ));
        }
    }
}

fn validate_proofs(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentProof,
                proof.canonical_identity(),
                format!("invalid proof scope {}", proof.scope),
            ));
        }
        if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentProof,
                proof.canonical_identity(),
                format!("invalid proof status {}", proof.status),
            ));
        }
        for target in &proof.targets {
            if surface.target_by_id(target).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof target {target}"),
                ));
            }
        }
        for hook in &proof.hooks {
            if surface.hook_by_id(hook).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof hook {hook}"),
                ));
            }
        }
        for evidence in &proof.evidence {
            if surface.evidence_by_id(evidence).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof evidence {evidence}"),
                ));
            }
        }
        for command in &proof.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof command {command}"),
                ));
            }
        }
        if proof.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentProof,
                proof.canonical_identity(),
                "deployment proofs must bind receipts",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|item| item == "phase_closure" || item == "global_complete")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "deployment proof must keep P00 phase open until closure gate",
            ));
        }
    }
}

fn validate_deployment_report(surface: &DeploymentSurface, errors: &mut Vec<ValidationError>) {
    let target_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )> = surface
        .targets
        .iter()
        .map(|target| {
            (
                target.id.clone(),
                target.kind.clone(),
                target.environment.clone(),
                target.artifacts.clone(),
                target.commands.clone(),
                target.receipts.clone(),
            )
        })
        .collect();
    let hook_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )> = surface
        .hooks
        .iter()
        .map(|hook| {
            (
                hook.id.clone(),
                hook.scope.clone(),
                hook.target.clone(),
                hook.requires.clone(),
                hook.evidence.clone(),
                hook.receipts.clone(),
            )
        })
        .collect();
    let report = deterministic_deployment_suite_report(
        &target_inputs,
        &hook_inputs,
        surface.evidence.len(),
        surface.proofs.len(),
    );
    if report.target_count != surface.targets.len() || report.hook_count != surface.hooks.len() {
        errors.push(ValidationError::reject(
            ErrorCode::DeploymentDriftAccepted,
            "k0_deployment_report",
            "deployment report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::DeploymentDriftAccepted,
            "k0_deployment_report",
            "deployment report hash must be stable fnv1a128",
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
    for (needle, code) in FORBIDDEN_DEPLOYMENT_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden deployment token {needle}"),
            ));
        }
    }
}
