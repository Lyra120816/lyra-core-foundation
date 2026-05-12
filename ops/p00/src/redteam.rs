use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_redteam::deterministic_redteam_rollback_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_redteam_model::{
    RedTeamProof, RedTeamRollbackSurface, RedTeamScenario, RollbackPath,
};

pub const P00_REDTEAM_ROLLBACK_CONTRACT: &str = "LYRA-P00-REDTEAM-ROLLBACK v1";

pub const REQUIRED_REDTEAM_RULES: &[&str] = &[
    "red_team_must_be_receipted",
    "rollback_must_be_receipt_bound",
    "adversarial_paths_must_be_executable",
    "challenge_rights_must_survive_rollback",
    "constitution_people_first_rebuild_coverage",
    "no_network_dependency",
    "no_unreceipted_rollback",
    "phase_open_until_redteam_proven",
];

pub const REQUIRED_REDTEAM_SCENARIOS: &[&str] = &[
    "determinism_drift_attack",
    "people_first_capture_attack",
    "rebuild_governance_bypass_attack",
    "remote_truth_rewrite_attack",
    "closure_fraud_attack",
];

pub const REQUIRED_ROLLBACK_PATHS: &[&str] = &[
    "receipt_bound_constitution_rollback",
    "people_first_policy_rollback",
    "rebuild_governance_replay_rollback",
    "control_plane_frontier_rollback",
    "challenge_review_rollback",
];

pub const REQUIRED_REDTEAM_PROOFS: &[&str] = &[
    "redteam_coverage_proof",
    "rollback_authority_proof",
    "receipt_binding_proof",
    "adversarial_rejection_proof",
    "p00_phase_open",
];

const REQUIRED_COVERAGE_ANCHORS: &[&str] = &[
    "determinism",
    "people_first",
    "rebuild_governance",
    "rollback",
    "redteam",
];
const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_SCENARIO_KINDS: &[&str] = &[
    "determinism",
    "people_first",
    "rebuild_governance",
    "remote_truth",
    "closure_fraud",
];
const ALLOWED_ROLLBACK_KINDS: &[&str] = &[
    "constitution",
    "policy",
    "rebuild",
    "control_plane",
    "challenge_review",
];
const ALLOWED_ROLLBACK_AUTHORITIES: &[&str] = &[
    "constitution_master",
    "people_first_law",
    "rebuild_governance",
    "control_plane",
    "challenge_right",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["redteam", "rollback", "receipt", "adversarial", "phase"];

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
    "lyra-p00-ecosystem-check",
    "lyra-p00-economics-check",
    "lyra-p00-redteam-check",
];

