use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_adversarial_corpus::deterministic_semantic_adversarial_corpus_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_adversarial_corpus::{
    canonical_semantic_adversarial_corpus_registry_hash, semantic_adversarial_all_case_ids,
    semantic_adversarial_harness_descriptor, semantic_ambiguity_probe_descriptor,
    semantic_collision_probe_descriptor, semantic_malformed_object_descriptor,
};
use crate::p01_semantic_adversarial_corpus_model::{
    SemanticAdversarialCorpusSurface, SemanticAdversarialHarnessBinding,
    SemanticAdversarialReceiptBinding, SemanticAmbiguityProbeBinding,
    SemanticCollisionProbeBinding, SemanticMalformedObjectBinding,
};

pub const P01_SEMANTIC_ADVERSARIAL_CORPUS_CONTRACT: &str =
    "LYRA-P01-SEMANTIC-ADVERSARIAL-CORPUS v1";

pub const REQUIRED_SEMANTIC_ADVERSARIAL_RULES: &[&str] = &[
    "collision_attempts_are_negative",
    "ambiguity_cases_resolve_or_reject_deterministically",
    "malformed_objects_are_rejected",
    "fixtures_bind_expected_error_codes",
    "harnesses_cover_all_case_families",
    "receipts_bind_adversarial_corpus_suite",
    "no_network_dependency",
    "no_probabilistic_acceptance",
    "no_hidden_randomness",
    "no_fixture_acceptance_by_default",
    "no_global_closure_claim",
];

