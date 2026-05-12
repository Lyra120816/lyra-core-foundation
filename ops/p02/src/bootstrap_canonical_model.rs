use crate::k0_bootstrap_canonical_model::deterministic_bootstrap_canonical_model_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_canonical_model_model::{
    BootstrapCanonicalFieldBinding, BootstrapCanonicalInvariantBinding,
    BootstrapCanonicalModelBinding, BootstrapCanonicalModelSurface, BootstrapCanonicalProofBinding,
    BootstrapCanonicalReceiptBinding, BootstrapCanonicalRelationBinding,
    BootstrapCanonicalSchemaBinding,
};
use std::collections::{BTreeMap, BTreeSet};

pub const P02_BOOTSTRAP_CANONICAL_MODEL_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-CANONICAL-MODEL v1";
pub const REQUIRED_BOOTSTRAP_CANONICAL_RULES: &[&str] = &[
    "bootstrap_models_must_be_canonical",
    "seed_runtime_law_models_must_be_explicit",
    "host_extinction_models_must_bind_receipts",
    "foreign_surface_models_must_be_visible",
    "operator_handoff_models_must_be_truth_neutral",
    "emergency_fallback_models_must_freeze_before_recovery",
    "every_model_requires_local_schema",
    "every_field_requires_canonical_order",
    "every_relation_must_bind_known_models",
    "every_model_invariant_requires_receipt",
    "no_network_model_source",
    "no_probabilistic_model_field",
    "no_hidden_randomness_model",
    "no_ambient_time_model",
    "no_placeholder_model",
    "no_global_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_CANONICAL_MODELS: &[&str] = &[
    "bootstrap_trust_model",
    "seed_runtime_law_model",
    "host_extinction_model",
    "foreign_surface_boundary_model",
    "operator_handoff_model",
    "emergency_fallback_model",
];
pub const REQUIRED_BOOTSTRAP_CANONICAL_SCHEMAS: &[&str] = &[
    "bootstrap_trust_schema",
    "seed_runtime_law_schema",
    "host_extinction_schema",
    "foreign_surface_boundary_schema",
    "operator_handoff_schema",
    "emergency_fallback_schema",
];
pub const REQUIRED_BOOTSTRAP_CANONICAL_FIELDS: &[&str] = &[
    "trust_receipt_hash",
    "trust_authority_floor",
    "seed_runtime_owner",
    "seed_runtime_replacement_state",
    "host_surface_id",
    "host_extinction_gate",
    "foreign_surface_id",
    "foreign_challenge_suite",
    "handoff_capture_channel",
    "handoff_truth_gate",
    "fallback_freeze_gate",
    "fallback_recovery_state",
];
pub const REQUIRED_BOOTSTRAP_CANONICAL_RELATIONS: &[&str] = &[
    "trust_model_to_seed_runtime",
    "seed_runtime_to_host_extinction",
    "host_extinction_to_foreign_surface",
    "foreign_surface_to_handoff",
    "handoff_to_fallback",
    "fallback_to_trust_model",
];
pub const REQUIRED_BOOTSTRAP_CANONICAL_INVARIANTS: &[&str] = &[
    "invariant_canonical_hash_required",
    "invariant_no_network_model_source",
    "invariant_no_probabilistic_fields",
    "invariant_no_hidden_randomness",
    "invariant_no_ambient_time",
    "invariant_no_global_closure",
    "invariant_operator_model_truth_neutral",
    "invariant_fallback_model_freezes",
];
pub const REQUIRED_BOOTSTRAP_CANONICAL_PROOFS: &[&str] = &[
    "proof_bootstrap_trust_model",
    "proof_seed_runtime_model",
    "proof_host_extinction_model",
    "proof_boundary_handoff_model",
    "proof_p02_canonical_model_phase_open",
];
pub const REQUIRED_BOOTSTRAP_CANONICAL_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_formal_semantics",
    "receipt_bootstrap_canonical_model",
    "receipt_trust_model",
    "receipt_seed_runtime_model",
    "receipt_host_extinction_model",
    "receipt_boundary_handoff_model",
    "receipt_phase_open_not_closed",
];

