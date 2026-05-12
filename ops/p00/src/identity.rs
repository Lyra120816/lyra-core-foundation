use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_identity_model::{IdentityLawSurface, PhaseIdentity, TaskIdentity};

pub const P00_IDENTITY_LAW_CONTRACT: &str = "LYRA-P00-IDENTITY-LAW v1";

pub const REQUIRED_IDENTITY_RULES: &[&str] = &[
    "phase_id_format",
    "task_id_format",
    "work_package_id_format",
    "closure_output_id_format",
    "identity_uniqueness",
    "phase_task_prefix_match",
    "supersession_target_must_exist_or_archive",
    "supersession_is_explicit",
    "owner_root_required",
    "no_placeholder_identity",
];

pub const ALLOWED_OWNER_ROOTS: &[&str] = &[
    "k0",
    "k1",
    "lyralang",
    "shells",
    "interfaces",
    "ops",
    "slices",
    "products",
    "android",
    "web",
];

const FORBIDDEN_IDENTITY_TEXT: &[(&str, ErrorCode)] = &[
    ("todo", ErrorCode::ForbiddenToken),
    ("tbd", ErrorCode::ForbiddenToken),
    ("stub", ErrorCode::ForbiddenToken),
    ("placeholder identity", ErrorCode::ForbiddenToken),
    ("fake task", ErrorCode::ForbiddenToken),
    ("unscoped supersession", ErrorCode::UnscopedSupersession),
    ("random phase", ErrorCode::InvalidPhaseIdentity),
];

pub fn parse_identity_law_surface(input: &str) -> Result<IdentityLawSurface, Vec<ValidationError>> {
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
            "no identity law surface lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_IDENTITY_LAW_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_IDENTITY_LAW_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut phases = Vec::new();
    let mut tasks = Vec::new();
    let mut rules = BTreeMap::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rule_names = BTreeSet::new();
    let mut seen_identity_keys = BTreeSet::new();

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

        if let Some(phase_id) = left.strip_prefix("phase:") {
            let identity_key = format!("phase:{phase_id}");
            if !seen_identity_keys.insert(identity_key.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateIdentity,
                    identity_key,
                    "phase identity must be unique",
                ));
                continue;
            }
            match parse_phase_identity(line_number, phase_id, value) {
                Ok(parsed) => phases.push(parsed),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(task_id) = left.strip_prefix("task:") {
            let identity_key = format!("task:{task_id}");
            if !seen_identity_keys.insert(identity_key.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateIdentity,
                    identity_key,
                    "task identity must be unique",
                ));
                continue;
            }
            match parse_task_identity(line_number, task_id, value) {
                Ok(parsed) => tasks.push(parsed),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(rule_name) = left.strip_prefix("rule:") {
            if rule_name.is_empty() || !seen_rule_names.insert(rule_name.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "identity rule names must be non-empty and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
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
                format!("unknown identity law field {left}"),
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
                "task=P00-003 is required",
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
        Ok(IdentityLawSurface {
            header,
            phase,
            task,
            status,
            phases,
            tasks,
            rules,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_identity_law_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_identity_law_surface(input) {
        Ok(surface) => validate_parsed_identity_law(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}
fn parse_phase_identity(
    line_number: usize,
    phase_id: &str,
    value: &str,
) -> Result<PhaseIdentity, ValidationError> {
    if !is_phase_id(phase_id) {
        return Err(ValidationError::reject(
            ErrorCode::InvalidPhaseIdentity,
            format!("line:{line_number:03}"),
            format!("invalid phase identity {phase_id}"),
        ));
    }

    let (name, mut fields) = parse_named_fields(line_number, value)?;
    let owner_roots = required_list_field(line_number, &mut fields, "owner_roots")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let supersedes = required_list_field(line_number, &mut fields, "supersedes")?;
    let requires = required_list_field(line_number, &mut fields, "requires")?;
    reject_unknown_fields(line_number, fields)?;

    Ok(PhaseIdentity {
        line_number,
        id: phase_id.to_string(),
        name,
        owner_roots,
        status,
        supersedes,
        requires,
    })
}

fn parse_task_identity(
    line_number: usize,
    task_id: &str,
    value: &str,
) -> Result<TaskIdentity, ValidationError> {
    if !is_task_like_id(task_id) {
        let code = if looks_like_closure_id(task_id) {
            ErrorCode::InvalidClosureIdentity
        } else if looks_like_work_package_id(task_id) {
            ErrorCode::InvalidWorkPackageIdentity
        } else {
            ErrorCode::InvalidTaskIdentity
        };
        return Err(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("invalid task identity {task_id}"),
        ));
    }

    let (name, mut fields) = parse_named_fields(line_number, value)?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let phase = required_string_field(line_number, &mut fields, "phase")?;
    let owner_roots = required_list_field(line_number, &mut fields, "owner_roots")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let supersedes = required_list_field(line_number, &mut fields, "supersedes")?;
    let requires = required_list_field(line_number, &mut fields, "requires")?;
    reject_unknown_fields(line_number, fields)?;

    Ok(TaskIdentity {
        line_number,
        id: task_id.to_string(),
        name,
        kind,
        phase,
        owner_roots,
        status,
        supersedes,
        requires,
    })
}

fn parse_named_fields(
    line_number: usize,
    value: &str,
) -> Result<(String, BTreeMap<String, String>), ValidationError> {
    let mut parts = value.split('|');
    let Some(name) = parts.next() else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "identity name is required",
        ));
    };
    if !is_symbolic_name(name) {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "identity name must be lowercase symbolic snake case",
        ));
    }

    let mut fields = BTreeMap::new();
    for part in parts {
        let Some((key, field_value)) = part.split_once('=') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "identity attributes must use key=value syntax",
            ));
        };
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
        {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "identity attributes must be non-empty and trimmed",
            ));
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
            return Err(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate identity attribute {key}"),
            ));
        }
    }

    Ok((name.to_string(), fields))
}

