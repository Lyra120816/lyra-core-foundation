use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_engine::deterministic_engine_trace;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_engine_model::{
    DeterministicEngineSurface, DeterministicEngineUnit, EngineExecutionProof,
    EngineTransitionBinding,
};

pub const P00_ENGINE_CONTRACT: &str = "LYRA-P00-DETERMINISTIC-ENGINE v1";

pub const REQUIRED_ENGINE_RULES: &[&str] = &[
    "deterministic_engine_units_required",
    "canonical_input_required",
    "canonical_output_required",
    "explicit_scheduler_required",
    "receipt_emission_required",
    "replay_witness_required",
    "no_ambient_time",
    "no_hidden_randomness",
    "no_network_dependency",
    "phase_open_until_engine_proven",
];

pub const REQUIRED_ENGINE_UNITS: &[&str] = &[
    "canonicalizer_core",
    "hash_core",
    "receipt_core",
    "verdict_core",
    "surface_parser_core",
    "validator_core",
    "control_surface_core",
    "evidence_receipt_core",
];

pub const REQUIRED_ENGINE_TRANSITIONS: &[&str] = &[
    "canonicalize_to_hash",
    "hash_to_receipt",
    "parse_to_validate",
    "validate_to_verdict",
    "verdict_to_receipt",
    "control_to_receipt",
];

pub const REQUIRED_ENGINE_PROOFS: &[&str] = &[
    "engine_local_execution",
    "deterministic_replay_witness",
    "unsafe_dependency_block",
    "p00_phase_open",
];

const ALLOWED_OWNER_ROOTS: &[&str] = &["k0", "interfaces", "ops"];
const ALLOWED_ENGINE_STATES: &[&str] = &["stateless", "explicit_state", "receipt_state"];
const ALLOWED_ENGINE_STATUSES: &[&str] = &[
    "working_slice",
    "execution_proven",
    "artifact_emitted",
    "blocked",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["task", "replay", "safety", "phase"];

const FORBIDDEN_ENGINE_TEXT: &[(&str, ErrorCode)] = &[
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("wall clock", ErrorCode::AmbientTimeAllowed),
    ("system time", ErrorCode::AmbientTimeAllowed),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("network fetch", ErrorCode::AmbientNetworkAllowed),
    ("cloud", ErrorCode::AmbientNetworkAllowed),
    ("unordered map", ErrorCode::EngineDriftAccepted),
    ("host order", ErrorCode::EngineDriftAccepted),
    ("thread race", ErrorCode::EngineDriftAccepted),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("engine placeholder", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_engine_surface(
    input: &str,
) -> Result<DeterministicEngineSurface, Vec<ValidationError>> {
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
            "no deterministic-engine lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_ENGINE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_ENGINE_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut engines = Vec::new();
    let mut transitions = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_engines = BTreeSet::new();
    let mut seen_transitions = BTreeSet::new();
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
                    "engine rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(engine_id) = left.strip_prefix("engine:") {
            if !is_symbolic_name(engine_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEngineUnit,
                    format!("line:{line_number:03}"),
                    format!("invalid engine identity {engine_id}"),
                ));
                continue;
            }
            if !seen_engines.insert(engine_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEngineUnit,
                    format!("engine:{engine_id}"),
                    "engine unit identity must be unique",
                ));
                continue;
            }
            match parse_engine_unit(line_number, engine_id, value) {
                Ok(engine) => engines.push(engine),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(transition_id) = left.strip_prefix("transition:") {
            if !is_symbolic_name(transition_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidTransitionBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid transition identity {transition_id}"),
                ));
                continue;
            }
            if !seen_transitions.insert(transition_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateTransitionBinding,
                    format!("transition:{transition_id}"),
                    "transition identity must be unique",
                ));
                continue;
            }
            match parse_transition(line_number, transition_id, value) {
                Ok(transition) => transitions.push(transition),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEngineProof,
                    format!("line:{line_number:03}"),
                    format!("invalid engine proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEngineProof,
                    format!("proof:{proof_id}"),
                    "engine proof identity must be unique",
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
                format!("unknown entry {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(DeterministicEngineSurface {
        header,
        phase: phase.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingPhase,
                "surface",
                "missing phase",
            )]
        })?,
        task: task.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingTask,
                "surface",
                "missing task",
            )]
        })?,
        status: status.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::InvalidEngineSurface,
                "surface",
                "missing status",
            )]
        })?,
        rules,
        engines,
        transitions,
        proofs,
    })
}

