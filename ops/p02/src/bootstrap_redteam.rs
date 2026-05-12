use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_redteam::deterministic_bootstrap_redteam_suite_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_redteam::{
    bootstrap_redteam_artifacts_bind_paths, bootstrap_redteam_carrier_signature,
    bootstrap_redteam_no_forbidden_descriptor_claims, bootstrap_redteam_proof_descriptor,
    bootstrap_redteam_proof_digest, bootstrap_redteam_proofs_bind_registry,
    bootstrap_redteam_receipts_cover_p02_001_through_p02_023, bootstrap_redteam_registry_hash,
    bootstrap_redteam_rollbacks_bind_challenge_rights, bootstrap_redteam_scenario_descriptor,
    bootstrap_redteam_scenario_digest, bootstrap_redteam_scenarios_bind_rollbacks,
    bootstrap_rollback_path_descriptor, bootstrap_rollback_path_digest,
    LYRA_P02_BOOTSTRAP_REDTEAM_CARRIER,
};
use crate::p02_bootstrap_redteam_model::{
    BootstrapRedTeamProof, BootstrapRedTeamScenario, BootstrapRedTeamSurface, BootstrapRollbackPath,
};

pub const P02_BOOTSTRAP_REDTEAM_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-REDTEAM-ROLLBACK-LAW v1";

pub const REQUIRED_BOOTSTRAP_REDTEAM_RULES: &[&str] = &[
    "bootstrap_redteam_must_be_receipted",
    "rollback_paths_must_restore_last_good_truth",
    "challenge_rights_must_be_operator_invocable",
    "remote_truth_rewrite_must_be_rejected",
    "seed_runtime_redteam_must_bind_replacement_milestones",
    "host_extinction_redteam_must_bind_extinction_ledger",
    "economics_capture_redteam_must_bind_public_interest",
    "no_network_required_for_redteam_or_rollback",
    "no_unreceipted_rollback_path",
    "phase_open_until_redteam_and_rollback_proven",
];

pub const REQUIRED_BOOTSTRAP_REDTEAM_SCENARIOS: &[&str] = &[
    "ambient_host_dependency_attack",
    "remote_truth_rewrite_attack",
    "seed_runtime_drift_attack",
    "extinction_ledger_bypass_attack",
    "deployment_receipt_replay_attack",
    "economics_capture_attack",
    "closure_premature_claim_attack",
];

pub const REQUIRED_BOOTSTRAP_ROLLBACK_PATHS: &[&str] = &[
    "host_dependency_quarantine_rollback",
    "remote_import_rejection_rollback",
    "seed_runtime_last_good_restore",
    "extinction_ledger_reseal",
    "deployment_packet_replay_rollback",
    "economics_capture_reversal",
    "phase_open_reassertion_rollback",
];

pub const REQUIRED_BOOTSTRAP_REDTEAM_PROOFS: &[&str] = &[
    "redteam_coverage_proof",
    "rollback_receipt_binding_proof",
    "remote_truth_rewrite_rejection_proof",
    "challenge_right_enforcement_proof",
    "economics_capture_redteam_bridge_proof",
    "p02_phase_open",
];

