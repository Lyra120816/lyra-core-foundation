use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_reference_semantics::deterministic_reference_semantics_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_reference_semantics::{
    reference_composition_descriptor, reference_eval_seed_descriptor,
    reference_eval_seed_trace_hash, reference_literal_descriptor, ReferenceCompositionDescriptor,
    ReferenceEvalSeedDescriptor, ReferenceLiteralDescriptor,
};
use crate::p01_reference_semantics_model::{
    ReferenceCompositionBinding, ReferenceEvalSeedBinding, ReferenceLiteralBinding,
    ReferenceSemanticsReceiptBinding, ReferenceSemanticsSurface,
};

pub const P01_REFERENCE_SEMANTICS_CONTRACT: &str = "LYRA-P01-REFERENCE-SEMANTICS v1";

pub const REQUIRED_REFERENCE_SEMANTICS_RULES: &[&str] = &[
    "literal_forms_are_self_evaluating",
    "literal_normal_forms_are_canonical",
    "composition_forms_reduce_structurally",
    "pair_evaluates_left_then_right",
    "list_evaluates_index_order",
    "record_keys_sort_ascii",
    "application_is_symbolic_seed_only",
    "binding_scope_is_explicit",
    "proof_step_preserves_receipt_seed",
    "evaluation_trace_is_hash_bound",
    "receipts_bind_reference_semantics_suite",
    "no_network_dependency",
    "no_probabilistic_reference_semantics",
    "no_placeholder_semantics",
    "no_global_closure_claim",
];

pub const REQUIRED_REFERENCE_LITERALS: &[&str] = &[
    "unit",
    "bool_true",
    "bool_false",
    "integer_zero",
    "integer_one",
    "text_empty",
    "symbol_core",
];

pub const REQUIRED_REFERENCE_COMPOSITIONS: &[&str] = &[
    "identity",
    "pair",
    "list",
    "record",
    "apply",
    "bind",
    "proof_step",
];

pub const REQUIRED_REFERENCE_EVAL_SEEDS: &[&str] = &[
    "literal_self",
    "pair_structural",
    "list_order",
    "record_key_sort",
    "apply_symbolic",
    "bind_scope",
    "proof_step",
];

pub const REQUIRED_REFERENCE_SEMANTICS_RECEIPTS: &[&str] = &["receipt_reference_semantics"];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_REFERENCE_SEMANTICS_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic reference semantics allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic semantics", ErrorCode::ProbabilisticTruthAllowed),
    ("random semantics", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("placeholder semantics", ErrorCode::PlaceholderAllowed),
    ("stub semantics", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("phase complete", ErrorCode::UnsupportedGlobalClosure),
    ("global closure", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_reference_semantics_surface(
    input: &str,
) -> Result<ReferenceSemanticsSurface, ValidationError> {
    let canonical = canonical_surface_text(input).map_err(|error| {
        ValidationError::reject(
            ErrorCode::CanonicalControlByte,
            "surface_text",
            format!("{error:?}"),
        )
    })?;
    if canonical.trim().is_empty() {
        return Err(ValidationError::reject(
            ErrorCode::EmptySurface,
            "surface_text",
            "empty P01 reference semantics surface",
        ));
    }

    let mut header = None;
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut literals = Vec::new();
    let mut compositions = Vec::new();
    let mut eval_seeds = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();

    let lines = canonical_lines(&canonical).map_err(|error| {
        ValidationError::reject(
            ErrorCode::CanonicalControlByte,
            "surface_text",
            format!("{error:?}"),
        )
    })?;
    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        if line_number == 1 {
            header = Some(line.to_string());
            continue;
        }
        let (key, value) = line.split_once('=').ok_or_else(|| {
            ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry must contain '='",
            )
        })?;
        if let Some(rule_id) = key.strip_prefix("rule:") {
            if rules
                .insert(rule_id.to_string(), value.to_string())
                .is_some()
            {
                return Err(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    rule_id,
                    "duplicate reference semantics rule",
                ));
            }
            continue;
        }
        if let Some(literal_id) = key.strip_prefix("literal:") {
            literals.push(parse_literal_binding(line_number, literal_id, value)?);
            continue;
        }
        if let Some(composition_id) = key.strip_prefix("composition:") {
            compositions.push(parse_composition_binding(
                line_number,
                composition_id,
                value,
            )?);
            continue;
        }
        if let Some(seed_id) = key.strip_prefix("eval_seed:") {
            eval_seeds.push(parse_eval_seed_binding(line_number, seed_id, value)?);
            continue;
        }
        if let Some(receipt_id) = key.strip_prefix("receipt:") {
            receipts.push(parse_receipt_binding(line_number, receipt_id, value)?);
            continue;
        }
        match key {
            "phase" => set_scalar(&mut phase, value, key, line_number, &mut seen_scalars)?,
            "task" => set_scalar(&mut task, value, key, line_number, &mut seen_scalars)?,
            "status" => set_scalar(&mut status, value, key, line_number, &mut seen_scalars)?,
            _ => {
                return Err(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    format!("unknown key {key}"),
                ))
            }
        }
    }

    Ok(ReferenceSemanticsSurface {
        header: header.ok_or_else(|| {
            ValidationError::reject(
                ErrorCode::InvalidHeader,
                "line:001",
                "missing P01 reference semantics header",
            )
        })?,
        phase: phase.ok_or_else(|| {
            ValidationError::reject(ErrorCode::MissingPhase, "phase", "missing phase")
        })?,
        task: task.ok_or_else(|| {
            ValidationError::reject(ErrorCode::MissingTask, "task", "missing task")
        })?,
        status: status.ok_or_else(|| {
            ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                "status",
                "missing status",
            )
        })?,
        rules,
        literals,
        compositions,
        eval_seeds,
        receipts,
    })
}