const FORBIDDEN_REDTEAM_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::RedTeamNetworkDependency),
    ("cloud required", ErrorCode::RedTeamNetworkDependency),
    ("online required", ErrorCode::RedTeamNetworkDependency),
    (
        "remote service required",
        ErrorCode::RedTeamNetworkDependency,
    ),
    ("remote fetch", ErrorCode::RedTeamNetworkDependency),
    (
        "remote truth rewrite allowed",
        ErrorCode::RemoteTruthRewriteAllowed,
    ),
    (
        "remote consensus may rewrite local truth",
        ErrorCode::RemoteTruthRewriteAllowed,
    ),
    (
        "rollback without receipt",
        ErrorCode::RedTeamRollbackUnreceipted,
    ),
    (
        "unreceipted rollback allowed",
        ErrorCode::RedTeamRollbackUnreceipted,
    ),
    ("rollback drift accepted", ErrorCode::RedTeamDriftAccepted),
    ("red team drift accepted", ErrorCode::RedTeamDriftAccepted),
    (
        "challenge bypass allowed",
        ErrorCode::RedTeamChallengeBypass,
    ),
    (
        "retaliatory rollback allowed",
        ErrorCode::RedTeamChallengeBypass,
    ),
    ("manual only", ErrorCode::DocsOnlyImplementation),
    ("docs only", ErrorCode::DocsOnlyImplementation),
    ("docs_only", ErrorCode::DocsOnlyImplementation),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_redteam_rollback_surface(
    input: &str,
) -> Result<RedTeamRollbackSurface, Vec<ValidationError>> {
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
            "no red-team rollback surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_REDTEAM_ROLLBACK_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_REDTEAM_ROLLBACK_CONTRACT}"),
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
                    "red-team rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(scenario_id) = left.strip_prefix("scenario:") {
            if !is_symbolic_name(scenario_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRedTeamScenario,
                    format!("line:{line_number:03}"),
                    format!("invalid red-team scenario identity {scenario_id}"),
                ));
                continue;
            }
            if !seen_scenarios.insert(scenario_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRedTeamScenario,
                    format!("scenario:{scenario_id}"),
                    "red-team scenario identity must be unique",
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
            if !is_symbolic_name(rollback_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRollbackPath,
                    format!("line:{line_number:03}"),
                    format!("invalid rollback path identity {rollback_id}"),
                ));
                continue;
            }
            if !seen_rollbacks.insert(rollback_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRollbackPath,
                    format!("rollback:{rollback_id}"),
                    "rollback path identity must be unique",
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
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRedTeamProof,
                    format!("line:{line_number:03}"),
                    format!("invalid red-team proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRedTeamProof,
                    format!("proof:{proof_id}"),
                    "red-team proof identity must be unique",
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
                format!("unknown red-team rollback key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(RedTeamRollbackSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        scenarios,
        rollbacks,
        proofs,
    })
}

pub fn validate_redteam_rollback_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_redteam_rollback_surface(input) {
        Ok(surface) => errors.extend(validate_redteam_rollback_model(&surface).errors),
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

pub fn validate_redteam_rollback_model(surface: &RedTeamRollbackSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "red-team rollback law must bind to P00",
        ));
    }
    if surface.task != "P00-023" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "red-team rollback law must bind to P00-023",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported red-team rollback status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_scenarios(surface, &mut errors);
    require_rollbacks(surface, &mut errors);
    require_proofs(surface, &mut errors);
    validate_scenarios(surface, &mut errors);
    validate_rollbacks(surface, &mut errors);
    validate_proofs(surface, &mut errors);
    validate_coverage(surface, &mut errors);
    validate_redteam_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_scenario(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<RedTeamScenario, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidRedTeamScenario,
            format!("line:{line_number:03}"),
            "scenario fields must be key:value segments",
        )
    })?;
    let scenario_kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidRedTeamScenario,
        line_number,
    )?;
    let path = required_field(
        &fields,
        "path",
        ErrorCode::InvalidRedTeamScenario,
        line_number,
    )?;
    let targets = split_csv(&required_field(
        &fields,
        "targets",
        ErrorCode::InvalidRedTeamScenario,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidRedTeamScenario,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidRedTeamScenario,
        line_number,
    )?);
    let rejects = split_csv(&required_field(
        &fields,
        "rejects",
        ErrorCode::InvalidRedTeamScenario,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidRedTeamScenario,
        line_number,
    )?;
    Ok(RedTeamScenario {
        line_number,
        id: id.to_string(),
        scenario_kind,
        path,
        targets,
        commands,
        receipts,
        rejects,
        status,
    })
}

fn parse_rollback(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<RollbackPath, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidRollbackPath,
            format!("line:{line_number:03}"),
            "rollback fields must be key:value segments",
        )
    })?;
    let rollback_kind =
        required_field(&fields, "kind", ErrorCode::InvalidRollbackPath, line_number)?;
    let path = required_field(&fields, "path", ErrorCode::InvalidRollbackPath, line_number)?;
    let authority = required_field(
        &fields,
        "authority",
        ErrorCode::InvalidRollbackPath,
        line_number,
    )?;
    let scenarios = split_csv(&required_field(
        &fields,
        "scenarios",
        ErrorCode::InvalidRollbackPath,
        line_number,
    )?);
    let proofs = split_csv(&required_field(
        &fields,
        "proofs",
        ErrorCode::InvalidRollbackPath,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidRollbackPath,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidRollbackPath,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidRollbackPath,
        line_number,
    )?;
    Ok(RollbackPath {
        line_number,
        id: id.to_string(),
        rollback_kind,
        path,
        authority,
        scenarios,
        proofs,
        receipts,
        commands,
        status,
    })
}

