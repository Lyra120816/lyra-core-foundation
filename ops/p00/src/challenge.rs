use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_challenge_model::{
    AmendmentGate, ChallengeLawSurface, ChallengeRight, ReviewGate, RollbackAuthority,
};

pub const P00_CHALLENGE_LAW_CONTRACT: &str = "LYRA-P00-CHALLENGE-LAW v1";

pub const REQUIRED_CHALLENGE_RULES: &[&str] = &[
    "red_team_review_required",
    "challenge_rights_required",
    "rollback_authority_required",
    "constitutional_amendment_required",
    "deterministic_evidence_required",
    "non_retaliation_required",
    "authority_order_preserved",
    "archive_cannot_be_primary",
    "bounded_amendment_only",
    "receipt_backed_rollback",
];

pub const REQUIRED_REVIEW_GATES: &[&str] = &["frontier_red_team", "closure_red_team"];

pub const REQUIRED_CHALLENGE_RIGHTS: &[&str] = &["operator_challenge", "public_interest_challenge"];

pub const REQUIRED_ROLLBACK_AUTHORITIES: &[&str] =
    &["frontier_rollback", "truth_snapshot_rollback"];

pub const REQUIRED_AMENDMENT_GATES: &[&str] =
    &["constitutional_amendment", "supersession_amendment"];

const REQUIRED_ROLLBACK_REQUIREMENTS: &[&str] = &["receipt", "truth_snapshot", "blocker_index"];

const REQUIRED_AMENDMENT_REQUIREMENTS: &[&str] = &[
    "authority_order",
    "red_team_review",
    "receipt",
    "truth_snapshot",
];

const REQUIRED_AMENDMENT_FORBIDS: &[&str] = &[
    "ambient_override",
    "archive_primary",
    "silent_supersession",
    "unbounded_scope",
];

