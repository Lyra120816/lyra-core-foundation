use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_closure::deterministic_bootstrap_closure_gate_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_closure::{
    bootstrap_closure_artifacts_bind_paths, bootstrap_closure_carrier_signature,
    bootstrap_closure_no_forbidden_descriptor_claims, bootstrap_closure_output_descriptor,
    bootstrap_closure_output_digest, bootstrap_closure_outputs_remain_open,
    bootstrap_closure_proof_descriptor, bootstrap_closure_proof_digest,
    bootstrap_closure_proofs_bind_registry,
    bootstrap_closure_receipts_cover_p02_001_through_p02_024, bootstrap_closure_registry_hash,
    bootstrap_closure_task_descriptor, bootstrap_closure_task_digest,
    bootstrap_closure_tasks_bind_receipts, LYRA_P02_BOOTSTRAP_CLOSURE_CARRIER,
};
use crate::p02_bootstrap_closure_model::{
    BootstrapClosureGateSurface, BootstrapClosureOutputGate, BootstrapClosureProof,
    BootstrapClosureTaskBinding,
};

pub const P02_BOOTSTRAP_CLOSURE_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-CLOSURE-GATE v1";

pub const REQUIRED_BOOTSTRAP_CLOSURE_RULES: &[&str] = &[
    "bounded_primary_closure_requires_all_p02_tasks",
    "global_closure_must_remain_denied_until_x_outputs",
    "closure_gate_must_be_receipted",
    "closure_claims_must_name_scope",
    "open_output_gates_must_name_next_frontier",
    "bootstrap_trust_closure_must_bind_redteam",
    "seed_runtime_closure_must_bind_replacement_law",
    "host_extinction_closure_must_bind_extinction_ledger",
    "no_network_required_for_closure_gate",
    "no_unreceipted_closure_gate",
    "no_docs_only_closure_gate",
    "phase_outputs_remain_open_after_primary_gate",
];
pub const REQUIRED_BOOTSTRAP_CLOSURE_TASKS: &[&str] = &[
    "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
    "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
    "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
];
pub const REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS: &[&str] =
    &["P02-X01", "P02-X02", "P02-X03", "P02-X04", "P02-X05"];
pub const REQUIRED_BOOTSTRAP_CLOSURE_PROOFS: &[&str] = &[
    "bootstrap_primary_task_receipt_chain",
    "bootstrap_negative_corpus_receipt_chain",
    "bootstrap_redteam_rollback_receipt_chain",
    "bootstrap_control_plane_transition_proof",
    "bootstrap_bounded_vs_global_closure_proof",
    "bootstrap_output_gate_open_proof",
];

const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p02-bootstrap-inventory-check",
    "lyra-p02-bootstrap-extinction-check",
    "lyra-p02-seed-runtime-replacement-check",
    "lyra-p02-bootstrap-interface-check",
    "lyra-p02-host-boundary-check",
    "lyra-p02-target-matrix-check",
    "lyra-p02-truth-cleanup-check",
    "lyra-p02-emergency-fallback-check",
    "lyra-p02-bootstrap-evidence-emission-check",
    "lyra-p02-operator-handoff-automation-check",
    "lyra-p02-foreign-surface-closure-check",
    "lyra-p02-bootstrap-formal-semantics-check",
    "lyra-p02-bootstrap-canonical-model-check",
    "lyra-p02-bootstrap-core-engine-check",
    "lyra-p02-bootstrap-falsification-check",
    "lyra-p02-bootstrap-replay-check",
    "lyra-p02-bootstrap-packaging-check",
    "lyra-p02-bootstrap-deployment-check",
    "lyra-p02-bootstrap-ecosystem-check",
    "lyra-p02-bootstrap-economics-check",
    "lyra-p02-bootstrap-redteam-check",
    "lyra-p02-bootstrap-closure-check",
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
    "inventory",
    "extinction_ledger",
    "seed_runtime_contracts",
    "session_rituals",
    "host_boundary_challenge",
    "target_matrix",
    "truth_cleanup",
    "emergency_fallback",
    "seed_runtime_replacement",
    "evidence_emission",
    "operator_handoff",
    "foreign_surface_closure",
    "formal_semantics",
    "canonical_model",
    "core_engine",
    "falsification",
    "replay",
    "operator_interface",
    "packaging",
    "deployment",
    "ecosystem",
    "economics",
    "redteam",
    "closure_gate",
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