pub fn validate_reference_semantics_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut errors = Vec::new();
    let parsed = match parse_reference_semantics_surface(input) {
        Ok(surface) => Some(surface),
        Err(error) => {
            errors.push(error);
            None
        }
    };

    scan_forbidden_text(&canonical, &mut errors);

    if let Some(surface) = &parsed {
        validate_surface_header(surface, &mut errors);
        validate_required_rules(surface, &mut errors);
        validate_required_literals(surface, &mut errors);
        validate_required_compositions(surface, &mut errors);
        validate_required_eval_seeds(surface, &mut errors);
        validate_required_receipts(surface, &mut errors);
        validate_literal_descriptors(surface, &mut errors);
        validate_composition_descriptors(surface, &mut errors);
        validate_eval_seed_descriptors(surface, &mut errors);
        validate_receipts(surface, &mut errors);
        validate_reference_semantics_report(surface, &mut errors);
    }

    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_surface_header(surface: &ReferenceSemanticsSurface, errors: &mut Vec<ValidationError>) {
    if surface.header != P01_REFERENCE_SEMANTICS_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_REFERENCE_SEMANTICS_CONTRACT}"),
        ));
    }
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01, got {}", surface.phase),
        ));
    }
    if surface.task != "P01-005" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-005, got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "P01-005 must remain artifact_emitted",
        ));
    }
}

fn validate_required_rules(surface: &ReferenceSemanticsSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_REFERENCE_SEMANTICS_RULES {
        match surface.rule_value(rule) {
            Some(value) if !value.trim().is_empty() => {}
            _ => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                *rule,
                "missing required P01-005 reference semantics rule",
            )),
        }
    }
}

fn validate_required_literals(
    surface: &ReferenceSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for literal in &surface.literals {
        if !seen.insert(literal.id.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateCanonicalModel,
                literal.canonical_identity(),
                "duplicate reference literal",
            ));
        }
    }
    for required in REQUIRED_REFERENCE_LITERALS {
        if surface.literal_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                *required,
                "missing required reference literal",
            ));
        }
    }
}

fn validate_required_compositions(
    surface: &ReferenceSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for composition in &surface.compositions {
        if !seen.insert(composition.id.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateModelBinding,
                composition.canonical_identity(),
                "duplicate reference composition",
            ));
        }
    }
    for required in REQUIRED_REFERENCE_COMPOSITIONS {
        if surface.composition_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                *required,
                "missing required reference composition",
            ));
        }
    }
}

fn validate_required_eval_seeds(
    surface: &ReferenceSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for seed in &surface.eval_seeds {
        if !seen.insert(seed.id.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateModelBinding,
                seed.canonical_identity(),
                "duplicate reference eval seed",
            ));
        }
    }
    for required in REQUIRED_REFERENCE_EVAL_SEEDS {
        if surface.eval_seed_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                *required,
                "missing required reference eval seed",
            ));
        }
    }
}

fn validate_required_receipts(
    surface: &ReferenceSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for receipt in &surface.receipts {
        if !seen.insert(receipt.id.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateProofBinding,
                receipt.canonical_identity(),
                "duplicate reference semantics receipt",
            ));
        }
    }
    for required in REQUIRED_REFERENCE_SEMANTICS_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                *required,
                "missing required reference semantics receipt",
            ));
        }
    }
}

