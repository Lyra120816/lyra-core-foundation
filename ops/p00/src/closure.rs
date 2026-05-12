use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_closure::deterministic_closure_gate_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_closure_model::{
    ClosureGateSurface, ClosureOutputGate, ClosureProof, ClosureTaskBinding,
};

pub const P00_CLOSURE_GATE_CONTRACT: &str = "LYRA-P00-CLOSURE-GATE v1";

pub const REQUIRED_CLOSURE_RULES: &[&str] = &[
    "bounded_closure_requires_all_admitted_tasks",
    "global_closure_requires_all_phase_outputs",
    "closure_gate_must_be_receipted",
    "closure_claims_must_name_scope",
    "blockers_must_survive_global_denial",
    "control_plane_must_advance_to_closure_outputs",
    "no_global_closure_without_x_outputs",
    "no_network_dependency",
    "no_docs_only_closure",
    "no_unreceipted_closure",
];

pub const REQUIRED_CLOSURE_TASKS: &[&str] = &[
    "P00-001", "P00-002", "P00-003", "P00-004", "P00-005", "P00-006", "P00-007", "P00-008",
    "P00-009", "P00-010", "P00-011", "P00-012", "P00-013", "P00-014", "P00-015", "P00-016",
    "P00-017", "P00-018", "P00-019", "P00-020", "P00-021", "P00-022", "P00-023", "P00-024",
];

pub const REQUIRED_CLOSURE_OUTPUTS: &[&str] =
    &["P00-X01", "P00-X02", "P00-X03", "P00-X04", "P00-X05"];

pub const REQUIRED_CLOSURE_PROOFS: &[&str] = &[
    "primary_task_receipt_chain",
    "negative_corpus_receipt_chain",
    "rollback_redteam_receipt_chain",
    "control_plane_transition_proof",
    "bounded_vs_global_closure_proof",
];

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
    "lyra-p00-closure-check",
];

const ALLOWED_GATE_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_TASK_STATUSES: &[&str] = &["bounded_closed", "execution_proven", "artifact_emitted"];
const ALLOWED_OUTPUT_STATUSES: &[&str] = &["blocked", "working_slice", "artifact_emitted"];
const ALLOWED_PROOF_STATUSES: &[&str] = &["working_slice", "artifact_emitted", "execution_proven"];
const ALLOWED_TASK_SCOPES: &[&str] = &[
    "constitution",
    "authority",
    "identity",
    "enforcement",
    "delivery",
    "challenge",
    "control",
    "owner_root",
    "benchmark",
    "public_interest",
    "canon",
    "acceptance",
    "formal_semantics",
    "canonical_model",
    "engine",
    "falsification",
    "replay",
    "interface",
    "packaging",
    "deployment",
    "ecosystem",
    "economics",
    "redteam",
    "closure",
];
const ALLOWED_OUTPUT_KINDS: &[&str] = &[
    "dependency_matrix",
    "proof_family_table",
    "benchmark_pack",
    "output_table",
    "retirement_law",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &[
    "bounded",
    "global_denial",
    "control_plane",
    "receipt_chain",
    "closure_outputs",
];

const FORBIDDEN_CLOSURE_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::ClosureNetworkDependency),
    ("cloud required", ErrorCode::ClosureNetworkDependency),
    ("online required", ErrorCode::ClosureNetworkDependency),
    (
        "remote service required",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("remote fetch", ErrorCode::ClosureNetworkDependency),
    ("unreceipted closure allowed", ErrorCode::ClosureUnreceipted),
    ("closure without receipt", ErrorCode::ClosureUnreceipted),
    ("closure drift accepted", ErrorCode::ClosureDriftAccepted),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("phase closure true", ErrorCode::UnsupportedGlobalClosure),
    ("x outputs complete", ErrorCode::ClosureOutputPremature),
    (
        "closure outputs complete",
        ErrorCode::ClosureOutputPremature,
    ),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_closure_gate_surface(input: &str) -> Result<ClosureGateSurface, Vec<ValidationError>> {
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
            "no closure gate surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_CLOSURE_GATE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_CLOSURE_GATE_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut bounded_closure = None;
    let mut global_closure = None;
    let mut next_frontier = None;
    let mut rules = BTreeMap::new();
    let mut tasks = Vec::new();
    let mut outputs = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_tasks = BTreeSet::new();
    let mut seen_outputs = BTreeSet::new();
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
                    "closure rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(task_id) = left.strip_prefix("task:") {
            if !is_closure_task_id(task_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    format!("line:{line_number:03}"),
                    format!("invalid closure task identity {task_id}"),
                ));
                continue;
            }
            if !seen_tasks.insert(task_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureTask,
                    format!("task:{task_id}"),
                    "closure task identity must be unique",
                ));
                continue;
            }
            match parse_task(line_number, task_id, value) {
                Ok(task) => tasks.push(task),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(output_id) = left.strip_prefix("output:") {
            if !is_closure_output_id(output_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid closure output identity {output_id}"),
                ));
                continue;
            }
            if !seen_outputs.insert(output_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("output:{output_id}"),
                    "closure output gate identity must be unique",
                ));
                continue;
            }
            match parse_output(line_number, output_id, value) {
                Ok(output) => outputs.push(output),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid closure proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("proof:{proof_id}"),
                    "closure proof identity must be unique",
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
            "bounded_closure" => bounded_closure = Some(value.to_string()),
            "global_closure" => global_closure = Some(value.to_string()),
            "next_frontier" => next_frontier = Some(value.to_string()),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown closure gate key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(ClosureGateSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        bounded_closure: bounded_closure.unwrap_or_default(),
        global_closure: global_closure.unwrap_or_default(),
        next_frontier: next_frontier.unwrap_or_default(),
        rules,
        tasks,
        outputs,
        proofs,
    })
}

