use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_canonical_model::{
    CanonicalDataModel, CanonicalModelProof, CanonicalModelSurface, FieldBinding, ModelBinding,
    SchemaBinding,
};

pub const P00_CANONICAL_MODEL_CONTRACT: &str = "LYRA-P00-CANONICAL-MODEL v1";

pub const REQUIRED_CANONICAL_MODEL_RULES: &[&str] = &[
    "canonical_schema_identity_required",
    "stable_field_order_required",
    "explicit_required_fields_required",
    "model_source_task_required",
    "receipt_bound_models_required",
    "no_ambient_model_defaults",
    "no_unordered_maps_in_canonical_paths",
    "phase_open_until_all_models_proven",
];

pub const REQUIRED_CANONICAL_MODELS: &[&str] = &[
    "surface_model",
    "verdict_model",
    "receipt_model",
    "authority_model",
    "identity_model",
    "control_model",
    "evidence_model",
    "formal_semantics_model",
];

pub const REQUIRED_SCHEMA_BINDINGS: &[&str] = &[
    "canonical_surface_schema",
    "validation_verdict_schema",
    "receipt_schema",
    "authority_order_schema",
    "identity_law_schema",
    "control_surface_schema",
    "benchmark_evidence_schema",
    "formal_semantics_schema",
];

pub const REQUIRED_FIELD_BINDINGS: &[&str] = &[
    "surface_header",
    "surface_phase",
    "surface_task",
    "surface_status",
    "surface_rules",
    "verdict_accepted",
    "verdict_errors",
    "receipt_input_hash",
    "receipt_canonical_hash",
    "receipt_verdict_hash",
    "receipt_receipt_hash",
    "authority_rank",
    "identity_task_id",
    "control_frontier",
    "evidence_receipt",
    "formal_semantic_domain",
];

pub const REQUIRED_MODEL_BINDINGS: &[&str] = &[
    "surface_to_verdict_model",
    "verdict_to_receipt_model",
    "authority_to_control_model",
    "identity_to_canon_model",
    "evidence_to_closure_model",
    "formal_semantics_to_model_law",
];

pub const REQUIRED_CANONICAL_MODEL_PROOFS: &[&str] = &[
    "canonical_model_local",
    "stable_order_proof",
    "receipt_model_proof",
    "p00_phase_open",
];

const ALLOWED_OWNER_ROOTS: &[&str] = &["k0", "interfaces", "ops"];
const ALLOWED_MODEL_STATUSES: &[&str] = &["working_slice", "execution_proven", "artifact_emitted"];
const ALLOWED_SCHEMA_SCOPES: &[&str] = &[
    "surface",
    "verdict",
    "receipt",
    "authority",
    "identity",
    "control",
    "evidence",
    "semantics",
];
const ALLOWED_FIELD_KINDS: &[&str] = &[
    "scalar", "list", "map", "object", "path", "hash", "verdict", "rank",
];
const ALLOWED_REQUIRED_TOKENS: &[&str] = &["yes", "no"];
const ALLOWED_PROOF_SCOPES: &[&str] = &["task", "schema", "receipt", "phase"];
const EXECUTED_TASKS: &[&str] = &[
    "P00-001", "P00-002", "P00-003", "P00-004", "P00-005", "P00-006", "P00-007", "P00-008",
    "P00-009", "P00-010", "P00-011", "P00-012", "P00-013", "P00-014",
];

