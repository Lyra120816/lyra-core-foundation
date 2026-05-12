use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_authority_model::{AuthorityLayer, AuthorityOrderSurface};

pub const P00_AUTHORITY_ORDER_CONTRACT: &str = "LYRA-P00-AUTHORITY-ORDER v1";

pub struct RequiredAuthorityLayer {
    pub rank: u16,
    pub name: &'static str,
    pub authority_contains: &'static str,
    pub scope_contains: &'static str,
    pub required_requires: &'static [&'static str],
}

pub const REQUIRED_AUTHORITY_LAYERS: &[RequiredAuthorityLayer] = &[
    RequiredAuthorityLayer {
        rank: 0,
        name: "single_file_master",
        authority_contains: "highest_live_execution_authority",
        scope_contains: "all_implementation_runs",
        required_requires: &["explicit_load"],
    },
    RequiredAuthorityLayer {
        rank: 10,
        name: "roadmap_phase_task",
        authority_contains: "canonical_work_inventory",
        scope_contains: "phase_and_task_selection",
        required_requires: &["master_consistency"],
    },
    RequiredAuthorityLayer {
        rank: 20,
        name: "frontier_lock",
        authority_contains: "active_bounded_frontier",
        scope_contains: "current_pass",
        required_requires: &["truth_snapshot_binding"],
    },
    RequiredAuthorityLayer {
        rank: 30,
        name: "truth_snapshot",
        authority_contains: "evidence_status_record",
        scope_contains: "status_language_and_closure_claims",
        required_requires: &["receipt_binding"],
    },
    RequiredAuthorityLayer {
        rank: 40,
        name: "blocker_index",
        authority_contains: "next_frontier_constraints",
        scope_contains: "remaining_work",
        required_requires: &["explicit_blockers"],
    },
    RequiredAuthorityLayer {
        rank: 50,
        name: "implementation_receipts",
        authority_contains: "proof_evidence",
        scope_contains: "claim_support",
        required_requires: &["stable_hash"],
    },
    RequiredAuthorityLayer {
        rank: 60,
        name: "operator_request",
        authority_contains: "bounded_user_direction",
        scope_contains: "delivery_constraints",
        required_requires: &["no_conflict_with_constitution"],
    },
    RequiredAuthorityLayer {
        rank: 70,
        name: "archive_context",
        authority_contains: "historical_reference_only",
        scope_contains: "lookup_not_execution",
        required_requires: &["explicit_historical_need"],
    },
    RequiredAuthorityLayer {
        rank: 80,
        name: "agent_memory",
        authority_contains: "personal_workflow_preference",
        scope_contains: "response_delivery",
        required_requires: &["no_conflict_with_master"],
    },
];

pub const REQUIRED_AUTHORITY_RULES: &[&str] = &[
    "strict_total_order",
    "lower_must_not_supersede_higher",
    "missing_master_reject",
    "archive_is_subordinate",
    "frontier_lock_binds_pass",
    "truth_snapshot_binds_status",
    "operator_request_cannot_override_constitution",
    "no_ambient_authority",
];

const FORBIDDEN_AUTHORITY_TEXT: &[(&str, ErrorCode)] = &[
    ("ambient authority", ErrorCode::AmbientAuthority),
    (
        "agent preference supersedes",
        ErrorCode::AuthoritySupersessionViolation,
    ),
    (
        "operator request overrides constitution",
        ErrorCode::OperatorOverrideConstitution,
    ),
    (
        "archive context is primary",
        ErrorCode::ArchiveAuthorityTooHigh,
    ),
    (
        "highest authority is local note",
        ErrorCode::MissingMasterAuthority,
    ),
];

pub fn parse_authority_order_surface(
    input: &str,
) -> Result<AuthorityOrderSurface, Vec<ValidationError>> {
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
            "no authority surface lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_AUTHORITY_ORDER_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_AUTHORITY_ORDER_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut layers = Vec::new();
    let mut rules = BTreeMap::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rule_names = BTreeSet::new();
    let mut seen_ranks = BTreeSet::new();
    let mut seen_layer_names = BTreeSet::new();

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

        if let Some(rank_text) = left.strip_prefix("authority:") {
            match parse_authority_layer(line_number, rank_text, value) {
                Ok(layer) => {
                    if !seen_ranks.insert(layer.rank) {
                        errors.push(ValidationError::reject(
                            ErrorCode::DuplicateAuthorityRank,
                            format!("authority:{:03}", layer.rank),
                            "authority rank must be unique",
                        ));
                    }
                    if !seen_layer_names.insert(layer.name.clone()) {
                        errors.push(ValidationError::reject(
                            ErrorCode::DuplicateAuthorityLayer,
                            format!("authority:{:03}", layer.rank),
                            layer.name.clone(),
                        ));
                    }
                    layers.push(layer);
                }
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(rule_name) = left.strip_prefix("rule:") {
            if rule_name.is_empty() || !seen_rule_names.insert(rule_name.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "authority rule names must be non-empty and unique",
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
                format!("unknown authority surface field {left}"),
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
                "task=P00-002 is required",
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
        Ok(AuthorityOrderSurface {
            header,
            phase,
            task,
            status,
            layers,
            rules,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_authority_order_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_authority_order_surface(input) {
        Ok(surface) => validate_parsed_authority_order(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_authority_layer(
    line_number: usize,
    rank_text: &str,
    value: &str,
) -> Result<AuthorityLayer, ValidationError> {
    if rank_text.len() != 3
        || !rank_text
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit())
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidAuthorityRank,
            format!("line:{line_number:03}"),
            "authority rank must be exactly three decimal digits",
        ));
    }

    let Ok(rank) = rank_text.parse::<u16>() else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidAuthorityRank,
            format!("line:{line_number:03}"),
            "authority rank is outside supported range",
        ));
    };

    let mut parts = value.split('|');
    let Some(name) = parts.next() else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "authority layer name is required",
        ));
    };

    if name.is_empty() || name != name.trim() {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "authority layer name must be non-empty and trimmed",
        ));
    }

    let mut fields = BTreeMap::new();
    for part in parts {
        let Some((key, field_value)) = part.split_once('=') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "authority layer attributes must use key=value syntax",
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
                "authority layer attributes must be non-empty and trimmed",
            ));
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
            return Err(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate authority attribute {key}"),
            ));
        }
    }

    let Some(authority) = fields.remove("authority") else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "authority attribute is required",
        ));
    };
    let Some(scope) = fields.remove("scope") else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "scope attribute is required",
        ));
    };
    let Some(supersedes) = fields.remove("supersedes") else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "supersedes attribute is required",
        ));
    };
    let Some(requires) = fields.remove("requires") else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "requires attribute is required",
        ));
    };
    if !fields.is_empty() {
        return Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "authority layer contains unsupported attributes",
        ));
    }

    Ok(AuthorityLayer {
        line_number,
        rank,
        name: name.to_string(),
        authority,
        scope,
        supersedes: split_list(&supersedes),
        requires: split_list(&requires),
    })
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