fn required_string_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<String, ValidationError> {
    let Some(value) = fields.remove(key) else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("{key} attribute is required"),
        ));
    };
    Ok(value)
}

fn required_list_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<Vec<String>, ValidationError> {
    let value = required_string_field(line_number, fields, key)?;
    Ok(split_list(&value))
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
            "identity contains unsupported attributes",
        ))
    }
}

fn split_list(value: &str) -> Vec<String> {
    if value == "nothing" {
        return Vec::new();
    }
    let mut items: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect();
    items.sort();
    items.dedup();
    items
}

fn validate_parsed_identity_law(surface: &IdentityLawSurface, raw_input: &str) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            format!("expected P00, found {}", surface.phase),
        ));
    }
    if surface.task != "P00-003" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-003, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            "field:status",
            "P00-003 may only declare working_slice status",
        ));
    }

    for rule in REQUIRED_IDENTITY_RULES {
        match surface.rule_value(rule) {
            Some(value) if value.contains("required") => {}
            Some(_) | None => errors.push(ValidationError::reject(
                ErrorCode::MissingIdentityRule,
                format!("rule:{rule}"),
                "required identity rule is absent or too weak",
            )),
        }
    }

    if surface.phase_by_id("P00").is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingIdentityPhase,
            "phase:P00",
            "P00 phase identity is required by P00-003",
        ));
    }
    for required_task in ["P00-001", "P00-002", "P00-003"] {
        if surface.task_by_id(required_task).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingIdentityTask,
                format!("task:{required_task}"),
                "P00 identity law must bind the executed P00 task chain",
            ));
        }
    }

    let mut identity_set = BTreeSet::new();
    for phase in &surface.phases {
        identity_set.insert(phase.id.clone());
        validate_owner_roots(&phase.canonical_identity(), &phase.owner_roots, &mut errors);
        validate_identity_status(&phase.canonical_identity(), &phase.status, &mut errors);
        validate_requires(&phase.canonical_identity(), &phase.requires, &mut errors);
    }
    for task in &surface.tasks {
        identity_set.insert(task.id.clone());
        validate_owner_roots(&task.canonical_identity(), &task.owner_roots, &mut errors);
        validate_identity_status(&task.canonical_identity(), &task.status, &mut errors);
        validate_requires(&task.canonical_identity(), &task.requires, &mut errors);
        validate_task_kind(task, &mut errors);

        if !is_phase_id(&task.phase) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPhaseIdentity,
                task.canonical_identity(),
                format!("task declares invalid phase {}", task.phase),
            ));
        } else if surface.phase_by_id(&task.phase).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingIdentityPhase,
                task.canonical_identity(),
                format!("task references missing phase {}", task.phase),
            ));
        }

        if phase_prefix_of(&task.id) != task.phase {
            errors.push(ValidationError::reject(
                ErrorCode::IdentityPrefixMismatch,
                task.canonical_identity(),
                format!(
                    "task id {} must be prefixed by declared phase {}",
                    task.id, task.phase
                ),
            ));
        }
    }

    for phase in &surface.phases {
        validate_supersession_targets(
            &phase.canonical_identity(),
            &phase.id,
            &phase.supersedes,
            &identity_set,
            &mut errors,
        );
    }
    for task in &surface.tasks {
        validate_supersession_targets(
            &task.canonical_identity(),
            &task.id,
            &task.supersedes,
            &identity_set,
            &mut errors,
        );
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_IDENTITY_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "identity:text",
                format!("forbidden identity phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_owner_roots(location: &str, owner_roots: &[String], errors: &mut Vec<ValidationError>) {
    if owner_roots.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingOwnerRoot,
            location,
            "each phase/task identity must declare at least one owner root",
        ));
        return;
    }

    for root in owner_roots {
        if !ALLOWED_OWNER_ROOTS.iter().any(|allowed| allowed == root) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                location,
                format!("unsupported owner root {root}"),
            ));
        }
    }
}

