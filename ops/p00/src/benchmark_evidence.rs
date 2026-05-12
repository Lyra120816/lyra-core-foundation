use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_benchmark_evidence_model::{
    BenchmarkEvidenceLawSurface, BenchmarkTarget, ClosureFormula, DefinitionOfDone, EvidenceBinding,
};

pub const P00_BENCHMARK_EVIDENCE_LAW_CONTRACT: &str = "LYRA-P00-BENCHMARK-EVIDENCE-LAW v1";

pub const REQUIRED_BENCHMARK_EVIDENCE_RULES: &[&str] = &[
    "benchmark_pack_required",
    "evidence_family_required",
    "local_definition_of_done_required",
    "global_definition_of_done_forbidden_until_all_blockers_closed",
    "receipt_binding_required",
    "command_record_required",
    "deterministic_stability_required",
    "negative_path_required",
    "rollback_path_required",
    "closure_formula_required",
];

pub struct RequiredBenchmark {
    pub id: &'static str,
    pub metric: &'static str,
}

pub const REQUIRED_BENCHMARKS: &[RequiredBenchmark] = &[
    RequiredBenchmark {
        id: "canonicalization_stability",
        metric: "correctness",
    },
    RequiredBenchmark {
        id: "validator_rejection_stability",
        metric: "correctness",
    },
    RequiredBenchmark {
        id: "receipt_replay_stability",
        metric: "stability",
    },
    RequiredBenchmark {
        id: "frontier_control_stability",
        metric: "correctness",
    },
];

pub const REQUIRED_EVIDENCE_FAMILIES: &[&str] = &[
    "happy_path",
    "negative_path",
    "adversarial_path",
    "rollback_path",
    "replay_path",
];

pub const REQUIRED_LOCAL_DEFINITIONS: &[&str] = &[
    "local_working_slice",
    "local_execution_proven",
    "phase_closure_blocked",
    "global_closure_blocked",
];

const BENCHMARK_METRICS: &[&str] = &["correctness", "throughput", "latency", "stability"];
const STABILITY_TOKENS: &[&str] = &[
    "byte_stable",
    "order_stable",
    "receipt_stable",
    "replay_stable",
];
const CLOSURE_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const DEFINITION_SCOPES: &[&str] = &["task", "phase", "global"];
const CLOSURE_SCOPES: &[&str] = &["task", "frontier", "phase", "global"];

const FORBIDDEN_BENCHMARK_EVIDENCE_TEXT: &[(&str, ErrorCode)] = &[
    ("benchmark later", ErrorCode::MissingBenchmarkTarget),
    ("evidence later", ErrorCode::MissingEvidenceBinding),
    ("receipt later", ErrorCode::BenchmarkMissingReceipt),
    ("command later", ErrorCode::MissingCommandRecord),
    ("definition later", ErrorCode::MissingDefinitionOfDone),
    ("approximate benchmark", ErrorCode::BenchmarkTargetUnstable),
    ("best effort benchmark", ErrorCode::BenchmarkTargetUnstable),
    ("manual evidence", ErrorCode::InvalidEvidenceBinding),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    (
        "complete without evidence",
        ErrorCode::ClosureFormulaViolation,
    ),
];

