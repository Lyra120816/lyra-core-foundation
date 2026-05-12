use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_serialization_hashing::deterministic_semantic_serialization_hashing_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_serialization_hashing::{
    canonical_semantic_core_serialization_registry_hash, semantic_core_object_refs,
    semantic_core_payload_hash, semantic_core_record_hash, semantic_core_serialization_descriptor,
    semantic_core_serialization_family_descriptor, semantic_core_serialization_round_trip_identity,
};
use crate::p01_semantic_serialization_hashing_model::{
    SemanticObjectHashBinding, SemanticRoundTripBinding, SemanticSerializationFamilyBinding,
    SemanticSerializationHashingSurface, SemanticSerializationReceiptBinding,
};

pub const P01_SEMANTIC_SERIALIZATION_HASHING_CONTRACT: &str =
    "LYRA-P01-SEMANTIC-SERIALIZATION-HASHING v1";
pub const REQUIRED_SEMANTIC_SERIALIZATION_RULES: &[&str] = &[
    "semantic_core_objects_have_single_canonical_text_form",
    "every_semantic_core_object_has_hash_domain",
    "object_refs_are_family_scoped",
    "payload_hashes_use_labeled_fnv1a128",
    "record_hashes_bind_payload_hash_and_comparison_key",
    "round_trip_identity_is_explicit",
    "record_order_is_ascii_sorted",
    "all_hash_inputs_are_local",
    "registry_hash_binds_all_object_rows",
    "receipts_bind_serialization_hashing_suite",
    "no_network_dependency",
    "no_probabilistic_hashing_truth",
    "no_hidden_randomness",
    "no_placeholder_serialization",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES: &[&str] = &[
    "semantic_atom",
    "core_ir",
    "semantic_object",
    "semantic_identity",
    "reference_literal",
    "reference_composition",
    "reference_eval_seed",
    "symbolic_equality_rule",
    "symbolic_equivalence_class",
    "symbolic_normalization_case",
    "symbolic_substitution_case",
    "error_object",
    "challenge_object",
    "evidence_object",
    "object_link",
];
pub const REQUIRED_SEMANTIC_SERIALIZATION_RECEIPTS: &[&str] =
    &["receipt_semantic_serialization_hashing"];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_SEMANTIC_SERIALIZATION_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic hashing truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic hashing", ErrorCode::ProbabilisticTruthAllowed),
    ("random hash", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder serialization", ErrorCode::PlaceholderAllowed),
    ("stub serialization", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_serialization_hashing_surface(
    input: &str,
) -> Result<SemanticSerializationHashingSurface, Vec<ValidationError>> {
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
            "no semantic serialization hashing lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_SERIALIZATION_HASHING_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_SERIALIZATION_HASHING_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut serializers = Vec::new();
    let mut object_hashes = Vec::new();
    let mut round_trips = Vec::new();
    let mut receipts = Vec::new();

    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_serializers = BTreeSet::new();
    let mut seen_object_hashes = BTreeSet::new();
    let mut seen_round_trips = BTreeSet::new();
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
            "serializer" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidCanonicalModel,
                        format!("line:{line_number:03}"),
                        "serializer missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_serializers.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateCanonicalModel,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid serializer {id}"),
                    ));
                    continue;
                }
                serializers.push(SemanticSerializationFamilyBinding {
                    line_number,
                    id: id.to_string(),
                    owner_root: field(&fields, "owner"),
                    serializer: field(&fields, "serializer"),
                    hash_domain: field(&fields, "hash_domain"),
                    registry: field(&fields, "registry"),
                    status: field(&fields, "status"),
                });
            }
            "object_hash" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "object_hash missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_object_hashes.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid object_hash {id}"),
                    ));
                    continue;
                }
                object_hashes.push(SemanticObjectHashBinding {
                    line_number,
                    id: id.to_string(),
                    family: field(&fields, "family"),
                    object_ref: field(&fields, "object_ref"),
                    payload_hash: field(&fields, "payload_hash"),
                    record_hash: field(&fields, "record_hash"),
                    comparison_key: field(&fields, "comparison_key"),
                    status: field(&fields, "status"),
                });
            }
            "round_trip" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "round_trip missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_round_trips.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid round_trip {id}"),
                    ));
                    continue;
                }
                round_trips.push(SemanticRoundTripBinding {
                    line_number,
                    id: id.to_string(),
                    object_ref: field(&fields, "object_ref"),
                    text_identity: field(&fields, "text_identity"),
                    hash_identity: field(&fields, "hash_identity"),
                    law: field(&fields, "law"),
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
                receipts.push(SemanticSerializationReceiptBinding {
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
    Ok(SemanticSerializationHashingSurface {
        header,
        phase,
        task,
        status,
        rules,
        serializers,
        object_hashes,
        round_trips,
        receipts,
    })
}

pub fn validate_semantic_serialization_hashing_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    let lower = input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_SERIALIZATION_TEXT {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden-token",
                format!("forbidden token {needle}"),
            ));
        }
    }
    let parsed = match parse_semantic_serialization_hashing_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_semantic_serialization_hashing_surface_model(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_serialization_hashing_surface_model(
    surface: &SemanticSerializationHashingSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-008" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-008 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }

    for required in REQUIRED_SEMANTIC_SERIALIZATION_RULES {
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
                "missing semantic serialization rule",
            )),
        }
    }
    for required in REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES {
        if surface.serializer_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("serializer:{required}"),
                "missing semantic serialization family",
            ));
        }
    }
    for object_ref in semantic_core_object_refs() {
        if surface.object_hash_by_ref(object_ref).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("object_hash:{object_ref}"),
                "missing semantic core object hash binding",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES {
        let round_trip_id = format!("round_trip_{required}");
        if surface.round_trip_by_id(&round_trip_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("round_trip:{round_trip_id}"),
                "missing serialization round trip binding",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_SERIALIZATION_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "missing semantic serialization receipt",
            ));
        }
    }

    for binding in &surface.serializers {
        validate_status(
            "serializer",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_core_serialization_family_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown serialization family {}", binding.id),
            ));
            continue;
        };
        if binding.owner_root != descriptor.owner_root
            || binding.serializer != descriptor.serializer
            || binding.hash_domain != descriptor.hash_domain
            || binding.registry != descriptor.source_registry
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("serializer descriptor drift {}", binding.id),
            ));
        }
    }

    for binding in &surface.object_hashes {
        validate_status(
            "object_hash",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_core_serialization_descriptor(&binding.object_ref) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic object ref {}", binding.object_ref),
            ));
            continue;
        };
        if binding.family != descriptor.family
            || binding.comparison_key != descriptor.comparison_key
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("object hash descriptor drift {}", binding.object_ref),
            ));
        }
        let expected_payload_hash = semantic_core_payload_hash(descriptor);
        let expected_record_hash = semantic_core_record_hash(descriptor);
        if binding.payload_hash != expected_payload_hash {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", binding.line_number),
                format!("payload hash mismatch {}", binding.object_ref),
            ));
        }
        if binding.record_hash != expected_record_hash {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", binding.line_number),
                format!("record hash mismatch {}", binding.object_ref),
            ));
        }
    }

    for binding in &surface.round_trips {
        validate_status(
            "round_trip",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_core_serialization_descriptor(&binding.object_ref) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown round trip object ref {}", binding.object_ref),
            ));
            continue;
        };
        let expected_id = format!("round_trip_{}", descriptor.family);
        if binding.id != expected_id
            || binding.text_identity != "true"
            || binding.hash_identity != "true"
            || binding.law != "serialize_parse_hash_identity"
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("round trip descriptor drift {}", binding.id),
            ));
        }
        match semantic_core_serialization_round_trip_identity(&binding.object_ref) {
            Ok(true) => {}
            Ok(false) => errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("round trip identity failed {}", binding.object_ref),
            )),
            Err(error) => errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                format!("line:{:03}", binding.line_number),
                format!("round trip error {error:?}"),
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
        if binding.id == "receipt_semantic_serialization_hashing" {
            if binding.path != "receipts/p01/pass_0037_semantic_serialization_hashing.receipt" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidProofBinding,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt path {}", binding.path),
                ));
            }
            if binding.target != "P01-008" {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt target {}", binding.target),
                ));
            }
        }
    }

    let family_rows: Vec<(String, String, String, String, String, String)> = surface
        .serializers
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.owner_root.clone(),
                item.serializer.clone(),
                item.hash_domain.clone(),
                item.registry.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let object_hash_rows: Vec<(String, String, String, String, String, String, String)> = surface
        .object_hashes
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family.clone(),
                item.object_ref.clone(),
                item.payload_hash.clone(),
                item.record_hash.clone(),
                item.comparison_key.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let round_trip_rows: Vec<(String, String, String, String, String, String)> = surface
        .round_trips
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.object_ref.clone(),
                item.text_identity.clone(),
                item.hash_identity.clone(),
                item.law.clone(),
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
    let suite = deterministic_semantic_serialization_hashing_suite_report(
        &family_rows,
        &object_hash_rows,
        &round_trip_rows,
        &receipt_rows,
    );
    if suite.family_count < REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES.len()
        || suite.object_hash_count < semantic_core_object_refs().len()
        || suite.round_trip_count < REQUIRED_SEMANTIC_SERIALIZATION_FAMILIES.len()
        || suite.receipt_count < REQUIRED_SEMANTIC_SERIALIZATION_RECEIPTS.len()
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !canonical_semantic_core_serialization_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "suite",
            "semantic serialization hashing suite report is incomplete or unhashable",
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
