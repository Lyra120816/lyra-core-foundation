use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_identity::deterministic_semantic_identity_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_identity::{
    canonical_identity_digest_from_parts, semantic_identity_descriptor, semantic_identity_domains,
};
use crate::p01_semantic_identity_model::{
    SemanticCollisionCaseBinding, SemanticDigestCaseBinding, SemanticIdentityReceiptBinding,
    SemanticIdentityRuleBinding, SemanticIdentitySurface,
};

pub const P01_SEMANTIC_IDENTITY_CONTRACT: &str = "LYRA-P01-SEMANTIC-IDENTITY v1";
pub const REQUIRED_SEMANTIC_IDENTITY_RULES: &[&str] = &[
    "all_identity_domains_declared",
    "symbol_identity_uses_canonical_path",
    "declaration_identity_binds_owner_and_type",
    "rewrite_identity_binds_lhs_rhs_law",
    "witness_row_identity_binds_trace_and_index",
    "artifact_identity_binds_path_and_digest",
    "digest_algorithm_is_labeled_fnv1a128",
    "canonical_preimage_is_byte_stable",
    "collision_equal_digest_unequal_preimage_rejected",
    "receipts_bind_identity_suite",
    "no_network_dependency",
    "no_probabilistic_identity_truth",
    "no_placeholder_identity",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_IDENTITY_DOMAINS: &[&str] = &[
    "symbol",
    "declaration",
    "rewrite",
    "witness_row",
    "artifact",
];
pub const REQUIRED_SEMANTIC_DIGEST_CASES: &[&str] = &[
    "symbol_core",
    "declaration_core_binding",
    "rewrite_beta_reduce",
    "witness_row_trace_000",
    "artifact_core_contract",
];
pub const REQUIRED_SEMANTIC_COLLISION_CASES: &[&str] = &[
    "symbol_collision_guard",
    "declaration_collision_guard",
    "rewrite_collision_guard",
    "witness_row_collision_guard",
    "artifact_collision_guard",
];
pub const REQUIRED_SEMANTIC_IDENTITY_RECEIPTS: &[&str] = &["receipt_semantic_identity"];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];
const REQUIRED_DIGEST: &str = "fnv1a128_labeled";
const REQUIRED_COLLISION_LAW: &str = "reject_equal_digest_unequal_preimage";
const FORBIDDEN_SEMANTIC_IDENTITY_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic identity truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic identity", ErrorCode::ProbabilisticTruthAllowed),
    ("random identity", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder identity", ErrorCode::PlaceholderAllowed),
    ("stub identity", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_identity_surface(
    input: &str,
) -> Result<SemanticIdentitySurface, Vec<ValidationError>> {
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
            "no semantic identity lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_IDENTITY_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_IDENTITY_CONTRACT}"),
        )]);
    }
    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut identities = Vec::new();
    let mut digest_cases = Vec::new();
    let mut collision_cases = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_identities = BTreeSet::new();
    let mut seen_digest_cases = BTreeSet::new();
    let mut seen_collision_cases = BTreeSet::new();
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
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "semantic identity rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("identity:") {
            if !is_symbolic_name(id) || !seen_identities.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateCanonicalModel,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic identity id {id}"),
                ));
                continue;
            }
            match parse_identity_binding(line_number, id, value) {
                Ok(row) => identities.push(row),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("digest_case:") {
            if !is_symbolic_name(id) || !seen_digest_cases.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid digest case id {id}"),
                ));
                continue;
            }
            match parse_digest_case_binding(line_number, id, value) {
                Ok(row) => digest_cases.push(row),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("collision:") {
            if !is_symbolic_name(id) || !seen_collision_cases.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid collision case id {id}"),
                ));
                continue;
            }
            match parse_collision_case_binding(line_number, id, value) {
                Ok(row) => collision_cases.push(row),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(id) || !seen_receipts.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateProofBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic identity receipt id {id}"),
                ));
                continue;
            }
            match parse_receipt_binding(line_number, id, value) {
                Ok(row) => receipts.push(row),
                Err(error) => errors.push(error),
            }
            continue;
        }
        match left {
            "phase" => set_scalar(
                &mut phase,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            "task" => set_scalar(
                &mut task,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            "status" => set_scalar(
                &mut status,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown semantic identity entry {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticIdentitySurface {
        header,
        phase: phase.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingPhase,
                "phase",
                "missing phase",
            )]
        })?,
        task: task.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingTask,
                "task",
                "missing task",
            )]
        })?,
        status: status.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                "status",
                "missing status",
            )]
        })?,
        rules,
        identities,
        digest_cases,
        collision_cases,
        receipts,
    })
}

