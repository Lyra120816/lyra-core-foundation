use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_redteam::deterministic_semantic_redteam_rollback_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_redteam_model::{
    SemanticRedTeamProof, SemanticRedTeamScenario, SemanticRedTeamSurface, SemanticRollbackPath,
};

pub const P01_SEMANTIC_REDTEAM_CONTRACT: &str = "LYRA-P01-SEMANTIC-REDTEAM-ROLLBACK v1";

pub const REQUIRED_SEMANTIC_REDTEAM_RULES: &[&str] = &[
    "semantic_redteam_must_be_receipted",
    "semantic_rollback_must_be_receipt_bound",
    "adversarial_semantic_paths_must_be_executable",
    "canonical_symbol_challenge_rights_must_survive_rollback",
    "semantic_atoms_core_ir_rebuild_coverage",
    "no_network_dependency",
    "no_unreceipted_rollback",
    "phase_open_until_semantic_redteam_proven",
];

pub const REQUIRED_SEMANTIC_REDTEAM_SCENARIOS: &[&str] = &[
    "canonical_symbol_drift_attack",
    "semantic_atom_mutation_attack",
    "core_ir_upgrade_bypass_attack",
    "receipt_replay_poisoning_attack",
    "remote_semantic_truth_rewrite_attack",
    "phase_closure_fraud_attack",
];

pub const REQUIRED_SEMANTIC_ROLLBACK_PATHS: &[&str] = &[
    "canonical_symbol_receipt_rollback",
    "semantic_atom_state_rollback",
    "core_ir_upgrade_rollback",
    "semantic_replay_witness_rollback",
    "control_plane_frontier_rollback",
    "challenge_review_rollback",
];

pub const REQUIRED_SEMANTIC_REDTEAM_PROOFS: &[&str] = &[
    "semantic_redteam_coverage_proof",
    "semantic_rollback_authority_proof",
    "receipt_binding_proof",
    "adversarial_semantic_rejection_proof",
    "remote_truth_rewrite_rejection_proof",
    "p01_phase_open",
];

const REQUIRED_COVERAGE_ANCHORS: &[&str] = &[
    "canonical_symbols",
    "semantic_atoms",
    "core_ir",
    "receipt_replay",
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
    "canonical_symbol",
    "semantic_atom",
    "core_ir",
    "replay_poisoning",
    "remote_truth",
    "closure_fraud",
];
const ALLOWED_ROLLBACK_KINDS: &[&str] = &[
    "canonical_symbol",
    "semantic_atom",
    "core_ir",
    "replay_witness",
    "control_plane",
    "challenge_review",
];
const ALLOWED_ROLLBACK_AUTHORITIES: &[&str] = &[
    "semantic_constitution",
    "receipt_chain",
    "replay_witness",
    "control_plane",
    "challenge_right",
    "package_release_law",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &[
    "redteam",
    "rollback",
    "receipt",
    "adversarial",
    "remote_truth",
    "phase",
];

const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p01-atom-check",
    "lyra-p01-ir-check",
    "lyra-p01-semantic-core-engine-check",
    "lyra-p01-semantic-falsification-check",
    "lyra-p01-semantic-replay-check",
    "lyra-p01-semantic-interface-check",
    "lyra-p01-semantic-packaging-check",
    "lyra-p01-semantic-deployment-check",
    "lyra-p01-semantic-economics-check",
    "lyra-p01-semantic-redteam-check",
];