const FORBIDDEN_CHALLENGE_TEXT: &[(&str, ErrorCode)] = &[
    ("todo", ErrorCode::ForbiddenToken),
    ("tbd", ErrorCode::ForbiddenToken),
    ("not implemented", ErrorCode::ForbiddenToken),
    ("will add later", ErrorCode::ForbiddenToken),
    ("finish later", ErrorCode::ForbiddenToken),
    ("retaliation allowed", ErrorCode::RetaliationAllowed),
    ("retaliation_allowed", ErrorCode::RetaliationAllowed),
    (
        "rollback without receipt",
        ErrorCode::RollbackWithoutReceipt,
    ),
    (
        "rollback_without_receipt",
        ErrorCode::RollbackWithoutReceipt,
    ),
    (
        "amend without authority",
        ErrorCode::AmendmentAuthorityBypass,
    ),
    ("authority bypass", ErrorCode::AmendmentAuthorityBypass),
    ("archive primary", ErrorCode::ArchivePrimaryAmendment),
    ("unbounded amendment", ErrorCode::UnboundedAmendment),
    ("unbounded_amendment", ErrorCode::UnboundedAmendment),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_challenge_law_surface(
    input: &str,
) -> Result<ChallengeLawSurface, Vec<ValidationError>> {
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
            "no challenge law surface lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_CHALLENGE_LAW_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_CHALLENGE_LAW_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut reviews = Vec::new();
    let mut challenges = Vec::new();
    let mut rollbacks = Vec::new();
    let mut amendments = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_reviews = BTreeSet::new();
    let mut seen_challenges = BTreeSet::new();
    let mut seen_rollbacks = BTreeSet::new();
    let mut seen_amendments = BTreeSet::new();

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
                    "challenge rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(review_id) = left.strip_prefix("review:") {
            if !is_symbolic_name(review_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReviewGate,
                    format!("line:{line_number:03}"),
                    format!("invalid review gate identity {review_id}"),
                ));
                continue;
            }
            if !seen_reviews.insert(review_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReviewGate,
                    format!("review:{review_id}"),
                    "review gate identity must be unique",
                ));
                continue;
            }
            match parse_review(line_number, review_id, value) {
                Ok(review) => reviews.push(review),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(challenge_id) = left.strip_prefix("challenge:") {
            if !is_symbolic_name(challenge_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidChallengeRight,
                    format!("line:{line_number:03}"),
                    format!("invalid challenge right identity {challenge_id}"),
                ));
                continue;
            }
            if !seen_challenges.insert(challenge_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateChallengeRight,
                    format!("challenge:{challenge_id}"),
                    "challenge right identity must be unique",
                ));
                continue;
            }
            match parse_challenge(line_number, challenge_id, value) {
                Ok(challenge) => challenges.push(challenge),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(rollback_id) = left.strip_prefix("rollback:") {
            if !is_symbolic_name(rollback_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRollbackAuthority,
                    format!("line:{line_number:03}"),
                    format!("invalid rollback authority identity {rollback_id}"),
                ));
                continue;
            }
            if !seen_rollbacks.insert(rollback_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRollbackAuthority,
                    format!("rollback:{rollback_id}"),
                    "rollback authority identity must be unique",
                ));
                continue;
            }
            match parse_rollback(line_number, rollback_id, value) {
                Ok(rollback) => rollbacks.push(rollback),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(amendment_id) = left.strip_prefix("amendment:") {
            if !is_symbolic_name(amendment_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidAmendmentGate,
                    format!("line:{line_number:03}"),
                    format!("invalid amendment gate identity {amendment_id}"),
                ));
                continue;
            }
            if !seen_amendments.insert(amendment_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateAmendmentGate,
                    format!("amendment:{amendment_id}"),
                    "amendment gate identity must be unique",
                ));
                continue;
            }
            match parse_amendment(line_number, amendment_id, value) {
                Ok(amendment) => amendments.push(amendment),
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
                format!("unknown challenge law field {left}"),
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
                "task=P00-006 is required",
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
        Ok(ChallengeLawSurface {
            header,
            phase,
            task,
            status,
            rules,
            reviews,
            challenges,
            rollbacks,
            amendments,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_challenge_law_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_challenge_law_surface(input) {
        Ok(surface) => validate_parsed_challenge_law_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_review(line_number: usize, id: &str, value: &str) -> Result<ReviewGate, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let required_before = required_string_field(line_number, &mut fields, "required_before")?;
    let reviewers = required_list_field(line_number, &mut fields, "reviewers")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ReviewGate {
        line_number,
        id: id.to_string(),
        scope,
        required_before,
        reviewers,
        evidence,
        status,
    })
}

fn parse_challenge(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ChallengeRight, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let holder = required_string_field(line_number, &mut fields, "holder")?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let trigger = required_string_field(line_number, &mut fields, "trigger")?;
    let remedy = required_string_field(line_number, &mut fields, "remedy")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    let protection = required_string_field(line_number, &mut fields, "protection")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ChallengeRight {
        line_number,
        id: id.to_string(),
        holder,
        scope,
        trigger,
        remedy,
        evidence,
        protection,
    })
}

fn parse_rollback(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<RollbackAuthority, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let holder = required_string_field(line_number, &mut fields, "holder")?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let target = required_string_field(line_number, &mut fields, "target")?;
    let requires = required_list_field(line_number, &mut fields, "requires")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(RollbackAuthority {
        line_number,
        id: id.to_string(),
        holder,
        scope,
        target,
        requires,
        evidence,
        status,
    })
}

fn parse_amendment(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<AmendmentGate, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let requires = required_list_field(line_number, &mut fields, "requires")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(AmendmentGate {
        line_number,
        id: id.to_string(),
        scope,
        requires,
        forbids,
        evidence,
        status,
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
                "challenge law attributes must use key:value fields",
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
            "challenge law surface contains unsupported attributes",
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

fn validate_parsed_challenge_law_surface(
    surface: &ChallengeLawSurface,
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
    if surface.task != "P00-006" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-006, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-006 may only declare working_slice status",
        ));
    }

    for rule in REQUIRED_CHALLENGE_RULES {
        match surface.rule_value(rule) {
            Some(value) if rule_value_satisfies(rule, value) => {}
            Some(_) | None => errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeRule,
                format!("rule:{rule}"),
                "required challenge law rule is absent or too weak",
            )),
        }
    }

    for required in REQUIRED_REVIEW_GATES {
        if surface.review_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReviewGate,
                format!("review:{required}"),
                "required review gate is absent",
            ));
        }
    }
    for required in REQUIRED_CHALLENGE_RIGHTS {
        if surface.challenge_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeRight,
                format!("challenge:{required}"),
                "required challenge right is absent",
            ));
        }
    }
    for required in REQUIRED_ROLLBACK_AUTHORITIES {
        if surface.rollback_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRollbackAuthority,
                format!("rollback:{required}"),
                "required rollback authority is absent",
            ));
        }
    }
    for required in REQUIRED_AMENDMENT_GATES {
        if surface.amendment_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingAmendmentGate,
                format!("amendment:{required}"),
                "required amendment gate is absent",
            ));
        }
    }

    for review in &surface.reviews {
        validate_review(review, &mut errors);
    }
    for challenge in &surface.challenges {
        validate_challenge(challenge, &mut errors);
    }
    for rollback in &surface.rollbacks {
        validate_rollback(rollback, &mut errors);
    }
    for amendment in &surface.amendments {
        validate_amendment(amendment, &mut errors);
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_CHALLENGE_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "challenge:text",
                format!("forbidden challenge law phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_review(review: &ReviewGate, errors: &mut Vec<ValidationError>) {
    let location = review.canonical_identity();
    match review.scope.as_str() {
        "frontier" | "task" | "phase" => {}
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidReviewGate,
            location.clone(),
            format!("unsupported review scope {other}"),
        )),
    }
    match review.required_before.as_str() {
        "closure_claim" | "frontier_advance" | "phase_close" => {}
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidReviewGate,
            location.clone(),
            format!("unsupported review gate target {other}"),
        )),
    }
    if review.reviewers.iter().any(|reviewer| weak_value(reviewer)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReviewGate,
            location.clone(),
            "review gate must name concrete reviewers",
        ));
    }
    require_receipt_evidence(
        &review.evidence,
        ErrorCode::InvalidReviewGate,
        location.clone(),
        errors,
    );
    match review.status.as_str() {
        "active" | "required" => {}
        "closed" | "complete" | "global_complete" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location,
            "review gate cannot close P00",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location,
            format!("unsupported review status {other}"),
        )),
    }
}