const FORBIDDEN_BOOTSTRAP_CLOSURE_TEXT: &[(&str, ErrorCode)] = &[
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
    ("p02 x outputs complete", ErrorCode::ClosureOutputPremature),
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

pub fn parse_bootstrap_closure_surface(
    input: &str,
) -> Result<BootstrapClosureGateSurface, Vec<ValidationError>> {
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
            "no bootstrap closure gate surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_CLOSURE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_CLOSURE_CONTRACT}"),
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
                    "bootstrap closure rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(task_id) = left.strip_prefix("task:") {
            if !is_bootstrap_closure_task_id(task_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    format!("line:{line_number:03}"),
                    format!("invalid bootstrap closure task identity {task_id}"),
                ));
                continue;
            }
            if !seen_tasks.insert(task_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureTask,
                    format!("task:{task_id}"),
                    "bootstrap closure task identity must be unique",
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
            if !is_bootstrap_closure_output_id(output_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid bootstrap closure output identity {output_id}"),
                ));
                continue;
            }
            if !seen_outputs.insert(output_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("output:{output_id}"),
                    "bootstrap closure output gate identity must be unique",
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
                    format!("invalid bootstrap closure proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("proof:{proof_id}"),
                    "bootstrap closure proof identity must be unique",
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
                format!("unknown bootstrap closure gate key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(BootstrapClosureGateSurface {
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

pub fn validate_bootstrap_closure_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_bootstrap_closure_surface(input) {
        Ok(surface) => errors.extend(validate_bootstrap_closure_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_closure_model(surface: &BootstrapClosureGateSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "bootstrap closure gate must bind to P02",
        ));
    }
    if surface.task != "P02-024" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "bootstrap closure gate must bind to P02-024",
        ));
    }
    if !ALLOWED_GATE_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported bootstrap closure status {}", surface.status),
        ));
    }
    if surface.bounded_closure != "true" {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "bounded_closure",
            "P02-024 must explicitly permit bounded primary closure",
        ));
    }
    if surface.global_closure != "false" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "global_closure",
            "P02-024 must deny global closure until P02-X outputs are emitted",
        ));
    }
    if surface.next_frontier != "P02-X01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "next_frontier",
            "bootstrap closure next frontier must be P02-X01",
        ));
    }
    for required in REQUIRED_BOOTSTRAP_CLOSURE_RULES {
        if !surface.rules.contains_key(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{required}"),
                "missing bootstrap closure rule",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_CLOSURE_TASKS {
        if surface.task_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureTask,
                format!("task:{required}"),
                "missing bootstrap closure task binding",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS {
        if surface.output_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("output:{required}"),
                "missing bootstrap closure output gate",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_CLOSURE_PROOFS {
        if surface.proof_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("proof:{required}"),
                "missing bootstrap closure proof",
            ));
        }
    }
    validate_tasks(surface, &mut errors);
    validate_outputs(surface, &mut errors);
    validate_proofs(surface, &mut errors);
    validate_descriptor_registry(surface, &mut errors);
    validate_bootstrap_closure_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_task(
    line_number: usize,
    task_id: &str,
    value: &str,
) -> Result<BootstrapClosureTaskBinding, ValidationError> {
    let Some(fields) = parse_field_map(value) else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureTask,
            format!("line:{line_number:03}"),
            "invalid bootstrap closure task field map",
        ));
    };
    Ok(BootstrapClosureTaskBinding {
        line_number,
        id: task_id.to_string(),
        scope: required_field(&fields, "scope", ErrorCode::InvalidClosureTask, line_number)?,
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidClosureTask,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidClosureTask,
            line_number,
        )?),
        evidence: split_csv(&required_field(
            &fields,
            "evidence",
            ErrorCode::InvalidClosureTask,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureTask,
            line_number,
        )?,
    })
}