const FORBIDDEN_MODEL_TEXT: &[(&str, ErrorCode)] = &[
    ("ambient default", ErrorCode::CanonicalModelDriftAccepted),
    ("ambient time", ErrorCode::CanonicalModelDriftAccepted),
    ("random order", ErrorCode::HiddenRandomnessAllowed),
    ("unordered map", ErrorCode::CanonicalModelDriftAccepted),
    ("host order", ErrorCode::CanonicalModelDriftAccepted),
    ("network fetch", ErrorCode::AmbientNetworkAllowed),
    ("schema todo", ErrorCode::PlaceholderAllowed),
    ("model placeholder", ErrorCode::PlaceholderAllowed),
    ("manual only", ErrorCode::InvalidCanonicalModel),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_canonical_model_surface(
    input: &str,
) -> Result<CanonicalModelSurface, Vec<ValidationError>> {
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
            "no canonical-model lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_CANONICAL_MODEL_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_CANONICAL_MODEL_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut models = Vec::new();
    let mut schemas = Vec::new();
    let mut fields = Vec::new();
    let mut bindings = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_models = BTreeSet::new();
    let mut seen_schemas = BTreeSet::new();
    let mut seen_fields = BTreeSet::new();
    let mut seen_bindings = BTreeSet::new();
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
                    "canonical model rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(model_id) = left.strip_prefix("model:") {
            if !is_symbolic_name(model_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidCanonicalModel,
                    format!("line:{line_number:03}"),
                    format!("invalid canonical model identity {model_id}"),
                ));
                continue;
            }
            if !seen_models.insert(model_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateCanonicalModel,
                    format!("model:{model_id}"),
                    "canonical model identity must be unique",
                ));
                continue;
            }
            match parse_model(line_number, model_id, value) {
                Ok(item) => models.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(schema_id) = left.strip_prefix("schema:") {
            if !is_symbolic_name(schema_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSchemaBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid schema identity {schema_id}"),
                ));
                continue;
            }
            if !seen_schemas.insert(schema_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateSchemaBinding,
                    format!("schema:{schema_id}"),
                    "schema binding identity must be unique",
                ));
                continue;
            }
            match parse_schema(line_number, schema_id, value) {
                Ok(item) => schemas.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(field_id) = left.strip_prefix("field:") {
            if !is_symbolic_name(field_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidFieldBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid field identity {field_id}"),
                ));
                continue;
            }
            if !seen_fields.insert(field_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateFieldBinding,
                    format!("field:{field_id}"),
                    "field binding identity must be unique",
                ));
                continue;
            }
            match parse_field(line_number, field_id, value) {
                Ok(item) => fields.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(binding_id) = left.strip_prefix("binding:") {
            if !is_symbolic_name(binding_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidModelBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid model binding identity {binding_id}"),
                ));
                continue;
            }
            if !seen_bindings.insert(binding_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("binding:{binding_id}"),
                    "model binding identity must be unique",
                ));
                continue;
            }
            match parse_binding(line_number, binding_id, value) {
                Ok(item) => bindings.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidCanonicalModel,
                    format!("line:{line_number:03}"),
                    format!("invalid proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("proof:{proof_id}"),
                    "canonical model proof identity must be unique",
                ));
                continue;
            }
            match parse_proof(line_number, proof_id, value) {
                Ok(item) => proofs.push(item),
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
                format!("unknown canonical-model field {left}"),
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
                "task=P00-014 is required",
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
        Ok(CanonicalModelSurface {
            header,
            phase,
            task,
            status,
            rules,
            models,
            schemas,
            fields,
            bindings,
            proofs,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_canonical_model_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = match canonical_surface_text(input) {
        Ok(text) => text,
        Err(error) => {
            let verdict = Verdict::rejected(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "byte-stream",
                format!("{error:?}"),
            )]);
            let receipt = build_receipt(input, "", verdict.clone());
            return (verdict, receipt);
        }
    };

    let verdict = match parse_canonical_model_surface(input) {
        Ok(surface) => validate_parsed_canonical_model_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_model(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CanonicalDataModel, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let owner_root = required_string_field(line_number, &mut fields, "owner_root")?;
    let source_task = required_string_field(line_number, &mut fields, "source_task")?;
    let schema = required_string_field(line_number, &mut fields, "schema")?;
    let canonical_order = required_list_field(line_number, &mut fields, "canonical_order")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(CanonicalDataModel {
        line_number,
        id: id.to_string(),
        owner_root,
        source_task,
        schema,
        canonical_order,
        status,
    })
}

fn parse_schema(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SchemaBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let model = required_string_field(line_number, &mut fields, "model")?;
    let schema_fields = required_list_field(line_number, &mut fields, "fields")?;
    let required = required_list_field(line_number, &mut fields, "required")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(SchemaBinding {
        line_number,
        id: id.to_string(),
        scope,
        model,
        fields: schema_fields,
        required,
        forbids,
        status,
    })
}

fn parse_field(line_number: usize, id: &str, value: &str) -> Result<FieldBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let model = required_string_field(line_number, &mut fields, "model")?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let required = required_string_field(line_number, &mut fields, "required")?;
    let order = required_string_field(line_number, &mut fields, "order")?;
    let canonical = required_string_field(line_number, &mut fields, "canonical")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(FieldBinding {
        line_number,
        id: id.to_string(),
        model,
        kind,
        required,
        order,
        canonical,
        status,
    })
}

fn parse_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ModelBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let from = required_string_field(line_number, &mut fields, "from")?;
    let to = required_string_field(line_number, &mut fields, "to")?;
    let through = required_string_field(line_number, &mut fields, "through")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ModelBinding {
        line_number,
        id: id.to_string(),
        from,
        to,
        through,
        receipts,
        commands,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CanonicalModelProof, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let models = required_list_field(line_number, &mut fields, "models")?;
    let schemas = required_list_field(line_number, &mut fields, "schemas")?;
    let proof_fields = required_list_field(line_number, &mut fields, "fields")?;
    let bindings = required_list_field(line_number, &mut fields, "bindings")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(CanonicalModelProof {
        line_number,
        id: id.to_string(),
        scope,
        models,
        schemas,
        fields: proof_fields,
        bindings,
        receipts,
        commands,
        status,
        forbids,
    })
}

fn validate_parsed_canonical_model_surface(
    surface: &CanonicalModelSurface,
    raw_input: &str,
) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            "canonical model law is scoped to P00",
        ));
    }
    if surface.task != "P00-014" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            "canonical model law must bind P00-014",
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-014 may only claim working_slice in this pass",
        ));
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_MODEL_TEXT {
        if lowered.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "canonical_model:text",
                format!("forbidden canonical model phrase detected: {token}"),
            ));
        }
    }

    for required in REQUIRED_CANONICAL_MODEL_RULES {
        match surface.rule_value(required) {
            Some(value) if value.starts_with("required:") || value.starts_with("forbidden:") => {}
            Some(_) => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "canonical model rule must be explicit required: or forbidden:",
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "required canonical model rule missing",
            )),
        }
    }

    for required in REQUIRED_CANONICAL_MODELS {
        if surface.model_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("model:{required}"),
                "required canonical model missing",
            ));
        }
    }
    for required in REQUIRED_SCHEMA_BINDINGS {
        if surface.schema_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingSchemaBinding,
                format!("schema:{required}"),
                "required schema binding missing",
            ));
        }
    }
    for required in REQUIRED_FIELD_BINDINGS {
        if surface.field_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingFieldBinding,
                format!("field:{required}"),
                "required field binding missing",
            ));
        }
    }
    for required in REQUIRED_MODEL_BINDINGS {
        if surface.binding_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("binding:{required}"),
                "required model binding missing",
            ));
        }
    }
    for required in REQUIRED_CANONICAL_MODEL_PROOFS {
        if surface.proof_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                format!("proof:{required}"),
                "required canonical model proof missing",
            ));
        }
    }

    let model_ids: BTreeSet<String> = surface.models.iter().map(|item| item.id.clone()).collect();
    let schema_ids: BTreeSet<String> = surface.schemas.iter().map(|item| item.id.clone()).collect();
    let field_ids: BTreeSet<String> = surface.fields.iter().map(|item| item.id.clone()).collect();
    let binding_ids: BTreeSet<String> = surface
        .bindings
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let mut field_order_by_model: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for model in &surface.models {
        validate_model(model, &mut errors);
    }
    for schema in &surface.schemas {
        validate_schema(schema, &model_ids, &field_ids, &mut errors);
    }
    for field in &surface.fields {
        validate_field(field, &model_ids, &mut field_order_by_model, &mut errors);
    }
    for binding in &surface.bindings {
        validate_binding(binding, &model_ids, &schema_ids, &mut errors);
    }
    for proof in &surface.proofs {
        validate_proof(
            proof,
            &model_ids,
            &schema_ids,
            &field_ids,
            &binding_ids,
            &mut errors,
        );
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_model(model: &CanonicalDataModel, errors: &mut Vec<ValidationError>) {
    let location = model.canonical_identity();
    if !ALLOWED_OWNER_ROOTS.contains(&model.owner_root.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location.clone(),
            format!(
                "canonical model owner root must be k0/interfaces/ops: {}",
                model.owner_root
            ),
        ));
    }
    if !EXECUTED_TASKS.contains(&model.source_task.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location.clone(),
            format!(
                "source task is not in executed P00 chain: {}",
                model.source_task
            ),
        ));
    }
    if !(model.schema.starts_with("interfaces/p00/src/")
        || model.schema.starts_with("k0/determinism/src/")
        || model.schema.starts_with("ops/p00/src/"))
        || !model.schema.ends_with(".rs")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location.clone(),
            format!(
                "model schema must bind owner-root Rust source: {}",
                model.schema
            ),
        ));
    }
    if model.canonical_order.len() < 2
        || model
            .canonical_order
            .iter()
            .any(|item| weak_value(item) || !is_symbolic_name(item))
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location.clone(),
            "canonical_order must contain concrete symbolic field names",
        ));
    }
    if has_duplicates(&model.canonical_order) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location.clone(),
            "canonical_order fields must be unique",
        ));
    }
    if !ALLOWED_MODEL_STATUSES.contains(&model.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location,
            format!("unsupported model status {}", model.status),
        ));
    }
}

