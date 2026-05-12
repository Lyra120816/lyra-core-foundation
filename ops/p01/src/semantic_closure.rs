use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_closure::deterministic_semantic_closure_gate_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_closure_model::{
    SemanticClosureGateSurface, SemanticClosureOutputGate, SemanticClosureProof,
    SemanticClosureTaskBinding,
};

pub const P01_SEMANTIC_CLOSURE_CONTRACT: &str = "LYRA-P01-SEMANTIC-CLOSURE-GATE v1";

pub const REQUIRED_SEMANTIC_CLOSURE_RULES: &[&str] = &[
    "bounded_closure_requires_all_admitted_semantic_tasks",
    "global_closure_requires_all_p01_x_outputs",
    "semantic_closure_gate_must_be_receipted",
    "closure_claims_must_name_canonical_symbol_scope",
    "semantic_blockers_must_survive_global_denial",
    "control_plane_must_advance_to_p01_x_outputs",
    "no_global_closure_without_p01_x_outputs",
    "no_network_dependency",
    "no_docs_only_closure",
    "no_unreceipted_closure",
];

pub const REQUIRED_SEMANTIC_CLOSURE_TASKS: &[&str] = &[
    "P01-001", "P01-002", "P01-003", "P01-004", "P01-005", "P01-006", "P01-007", "P01-008",
    "P01-009", "P01-010", "P01-011", "P01-012", "P01-013", "P01-014", "P01-015", "P01-016",
    "P01-017", "P01-018", "P01-019", "P01-020", "P01-021", "P01-022", "P01-023", "P01-024",
];

pub const REQUIRED_SEMANTIC_CLOSURE_OUTPUTS: &[&str] =
    &["P01-X01", "P01-X02", "P01-X03", "P01-X04", "P01-X05"];

pub const REQUIRED_SEMANTIC_CLOSURE_PROOFS: &[&str] = &[
    "semantic_primary_task_receipt_chain",
    "semantic_negative_corpus_receipt_chain",
    "semantic_redteam_rollback_receipt_chain",
    "semantic_control_plane_transition_proof",
    "semantic_bounded_vs_global_closure_proof",
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
    "lyra-p01-semantic-ecosystem-check",
    "lyra-p01-semantic-economics-check",
    "lyra-p01-semantic-redteam-check",
    "lyra-p01-semantic-closure-check",
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
    "semantic_atoms",
    "core_ir",
    "semantic_objects",
    "semantic_identity",
    "reference_semantics",
    "symbolic_equality",
    "error_challenge_evidence",
    "serialization_hashing",
    "adversarial_corpus",
    "core_ir_reuse",
    "atom_reference",
    "bedrock_receipts",
    "formal_semantic_constitution",
    "canonical_data_model",
    "core_engine",
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

const FORBIDDEN_SEMANTIC_CLOSURE_TEXT: &[(&str, ErrorCode)] = &[
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
    ("p01 x outputs complete", ErrorCode::ClosureOutputPremature),
    (
        "closure outputs complete",
        ErrorCode::ClosureOutputPremature,
    ),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
];

pub fn parse_semantic_closure_surface(
    input: &str,
) -> Result<SemanticClosureGateSurface, Vec<ValidationError>> {
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
            "no semantic closure gate surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_CLOSURE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_CLOSURE_CONTRACT}"),
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
                    "semantic closure rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(task_id) = left.strip_prefix("task:") {
            if !is_semantic_closure_task_id(task_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic closure task identity {task_id}"),
                ));
                continue;
            }
            if !seen_tasks.insert(task_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureTask,
                    format!("task:{task_id}"),
                    "semantic closure task identity must be unique",
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
            if !is_semantic_closure_output_id(output_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic closure output identity {output_id}"),
                ));
                continue;
            }
            if !seen_outputs.insert(output_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("output:{output_id}"),
                    "semantic closure output gate identity must be unique",
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
                    format!("invalid semantic closure proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("proof:{proof_id}"),
                    "semantic closure proof identity must be unique",
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
                format!("unknown semantic closure gate key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(SemanticClosureGateSurface {
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

pub fn validate_semantic_closure_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_semantic_closure_surface(input) {
        Ok(surface) => errors.extend(validate_semantic_closure_model(&surface).errors),
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

pub fn validate_semantic_closure_model(surface: &SemanticClosureGateSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic closure gate must bind to P01",
        ));
    }
    if surface.task != "P01-024" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic closure gate must bind to P01-024",
        ));
    }
    if !ALLOWED_GATE_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported semantic closure status {}", surface.status),
        ));
    }
    if surface.bounded_closure != "true" {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "bounded_closure",
            "P01-024 must explicitly permit bounded primary-task closure",
        ));
    }
    if surface.global_closure != "false" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "global_closure",
            "P01 cannot claim global closure before P01-X outputs",
        ));
    }
    if surface.next_frontier != "P01-X01" {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "next_frontier",
            "semantic closure gate must advance to P01-X01",
        ));
    }
    require_rules(surface, &mut errors);
    require_tasks(surface, &mut errors);
    require_outputs(surface, &mut errors);
    require_proofs(surface, &mut errors);
    validate_tasks(surface, &mut errors);
    validate_outputs(surface, &mut errors);
    validate_proofs(surface, &mut errors);
    validate_semantic_closure_report(surface, &mut errors);
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
) -> Result<SemanticClosureTaskBinding, ValidationError> {
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
    Ok(SemanticClosureTaskBinding {
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
) -> Result<SemanticClosureOutputGate, ValidationError> {
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
    Ok(SemanticClosureOutputGate {
        line_number,
        id: id.to_string(),
        output_kind,
        path,
        depends,
        receipts,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticClosureProof, ValidationError> {
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
    Ok(SemanticClosureProof {
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

fn require_rules(surface: &SemanticClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_SEMANTIC_CLOSURE_RULES {
        match surface.rule_value(rule) {
            Some("required")
            | Some("forbidden")
            | Some("blocked_until_outputs")
            | Some("blocked_until_local_validation") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "required semantic closure rule missing",
            )),
        }
    }
}

fn require_tasks(surface: &SemanticClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_CLOSURE_TASKS {
        if surface.task_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureTask,
                format!("task:{id}"),
                "required semantic closure task binding missing",
            ));
        }
    }
}

fn require_outputs(surface: &SemanticClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_CLOSURE_OUTPUTS {
        if surface.output_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("output:{id}"),
                "required semantic closure output gate missing",
            ));
        }
    }
}