const FORBIDDEN_SEMANTIC_REDTEAM_TEXT: &[(&str, ErrorCode)] = &[
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
    ("redteam drift accepted", ErrorCode::RedTeamDriftAccepted),
    ("red team drift accepted", ErrorCode::RedTeamDriftAccepted),
    (
        "challenge bypass allowed",
        ErrorCode::RedTeamChallengeBypass,
    ),
    (
        "retaliatory rollback allowed",
        ErrorCode::RedTeamChallengeBypass,
    ),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_redteam_surface(
    input: &str,
) -> Result<SemanticRedTeamSurface, Vec<ValidationError>> {
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
            "no semantic redteam surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_REDTEAM_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_REDTEAM_CONTRACT}"),
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
                    "semantic redteam rule names must be symbolic and unique",
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
                    format!("invalid semantic redteam scenario identity {scenario_id}"),
                ));
                continue;
            }
            if !seen_scenarios.insert(scenario_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRedTeamScenario,
                    format!("scenario:{scenario_id}"),
                    "semantic redteam scenario identity must be unique",
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
                    format!("invalid semantic rollback path identity {rollback_id}"),
                ));
                continue;
            }
            if !seen_rollbacks.insert(rollback_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRollbackPath,
                    format!("rollback:{rollback_id}"),
                    "semantic rollback path identity must be unique",
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
                    format!("invalid semantic redteam proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRedTeamProof,
                    format!("proof:{proof_id}"),
                    "semantic redteam proof identity must be unique",
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
                format!("unknown semantic redteam key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(SemanticRedTeamSurface {
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

pub fn validate_semantic_redteam_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_semantic_redteam_surface(input) {
        Ok(surface) => errors.extend(validate_semantic_redteam_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }

    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_semantic_redteam_model(surface: &SemanticRedTeamSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic redteam law must bind to P01",
        ));
    }
    if surface.task != "P01-023" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic redteam law must bind to P01-023",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported semantic redteam status {}", surface.status),
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
    validate_semantic_redteam_report(surface, &mut errors);
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
) -> Result<SemanticRedTeamScenario, ValidationError> {
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
    Ok(SemanticRedTeamScenario {
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
) -> Result<SemanticRollbackPath, ValidationError> {
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
    Ok(SemanticRollbackPath {
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

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticRedTeamProof, ValidationError> {
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
    Ok(SemanticRedTeamProof {
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

fn require_rules(surface: &SemanticRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_SEMANTIC_REDTEAM_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") | Some("blocked_until_proven") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamRule,
                format!("rule:{rule}"),
                "required semantic redteam rule missing",
            )),
        }
    }
}

fn require_scenarios(surface: &SemanticRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_REDTEAM_SCENARIOS {
        if surface.scenario_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamScenario,
                format!("scenario:{id}"),
                "required semantic redteam scenario missing",
            ));
        }
    }
}

fn require_rollbacks(surface: &SemanticRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_ROLLBACK_PATHS {
        if surface.rollback_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRollbackPath,
                format!("rollback:{id}"),
                "required semantic rollback path missing",
            ));
        }
    }
}

fn require_proofs(surface: &SemanticRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_REDTEAM_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRedTeamProof,
                format!("proof:{id}"),
                "required semantic redteam proof missing",
            ));
        }
    }
}

fn validate_scenarios(surface: &SemanticRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for scenario in &surface.scenarios {
        if !ALLOWED_SCENARIO_KINDS.contains(&scenario.scenario_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                format!("invalid scenario kind {}", scenario.scenario_kind),
            ));
        }
        if !scenario.path.starts_with("fixtures/p01/")
            && !scenario.path.starts_with("examples/p01/")
            && !scenario.path.starts_with("ops/p01/")
            && !scenario.path.starts_with("products/p01/")
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
            .any(|command| command == "lyra-p01-semantic-redteam-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRedTeamScenario,
                scenario.canonical_identity(),
                "scenarios must be checkable by lyra-p01-semantic-redteam-check",
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

fn validate_rollbacks(surface: &SemanticRedTeamSurface, errors: &mut Vec<ValidationError>) {
    for rollback in &surface.rollbacks {
        if !ALLOWED_ROLLBACK_KINDS.contains(&rollback.rollback_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                format!("invalid rollback kind {}", rollback.rollback_kind),
            ));
        }
        if !rollback.path.starts_with("examples/p01/")
            && !rollback.path.starts_with("ops/p01/")
            && !rollback.path.starts_with("products/p01/")
            && !rollback.path.starts_with("docs/p01/")
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
            .any(|command| command == "lyra-p01-semantic-redteam-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                "rollback paths must be checkable by lyra-p01-semantic-redteam-check",
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

fn validate_proofs(surface: &SemanticRedTeamSurface, errors: &mut Vec<ValidationError>) {
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
                "semantic redteam proofs must bind receipts",
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
                "semantic redteam proof must keep P01 phase open until closure gate",
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
                "semantic redteam proof must forbid unreceipted rollback",
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
                "semantic redteam proof must forbid remote truth rewrite",
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
                "semantic redteam proof must forbid challenge bypass and retaliatory rollback",
            ));
        }
    }
}

fn validate_coverage(surface: &SemanticRedTeamSurface, errors: &mut Vec<ValidationError>) {
    let mut covered = BTreeSet::new();
    for scenario in &surface.scenarios {
        for target in &scenario.targets {
            covered.insert(target.as_str());
        }
    }
    for anchor in REQUIRED_COVERAGE_ANCHORS {
        if !covered.contains(*anchor) {
            errors.push(ValidationError::reject(ErrorCode::InvalidRedTeamScenario, format!("coverage:{anchor}"), "semantic redteam scenarios must cover canonical symbols, semantic atoms, core IR, receipt replay, rollback, and redteam paths"));
        }
    }
}

fn validate_semantic_redteam_report(
    surface: &SemanticRedTeamSurface,
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
        String,
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
                scenario.status.clone(),
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
        String,
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
    let report = deterministic_semantic_redteam_rollback_report(
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
            "k0_semantic_redteam_report",
            "semantic redteam report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::RedTeamDriftAccepted,
            "k0_semantic_redteam_report",
            "semantic redteam report hash must be stable fnv1a128",
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
    for (needle, code) in FORBIDDEN_SEMANTIC_REDTEAM_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic redteam token {needle}"),
            ));
        }
    }
}