pub fn parse_benchmark_evidence_law_surface(
    input: &str,
) -> Result<BenchmarkEvidenceLawSurface, Vec<ValidationError>> {
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
            "no benchmark-evidence law lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_BENCHMARK_EVIDENCE_LAW_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_BENCHMARK_EVIDENCE_LAW_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut benchmarks = Vec::new();
    let mut evidence = Vec::new();
    let mut definitions = Vec::new();
    let mut closures = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_benchmarks = BTreeSet::new();
    let mut seen_evidence = BTreeSet::new();
    let mut seen_definitions = BTreeSet::new();
    let mut seen_closures = BTreeSet::new();

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

        if value.is_empty() || value != value.trim() || left.is_empty() || left != left.trim() {
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
                    "benchmark-evidence rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(benchmark_id) = left.strip_prefix("benchmark:") {
            if !is_symbolic_name(benchmark_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidBenchmarkTarget,
                    format!("line:{line_number:03}"),
                    format!("invalid benchmark identity {benchmark_id}"),
                ));
                continue;
            }
            if !seen_benchmarks.insert(benchmark_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateBenchmarkTarget,
                    format!("benchmark:{benchmark_id}"),
                    "benchmark identity must be unique",
                ));
                continue;
            }
            match parse_benchmark(line_number, benchmark_id, value) {
                Ok(benchmark) => benchmarks.push(benchmark),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(evidence_id) = left.strip_prefix("evidence:") {
            if !is_symbolic_name(evidence_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEvidenceBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid evidence identity {evidence_id}"),
                ));
                continue;
            }
            if !seen_evidence.insert(evidence_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEvidenceBinding,
                    format!("evidence:{evidence_id}"),
                    "evidence identity must be unique",
                ));
                continue;
            }
            match parse_evidence(line_number, evidence_id, value) {
                Ok(binding) => evidence.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(definition_id) = left.strip_prefix("definition:") {
            if !is_symbolic_name(definition_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDefinitionOfDone,
                    format!("line:{line_number:03}"),
                    format!("invalid definition identity {definition_id}"),
                ));
                continue;
            }
            if !seen_definitions.insert(definition_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDefinitionOfDone,
                    format!("definition:{definition_id}"),
                    "definition-of-done identity must be unique",
                ));
                continue;
            }
            match parse_definition(line_number, definition_id, value) {
                Ok(definition) => definitions.push(definition),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(closure_id) = left.strip_prefix("closure:") {
            if !is_symbolic_name(closure_id) || !seen_closures.insert(closure_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "closure formula identity must be symbolic and unique",
                ));
                continue;
            }
            match parse_closure(line_number, closure_id, value) {
                Ok(formula) => closures.push(formula),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if !seen_scalars.insert(left.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                left.to_string(),
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
                format!("unknown benchmark-evidence field {left}"),
            )),
        }
    }

    let phase = match phase {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPhase,
                "field:phase",
                "phase=P00 is required",
            ));
            String::new()
        }
    };
    let task = match task {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::MissingTask,
                "field:task",
                "task=P00-009 is required",
            ));
            String::new()
        }
    };
    let status = match status {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                "field:status",
                "status=working_slice is required",
            ));
            String::new()
        }
    };

    if errors.is_empty() {
        Ok(BenchmarkEvidenceLawSurface {
            header,
            phase,
            task,
            status,
            rules,
            benchmarks,
            evidence,
            definitions,
            closures,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_benchmark_evidence_law_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_benchmark_evidence_law_surface(input) {
        Ok(surface) => validate_parsed_benchmark_evidence_law_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_benchmark(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BenchmarkTarget, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let metric = required_string_field(line_number, &mut fields, "metric")?;
    let target = required_string_field(line_number, &mut fields, "target")?;
    let method = required_string_field(line_number, &mut fields, "method")?;
    let stability = required_string_field(line_number, &mut fields, "stability")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(BenchmarkTarget {
        line_number,
        id: id.to_string(),
        metric,
        target,
        method,
        stability,
        evidence,
    })
}

fn parse_evidence(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<EvidenceBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let family = required_string_field(line_number, &mut fields, "family")?;
    let artifacts = required_list_field(line_number, &mut fields, "artifacts")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(EvidenceBinding {
        line_number,
        id: id.to_string(),
        family,
        artifacts,
        receipts,
        commands,
        status,
    })
}

fn parse_definition(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<DefinitionOfDone, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let requires = required_list_field(line_number, &mut fields, "requires")?;
    let allows = required_list_field(line_number, &mut fields, "allows")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(DefinitionOfDone {
        line_number,
        id: id.to_string(),
        scope,
        requires,
        allows,
        forbids,
        evidence,
    })
}

fn parse_closure(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ClosureFormula, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let benchmarks = required_list_field(line_number, &mut fields, "benchmarks")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    let definitions = required_list_field(line_number, &mut fields, "definitions")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ClosureFormula {
        line_number,
        id: id.to_string(),
        scope,
        status,
        benchmarks,
        evidence,
        definitions,
        receipts,
        commands,
    })
}

fn parse_fields(
    line_number: usize,
    value: &str,
) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        let Some((key, field_value)) = part.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "benchmark-evidence attributes must use key:value fields",
            ));
        };
        if !is_symbolic_name(key) || field_value.is_empty() || field_value != field_value.trim() {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field keys must be symbolic and values must be non-empty trimmed text",
            ));
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
            return Err(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate attribute {key}"),
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
    match fields.remove(key) {
        Some(value) if !value.is_empty() => Ok(value),
        _ => Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("required field {key} is absent or empty"),
        )),
    }
}

fn required_list_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<Vec<String>, ValidationError> {
    let value = required_string_field(line_number, fields, key)?;
    let items = split_list(&value);
    if items.is_empty() {
        Err(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            format!("line:{line_number:03}"),
            format!("required list field {key} must not be empty"),
        ))
    } else {
        Ok(items)
    }
}

