use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_public_interest_model::{
    AntiExtractiveDuty, ParticipationRight, PublicInterestLawSurface, PublicInterestSafeguard,
    StewardshipClaim,
};

pub const P00_PUBLIC_INTEREST_LAW_CONTRACT: &str = "LYRA-P00-PUBLIC-INTEREST-LAW v1";

pub const REQUIRED_PUBLIC_INTEREST_RULES: &[&str] = &[
    "public_benefit_required",
    "anti_extractive_default_required",
    "labor_participation_required",
    "community_challenge_required",
    "anti_capture_required",
    "transparent_evidence_required",
    "non_retaliation_required",
    "offline_sovereignty_required",
    "accessibility_required",
    "stewardship_receipt_required",
];

pub const REQUIRED_PUBLIC_INTEREST_SAFEGUARDS: &[&str] = &[
    "public_benefit_floor",
    "anti_capture_gate",
    "accessibility_floor",
    "offline_sovereignty_guard",
];

pub const REQUIRED_PARTICIPATION_CONSTITUENCIES: &[&str] =
    &["operator", "labor", "community", "public_interest"];

pub const REQUIRED_ANTI_EXTRACTIVE_DUTIES: &[&str] = &[
    "no_extract_default",
    "no_lock_in_capture",
    "no_hidden_monetization",
    "no_labor_displacement_without_participation",
];

const SAFEGUARD_CATEGORIES: &[&str] = &[
    "public_benefit",
    "anti_capture",
    "accessibility",
    "sovereignty",
    "labor",
];
const CLAIM_SCOPES: &[&str] = &["task", "frontier", "phase"];
const CLAIM_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];