fn validate_parsed_authority_order(surface: &AuthorityOrderSurface, raw_input: &str) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            format!("expected P00, found {}", surface.phase),
        ));
    }
    if surface.task != "P00-002" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-002, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            "field:status",
            "P00-002 may only declare working_slice status",
        ));
    }

    for required in REQUIRED_AUTHORITY_LAYERS {
        let Some(layer) = surface.layer_by_name(required.name) else {
            errors.push(ValidationError::reject(
                ErrorCode::MissingAuthorityLayer,
                format!("authority:{:03}", required.rank),
                required.name,
            ));
            continue;
        };
        if layer.rank != required.rank {
            errors.push(ValidationError::reject(
                ErrorCode::AuthorityOrderViolation,
                layer.canonical_identity(),
                format!("{} must be rank {:03}", required.name, required.rank),
            ));
        }
        if !layer.authority.contains(required.authority_contains) {
            errors.push(ValidationError::reject(
                ErrorCode::AuthorityOrderViolation,
                layer.canonical_identity(),
                format!("authority must contain {}", required.authority_contains),
            ));
        }
        if !layer.scope.contains(required.scope_contains) {
            errors.push(ValidationError::reject(
                ErrorCode::AuthorityOrderViolation,
                layer.canonical_identity(),
                format!("scope must contain {}", required.scope_contains),
            ));
        }
        for token in required.required_requires {
            if !layer.requires_token(token) {
                errors.push(ValidationError::reject(
                    ErrorCode::AuthorityOrderViolation,
                    layer.canonical_identity(),
                    format!("requires must contain {token}"),
                ));
            }
        }
    }

    for rule in REQUIRED_AUTHORITY_RULES {
        match surface.rule_value(rule) {
            Some(value) if value.contains("required") => {}
            Some(_) | None => errors.push(ValidationError::reject(
                ErrorCode::MissingAuthorityRule,
                format!("rule:{rule}"),
                "required authority rule is absent or too weak",
            )),
        }
    }

    let name_to_rank: BTreeMap<&str, u16> = surface
        .layers
        .iter()
        .map(|layer| (layer.name.as_str(), layer.rank))
        .collect();

    for layer in &surface.layers {
        let lowered_name = layer.name.to_ascii_lowercase();
        let lowered_authority = layer.authority.to_ascii_lowercase();
        if lowered_name.contains("ambient") || lowered_authority.contains("ambient") {
            errors.push(ValidationError::reject(
                ErrorCode::AmbientAuthority,
                layer.canonical_identity(),
                "ambient authority is not admitted into P00 authority order",
            ));
        }

        for target in &layer.supersedes {
            if let Some(target_rank) = name_to_rank.get(target.as_str()) {
                if *target_rank < layer.rank {
                    errors.push(ValidationError::reject(
                        ErrorCode::AuthoritySupersessionViolation,
                        layer.canonical_identity(),
                        format!(
                            "{} at rank {:03} cannot supersede higher authority {} at rank {:03}",
                            layer.name, layer.rank, target, target_rank
                        ),
                    ));
                }
            }
        }

        if layer.name == "operator_request"
            && (layer.supersedes_name("single_file_master")
                || layer.supersedes_name("roadmap_phase_task")
                || layer.supersedes_name("constitution"))
        {
            errors.push(ValidationError::reject(
                    ErrorCode::OperatorOverrideConstitution,
                    layer.canonical_identity(),
                    "operator request cannot override constitution, roadmap, or single-file master authority",
                ));
        }

        if layer.name == "archive_context" && layer.rank < 70 {
            errors.push(ValidationError::reject(
                ErrorCode::ArchiveAuthorityTooHigh,
                layer.canonical_identity(),
                "archive context must remain subordinate historical lookup authority",
            ));
        }
    }

    if surface.layer_by_name("single_file_master").is_none()
        || surface.layer_by_rank(0).map(|layer| layer.name.as_str()) != Some("single_file_master")
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingMasterAuthority,
            "authority:000",
            "single_file_master must be the rank 000 authority root",
        ));
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_AUTHORITY_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "authority:text",
                format!("forbidden authority phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}
