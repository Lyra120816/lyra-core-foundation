use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_core_ir_reuse::deterministic_core_ir_reuse_suite_report;
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_core_ir_reuse::{
    canonical_core_ir_reuse_registry_hash, core_ir_reuse_consumer_descriptor,
    core_ir_reuse_consumer_ids, core_ir_reuse_edge_descriptor,
    core_ir_reuse_edge_endpoints_are_bound, core_ir_reuse_edge_ids, core_ir_reuse_gate_descriptor,
    core_ir_reuse_gate_ids, core_ir_reuse_ref_is_bound,
};
use crate::p01_core_ir_reuse_model::{
    CoreIrReuseConsumerBinding, CoreIrReuseEdgeBinding, CoreIrReuseGateBinding,
    CoreIrReuseReceiptBinding, CoreIrReuseSurface,
};

pub const P01_CORE_IR_REUSE_CONTRACT: &str = "LYRA-P01-CORE-IR-REUSE v1";
pub const REQUIRED_CORE_IR_REUSE_RULES: &[&str] = &[
    "core_ir_is_single_cross_phase_contract",
    "parser_checker_evaluator_vm_proof_product_consume_same_ir",
    "no_private_ir_forks",
    "reuse_edges_preserve_identity_digest",
    "fixtures_cover_each_consumer",
    "drift_is_rejected",
    "receipts_bind_cross_phase_reuse",
    "no_network_dependency",
    "no_probabilistic_truth",
    "no_hidden_randomness",
    "no_placeholder_reuse",
    "no_global_closure_claim",
];
pub const REQUIRED_CORE_IR_REUSE_CONSUMERS: &[&str] = &[
    "parser_surface",
    "checker_surface",
    "evaluator_surface",
    "vm_surface",
    "proof_surface",
    "product_surface",
];
pub const REQUIRED_CORE_IR_REUSE_EDGES: &[&str] = &[
    "core_ir_registry_to_parser",
    "parser_to_checker_ir_edge",
    "checker_to_evaluator_ir_edge",
    "evaluator_to_vm_ir_edge",
    "vm_to_proof_ir_edge",
    "proof_to_product_ir_edge",
];
pub const REQUIRED_CORE_IR_REUSE_GATES: &[&str] = &[
    "single_ir_contract_gate",
    "cross_phase_identity_gate",
    "product_truth_gate",
];
pub const REQUIRED_CORE_IR_REUSE_RECEIPTS: &[&str] = &["receipt_core_ir_reuse"];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_CORE_IR_REUSE_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic reuse", ErrorCode::ProbabilisticTruthAllowed),
    ("random reuse", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    (
        "private ir fork allowed",
        ErrorCode::CanonicalModelDriftAccepted,
    ),
    (
        "private_ir_forks_allowed",
        ErrorCode::CanonicalModelDriftAccepted,
    ),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder reuse", ErrorCode::PlaceholderAllowed),
    ("stub reuse", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_core_ir_reuse_surface(
    input: &str,
) -> Result<CoreIrReuseSurface, Vec<ValidationError>> {
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
            "no core ir reuse lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_CORE_IR_REUSE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_CORE_IR_REUSE_CONTRACT}"),
        )]);
    }
    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut consumers = Vec::new();
    let mut edges = Vec::new();
    let mut gates = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_consumers = BTreeSet::new();
    let mut seen_edges = BTreeSet::new();
    let mut seen_gates = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();
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
                    ErrorCode::DuplicateCanonicalModel,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid rule {rule_name}"),
                ));
                continue;
            }
            rules.insert(rule_name.to_string(), value.to_string());
            continue;
        }
        match left {
            "phase" | "task" | "status" => {
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
                    _ => {}
                }
            }
            "consumer" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidCanonicalModel,
                        format!("line:{line_number:03}"),
                        "consumer missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_consumers.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateCanonicalModel,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid consumer {id}"),
                    ));
                    continue;
                }
                consumers.push(CoreIrReuseConsumerBinding {
                    line_number,
                    id: id.to_string(),
                    surface: field(&fields, "surface"),
                    target_phase: field(&fields, "phase"),
                    owner_root: field(&fields, "owner"),
                    core_ir_ref: field(&fields, "core_ir_ref"),
                    adapter: field(&fields, "adapter"),
                    fixture_path: field(&fields, "fixture"),
                    status: field(&fields, "status"),
                });
            }
            "edge" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "edge missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_edges.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid edge {id}"),
                    ));
                    continue;
                }
                edges.push(CoreIrReuseEdgeBinding {
                    line_number,
                    id: id.to_string(),
                    from_consumer: field(&fields, "from"),
                    to_consumer: field(&fields, "to"),
                    form: field(&fields, "form"),
                    guard: field(&fields, "guard"),
                    rejection: field(&fields, "rejection"),
                    receipt_ref: field(&fields, "receipt"),
                    status: field(&fields, "status"),
                });
            }
            "gate" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "gate missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_gates.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid gate {id}"),
                    ));
                    continue;
                }
                gates.push(CoreIrReuseGateBinding {
                    line_number,
                    id: id.to_string(),
                    scope: field(&fields, "scope"),
                    law: field(&fields, "law"),
                    evidence: field(&fields, "evidence"),
                    status: field(&fields, "status"),
                });
            }
            "receipt" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidProofBinding,
                        format!("line:{line_number:03}"),
                        "receipt missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_receipts.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateProofBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid receipt {id}"),
                    ));
                    continue;
                }
                receipts.push(CoreIrReuseReceiptBinding {
                    line_number,
                    id: id.to_string(),
                    path: field(&fields, "path"),
                    target: field(&fields, "target"),
                    status: field(&fields, "status"),
                });
            }
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown entry {left}"),
            )),
        }
    }
    let Some(phase) = phase else {
        errors.push(ValidationError::reject(
            ErrorCode::MissingPhase,
            "phase",
            "missing phase",
        ));
        return Err(errors);
    };
    let Some(task) = task else {
        errors.push(ValidationError::reject(
            ErrorCode::MissingTask,
            "task",
            "missing task",
        ));
        return Err(errors);
    };
    let Some(status) = status else {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "missing status",
        ));
        return Err(errors);
    };
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(CoreIrReuseSurface {
        header,
        phase,
        task,
        status,
        rules,
        consumers,
        edges,
        gates,
        receipts,
    })
}