pub fn validate_closure_gate_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_closure_gate_surface(input) {
        Ok(surface) => errors.extend(validate_closure_gate_model(&surface).errors),
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

pub fn validate_closure_gate_model(surface: &ClosureGateSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "closure gate must bind to P00",
        ));
    }
    if surface.task != "P00-024" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "closure gate must bind to P00-024",
        ));
    }
    if !ALLOWED_GATE_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported closure gate status {}", surface.status),
        ));
    }
    if surface.bounded_closure != "true" {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "bounded_closure",
            "P00-024 must explicitly permit bounded primary-task closure",
        ));
    }
    if surface.global_closure != "false" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "global_closure",
            "P00 cannot claim global closure before P00-X closure outputs",
        ));
    }
    if surface.next_frontier != "P00-X01" {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "next_frontier",
            "closure gate must advance to P00-X01",
        ));
    }
    require_rules(surface, &mut errors);
    require_tasks(surface, &mut errors);
    require_outputs(surface, &mut errors);
    require_proofs(surface, &mut errors);
    validate_tasks(surface, &mut errors);
    validate_outputs(surface, &mut errors);
    validate_proofs(surface, &mut errors);
    validate_closure_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_task(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ClosureTaskBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureTask,
            format!("line:{line_number:03}"),
            "task fields must be key:value segments",
        )
    })?;
    let scope = required_field(&fields, "scope", ErrorCode::InvalidClosureTask, line_number)?;
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?);
    let evidence = split_csv(&required_field(
        &fields,
        "evidence",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?;
    Ok(ClosureTaskBinding {
        line_number,
        id: id.to_string(),
        scope,
        receipts,
        commands,
        evidence,
        status,
    })
}

fn parse_output(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ClosureOutputGate, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "output fields must be key:value segments",
        )
    })?;
    let output_kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?;
    let path = required_field(
        &fields,
        "path",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?;
    let depends = split_csv(&required_field(
        &fields,
        "depends",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?;
    Ok(ClosureOutputGate {
        line_number,
        id: id.to_string(),
        output_kind,
        path,
        depends,
        receipts,
        status,
    })
}

fn parse_proof(line_number: usize, id: &str, value: &str) -> Result<ClosureProof, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "proof fields must be key:value segments",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidClosureProof,
        line_number,
    )?;
    let tasks = split_csv(&required_field(
        &fields,
        "tasks",
        ErrorCode::InvalidClosureProof,
        line_number,
    )?);
    let outputs = split_csv(&required_field(
        &fields,
        "outputs",
        ErrorCode::InvalidClosureProof,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidClosureProof,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidClosureProof,
        line_number,
    )?);
    let permits = split_csv(&required_field(
        &fields,
        "permits",
        ErrorCode::InvalidClosureProof,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidClosureProof,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidClosureProof,
        line_number,
    )?;
    Ok(ClosureProof {
        line_number,
        id: id.to_string(),
        scope,
        tasks,
        outputs,
        receipts,
        commands,
        permits,
        forbids,
        status,
    })
}

fn require_rules(surface: &ClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_CLOSURE_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") | Some("blocked_until_outputs") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "required closure rule missing",
            )),
        }
    }
}

fn require_tasks(surface: &ClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_CLOSURE_TASKS {
        if surface.task_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureTask,
                format!("task:{id}"),
                "required closure task binding missing",
            ));
        }
    }
}

fn require_outputs(surface: &ClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_CLOSURE_OUTPUTS {
        if surface.output_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("output:{id}"),
                "required closure output gate missing",
            ));
        }
    }
}

fn require_proofs(surface: &ClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_CLOSURE_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("proof:{id}"),
                "required closure proof missing",
            ));
        }
    }
}

fn validate_tasks(surface: &ClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for task in &surface.tasks {
        if !REQUIRED_CLOSURE_TASKS.contains(&task.id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                format!("unknown P00 closure task {}", task.id),
            ));
        }
        if !ALLOWED_TASK_SCOPES.contains(&task.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                format!("invalid task scope {}", task.scope),
            ));
        }
        if !ALLOWED_TASK_STATUSES.contains(&task.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                format!("invalid task closure status {}", task.status),
            ));
        }
        if task.receipts.is_empty() || task.commands.is_empty() || task.evidence.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                "closure task bindings must include receipts, commands, and evidence",
            ));
        }
        if !task
            .commands
            .iter()
            .any(|command| command == "lyra-p00-closure-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                "closure task must be checkable by lyra-p00-closure-check",
            ));
        }
        for command in &task.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    task.canonical_identity(),
                    format!("unknown task command {command}"),
                ));
            }
        }
        for receipt in &task.receipts {
            if !receipt.starts_with("receipts/p00/pass_") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    task.canonical_identity(),
                    format!("task receipt must be P00 pass receipt: {receipt}"),
                ));
            }
        }
    }
}