fn parse_proof(line_number: usize, id: &str, value: &str) -> Result<RedTeamProof, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidRedTeamProof,
            format!("line:{line_number:03}"),
            "proof fields must be key:value segments",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidRedTeamProof,
        line_number,
    )?;
    let scenarios = split_csv(&required_field(
        &fields,
        "scenarios",
        ErrorCode::InvalidRedTeamProof,
        line_number,
    )?);
    let rollbacks = split_csv(&required_field(
        &fields,
        "rollbacks",
        ErrorCode::InvalidRedTeamProof,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidRedTeamProof,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidRedTeamProof,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidRedTeamProof,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidRedTeamProof,
        line_number,
    )?;
    Ok(RedTeamProof {
        line_number,
        id: id.to_string(),
        scope,
        scenarios,
        rollbacks,
        receipts,
        commands,
        forbids,
        status,
    })
}

fn require_rules(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_REDTEAM_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("blocked_until_proven") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamRule,
                format!("rule:{rule}"),
                "required red-team rollback rule missing",
            )),
        }
    }
}

fn require_scenarios(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_REDTEAM_SCENARIOS {
        if surface.scenario_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamScenario,
                format!("scenario:{id}"),
                "required red-team scenario missing",
            ));
        }
    }
}

fn require_rollbacks(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_ROLLBACK_PATHS {
        if surface.rollback_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRollbackPath,
                format!("rollback:{id}"),
                "required rollback path missing",
            ));
        }
    }
}

fn require_proofs(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_REDTEAM_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamProof,
                format!("proof:{id}"),
                "required red-team rollback proof missing",
            ));
        }
    }
}

fn validate_scenarios(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
    for scenario in &surface.scenarios {
        if !ALLOWED_SCENARIO_KINDS.contains(&scenario.scenario_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                format!("invalid scenario kind {}", scenario.scenario_kind),
            ));
        }
        if !scenario.path.starts_with("fixtures/")
            && !scenario.path.starts_with("examples/")
            && !scenario.path.starts_with("ops/")
            && !scenario.path.starts_with("products/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                format!("invalid scenario path {}", scenario.path),
            ));
        }
        if scenario.targets.is_empty()
            || scenario.commands.is_empty()
            || scenario.receipts.is_empty()
            || scenario.rejects.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                "scenarios must bind targets, commands, receipts, and rejection assertions",
            ));
        }
        if !scenario
            .commands
            .iter()
            .any(|command| command == "lyra-p00-redteam-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                "scenarios must be checkable by lyra-p00-redteam-check",
            ));
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
        if !ALLOWED_STATUSES.contains(&scenario.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                format!("invalid scenario status {}", scenario.status),
            ));
        }
    }
}