fn reject_unknown_fields(
    line_number: usize,
    fields: BTreeMap<String, String>,
) -> Result<(), ValidationError> {
    if fields.is_empty() {
        Ok(())
    } else {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "benchmark-evidence surface contains unsupported attributes",
        ))
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

fn validate_parsed_benchmark_evidence_law_surface(
    surface: &BenchmarkEvidenceLawSurface,
    raw_input: &str,
) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            format!("expected P00, found {}", surface.phase),
        ));
    }
    if surface.task != "P00-009" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-009, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-009 may only declare working_slice status",
        ));
    }

    for rule in REQUIRED_BENCHMARK_EVIDENCE_RULES {
        match surface.rule_value(rule) {
            Some(value) if value.contains("required") || value.contains("forbidden") => {}
            Some(_) | None => errors.push(ValidationError::reject(
                ErrorCode::MissingBenchmarkRule,
                format!("rule:{rule}"),
                "required benchmark/evidence rule is absent or too weak",
            )),
        }
    }

    if surface.benchmarks.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBenchmarkTarget,
            "benchmark:*",
            "at least one benchmark target is required",
        ));
    }
    for required in REQUIRED_BENCHMARKS {
        match surface.benchmark_by_id(required.id) {
            Some(benchmark) if benchmark.metric == required.metric => {}
            Some(_) => errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                format!("benchmark:{}", required.id),
                "required benchmark metric does not match law",
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingBenchmarkTarget,
                format!("benchmark:{}", required.id),
                "required benchmark target is absent",
            )),
        }
    }

    let evidence_ids: BTreeSet<String> = surface
        .evidence
        .iter()
        .map(|binding| binding.id.clone())
        .collect();
    for benchmark in &surface.benchmarks {
        validate_benchmark(benchmark, &evidence_ids, &mut errors);
    }

    if surface.evidence.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingEvidenceBinding,
            "evidence:*",
            "at least one evidence binding is required",
        ));
    }
    for family in REQUIRED_EVIDENCE_FAMILIES {
        if !surface
            .evidence
            .iter()
            .any(|binding| binding.family == *family)
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                format!("evidence-family:{family}"),
                "required evidence family is not bound",
            ));
        }
    }
    for binding in &surface.evidence {
        validate_evidence(binding, &mut errors);
    }

    if surface.definitions.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingDefinitionOfDone,
            "definition:*",
            "at least one definition-of-done formula is required",
        ));
    }
    for required in REQUIRED_LOCAL_DEFINITIONS {
        if surface.definition_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDefinitionOfDone,
                format!("definition:{required}"),
                "required local/global definition-of-done formula is absent",
            ));
        }
    }
    for definition in &surface.definitions {
        validate_definition(definition, &evidence_ids, &mut errors);
    }

    let benchmark_ids: BTreeSet<String> = surface
        .benchmarks
        .iter()
        .map(|benchmark| benchmark.id.clone())
        .collect();
    let definition_ids: BTreeSet<String> = surface
        .definitions
        .iter()
        .map(|definition| definition.id.clone())
        .collect();
    if surface.closures.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            "closure:*",
            "at least one closure formula is required",
        ));
    }
    for formula in &surface.closures {
        validate_closure_formula(
            formula,
            &benchmark_ids,
            &evidence_ids,
            &definition_ids,
            &mut errors,
        );
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_BENCHMARK_EVIDENCE_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "benchmark-evidence:text",
                format!("forbidden benchmark/evidence phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_benchmark(
    benchmark: &BenchmarkTarget,
    evidence_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = benchmark.canonical_identity();
    if !BENCHMARK_METRICS.contains(&benchmark.metric.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidBenchmarkTarget,
            location.clone(),
            format!("unsupported benchmark metric {}", benchmark.metric),
        ));
    }
    if weak_value(&benchmark.target)
        || benchmark.target.contains("~")
        || benchmark.target.contains("approx")
    {
        errors.push(ValidationError::reject(
            ErrorCode::BenchmarkTargetUnstable,
            location.clone(),
            "benchmark target must be exact and deterministic",
        ));
    }
    if weak_value(&benchmark.method) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidBenchmarkTarget,
            location.clone(),
            "benchmark method must be concrete",
        ));
    }
    if !STABILITY_TOKENS.contains(&benchmark.stability.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::BenchmarkTargetUnstable,
            location.clone(),
            format!("unsupported stability token {}", benchmark.stability),
        ));
    }
    for evidence_id in &benchmark.evidence {
        if !evidence_ids.contains(evidence_id) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("unknown benchmark evidence binding {evidence_id}"),
            ));
        }
    }
}

