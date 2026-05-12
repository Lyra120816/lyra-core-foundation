use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_control_model::{
    ControlClaim, ControlFieldBinding, ControlSurfaceBinding, ControlSurfaceFormatLaw,
    PassTemplateBinding,
};

pub const P00_CONTROL_SURFACES_CONTRACT: &str = "LYRA-P00-CONTROL-SURFACES v1";

pub const REQUIRED_CONTROL_RULES: &[&str] = &[
    "frontier_lock_required",
    "truth_snapshot_required",
    "pass_template_required",
    "blocker_index_required",
    "machine_readable_only",
    "stable_field_order_required",
    "receipt_binding_required",
    "closure_blocker_required",
    "next_frontier_required",
    "operator_report_required",
];

pub const REQUIRED_CONTROL_SURFACES: &[&str] = &[
    "frontier_lock",
    "truth_snapshot",
    "pass_template",
    "blocker_index",
];

pub const REQUIRED_PASS_TEMPLATE_FIELDS: &[&str] = &[
    "selected_frontier",
    "files_changed",
    "tests_fixtures_goldens",
    "commands_run",
    "concrete_result",
    "next_frontier",
];

const REQUIRED_FRONTIER_FIELDS: &[&str] = &[
    "phase",
    "current_task",
    "current_work_package",
    "previous_frontier",
    "selected_frontier",
    "allowed_claim",
    "rejected_claim",
    "owner_roots",
    "truth_bound",
    "next_frontier",
];

const REQUIRED_TRUTH_FIELDS: &[&str] = &[
    "phase",
    "status",
    "closed",
    "latest_finished_frontier",
    "current_frontier",
    "truth_bound",
    "not_closed",
];

const REQUIRED_BLOCKER_FIELDS: &[&str] = &[
    "phase",
    "current_frontier",
    "blocked_global_closure",
    "blocker:P00-008",
    "blocker:P00-009",
    "next_immediate_frontier",
];

const CONTROL_KINDS: &[&str] = &[
    "frontier_lock",
    "truth_snapshot",
    "pass_template",
    "blocker_index",
];
const CONTROL_OWNER_ROOTS: &[&str] = &[
    "ops",
    "interfaces",
    "fixtures",
    "goldens",
    "receipts",
    "tests",
    "src",
];
const CONTROL_FIELD_KINDS: &[&str] = &["scalar", "list", "receipt", "blocker", "claim"];
const CLAIM_STATUSES: &[&str] = &["working_slice", "artifact_emitted", "execution_proven"];