const REQUIRED_COVERAGE_ANCHORS: &[&str] = &[
    "bootstrap_trust",
    "seed_runtime_law",
    "host_extinction_framework",
    "redteam",
    "rollback",
    "challenge_right",
    "remote_truth_rewrite",
    "receipt_replay",
    "economics_capture",
    "public_interest",
    "phase_open",
];
const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_ATTACK_KINDS: &[&str] = &[
    "host_dependency",
    "remote_truth_rewrite",
    "seed_runtime_drift",
    "extinction_bypass",
    "receipt_replay",
    "economics_capture",
    "closure_premature",
];
const ALLOWED_ROLLBACK_KINDS: &[&str] = &[
    "quarantine",
    "import_rejection",
    "last_good_restore",
    "ledger_reseal",
    "packet_replay",
    "capture_reversal",
    "phase_open_reassertion",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &[
    "redteam",
    "rollback",
    "remote_truth",
    "challenge",
    "economics_bridge",
    "phase",
];
const ALLOWED_TARGETS: &[&str] = &[
    "bootstrap_trust",
    "seed_runtime_law",
    "host_extinction_framework",
    "redteam",
    "rollback",
    "challenge_right",
    "remote_truth_rewrite",
    "receipt_replay",
    "economics_capture",
    "public_interest",
    "phase_open",
];
const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p02-bootstrap-inventory-check",
    "lyra-p02-bootstrap-extinction-check",
    "lyra-p02-host-boundary-check",
    "lyra-p02-target-matrix-check",
    "lyra-p02-truth-cleanup-check",
    "lyra-p02-emergency-fallback-check",
    "lyra-p02-seed-runtime-replacement-check",
    "lyra-p02-bootstrap-evidence-emission-check",
    "lyra-p02-operator-handoff-automation-check",
    "lyra-p02-foreign-surface-closure-check",
    "lyra-p02-bootstrap-formal-semantics-check",
    "lyra-p02-bootstrap-canonical-model-check",
    "lyra-p02-bootstrap-core-engine-check",
    "lyra-p02-bootstrap-falsification-check",
    "lyra-p02-bootstrap-replay-check",
    "lyra-p02-bootstrap-interface-check",
    "lyra-p02-bootstrap-packaging-check",
    "lyra-p02-bootstrap-deployment-check",
    "lyra-p02-bootstrap-ecosystem-check",
    "lyra-p02-bootstrap-economics-check",
    "lyra-p02-bootstrap-redteam-check",
];

const FORBIDDEN_BOOTSTRAP_REDTEAM_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::RedTeamNetworkDependency),
    ("cloud required", ErrorCode::RedTeamNetworkDependency),
    ("online required", ErrorCode::RedTeamNetworkDependency),
    (
        "remote service required",
        ErrorCode::RedTeamNetworkDependency,
    ),
    ("remote fetch", ErrorCode::RedTeamNetworkDependency),
    (
        "rollback unreceipted",
        ErrorCode::RedTeamRollbackUnreceipted,
    ),
    (
        "unreceipted rollback",
        ErrorCode::RedTeamRollbackUnreceipted,
    ),
    ("challenge bypass", ErrorCode::RedTeamChallengeBypass),
    ("challenge rights bypass", ErrorCode::RedTeamChallengeBypass),
    (
        "remote truth rewrite allowed",
        ErrorCode::RemoteTruthRewriteAllowed,
    ),
    (
        "remote consensus may silently rewrite",
        ErrorCode::RemoteTruthRewriteAllowed,
    ),
    ("redteam drift accepted", ErrorCode::RedTeamDriftAccepted),
    ("red-team drift accepted", ErrorCode::RedTeamDriftAccepted),
    ("corpus drift accepted", ErrorCode::CorpusDriftAccepted),
    ("manual only", ErrorCode::DocsOnlyImplementation),
    ("docs only", ErrorCode::DocsOnlyImplementation),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_bootstrap_redteam_surface(
    input: &str,
) -> Result<BootstrapRedTeamSurface, Vec<ValidationError>> {
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
            "no bootstrap redteam/rollback surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_REDTEAM_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_REDTEAM_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut scenarios = Vec::new();
    let mut rollbacks = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_scenarios = BTreeSet::new();
    let mut seen_rollbacks = BTreeSet::new();
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
                    "bootstrap redteam rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(scenario_id) = left.strip_prefix("scenario:") {
            if !is_symbolic_name(scenario_id) || !seen_scenarios.insert(scenario_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRedTeamScenario,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap redteam scenario {scenario_id}"),
                ));
                continue;
            }
            match parse_scenario(line_number, scenario_id, value) {
                Ok(scenario) => scenarios.push(scenario),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(rollback_id) = left.strip_prefix("rollback:") {
            if !is_symbolic_name(rollback_id) || !seen_rollbacks.insert(rollback_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRollbackPath,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap rollback path {rollback_id}"),
                ));
                continue;
            }
            match parse_rollback(line_number, rollback_id, value) {
                Ok(rollback) => rollbacks.push(rollback),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) || !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRedTeamProof,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap redteam proof {proof_id}"),
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
                format!("unknown bootstrap redteam key {left}"),
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
    Ok(BootstrapRedTeamSurface {
        header,
        phase: phase.unwrap(),
        task: task.unwrap(),
        status: status.unwrap(),
        rules,
        scenarios,
        rollbacks,
        proofs,
    })
}