const FORBIDDEN_PUBLIC_INTEREST_TEXT: &[(&str, ErrorCode)] = &[
    ("profit first", ErrorCode::ExtractiveDefaultAllowed),
    ("extract by default", ErrorCode::ExtractiveDefaultAllowed),
    ("capture allowed", ErrorCode::CaptureRiskAllowed),
    ("vendor lock in allowed", ErrorCode::CaptureRiskAllowed),
    ("labor bypass", ErrorCode::LaborParticipationBypass),
    ("community bypass", ErrorCode::MissingParticipationRight),
    ("retaliation allowed", ErrorCode::RetaliationAllowed),
    ("public benefit later", ErrorCode::PublicBenefitUnbound),
    ("manual governance only", ErrorCode::InvalidStewardshipClaim),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_public_interest_law_surface(
    input: &str,
) -> Result<PublicInterestLawSurface, Vec<ValidationError>> {
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
            "no public-interest law lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_PUBLIC_INTEREST_LAW_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_PUBLIC_INTEREST_LAW_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut safeguards = Vec::new();
    let mut rights = Vec::new();
    let mut duties = Vec::new();
    let mut stewardship = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_safeguards = BTreeSet::new();
    let mut seen_rights = BTreeSet::new();
    let mut seen_duties = BTreeSet::new();
    let mut seen_stewardship = BTreeSet::new();

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
                    "public-interest rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(safeguard_id) = left.strip_prefix("safeguard:") {
            if !is_symbolic_name(safeguard_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSafeguard,
                    format!("line:{line_number:03}"),
                    format!("invalid safeguard identity {safeguard_id}"),
                ));
                continue;
            }
            if !seen_safeguards.insert(safeguard_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateSafeguard,
                    format!("safeguard:{safeguard_id}"),
                    "safeguard identity must be unique",
                ));
                continue;
            }
            match parse_safeguard(line_number, safeguard_id, value) {
                Ok(item) => safeguards.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(right_id) = left.strip_prefix("right:") {
            if !is_symbolic_name(right_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidParticipationRight,
                    format!("line:{line_number:03}"),
                    format!("invalid right identity {right_id}"),
                ));
                continue;
            }
            if !seen_rights.insert(right_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateParticipationRight,
                    format!("right:{right_id}"),
                    "participation right identity must be unique",
                ));
                continue;
            }
            match parse_right(line_number, right_id, value) {
                Ok(item) => rights.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(duty_id) = left.strip_prefix("duty:") {
            if !is_symbolic_name(duty_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidAntiExtractiveDuty,
                    format!("line:{line_number:03}"),
                    format!("invalid duty identity {duty_id}"),
                ));
                continue;
            }
            if !seen_duties.insert(duty_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateAntiExtractiveDuty,
                    format!("duty:{duty_id}"),
                    "anti-extractive duty identity must be unique",
                ));
                continue;
            }
            match parse_duty(line_number, duty_id, value) {
                Ok(item) => duties.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(claim_id) = left.strip_prefix("stewardship:") {
            if !is_symbolic_name(claim_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidStewardshipClaim,
                    format!("line:{line_number:03}"),
                    format!("invalid stewardship claim identity {claim_id}"),
                ));
                continue;
            }
            if !seen_stewardship.insert(claim_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateStewardshipClaim,
                    format!("stewardship:{claim_id}"),
                    "stewardship claim identity must be unique",
                ));
                continue;
            }
            match parse_stewardship(line_number, claim_id, value) {
                Ok(item) => stewardship.push(item),
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
                format!("unknown public-interest field {left}"),
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
                "task=P00-010 is required",
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
        Ok(PublicInterestLawSurface {
            header,
            phase,
            task,
            status,
            rules,
            safeguards,
            rights,
            duties,
            stewardship,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_public_interest_law_surface(input: &str) -> (Verdict, Receipt) {
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

    let verdict = match parse_public_interest_law_surface(input) {
        Ok(surface) => validate_parsed_public_interest_law_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_safeguard(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<PublicInterestSafeguard, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let category = required_string_field(line_number, &mut fields, "category")?;
    let protects = required_list_field(line_number, &mut fields, "protects")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    let review = required_string_field(line_number, &mut fields, "review")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(PublicInterestSafeguard {
        line_number,
        id: id.to_string(),
        category,
        protects,
        forbids,
        evidence,
        review,
    })
}

fn parse_right(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ParticipationRight, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let constituency = required_string_field(line_number, &mut fields, "constituency")?;
    let rights = required_list_field(line_number, &mut fields, "rights")?;
    let channels = required_list_field(line_number, &mut fields, "channels")?;
    let protections = required_list_field(line_number, &mut fields, "protections")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ParticipationRight {
        line_number,
        id: id.to_string(),
        constituency,
        rights,
        channels,
        protections,
        evidence,
    })
}

fn parse_duty(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<AntiExtractiveDuty, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let duty = required_string_field(line_number, &mut fields, "duty")?;
    let requires = required_list_field(line_number, &mut fields, "requires")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    let audit = required_string_field(line_number, &mut fields, "audit")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(AntiExtractiveDuty {
        line_number,
        id: id.to_string(),
        duty,
        requires,
        forbids,
        audit,
        evidence,
    })
}

fn parse_stewardship(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<StewardshipClaim, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let safeguards = required_list_field(line_number, &mut fields, "safeguards")?;
    let rights = required_list_field(line_number, &mut fields, "rights")?;
    let duties = required_list_field(line_number, &mut fields, "duties")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(StewardshipClaim {
        line_number,
        id: id.to_string(),
        scope,
        status,
        safeguards,
        rights,
        duties,
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
            "public-interest surface contains unsupported attributes",
        ))
    }
}

fn validate_parsed_public_interest_law_surface(
    surface: &PublicInterestLawSurface,
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
    if surface.task != "P00-010" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-010, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-010 may only declare working_slice status",
        ));
    }

    for rule in REQUIRED_PUBLIC_INTEREST_RULES {
        match surface.rule_value(rule) {
            Some(value) if value.contains("required") || value.contains("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingPublicInterestRule,
                format!("rule:{rule}"),
                format!("rule must carry required or forbidden force, found {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingPublicInterestRule,
                format!("rule:{rule}"),
                "required public-interest rule missing",
            )),
        }
    }

    let safeguard_ids: BTreeSet<String> = surface
        .safeguards
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let right_ids: BTreeSet<String> = surface.rights.iter().map(|item| item.id.clone()).collect();
    let duty_ids: BTreeSet<String> = surface.duties.iter().map(|item| item.id.clone()).collect();

    for required in REQUIRED_PUBLIC_INTEREST_SAFEGUARDS {
        if surface.safeguard_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingSafeguard,
                format!("safeguard:{required}"),
                "required public-interest safeguard is absent",
            ));
        }
    }
    for safeguard in &surface.safeguards {
        validate_safeguard(safeguard, &mut errors);
    }

    for constituency in REQUIRED_PARTICIPATION_CONSTITUENCIES {
        if !surface
            .rights
            .iter()
            .any(|right| right.constituency == *constituency)
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingParticipationRight,
                format!("constituency:{constituency}"),
                "required participation constituency is absent",
            ));
        }
    }
    for right in &surface.rights {
        validate_right(right, &mut errors);
    }

    for required in REQUIRED_ANTI_EXTRACTIVE_DUTIES {
        if surface.duty_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingAntiExtractiveDuty,
                format!("duty:{required}"),
                "required anti-extractive duty is absent",
            ));
        }
    }
    for duty in &surface.duties {
        validate_duty(duty, &mut errors);
    }

    if surface.stewardship.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingStewardshipClaim,
            "stewardship:*",
            "at least one stewardship claim is required",
        ));
    }
    for claim in &surface.stewardship {
        validate_stewardship_claim(claim, &safeguard_ids, &right_ids, &duty_ids, &mut errors);
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_PUBLIC_INTEREST_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "public-interest:text",
                format!("forbidden public-interest phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_safeguard(safeguard: &PublicInterestSafeguard, errors: &mut Vec<ValidationError>) {
    let location = safeguard.canonical_identity();
    if !SAFEGUARD_CATEGORIES.contains(&safeguard.category.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSafeguard,
            location.clone(),
            format!("unsupported safeguard category {}", safeguard.category),
        ));
    }
    if safeguard.protects.iter().any(|value| weak_value(value))
        || safeguard.forbids.iter().any(|value| weak_value(value))
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSafeguard,
            location.clone(),
            "safeguard protects/forbids lists must be concrete",
        ));
    }
    if !safeguard.protects.iter().any(|value| {
        value == "public_benefit" || value == "human_agency" || value == "offline_sovereignty"
    }) {
        errors.push(ValidationError::reject(
            ErrorCode::PublicBenefitUnbound,
            location.clone(),
            "safeguard must protect public benefit, human agency, or offline sovereignty",
        ));
    }
    if safeguard.id == "anti_capture_gate"
        && !safeguard
            .forbids
            .iter()
            .any(|value| value == "platform_capture" || value == "vendor_lock_in")
    {
        errors.push(ValidationError::reject(
            ErrorCode::CaptureRiskAllowed,
            location.clone(),
            "anti-capture gate must forbid platform capture or vendor lock-in",
        ));
    }
    if safeguard.id == "offline_sovereignty_guard"
        && !safeguard
            .forbids
            .iter()
            .any(|value| value == "cloud_truth_dependency" || value == "hidden_network_dependency")
    {
        errors.push(ValidationError::reject(
            ErrorCode::AmbientNetworkAllowed,
            location.clone(),
            "offline sovereignty guard must forbid cloud or hidden network dependency",
        ));
    }
    if !has_known_evidence(&safeguard.evidence) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingRequiredEvidence,
            location.clone(),
            "safeguard must bind known-root evidence",
        ));
    }
    if weak_value(&safeguard.review) || safeguard.review == "manual_only" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSafeguard,
            location,
            "safeguard review must be concrete",
        ));
    }
}

