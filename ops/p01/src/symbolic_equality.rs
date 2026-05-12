use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_symbolic_equality::deterministic_symbolic_equality_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_symbolic_equality::{
    evaluate_substitution_case, evaluated_normalization_case_output, normalization_case_digest,
    substitution_case_digest, symbolic_equality_rule_descriptor,
    symbolic_equivalence_class_descriptor, symbolic_normalization_case_descriptor,
    symbolic_substitution_case_descriptor,
};
use crate::p01_symbolic_equality_model::{
    SymbolicEqualityReceiptBinding, SymbolicEqualityRuleBinding, SymbolicEqualitySurface,
    SymbolicEquivalenceClassBinding, SymbolicNormalizationCaseBinding,
    SymbolicSubstitutionCaseBinding,
};

pub const P01_SYMBOLIC_EQUALITY_CONTRACT: &str = "LYRA-P01-SYMBOLIC-EQUALITY v1";

pub const REQUIRED_SYMBOLIC_EQUALITY_RULES: &[&str] = &[
    "equality_is_normal_form_based",
    "equivalence_classes_have_canonical_representative",
    "normalization_is_recursive_and_byte_stable",
    "substitution_is_capture_avoiding",
    "alpha_equivalence_is_canonicalized",
    "record_keys_sort_ascii",
    "equality_witnesses_are_hash_bound",
    "substitution_cases_are_normalized_after_rewrite",
    "capture_risk_rejects_not_renames_silently",
    "receipts_bind_symbolic_equality_suite",
    "no_network_dependency",
    "no_probabilistic_equality_truth",
    "no_placeholder_equality",
    "no_global_closure_claim",
];

pub const REQUIRED_SYMBOLIC_EQUALITY_LAWS: &[&str] = &[
    "reflexive",
    "symmetric",
    "transitive",
    "alpha_equivalent",
    "structural",
];

pub const REQUIRED_SYMBOLIC_EQUIVALENCE_CLASSES: &[&str] = &[
    "unit_singleton",
    "bool_true_singleton",
    "record_order_class",
    "list_child_normal_class",
    "alpha_bind_class",
];

pub const REQUIRED_SYMBOLIC_NORMALIZATIONS: &[&str] = &[
    "literal_unit_normal",
    "record_key_sort_normal",
    "nested_record_normal",
    "apply_child_normal",
    "bind_alpha_normal",
];

pub const REQUIRED_SYMBOLIC_SUBSTITUTIONS: &[&str] = &[
    "substitute_symbol",
    "substitute_pair",
    "substitute_record",
    "binder_shadow_guard",
    "capture_rejection",
];