fn validate_identity_status(location: &str, status: &str, errors: &mut Vec<ValidationError>) {
    match status {
        "planned" | "working_slice" | "partial" | "execution_proven" | "artifact_emitted"
        | "closed" => {}
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            location,
            format!("unsupported identity status {other}"),
        )),
    }
}

fn validate_requires(location: &str, requires: &[String], errors: &mut Vec<ValidationError>) {
    if requires.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            location,
            "requires must name explicit prerequisite authority or task",
        ));
    }
}

fn validate_task_kind(task: &TaskIdentity, errors: &mut Vec<ValidationError>) {
    match task.kind.as_str() {
        "primary" | "deep_work_package" | "additional_execution" | "closure_output" => {}
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            task.canonical_identity(),
            format!("unsupported task kind {other}"),
        )),
    }
}

fn validate_supersession_targets(
    location: &str,
    identity: &str,
    supersedes: &[String],
    identity_set: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    for target in supersedes {
        if target == identity {
            errors.push(ValidationError::reject(
                ErrorCode::SelfSupersession,
                location,
                "identity must not supersede itself",
            ));
            continue;
        }
        if target.contains(' ') || target.contains('*') || target == "later" || target == "future" {
            errors.push(ValidationError::reject(
                ErrorCode::UnscopedSupersession,
                location,
                format!("supersession target {target} is not explicit"),
            ));
            continue;
        }
        if target.starts_with("archive:") {
            if target.len() <= "archive:".len() {
                errors.push(ValidationError::reject(
                    ErrorCode::UnscopedSupersession,
                    location,
                    "archive supersession target must name an archived identity",
                ));
            }
            continue;
        }
        if !identity_set.contains(target) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownSupersessionTarget,
                location,
                format!("supersession target {target} is not declared or archive-scoped"),
            ));
        }
    }
}

fn is_phase_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 3 && is_phase_id_bytes(bytes)
}

fn is_phase_id_bytes(bytes: &[u8]) -> bool {
    bytes.len() == 3 && bytes[0] == b'P' && bytes[1].is_ascii_digit() && bytes[2].is_ascii_digit()
}

fn is_task_like_id(value: &str) -> bool {
    is_primary_task_id(value) || is_work_package_id(value) || is_closure_output_id(value)
}

fn is_primary_task_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 7
        && is_phase_id_bytes(&bytes[0..3])
        && bytes[3] == b'-'
        && bytes[4].is_ascii_digit()
        && bytes[5].is_ascii_digit()
        && bytes[6].is_ascii_digit()
}

fn is_work_package_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 5
        && is_phase_id_bytes(&bytes[0..3])
        && bytes[3] == b'-'
        && bytes[4].is_ascii_uppercase()
}

fn is_closure_output_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 7
        && is_phase_id_bytes(&bytes[0..3])
        && bytes[3] == b'-'
        && bytes[4] == b'X'
        && bytes[5].is_ascii_digit()
        && bytes[6].is_ascii_digit()
}

fn looks_like_work_package_id(value: &str) -> bool {
    value.contains('-')
        && value
            .chars()
            .last()
            .map(|ch| ch.is_ascii_alphabetic())
            .unwrap_or(false)
}

fn looks_like_closure_id(value: &str) -> bool {
    value.contains("-X")
}

fn phase_prefix_of(task_id: &str) -> String {
    task_id.chars().take(3).collect()
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