fn validate_outputs(surface: &ClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for output in &surface.outputs {
        if !ALLOWED_OUTPUT_KINDS.contains(&output.output_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                output.canonical_identity(),
                format!("invalid output kind {}", output.output_kind),
            ));
        }
        if !output.path.starts_with("ops/")
            && !output.path.starts_with("docs/")
            && !output.path.starts_with("products/")
            && !output.path.starts_with("interfaces/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                output.canonical_identity(),
                format!("invalid output path {}", output.path),
            ));
        }
        if !ALLOWED_OUTPUT_STATUSES.contains(&output.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureOutputPremature,
                output.canonical_identity(),
                format!("closure output gate cannot be {} at P00-024", output.status),
            ));
        }
        if output.depends.is_empty() || output.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                output.canonical_identity(),
                "closure output gates must bind dependencies and receipts",
            ));
        }
        for dependency in &output.depends {
            if surface.task_by_id(dependency).is_none()
                && surface.output_by_id(dependency).is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    output.canonical_identity(),
                    format!("unknown output dependency {dependency}"),
                ));
            }
        }
    }
}

fn validate_proofs(surface: &ClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                proof.canonical_identity(),
                format!("invalid proof scope {}", proof.scope),
            ));
        }
        if !ALLOWED_PROOF_STATUSES.contains(&proof.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                proof.canonical_identity(),
                format!("invalid proof status {}", proof.status),
            ));
        }
        if proof.receipts.is_empty() || proof.commands.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                proof.canonical_identity(),
                "closure proofs must bind receipts and commands",
            ));
        }
        if !proof
            .commands
            .iter()
            .any(|command| command == "lyra-p00-closure-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                proof.canonical_identity(),
                "closure proofs must be checkable by lyra-p00-closure-check",
            ));
        }
        for task in &proof.tasks {
            if surface.task_by_id(task).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof task {task}"),
                ));
            }
        }
        for output in &proof.outputs {
            if surface.output_by_id(output).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof output {output}"),
                ));
            }
        }
        for command in &proof.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof command {command}"),
                ));
            }
        }
        if !proof
            .permits
            .iter()
            .any(|item| item == "bounded_primary_closure")
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureFormulaViolation,
                proof.canonical_identity(),
                "closure proofs must permit bounded primary closure only",
            ));
        }
        if !proof.forbids.iter().any(|item| item == "global_closure") {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "closure proofs must forbid global closure until P00-X outputs",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|item| item == "unreceipted_closure")
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureUnreceipted,
                proof.canonical_identity(),
                "closure proofs must forbid unreceipted closure",
            ));
        }
    }
}

fn validate_closure_report(surface: &ClosureGateSurface, errors: &mut Vec<ValidationError>) {
    let task_inputs: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .tasks
        .iter()
        .map(|task| {
            (
                task.id.clone(),
                task.scope.clone(),
                task.receipts.clone(),
                task.commands.clone(),
                task.evidence.clone(),
                task.status.clone(),
            )
        })
        .collect();
    let output_inputs: Vec<(String, String, String, Vec<String>, Vec<String>, String)> = surface
        .outputs
        .iter()
        .map(|output| {
            (
                output.id.clone(),
                output.output_kind.clone(),
                output.path.clone(),
                output.depends.clone(),
                output.receipts.clone(),
                output.status.clone(),
            )
        })
        .collect();
    let report =
        deterministic_closure_gate_report(&task_inputs, &output_inputs, surface.proofs.len());
    if report.task_count != REQUIRED_CLOSURE_TASKS.len()
        || report.output_gate_count != REQUIRED_CLOSURE_OUTPUTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_closure_report",
            "closure gate report count mismatch",
        ));
    }
    if report.bounded_task_count != REQUIRED_CLOSURE_TASKS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "k0_closure_report",
            "every admitted P00 primary task must be bounded_closed",
        ));
    }
    if report.open_output_count != REQUIRED_CLOSURE_OUTPUTS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureOutputPremature,
            "k0_closure_report",
            "all P00-X outputs must remain explicitly open after P00-024",
        ));
    }
    if !report.gate_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_closure_report",
            "closure gate report hash must be stable fnv1a128",
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

fn is_closure_task_id(value: &str) -> bool {
    value.len() == 7
        && value.starts_with("P00-")
        && value[4..].bytes().all(|byte| byte.is_ascii_digit())
}

fn is_closure_output_id(value: &str) -> bool {
    value.len() == 7
        && value.starts_with("P00-X")
        && value[5..].bytes().all(|byte| byte.is_ascii_digit())
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_CLOSURE_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden closure gate token {needle}"),
            ));
        }
    }
}