pub fn validate_bootstrap_redteam_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_bootstrap_redteam_surface(input) {
        Ok(surface) => validate_bootstrap_redteam_model(&surface, &mut errors),
        Err(mut parse_errors) => errors.append(&mut parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_redteam_model(
    surface: &BootstrapRedTeamSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "bootstrap redteam/rollback law must bind to P02",
        ));
    }
    if surface.task != "P02-023" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "bootstrap redteam/rollback law must bind to P02-023",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported bootstrap redteam status {}", surface.status),
        ));
    }
    require_rules(surface, errors);
    require_scenarios(surface, errors);
    require_rollbacks(surface, errors);
    require_proofs(surface, errors);
    validate_scenarios(surface, errors);
    validate_rollbacks(surface, errors);
    validate_proofs(surface, errors);
    validate_coverage(surface, errors);
    validate_descriptor_registry(surface, errors);
    validate_bootstrap_redteam_report(surface, errors);
}

fn parse_scenario(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapRedTeamScenario, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidRedTeamScenario,
            format!("line:{line_number:03}"),
            "scenario fields must be key:value segments",
        )
    })?;
    Ok(BootstrapRedTeamScenario {
        line_number,
        id: id.to_string(),
        attack_kind: required_field(
            &fields,
            "kind",
            ErrorCode::InvalidRedTeamScenario,
            line_number,
        )?,
        path: required_field(
            &fields,
            "path",
            ErrorCode::InvalidRedTeamScenario,
            line_number,
        )?,
        targets: split_csv(&required_field(
            &fields,
            "targets",
            ErrorCode::InvalidRedTeamScenario,
            line_number,
        )?),
        rollback_paths: split_csv(&required_field(
            &fields,
            "rollbacks",
            ErrorCode::InvalidRedTeamScenario,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidRedTeamScenario,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidRedTeamScenario,
            line_number,
        )?),
        rejects: split_csv(&required_field(
            &fields,
            "rejects",
            ErrorCode::InvalidRedTeamScenario,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidRedTeamScenario,
            line_number,
        )?,
    })
}

fn parse_rollback(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapRollbackPath, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidRollbackPath,
            format!("line:{line_number:03}"),
            "rollback fields must be key:value segments",
        )
    })?;
    Ok(BootstrapRollbackPath {
        line_number,
        id: id.to_string(),
        rollback_kind: required_field(
            &fields,
            "kind",
            ErrorCode::InvalidRollbackPath,
            line_number,
        )?,
        path: required_field(&fields, "path", ErrorCode::InvalidRollbackPath, line_number)?,
        triggers: split_csv(&required_field(
            &fields,
            "triggers",
            ErrorCode::InvalidRollbackPath,
            line_number,
        )?),
        restores: split_csv(&required_field(
            &fields,
            "restores",
            ErrorCode::InvalidRollbackPath,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidRollbackPath,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidRollbackPath,
            line_number,
        )?),
        challenge_rights: split_csv(&required_field(
            &fields,
            "rights",
            ErrorCode::InvalidRollbackPath,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidRollbackPath,
            line_number,
        )?,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapRedTeamProof, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidRedTeamProof,
            format!("line:{line_number:03}"),
            "proof fields must be key:value segments",
        )
    })?;
    Ok(BootstrapRedTeamProof {
        line_number,
        id: id.to_string(),
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::InvalidRedTeamProof,
            line_number,
        )?,
        scenarios: split_csv(&required_field(
            &fields,
            "scenarios",
            ErrorCode::InvalidRedTeamProof,
            line_number,
        )?),
        rollbacks: split_csv(&required_field(
            &fields,
            "rollbacks",
            ErrorCode::InvalidRedTeamProof,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidRedTeamProof,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidRedTeamProof,
            line_number,
        )?),
        forbids: split_csv(&required_field(
            &fields,
            "forbids",
            ErrorCode::InvalidRedTeamProof,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidRedTeamProof,
            line_number,
        )?,
    })
}