fn validate_right(right: &ParticipationRight, errors: &mut Vec<ValidationError>) {
    let location = right.canonical_identity();
    if !REQUIRED_PARTICIPATION_CONSTITUENCIES.contains(&right.constituency.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidParticipationRight,
            location.clone(),
            format!("unsupported constituency {}", right.constituency),
        ));
    }
    for required in ["inspect", "challenge"] {
        if !right.rights.iter().any(|value| value == required) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidParticipationRight,
                location.clone(),
                format!("participation right must include {required}"),
            ));
        }
    }
    if right.constituency == "labor" && !right.rights.iter().any(|value| value == "participate") {
        errors.push(ValidationError::reject(
            ErrorCode::LaborParticipationBypass,
            location.clone(),
            "labor constituency must carry participate right",
        ));
    }
    if !right
        .protections
        .iter()
        .any(|value| value == "non_retaliation")
    {
        errors.push(ValidationError::reject(
            ErrorCode::RetaliationAllowed,
            location.clone(),
            "participation rights must include non-retaliation protection",
        ));
    }
    if right.channels.iter().any(|value| weak_value(value)) || !has_known_evidence(&right.evidence)
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidParticipationRight,
            location,
            "participation rights require concrete channels and evidence",
        ));
    }
}

fn validate_duty(duty: &AntiExtractiveDuty, errors: &mut Vec<ValidationError>) {
    let location = duty.canonical_identity();
    if weak_value(&duty.duty) || weak_value(&duty.audit) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAntiExtractiveDuty,
            location.clone(),
            "anti-extractive duty and audit must be concrete",
        ));
    }
    if duty.id == "no_extract_default"
        && !duty
            .forbids
            .iter()
            .any(|value| value == "extractive_default" || value == "profit_first_governance")
    {
        errors.push(ValidationError::reject(
            ErrorCode::ExtractiveDefaultAllowed,
            location.clone(),
            "no-extract default must forbid extractive defaults or profit-first governance",
        ));
    }
    if duty.id == "no_lock_in_capture"
        && !duty
            .forbids
            .iter()
            .any(|value| value == "vendor_lock_in" || value == "platform_capture")
    {
        errors.push(ValidationError::reject(
            ErrorCode::CaptureRiskAllowed,
            location.clone(),
            "lock-in/capture duty must forbid lock-in or capture",
        ));
    }
    if duty.id == "no_labor_displacement_without_participation"
        && !duty
            .requires
            .iter()
            .any(|value| value == "labor_review" || value == "labor_challenge_path")
    {
        errors.push(ValidationError::reject(
            ErrorCode::LaborParticipationBypass,
            location.clone(),
            "labor displacement duty must require labor review or challenge path",
        ));
    }
    if duty.requires.iter().any(|value| weak_value(value))
        || duty.forbids.iter().any(|value| weak_value(value))
        || !has_known_evidence(&duty.evidence)
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAntiExtractiveDuty,
            location,
            "anti-extractive duty requires concrete requirements, forbids, and evidence",
        ));
    }
}