fn validate_literal_descriptors(
    surface: &ReferenceSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    for literal in &surface.literals {
        let Some(descriptor) = reference_literal_descriptor(&literal.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                literal.canonical_identity(),
                format!("unknown reference literal {}", literal.id),
            ));
            continue;
        };
        compare_literal_descriptor(literal, descriptor, errors);
        if !ALLOWED_STATUSES.contains(&literal.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                literal.canonical_identity(),
                format!("invalid literal status {}", literal.status),
            ));
        }
    }
}

fn validate_composition_descriptors(
    surface: &ReferenceSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    for composition in &surface.compositions {
        let Some(descriptor) = reference_composition_descriptor(&composition.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                composition.canonical_identity(),
                format!("unknown reference composition {}", composition.id),
            ));
            continue;
        };
        compare_composition_descriptor(composition, descriptor, errors);
        if !ALLOWED_STATUSES.contains(&composition.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                composition.canonical_identity(),
                format!("invalid composition status {}", composition.status),
            ));
        }
    }
}

fn validate_eval_seed_descriptors(
    surface: &ReferenceSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    for seed in &surface.eval_seeds {
        let Some(descriptor) = reference_eval_seed_descriptor(&seed.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                seed.canonical_identity(),
                format!("unknown reference eval seed {}", seed.id),
            ));
            continue;
        };
        compare_eval_seed_descriptor(seed, descriptor, errors);
        let expected_trace = reference_eval_seed_trace_hash(descriptor);
        if seed.trace != expected_trace {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                seed.canonical_identity(),
                format!("eval seed trace mismatch expected {expected_trace}"),
            ));
        }
        if !ALLOWED_STATUSES.contains(&seed.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                seed.canonical_identity(),
                format!("invalid eval seed status {}", seed.status),
            ));
        }
    }
}

fn validate_receipts(surface: &ReferenceSemanticsSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p01/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                receipt.canonical_identity(),
                format!("receipt path must be a P01 receipt: {}", receipt.path),
            ));
        }
        if receipt.target != "reference_semantics"
            && surface.literal_by_id(&receipt.target).is_none()
            && surface.composition_by_id(&receipt.target).is_none()
            && surface.eval_seed_by_id(&receipt.target).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                receipt.canonical_identity(),
                format!("unknown receipt target {}", receipt.target),
            ));
        }
        if !ALLOWED_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!("invalid receipt status {}", receipt.status),
            ));
        }
    }
}

fn validate_reference_semantics_report(
    surface: &ReferenceSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    let literal_inputs: Vec<(String, String, String, String, String, String, String)> = surface
        .literals
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.atom.clone(),
                item.canonical.clone(),
                item.normal.clone(),
                item.evaluator.clone(),
                item.proof.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let composition_inputs: Vec<(String, String, String, String, String, String, String)> = surface
        .compositions
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.operator.clone(),
                item.arity.clone(),
                item.input_order.clone(),
                item.output.clone(),
                item.law.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let eval_seed_inputs: Vec<(String, String, String, String, String, String, String)> = surface
        .eval_seeds
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.input.clone(),
                item.reduction.clone(),
                item.expected.clone(),
                item.law.clone(),
                item.trace.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let receipt_inputs: Vec<(String, String, String, String)> = surface
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

    let report = deterministic_reference_semantics_suite_report(
        &literal_inputs,
        &composition_inputs,
        &eval_seed_inputs,
        &receipt_inputs,
    );
    if report.literal_count != REQUIRED_REFERENCE_LITERALS.len()
        || report.composition_count != REQUIRED_REFERENCE_COMPOSITIONS.len()
        || report.eval_seed_count != REQUIRED_REFERENCE_EVAL_SEEDS.len()
        || report.receipt_count != REQUIRED_REFERENCE_SEMANTICS_RECEIPTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_reference_semantics_report",
            "reference semantics report does not cover required P01-005 suite",
        ));
    }
    if report.traced_eval_seed_count != REQUIRED_REFERENCE_EVAL_SEEDS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ReceiptHashMismatch,
            "k0_reference_semantics_report",
            "all eval seeds must be trace-hash bound",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_reference_semantics_report",
            "reference semantics suite hash must be stable fnv1a128",
        ));
    }
}