fn validate_schema(
    schema: &SchemaBinding,
    model_ids: &BTreeSet<String>,
    field_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = schema.canonical_identity();
    if !ALLOWED_SCHEMA_SCOPES.contains(&schema.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSchemaBinding,
            location.clone(),
            format!("unsupported schema scope {}", schema.scope),
        ));
    }
    if !model_ids.contains(&schema.model) {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelUnbound,
            location.clone(),
            format!("schema references unknown model {}", schema.model),
        ));
    }
    if schema.fields.is_empty() || schema.fields.iter().any(|field| weak_value(field)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSchemaBinding,
            location.clone(),
            "schema fields must be concrete",
        ));
    }
    for field in &schema.fields {
        if !field_ids.contains(field) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                location.clone(),
                format!("schema references unknown field {field}"),
            ));
        }
    }
    for required in &schema.required {
        if !schema.fields.iter().any(|field| field == required) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSchemaBinding,
                location.clone(),
                format!("required field {required} is not listed in schema fields"),
            ));
        }
    }
    if schema.forbids.is_empty() || schema.forbids.iter().any(|item| weak_value(item)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSchemaBinding,
            location.clone(),
            "schema forbid list must be concrete",
        ));
    }
    if !ALLOWED_MODEL_STATUSES.contains(&schema.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSchemaBinding,
            location,
            format!("unsupported schema status {}", schema.status),
        ));
    }
}

