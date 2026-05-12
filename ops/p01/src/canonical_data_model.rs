use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_canonical_data_model::deterministic_canonical_data_model_suite_report;
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_canonical_data_model::{
    canonical_data_model_descriptor, canonical_data_model_ids, canonical_data_model_registry_hash,
    canonical_data_models_have_schema_refs, canonical_data_no_forbidden_descriptor_claims,
    canonical_data_proof_descriptor, canonical_data_proof_ids,
    canonical_data_proofs_bind_artifacts, canonical_field_descriptor, canonical_field_ids,
    canonical_fields_bind_known_models, canonical_model_bridge_descriptor,
    canonical_model_bridge_ids, canonical_model_bridges_bind_one_carrier,
    canonical_schema_descriptor, canonical_schema_ids,
    canonical_schemas_bind_known_models_and_fields, LYRA_P01_CANONICAL_DATA_CARRIER,
};
use crate::p01_canonical_data_model_model::{
    P01CanonicalDataModelBinding, P01CanonicalDataModelSurface, P01CanonicalDataProofBinding,
    P01CanonicalFieldBinding, P01CanonicalModelBridgeBinding, P01CanonicalSchemaBinding,
};

pub const P01_CANONICAL_DATA_MODEL_CONTRACT: &str = "LYRA-P01-CANONICAL-DATA-MODEL v1";
pub const REQUIRED_P01_CANONICAL_DATA_MODEL_RULES: &[&str] = &[
    "canonical_symbol_model_stable",
    "semantic_atom_model_closed",
    "core_ir_model_single_carrier",
    "model_fields_explicitly_ordered",
    "schemas_forbid_ambient_defaults",
    "bridges_bind_receipts",
    "proofs_bind_fixture_golden_receipt",
    "no_network_model_source",
    "no_probabilistic_fields",
    "no_hidden_randomness",
    "no_placeholder_models",
    "no_phase_closure_claim",
];
pub const REQUIRED_P01_CANONICAL_DATA_MODELS: &[&str] = &[
    "canonical_symbol_model",
    "semantic_atom_model",
    "core_ir_term_model",
    "core_ir_form_model",
    "semantic_object_model",
    "semantic_identity_model",
    "symbolic_equality_model",
    "semantic_receipt_model",
];
pub const REQUIRED_P01_CANONICAL_SCHEMAS: &[&str] = &[
    "canonical_symbol_schema",
    "semantic_atom_schema",
    "core_ir_term_schema",
    "core_ir_form_schema",
    "semantic_object_schema",
    "semantic_identity_schema",
    "symbolic_equality_schema",
    "semantic_receipt_schema",
];
pub const REQUIRED_P01_CANONICAL_FIELDS: &[&str] = &[
    "symbol_id",
    "symbol_namespace",
    "symbol_kind",
    "atom_family",
    "atom_name",
    "atom_capability",
    "ir_kind",
    "ir_children",
    "ir_effects",
    "ir_capabilities",
    "ir_receipt_ref",
    "object_id",
    "object_kind",
    "object_digest",
    "identity_digest",
    "equality_normal_form",
    "receipt_hash",
    "receipt_verdict",
];
pub const REQUIRED_P01_CANONICAL_MODEL_BRIDGES: &[&str] = &[
    "symbols_to_atoms",
    "atoms_to_core_ir",
    "core_ir_to_objects",
    "objects_to_identity",
    "equality_to_receipts",
    "receipts_to_proofs",
];
pub const REQUIRED_P01_CANONICAL_DATA_PROOFS: &[&str] = &[
    "symbol_model_proof",
    "atom_model_proof",
    "core_ir_model_proof",
    "object_identity_model_proof",
    "equality_receipt_model_proof",
    "p01_canonical_data_parity_proof",
];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const ALLOWED_OWNER_ROOTS: &[&str] = &["lyralang", "interfaces", "k0", "ops"];
const ALLOWED_FIELD_KINDS: &[&str] = &[
    "symbol",
    "enum",
    "capability",
    "list",
    "path",
    "object",
    "hash",
    "verdict",
];
const FORBIDDEN_CANONICAL_DATA_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("probabilistic field", ErrorCode::ProbabilisticTruthAllowed),
    ("probabilistic model", ErrorCode::ProbabilisticTruthAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    ("stochastic data", ErrorCode::ProbabilisticTruthAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("random model", ErrorCode::HiddenRandomnessAllowed),
    ("placeholder model", ErrorCode::PlaceholderAllowed),
    ("placeholder data", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("stub model", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("forked carrier", ErrorCode::SemanticDriftAccepted),
];

pub fn parse_canonical_data_model_surface(
    input: &str,
) -> Result<P01CanonicalDataModelSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => {
            return Err(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "input",
                format!("{error:?}"),
            )])
        }
    };
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "empty canonical data model surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P01_CANONICAL_DATA_MODEL_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_CANONICAL_DATA_MODEL_CONTRACT}"),
        ));
    }

    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut models = Vec::new();
    let mut schemas = Vec::new();
    let mut fields = Vec::new();
    let mut bridges = Vec::new();
    let mut proofs = Vec::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
        if let Some(value) = line.strip_prefix("phase=") {
            phase = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("task=") {
            task = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("status=") {
            status = Some(value.to_string());
        } else if let Some((name, value)) = line
            .strip_prefix("rule:")
            .and_then(|value| value.split_once('='))
        {
            if rules.insert(name.to_string(), value.to_string()).is_some() {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    format!("line:{line_number:03}"),
                    format!("duplicate rule {name}"),
                ));
            }
        } else if let Some(value) = line.strip_prefix("model=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &[
                    "id", "scope", "owner", "source", "schema", "order", "status",
                ],
                "model",
                line_number,
                &mut errors,
            );
            models.push(P01CanonicalDataModelBinding {
                line_number,
                id: field(&field_map, "id"),
                scope: field(&field_map, "scope"),
                owner_root: field(&field_map, "owner"),
                source_task: field(&field_map, "source"),
                schema_ref: field(&field_map, "schema"),
                canonical_order: list_field(&field_map, "order"),
                status: field(&field_map, "status"),
            });
        } else if let Some(value) = line.strip_prefix("schema=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &["id", "model", "fields", "required", "forbids", "status"],
                "schema",
                line_number,
                &mut errors,
            );
            schemas.push(P01CanonicalSchemaBinding {
                line_number,
                id: field(&field_map, "id"),
                model_ref: field(&field_map, "model"),
                fields: list_field(&field_map, "fields"),
                required: list_field(&field_map, "required"),
                forbids: list_field(&field_map, "forbids"),
                status: field(&field_map, "status"),
            });
        } else if let Some(value) = line.strip_prefix("field=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &["id", "model", "kind", "order", "normalization", "status"],
                "field",
                line_number,
                &mut errors,
            );
            fields.push(P01CanonicalFieldBinding {
                line_number,
                id: field(&field_map, "id"),
                model_ref: field(&field_map, "model"),
                kind: field(&field_map, "kind"),
                order: field(&field_map, "order"),
                normalization: field(&field_map, "normalization"),
                status: field(&field_map, "status"),
            });
        } else if let Some(value) = line.strip_prefix("bridge=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &["id", "from", "to", "carrier", "receipt", "status"],
                "bridge",
                line_number,
                &mut errors,
            );
            bridges.push(P01CanonicalModelBridgeBinding {
                line_number,
                id: field(&field_map, "id"),
                from_model: field(&field_map, "from"),
                to_model: field(&field_map, "to"),
                carrier: field(&field_map, "carrier"),
                receipt_ref: field(&field_map, "receipt"),
                status: field(&field_map, "status"),
            });
        } else if let Some(value) = line.strip_prefix("proof=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &[
                    "id", "models", "schemas", "fields", "bridges", "fixture", "golden", "receipt",
                    "status",
                ],
                "proof",
                line_number,
                &mut errors,
            );
            proofs.push(P01CanonicalDataProofBinding {
                line_number,
                id: field(&field_map, "id"),
                models: list_field(&field_map, "models"),
                schemas: list_field(&field_map, "schemas"),
                fields: list_field(&field_map, "fields"),
                bridges: list_field(&field_map, "bridges"),
                fixture: field(&field_map, "fixture"),
                golden: field(&field_map, "golden"),
                receipt: field(&field_map, "receipt"),
                status: field(&field_map, "status"),
            });
        } else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unrecognized canonical data model line {line}"),
            ));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(P01CanonicalDataModelSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        models,
        schemas,
        fields,
        bridges,
        proofs,
    })
}