fn require_rules(surface: &BootstrapRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_BOOTSTRAP_REDTEAM_RULES {
        if !surface.rules.contains_key(*rule) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamRule,
                format!("rule:{rule}"),
                "missing required bootstrap redteam rule",
            ));
        }
    }
}
fn require_scenarios(surface: &BootstrapRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_BOOTSTRAP_REDTEAM_SCENARIOS {
        if surface.scenario_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamScenario,
                format!("scenario:{id}"),
                "missing required bootstrap redteam scenario",
            ));
        }
    }
}
fn require_rollbacks(surface: &BootstrapRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_BOOTSTRAP_ROLLBACK_PATHS {
        if surface.rollback_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRollbackPath,
                format!("rollback:{id}"),
                "missing required bootstrap rollback path",
            ));
        }
    }
}
fn require_proofs(surface: &BootstrapRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_BOOTSTRAP_REDTEAM_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamProof,
                format!("proof:{id}"),
                "missing required bootstrap redteam proof",
            ));
        }
    }
}

fn validate_scenarios(surface: &BootstrapRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for scenario in &surface.scenarios {
        if !ALLOWED_ATTACK_KINDS.contains(&scenario.attack_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                format!("invalid attack kind {}", scenario.attack_kind),
            ));
        }
        if !allowed_artifact_path(&scenario.path) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                format!("invalid scenario path {}", scenario.path),
            ));
        }
        if scenario.targets.is_empty()
            || scenario.rollback_paths.is_empty()
            || scenario.commands.is_empty()
            || scenario.receipts.is_empty()
            || scenario.rejects.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                "scenario must bind targets, rollback paths, commands, receipts, and rejects",
            ));
        }
        for target in &scenario.targets {
            if !ALLOWED_TARGETS.contains(&target.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRedTeamScenario,
                    scenario.canonical_identity(),
                    format!("unknown scenario target {target}"),
                ));
            }
        }
        for rollback in &scenario.rollback_paths {
            if surface.rollback_by_id(rollback).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRedTeamScenario,
                    scenario.canonical_identity(),
                    format!("unknown scenario rollback {rollback}"),
                ));
            }
        }
        for command in &scenario.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRedTeamScenario,
                    scenario.canonical_identity(),
                    format!("unknown scenario command {command}"),
                ));
            }
        }
        for receipt in &scenario.receipts {
            if !receipt.starts_with("receipts/p02/") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRedTeamScenario,
                    scenario.canonical_identity(),
                    format!("invalid scenario receipt {receipt}"),
                ));
            }
        }
        if !ALLOWED_STATUSES.contains(&scenario.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                format!("invalid scenario status {}", scenario.status),
            ));
        }
    }
}

fn validate_rollbacks(surface: &BootstrapRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for rollback in &surface.rollbacks {
        if !ALLOWED_ROLLBACK_KINDS.contains(&rollback.rollback_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                format!("invalid rollback kind {}", rollback.rollback_kind),
            ));
        }
        if !allowed_artifact_path(&rollback.path) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                format!("invalid rollback path {}", rollback.path),
            ));
        }
        if rollback.triggers.is_empty()
            || rollback.restores.is_empty()
            || rollback.receipts.is_empty()
            || rollback.commands.is_empty()
            || rollback.challenge_rights.is_empty()
        {
            errors.push(ValidationError::reject(ErrorCode::InvalidRollbackPath, rollback.canonical_identity(), "rollback must bind triggers, restore targets, receipts, commands, and challenge rights"));
        }
        for restore in &rollback.restores {
            if !ALLOWED_TARGETS.contains(&restore.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRollbackPath,
                    rollback.canonical_identity(),
                    format!("unknown rollback restore target {restore}"),
                ));
            }
        }
        for command in &rollback.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRollbackPath,
                    rollback.canonical_identity(),
                    format!("unknown rollback command {command}"),
                ));
            }
        }
        for receipt in &rollback.receipts {
            if !receipt.starts_with("receipts/p02/") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRollbackPath,
                    rollback.canonical_identity(),
                    format!("invalid rollback receipt {receipt}"),
                ));
            }
        }
        if !rollback
            .challenge_rights
            .iter()
            .any(|right| right.ends_with("challenge"))
        {
            errors.push(ValidationError::reject(
                ErrorCode::RedTeamChallengeBypass,
                rollback.canonical_identity(),
                "rollback must expose at least one challenge right",
            ));
        }
        if !ALLOWED_STATUSES.contains(&rollback.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                format!("invalid rollback status {}", rollback.status),
            ));
        }
    }
}