fn compare_literal_descriptor(
    binding: &ReferenceLiteralBinding,
    descriptor: ReferenceLiteralDescriptor,
    errors: &mut Vec<ValidationError>,
) {
    if binding.atom != descriptor.atom {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("atom drift expected {}", descriptor.atom),
        ));
    }
    if binding.canonical != descriptor.canonical {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("canonical drift expected {}", descriptor.canonical),
        ));
    }
    if binding.normal != descriptor.normal {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("normal drift expected {}", descriptor.normal),
        ));
    }
    if binding.evaluator != descriptor.evaluator {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("evaluator drift expected {}", descriptor.evaluator),
        ));
    }
    if binding.proof != descriptor.proof {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("proof drift expected {}", descriptor.proof),
        ));
    }
}

fn compare_composition_descriptor(
    binding: &ReferenceCompositionBinding,
    descriptor: ReferenceCompositionDescriptor,
    errors: &mut Vec<ValidationError>,
) {
    if binding.operator != descriptor.operator {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("operator drift expected {}", descriptor.operator),
        ));
    }
    if binding.arity != descriptor.arity {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("arity drift expected {}", descriptor.arity),
        ));
    }
    if binding.input_order != descriptor.input_order {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("input order drift expected {}", descriptor.input_order),
        ));
    }
    if binding.output != descriptor.output {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("output drift expected {}", descriptor.output),
        ));
    }
    if binding.law != descriptor.law {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("law drift expected {}", descriptor.law),
        ));
    }
}

fn compare_eval_seed_descriptor(
    binding: &ReferenceEvalSeedBinding,
    descriptor: ReferenceEvalSeedDescriptor,
    errors: &mut Vec<ValidationError>,
) {
    if binding.input != descriptor.input {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("input drift expected {}", descriptor.input),
        ));
    }
    if binding.reduction != descriptor.reduction {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("reduction drift expected {}", descriptor.reduction),
        ));
    }
    if binding.expected != descriptor.expected {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("expected drift expected {}", descriptor.expected),
        ));
    }
    if binding.law != descriptor.law {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            binding.canonical_identity(),
            format!("law drift expected {}", descriptor.law),
        ));
    }
}

fn parse_literal_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReferenceLiteralBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            format!("line:{line_number:03}"),
            "invalid literal field map",
        )
    })?;
    Ok(ReferenceLiteralBinding {
        line_number,
        id: id.to_string(),
        atom: required_field(
            &fields,
            "atom",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        canonical: required_field(
            &fields,
            "canonical",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        normal: required_field(
            &fields,
            "normal",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        evaluator: required_field(
            &fields,
            "evaluator",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        proof: required_field(
            &fields,
            "proof",
            ErrorCode::InvalidProofBinding,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}

fn parse_composition_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReferenceCompositionBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid composition field map",
        )
    })?;
    Ok(ReferenceCompositionBinding {
        line_number,
        id: id.to_string(),
        operator: required_field(
            &fields,
            "operator",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        arity: required_field(
            &fields,
            "arity",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        input_order: required_field(
            &fields,
            "input_order",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        output: required_field(
            &fields,
            "output",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        law: required_field(&fields, "law", ErrorCode::InvalidModelBinding, line_number)?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}

fn parse_eval_seed_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReferenceEvalSeedBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid eval seed field map",
        )
    })?;
    Ok(ReferenceEvalSeedBinding {
        line_number,
        id: id.to_string(),
        input: required_field(
            &fields,
            "input",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        reduction: required_field(
            &fields,
            "reduction",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        expected: required_field(
            &fields,
            "expected",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        law: required_field(&fields, "law", ErrorCode::InvalidModelBinding, line_number)?,
        trace: required_field(
            &fields,
            "trace",
            ErrorCode::InvalidProofBinding,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}

fn parse_receipt_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReferenceSemanticsReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            format!("line:{line_number:03}"),
            "invalid receipt field map",
        )
    })?;
    Ok(ReferenceSemanticsReceiptBinding {
        line_number,
        id: id.to_string(),
        path: required_field(&fields, "path", ErrorCode::InvalidProofBinding, line_number)?,
        target: required_field(
            &fields,
            "target",
            ErrorCode::InvalidProofBinding,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}

fn set_scalar(
    target: &mut Option<String>,
    value: &str,
    key: &str,
    line_number: usize,
    seen_scalars: &mut BTreeSet<String>,
) -> Result<(), ValidationError> {
    if !seen_scalars.insert(key.to_string()) || target.is_some() {
        Err(ValidationError::reject(
            ErrorCode::DuplicateEntry,
            format!("line:{line_number:03}"),
            format!("duplicate scalar {key}"),
        ))
    } else {
        *target = Some(value.to_string());
        Ok(())
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

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_REFERENCE_SEMANTICS_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden reference semantics token {needle}"),
            ));
        }
    }
}