const FORBIDDEN_CONTROL_TEXT: &[(&str, ErrorCode)] = &[
    ("todo", ErrorCode::ForbiddenToken),
    ("tbd", ErrorCode::ForbiddenToken),
    ("not implemented", ErrorCode::ForbiddenToken),
    ("will add later", ErrorCode::ForbiddenToken),
    ("finish later", ErrorCode::ForbiddenToken),
    ("plain markdown", ErrorCode::InvalidControlSurface),
    ("manual only", ErrorCode::InvalidControlSurface),
    ("human only", ErrorCode::InvalidControlSurface),
    ("closure without blocker", ErrorCode::MissingBlockerBinding),
    (
        "truth without receipt",
        ErrorCode::MissingTruthSnapshotBinding,
    ),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_control_surface_format_law(
    input: &str,
) -> Result<ControlSurfaceFormatLaw, Vec<ValidationError>> {
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
            "no control surface format law lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_CONTROL_SURFACES_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_CONTROL_SURFACES_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut surfaces = Vec::new();
    let mut fields = Vec::new();
    let mut templates = Vec::new();
    let mut claims = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_surfaces = BTreeSet::new();
    let mut seen_fields = BTreeSet::new();
    let mut seen_templates = BTreeSet::new();
    let mut seen_claims = BTreeSet::new();

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
                    "control rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(surface_id) = left.strip_prefix("surface:") {
            if !is_symbolic_name(surface_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidControlSurface,
                    format!("line:{line_number:03}"),
                    format!("invalid control surface identity {surface_id}"),
                ));
                continue;
            }
            if !seen_surfaces.insert(surface_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateControlSurface,
                    format!("surface:{surface_id}"),
                    "control surface identity must be unique",
                ));
                continue;
            }
            match parse_surface_binding(line_number, surface_id, value) {
                Ok(surface) => surfaces.push(surface),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(field_id) = left.strip_prefix("field:") {
            if !is_field_identity(field_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidControlField,
                    format!("line:{line_number:03}"),
                    format!("invalid control field identity {field_id}"),
                ));
                continue;
            }
            if !seen_fields.insert(field_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateControlField,
                    format!("field:{field_id}"),
                    "control field identity must be unique",
                ));
                continue;
            }
            match parse_field_binding(line_number, field_id, value) {
                Ok(field) => fields.push(field),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(template_id) = left.strip_prefix("template:") {
            if !is_symbolic_name(template_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidPassTemplate,
                    format!("line:{line_number:03}"),
                    format!("invalid pass template identity {template_id}"),
                ));
                continue;
            }
            if !seen_templates.insert(template_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicatePassTemplate,
                    format!("template:{template_id}"),
                    "pass template identity must be unique",
                ));
                continue;
            }
            match parse_template_binding(line_number, template_id, value) {
                Ok(template) => templates.push(template),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(claim_id) = left.strip_prefix("claim:") {
            if !is_symbolic_name(claim_id) || !seen_claims.insert(claim_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "control claim identity must be symbolic and unique",
                ));
                continue;
            }
            match parse_claim(line_number, claim_id, value) {
                Ok(claim) => claims.push(claim),
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
                format!("unknown control format field {left}"),
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
                "task=P00-007 is required",
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
        Ok(ControlSurfaceFormatLaw {
            header,
            phase,
            task,
            status,
            rules,
            surfaces,
            fields,
            templates,
            claims,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_control_surface_format_law(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_control_surface_format_law(input) {
        Ok(surface) => validate_parsed_control_surface_format_law(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_surface_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ControlSurfaceBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let schema = required_string_field(line_number, &mut fields, "schema")?;
    let required_fields = required_list_field(line_number, &mut fields, "required_fields")?;
    let owner_root = required_string_field(line_number, &mut fields, "owner_root")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ControlSurfaceBinding {
        line_number,
        id: id.to_string(),
        kind,
        schema,
        required_fields,
        owner_root,
        status,
        evidence,
    })
}

fn parse_field_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ControlFieldBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let required = required_string_field(line_number, &mut fields, "required")?;
    let value = required_string_field(line_number, &mut fields, "value")?;
    let stable = required_string_field(line_number, &mut fields, "stable")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ControlFieldBinding {
        line_number,
        id: id.to_string(),
        kind,
        required,
        value,
        stable,
    })
}

fn parse_template_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<PassTemplateBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let path = required_string_field(line_number, &mut fields, "path")?;
    let requires = required_list_field(line_number, &mut fields, "requires")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(PassTemplateBinding {
        line_number,
        id: id.to_string(),
        path,
        requires,
        forbids,
        status,
        evidence,
    })
}

fn parse_claim(line_number: usize, id: &str, value: &str) -> Result<ControlClaim, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let surfaces = required_list_field(line_number, &mut fields, "surfaces")?;
    let templates = required_list_field(line_number, &mut fields, "templates")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ControlClaim {
        line_number,
        id: id.to_string(),
        scope,
        status,
        surfaces,
        templates,
        receipts,
        commands,
    })
}

fn parse_fields(
    line_number: usize,
    value: &str,
) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut fields = BTreeMap::new();
    for raw_part in value.split(';') {
        let part = raw_part.trim();
        if part.is_empty() {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "empty field segment is not allowed",
            ));
        }
        let Some((key, field_value)) = part.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field segment must use key:value syntax",
            ));
        };
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
            || !is_symbolic_name(key)
        {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field key/value must be non-empty canonical tokens",
            ));
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
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
            "control surface contains unsupported attributes",
        ))
    }
}