fn validate_proofs(surface: &BootstrapRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamProof,
                proof.canonical_identity(),
                format!("invalid proof scope {}", proof.scope),
            ));
        }
        if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamProof,
                proof.canonical_identity(),
                format!("invalid proof status {}", proof.status),
            ));
        }
        for scenario in &proof.scenarios {
            if surface.scenario_by_id(scenario).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::RedTeamProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof scenario {scenario}"),
                ));
            }
        }
        for rollback in &proof.rollbacks {
            if surface.rollback_by_id(rollback).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::RedTeamProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof rollback {rollback}"),
                ));
            }
        }
        for command in &proof.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::RedTeamProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof command {command}"),
                ));
            }
        }
        if proof.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamProof,
                proof.canonical_identity(),
                "bootstrap redteam proofs must bind receipts",
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
                "redteam proof must keep P02 phase open until closure gate",
            ));
        }
        if !proof.forbids.iter().any(|item| {
            item == "remote_truth_rewrite"
                || item == "challenge_bypass"
                || item == "unreceipted_rollback"
                || item == "capture"
        }) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamProof,
                proof.canonical_identity(),
                "redteam proof must forbid at least one rollback/challenge/remote/capture failure",
            ));
        }
    }
}

fn validate_coverage(surface: &BootstrapRedTeamSurface, errors: &mut Vec<ValidationError>) {
    let mut covered = BTreeSet::new();
    covered.insert("redteam");
    for scenario in &surface.scenarios {
        for target in &scenario.targets {
            covered.insert(target.as_str());
        }
    }
    for anchor in REQUIRED_COVERAGE_ANCHORS {
        if !covered.contains(*anchor) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                format!("coverage:{anchor}"),
                "bootstrap redteam scenario targets must name each governance anchor explicitly (rollback restores cannot satisfy coverage alone)",
            ));
        }
    }
}

fn validate_descriptor_registry(
    surface: &BootstrapRedTeamSurface,
    errors: &mut Vec<ValidationError>,
) {
    if LYRA_P02_BOOTSTRAP_REDTEAM_CARRIER != "lyra.p02.bootstrap_redteam.carrier.v1"
        || !bootstrap_redteam_carrier_signature().starts_with("fnv1a128:")
        || !bootstrap_redteam_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::RedTeamDriftAccepted,
            "lyralang_bootstrap_redteam_registry",
            "bootstrap redteam carrier and registry hashes must be stable",
        ));
    }
    if !bootstrap_redteam_scenarios_bind_rollbacks()
        || !bootstrap_redteam_rollbacks_bind_challenge_rights()
        || !bootstrap_redteam_proofs_bind_registry()
        || !bootstrap_redteam_artifacts_bind_paths()
        || !bootstrap_redteam_receipts_cover_p02_001_through_p02_023()
    {
        errors.push(ValidationError::reject(
            ErrorCode::RedTeamDriftAccepted,
            "lyralang_bootstrap_redteam_registry",
            "bootstrap redteam descriptor registry is not fully bound",
        ));
    }
    if !bootstrap_redteam_no_forbidden_descriptor_claims() {
        errors.push(ValidationError::reject(
            ErrorCode::RedTeamChallengeBypass,
            "lyralang_bootstrap_redteam_registry",
            "bootstrap redteam descriptors contain forbidden challenge/network/remote claims",
        ));
    }
    for scenario in &surface.scenarios {
        match bootstrap_redteam_scenario_descriptor(&scenario.id) {
            Some(descriptor)
                if descriptor.kind == scenario.attack_kind
                    && descriptor.path == scenario.path
                    && descriptor_slice_eq(descriptor.targets, &scenario.targets)
                    && descriptor_slice_eq(descriptor.rollbacks, &scenario.rollback_paths)
                    && descriptor_slice_eq(descriptor.commands, &scenario.commands)
                    && descriptor_slice_eq(descriptor.receipts, &scenario.receipts)
                    && descriptor_slice_eq(descriptor.rejects, &scenario.rejects)
                    && descriptor.status == scenario.status
                    && bootstrap_redteam_scenario_digest(&scenario.id)
                        .map(|hash| hash.starts_with("fnv1a128:"))
                        .unwrap_or(false) => {}
            _ => errors.push(ValidationError::reject(
                ErrorCode::RedTeamDriftAccepted,
                scenario.canonical_identity(),
                "bootstrap redteam scenario drifted from LyraLang descriptor",
            )),
        }
    }
    for rollback in &surface.rollbacks {
        match bootstrap_rollback_path_descriptor(&rollback.id) {
            Some(descriptor)
                if descriptor.kind == rollback.rollback_kind
                    && descriptor.path == rollback.path
                    && descriptor_slice_eq(descriptor.triggers, &rollback.triggers)
                    && descriptor_slice_eq(descriptor.restores, &rollback.restores)
                    && descriptor_slice_eq(descriptor.receipts, &rollback.receipts)
                    && descriptor_slice_eq(descriptor.commands, &rollback.commands)
                    && descriptor_slice_eq(
                        descriptor.challenge_rights,
                        &rollback.challenge_rights,
                    )
                    && descriptor.status == rollback.status
                    && bootstrap_rollback_path_digest(&rollback.id)
                        .map(|hash| hash.starts_with("fnv1a128:"))
                        .unwrap_or(false) => {}
            _ => errors.push(ValidationError::reject(
                ErrorCode::RedTeamDriftAccepted,
                rollback.canonical_identity(),
                "bootstrap rollback path drifted from LyraLang descriptor",
            )),
        }
    }
    for proof in &surface.proofs {
        match bootstrap_redteam_proof_descriptor(&proof.id) {
            Some(descriptor)
                if descriptor.scope == proof.scope
                    && descriptor_slice_eq(descriptor.scenarios, &proof.scenarios)
                    && descriptor_slice_eq(descriptor.rollbacks, &proof.rollbacks)
                    && descriptor_slice_eq(descriptor.receipts, &proof.receipts)
                    && descriptor_slice_eq(descriptor.commands, &proof.commands)
                    && descriptor_slice_eq(descriptor.forbids, &proof.forbids)
                    && descriptor.status == proof.status
                    && bootstrap_redteam_proof_digest(&proof.id)
                        .map(|hash| hash.starts_with("fnv1a128:"))
                        .unwrap_or(false) => {}
            _ => errors.push(ValidationError::reject(
                ErrorCode::RedTeamDriftAccepted,
                proof.canonical_identity(),
                "bootstrap redteam proof drifted from LyraLang descriptor",
            )),
        }
    }
}