pub const REQUIRED_SYMBOLIC_EQUALITY_RECEIPTS: &[&str] = &["receipt_symbolic_equality"];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_SYMBOLIC_EQUALITY_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic equality truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic equality", ErrorCode::ProbabilisticTruthAllowed),
    ("random equality", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder equality", ErrorCode::PlaceholderAllowed),
    ("stub equality", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_symbolic_equality_surface(
    input: &str,
) -> Result<SymbolicEqualitySurface, Vec<ValidationError>> {
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
            "no symbolic equality lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SYMBOLIC_EQUALITY_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SYMBOLIC_EQUALITY_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut equality_rules = Vec::new();
    let mut equivalence_classes = Vec::new();
    let mut normalizations = Vec::new();
    let mut substitutions = Vec::new();
    let mut receipts = Vec::new();

    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_equality_rules = BTreeSet::new();
    let mut seen_equivalence_classes = BTreeSet::new();
    let mut seen_normalizations = BTreeSet::new();
    let mut seen_substitutions = BTreeSet::new();
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
            "equality_rule" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidCanonicalModel,
                        format!("line:{line_number:03}"),
                        "equality_rule missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_equality_rules.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateCanonicalModel,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid equality_rule {id}"),
                    ));
                    continue;
                }
                equality_rules.push(SymbolicEqualityRuleBinding {
                    line_number,
                    id: id.to_string(),
                    domain: required_field(&fields, "domain").unwrap_or("").to_string(),
                    relation: required_field(&fields, "relation")
                        .unwrap_or("")
                        .to_string(),
                    law: required_field(&fields, "law").unwrap_or("").to_string(),
                    status: required_field(&fields, "status").unwrap_or("").to_string(),
                });
            }
            "equivalence_class" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "equivalence_class missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_equivalence_classes.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid equivalence_class {id}"),
                    ));
                    continue;
                }
                equivalence_classes.push(SymbolicEquivalenceClassBinding {
                    line_number,
                    id: id.to_string(),
                    members: required_field(&fields, "members").unwrap_or("").to_string(),
                    canonical: required_field(&fields, "canonical")
                        .unwrap_or("")
                        .to_string(),
                    normalizer: required_field(&fields, "normalizer")
                        .unwrap_or("")
                        .to_string(),
                    status: required_field(&fields, "status").unwrap_or("").to_string(),
                });
            }
            "normalization" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "normalization missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_normalizations.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid normalization {id}"),
                    ));
                    continue;
                }
                normalizations.push(SymbolicNormalizationCaseBinding {
                    line_number,
                    id: id.to_string(),
                    input: required_field(&fields, "input").unwrap_or("").to_string(),
                    output: required_field(&fields, "output").unwrap_or("").to_string(),
                    law: required_field(&fields, "law").unwrap_or("").to_string(),
                    digest: required_field(&fields, "digest").unwrap_or("").to_string(),
                    status: required_field(&fields, "status").unwrap_or("").to_string(),
                });
            }
            "substitution" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "substitution missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_substitutions.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid substitution {id}"),
                    ));
                    continue;
                }
                substitutions.push(SymbolicSubstitutionCaseBinding {
                    line_number,
                    id: id.to_string(),
                    target: required_field(&fields, "target").unwrap_or("").to_string(),
                    replacement: required_field(&fields, "replacement")
                        .unwrap_or("")
                        .to_string(),
                    scope: required_field(&fields, "scope").unwrap_or("").to_string(),
                    expected: required_field(&fields, "expected")
                        .unwrap_or("")
                        .to_string(),
                    law: required_field(&fields, "law").unwrap_or("").to_string(),
                    digest: required_field(&fields, "digest").unwrap_or("").to_string(),
                    status: required_field(&fields, "status").unwrap_or("").to_string(),
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
                receipts.push(SymbolicEqualityReceiptBinding {
                    line_number,
                    id: id.to_string(),
                    path: required_field(&fields, "path").unwrap_or("").to_string(),
                    target: required_field(&fields, "target").unwrap_or("").to_string(),
                    status: required_field(&fields, "status").unwrap_or("").to_string(),
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

    Ok(SymbolicEqualitySurface {
        header,
        phase,
        task,
        status,
        rules,
        equality_rules,
        equivalence_classes,
        normalizations,
        substitutions,
        receipts,
    })
}

pub fn validate_symbolic_equality_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();

    let lower = input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SYMBOLIC_EQUALITY_TEXT {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden-token",
                format!("forbidden token {needle}"),
            ));
        }
    }

    let parsed = match parse_symbolic_equality_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };

    validate_symbolic_equality_surface_model(&parsed, &mut errors);

    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_symbolic_equality_surface_model(
    surface: &SymbolicEqualitySurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-006" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-006 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }

    for required in REQUIRED_SYMBOLIC_EQUALITY_RULES {
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
                "missing symbolic equality rule",
            )),
        }
    }

    for required in REQUIRED_SYMBOLIC_EQUALITY_LAWS {
        if surface.equality_rule_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("equality_rule:{required}"),
                "missing equality law",
            ));
        }
    }
    for required in REQUIRED_SYMBOLIC_EQUIVALENCE_CLASSES {
        if surface.equivalence_class_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("equivalence_class:{required}"),
                "missing equivalence class",
            ));
        }
    }
    for required in REQUIRED_SYMBOLIC_NORMALIZATIONS {
        if surface.normalization_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("normalization:{required}"),
                "missing normalization case",
            ));
        }
    }
    for required in REQUIRED_SYMBOLIC_SUBSTITUTIONS {
        if surface.substitution_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("substitution:{required}"),
                "missing substitution case",
            ));
        }
    }
    for required in REQUIRED_SYMBOLIC_EQUALITY_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "missing symbolic equality receipt",
            ));
        }
    }

    for binding in &surface.equality_rules {
        validate_status(
            "equality_rule",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = symbolic_equality_rule_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown equality rule {}", binding.id),
            ));
            continue;
        };
        if binding.domain != descriptor.domain
            || binding.relation != descriptor.relation
            || binding.law != descriptor.law
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("equality rule descriptor drift {}", binding.id),
            ));
        }
    }

    for binding in &surface.equivalence_classes {
        validate_status(
            "equivalence_class",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = symbolic_equivalence_class_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown equivalence class {}", binding.id),
            ));
            continue;
        };
        if binding.members != descriptor.members
            || binding.canonical != descriptor.canonical
            || binding.normalizer != descriptor.normalizer
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("equivalence class descriptor drift {}", binding.id),
            ));
        }
    }

    for binding in &surface.normalizations {
        validate_status(
            "normalization",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = symbolic_normalization_case_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown normalization case {}", binding.id),
            ));
            continue;
        };
        let expected_digest =
            normalization_case_digest(&binding.id, &binding.input, &binding.output, &binding.law);
        if binding.input != descriptor.input
            || binding.output != descriptor.output
            || binding.law != descriptor.law
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("normalization descriptor drift {}", binding.id),
            ));
        }
        if binding.digest != expected_digest {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", binding.line_number),
                format!("normalization digest mismatch {}", binding.id),
            ));
        }
        match evaluated_normalization_case_output(&binding.id) {
            Ok(output) if output == binding.output => {}
            Ok(output) => errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!(
                    "evaluated normalization mismatch {} got {}",
                    binding.id, output
                ),
            )),
            Err(error) => errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                format!("line:{:03}", binding.line_number),
                format!(
                    "normalization evaluation error {:?} for {}",
                    error, binding.id
                ),
            )),
        }
    }

    for binding in &surface.substitutions {
        validate_status(
            "substitution",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = symbolic_substitution_case_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown substitution case {}", binding.id),
            ));
            continue;
        };
        let expected_digest = substitution_case_digest(
            &binding.id,
            &binding.target,
            &binding.replacement,
            &binding.scope,
            &binding.expected,
            &binding.law,
        );
        if binding.target != descriptor.target
            || binding.replacement != descriptor.replacement
            || binding.scope != descriptor.scope
            || binding.expected != descriptor.expected
            || binding.law != descriptor.law
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("substitution descriptor drift {}", binding.id),
            ));
        }
        if binding.digest != expected_digest {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", binding.line_number),
                format!("substitution digest mismatch {}", binding.id),
            ));
        }
        match evaluate_substitution_case(&binding.id) {
            Ok(output) if output == binding.expected => {}
            Ok(output) => errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!(
                    "evaluated substitution mismatch {} got {}",
                    binding.id, output
                ),
            )),
            Err(error) => errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                format!("line:{:03}", binding.line_number),
                format!(
                    "substitution evaluation error {:?} for {}",
                    error, binding.id
                ),
            )),
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
        if binding.id == "receipt_symbolic_equality" {
            if binding.path != "receipts/p01/pass_0035_symbolic_equality.receipt" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidProofBinding,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt path {}", binding.path),
                ));
            }
            if binding.target != "P01-006" {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt target {}", binding.target),
                ));
            }
        }
    }

    let equality_rule_rows: Vec<(String, String, String, String, String)> = surface
        .equality_rules
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.domain.clone(),
                item.relation.clone(),
                item.law.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let equivalence_rows: Vec<(String, String, String, String, String)> = surface
        .equivalence_classes
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.members.clone(),
                item.canonical.clone(),
                item.normalizer.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let normalization_rows: Vec<(String, String, String, String, String, String)> = surface
        .normalizations
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.input.clone(),
                item.output.clone(),
                item.law.clone(),
                item.digest.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let substitution_rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .substitutions
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.target.clone(),
                item.replacement.clone(),
                item.scope.clone(),
                item.expected.clone(),
                item.law.clone(),
                item.digest.clone(),
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
    let suite = deterministic_symbolic_equality_suite_report(
        &equality_rule_rows,
        &equivalence_rows,
        &normalization_rows,
        &substitution_rows,
        &receipt_rows,
    );
    if suite.equality_rule_count < REQUIRED_SYMBOLIC_EQUALITY_LAWS.len()
        || suite.equivalence_class_count < REQUIRED_SYMBOLIC_EQUIVALENCE_CLASSES.len()
        || suite.normalization_count < REQUIRED_SYMBOLIC_NORMALIZATIONS.len()
        || suite.substitution_count < REQUIRED_SYMBOLIC_SUBSTITUTIONS.len()
        || suite.receipt_count < REQUIRED_SYMBOLIC_EQUALITY_RECEIPTS.len()
        || !suite.suite_hash.starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "suite",
            "symbolic equality suite report is incomplete or unhashable",
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

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && matches!(value.as_bytes().first(), Some(byte) if byte.is_ascii_lowercase())
}