fn validate_parsed_control_surface_format_law(
    surface: &ControlSurfaceFormatLaw,
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
    if surface.task != "P00-007" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-007, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-007 may only declare working_slice status",
        ));
    }

    for rule in REQUIRED_CONTROL_RULES {
        match surface.rule_value(rule) {
            Some(value) if value.contains("required") => {}
            Some(_) | None => errors.push(ValidationError::reject(
                ErrorCode::MissingControlRule,
                format!("rule:{rule}"),
                "required control rule is absent or too weak",
            )),
        }
    }

    if surface.surfaces.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingControlSurface,
            "surface:*",
            "control format law must bind control surfaces",
        ));
    }
    for required in REQUIRED_CONTROL_SURFACES {
        if surface.surface_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingControlSurface,
                format!("surface:{required}"),
                "required control surface binding is absent",
            ));
        }
    }

    let surface_ids: BTreeSet<String> = surface
        .surfaces
        .iter()
        .map(|binding| binding.id.clone())
        .collect();
    for binding in &surface.surfaces {
        validate_surface_binding(binding, &mut errors);
    }

    let mut fields_by_surface: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for field in &surface.fields {
        validate_field_binding(field, &surface_ids, &mut fields_by_surface, &mut errors);
    }
    require_fields(
        "frontier_lock",
        REQUIRED_FRONTIER_FIELDS,
        &fields_by_surface,
        &mut errors,
    );
    require_fields(
        "truth_snapshot",
        REQUIRED_TRUTH_FIELDS,
        &fields_by_surface,
        &mut errors,
    );
    require_fields(
        "blocker_index",
        REQUIRED_BLOCKER_FIELDS,
        &fields_by_surface,
        &mut errors,
    );

    if surface.templates.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingPassTemplate,
            "template:*",
            "pass template binding is required",
        ));
    }
    if surface.template_by_id("pass_template").is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingPassTemplate,
            "template:pass_template",
            "canonical pass template binding is absent",
        ));
    }
    let template_ids: BTreeSet<String> = surface
        .templates
        .iter()
        .map(|template| template.id.clone())
        .collect();
    for template in &surface.templates {
        validate_template_binding(template, &mut errors);
    }

    if surface.claims.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            "claim:*",
            "control surface law must declare a truthful claim",
        ));
    }
    for claim in &surface.claims {
        validate_claim(claim, &surface_ids, &template_ids, &mut errors);
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_CONTROL_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "control:text",
                format!("forbidden control phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_surface_binding(binding: &ControlSurfaceBinding, errors: &mut Vec<ValidationError>) {
    let location = binding.canonical_identity();
    if !CONTROL_KINDS.contains(&binding.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlSurface,
            location.clone(),
            format!("unsupported control kind {}", binding.kind),
        ));
    }
    if binding.kind != binding.id {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlSurface,
            location.clone(),
            "surface identity must match its kind",
        ));
    }
    if !CONTROL_OWNER_ROOTS.contains(&binding.owner_root.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlSurface,
            location.clone(),
            format!("unsupported owner root {}", binding.owner_root),
        ));
    }
    let expected_prefix = format!("{}/", binding.owner_root);
    if !binding.schema.starts_with(&expected_prefix) {
        errors.push(ValidationError::reject(
            ErrorCode::MisplacedOwnerRoot,
            location.clone(),
            format!(
                "schema {} is not under root {}",
                binding.schema, binding.owner_root
            ),
        ));
    }
    if binding.status != "active" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlSurface,
            location.clone(),
            format!(
                "control surface status must be active, found {}",
                binding.status
            ),
        ));
    }
    if binding.evidence.iter().all(|path| path != &binding.schema) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            location.clone(),
            "control surface evidence must include its schema path",
        ));
    }
    if binding.required_fields.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingControlField,
            location,
            "control surface must list required fields",
        ));
    }
}

fn validate_field_binding(
    field: &ControlFieldBinding,
    surface_ids: &BTreeSet<String>,
    fields_by_surface: &mut BTreeMap<String, BTreeSet<String>>,
    errors: &mut Vec<ValidationError>,
) {
    let location = field.canonical_identity();
    let Some(surface_id) = field.surface_id() else {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlField,
            location,
            "field must bind to surface.field identity",
        ));
        return;
    };
    let Some(field_name) = field.field_name() else {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlField,
            location,
            "field must bind a named control field",
        ));
        return;
    };
    if !surface_ids.contains(surface_id) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlField,
            location.clone(),
            format!("unknown control surface {surface_id}"),
        ));
    }
    if !CONTROL_FIELD_KINDS.contains(&field.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlField,
            location.clone(),
            format!("unsupported control field kind {}", field.kind),
        ));
    }
    if field.required != "yes" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingControlField,
            location.clone(),
            "control field must be required=yes",
        ));
    }
    if field.stable != "yes" {
        errors.push(ValidationError::reject(
            ErrorCode::ControlSurfaceDrift,
            location.clone(),
            "control field must declare stable=yes",
        ));
    }
    if weak_control_value(&field.value) {
        errors.push(ValidationError::reject(
            ErrorCode::ControlSurfaceDrift,
            location.clone(),
            "control field must bind a concrete value",
        ));
    }
    fields_by_surface
        .entry(surface_id.to_string())
        .or_default()
        .insert(field_name.to_string());
}