fn validate_challenge(challenge: &ChallengeRight, errors: &mut Vec<ValidationError>) {
    let location = challenge.canonical_identity();
    match challenge.holder.as_str() {
        "operator" | "public" | "red_team" | "maintainer" => {}
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeRight,
            location.clone(),
            format!("unsupported challenge holder {other}"),
        )),
    }
    match challenge.scope.as_str() {
        "frontier" | "task" | "phase" | "constitution" => {}
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeRight,
            location.clone(),
            format!("unsupported challenge scope {other}"),
        )),
    }
    if weak_value(&challenge.trigger) || weak_value(&challenge.remedy) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeRight,
            location.clone(),
            "challenge right must bind concrete trigger and remedy",
        ));
    }
    if challenge.protection != "non_retaliation" {
        errors.push(ValidationError::reject(
            ErrorCode::RetaliationAllowed,
            location.clone(),
            "challenge right must protect non-retaliation",
        ));
    }
    require_receipt_evidence(
        &challenge.evidence,
        ErrorCode::InvalidChallengeRight,
        location,
        errors,
    );
}

fn validate_rollback(rollback: &RollbackAuthority, errors: &mut Vec<ValidationError>) {
    let location = rollback.canonical_identity();
    match rollback.holder.as_str() {
        "operator" | "red_team" | "maintainer" => {}
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidRollbackAuthority,
            location.clone(),
            format!("unsupported rollback holder {other}"),
        )),
    }
    match rollback.scope.as_str() {
        "frontier" | "truth_plane" | "task" => {}
        "phase" | "global" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "P00-006 rollback authority cannot claim phase/global closure",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidRollbackAuthority,
            location.clone(),
            format!("unsupported rollback scope {other}"),
        )),
    }
    if weak_value(&rollback.target) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRollbackAuthority,
            location.clone(),
            "rollback target must be concrete",
        ));
    }
    for requirement in REQUIRED_ROLLBACK_REQUIREMENTS {
        if !rollback
            .requires
            .iter()
            .any(|value| value.as_str() == *requirement)
        {
            errors.push(ValidationError::reject(
                ErrorCode::RollbackWithoutReceipt,
                location.clone(),
                format!("rollback must require {requirement}"),
            ));
        }
    }
    require_receipt_evidence(
        &rollback.evidence,
        ErrorCode::RollbackWithoutReceipt,
        location.clone(),
        errors,
    );
    match rollback.status.as_str() {
        "active" | "required" => {}
        "closed" | "complete" | "global_complete" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location,
            "rollback authority cannot close P00",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location,
            format!("unsupported rollback status {other}"),
        )),
    }
}