fn parse_output(
    line_number: usize,
    output_id: &str,
    value: &str,
) -> Result<BootstrapClosureOutputGate, ValidationError> {
    let Some(fields) = parse_field_map(value) else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "invalid bootstrap closure output field map",
        ));
    };
    Ok(BootstrapClosureOutputGate {
        line_number,
        id: output_id.to_string(),
        output_kind: required_field(
            &fields,
            "kind",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        path: required_field(
            &fields,
            "path",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        depends: split_csv(&required_field(
            &fields,
            "depends",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
    })
}

fn parse_proof(
    line_number: usize,
    proof_id: &str,
    value: &str,
) -> Result<BootstrapClosureProof, ValidationError> {
    let Some(fields) = parse_field_map(value) else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "invalid bootstrap closure proof field map",
        ));
    };
    Ok(BootstrapClosureProof {
        line_number,
        id: proof_id.to_string(),
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
        tasks: split_csv(&required_field(
            &fields,
            "tasks",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        outputs: split_csv(&required_field(
            &fields,
            "outputs",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        permits: split_csv(&required_field(
            &fields,
            "permits",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        forbids: split_csv(&required_field(
            &fields,
            "forbids",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
    })
}

fn validate_tasks(surface: &BootstrapClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for task in &surface.tasks {
        if !ALLOWED_TASK_SCOPES.contains(&task.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                format!("invalid bootstrap closure task scope {}", task.scope),
            ));
        }
        if !ALLOWED_TASK_STATUSES.contains(&task.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                format!("invalid bootstrap closure task status {}", task.status),
            ));
        }
        if task.receipts.is_empty() || task.commands.is_empty() || task.evidence.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                "bootstrap closure task bindings must include receipts, commands, and evidence",
            ));
        }
        if !task
            .commands
            .iter()
            .any(|command| command == "lyra-p02-bootstrap-closure-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                task.canonical_identity(),
                "bootstrap closure task must be checkable by lyra-p02-bootstrap-closure-check",
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
            if !receipt.starts_with("receipts/p02/pass_") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    task.canonical_identity(),
                    format!("task receipt must be P02 pass receipt: {receipt}"),
                ));
            }
        }
        for evidence in &task.evidence {
            if !allowed_artifact_path(evidence) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    task.canonical_identity(),
                    format!("invalid task evidence path {evidence}"),
                ));
            }
        }
    }
}

fn validate_outputs(surface: &BootstrapClosureGateSurface, errors: &mut Vec<ValidationError>) {
    for output in &surface.outputs {
        if !ALLOWED_OUTPUT_KINDS.contains(&output.output_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                output.canonical_identity(),
                format!("invalid output kind {}", output.output_kind),
            ));
        }
        if !allowed_artifact_path(&output.path) {
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
                    "bootstrap closure output gate cannot be {} at P02-024",
                    output.status
                ),
            ));
        }
        if output.depends.is_empty() || output.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                output.canonical_identity(),
                "bootstrap closure output gates must bind dependencies and receipts",
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

fn validate_proofs(surface: &BootstrapClosureGateSurface, errors: &mut Vec<ValidationError>) {
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
                "bootstrap closure proofs must bind receipts and commands",
            ));
        }
        if !proof
            .commands
            .iter()
            .any(|command| command == "lyra-p02-bootstrap-closure-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                proof.canonical_identity(),
                "bootstrap closure proofs must be checkable by lyra-p02-bootstrap-closure-check",
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
            .any(|item| item == "bounded_primary_closure" || item == "next_frontier_p02_x01")
        {
            errors.push(ValidationError::reject(ErrorCode::ClosureFormulaViolation, proof.canonical_identity(), "bootstrap closure proofs must permit bounded primary closure or next-frontier advance"));
        }
        if !proof.forbids.iter().any(|item| item == "global_closure") {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "bootstrap closure proofs must forbid global closure until P02-X outputs",
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
                "bootstrap closure proofs must forbid unreceipted closure",
            ));
        }
    }
}