fn validate_template_binding(template: &PassTemplateBinding, errors: &mut Vec<ValidationError>) {
    let location = template.canonical_identity();
    if template.path != "ops/p00/control/pass_template.v1.lyra" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPassTemplate,
            location.clone(),
            "pass template must bind ops/p00/control/pass_template.v1.lyra",
        ));
    }
    for required in REQUIRED_PASS_TEMPLATE_FIELDS {
        if !template.requires.iter().any(|field| field == required) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPassTemplate,
                location.clone(),
                format!("pass template is missing required report field {required}"),
            ));
        }
    }
    for forbidden in ["global_complete", "placeholder", "no_artifact"] {
        if !template.forbids.iter().any(|item| item == forbidden) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPassTemplate,
                location.clone(),
                format!("pass template must forbid {forbidden}"),
            ));
        }
    }
    if template.status != "active" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPassTemplate,
            location.clone(),
            format!("template status must be active, found {}", template.status),
        ));
    }
    if template.evidence.iter().all(|path| path != &template.path) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            location,
            "template evidence must include template path",
        ));
    }
}

fn validate_claim(
    claim: &ControlClaim,
    surface_ids: &BTreeSet<String>,
    template_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = claim.canonical_identity();
    if claim.scope != "task" && claim.scope != "frontier" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location.clone(),
            format!("unsupported claim scope {}", claim.scope),
        ));
    }
    if !CLAIM_STATUSES.contains(&claim.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location.clone(),
            format!("unsupported claim status {}", claim.status),
        ));
    }
    for required in REQUIRED_CONTROL_SURFACES {
        if !claim.surfaces.iter().any(|surface| surface == required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingControlSurface,
                location.clone(),
                format!("claim does not bind required surface {required}"),
            ));
        }
    }
    for surface in &claim.surfaces {
        if !surface_ids.contains(surface) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("claim references unknown control surface {surface}"),
            ));
        }
    }
    for template in &claim.templates {
        if !template_ids.contains(template) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("claim references unknown pass template {template}"),
            ));
        }
    }
    if claim
        .receipts
        .iter()
        .all(|receipt| !receipt.ends_with(".receipt"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "claim must bind at least one receipt path",
        ));
    }
    if claim.commands.is_empty()
        || claim
            .commands
            .iter()
            .any(|command| weak_control_value(command))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location,
            "claim must bind command records",
        ));
    }
}

fn require_fields(
    surface_id: &str,
    required_fields: &[&str],
    fields_by_surface: &BTreeMap<String, BTreeSet<String>>,
    errors: &mut Vec<ValidationError>,
) {
    let Some(fields) = fields_by_surface.get(surface_id) else {
        errors.push(ValidationError::reject(
            ErrorCode::MissingControlField,
            format!("surface:{surface_id}"),
            "required surface has no field bindings",
        ));
        return;
    };
    for required in required_fields {
        if !fields.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingControlField,
                format!("field:{surface_id}.{required}"),
                "required control field binding is absent",
            ));
        }
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

fn weak_control_value(value: &str) -> bool {
    matches!(
        value,
        "none" | "nothing" | "declared_only" | "manual_only" | "human_only" | "unbound"
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

fn is_field_identity(value: &str) -> bool {
    let Some((surface, field)) = value.split_once('.') else {
        return false;
    };
    is_symbolic_name(surface)
        && !field.is_empty()
        && field == field.trim()
        && field.as_bytes().iter().all(|byte| {
            byte.is_ascii_alphanumeric() || *byte == b'_' || *byte == b':' || *byte == b'-'
        })
}