const ALLOWED_STATUS: &[&str] = &["bootstrap_canonical_model_artifact_emitted"];
const ALLOWED_MODEL_STATUS: &[&str] = &["canonical_model_bound"];
const ALLOWED_SCHEMA_STATUS: &[&str] = &["schema_bound"];
const ALLOWED_FIELD_STATUS: &[&str] = &["field_bound"];
const ALLOWED_RELATION_STATUS: &[&str] = &["relation_bound"];
const ALLOWED_INVARIANT_STATUS: &[&str] = &["model_invariant_bound"];
const ALLOWED_PROOF_STATUS: &[&str] = &["canonical_model_proof_bound"];
const ALLOWED_RECEIPT_STATUS: &[&str] = &["receipt_bound"];
const ALLOWED_OWNER_ROOTS: &[&str] = &["interfaces", "k0", "lyralang", "ops"];
const ALLOWED_MODEL_KINDS: &[&str] = &[
    "trust_model",
    "runtime_law_model",
    "extinction_model",
    "boundary_model",
    "handoff_model",
    "fallback_model",
];
const ALLOWED_FIELD_TYPES: &[&str] = &["hash", "symbol", "enum", "owner_root", "receipt_path"];
const ALLOWED_RELATION_KINDS: &[&str] = &[
    "receipt_enables",
    "replacement_gates",
    "boundary_exposes",
    "challenge_feeds",
    "freeze_controls",
    "recovery_rechecks",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &[
    "bootstrap_trust",
    "seed_runtime",
    "host_extinction",
    "boundary_handoff",
    "phase_open",
];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote schema", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("random field", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    ("probabilistic field", ErrorCode::ProbabilisticTruthAllowed),
    ("stochastic model", ErrorCode::ProbabilisticTruthAllowed),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("placeholder model", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    (
        "foreign truth accepted",
        ErrorCode::CanonicalModelDriftAccepted,
    ),
];

pub fn parse_bootstrap_canonical_model_surface(
    input: &str,
) -> Result<BootstrapCanonicalModelSurface, Vec<ValidationError>> {
    let lines = canonical_lines(input).map_err(|e| {
        vec![ValidationError::reject(
            ErrorCode::CanonicalControlByte,
            "input",
            format!("{e:?}"),
        )]
    })?;
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "empty bootstrap canonical model surface",
        )]);
    }
    if lines[0] != P02_BOOTSTRAP_CANONICAL_MODEL_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_CANONICAL_MODEL_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut previous_semantics_receipt = None;
    let mut rules = BTreeMap::new();
    let mut models = Vec::new();
    let mut schemas = Vec::new();
    let mut fields = Vec::new();
    let mut relations = Vec::new();
    let mut invariants = Vec::new();
    let mut proofs = Vec::new();
    let mut receipts = Vec::new();
    let mut seen = BTreeSet::new();
    let mut errors = Vec::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let n = index + 1;
        let Some((left, value)) = line.split_once('=') else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "missing =",
            ));
            continue;
        };
        let key = left.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() || key != left || value != value.trim() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "entries must be trimmed and non-empty",
            ));
            continue;
        }
        if !seen.insert(key.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{n:03}"),
                key,
            ));
            continue;
        }
        if key == "phase" {
            phase = Some(value.to_string());
            continue;
        }
        if key == "task" {
            task = Some(value.to_string());
            continue;
        }
        if key == "status" {
            status = Some(value.to_string());
            continue;
        }
        if key == "previous_semantics_receipt" {
            previous_semantics_receipt = Some(value.to_string());
            continue;
        }
        if let Some(id) = bracket_id(key, "rule") {
            rules.insert(id.to_string(), value.to_string());
            continue;
        }
        if let Some(id) = bracket_id(key, "model") {
            let fs = split_fields(value);
            if fs.len() != 6 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidCanonicalModel,
                    format!("line:{n:03}"),
                    "model row requires 6 fields",
                ));
                continue;
            }
            models.push(BootstrapCanonicalModelBinding {
                line_number: n,
                id: id.to_string(),
                owner_root: fs[0].clone(),
                domain_id: fs[1].clone(),
                canonical_kind: fs[2].clone(),
                schema_path: fs[3].clone(),
                hash_policy: fs[4].clone(),
                status: fs[5].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "schema") {
            let fs = split_fields(value);
            if fs.len() != 5 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSchemaBinding,
                    format!("line:{n:03}"),
                    "schema row requires 5 fields",
                ));
                continue;
            }
            schemas.push(BootstrapCanonicalSchemaBinding {
                line_number: n,
                id: id.to_string(),
                model_id: fs[0].clone(),
                contract_path: fs[1].clone(),
                encoding: fs[2].clone(),
                version: fs[3].clone(),
                status: fs[4].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "field") {
            let fs = split_fields(value);
            if fs.len() != 6 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidFieldBinding,
                    format!("line:{n:03}"),
                    "field row requires 6 fields",
                ));
                continue;
            }
            fields.push(BootstrapCanonicalFieldBinding {
                line_number: n,
                id: id.to_string(),
                model_id: fs[0].clone(),
                name: fs[1].clone(),
                field_type: fs[2].clone(),
                required: fs[3].clone(),
                canonical_order: fs[4].clone(),
                status: fs[5].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "relation") {
            let fs = split_fields(value);
            if fs.len() != 5 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidModelBinding,
                    format!("line:{n:03}"),
                    "relation row requires 5 fields",
                ));
                continue;
            }
            relations.push(BootstrapCanonicalRelationBinding {
                line_number: n,
                id: id.to_string(),
                from_model: fs[0].clone(),
                to_model: fs[1].clone(),
                relation_kind: fs[2].clone(),
                cardinality: fs[3].clone(),
                status: fs[4].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "invariant") {
            let fs = split_fields(value);
            if fs.len() != 5 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInvariantBinding,
                    format!("line:{n:03}"),
                    "invariant row requires 5 fields",
                ));
                continue;
            }
            invariants.push(BootstrapCanonicalInvariantBinding {
                line_number: n,
                id: id.to_string(),
                model_id: fs[0].clone(),
                assertion: fs[1].clone(),
                rejects: split_list(&fs[2]),
                receipt: fs[3].clone(),
                status: fs[4].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "proof") {
            let fs = split_fields(value);
            if fs.len() != 8 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidProofBinding,
                    format!("line:{n:03}"),
                    "proof row requires 8 fields",
                ));
                continue;
            }
            proofs.push(BootstrapCanonicalProofBinding {
                line_number: n,
                id: id.to_string(),
                scope: fs[0].clone(),
                models: split_list(&fs[1]),
                schemas: split_list(&fs[2]),
                fields: split_list(&fs[3]),
                relations: split_list(&fs[4]),
                invariants: split_list(&fs[5]),
                receipts: split_list(&fs[6]),
                status: fs[7].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "receipt") {
            let fs = split_fields(value);
            if fs.len() != 3 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReceiptChainBinding,
                    format!("line:{n:03}"),
                    "receipt row requires 3 fields",
                ));
                continue;
            }
            receipts.push(BootstrapCanonicalReceiptBinding {
                line_number: n,
                id: id.to_string(),
                path: fs[0].clone(),
                binds: fs[1].clone(),
                status: fs[2].clone(),
            });
            continue;
        }
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{n:03}"),
            format!("unknown key {key}"),
        ));
    }
    let phase = require_scalar(phase, ErrorCode::MissingPhase, "phase", &mut errors);
    let task = require_scalar(task, ErrorCode::MissingTask, "task", &mut errors);
    let status = require_scalar(
        status,
        ErrorCode::UnsupportedEvidenceClaim,
        "status",
        &mut errors,
    );
    let previous_semantics_receipt = require_scalar(
        previous_semantics_receipt,
        ErrorCode::MissingReceiptProof,
        "previous_semantics_receipt",
        &mut errors,
    );
    if errors.is_empty() {
        Ok(BootstrapCanonicalModelSurface {
            header: P02_BOOTSTRAP_CANONICAL_MODEL_CONTRACT.to_string(),
            phase,
            task,
            status,
            previous_semantics_receipt,
            rules,
            models,
            schemas,
            fields,
            relations,
            invariants,
            proofs,
            receipts,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_bootstrap_canonical_model_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let verdict = match parse_bootstrap_canonical_model_surface(input) {
        Ok(surface) => validate_bootstrap_canonical_model_model(&surface),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_canonical_model_model(
    surface: &BootstrapCanonicalModelSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "expected P02",
        ));
    }
    if surface.task != "P02-014" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "expected P02-014",
        ));
    }
    if !ALLOWED_STATUS.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            "status",
            surface.status.as_str(),
        ));
    }
    if !surface
        .previous_semantics_receipt
        .starts_with("receipts/p02/")
    {
        errors.push(ValidationError::reject(
            ErrorCode::UnknownEvidencePath,
            "previous_semantics_receipt",
            surface.previous_semantics_receipt.as_str(),
        ));
    }
    scan_forbidden(surface, &mut errors);
    require_rules(&surface.rules, &mut errors);
    require_named(
        REQUIRED_BOOTSTRAP_CANONICAL_MODELS,
        surface.models.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingCanonicalModel,
        "model",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_CANONICAL_SCHEMAS,
        surface.schemas.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingSchemaBinding,
        "schema",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_CANONICAL_FIELDS,
        surface.fields.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingFieldBinding,
        "field",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_CANONICAL_RELATIONS,
        surface.relations.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingModelBinding,
        "relation",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_CANONICAL_INVARIANTS,
        surface.invariants.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingInvariantBinding,
        "invariant",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_CANONICAL_PROOFS,
        surface.proofs.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingProofBinding,
        "proof",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_CANONICAL_RECEIPTS,
        surface.receipts.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingReceiptProof,
        "receipt",
        &mut errors,
    );

    for model in &surface.models {
        if !ALLOWED_OWNER_ROOTS.contains(&model.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                model.canonical_identity(),
                model.owner_root.as_str(),
            ));
        }
        if !ALLOWED_MODEL_KINDS.contains(&model.canonical_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                model.canonical_identity(),
                model.canonical_kind.as_str(),
            ));
        }
        if !model.local_schema() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                model.canonical_identity(),
                model.schema_path.as_str(),
            ));
        }
        if !model.deterministic_hash_policy() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                model.canonical_identity(),
                model.hash_policy.as_str(),
            ));
        }
        if !ALLOWED_MODEL_STATUS.contains(&model.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                model.canonical_identity(),
                model.status.as_str(),
            ));
        }
        if surface.fields_for_model(&model.id).is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                model.canonical_identity(),
                "model must expose fields",
            ));
        }
        if surface.invariants_for_model(&model.id).is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                model.canonical_identity(),
                "model must have invariant coverage",
            ));
        }
    }
    for schema in &surface.schemas {
        if surface.model_by_id(&schema.model_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                schema.canonical_identity(),
                format!("unknown model {}", schema.model_id),
            ));
        }
        if !schema.local_contract() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                schema.canonical_identity(),
                schema.contract_path.as_str(),
            ));
        }
        if schema.encoding != "canonical_lyra_kv" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSchemaBinding,
                schema.canonical_identity(),
                schema.encoding.as_str(),
            ));
        }
        if schema.version != "v1" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSchemaBinding,
                schema.canonical_identity(),
                schema.version.as_str(),
            ));
        }
        if !ALLOWED_SCHEMA_STATUS.contains(&schema.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSchemaBinding,
                schema.canonical_identity(),
                schema.status.as_str(),
            ));
        }
    }
    for field in &surface.fields {
        if surface.model_by_id(&field.model_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                field.canonical_identity(),
                format!("unknown model {}", field.model_id),
            ));
        }
        if !ALLOWED_FIELD_TYPES.contains(&field.field_type.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                field.canonical_identity(),
                field.field_type.as_str(),
            ));
        }
        if field.required != "required" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                field.canonical_identity(),
                field.required.as_str(),
            ));
        }
        if field.canonical_order.len() != 3
            || !field.canonical_order.chars().all(|ch| ch.is_ascii_digit())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                field.canonical_identity(),
                field.canonical_order.as_str(),
            ));
        }
        if !ALLOWED_FIELD_STATUS.contains(&field.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                field.canonical_identity(),
                field.status.as_str(),
            ));
        }
    }
    for relation in &surface.relations {
        if surface.model_by_id(&relation.from_model).is_none()
            || surface.model_by_id(&relation.to_model).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                relation.canonical_identity(),
                "relation must bind known models",
            ));
        }
        if relation.from_model == relation.to_model {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                relation.canonical_identity(),
                "relation cannot self-bind",
            ));
        }
        if !ALLOWED_RELATION_KINDS.contains(&relation.relation_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                relation.canonical_identity(),
                relation.relation_kind.as_str(),
            ));
        }
        if relation.cardinality != "one_to_one" && relation.cardinality != "one_to_many" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                relation.canonical_identity(),
                relation.cardinality.as_str(),
            ));
        }
        if !ALLOWED_RELATION_STATUS.contains(&relation.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                relation.canonical_identity(),
                relation.status.as_str(),
            ));
        }
    }
    for invariant in &surface.invariants {
        if surface.model_by_id(&invariant.model_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                invariant.canonical_identity(),
                format!("unknown model {}", invariant.model_id),
            ));
        }
        if invariant.rejects.is_empty() || invariant.rejects.iter().any(|x| x == "none") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInvariantBinding,
                invariant.canonical_identity(),
                "invariant must reject concrete unsafe states",
            ));
        }
        if !invariant.receipt_bound() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                invariant.canonical_identity(),
                invariant.receipt.as_str(),
            ));
        }
        if !ALLOWED_INVARIANT_STATUS.contains(&invariant.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInvariantBinding,
                invariant.canonical_identity(),
                invariant.status.as_str(),
            ));
        }
    }
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                proof.canonical_identity(),
                proof.scope.as_str(),
            ));
        }
        if proof.models.is_empty()
            || proof.schemas.is_empty()
            || proof.fields.is_empty()
            || proof.relations.is_empty()
            || proof.invariants.is_empty()
            || proof.receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                proof.canonical_identity(),
                "proof must bind models schemas fields relations invariants receipts",
            ));
        }
        for model in &proof.models {
            if surface.model_by_id(model).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    proof.canonical_identity(),
                    format!("unknown model {model}"),
                ));
            }
        }
        for schema in &proof.schemas {
            if surface.schema_by_id(schema).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    proof.canonical_identity(),
                    format!("unknown schema {schema}"),
                ));
            }
        }
        for field in &proof.fields {
            if surface.field_by_id(field).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    proof.canonical_identity(),
                    format!("unknown field {field}"),
                ));
            }
        }
        for relation in &proof.relations {
            if surface.relation_by_id(relation).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    proof.canonical_identity(),
                    format!("unknown relation {relation}"),
                ));
            }
        }
        for invariant in &proof.invariants {
            if surface.invariant_by_id(invariant).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    proof.canonical_identity(),
                    format!("unknown invariant {invariant}"),
                ));
            }
        }
        if !proof.receipt_bound() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                proof.canonical_identity(),
                "proof receipts must be local p02 receipts",
            ));
        }
        if !ALLOWED_PROOF_STATUS.contains(&proof.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                proof.canonical_identity(),
                proof.status.as_str(),
            ));
        }
    }
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p02/") {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                receipt.canonical_identity(),
                receipt.path.as_str(),
            ));
        }
        if !ALLOWED_RECEIPT_STATUS.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                receipt.canonical_identity(),
                receipt.status.as_str(),
            ));
        }
    }
    require_core_invariant_coverage(surface, &mut errors);
    let report = deterministic_bootstrap_canonical_model_report(surface);
    if report.model_count < REQUIRED_BOOTSTRAP_CANONICAL_MODELS.len()
        || report.schema_count < REQUIRED_BOOTSTRAP_CANONICAL_SCHEMAS.len()
        || report.field_count < REQUIRED_BOOTSTRAP_CANONICAL_FIELDS.len()
        || report.relation_count < REQUIRED_BOOTSTRAP_CANONICAL_RELATIONS.len()
        || report.invariant_count < REQUIRED_BOOTSTRAP_CANONICAL_INVARIANTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::UnderbuildViolation,
            "report",
            report.canonical_model_hash,
        ));
    }
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn require_scalar(
    value: Option<String>,
    code: ErrorCode,
    location: &str,
    errors: &mut Vec<ValidationError>,
) -> String {
    match value {
        Some(v) => v,
        None => {
            errors.push(ValidationError::reject(
                code,
                location,
                "required scalar missing",
            ));
            String::new()
        }
    }
}
fn bracket_id<'a>(key: &'a str, prefix: &str) -> Option<&'a str> {
    let start = format!("{prefix}[");
    if key.starts_with(&start) && key.ends_with(']') {
        Some(&key[start.len()..key.len() - 1])
    } else {
        None
    }
}
fn split_fields(value: &str) -> Vec<String> {
    value.split('|').map(|x| x.trim().to_string()).collect()
}
fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}
fn require_rules(rules: &BTreeMap<String, String>, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_BOOTSTRAP_CANONICAL_RULES {
        if rules.get(*required).map(String::as_str) != Some("required") {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule[{required}]"),
                "required bootstrap canonical model rule missing",
            ));
        }
    }
}
fn require_named(
    required: &[&str],
    present: Vec<&str>,
    code: ErrorCode,
    prefix: &str,
    errors: &mut Vec<ValidationError>,
) {
    let set = present.into_iter().collect::<BTreeSet<_>>();
    for item in required {
        if !set.contains(item) {
            errors.push(ValidationError::reject(
                code,
                format!("{prefix}[{item}]"),
                "required binding missing",
            ));
        }
    }
}
fn require_core_invariant_coverage(
    surface: &BootstrapCanonicalModelSurface,
    errors: &mut Vec<ValidationError>,
) {
    let rejects = surface
        .invariants
        .iter()
        .flat_map(|x| x.rejects.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();
    for token in [
        "network_required",
        "probabilistic_field",
        "hidden_randomness",
        "ambient_time",
        "global_closure",
        "truth_drift",
    ] {
        if !rejects.contains(token) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInvariantBinding,
                format!("rejects:{token}"),
                "core forbidden token not covered by model invariants",
            ));
        }
    }
}
fn scan_forbidden(surface: &BootstrapCanonicalModelSurface, errors: &mut Vec<ValidationError>) {
    let mut text = format!(
        "{} {} {} {}",
        surface.phase, surface.task, surface.status, surface.previous_semantics_receipt
    );
    for value in surface.rules.values() {
        text.push(' ');
        text.push_str(value);
    }
    for model in &surface.models {
        text.push(' ');
        text.push_str(&model.canonical_kind);
        text.push(' ');
        text.push_str(&model.schema_path);
        text.push(' ');
        text.push_str(&model.hash_policy);
    }
    for schema in &surface.schemas {
        text.push(' ');
        text.push_str(&schema.contract_path);
        text.push(' ');
        text.push_str(&schema.encoding);
    }
    for field in &surface.fields {
        text.push(' ');
        text.push_str(&field.name);
        text.push(' ');
        text.push_str(&field.field_type);
    }
    for relation in &surface.relations {
        text.push(' ');
        text.push_str(&relation.relation_kind);
        text.push(' ');
        text.push_str(&relation.cardinality);
    }
    for invariant in &surface.invariants {
        text.push(' ');
        text.push_str(&invariant.assertion);
    }
    for proof in &surface.proofs {
        text.push(' ');
        text.push_str(&proof.scope);
        text.push(' ');
        text.push_str(&proof.status);
    }
    let lower = text.to_ascii_lowercase().replace('_', " ");
    let raw_lower = text.to_ascii_lowercase();
    for (token, code) in FORBIDDEN {
        if lower.contains(token) || raw_lower.contains(token) {
            errors.push(ValidationError::reject(*code, "forbidden", *token));
        }
    }
}
