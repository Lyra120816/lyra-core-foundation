use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_deployment::deterministic_semantic_deployment_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_deployment::{
    semantic_deployment_artifacts_bind_paths, semantic_deployment_evidence_bind_registry,
    semantic_deployment_evidence_descriptor, semantic_deployment_evidence_digest,
    semantic_deployment_hook_descriptor, semantic_deployment_hook_digest,
    semantic_deployment_hooks_bind_targets, semantic_deployment_no_forbidden_descriptor_claims,
    semantic_deployment_proof_descriptor, semantic_deployment_proof_digest,
    semantic_deployment_proofs_bind_registry,
    semantic_deployment_receipts_cover_p01_001_through_p01_020, semantic_deployment_registry_hash,
    semantic_deployment_target_descriptor, semantic_deployment_target_digest,
};
use crate::p01_semantic_deployment_model::{
    SemanticComplianceHook, SemanticDeploymentProof, SemanticDeploymentSurface,
    SemanticDeploymentTarget, SemanticReleaseEvidence,
};

pub const P01_SEMANTIC_DEPLOYMENT_CONTRACT: &str = "LYRA-P01-SEMANTIC-DEPLOYMENT-HOOKS v1";

pub const REQUIRED_SEMANTIC_DEPLOYMENT_RULES: &[&str] = &[
    "semantic_deployment_manifest_required",
    "offline_deployment_required",
    "enterprise_hook_required",
    "compliance_hook_required",
    "release_evidence_required",
    "rollback_readiness_required",
    "receipt_bound_deployment_required",
    "semantic_packaging_bridge_required",
    "no_remote_service_dependency",
    "no_unreceipted_deployment",
    "no_deployment_drift_acceptance",
    "phase_open_until_deployment_proven",
];

pub const REQUIRED_SEMANTIC_DEPLOYMENT_TARGETS: &[&str] = &[
    "semantic_local_workstation_deployment",
    "semantic_airgap_archive_deployment",
    "semantic_sovereign_site_review",
    "semantic_enterprise_operator_review",
];

pub const REQUIRED_SEMANTIC_COMPLIANCE_HOOKS: &[&str] = &[
    "semantic_artifact_inventory_check",
    "semantic_receipt_chain_gate",
    "semantic_negative_corpus_gate",
    "semantic_offline_install_gate",
    "semantic_rollout_replay_gate",
    "semantic_phase_open_gate",
];

pub const REQUIRED_SEMANTIC_RELEASE_EVIDENCE: &[&str] = &[
    "semantic_deployment_manifest",
    "artifact_hash_manifest",
    "command_matrix_receipt",
    "operator_review_record",
    "rollback_rehearsal_receipt",
    "offline_install_receipt",
];

pub const REQUIRED_SEMANTIC_DEPLOYMENT_PROOFS: &[&str] = &[
    "semantic_target_coverage",
    "semantic_offline_deployment_gate",
    "semantic_compliance_hook_binding",
    "semantic_release_evidence_replay",
    "semantic_packaging_bridge",
    "p01_phase_open",
];

const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_TARGET_KINDS: &[&str] = &["workstation", "archive", "review", "site"];
const ALLOWED_ENVIRONMENTS: &[&str] = &["offline", "airgap", "sovereign"];
const ALLOWED_HOOK_SCOPES: &[&str] = &[
    "target",
    "release",
    "compliance",
    "rollback",
    "phase",
    "enterprise",
];
const ALLOWED_EVIDENCE_KINDS: &[&str] = &["receipt", "manifest", "matrix", "record", "rehearsal"];
const ALLOWED_PROOF_SCOPES: &[&str] = &[
    "target",
    "release",
    "compliance",
    "rollback",
    "phase",
    "enterprise",
];