fn require_proofs(surface: &SemanticClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_CLOSURE_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("proof:{id}"),
                "required semantic closure proof missing",
            ));
        }
    }
}

fn validate_tasks(surface: &SemanticClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for task in &surface.tasks {
        if !REQUIRED_SEMANTIC_CLOSURE_TASKS.contains(&task.id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                format!("unknown P01 semantic closure task {}", task.id),
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
                format!("invalid semantic closure task status {}", task.status),
            ));
        }
        if task.receipts.is_empty() || task.commands.is_empty() || task.evidence.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                "semantic closure task bindings must include receipts, commands, and evidence",
            ));
        }
        if !task
            .commands
            .iter()
            .any(|command| command == "lyra-p01-semantic-closure-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                "semantic closure task must be checkable by lyra-p01-semantic-closure-check",
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
            if !receipt.starts_with("receipts/p01/pass_") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    task.canonical_identity(),
                    format!("task receipt must be P01 pass receipt: {receipt}"),
                ));
            }
        }
    }
}

fn validate_outputs(surface: &SemanticClosureGateSurface, errors: &mut Vec<ValidationError>) {
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
                format!(
                    "semantic closure output gate cannot be {} at P01-024",
                    output.status
                ),
            ));
        }
        if output.depends.is_empty() || output.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                output.canonical_identity(),
                "semantic closure output gates must bind dependencies and receipts",
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

fn validate_proofs(surface: &SemanticClosureGateSurface, errors: &mut Vec<ValidationError>) {
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
                "semantic closure proofs must bind receipts and commands",
            ));
        }
        if !proof
            .commands
            .iter()
            .any(|command| command == "lyra-p01-semantic-closure-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                proof.canonical_identity(),
                "semantic closure proofs must be checkable by lyra-p01-semantic-closure-check",
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
                "semantic closure proofs must permit bounded primary closure only",
            ));
        }
        if !proof.forbids.iter().any(|item| item == "global_closure") {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "semantic closure proofs must forbid global closure until P01-X outputs",
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
                "semantic closure proofs must forbid unreceipted closure",
            ));
        }
    }
}

fn validate_semantic_closure_report(
    surface: &SemanticClosureGateSurface,
    errors: &mut Vec<ValidationError>,
) {
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
    let proof_inputs: Vec<(
        String,
        String,
        Vec<String>,
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
                proof.tasks.clone(),
                proof.outputs.clone(),
                proof.receipts.clone(),
                proof.commands.clone(),
                proof.permits.clone(),
                proof.forbids.clone(),
                proof.status.clone(),
            )
        })
        .collect();
    let report =
        deterministic_semantic_closure_gate_report(&task_inputs, &output_inputs, &proof_inputs);
    if report.task_count != REQUIRED_SEMANTIC_CLOSURE_TASKS.len()
        || report.output_gate_count != REQUIRED_SEMANTIC_CLOSURE_OUTPUTS.len()
        || report.proof_count != REQUIRED_SEMANTIC_CLOSURE_PROOFS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_closure_report",
            "semantic closure gate report count mismatch",
        ));
    }
    if report.bounded_task_count != REQUIRED_SEMANTIC_CLOSURE_TASKS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "k0_semantic_closure_report",
            "every admitted P01 primary task must be bounded_closed",
        ));
    }
    if report.open_output_count != REQUIRED_SEMANTIC_CLOSURE_OUTPUTS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureOutputPremature,
            "k0_semantic_closure_report",
            "all P01-X outputs must remain explicitly open after P01-024",
        ));
    }
    if !report.gate_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_closure_report",
            "semantic closure gate report hash must be stable fnv1a128",
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

fn is_semantic_closure_task_id(value: &str) -> bool {
    value.len() == 7
        && value.starts_with("P01-")
        && value[4..].bytes().all(|byte| byte.is_ascii_digit())
}

fn is_semantic_closure_output_id(value: &str) -> bool {
    value.len() == 7
        && value.starts_with("P01-X")
        && value[5..].bytes().all(|byte| byte.is_ascii_digit())
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_CLOSURE_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic closure gate token {needle}"),
            ));
        }
    }
}