fn validate_field(
    field: &FieldBinding,
    model_ids: &BTreeSet<String>,
    field_order_by_model: &mut BTreeMap<String, BTreeSet<String>>,
    errors: &mut Vec<ValidationError>,
) {
    let location = field.canonical_identity();
    if !model_ids.contains(&field.model) {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelUnbound,
            location.clone(),
            format!("field references unknown model {}", field.model),
        ));
    }
    if !ALLOWED_FIELD_KINDS.contains(&field.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFieldBinding,
            location.clone(),
            format!("unsupported field kind {}", field.kind),
        ));
    }
    if !ALLOWED_REQUIRED_TOKENS.contains(&field.required.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFieldBinding,
            location.clone(),
            format!("required must be yes/no: {}", field.required),
        ));
    }
    if !stable_order_token(&field.order) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFieldBinding,
            location.clone(),
            format!(
                "field order must be stable three-digit token: {}",
                field.order
            ),
        ));
    } else {
        let orders = field_order_by_model.entry(field.model.clone()).or_default();
        if !orders.insert(field.order.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                location.clone(),
                format!(
                    "duplicate field order {} in model {}",
                    field.order, field.model
                ),
            ));
        }
    }
    if weak_value(&field.canonical) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFieldBinding,
            location.clone(),
            "canonical field rule must be concrete",
        ));
    }
    if !ALLOWED_MODEL_STATUSES.contains(&field.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFieldBinding,
            location,
            format!("unsupported field status {}", field.status),
        ));
    }
}