const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p01-atom-check",
    "lyra-p01-ir-check",
    "lyra-p01-object-check",
    "lyra-p01-identity-check",
    "lyra-p01-reference-semantics-check",
    "lyra-p01-symbolic-equality-check",
    "lyra-p01-error-challenge-evidence-check",
    "lyra-p01-semantic-serialization-hashing-check",
    "lyra-p01-semantic-adversarial-corpus-check",
    "lyra-p01-core-ir-reuse-check",
    "lyra-p01-semantic-atom-reference-check",
    "lyra-p01-semantic-bedrock-receipts-check",
    "lyra-p01-formal-semantic-constitution-check",
    "lyra-p01-canonical-data-model-check",
    "lyra-p01-semantic-core-engine-check",
    "lyra-p01-semantic-falsification-check",
    "lyra-p01-semantic-replay-check",
    "lyra-p01-semantic-interface-check",
    "lyra-p01-semantic-packaging-check",
    "lyra-p01-semantic-deployment-check",
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
    (
        "unreceipted deployment action",
        ErrorCode::DeploymentDriftAccepted,
    ),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_deployment_surface(
    input: &str,
) -> Result<SemanticDeploymentSurface, Vec<ValidationError>> {
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
            "no semantic deployment surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_DEPLOYMENT_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_DEPLOYMENT_CONTRACT}"),
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
                    "semantic deployment rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(target_id) = left.strip_prefix("target:") {
            if !is_symbolic_name(target_id) || !seen_targets.insert(target_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeploymentTarget,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic target {target_id}"),
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
            if !is_symbolic_name(hook_id) || !seen_hooks.insert(hook_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateComplianceHook,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic hook {hook_id}"),
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
            if !is_symbolic_name(evidence_id) || !seen_evidence.insert(evidence_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReleaseEvidence,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic evidence {evidence_id}"),
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
            if !is_symbolic_name(proof_id) || !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeploymentProof,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic proof {proof_id}"),
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
                format!("unknown semantic deployment key {left}"),
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
    Ok(SemanticDeploymentSurface {
        header,
        phase: phase.unwrap(),
        task: task.unwrap(),
        status: status.unwrap(),
        rules,
        targets,
        hooks,
        evidence,
        proofs,
    })
}

pub fn validate_semantic_deployment_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_semantic_deployment_surface(input) {
        Ok(surface) => validate_semantic_deployment_model(&surface, &mut errors),
        Err(mut parse_errors) => errors.append(&mut parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_deployment_model(
    surface: &SemanticDeploymentSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-020" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-020 got {}", surface.task),
        ));
    }
    validate_status("surface", "P01-020", 0, &surface.status, errors);

    for rule in REQUIRED_SEMANTIC_DEPLOYMENT_RULES {
        match surface.rules.get(*rule) {
            Some(value)
                if value == "required"
                    || value == "forbidden"
                    || value == "blocked_until_proven" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentRule,
                format!("rule:{rule}"),
                format!("unsupported deployment rule value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentRule,
                format!("rule:{rule}"),
                "missing semantic deployment rule",
            )),
        }
    }

    require_ids(
        "target",
        REQUIRED_SEMANTIC_DEPLOYMENT_TARGETS,
        surface
            .targets
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingDeploymentTarget,
        errors,
    );
    require_ids(
        "hook",
        REQUIRED_SEMANTIC_COMPLIANCE_HOOKS,
        surface.hooks.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingComplianceHook,
        errors,
    );
    require_ids(
        "evidence",
        REQUIRED_SEMANTIC_RELEASE_EVIDENCE,
        surface
            .evidence
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingReleaseEvidence,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_SEMANTIC_DEPLOYMENT_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingDeploymentProof,
        errors,
    );

    validate_targets(surface, errors);
    validate_hooks(surface, errors);
    validate_evidence(surface, errors);
    validate_proofs(surface, errors);

    if !semantic_deployment_artifacts_bind_paths()
        || !semantic_deployment_hooks_bind_targets()
        || !semantic_deployment_evidence_bind_registry()
        || !semantic_deployment_proofs_bind_registry()
        || !semantic_deployment_receipts_cover_p01_001_through_p01_020()
        || !semantic_deployment_no_forbidden_descriptor_claims()
        || semantic_deployment_registry_hash().is_empty()
    {
        errors.push(ValidationError::reject(
            ErrorCode::DeploymentDriftAccepted,
            "lyralang",
            "semantic deployment registry failed binding checks",
        ));
    }

    let report = deterministic_semantic_deployment_suite_report(
        &surface
            .targets
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.kind.clone(),
                    item.environment.clone(),
                    item.artifacts.clone(),
                    item.commands.clone(),
                    item.receipts.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .hooks
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.scope.clone(),
                    item.target.clone(),
                    item.requires.clone(),
                    item.evidence.clone(),
                    item.receipts.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .evidence
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.kind.clone(),
                    item.path.clone(),
                    item.targets.clone(),
                    item.hooks.clone(),
                    item.receipts.clone(),
                    item.commands.clone(),
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
                    item.targets.clone(),
                    item.hooks.clone(),
                    item.evidence.clone(),
                    item.receipts.clone(),
                    item.commands.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
    );
    if report.target_count != surface.targets.len()
        || report.hook_count != surface.hooks.len()
        || report.evidence_count != surface.evidence.len()
        || report.proof_count != surface.proofs.len()
        || !report.suite_hash.starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::DeploymentDriftAccepted,
            "deterministic-report",
            "semantic deployment report drift",
        ));
    }
}