fn validate_amendment(amendment: &AmendmentGate, errors: &mut Vec<ValidationError>) {
    let location = amendment.canonical_identity();
    match amendment.scope.as_str() {
        "constitution" | "authority_order" | "supersession" => {}
        "global" | "unbounded" => errors.push(ValidationError::reject(
            ErrorCode::UnboundedAmendment,
            location.clone(),
            "amendment scope must be bounded",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidAmendmentGate,
            location.clone(),
            format!("unsupported amendment scope {other}"),
        )),
    }
    for requirement in REQUIRED_AMENDMENT_REQUIREMENTS {
        if !amendment
            .requires
            .iter()
            .any(|value| value.as_str() == *requirement)
        {
            errors.push(ValidationError::reject(
                ErrorCode::AmendmentAuthorityBypass,
                location.clone(),
                format!("amendment must require {requirement}"),
            ));
        }
    }
    for forbidden in REQUIRED_AMENDMENT_FORBIDS {
        if !amendment
            .forbids
            .iter()
            .any(|value| value.as_str() == *forbidden)
        {
            let code = if *forbidden == "archive_primary" {
                ErrorCode::ArchivePrimaryAmendment
            } else {
                ErrorCode::AmendmentAuthorityBypass
            };
            errors.push(ValidationError::reject(
                code,
                location.clone(),
                format!("amendment must forbid {forbidden}"),
            ));
        }
    }
    require_receipt_evidence(
        &amendment.evidence,
        ErrorCode::InvalidAmendmentGate,
        location.clone(),
        errors,
    );
    match amendment.status.as_str() {
        "bounded" | "required" | "active" => {}
        "closed" | "complete" | "global_complete" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location,
            "amendment gate cannot close P00",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location,
            format!("unsupported amendment status {other}"),
        )),
    }
}

fn require_receipt_evidence(
    evidence: &[String],
    code: ErrorCode,
    location: String,
    errors: &mut Vec<ValidationError>,
) {
    let has_receipt = evidence.iter().any(|item| item.ends_with(".receipt"));
    let has_test_or_fixture = evidence
        .iter()
        .any(|item| item.starts_with("tests/") || item.starts_with("fixtures/"));
    if !has_receipt || !has_test_or_fixture {
        errors.push(ValidationError::reject(
            code,
            location,
            "evidence must bind at least one receipt and one test or fixture path",
        ));
    }
}

fn rule_value_satisfies(rule: &str, value: &str) -> bool {
    match rule {
        "archive_cannot_be_primary" => value.contains("forbidden"),
        "non_retaliation_required" => value.contains("required"),
        "bounded_amendment_only" => value.contains("required"),
        _ => value.contains("required"),
    }
}

fn weak_value(value: &str) -> bool {
    matches!(
        value,
        "none"
            | "nothing"
            | "declared_only"
            | "documentation_only"
            | "docs_only"
            | "thin_patch"
            | "unbounded"
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