pub fn validate_core_ir_reuse_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_core_ir_reuse_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_core_ir_reuse_surface_model(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_core_ir_reuse_surface_model(
    surface: &CoreIrReuseSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-010" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-010 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }
    for required in REQUIRED_CORE_IR_REUSE_RULES {
        match surface.rules.get(*required) {
            Some(value) if value == "required" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                format!("rule:{required}"),
                format!("expected required got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "missing core ir reuse rule",
            )),
        }
    }
    for required in REQUIRED_CORE_IR_REUSE_CONSUMERS {
        if surface.consumer_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("consumer:{required}"),
                "missing core ir reuse consumer",
            ));
        }
    }
    for required in REQUIRED_CORE_IR_REUSE_EDGES {
        if surface.edge_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("edge:{required}"),
                "missing core ir reuse edge",
            ));
        }
    }
    for required in REQUIRED_CORE_IR_REUSE_GATES {
        if surface.gate_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("gate:{required}"),
                "missing core ir reuse gate",
            ));
        }
    }
    for required in REQUIRED_CORE_IR_REUSE_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "missing core ir reuse receipt",
            ));
        }
    }
    for binding in &surface.consumers {
        validate_status(
            "consumer",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = core_ir_reuse_consumer_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown core ir reuse consumer {}", binding.id),
            ));
            continue;
        };
        if binding.surface != descriptor.surface
            || binding.target_phase != descriptor.target_phase
            || binding.owner_root != descriptor.owner_root
            || binding.core_ir_ref != descriptor.core_ir_ref
            || binding.adapter != descriptor.adapter
            || binding.fixture_path != descriptor.fixture_path
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("consumer descriptor drift {}", binding.id),
            ));
        }
        if !core_ir_reuse_ref_is_bound(&binding.core_ir_ref) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unbound core ir ref {}", binding.core_ir_ref),
            ));
        }
    }
    for binding in &surface.edges {
        validate_status(
            "edge",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = core_ir_reuse_edge_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown core ir reuse edge {}", binding.id),
            ));
            continue;
        };
        if binding.from_consumer != descriptor.from_consumer
            || binding.to_consumer != descriptor.to_consumer
            || binding.form != descriptor.form
            || binding.guard != descriptor.guard
            || binding.rejection != descriptor.rejection
            || binding.receipt_ref != descriptor.receipt_ref
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("edge descriptor drift {}", binding.id),
            ));
        }
        if !core_ir_reuse_edge_endpoints_are_bound(descriptor) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unbound edge endpoint {}", binding.id),
            ));
        }
    }
    for binding in &surface.gates {
        validate_status(
            "gate",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = core_ir_reuse_gate_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown core ir reuse gate {}", binding.id),
            ));
            continue;
        };
        if binding.scope != descriptor.scope
            || binding.law != descriptor.law
            || binding.evidence != descriptor.evidence
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("gate descriptor drift {}", binding.id),
            ));
        }
    }
    for binding in &surface.receipts {
        validate_status(
            "receipt",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        if binding.id == "receipt_core_ir_reuse" {
            if binding.path != "receipts/p01/pass_0039_core_ir_reuse.receipt" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidProofBinding,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt path {}", binding.path),
                ));
            }
            if binding.target != "P01-010" {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt target {}", binding.target),
                ));
            }
        }
    }
    let consumer_rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .consumers
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.surface.clone(),
                item.target_phase.clone(),
                item.owner_root.clone(),
                item.core_ir_ref.clone(),
                item.adapter.clone(),
                item.fixture_path.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let edge_rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .edges
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.from_consumer.clone(),
                item.to_consumer.clone(),
                item.form.clone(),
                item.guard.clone(),
                item.rejection.clone(),
                item.receipt_ref.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let gate_rows: Vec<(String, String, String, String, String)> = surface
        .gates
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.scope.clone(),
                item.law.clone(),
                item.evidence.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let receipt_rows: Vec<(String, String, String, String)> = surface
        .receipts
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.path.clone(),
                item.target.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let suite = deterministic_core_ir_reuse_suite_report(
        &consumer_rows,
        &edge_rows,
        &gate_rows,
        &receipt_rows,
    );
    if suite.consumer_count < core_ir_reuse_consumer_ids().len()
        || suite.edge_count < core_ir_reuse_edge_ids().len()
        || suite.gate_count < core_ir_reuse_gate_ids().len()
        || suite.receipt_count < REQUIRED_CORE_IR_REUSE_RECEIPTS.len()
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !canonical_core_ir_reuse_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "suite",
            "core ir reuse suite report is incomplete or unhashable",
        ));
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
            format!("line:{line_number:03}"),
            format!("{kind} {id} has unsupported status {status}"),
        ));
    }
}
fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_CORE_IR_REUSE_TEXT {
        if lower.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden_text",
                format!("forbidden token {token}"),
            ));
        }
    }
}
fn parse_pipe_fields(value: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        if let Some((key, field_value)) = part.split_once(':') {
            fields.insert(key.to_string(), field_value.to_string());
        }
    }
    fields
}
fn required_field<'a>(fields: &'a BTreeMap<String, String>, name: &str) -> Option<&'a str> {
    fields
        .get(name)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
}
fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    required_field(fields, name).unwrap_or("").to_string()
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && matches!(value.as_bytes().first(), Some(byte) if byte.is_ascii_lowercase())
}