fn parse_target(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticDeploymentTarget, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidDeploymentTarget,
            format!("line:{line_number:03}"),
            "target fields must be key:value segments",
        )
    })?;
    Ok(SemanticDeploymentTarget {
        line_number,
        id: id.to_string(),
        kind: required_field(
            &fields,
            "kind",
            ErrorCode::InvalidDeploymentTarget,
            line_number,
        )?,
        environment: required_field(
            &fields,
            "environment",
            ErrorCode::InvalidDeploymentTarget,
            line_number,
        )?,
        artifacts: split_csv(&required_field(
            &fields,
            "artifacts",
            ErrorCode::InvalidDeploymentTarget,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidDeploymentTarget,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidDeploymentTarget,
            line_number,
        )?),
        forbids: split_csv(&required_field(
            &fields,
            "forbids",
            ErrorCode::InvalidDeploymentTarget,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidDeploymentTarget,
            line_number,
        )?,
    })
}

fn parse_hook(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticComplianceHook, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidComplianceHook,
            format!("line:{line_number:03}"),
            "hook fields must be key:value segments",
        )
    })?;
    Ok(SemanticComplianceHook {
        line_number,
        id: id.to_string(),
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::InvalidComplianceHook,
            line_number,
        )?,
        target: required_field(
            &fields,
            "target",
            ErrorCode::InvalidComplianceHook,
            line_number,
        )?,
        requires: split_csv(&required_field(
            &fields,
            "requires",
            ErrorCode::InvalidComplianceHook,
            line_number,
        )?),
        evidence: split_csv(&required_field(
            &fields,
            "evidence",
            ErrorCode::InvalidComplianceHook,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidComplianceHook,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidComplianceHook,
            line_number,
        )?,
    })
}

fn parse_evidence(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticReleaseEvidence, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidReleaseEvidence,
            format!("line:{line_number:03}"),
            "evidence fields must be key:value segments",
        )
    })?;
    Ok(SemanticReleaseEvidence {
        line_number,
        id: id.to_string(),
        kind: required_field(
            &fields,
            "kind",
            ErrorCode::InvalidReleaseEvidence,
            line_number,
        )?,
        path: required_field(
            &fields,
            "path",
            ErrorCode::InvalidReleaseEvidence,
            line_number,
        )?,
        targets: split_csv(&required_field(
            &fields,
            "targets",
            ErrorCode::InvalidReleaseEvidence,
            line_number,
        )?),
        hooks: split_csv(&required_field(
            &fields,
            "hooks",
            ErrorCode::InvalidReleaseEvidence,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidReleaseEvidence,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidReleaseEvidence,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidReleaseEvidence,
            line_number,
        )?,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticDeploymentProof, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidDeploymentProof,
            format!("line:{line_number:03}"),
            "proof fields must be key:value segments",
        )
    })?;
    Ok(SemanticDeploymentProof {
        line_number,
        id: id.to_string(),
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::InvalidDeploymentProof,
            line_number,
        )?,
        targets: split_csv(&required_field(
            &fields,
            "targets",
            ErrorCode::InvalidDeploymentProof,
            line_number,
        )?),
        hooks: split_csv(&required_field(
            &fields,
            "hooks",
            ErrorCode::InvalidDeploymentProof,
            line_number,
        )?),
        evidence: split_csv(&required_field(
            &fields,
            "evidence",
            ErrorCode::InvalidDeploymentProof,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidDeploymentProof,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidDeploymentProof,
            line_number,
        )?),
        forbids: split_csv(&required_field(
            &fields,
            "forbids",
            ErrorCode::InvalidDeploymentProof,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidDeploymentProof,
            line_number,
        )?,
    })
}