pub fn validate_engine_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    for (token, code) in FORBIDDEN_ENGINE_TEXT {
        if input.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "surface",
                format!("forbidden deterministic-engine token {token}"),
            ));
        }
    }

    match parse_engine_surface(input) {
        Ok(surface) => errors.extend(validate_engine_surface_model(&surface).errors),
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

pub fn validate_engine_surface_model(surface: &DeterministicEngineSurface) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P00 got {}", surface.phase),
        ));
    }
    if surface.task != "P00-015" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P00-015 got {}", surface.task),
        ));
    }
    if !matches!(
        surface.status.as_str(),
        "working_slice" | "artifact_emitted"
    ) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineSurface,
            "status",
            format!("unsupported engine status {}", surface.status),
        ));
    }

    for required in REQUIRED_ENGINE_RULES {
        match surface.rule_value(required) {
            Some(value) if strong_required_value(value) => {}
            Some(_) => errors.push(ValidationError::reject(
                ErrorCode::MissingEngineRule,
                format!("rule:{required}"),
                "engine rule must be explicit and enforced",
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingEngineRule,
                format!("rule:{required}"),
                "missing required deterministic engine rule",
            )),
        }
    }

    let engine_ids: BTreeSet<String> = surface.engines.iter().map(|item| item.id.clone()).collect();
    let transition_ids: BTreeSet<String> = surface
        .transitions
        .iter()
        .map(|item| item.id.clone())
        .collect();

    for required in REQUIRED_ENGINE_UNITS {
        if !engine_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEngineUnit,
                format!("engine:{required}"),
                "missing required deterministic engine unit",
            ));
        }
    }
    for required in REQUIRED_ENGINE_TRANSITIONS {
        if !transition_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingTransitionBinding,
                format!("transition:{required}"),
                "missing required engine transition binding",
            ));
        }
    }
    for required in REQUIRED_ENGINE_PROOFS {
        if surface.proof_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEngineProof,
                format!("proof:{required}"),
                "missing required deterministic engine proof",
            ));
        }
    }

    let mut order_by_root = BTreeMap::new();
    for engine in &surface.engines {
        validate_engine_unit(engine, &mut order_by_root, &mut errors);
    }
    for transition in &surface.transitions {
        validate_transition(transition, &engine_ids, &mut errors);
    }
    for proof in &surface.proofs {
        validate_proof(proof, &engine_ids, &transition_ids, &mut errors);
    }

    if let Ok(trace) = deterministic_engine_trace("P00-015", P00_ENGINE_CONTRACT) {
        if trace.steps.len() != 3 || trace.trace_hash.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEngineSurface,
                "engine_trace",
                "k0 deterministic trace must emit three stable steps and trace hash",
            ));
        }
    } else {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineSurface,
            "engine_trace",
            "k0 deterministic trace must canonicalize admitted input",
        ));
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_engine_unit(
    engine: &DeterministicEngineUnit,
    order_by_root: &mut BTreeMap<String, BTreeSet<String>>,
    errors: &mut Vec<ValidationError>,
) {
    let location = engine.canonical_identity();
    if !ALLOWED_OWNER_ROOTS.contains(&engine.owner_root.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineUnit,
            location.clone(),
            format!(
                "engine owner root must be k0/interfaces/ops: {}",
                engine.owner_root
            ),
        ));
    }
    if !engine
        .module
        .starts_with(&format!("{}/", engine.owner_root))
        || !engine.module.ends_with(".rs")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineUnit,
            location.clone(),
            format!(
                "engine module must live under owner root and be Rust source: {}",
                engine.module
            ),
        ));
    }
    if engine.inputs.is_empty()
        || engine
            .inputs
            .iter()
            .any(|item| weak_value(item) || !is_symbolic_name(item))
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineUnit,
            location.clone(),
            "engine inputs must be concrete symbolic names",
        ));
    }
    if engine.outputs.is_empty()
        || engine
            .outputs
            .iter()
            .any(|item| weak_value(item) || !is_symbolic_name(item))
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineUnit,
            location.clone(),
            "engine outputs must be concrete symbolic names",
        ));
    }
    if !ALLOWED_ENGINE_STATES.contains(&engine.state.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineUnit,
            location.clone(),
            format!("unsupported engine state {}", engine.state),
        ));
    }
    if !stable_order_token(&engine.order) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineUnit,
            location.clone(),
            format!(
                "engine order must be stable three-digit token: {}",
                engine.order
            ),
        ));
    } else {
        let seen = order_by_root.entry(engine.owner_root.clone()).or_default();
        if !seen.insert(engine.order.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEngineUnit,
                location.clone(),
                format!(
                    "duplicate engine order {} in owner root {}",
                    engine.order, engine.owner_root
                ),
            ));
        }
    }
    if engine.receipts.is_empty() || engine.receipts.iter().any(|receipt| !receipt_path(receipt)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "engine unit must bind canonical P00 receipt paths",
        ));
    }
    if !ALLOWED_ENGINE_STATUSES.contains(&engine.status.as_str()) || engine.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineUnit,
            location,
            format!("unsupported engine unit status {}", engine.status),
        ));
    }
}