fn validate_binding(
    binding: &ModelBinding,
    model_ids: &BTreeSet<String>,
    schema_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = binding.canonical_identity();
    if !model_ids.contains(&binding.from) {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelUnbound,
            location.clone(),
            format!("binding references unknown from model {}", binding.from),
        ));
    }
    if !model_ids.contains(&binding.to) {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelUnbound,
            location.clone(),
            format!("binding references unknown to model {}", binding.to),
        ));
    }
    if !schema_ids.contains(&binding.through) {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelUnbound,
            location.clone(),
            format!("binding references unknown schema {}", binding.through),
        ));
    }
    if binding.from == binding.to {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            location.clone(),
            "model binding must transform between distinct models",
        ));
    }
    if binding.receipts.is_empty()
        || binding
            .receipts
            .iter()
            .any(|receipt| !receipt_path(receipt))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "model binding must bind canonical P00 receipt paths",
        ));
    }
    if binding.commands.is_empty() || binding.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "model binding must bind command records",
        ));
    }
    if !ALLOWED_MODEL_STATUSES.contains(&binding.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            location,
            format!("unsupported model binding status {}", binding.status),
        ));
    }
}

fn validate_proof(
    proof: &CanonicalModelProof,
    model_ids: &BTreeSet<String>,
    schema_ids: &BTreeSet<String>,
    field_ids: &BTreeSet<String>,
    binding_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = proof.canonical_identity();
    if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location.clone(),
            format!("unsupported canonical model proof scope {}", proof.scope),
        ));
    }
    if !ALLOWED_MODEL_STATUSES.contains(&proof.status.as_str()) && proof.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location.clone(),
            format!("unsupported proof status {}", proof.status),
        ));
    }
    if proof.scope == "phase" && proof.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "phase model proof must remain blocked until all P00 tasks close",
        ));
    }
    for model in &proof.models {
        if !model_ids.contains(model) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                location.clone(),
                format!("unknown proof model {model}"),
            ));
        }
    }
    for schema in &proof.schemas {
        if !schema_ids.contains(schema) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                location.clone(),
                format!("unknown proof schema {schema}"),
            ));
        }
    }
    for field in &proof.fields {
        if !field_ids.contains(field) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                location.clone(),
                format!("unknown proof field {field}"),
            ));
        }
    }
    for binding in &proof.bindings {
        if !binding_ids.contains(binding) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                location.clone(),
                format!("unknown proof binding {binding}"),
            ));
        }
    }
    if proof.receipts.is_empty() || proof.receipts.iter().any(|receipt| !receipt_path(receipt)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "canonical model proof must bind canonical P00 receipts",
        ));
    }
    if proof.commands.is_empty() || proof.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "canonical model proof must bind command records",
        ));
    }
    if proof.forbids.is_empty() || proof.forbids.iter().any(|item| weak_value(item)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            location.clone(),
            "canonical model proof forbid list must be concrete",
        ));
    }
    if proof.id == "stable_order_proof" {
        for required in [
            "surface_header",
            "surface_phase",
            "surface_task",
            "receipt_input_hash",
            "receipt_receipt_hash",
        ] {
            if !proof.fields.iter().any(|field| field == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingFieldBinding,
                    location.clone(),
                    format!("stable order proof misses field {required}"),
                ));
            }
        }
    }
    if proof.id == "receipt_model_proof" {
        for required in ["receipt_model", "verdict_model"] {
            if !proof.models.iter().any(|model| model == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingCanonicalModel,
                    location.clone(),
                    format!("receipt model proof misses model {required}"),
                ));
            }
        }
    }
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

fn has_duplicates(values: &[String]) -> bool {
    let mut seen = BTreeSet::new();
    values.iter().any(|value| !seen.insert(value))
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