fn validate_descriptor_registry(
    surface: &BootstrapClosureGateSurface,
    errors: &mut Vec<ValidationError>,
) {
    if LYRA_P02_BOOTSTRAP_CLOSURE_CARRIER != "lyra.p02.bootstrap_closure.carrier.v1"
        || !bootstrap_closure_carrier_signature().starts_with("fnv1a128:")
        || !bootstrap_closure_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "lyralang_bootstrap_closure_registry",
            "bootstrap closure carrier and registry hashes must be stable",
        ));
    }
    if !bootstrap_closure_tasks_bind_receipts()
        || !bootstrap_closure_outputs_remain_open()
        || !bootstrap_closure_proofs_bind_registry()
        || !bootstrap_closure_artifacts_bind_paths()
        || !bootstrap_closure_receipts_cover_p02_001_through_p02_024()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "lyralang_bootstrap_closure_registry",
            "bootstrap closure descriptor registry is not fully bound",
        ));
    }
    if !bootstrap_closure_no_forbidden_descriptor_claims() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "lyralang_bootstrap_closure_registry",
            "bootstrap closure descriptors contain forbidden closure/network claims",
        ));
    }
    for task in &surface.tasks {
        match bootstrap_closure_task_descriptor(&task.id) {
            Some(descriptor)
                if descriptor.scope == task.scope
                    && descriptor_slice_eq(descriptor.receipts, &task.receipts)
                    && descriptor_slice_eq(descriptor.commands, &task.commands)
                    && descriptor_slice_eq(descriptor.evidence, &task.evidence)
                    && descriptor.status == task.status
                    && bootstrap_closure_task_digest(&task.id)
                        .map(|hash| hash.starts_with("fnv1a128:"))
                        .unwrap_or(false) => {}
            _ => errors.push(ValidationError::reject(
                ErrorCode::ClosureDriftAccepted,
                task.canonical_identity(),
                "bootstrap closure task drifted from LyraLang descriptor",
            )),
        }
    }
    for output in &surface.outputs {
        match bootstrap_closure_output_descriptor(&output.id) {
            Some(descriptor)
                if descriptor.kind == output.output_kind
                    && descriptor.path == output.path
                    && descriptor_slice_eq(descriptor.depends, &output.depends)
                    && descriptor_slice_eq(descriptor.receipts, &output.receipts)
                    && descriptor.status == output.status
                    && bootstrap_closure_output_digest(&output.id)
                        .map(|hash| hash.starts_with("fnv1a128:"))
                        .unwrap_or(false) => {}
            _ => errors.push(ValidationError::reject(
                ErrorCode::ClosureDriftAccepted,
                output.canonical_identity(),
                "bootstrap closure output drifted from LyraLang descriptor",
            )),
        }
    }
    for proof in &surface.proofs {
        match bootstrap_closure_proof_descriptor(&proof.id) {
            Some(descriptor)
                if descriptor.scope == proof.scope
                    && descriptor_slice_eq(descriptor.tasks, &proof.tasks)
                    && descriptor_slice_eq(descriptor.outputs, &proof.outputs)
                    && descriptor_slice_eq(descriptor.receipts, &proof.receipts)
                    && descriptor_slice_eq(descriptor.commands, &proof.commands)
                    && descriptor_slice_eq(descriptor.permits, &proof.permits)
                    && descriptor_slice_eq(descriptor.forbids, &proof.forbids)
                    && descriptor.status == proof.status
                    && bootstrap_closure_proof_digest(&proof.id)
                        .map(|hash| hash.starts_with("fnv1a128:"))
                        .unwrap_or(false) => {}
            _ => errors.push(ValidationError::reject(
                ErrorCode::ClosureDriftAccepted,
                proof.canonical_identity(),
                "bootstrap closure proof drifted from LyraLang descriptor",
            )),
        }
    }
}

fn descriptor_slice_eq(left: &[&str], right: &[String]) -> bool {
    left.len() == right.len() && left.iter().zip(right.iter()).all(|(a, b)| *a == b.as_str())
}

fn validate_bootstrap_closure_report(
    surface: &BootstrapClosureGateSurface,
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
        deterministic_bootstrap_closure_gate_report(&task_inputs, &output_inputs, &proof_inputs);
    if report.task_count != REQUIRED_BOOTSTRAP_CLOSURE_TASKS.len()
        || report.output_gate_count != REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS.len()
        || report.proof_count != REQUIRED_BOOTSTRAP_CLOSURE_PROOFS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_bootstrap_closure_report",
            "bootstrap closure gate report count mismatch",
        ));
    }
    if report.bounded_task_count != REQUIRED_BOOTSTRAP_CLOSURE_TASKS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "k0_bootstrap_closure_report",
            "every admitted P02 primary task must be bounded_closed",
        ));
    }
    if report.open_output_count != REQUIRED_BOOTSTRAP_CLOSURE_OUTPUTS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureOutputPremature,
            "k0_bootstrap_closure_report",
            "all P02-X outputs must remain explicitly open after P02-024",
        ));
    }
    if !report.gate_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_bootstrap_closure_report",
            "bootstrap closure gate report hash must be stable fnv1a128",
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
fn is_bootstrap_closure_task_id(value: &str) -> bool {
    value.len() == 7
        && value.starts_with("P02-")
        && value[4..].bytes().all(|byte| byte.is_ascii_digit())
}
fn is_bootstrap_closure_output_id(value: &str) -> bool {
    value.len() == 7
        && value.starts_with("P02-X")
        && value[5..].bytes().all(|byte| byte.is_ascii_digit())
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
    for (needle, code) in FORBIDDEN_BOOTSTRAP_CLOSURE_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden bootstrap closure gate token {needle}"),
            ));
        }
    }
}