fn descriptor_slice_eq(left: &[&str], right: &[String]) -> bool {
    left.len() == right.len() && left.iter().zip(right.iter()).all(|(a, b)| *a == b.as_str())
}

fn validate_bootstrap_redteam_report(
    surface: &BootstrapRedTeamSurface,
    errors: &mut Vec<ValidationError>,
) {
    let scenario_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .scenarios
        .iter()
        .map(|scenario| {
            (
                scenario.id.clone(),
                scenario.attack_kind.clone(),
                scenario.path.clone(),
                scenario.targets.clone(),
                scenario.rollback_paths.clone(),
                scenario.commands.clone(),
                scenario.receipts.clone(),
                scenario.rejects.clone(),
                scenario.status.clone(),
            )
        })
        .collect();
    let rollback_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .rollbacks
        .iter()
        .map(|rollback| {
            (
                rollback.id.clone(),
                rollback.rollback_kind.clone(),
                rollback.path.clone(),
                rollback.triggers.clone(),
                rollback.restores.clone(),
                rollback.receipts.clone(),
                rollback.commands.clone(),
                rollback.challenge_rights.clone(),
                rollback.status.clone(),
            )
        })
        .collect();
    let proof_inputs: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .proofs
        .iter()
        .map(|proof| {
            (
                proof.id.clone(),
                proof.scope.clone(),
                proof.scenarios.clone(),
                proof.rollbacks.clone(),
                proof.receipts.clone(),
                proof.commands.clone(),
                proof.forbids.clone(),
                proof.status.clone(),
            )
        })
        .collect();
    let report = deterministic_bootstrap_redteam_suite_report(
        &scenario_inputs,
        &rollback_inputs,
        &proof_inputs,
    );
    if report.scenario_count != surface.scenarios.len()
        || report.rollback_count != surface.rollbacks.len()
        || report.proof_count != surface.proofs.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::RedTeamDriftAccepted,
            "k0_bootstrap_redteam_report",
            "bootstrap redteam report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::RedTeamDriftAccepted,
            "k0_bootstrap_redteam_report",
            "bootstrap redteam report hash must be stable fnv1a128",
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
fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_BOOTSTRAP_REDTEAM_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden bootstrap redteam token {needle}"),
            ));
        }
    }
}