fn validate_rollbacks(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
    for rollback in &surface.rollbacks {
        if !ALLOWED_ROLLBACK_KINDS.contains(&rollback.rollback_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                format!("invalid rollback kind {}", rollback.rollback_kind),
            ));
        }
        if !rollback.path.starts_with("examples/")
            && !rollback.path.starts_with("ops/")
            && !rollback.path.starts_with("products/")
            && !rollback.path.starts_with("docs/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                format!("invalid rollback path {}", rollback.path),
            ));
        }
        if !ALLOWED_ROLLBACK_AUTHORITIES.contains(&rollback.authority.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                format!("invalid rollback authority {}", rollback.authority),
            ));
        }
        if rollback.scenarios.is_empty()
            || rollback.proofs.is_empty()
            || rollback.receipts.is_empty()
            || rollback.commands.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                "rollback paths must bind scenarios, proofs, receipts, and commands",
            ));
        }
        if !rollback
            .commands
            .iter()
            .any(|command| command == "lyra-p00-redteam-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                "rollback paths must be checkable by lyra-p00-redteam-check",
            ));
        }
        for scenario in &rollback.scenarios {
            if surface.scenario_by_id(scenario).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::RedTeamProofUnbound,
                    rollback.canonical_identity(),
                    format!("unknown rollback scenario {scenario}"),
                ));
            }
        }
        for proof in &rollback.proofs {
            if surface.proof_by_id(proof).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::RedTeamProofUnbound,
                    rollback.canonical_identity(),
                    format!("unknown rollback proof {proof}"),
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
        if !ALLOWED_STATUSES.contains(&rollback.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                format!("invalid rollback status {}", rollback.status),
            ));
        }
    }
}

fn validate_proofs(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
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
                "red-team proofs must bind receipts",
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
                "red-team proof must keep P00 phase open until closure gate",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|item| item == "unreceipted_rollback")
        {
            errors.push(ValidationError::reject(
                ErrorCode::RedTeamRollbackUnreceipted,
                proof.canonical_identity(),
                "red-team proof must forbid unreceipted rollback",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|item| item == "remote_truth_rewrite")
        {
            errors.push(ValidationError::reject(
                ErrorCode::RemoteTruthRewriteAllowed,
                proof.canonical_identity(),
                "red-team proof must forbid remote truth rewrite",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|item| item == "challenge_bypass" || item == "retaliatory_rollback")
        {
            errors.push(ValidationError::reject(
                ErrorCode::RedTeamChallengeBypass,
                proof.canonical_identity(),
                "red-team proof must forbid challenge bypass and retaliatory rollback",
            ));
        }
    }
}

fn validate_coverage(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
    let mut covered = BTreeSet::new();
    for scenario in &surface.scenarios {
        for target in &scenario.targets {
            covered.insert(target.as_str());
        }
    }
    for anchor in REQUIRED_COVERAGE_ANCHORS {
        if !covered.contains(*anchor) {
            errors.push(ValidationError::reject(ErrorCode::InvalidRedTeamScenario, format!("coverage:{anchor}"), "red-team scenarios must cover determinism, people-first law, rebuild governance, rollback, and red-team paths"));
        }
    }
}

fn validate_redteam_report(surface: &RedTeamRollbackSurface, errors: &mut Vec<ValidationError>) {
    let scenario_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )> = surface
        .scenarios
        .iter()
        .map(|scenario| {
            (
                scenario.id.clone(),
                scenario.scenario_kind.clone(),
                scenario.path.clone(),
                scenario.targets.clone(),
                scenario.commands.clone(),
                scenario.rejects.clone(),
                scenario.receipts.clone(),
            )
        })
        .collect();
    let rollback_inputs: Vec<(
        String,
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )> = surface
        .rollbacks
        .iter()
        .map(|rollback| {
            (
                rollback.id.clone(),
                rollback.rollback_kind.clone(),
                rollback.path.clone(),
                rollback.authority.clone(),
                rollback.scenarios.clone(),
                rollback.proofs.clone(),
                rollback.receipts.clone(),
                rollback.commands.clone(),
            )
        })
        .collect();
    let report = deterministic_redteam_rollback_report(
        &scenario_inputs,
        &rollback_inputs,
        surface.proofs.len(),
    );
    if report.scenario_count != surface.scenarios.len()
        || report.rollback_count != surface.rollbacks.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::RedTeamDriftAccepted,
            "k0_redteam_report",
            "red-team rollback report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::RedTeamDriftAccepted,
            "k0_redteam_report",
            "red-team rollback report hash must be stable fnv1a128",
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
    for (needle, code) in FORBIDDEN_REDTEAM_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden red-team rollback token {needle}"),
            ));
        }
    }
}