fn validate_stewardship_claim(
    claim: &StewardshipClaim,
    safeguard_ids: &BTreeSet<String>,
    right_ids: &BTreeSet<String>,
    duty_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = claim.canonical_identity();
    if !CLAIM_SCOPES.contains(&claim.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidStewardshipClaim,
            location.clone(),
            format!("unsupported stewardship scope {}", claim.scope),
        ));
    }
    if !CLAIM_STATUSES.contains(&claim.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location.clone(),
            format!("unsupported stewardship status {}", claim.status),
        ));
    }
    if claim.scope == "phase" && claim.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "phase-level stewardship must remain blocked until P00 closure",
        ));
    }
    for safeguard in &claim.safeguards {
        if !safeguard_ids.contains(safeguard) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidStewardshipClaim,
                location.clone(),
                format!("unknown safeguard binding {safeguard}"),
            ));
        }
    }
    for right in &claim.rights {
        if !right_ids.contains(right) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidStewardshipClaim,
                location.clone(),
                format!("unknown right binding {right}"),
            ));
        }
    }
    for duty in &claim.duties {
        if !duty_ids.contains(duty) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidStewardshipClaim,
                location.clone(),
                format!("unknown duty binding {duty}"),
            ));
        }
    }
    for required in REQUIRED_PUBLIC_INTEREST_SAFEGUARDS {
        if !claim.safeguards.iter().any(|value| value == required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingSafeguard,
                location.clone(),
                format!("stewardship claim does not bind safeguard {required}"),
            ));
        }
    }
    for required in REQUIRED_ANTI_EXTRACTIVE_DUTIES {
        if !claim.duties.iter().any(|value| value == required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingAntiExtractiveDuty,
                location.clone(),
                format!("stewardship claim does not bind duty {required}"),
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
            "stewardship claim must bind receipt paths",
        ));
    }
    if claim.commands.is_empty() || claim.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location,
            "stewardship claim must bind command records",
        ));
    }
}

fn has_known_evidence(paths: &[String]) -> bool {
    !paths.is_empty()
        && paths.iter().all(|path| {
            path.starts_with("ops/")
                || path.starts_with("interfaces/")
                || path.starts_with("k0/")
                || path.starts_with("fixtures/")
                || path.starts_with("goldens/")
                || path.starts_with("receipts/")
                || path.starts_with("tests/")
                || path.starts_with("src/")
        })
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