pub fn validate_semantic_identity_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_semantic_identity_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_surface_header(&parsed, &mut errors);
    validate_required_rules(&parsed, &mut errors);
    validate_required_identities(&parsed, &mut errors);
    validate_required_digest_cases(&parsed, &mut errors);
    validate_required_collision_cases(&parsed, &mut errors);
    validate_required_receipts(&parsed, &mut errors);
    validate_identity_descriptors(&parsed, &mut errors);
    validate_digest_cases(&parsed, &mut errors);
    validate_collision_cases(&parsed, &mut errors);
    validate_receipts(&parsed, &mut errors);
    validate_semantic_identity_report(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_surface_header(surface: &SemanticIdentitySurface, errors: &mut Vec<ValidationError>) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-004" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-004 got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("expected artifact_emitted got {}", surface.status),
        ));
    }
}
fn validate_required_rules(surface: &SemanticIdentitySurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_IDENTITY_RULES {
        if !surface.rules.contains_key(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "missing semantic identity rule",
            ));
        }
    }
}
fn validate_required_identities(
    surface: &SemanticIdentitySurface,
    errors: &mut Vec<ValidationError>,
) {
    for required in REQUIRED_SEMANTIC_IDENTITY_DOMAINS {
        if !surface
            .identities
            .iter()
            .any(|item| item.id == *required && item.domain == *required)
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("identity:{required}"),
                "missing required semantic identity domain",
            ));
        }
    }
}
fn validate_required_digest_cases(
    surface: &SemanticIdentitySurface,
    errors: &mut Vec<ValidationError>,
) {
    for required in REQUIRED_SEMANTIC_DIGEST_CASES {
        if !surface.digest_cases.iter().any(|item| item.id == *required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("digest_case:{required}"),
                "missing required semantic digest case",
            ));
        }
    }
    for required_domain in REQUIRED_SEMANTIC_IDENTITY_DOMAINS {
        if !surface
            .digest_cases
            .iter()
            .any(|item| item.domain == *required_domain)
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("digest_case:{required_domain}"),
                "missing digest case for semantic identity domain",
            ));
        }
    }
}
fn validate_required_collision_cases(
    surface: &SemanticIdentitySurface,
    errors: &mut Vec<ValidationError>,
) {
    for required in REQUIRED_SEMANTIC_COLLISION_CASES {
        if !surface
            .collision_cases
            .iter()
            .any(|item| item.id == *required)
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("collision:{required}"),
                "missing required semantic collision case",
            ));
        }
    }
    for required_domain in REQUIRED_SEMANTIC_IDENTITY_DOMAINS {
        if !surface
            .collision_cases
            .iter()
            .any(|item| item.domain == *required_domain)
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("collision:{required_domain}"),
                "missing collision case for semantic identity domain",
            ));
        }
    }
}
fn validate_required_receipts(
    surface: &SemanticIdentitySurface,
    errors: &mut Vec<ValidationError>,
) {
    for required in REQUIRED_SEMANTIC_IDENTITY_RECEIPTS {
        if !surface.receipts.iter().any(|item| item.id == *required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "missing required semantic identity receipt",
            ));
        }
    }
}
fn validate_identity_descriptors(
    surface: &SemanticIdentitySurface,
    errors: &mut Vec<ValidationError>,
) {
    for identity in &surface.identities {
        let Some(descriptor) = semantic_identity_descriptor(&identity.domain) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                identity.canonical_identity(),
                format!("unknown semantic identity domain {}", identity.domain),
            ));
            continue;
        };
        if identity.id != identity.domain {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                identity.canonical_identity(),
                "identity row id must equal domain",
            ));
        }
        if identity.scope != descriptor.scope {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                identity.canonical_identity(),
                format!(
                    "scope drift expected {} got {}",
                    descriptor.scope, identity.scope
                ),
            ));
        }
        if identity.material != descriptor.material {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                identity.canonical_identity(),
                format!(
                    "material drift expected {} got {}",
                    descriptor.material, identity.material
                ),
            ));
        }
        if identity.canonicalizer != descriptor.canonicalizer {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                identity.canonical_identity(),
                format!(
                    "canonicalizer drift expected {} got {}",
                    descriptor.canonicalizer, identity.canonicalizer
                ),
            ));
        }
        if identity.digest != descriptor.digest || identity.digest != REQUIRED_DIGEST {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                identity.canonical_identity(),
                format!(
                    "digest drift expected {} got {}",
                    descriptor.digest, identity.digest
                ),
            ));
        }
        if identity.collision != descriptor.collision_law
            || identity.collision != REQUIRED_COLLISION_LAW
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                identity.canonical_identity(),
                format!(
                    "collision law drift expected {} got {}",
                    descriptor.collision_law, identity.collision
                ),
            ));
        }
        if !ALLOWED_STATUSES.contains(&identity.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                identity.canonical_identity(),
                format!("invalid identity status {}", identity.status),
            ));
        }
    }
    for required in semantic_identity_domains() {
        if !surface
            .identities
            .iter()
            .any(|item| item.domain == required)
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("identity:{required}"),
                "missing LyraLang semantic identity descriptor",
            ));
        }
    }
}
fn validate_digest_cases(surface: &SemanticIdentitySurface, errors: &mut Vec<ValidationError>) {
    for digest_case in &surface.digest_cases {
        if surface.identity_by_id(&digest_case.domain).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                digest_case.canonical_identity(),
                format!("unknown digest case domain {}", digest_case.domain),
            ));
            continue;
        }
        if !is_symbolic_name(&digest_case.owner)
            || !is_symbolic_name(&digest_case.payload)
            || !is_symbolic_name(&digest_case.normalization)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                digest_case.canonical_identity(),
                "digest case fields must be symbolic names",
            ));
        }
        let expected = match canonical_identity_digest_from_parts(
            &digest_case.domain,
            &digest_case.id,
            &digest_case.owner,
            &digest_case.payload,
            &digest_case.normalization,
        ) {
            Ok(digest) => digest,
            Err(error) => {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidModelBinding,
                    digest_case.canonical_identity(),
                    format!("digest case material rejected: {error:?}"),
                ));
                continue;
            }
        };
        if digest_case.expected_digest != expected {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                digest_case.canonical_identity(),
                format!(
                    "expected digest {} got {}",
                    expected, digest_case.expected_digest
                ),
            ));
        }
        if !digest_case.expected_digest.starts_with("fnv1a128:") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                digest_case.canonical_identity(),
                "digest case must use fnv1a128 label",
            ));
        }
        if !ALLOWED_STATUSES.contains(&digest_case.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                digest_case.canonical_identity(),
                format!("invalid digest case status {}", digest_case.status),
            ));
        }
    }
}
fn validate_collision_cases(surface: &SemanticIdentitySurface, errors: &mut Vec<ValidationError>) {
    for collision_case in &surface.collision_cases {
        if surface.identity_by_id(&collision_case.domain).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                collision_case.canonical_identity(),
                format!("unknown collision case domain {}", collision_case.domain),
            ));
        }
        if !is_symbolic_name(&collision_case.left)
            || !is_symbolic_name(&collision_case.right)
            || !is_symbolic_name(&collision_case.law)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                collision_case.canonical_identity(),
                "collision fields must be symbolic names",
            ));
        }
        if collision_case.left == collision_case.right {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                collision_case.canonical_identity(),
                "collision case must compare different preimages",
            ));
        }
        if collision_case.law != REQUIRED_COLLISION_LAW {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                collision_case.canonical_identity(),
                format!("collision law must be {REQUIRED_COLLISION_LAW}"),
            ));
        }
        if !ALLOWED_STATUSES.contains(&collision_case.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                collision_case.canonical_identity(),
                format!("invalid collision case status {}", collision_case.status),
            ));
        }
    }
}
fn validate_receipts(surface: &SemanticIdentitySurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p01/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                receipt.canonical_identity(),
                format!("receipt path must be a P01 receipt: {}", receipt.path),
            ));
        }
        if receipt.target != "semantic_identity"
            && surface.identity_by_id(&receipt.target).is_none()
            && surface.digest_case_by_id(&receipt.target).is_none()
            && surface.collision_case_by_id(&receipt.target).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                receipt.canonical_identity(),
                format!("unknown receipt target {}", receipt.target),
            ));
        }
        if !ALLOWED_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!("invalid receipt status {}", receipt.status),
            ));
        }
    }
}
fn validate_semantic_identity_report(
    surface: &SemanticIdentitySurface,
    errors: &mut Vec<ValidationError>,
) {
    let identity_inputs: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .identities
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.domain.clone(),
                item.scope.clone(),
                item.material.clone(),
                item.canonicalizer.clone(),
                item.digest.clone(),
                item.collision.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let digest_inputs: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .digest_cases
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.domain.clone(),
                item.owner.clone(),
                item.payload.clone(),
                item.normalization.clone(),
                item.expected_digest.clone(),
                item.status.clone(),
                item.line_number.to_string(),
            )
        })
        .collect();
    let collision_inputs: Vec<(String, String, String, String, String, String)> = surface
        .collision_cases
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.domain.clone(),
                item.left.clone(),
                item.right.clone(),
                item.law.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let receipt_inputs: Vec<(String, String, String, String)> = surface
        .receipts
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.path.clone(),
                item.target.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let report = deterministic_semantic_identity_suite_report(
        &identity_inputs,
        &digest_inputs,
        &collision_inputs,
        &receipt_inputs,
    );
    if report.identity_count != surface.identities.len()
        || report.digest_case_count != surface.digest_cases.len()
        || report.collision_case_count != surface.collision_cases.len()
        || report.receipt_count != surface.receipts.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_semantic_identity_report",
            "semantic identity report count mismatch",
        ));
    }
    if report.identity_count != REQUIRED_SEMANTIC_IDENTITY_DOMAINS.len()
        || report.digest_case_count != REQUIRED_SEMANTIC_DIGEST_CASES.len()
        || report.collision_case_count != REQUIRED_SEMANTIC_COLLISION_CASES.len()
        || report.receipt_count != REQUIRED_SEMANTIC_IDENTITY_RECEIPTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCanonicalModel,
            "k0_semantic_identity_report",
            "semantic identity report does not cover required P01-004 identity/digest suite",
        ));
    }
    if report.stable_digest_count != REQUIRED_SEMANTIC_DIGEST_CASES.len() {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_semantic_identity_report",
            "all semantic identity digest cases must emit stable fnv1a128 labels",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_semantic_identity_report",
            "semantic identity suite hash must be stable fnv1a128",
        ));
    }
}
fn parse_identity_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticIdentityRuleBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid identity field map",
        )
    })?;
    Ok(SemanticIdentityRuleBinding {
        line_number,
        id: id.to_string(),
        domain: required_field(
            &fields,
            "domain",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        material: required_field(
            &fields,
            "material",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        canonicalizer: required_field(
            &fields,
            "canonicalizer",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        digest: required_field(
            &fields,
            "digest",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        collision: required_field(
            &fields,
            "collision",
            ErrorCode::InvalidCanonicalModel,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}
fn parse_digest_case_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticDigestCaseBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid digest case field map",
        )
    })?;
    Ok(SemanticDigestCaseBinding {
        line_number,
        id: id.to_string(),
        domain: required_field(
            &fields,
            "domain",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        owner: required_field(
            &fields,
            "owner",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        payload: required_field(
            &fields,
            "payload",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        normalization: required_field(
            &fields,
            "normalization",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        expected_digest: required_field(
            &fields,
            "expected_digest",
            ErrorCode::InvalidProofBinding,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}
fn parse_collision_case_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticCollisionCaseBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid collision field map",
        )
    })?;
    Ok(SemanticCollisionCaseBinding {
        line_number,
        id: id.to_string(),
        domain: required_field(
            &fields,
            "domain",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        left: required_field(&fields, "left", ErrorCode::InvalidModelBinding, line_number)?,
        right: required_field(
            &fields,
            "right",
            ErrorCode::InvalidModelBinding,
            line_number,
        )?,
        law: required_field(&fields, "law", ErrorCode::InvalidModelBinding, line_number)?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}
fn parse_receipt_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticIdentityReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            format!("line:{line_number:03}"),
            "invalid receipt field map",
        )
    })?;
    Ok(SemanticIdentityReceiptBinding {
        line_number,
        id: id.to_string(),
        path: required_field(&fields, "path", ErrorCode::InvalidProofBinding, line_number)?,
        target: required_field(
            &fields,
            "target",
            ErrorCode::InvalidProofBinding,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}
fn set_scalar(
    target: &mut Option<String>,
    value: &str,
    key: &str,
    line_number: usize,
    seen_scalars: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    if !seen_scalars.insert(key.to_string()) || target.is_some() {
        errors.push(ValidationError::reject(
            ErrorCode::DuplicateEntry,
            format!("line:{line_number:03}"),
            format!("duplicate scalar {key}"),
        ));
    } else {
        *target = Some(value.to_string());
    }
}
fn parse_field_map(value: &str) -> Option<BTreeMap<String, String>> {
    let mut output = BTreeMap::new();
    for segment in value.split('|') {
        let (key, val) = segment.split_once(':')?;
        if key.is_empty() || val.is_empty() || key != key.trim() || val != val.trim() {
            return None;
        }
        if output.insert(key.to_string(), val.to_string()).is_some() {
            return None;
        }
    }
    Some(output)
}
fn required_field(
    fields: &BTreeMap<String, String>,
    name: &str,
    code: ErrorCode,
    line_number: usize,
) -> Result<String, ValidationError> {
    fields
        .get(name)
        .cloned()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ValidationError::reject(
                code,
                format!("line:{line_number:03}"),
                format!("missing field {name}"),
            )
        })
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'.'
        })
}
fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_IDENTITY_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic identity token {needle}"),
            ));
        }
    }
}