pub const REQUIRED_SEMANTIC_COLLISION_PROBES: &[&str] = &[
    "digest_alias_rewrite",
    "object_hash_payload_swap",
    "receipt_target_alias",
];
pub const REQUIRED_SEMANTIC_AMBIGUITY_PROBES: &[&str] = &[
    "text_ir_header_case_drift",
    "record_key_order_shadow",
    "alpha_bind_rename_shadow",
];
pub const REQUIRED_SEMANTIC_MALFORMED_OBJECTS: &[&str] = &[
    "semantic_object_missing_relation",
    "binary_ir_header_drift",
    "unknown_serializer_object_ref",
    "semantic_object_cycle",
    "ambient_network_requirement",
];
pub const REQUIRED_SEMANTIC_ADVERSARIAL_HARNESSES: &[&str] = &[
    "collision_guard_harness",
    "ambiguity_guard_harness",
    "malformed_object_guard_harness",
    "full_semantic_core_adversarial_harness",
];
pub const REQUIRED_SEMANTIC_ADVERSARIAL_RECEIPTS: &[&str] =
    &["receipt_semantic_adversarial_corpus"];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_SEMANTIC_ADVERSARIAL_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic acceptance allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic corpus", ErrorCode::ProbabilisticTruthAllowed),
    ("random corpus", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("stub corpus", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_adversarial_corpus_surface(
    input: &str,
) -> Result<SemanticAdversarialCorpusSurface, Vec<ValidationError>> {
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
            "no semantic adversarial corpus lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_ADVERSARIAL_CORPUS_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_ADVERSARIAL_CORPUS_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut collision_probes = Vec::new();
    let mut ambiguity_probes = Vec::new();
    let mut malformed_objects = Vec::new();
    let mut harnesses = Vec::new();
    let mut receipts = Vec::new();

    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_collision = BTreeSet::new();
    let mut seen_ambiguity = BTreeSet::new();
    let mut seen_malformed = BTreeSet::new();
    let mut seen_harnesses = BTreeSet::new();
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
            "collision_probe" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "collision_probe missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_collision.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid collision_probe {id}"),
                    ));
                    continue;
                }
                collision_probes.push(SemanticCollisionProbeBinding {
                    line_number,
                    id: id.to_string(),
                    target_family: field(&fields, "target_family"),
                    left_ref: field(&fields, "left_ref"),
                    right_ref: field(&fields, "right_ref"),
                    collision_domain: field(&fields, "collision_domain"),
                    guard: field(&fields, "guard"),
                    expected_error: field(&fields, "expected_error"),
                    fixture_path: field(&fields, "fixture"),
                    status: field(&fields, "status"),
                });
            }
            "ambiguity_probe" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "ambiguity_probe missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_ambiguity.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid ambiguity_probe {id}"),
                    ));
                    continue;
                }
                ambiguity_probes.push(SemanticAmbiguityProbeBinding {
                    line_number,
                    id: id.to_string(),
                    target_family: field(&fields, "target_family"),
                    ambiguous_surface: field(&fields, "ambiguous_surface"),
                    deterministic_resolution: field(&fields, "deterministic_resolution"),
                    guard: field(&fields, "guard"),
                    expected_error: field(&fields, "expected_error"),
                    fixture_path: field(&fields, "fixture"),
                    status: field(&fields, "status"),
                });
            }
            "malformed_object" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "malformed_object missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_malformed.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid malformed_object {id}"),
                    ));
                    continue;
                }
                malformed_objects.push(SemanticMalformedObjectBinding {
                    line_number,
                    id: id.to_string(),
                    target_family: field(&fields, "target_family"),
                    object_ref: field(&fields, "object_ref"),
                    malformed_field: field(&fields, "malformed_field"),
                    rejection_law: field(&fields, "rejection_law"),
                    expected_error: field(&fields, "expected_error"),
                    fixture_path: field(&fields, "fixture"),
                    status: field(&fields, "status"),
                });
            }
            "harness" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "harness missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_harnesses.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid harness {id}"),
                    ));
                    continue;
                }
                harnesses.push(SemanticAdversarialHarnessBinding {
                    line_number,
                    id: id.to_string(),
                    validator: field(&fields, "validator"),
                    case_ids: field(&fields, "case_ids"),
                    coverage: field(&fields, "coverage"),
                    receipt_ref: field(&fields, "receipt_ref"),
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
                receipts.push(SemanticAdversarialReceiptBinding {
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

    Ok(SemanticAdversarialCorpusSurface {
        header,
        phase,
        task,
        status,
        rules,
        collision_probes,
        ambiguity_probes,
        malformed_objects,
        harnesses,
        receipts,
    })
}

pub fn validate_semantic_adversarial_corpus_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    let lower = input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_ADVERSARIAL_TEXT {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden-token",
                format!("forbidden token {needle}"),
            ));
        }
    }
    let parsed = match parse_semantic_adversarial_corpus_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_semantic_adversarial_corpus_surface_model(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_adversarial_corpus_surface_model(
    surface: &SemanticAdversarialCorpusSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-009" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-009 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }

    for required in REQUIRED_SEMANTIC_ADVERSARIAL_RULES {
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
                "missing semantic adversarial corpus rule",
            )),
        }
    }
    for required in REQUIRED_SEMANTIC_COLLISION_PROBES {
        if surface.collision_probe_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("collision_probe:{required}"),
                "missing collision probe",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_AMBIGUITY_PROBES {
        if surface.ambiguity_probe_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("ambiguity_probe:{required}"),
                "missing ambiguity probe",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_MALFORMED_OBJECTS {
        if surface.malformed_object_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("malformed_object:{required}"),
                "missing malformed object case",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_ADVERSARIAL_HARNESSES {
        if surface.harness_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("harness:{required}"),
                "missing adversarial harness",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_ADVERSARIAL_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "missing semantic adversarial corpus receipt",
            ));
        }
    }

    for binding in &surface.collision_probes {
        validate_status(
            "collision_probe",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_collision_probe_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown collision probe {}", binding.id),
            ));
            continue;
        };
        if binding.target_family != descriptor.target_family
            || binding.left_ref != descriptor.left_ref
            || binding.right_ref != descriptor.right_ref
            || binding.collision_domain != descriptor.collision_domain
            || binding.guard != descriptor.guard
            || binding.expected_error != descriptor.expected_error
            || binding.fixture_path != descriptor.fixture_path
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("collision probe descriptor drift {}", binding.id),
            ));
        }
    }
    for binding in &surface.ambiguity_probes {
        validate_status(
            "ambiguity_probe",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_ambiguity_probe_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown ambiguity probe {}", binding.id),
            ));
            continue;
        };
        if binding.target_family != descriptor.target_family
            || binding.ambiguous_surface != descriptor.ambiguous_surface
            || binding.deterministic_resolution != descriptor.deterministic_resolution
            || binding.guard != descriptor.guard
            || binding.expected_error != descriptor.expected_error
            || binding.fixture_path != descriptor.fixture_path
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("ambiguity probe descriptor drift {}", binding.id),
            ));
        }
    }
    for binding in &surface.malformed_objects {
        validate_status(
            "malformed_object",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_malformed_object_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown malformed object {}", binding.id),
            ));
            continue;
        };
        if binding.target_family != descriptor.target_family
            || binding.object_ref != descriptor.object_ref
            || binding.malformed_field != descriptor.malformed_field
            || binding.rejection_law != descriptor.rejection_law
            || binding.expected_error != descriptor.expected_error
            || binding.fixture_path != descriptor.fixture_path
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("malformed object descriptor drift {}", binding.id),
            ));
        }
    }
    for binding in &surface.harnesses {
        validate_status(
            "harness",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_adversarial_harness_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown harness {}", binding.id),
            ));
            continue;
        };
        if binding.validator != descriptor.validator
            || binding.case_ids != descriptor.case_ids
            || binding.coverage != descriptor.coverage
            || binding.receipt_ref != descriptor.receipt_ref
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("harness descriptor drift {}", binding.id),
            ));
        }
        for case_id in binding.case_ids.split(',') {
            if !semantic_adversarial_all_case_ids().contains(&case_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidModelBinding,
                    format!("line:{:03}", binding.line_number),
                    format!("harness references unknown case {case_id}"),
                ));
            }
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
        if binding.id == "receipt_semantic_adversarial_corpus" {
            if binding.path != "receipts/p01/pass_0038_semantic_adversarial_corpus.receipt" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidProofBinding,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt path {}", binding.path),
                ));
            }
            if binding.target != "P01-009" {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt target {}", binding.target),
                ));
            }
        }
    }

    let collision_rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .collision_probes
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.target_family.clone(),
                item.left_ref.clone(),
                item.right_ref.clone(),
                item.collision_domain.clone(),
                item.guard.clone(),
                item.expected_error.clone(),
                item.fixture_path.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let ambiguity_rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .ambiguity_probes
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.target_family.clone(),
                item.ambiguous_surface.clone(),
                item.deterministic_resolution.clone(),
                item.guard.clone(),
                item.expected_error.clone(),
                item.fixture_path.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let malformed_rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .malformed_objects
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.target_family.clone(),
                item.object_ref.clone(),
                item.malformed_field.clone(),
                item.rejection_law.clone(),
                item.expected_error.clone(),
                item.fixture_path.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let harness_rows: Vec<(String, String, String, String, String, String)> = surface
        .harnesses
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.validator.clone(),
                item.case_ids.clone(),
                item.coverage.clone(),
                item.receipt_ref.clone(),
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
    let suite = deterministic_semantic_adversarial_corpus_suite_report(
        &collision_rows,
        &ambiguity_rows,
        &malformed_rows,
        &harness_rows,
        &receipt_rows,
    );
    if suite.collision_probe_count < REQUIRED_SEMANTIC_COLLISION_PROBES.len()
        || suite.ambiguity_probe_count < REQUIRED_SEMANTIC_AMBIGUITY_PROBES.len()
        || suite.malformed_object_count < REQUIRED_SEMANTIC_MALFORMED_OBJECTS.len()
        || suite.harness_count < REQUIRED_SEMANTIC_ADVERSARIAL_HARNESSES.len()
        || suite.receipt_count < REQUIRED_SEMANTIC_ADVERSARIAL_RECEIPTS.len()
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !canonical_semantic_adversarial_corpus_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "suite",
            "semantic adversarial corpus suite report is incomplete or unhashable",
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