fn validate_transition(
    transition: &EngineTransitionBinding,
    engine_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = transition.canonical_identity();
    if !engine_ids.contains(&transition.from) {
        errors.push(ValidationError::reject(
            ErrorCode::EngineProofUnbound,
            location.clone(),
            format!(
                "transition references unknown from engine {}",
                transition.from
            ),
        ));
    }
    if !engine_ids.contains(&transition.to) {
        errors.push(ValidationError::reject(
            ErrorCode::EngineProofUnbound,
            location.clone(),
            format!("transition references unknown to engine {}", transition.to),
        ));
    }
    if transition.from == transition.to {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTransitionBinding,
            location.clone(),
            "engine transition must move between distinct units",
        ));
    }
    if weak_value(&transition.trigger)
        || weak_value(&transition.guard)
        || weak_value(&transition.effect)
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTransitionBinding,
            location.clone(),
            "trigger, guard, and effect must be concrete",
        ));
    }
    if transition.receipts.is_empty()
        || transition
            .receipts
            .iter()
            .any(|receipt| !receipt_path(receipt))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "engine transition must bind canonical P00 receipts",
        ));
    }
    if transition.commands.is_empty()
        || transition
            .commands
            .iter()
            .any(|command| weak_value(command))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "engine transition must bind command records",
        ));
    }
    if !ALLOWED_ENGINE_STATUSES.contains(&transition.status.as_str())
        || transition.status == "blocked"
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTransitionBinding,
            location,
            format!("unsupported transition status {}", transition.status),
        ));
    }
}

fn validate_proof(
    proof: &EngineExecutionProof,
    engine_ids: &BTreeSet<String>,
    transition_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = proof.canonical_identity();
    if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineProof,
            location.clone(),
            format!("unsupported engine proof scope {}", proof.scope),
        ));
    }
    if !ALLOWED_ENGINE_STATUSES.contains(&proof.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineProof,
            location.clone(),
            format!("unsupported engine proof status {}", proof.status),
        ));
    }
    if proof.scope == "phase" && proof.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "phase engine proof must remain blocked until all P00 tasks close",
        ));
    }
    for engine in &proof.engines {
        if !engine_ids.contains(engine) {
            errors.push(ValidationError::reject(
                ErrorCode::EngineProofUnbound,
                location.clone(),
                format!("unknown proof engine {engine}"),
            ));
        }
    }
    for transition in &proof.transitions {
        if !transition_ids.contains(transition) {
            errors.push(ValidationError::reject(
                ErrorCode::EngineProofUnbound,
                location.clone(),
                format!("unknown proof transition {transition}"),
            ));
        }
    }
    if proof.receipts.is_empty() || proof.receipts.iter().any(|receipt| !receipt_path(receipt)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "engine proof must bind canonical P00 receipts",
        ));
    }
    if proof.commands.is_empty() || proof.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "engine proof must bind command records",
        ));
    }
    if proof.forbids.is_empty() || proof.forbids.iter().any(|item| weak_value(item)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineProof,
            location.clone(),
            "engine proof forbid list must be concrete",
        ));
    }
    if proof.id == "deterministic_replay_witness" {
        for required in ["canonicalizer_core", "hash_core", "receipt_core"] {
            if !proof.engines.iter().any(|engine| engine == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingEngineUnit,
                    location.clone(),
                    format!("replay witness misses engine {required}"),
                ));
            }
        }
    }
    if proof.id == "unsafe_dependency_block" {
        for required in [
            "ambient_time",
            "hidden_randomness",
            "ambient_network",
            "probabilistic_truth",
        ] {
            if !proof.forbids.iter().any(|item| item == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEngineProof,
                    location.clone(),
                    format!("unsafe dependency block misses forbid token {required}"),
                ));
            }
        }
    }
}

