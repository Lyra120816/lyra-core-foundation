use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_error_challenge_evidence::deterministic_error_challenge_evidence_suite_report;
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_error_challenge_evidence::{
    challenge_object_descriptor, challenge_object_digest, diagnostic_object_ref_exists,
    error_object_descriptor, error_object_digest, evidence_object_descriptor,
    evidence_object_digest, object_link_descriptor, object_link_digest,
    validate_error_challenge_evidence_references,
};
use crate::p01_error_challenge_evidence_model::{
    ChallengeObjectBinding, ErrorChallengeEvidenceReceiptBinding, ErrorChallengeEvidenceSurface,
    ErrorObjectBinding, EvidenceObjectBinding, ObjectLinkBinding,
};
use std::collections::{BTreeMap, BTreeSet};
pub const P01_ERROR_CHALLENGE_EVIDENCE_CONTRACT: &str = "LYRA-P01-ERROR-CHALLENGE-EVIDENCE v1";
pub const REQUIRED_ERROR_CHALLENGE_EVIDENCE_RULES: &[&str] = &[
    "error_objects_are_first_class_symbolic_terms",
    "challenge_objects_target_claim_or_error_objects",
    "evidence_objects_are_digest_bound",
    "every_error_object_has_evidence_ref",
    "every_challenge_has_counter_evidence_ref",
    "evidence_sources_are_local_or_receipted",
    "object_links_are_explicit_and_acyclic",
    "canonical_projection_reuses_symbolic_equality",
    "severity_order_is_declared",
    "challenge_adjudication_law_required",
    "receipts_bind_error_challenge_evidence_suite",
    "no_network_dependency",
    "no_probabilistic_adjudication_truth",
    "no_placeholder_objects",
    "no_global_closure_claim",
];
pub const REQUIRED_ERROR_OBJECTS: &[&str] = &[
    "parse_missing_token",
    "type_effect_violation",
    "capability_denied",
    "proof_obligation_unmet",
    "receipt_mismatch",
];
pub const REQUIRED_CHALLENGE_OBJECTS: &[&str] = &[
    "challenge_parse_error",
    "challenge_type_effect",
    "challenge_capability",
    "challenge_proof",
    "challenge_receipt",
];
pub const REQUIRED_EVIDENCE_OBJECTS: &[&str] = &[
    "evidence_parser_replay",
    "evidence_type_trace",
    "evidence_capability_policy",
    "evidence_proof_bundle",
    "evidence_receipt_chain",
];
pub const REQUIRED_OBJECT_LINKS: &[&str] = &[
    "error_parse_supported",
    "error_type_supported",
    "challenge_parse_targets",
    "challenge_parse_countered",
    "receipt_error_supported",
];
pub const REQUIRED_ERROR_CHALLENGE_EVIDENCE_RECEIPTS: &[&str] =
    &["receipt_error_challenge_evidence"];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_ECE_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic adjudication truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "stochastic adjudication",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("random challenge", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder object", ErrorCode::PlaceholderAllowed),
    ("stub object", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_error_challenge_evidence_surface(
    input: &str,
) -> Result<ErrorChallengeEvidenceSurface, Vec<ValidationError>> {
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
            "no error/challenge/evidence lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_ERROR_CHALLENGE_EVIDENCE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_ERROR_CHALLENGE_EVIDENCE_CONTRACT}"),
        )]);
    }
    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut error_objects = Vec::new();
    let mut challenge_objects = Vec::new();
    let mut evidence_objects = Vec::new();
    let mut object_links = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_errors = BTreeSet::new();
    let mut seen_challenges = BTreeSet::new();
    let mut seen_evidence = BTreeSet::new();
    let mut seen_links = BTreeSet::new();
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
            "error_object" => {
                let f = parse_pipe_fields(value);
                let Some(id) = required_field(&f, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "error_object missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_errors.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid error_object {id}"),
                    ));
                    continue;
                }
                error_objects.push(ErrorObjectBinding {
                    line_number,
                    id: id.to_string(),
                    severity: field(&f, "severity"),
                    domain: field(&f, "domain"),
                    subject: field(&f, "subject"),
                    message: field(&f, "message"),
                    evidence_ref: field(&f, "evidence_ref"),
                    digest: field(&f, "digest"),
                    status: field(&f, "status"),
                });
            }
            "challenge_object" => {
                let f = parse_pipe_fields(value);
                let Some(id) = required_field(&f, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidChallengeRight,
                        format!("line:{line_number:03}"),
                        "challenge_object missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_challenges.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateChallengeRight,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid challenge_object {id}"),
                    ));
                    continue;
                }
                challenge_objects.push(ChallengeObjectBinding {
                    line_number,
                    id: id.to_string(),
                    target: field(&f, "target"),
                    challenger: field(&f, "challenger"),
                    claim_ref: field(&f, "claim_ref"),
                    counter_evidence_ref: field(&f, "counter_evidence_ref"),
                    adjudication_law: field(&f, "adjudication_law"),
                    digest: field(&f, "digest"),
                    status: field(&f, "status"),
                });
            }
            "evidence_object" => {
                let f = parse_pipe_fields(value);
                let Some(id) = required_field(&f, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidEvidenceBinding,
                        format!("line:{line_number:03}"),
                        "evidence_object missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_evidence.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateEvidenceBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid evidence_object {id}"),
                    ));
                    continue;
                }
                evidence_objects.push(EvidenceObjectBinding {
                    line_number,
                    id: id.to_string(),
                    kind: field(&f, "kind"),
                    source: field(&f, "source"),
                    payload_digest: field(&f, "payload_digest"),
                    witness: field(&f, "witness"),
                    digest: field(&f, "digest"),
                    status: field(&f, "status"),
                });
            }
            "object_link" => {
                let f = parse_pipe_fields(value);
                let Some(id) = required_field(&f, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "object_link missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_links.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid object_link {id}"),
                    ));
                    continue;
                }
                object_links.push(ObjectLinkBinding {
                    line_number,
                    id: id.to_string(),
                    from: field(&f, "from"),
                    relation: field(&f, "relation"),
                    to: field(&f, "to"),
                    law: field(&f, "law"),
                    digest: field(&f, "digest"),
                    status: field(&f, "status"),
                });
            }
            "receipt" => {
                let f = parse_pipe_fields(value);
                let Some(id) = required_field(&f, "id") else {
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
                receipts.push(ErrorChallengeEvidenceReceiptBinding {
                    line_number,
                    id: id.to_string(),
                    path: field(&f, "path"),
                    target: field(&f, "target"),
                    status: field(&f, "status"),
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
    Ok(ErrorChallengeEvidenceSurface {
        header,
        phase,
        task,
        status,
        rules,
        error_objects,
        challenge_objects,
        evidence_objects,
        object_links,
        receipts,
    })
}

pub fn validate_error_challenge_evidence_surface(input: &str) -> (Verdict, Receipt) {
    let mut errors = Vec::new();
    let lower = input.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_ECE_TEXT {
        if lower.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "surface",
                format!("forbidden token {token}"),
            ));
        }
    }
    match parse_error_challenge_evidence_surface(input) {
        Ok(surface) => validate_surface_model(&surface, &mut errors),
        Err(mut parse_errors) => errors.append(&mut parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}
fn validate_surface_model(
    surface: &ErrorChallengeEvidenceSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "expected P01",
        ));
    }
    if surface.task != "P01-007" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "expected P01-007",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }
    for required in REQUIRED_ERROR_CHALLENGE_EVIDENCE_RULES {
        if !surface.rules.contains_key(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "missing required rule",
            ));
        }
    }
    for required in REQUIRED_ERROR_OBJECTS {
        if surface.error_object_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("error_object:{required}"),
                "missing required error object",
            ));
        }
    }
    for required in REQUIRED_CHALLENGE_OBJECTS {
        if surface.challenge_object_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeRule,
                format!("challenge_object:{required}"),
                "missing required challenge object",
            ));
        }
    }
    for required in REQUIRED_EVIDENCE_OBJECTS {
        if surface.evidence_object_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                format!("evidence_object:{required}"),
                "missing required evidence object",
            ));
        }
    }
    for required in REQUIRED_OBJECT_LINKS {
        if surface.object_link_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("object_link:{required}"),
                "missing required object link",
            ));
        }
    }
    for required in REQUIRED_ERROR_CHALLENGE_EVIDENCE_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "missing required receipt",
            ));
        }
    }
    for item in &surface.error_objects {
        validate_status(&item.status, item.line_number, errors);
        let Some(d) = error_object_descriptor(&item.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", item.line_number),
                format!("unknown error object {}", item.id),
            ));
            continue;
        };
        if item.severity != d.severity
            || item.domain != d.domain
            || item.subject != d.subject
            || item.message != d.message
            || item.evidence_ref != d.evidence_ref
            || item.status != d.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", item.line_number),
                format!("error object {} drifts", item.id),
            ));
        }
        if item.digest != error_object_digest(d) {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", item.line_number),
                format!("error object {} digest mismatch", item.id),
            ));
        }
        if evidence_object_descriptor(&item.evidence_ref).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", item.line_number),
                format!("unknown evidence ref {}", item.evidence_ref),
            ));
        }
    }
    for item in &surface.challenge_objects {
        validate_status(&item.status, item.line_number, errors);
        let Some(d) = challenge_object_descriptor(&item.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", item.line_number),
                format!("unknown challenge object {}", item.id),
            ));
            continue;
        };
        if item.target != d.target
            || item.challenger != d.challenger
            || item.claim_ref != d.claim_ref
            || item.counter_evidence_ref != d.counter_evidence_ref
            || item.adjudication_law != d.adjudication_law
            || item.status != d.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", item.line_number),
                format!("challenge object {} drifts", item.id),
            ));
        }
        if item.digest != challenge_object_digest(d) {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", item.line_number),
                format!("challenge object {} digest mismatch", item.id),
            ));
        }
        if !diagnostic_object_ref_exists(&item.target) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeRight,
                format!("line:{:03}", item.line_number),
                format!("unknown challenge target {}", item.target),
            ));
        }
        if evidence_object_descriptor(&item.counter_evidence_ref).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", item.line_number),
                format!("unknown counter evidence {}", item.counter_evidence_ref),
            ));
        }
    }
    for item in &surface.evidence_objects {
        validate_status(&item.status, item.line_number, errors);
        let Some(d) = evidence_object_descriptor(&item.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEvidenceBinding,
                format!("line:{:03}", item.line_number),
                format!("unknown evidence object {}", item.id),
            ));
            continue;
        };
        if item.kind != d.kind
            || item.source != d.source
            || item.payload_digest != d.payload_digest
            || item.witness != d.witness
            || item.status != d.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", item.line_number),
                format!("evidence object {} drifts", item.id),
            ));
        }
        if item.digest != evidence_object_digest(d) {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", item.line_number),
                format!("evidence object {} digest mismatch", item.id),
            ));
        }
    }
    for item in &surface.object_links {
        validate_status(&item.status, item.line_number, errors);
        let Some(d) = object_link_descriptor(&item.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", item.line_number),
                format!("unknown object link {}", item.id),
            ));
            continue;
        };
        if item.from != d.from
            || item.relation != d.relation
            || item.to != d.to
            || item.law != d.law
            || item.status != d.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", item.line_number),
                format!("object link {} drifts", item.id),
            ));
        }
        if item.digest != object_link_digest(d) {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", item.line_number),
                format!("object link {} digest mismatch", item.id),
            ));
        }
        if !diagnostic_object_ref_exists(&item.from) || !diagnostic_object_ref_exists(&item.to) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                format!("line:{:03}", item.line_number),
                format!("object link {} has unknown endpoint", item.id),
            ));
        }
    }
    for item in &surface.receipts {
        validate_status(&item.status, item.line_number, errors);
        if item.target != "error_challenge_evidence_objects" {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", item.line_number),
                format!("receipt {} target mismatch", item.id),
            ));
        }
        if item.path != "receipts/p01/pass_0036_error_challenge_evidence.receipt" {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", item.line_number),
                format!("receipt {} path mismatch", item.id),
            ));
        }
    }
    if let Err(error) = validate_error_challenge_evidence_references() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            "descriptor_references",
            format!("{error:?}"),
        ));
    }
    let _report = deterministic_error_challenge_evidence_suite_report(
        &surface
            .error_objects
            .iter()
            .map(|i| {
                (
                    i.id.clone(),
                    i.severity.clone(),
                    i.domain.clone(),
                    i.subject.clone(),
                    i.message.clone(),
                    i.evidence_ref.clone(),
                    i.digest.clone(),
                    i.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .challenge_objects
            .iter()
            .map(|i| {
                (
                    i.id.clone(),
                    i.target.clone(),
                    i.challenger.clone(),
                    i.claim_ref.clone(),
                    i.counter_evidence_ref.clone(),
                    i.adjudication_law.clone(),
                    i.digest.clone(),
                    i.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .evidence_objects
            .iter()
            .map(|i| {
                (
                    i.id.clone(),
                    i.kind.clone(),
                    i.source.clone(),
                    i.payload_digest.clone(),
                    i.witness.clone(),
                    i.digest.clone(),
                    i.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .object_links
            .iter()
            .map(|i| {
                (
                    i.id.clone(),
                    i.from.clone(),
                    i.relation.clone(),
                    i.to.clone(),
                    i.law.clone(),
                    i.digest.clone(),
                    i.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .receipts
            .iter()
            .map(|i| {
                (
                    i.id.clone(),
                    i.path.clone(),
                    i.target.clone(),
                    i.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
    );
}
fn validate_status(status: &str, line_number: usize, errors: &mut Vec<ValidationError>) {
    if !ALLOWED_STATUSES.contains(&status) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("line:{line_number:03}"),
            format!("unsupported status {status}"),
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
    fields.get(name).map(String::as_str)
}
fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    required_field(fields, name).unwrap_or("").to_string()
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
        })
}