pub fn validate_canonical_data_model_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_canonical_data_model_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_canonical_data_model(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_canonical_data_model(
    surface: &P01CanonicalDataModelSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-014" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-014 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }

    for required in REQUIRED_P01_CANONICAL_DATA_MODEL_RULES {
        match surface.rules.get(*required) {
            Some(value) if value == "required" || value == "forbidden" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                format!("expected required/forbidden got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "missing canonical data model rule",
            )),
        }
    }

    require_ids(
        "model",
        REQUIRED_P01_CANONICAL_DATA_MODELS,
        surface.models.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingCanonicalModel,
        errors,
    );
    require_ids(
        "schema",
        REQUIRED_P01_CANONICAL_SCHEMAS,
        surface
            .schemas
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingSchemaBinding,
        errors,
    );
    require_ids(
        "field",
        REQUIRED_P01_CANONICAL_FIELDS,
        surface.fields.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingFieldBinding,
        errors,
    );
    require_ids(
        "bridge",
        REQUIRED_P01_CANONICAL_MODEL_BRIDGES,
        surface
            .bridges
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingModelBinding,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_P01_CANONICAL_DATA_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingSemanticProof,
        errors,
    );

    check_duplicate_bindings(
        "model",
        surface
            .models
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "schema",
        surface
            .schemas
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "field",
        surface
            .fields
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "bridge",
        surface
            .bridges
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "proof",
        surface
            .proofs
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );

    let model_ids: BTreeSet<&str> = surface.models.iter().map(|item| item.id.as_str()).collect();
    let schema_ids: BTreeSet<&str> = surface
        .schemas
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let field_ids: BTreeSet<&str> = surface.fields.iter().map(|item| item.id.as_str()).collect();
    let bridge_ids: BTreeSet<&str> = surface
        .bridges
        .iter()
        .map(|item| item.id.as_str())
        .collect();

    for model in &surface.models {
        validate_status("model", &model.id, model.line_number, &model.status, errors);
        if !ALLOWED_OWNER_ROOTS.contains(&model.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                format!("line:{:03}", model.line_number),
                format!(
                    "model {} owner root {} is not allowed",
                    model.id, model.owner_root
                ),
            ));
        }
        let Some(descriptor) = canonical_data_model_descriptor(&model.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                format!("line:{:03}", model.line_number),
                format!("unknown canonical data model {}", model.id),
            ));
            continue;
        };
        if model.scope != descriptor.scope
            || model.owner_root != descriptor.owner_root
            || model.source_task != descriptor.source_task
            || model.schema_ref != descriptor.schema_ref
            || model.canonical_order
                != descriptor
                    .canonical_order
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || model.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", model.line_number),
                format!("model descriptor drift {}", model.id),
            ));
        }
        if !schema_ids.contains(model.schema_ref.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", model.line_number),
                format!("model {} schema {} is unbound", model.id, model.schema_ref),
            ));
        }
    }

    for schema in &surface.schemas {
        validate_status(
            "schema",
            &schema.id,
            schema.line_number,
            &schema.status,
            errors,
        );
        let Some(descriptor) = canonical_schema_descriptor(&schema.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSchemaBinding,
                format!("line:{:03}", schema.line_number),
                format!("unknown canonical schema {}", schema.id),
            ));
            continue;
        };
        if schema.model_ref != descriptor.model_ref
            || schema.fields
                != descriptor
                    .fields
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || schema.required
                != descriptor
                    .required
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || schema.forbids
                != descriptor
                    .forbids
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || schema.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", schema.line_number),
                format!("schema descriptor drift {}", schema.id),
            ));
        }
        if !model_ids.contains(schema.model_ref.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", schema.line_number),
                format!("schema {} model {} is unbound", schema.id, schema.model_ref),
            ));
        }
        for field in &schema.fields {
            if !field_ids.contains(field.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    format!("line:{:03}", schema.line_number),
                    format!("schema {} references unbound field {}", schema.id, field),
                ));
            }
        }
        for required in &schema.required {
            if !schema.fields.contains(required) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSchemaBinding,
                    format!("line:{:03}", schema.line_number),
                    format!(
                        "schema {} required field {} missing from fields",
                        schema.id, required
                    ),
                ));
            }
        }
        if schema.forbids.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSchemaBinding,
                format!("line:{:03}", schema.line_number),
                format!("schema {} has empty forbid list", schema.id),
            ));
        }
    }

    let mut order_by_model: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for field_binding in &surface.fields {
        validate_status(
            "field",
            &field_binding.id,
            field_binding.line_number,
            &field_binding.status,
            errors,
        );
        if !ALLOWED_FIELD_KINDS.contains(&field_binding.kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                format!("line:{:03}", field_binding.line_number),
                format!(
                    "field {} has invalid kind {}",
                    field_binding.id, field_binding.kind
                ),
            ));
        }
        if !stable_order(&field_binding.order) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                format!("line:{:03}", field_binding.line_number),
                format!(
                    "field {} has unstable order {}",
                    field_binding.id, field_binding.order
                ),
            ));
        }
        let bucket = order_by_model
            .entry(field_binding.model_ref.clone())
            .or_default();
        if !bucket.insert(field_binding.order.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                format!("line:{:03}", field_binding.line_number),
                format!(
                    "duplicate field order {} for model {}",
                    field_binding.order, field_binding.model_ref
                ),
            ));
        }
        let Some(descriptor) = canonical_field_descriptor(&field_binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                format!("line:{:03}", field_binding.line_number),
                format!("unknown canonical field {}", field_binding.id),
            ));
            continue;
        };
        if field_binding.model_ref != descriptor.model_ref
            || field_binding.kind != descriptor.kind
            || field_binding.order != descriptor.order
            || field_binding.normalization != descriptor.normalization
            || field_binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", field_binding.line_number),
                format!("field descriptor drift {}", field_binding.id),
            ));
        }
        if !model_ids.contains(field_binding.model_ref.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", field_binding.line_number),
                format!(
                    "field {} model {} is unbound",
                    field_binding.id, field_binding.model_ref
                ),
            ));
        }
    }

    for bridge in &surface.bridges {
        validate_status(
            "bridge",
            &bridge.id,
            bridge.line_number,
            &bridge.status,
            errors,
        );
        let Some(descriptor) = canonical_model_bridge_descriptor(&bridge.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                format!("line:{:03}", bridge.line_number),
                format!("unknown canonical model bridge {}", bridge.id),
            ));
            continue;
        };
        if bridge.from_model != descriptor.from_model
            || bridge.to_model != descriptor.to_model
            || bridge.carrier != descriptor.carrier
            || bridge.receipt_ref != descriptor.receipt_ref
            || bridge.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", bridge.line_number),
                format!("bridge descriptor drift {}", bridge.id),
            ));
        }
        if bridge.carrier != LYRA_P01_CANONICAL_DATA_CARRIER {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticDriftAccepted,
                format!("line:{:03}", bridge.line_number),
                format!(
                    "bridge {} uses forked carrier {}",
                    bridge.id, bridge.carrier
                ),
            ));
        }
        if !model_ids.contains(bridge.from_model.as_str())
            || !model_ids.contains(bridge.to_model.as_str())
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", bridge.line_number),
                format!("bridge {} has unbound endpoint", bridge.id),
            ));
        }
        if !bridge.receipt_ref.starts_with("receipts/p01/")
            || !bridge.receipt_ref.ends_with(".receipt")
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                format!("line:{:03}", bridge.line_number),
                format!("bridge {} receipt path is not a P01 receipt", bridge.id),
            ));
        }
    }

    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        let Some(descriptor) = canonical_data_proof_descriptor(&proof.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticProof,
                format!("line:{:03}", proof.line_number),
                format!("unknown canonical data proof {}", proof.id),
            ));
            continue;
        };
        if proof.models
            != descriptor
                .models
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
            || proof.schemas
                != descriptor
                    .schemas
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.fields
                != descriptor
                    .fields
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.bridges
                != descriptor
                    .bridges
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.fixture != descriptor.fixture
            || proof.golden != descriptor.golden
            || proof.receipt != descriptor.receipt
            || proof.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticDriftAccepted,
                format!("line:{:03}", proof.line_number),
                format!("proof descriptor drift {}", proof.id),
            ));
        }
        for model in &proof.models {
            if !model_ids.contains(model.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::SemanticProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown model {}", proof.id, model),
                ));
            }
        }
        for schema in &proof.schemas {
            if !schema_ids.contains(schema.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::SemanticProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown schema {}", proof.id, schema),
                ));
            }
        }
        for field in &proof.fields {
            if !field_ids.contains(field.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::SemanticProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown field {}", proof.id, field),
                ));
            }
        }
        for bridge in &proof.bridges {
            if !bridge_ids.contains(bridge.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::SemanticProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown bridge {}", proof.id, bridge),
                ));
            }
        }
        if !proof.fixture.ends_with(".lyra")
            || !proof.golden.ends_with(".receipt")
            || !proof.receipt.ends_with(".receipt")
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} artifact paths are invalid", proof.id),
            ));
        }
    }

    if !canonical_data_models_have_schema_refs()
        || !canonical_schemas_bind_known_models_and_fields()
        || !canonical_fields_bind_known_models()
        || !canonical_model_bridges_bind_one_carrier()
        || !canonical_data_proofs_bind_artifacts()
        || !canonical_data_no_forbidden_descriptor_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "canonical_data_model",
            "canonical data descriptor registry is incomplete or drifted",
        ));
    }

    let model_rows: Vec<(String, String, String, String, String, Vec<String>, String)> = surface
        .models
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.scope.clone(),
                item.owner_root.clone(),
                item.source_task.clone(),
                item.schema_ref.clone(),
                item.canonical_order.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let schema_rows: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .schemas
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.model_ref.clone(),
                item.fields.clone(),
                item.required.clone(),
                item.forbids.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let field_rows: Vec<(String, String, String, String, String, String)> = surface
        .fields
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.model_ref.clone(),
                item.kind.clone(),
                item.order.clone(),
                item.normalization.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let bridge_rows: Vec<(String, String, String, String, String, String)> = surface
        .bridges
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.from_model.clone(),
                item.to_model.clone(),
                item.carrier.clone(),
                item.receipt_ref.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let proof_rows: Vec<(
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
        String,
        String,
        String,
    )> = surface
        .proofs
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.models.clone(),
                item.schemas.clone(),
                item.fields.clone(),
                item.bridges.clone(),
                item.fixture.clone(),
                item.golden.clone(),
                item.receipt.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let suite = deterministic_canonical_data_model_suite_report(
        &model_rows,
        &schema_rows,
        &field_rows,
        &bridge_rows,
        &proof_rows,
    );
    if suite.model_count < canonical_data_model_ids().len()
        || suite.schema_count < canonical_schema_ids().len()
        || suite.field_count < canonical_field_ids().len()
        || suite.bridge_count < canonical_model_bridge_ids().len()
        || suite.proof_count < canonical_data_proof_ids().len()
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !canonical_data_model_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "suite",
            "canonical data model suite report is incomplete or unhashable",
        ));
    }
}

