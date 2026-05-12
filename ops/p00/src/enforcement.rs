use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_enforcement_model::{ClosureClaim, EnforcementSurface, ImplementationUnit};
use crate::p00_identity::ALLOWED_OWNER_ROOTS;

pub const P00_ENFORCEMENT_CONTRACT: &str = "LYRA-P00-ENFORCEMENT-LAW v1";

pub struct RequiredImplementationUnit {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
}

pub const REQUIRED_ENFORCEMENT_RULES: &[&str] = &[
    "anti_underbuild",
    "anti_placeholder",
    "anti_fake_closure",
    "behavior_before_declaration",
    "test_fixture_receipt_required",
    "owner_root_path_binding",
    "truthful_partiality_only",
    "thin_patch_rejected",
    "docs_only_rejected",
    "empty_shell_rejected",
];

pub const REQUIRED_IMPLEMENTATION_UNITS: &[RequiredImplementationUnit] = &[
    RequiredImplementationUnit {
        id: "p00_constitution_validator",
        owner_root: "ops",
        path: "ops/p00/src/validator.rs",
    },
    RequiredImplementationUnit {
        id: "p00_authority_order_validator",
        owner_root: "ops",
        path: "ops/p00/src/authority.rs",
    },
    RequiredImplementationUnit {
        id: "p00_identity_law_validator",
        owner_root: "ops",
        path: "ops/p00/src/identity.rs",
    },
    RequiredImplementationUnit {
        id: "p00_enforcement_law_validator",
        owner_root: "ops",
        path: "ops/p00/src/enforcement.rs",
    },
];

const FORBIDDEN_ENFORCEMENT_TEXT: &[(&str, ErrorCode)] = &[
    ("todo", ErrorCode::ForbiddenToken),
    ("tbd", ErrorCode::ForbiddenToken),
    ("not implemented", ErrorCode::ForbiddenToken),
    ("will add later", ErrorCode::ForbiddenToken),
    ("finish later", ErrorCode::ForbiddenToken),
    ("placeholder scaffold", ErrorCode::PlaceholderAllowed),
    ("placeholder_scaffold", ErrorCode::PlaceholderAllowed),
    ("placeholder allowed", ErrorCode::PlaceholderAllowed),
    ("docs only", ErrorCode::DocsOnlyImplementation),
    ("documentation only", ErrorCode::DocsOnlyImplementation),
    ("folder only", ErrorCode::EmptyImplementation),
    ("empty shell", ErrorCode::EmptyImplementation),
    ("thin patch", ErrorCode::ThinPatchViolation),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("complete without evidence", ErrorCode::FakeClosureClaim),
];