fn validate_targets(surface: &SemanticDeploymentSurface, errors: &mut Vec<ValidationError>) {
    for target in &surface.targets {
        validate_status(
            "target",
            &target.id,
            target.line_number,
            &target.status,
            errors,
        );
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
                format!("invalid environment {}", target.environment),
            ));
        }
        if target.artifacts.is_empty()
            || target.commands.is_empty()
            || target.receipts.is_empty()
            || target.forbids.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                "targets must bind artifacts, commands, receipts, and forbidden claims",
            ));
        }
        if !target.forbids.iter().any(|item| {
            item == "network_required" || item == "remote_service" || item == "cloud_dependency"
        }) {
            errors.push(ValidationError::reject(
                ErrorCode::DeploymentNetworkDependency,
                target.canonical_identity(),
                "targets must explicitly forbid remote dependency",
            ));
        }
        for artifact in &target.artifacts {
            validate_artifact_path(
                artifact,
                target.line_number,
                ErrorCode::InvalidDeploymentTarget,
                errors,
            );
        }
        for receipt in &target.receipts {
            validate_receipt_path(
                receipt,
                target.line_number,
                ErrorCode::InvalidDeploymentTarget,
                errors,
            );
        }
        for command in &target.commands {
            validate_command(
                command,
                target.line_number,
                ErrorCode::InvalidDeploymentTarget,
                errors,
            );
        }
        if let Some(descriptor) = semantic_deployment_target_descriptor(&target.id) {
            let digest = semantic_deployment_target_digest(&target.id).unwrap_or_default();
            if descriptor.kind != target.kind
                || descriptor.environment != target.environment
                || descriptor.artifacts != target.artifacts.as_slice()
                || descriptor.commands != target.commands.as_slice()
                || descriptor.receipts != target.receipts.as_slice()
                || descriptor.forbids != target.forbids.as_slice()
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentDriftAccepted,
                    target.canonical_identity(),
                    "target descriptor drift",
                ));
            }
        }
    }
}

fn validate_hooks(surface: &SemanticDeploymentSurface, errors: &mut Vec<ValidationError>) {
    for hook in &surface.hooks {
        validate_status("hook", &hook.id, hook.line_number, &hook.status, errors);
        if !ALLOWED_HOOK_SCOPES.contains(&hook.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidComplianceHook,
                hook.canonical_identity(),
                format!("invalid hook scope {}", hook.scope),
            ));
        }
        if surface.target_by_id(&hook.target).is_none() && hook.target != "P01" {
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
        for evidence_id in &hook.evidence {
            if surface.evidence_by_id(evidence_id).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidComplianceHook,
                    hook.canonical_identity(),
                    format!("unknown hook evidence {evidence_id}"),
                ));
            }
        }
        for receipt in &hook.receipts {
            validate_receipt_path(
                receipt,
                hook.line_number,
                ErrorCode::InvalidComplianceHook,
                errors,
            );
        }
        if let Some(descriptor) = semantic_deployment_hook_descriptor(&hook.id) {
            let digest = semantic_deployment_hook_digest(&hook.id).unwrap_or_default();
            if descriptor.scope != hook.scope
                || descriptor.target != hook.target
                || descriptor.requires != hook.requires.as_slice()
                || descriptor.evidence != hook.evidence.as_slice()
                || descriptor.receipts != hook.receipts.as_slice()
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentDriftAccepted,
                    hook.canonical_identity(),
                    "hook descriptor drift",
                ));
            }
        }
    }
}