fn require_ids(
    kind: &str,
    required: &[&str],
    actual: BTreeSet<&str>,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    for id in required {
        if !actual.contains(id) {
            errors.push(ValidationError::reject(
                code,
                format!("{kind}:{id}"),
                format!("missing required canonical data {kind} {id}"),
            ));
        }
    }
}

fn check_duplicate_bindings(
    kind: &str,
    items: Vec<(&str, usize)>,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for (id, line_number) in items {
        if !seen.insert(id.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate canonical data {kind} {id}"),
            ));
        }
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
    for (token, code) in FORBIDDEN_CANONICAL_DATA_TEXT {
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
        if let Some((key, val)) = part.split_once(':') {
            fields.insert(key.to_string(), val.to_string());
        }
    }
    fields
}

fn require_fields(
    fields: &BTreeMap<String, String>,
    required: &[&str],
    kind: &str,
    line_number: usize,
    errors: &mut Vec<ValidationError>,
) {
    for key in required {
        if !fields.contains_key(*key) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("{kind} requires {key}"),
            ));
        }
    }
}

fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    fields.get(name).cloned().unwrap_or_default()
}

fn list_field(fields: &BTreeMap<String, String>, name: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    fields
        .get(name)
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .filter(|item| seen.insert((*item).to_string()))
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn stable_order(value: &str) -> bool {
    value.len() == 3 && value.as_bytes().iter().all(|byte| byte.is_ascii_digit()) && value != "000"
}