fn validate_evidence(binding: &EvidenceBinding, errors: &mut Vec<ValidationError>) {
    let location = binding.canonical_identity();
    if !REQUIRED_EVIDENCE_FAMILIES.contains(&binding.family.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEvidenceBinding,
            location.clone(),
            format!("unsupported evidence family {}", binding.family),
        ));
    }
    if binding
        .artifacts
        .iter()
        .any(|artifact| weak_value(artifact) || !has_known_evidence_root(artifact))
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEvidenceBinding,
            location.clone(),
            "evidence artifacts must be concrete known-root paths",
        ));
    }
    if binding
        .receipts
        .iter()
        .all(|receipt| !receipt.ends_with(".receipt"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::BenchmarkMissingReceipt,
            location.clone(),
            "evidence must bind at least one receipt path",
        ));
    }
    if binding.commands.is_empty() || binding.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "evidence must bind command records",
        ));
    }
    match binding.status.as_str() {
        "working_slice" | "artifact_emitted" | "execution_proven" => {}
        "closed" | "complete" | "global_complete" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location,
            "evidence binding cannot close P00",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location,
            format!("unsupported evidence status {other}"),
        )),
    }
}

fn validate_definition(
    definition: &DefinitionOfDone,
    evidence_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = definition.canonical_identity();
    if !DEFINITION_SCOPES.contains(&definition.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidDefinitionOfDone,
            location.clone(),
            format!("unsupported definition scope {}", definition.scope),
        ));
    }
    for required in [
        "implementation",
        "tests",
        "fixtures",
        "receipts",
        "commands",
    ] {
        if definition.scope == "task" && !definition.requires.iter().any(|item| item == required) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDefinitionOfDone,
                location.clone(),
                format!("task definition must require {required}"),
            ));
        }
    }
    if definition.scope == "phase"
        && !definition
            .forbids
            .iter()
            .any(|item| item == "phase_closed_until_all_p00_blockers_closed")
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            location.clone(),
            "phase definition must keep phase closure blocked",
        ));
    }
    if definition.scope == "global"
        && !definition
            .forbids
            .iter()
            .any(|item| item == "global_complete_until_all_phases_closed")
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            location.clone(),
            "global definition must keep global closure blocked",
        ));
    }
    if definition
        .allows
        .iter()
        .any(|allowed| allowed == "complete" || allowed == "closed" || allowed == "global_complete")
    {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "definition cannot allow unsupported closure status",
        ));
    }
    for evidence_id in &definition.evidence {
        if !evidence_ids.contains(evidence_id) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("unknown definition evidence binding {evidence_id}"),
            ));
        }
    }
}

fn validate_closure_formula(
    formula: &ClosureFormula,
    benchmark_ids: &BTreeSet<String>,
    evidence_ids: &BTreeSet<String>,
    definition_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = formula.canonical_identity();
    if !CLOSURE_SCOPES.contains(&formula.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureFormulaViolation,
            location.clone(),
            format!("unsupported closure scope {}", formula.scope),
        ));
    }
    if !CLOSURE_STATUSES.contains(&formula.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location.clone(),
            format!("unsupported closure formula status {}", formula.status),
        ));
    }
    if (formula.scope == "phase" || formula.scope == "global") && formula.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "phase/global closure formulas must remain blocked in P00-009",
        ));
    }
    for benchmark_id in &formula.benchmarks {
        if !benchmark_ids.contains(benchmark_id) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("unknown benchmark binding {benchmark_id}"),
            ));
        }
    }
    for evidence_id in &formula.evidence {
        if !evidence_ids.contains(evidence_id) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("unknown evidence binding {evidence_id}"),
            ));
        }
    }
    for definition_id in &formula.definitions {
        if !definition_ids.contains(definition_id) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("unknown definition binding {definition_id}"),
            ));
        }
    }
    if formula
        .receipts
        .iter()
        .all(|receipt| !receipt.ends_with(".receipt"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::BenchmarkMissingReceipt,
            location.clone(),
            "closure formula must bind receipt paths",
        ));
    }
    if formula.commands.is_empty() || formula.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location,
            "closure formula must bind command records",
        ));
    }
}

fn has_known_evidence_root(path: &str) -> bool {
    path.starts_with("ops/")
        || path.starts_with("interfaces/")
        || path.starts_with("k0/")
        || path.starts_with("fixtures/")
        || path.starts_with("goldens/")
        || path.starts_with("receipts/")
        || path.starts_with("tests/")
        || path.starts_with("src/")
}

fn weak_value(value: &str) -> bool {
    matches!(
        value,
        "none"
            | "nothing"
            | "declared_only"
            | "manual_only"
            | "documentation_only"
            | "docs_only"
            | "approximate"
            | "best_effort"
    )
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