fn validate_evidence(surface: &SemanticDeploymentSurface, errors: &mut Vec<ValidationError>) {
    for evidence in &surface.evidence {
        validate_status(
            "evidence",
            &evidence.id,
            evidence.line_number,
            &evidence.status,
            errors,
        );
        if !ALLOWED_EVIDENCE_KINDS.contains(&evidence.kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseEvidence,
                evidence.canonical_identity(),
                format!("invalid evidence kind {}", evidence.kind),
            ));
        }
        validate_artifact_path(
            &evidence.path,
            evidence.line_number,
            ErrorCode::InvalidReleaseEvidence,
            errors,
        );
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
        if evidence.receipts.is_empty() || evidence.commands.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReleaseEvidence,
                evidence.canonical_identity(),
                "evidence must bind receipts and commands",
            ));
        }
        for receipt in &evidence.receipts {
            validate_receipt_path(
                receipt,
                evidence.line_number,
                ErrorCode::InvalidReleaseEvidence,
                errors,
            );
        }
        for command in &evidence.commands {
            validate_command(
                command,
                evidence.line_number,
                ErrorCode::InvalidReleaseEvidence,
                errors,
            );
        }
        if let Some(descriptor) = semantic_deployment_evidence_descriptor(&evidence.id) {
            let digest = semantic_deployment_evidence_digest(&evidence.id).unwrap_or_default();
            if descriptor.kind != evidence.kind
                || descriptor.path != evidence.path
                || descriptor.targets != evidence.targets.as_slice()
                || descriptor.hooks != evidence.hooks.as_slice()
                || descriptor.receipts != evidence.receipts.as_slice()
                || descriptor.commands != evidence.commands.as_slice()
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentDriftAccepted,
                    evidence.canonical_identity(),
                    "evidence descriptor drift",
                ));
            }
        }
    }
}

fn validate_proofs(surface: &SemanticDeploymentSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentProof,
                proof.canonical_identity(),
                format!("invalid proof scope {}", proof.scope),
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
        for evidence_id in &proof.evidence {
            if surface.evidence_by_id(evidence_id).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof evidence {evidence_id}"),
                ));
            }
        }
        if proof.receipts.is_empty() || proof.commands.is_empty() || proof.forbids.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentProof,
                proof.canonical_identity(),
                "proofs must bind receipts, commands, and forbidden claims",
            ));
        }
        for receipt in &proof.receipts {
            validate_receipt_path(
                receipt,
                proof.line_number,
                ErrorCode::InvalidDeploymentProof,
                errors,
            );
        }
        for command in &proof.commands {
            validate_command(
                command,
                proof.line_number,
                ErrorCode::InvalidDeploymentProof,
                errors,
            );
        }
        if proof.id == "p01_phase_open"
            && !proof
                .forbids
                .iter()
                .any(|item| item == "phase_closure" || item == "global_complete")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "phase-open proof must forbid closure claims",
            ));
        }
        if let Some(descriptor) = semantic_deployment_proof_descriptor(&proof.id) {
            let digest = semantic_deployment_proof_digest(&proof.id).unwrap_or_default();
            if descriptor.scope != proof.scope
                || descriptor.targets != proof.targets.as_slice()
                || descriptor.hooks != proof.hooks.as_slice()
                || descriptor.evidence != proof.evidence.as_slice()
                || descriptor.receipts != proof.receipts.as_slice()
                || descriptor.commands != proof.commands.as_slice()
                || descriptor.forbids != proof.forbids.as_slice()
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::DeploymentDriftAccepted,
                    proof.canonical_identity(),
                    "proof descriptor drift",
                ));
            }
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
    if path.contains("..") || !allowed.iter().any(|prefix| path.starts_with(prefix)) {
        errors.push(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("invalid artifact path {path}"),
        ));
    }
}

fn validate_command(
    command: &str,
    line_number: usize,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    if !REQUIRED_COMMANDS.contains(&command) {
        errors.push(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("unknown semantic deployment command {command}"),
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
                format!("missing semantic deployment {kind} {id}"),
            ));
        }
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
                format!("forbidden semantic deployment token {needle}"),
            ));
        }
    }
}