fn parse_engine_unit(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<DeterministicEngineUnit, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let unit = DeterministicEngineUnit {
        line_number,
        id: id.to_string(),
        owner_root: required_string_field(line_number, &mut fields, "owner_root")?,
        module: required_string_field(line_number, &mut fields, "module")?,
        inputs: required_list_field(line_number, &mut fields, "inputs")?,
        outputs: required_list_field(line_number, &mut fields, "outputs")?,
        state: required_string_field(line_number, &mut fields, "state")?,
        order: required_string_field(line_number, &mut fields, "order")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(unit)
}

fn parse_transition(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<EngineTransitionBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let transition = EngineTransitionBinding {
        line_number,
        id: id.to_string(),
        from: required_string_field(line_number, &mut fields, "from")?,
        to: required_string_field(line_number, &mut fields, "to")?,
        trigger: required_string_field(line_number, &mut fields, "trigger")?,
        guard: required_string_field(line_number, &mut fields, "guard")?,
        effect: required_string_field(line_number, &mut fields, "effect")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        commands: required_list_field(line_number, &mut fields, "commands")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(transition)
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<EngineExecutionProof, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let proof = EngineExecutionProof {
        line_number,
        id: id.to_string(),
        scope: required_string_field(line_number, &mut fields, "scope")?,
        engines: required_list_field(line_number, &mut fields, "engines")?,
        transitions: required_list_field(line_number, &mut fields, "transitions")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        commands: required_list_field(line_number, &mut fields, "commands")?,
        forbids: required_list_field(line_number, &mut fields, "forbids")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(proof)
}

fn parse_fields(
    line_number: usize,
    value: &str,
) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        let Some((key, val)) = part.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field must use key:value syntax",
            ));
        };
        if key.is_empty() || val.is_empty() || key != key.trim() || val != val.trim() {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field key/value must be non-empty and trimmed",
            ));
        }
        if fields.insert(key.to_string(), val.to_string()).is_some() {
            return Err(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate field {key}"),
            ));
        }
    }
    Ok(fields)
}

fn required_string_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<String, ValidationError> {
    fields.remove(key).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("missing field {key}"),
        )
    })
}

fn required_list_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<Vec<String>, ValidationError> {
    let value = required_string_field(line_number, fields, key)?;
    let values = split_list(&value);
    if values.is_empty() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("field {key} must contain at least one item"),
        ))
    } else {
        Ok(values)
    }
}

fn reject_unknown_fields(
    line_number: usize,
    fields: BTreeMap<String, String>,
) -> Result<(), ValidationError> {
    if let Some(key) = fields.keys().next() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("unknown field {key}"),
        ))
    } else {
        Ok(())
    }
}

fn split_list(value: &str) -> Vec<String> {
    let mut items: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty() && *item != "none" && *item != "nothing")
        .map(ToString::to_string)
        .collect();
    items.sort();
    items.dedup();
    items
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value == value.trim()
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        && value.as_bytes()[0].is_ascii_lowercase()
}

fn stable_order_token(value: &str) -> bool {
    value.len() == 3 && value.as_bytes().iter().all(|byte| byte.is_ascii_digit()) && value != "000"
}

fn receipt_path(value: &str) -> bool {
    value.starts_with("receipts/p00/") && value.ends_with(".receipt")
}

fn strong_required_value(value: &str) -> bool {
    matches!(
        value,
        "required" | "enforced" | "forbidden" | "blocked" | "receipt_bound" | "replay_required"
    )
}

fn weak_value(value: &str) -> bool {
    matches!(
        value,
        "none"
            | "nothing"
            | "declared_only"
            | "manual_only"
            | "human_only"
            | "unbound"
            | "empty"
            | "future"
            | "later"
            | "best_effort"
            | "placeholder"
            | "todo"
    )
}