pub fn parse_enforcement_surface(input: &str) -> Result<EnforcementSurface, Vec<ValidationError>> {
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
            "no enforcement surface lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_ENFORCEMENT_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_ENFORCEMENT_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut units = Vec::new();
    let mut claims = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_units = BTreeSet::new();
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
                    "enforcement rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(unit_id) = left.strip_prefix("unit:") {
            if !is_symbolic_name(unit_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidImplementationUnit,
                    format!("line:{line_number:03}"),
                    format!("invalid unit identity {unit_id}"),
                ));
                continue;
            }
            if !seen_units.insert(unit_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateImplementationUnit,
                    format!("unit:{unit_id}"),
                    "implementation unit identity must be unique",
                ));
                continue;
            }
            match parse_implementation_unit(line_number, unit_id, value) {
                Ok(unit) => units.push(unit),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(claim_id) = left.strip_prefix("claim:") {
            if claim_id.is_empty()
                || claim_id != claim_id.trim()
                || !seen_claims.insert(claim_id.to_string())
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "closure claim identity must be non-empty and unique",
                ));
                continue;
            }
            match parse_closure_claim(line_number, claim_id, value) {
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
                format!("unknown enforcement surface field {left}"),
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
                "task=P00-004 is required",
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
        Ok(EnforcementSurface {
            header,
            phase,
            task,
            status,
            rules,
            units,
            claims,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_enforcement_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_enforcement_surface(input) {
        Ok(surface) => validate_parsed_enforcement_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_implementation_unit(
    line_number: usize,
    unit_id: &str,
    value: &str,
) -> Result<ImplementationUnit, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let owner_root = required_string_field(line_number, &mut fields, "owner_root")?;
    let path = required_string_field(line_number, &mut fields, "path")?;
    let responsibility = required_string_field(line_number, &mut fields, "responsibility")?;
    let behavior = required_string_field(line_number, &mut fields, "behavior")?;
    let tests = required_string_field(line_number, &mut fields, "tests")?;
    let fixtures = required_string_field(line_number, &mut fields, "fixtures")?;
    let receipts = required_string_field(line_number, &mut fields, "receipts")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;

    Ok(ImplementationUnit {
        line_number,
        id: unit_id.to_string(),
        owner_root,
        path,
        responsibility,
        behavior,
        tests,
        fixtures,
        receipts,
        status,
    })
}

fn parse_closure_claim(
    line_number: usize,
    claim_id: &str,
    value: &str,
) -> Result<ClosureClaim, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;

    Ok(ClosureClaim {
        line_number,
        id: claim_id.to_string(),
        scope,
        status,
        evidence,
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
                "unit and claim attributes must use key:value fields",
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
            "enforcement surface contains unsupported attributes",
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

fn validate_parsed_enforcement_surface(surface: &EnforcementSurface, raw_input: &str) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            format!("expected P00, found {}", surface.phase),
        ));
    }
    if surface.task != "P00-004" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-004, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-004 may only declare working_slice status",
        ));
    }

    for rule in REQUIRED_ENFORCEMENT_RULES {
        match surface.rule_value(rule) {
            Some(value) if value.contains("required") => {}
            Some(_) | None => errors.push(ValidationError::reject(
                ErrorCode::MissingEnforcementRule,
                format!("rule:{rule}"),
                "required enforcement rule is absent or too weak",
            )),
        }
    }

    if surface.units.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingImplementationUnit,
            "unit:*",
            "at least one implementation unit is required",
        ));
    }
    for required in REQUIRED_IMPLEMENTATION_UNITS {
        match surface.unit_by_id(required.id) {
            Some(unit) => {
                if unit.owner_root != required.owner_root || unit.path != required.path {
                    errors.push(ValidationError::reject(
                        ErrorCode::MisplacedOwnerRoot,
                        format!("unit:{}", required.id),
                        "required implementation unit is not bound to its owner root path",
                    ));
                }
            }
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingImplementationUnit,
                format!("unit:{}", required.id),
                "required implementation unit is absent",
            )),
        }
    }

    for unit in &surface.units {
        validate_unit(unit, &mut errors);
    }

    if surface.claims.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            "claim:*",
            "at least one truthful partiality claim is required",
        ));
    }
    for claim in &surface.claims {
        validate_claim(claim, &mut errors);
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_ENFORCEMENT_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "enforcement:text",
                format!("forbidden enforcement phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_unit(unit: &ImplementationUnit, errors: &mut Vec<ValidationError>) {
    let location = unit.canonical_identity();

    if !ALLOWED_OWNER_ROOTS.contains(&unit.owner_root.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidOwnerRoot,
            location.clone(),
            format!("unsupported owner root {}", unit.owner_root),
        ));
    }

    let expected_prefix = format!("{}/", unit.owner_root);
    if !unit.path.starts_with(&expected_prefix) {
        errors.push(ValidationError::reject(
            ErrorCode::MisplacedOwnerRoot,
            location.clone(),
            format!(
                "path {} is not under owner root {}",
                unit.path, unit.owner_root
            ),
        ));
    }

    if weak_implementation_value(&unit.responsibility) || weak_implementation_value(&unit.behavior)
    {
        errors.push(ValidationError::reject(
            ErrorCode::UnderbuildViolation,
            location.clone(),
            "implementation unit must declare concrete responsibility and behavior",
        ));
    }

    if lacks_behavior_proof(&unit.behavior) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBehaviorProof,
            location.clone(),
            "behavior must name concrete executable validation or receipt behavior",
        ));
    }

    if missing_path_like(&unit.tests) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingTestProof,
            location.clone(),
            "implementation unit must bind tests",
        ));
    }
    if missing_path_like(&unit.fixtures) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingFixtureProof,
            location.clone(),
            "implementation unit must bind fixtures",
        ));
    }
    if missing_path_like(&unit.receipts) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "implementation unit must bind receipts",
        ));
    }

    match unit.status.as_str() {
        "working_slice" | "execution_proven" | "artifact_emitted" => {}
        "closed" | "global_complete" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location,
            "P00-004 implementation units cannot close the phase",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            location,
            format!("unsupported implementation status {other}"),
        )),
    }
}

fn validate_claim(claim: &ClosureClaim, errors: &mut Vec<ValidationError>) {
    let location = claim.canonical_identity();

    match claim.scope.as_str() {
        "task" | "frontier" | "phase" => {}
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            location.clone(),
            format!("unsupported claim scope {other}"),
        )),
    }

    match claim.status.as_str() {
        "working_slice" | "partial" | "artifact_emitted" => {}
        "closed" | "global_complete" | "complete" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "closure claims must remain truthful partiality until full P00 closure gates are proven",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location.clone(),
            format!("unsupported claim status {other}"),
        )),
    }

    if claim.evidence.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            location.clone(),
            "claim must bind explicit evidence",
        ));
    }
    for item in &claim.evidence {
        if weak_implementation_value(item) || item == "none" || item == "nothing" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedEvidenceClaim,
                location.clone(),
                format!("unsupported evidence item {item}"),
            ));
        }
    }
}

fn weak_implementation_value(value: &str) -> bool {
    matches!(
        value,
        "none"
            | "nothing"
            | "declared_only"
            | "folder_only"
            | "empty_shell"
            | "documentation_only"
            | "docs_only"
            | "thin_patch"
    )
}

fn lacks_behavior_proof(value: &str) -> bool {
    !(value.contains("validates")
        || value.contains("rejects")
        || value.contains("receipts")
        || value.contains("checks")
        || value.contains("parses"))
}

fn missing_path_like(value: &str) -> bool {
    weak_implementation_value(value)
        || !(value.contains('/') || value.contains(".rs") || value.contains(".receipt"))
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
