use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_proof_family::deterministic_semantic_proof_family_table_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_proof_family_model::{
    SemanticProofFamilyBinding, SemanticProofFamilyTableSurface, SemanticProofPathBinding,
    SemanticProofReceiptBinding,
};

pub const P01_SEMANTIC_PROOF_FAMILY_CONTRACT: &str = "LYRA-P01-SEMANTIC-PROOF-FAMILY-TABLE v1";
pub const REQUIRED_SEMANTIC_PROOF_FAMILY_RULES: &[&str] = &[
    "semantic_proof_family_table_must_cover_required_families",
    "semantic_happy_path_receipts_must_bind_primary_and_closure_success",
    "semantic_negative_path_receipts_must_bind_rejection_corpus",
    "semantic_adversarial_path_receipts_must_bind_challenge_and_redteam",
    "semantic_rollback_path_receipts_must_bind_replay_and_rollback",
    "semantic_proof_paths_must_bind_entry_challenge_and_rollback",
    "no_network_dependency",
    "no_docs_only_proof_table",
    "no_unreceipted_proof_family",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_PROOF_FAMILIES: &[&str] = &[
    "happy_path",
    "negative_path",
    "adversarial_path",
    "rollback_path",
];
pub const REQUIRED_SEMANTIC_PROOF_PATHS: &[&str] = &[
    "semantic_happy_primary_chain",
    "semantic_negative_rejection_chain",
    "semantic_adversarial_challenge_chain",
    "semantic_rollback_replay_chain",
];
pub const REQUIRED_SEMANTIC_PROOF_RECEIPTS: &[&str] = &[
    "receipt_semantic_atoms",
    "receipt_core_ir",
    "receipt_semantic_objects",
    "receipt_semantic_identity",
    "receipt_reference_semantics",
    "receipt_symbolic_equality",
    "receipt_error_challenge_evidence",
    "receipt_semantic_serialization_hashing",
    "receipt_semantic_adversarial_corpus",
    "receipt_core_ir_reuse",
    "receipt_semantic_atom_reference",
    "receipt_semantic_bedrock_receipts",
    "receipt_formal_semantic_constitution",
    "receipt_canonical_data_model",
    "receipt_semantic_core_engine",
    "receipt_semantic_falsification",
    "receipt_semantic_replay",
    "receipt_semantic_interface",
    "receipt_semantic_packaging",
    "receipt_semantic_deployment",
    "receipt_semantic_ecosystem",
    "receipt_semantic_economics",
    "receipt_semantic_redteam",
    "receipt_semantic_closure",
    "receipt_semantic_dependency_matrix",
];
const REQUIRED_HAPPY_RECEIPTS: &[&str] = &[
    "receipt_semantic_atoms",
    "receipt_core_ir",
    "receipt_semantic_objects",
    "receipt_semantic_identity",
    "receipt_reference_semantics",
    "receipt_symbolic_equality",
    "receipt_error_challenge_evidence",
    "receipt_semantic_serialization_hashing",
    "receipt_core_ir_reuse",
    "receipt_semantic_atom_reference",
    "receipt_semantic_bedrock_receipts",
    "receipt_formal_semantic_constitution",
    "receipt_canonical_data_model",
    "receipt_semantic_core_engine",
    "receipt_semantic_interface",
    "receipt_semantic_packaging",
    "receipt_semantic_deployment",
    "receipt_semantic_ecosystem",
    "receipt_semantic_economics",
    "receipt_semantic_dependency_matrix",
];
const REQUIRED_COVERAGE_TARGETS: &[&str] = &[
    "P01-001", "P01-002", "P01-003", "P01-004", "P01-005", "P01-006", "P01-007", "P01-008",
    "P01-009", "P01-010", "P01-011", "P01-012", "P01-013", "P01-014", "P01-015", "P01-016",
    "P01-017", "P01-018", "P01-019", "P01-020", "P01-021", "P01-022", "P01-023", "P01-024",
    "P01-X01",
];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "bounded_closed"];
const ALLOWED_VERDICTS: &[&str] = &["accepted", "rejected_expected"];

pub fn parse_semantic_proof_family_table_surface(
    input: &str,
) -> Result<SemanticProofFamilyTableSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => {
            return Err(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "input",
                format!("canonicalization failed: {error:?}"),
            )])
        }
    };
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "semantic proof family table surface is empty",
        )]);
    }
    let mut errors = Vec::new();
    let header = lines[0].clone();
    if header != P01_SEMANTIC_PROOF_FAMILY_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_PROOF_FAMILY_CONTRACT}"),
        ));
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut families = Vec::new();
    let mut receipts = Vec::new();
    let mut paths = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_families = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();
    let mut seen_paths = BTreeSet::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
        let Some((left, value)) = line.split_once('=') else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry must contain exactly one key/value separator",
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
                    "semantic proof family rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(family_id) = left.strip_prefix("proof_family:") {
            if !is_required_family(family_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic proof family identity {family_id}"),
                ));
                continue;
            }
            if !seen_families.insert(family_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("proof_family:{family_id}"),
                    "semantic proof family identity must be unique",
                ));
                continue;
            }
            match parse_family(line_number, family_id, value) {
                Ok(item) => families.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(receipt_id) = left.strip_prefix("receipt:") {
            if !is_required_receipt(receipt_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic receipt identity {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("receipt:{receipt_id}"),
                    "semantic receipt identity must be unique",
                ));
                continue;
            }
            match parse_receipt(line_number, receipt_id, value) {
                Ok(item) => receipts.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(path_id) = left.strip_prefix("path:") {
            if !is_required_path(path_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic proof path identity {path_id}"),
                ));
                continue;
            }
            if !seen_paths.insert(path_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("path:{path_id}"),
                    "semantic proof path identity must be unique",
                ));
                continue;
            }
            match parse_path(line_number, path_id, value) {
                Ok(item) => paths.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
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
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown semantic proof family key {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticProofFamilyTableSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        families,
        receipts,
        paths,
    })
}

pub fn validate_semantic_proof_family_table_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_semantic_proof_family_table_surface(input) {
        Ok(surface) => surface,
        Err(parse_errors) => {
            errors.extend(parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    if parsed.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic proof family table must bind phase P01",
        ));
    }
    if parsed.task != "P01-X02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic proof family table must bind task P01-X02",
        ));
    }
    if parsed.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "P01-X02 must be artifact_emitted",
        ));
    }
    for required in REQUIRED_SEMANTIC_PROOF_FAMILY_RULES {
        if !parsed.rules.contains_key(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{required}"),
                "missing required semantic proof family rule",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_PROOF_FAMILIES {
        if parsed.family_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("proof_family:{required}"),
                "missing required semantic proof family",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_PROOF_RECEIPTS {
        if parsed.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{required}"),
                "missing required semantic receipt binding",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_PROOF_PATHS {
        if parsed.path_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("path:{required}"),
                "missing required semantic proof path",
            ));
        }
    }
    validate_families(&parsed, &mut errors);
    validate_receipts(&parsed, &mut errors);
    validate_paths(&parsed, &mut errors);
    validate_family_coverage(&parsed, &mut errors);
    validate_report(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn parse_family(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticProofFamilyBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let family_kind =
        required_field(&fields, "kind", line_number, ErrorCode::InvalidClosureProof)?.to_string();
    let scope = required_field(
        &fields,
        "scope",
        line_number,
        ErrorCode::InvalidClosureProof,
    )?
    .to_string();
    let receipts = split_list(required_field(
        &fields,
        "receipts",
        line_number,
        ErrorCode::ClosureUnreceipted,
    )?);
    let covers = split_list(required_field(
        &fields,
        "covers",
        line_number,
        ErrorCode::ClosureProofUnbound,
    )?);
    let proofs = split_list(required_field(
        &fields,
        "proofs",
        line_number,
        ErrorCode::MissingClosureProof,
    )?);
    let status = required_field(
        &fields,
        "status",
        line_number,
        ErrorCode::UnsupportedClosureStatus,
    )?
    .to_string();
    if family_kind != id
        || scope != "P01"
        || receipts.is_empty()
        || covers.is_empty()
        || proofs.is_empty()
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("proof_family:{id}"),
            "family must bind kind, scope, receipts, coverage, and proofs",
        ));
    }
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("proof_family:{id}"),
            format!("unsupported semantic proof family status {status}"),
        ));
    }
    Ok(SemanticProofFamilyBinding {
        line_number,
        id: id.to_string(),
        family_kind,
        scope,
        receipts,
        covers,
        proofs,
        status,
    })
}
fn parse_receipt(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticProofReceiptBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let family = required_field(
        &fields,
        "family",
        line_number,
        ErrorCode::InvalidClosureProof,
    )?
    .to_string();
    let path =
        required_field(&fields, "path", line_number, ErrorCode::InvalidClosureProof)?.to_string();
    let covers = split_list(required_field(
        &fields,
        "covers",
        line_number,
        ErrorCode::ClosureProofUnbound,
    )?);
    let verdict = required_field(
        &fields,
        "verdict",
        line_number,
        ErrorCode::InvalidClosureProof,
    )?
    .to_string();
    let status = required_field(
        &fields,
        "status",
        line_number,
        ErrorCode::UnsupportedClosureStatus,
    )?
    .to_string();
    if !is_required_family(&family)
        || !path.starts_with("receipts/p01/")
        || !path.ends_with(".receipt")
        || covers.is_empty()
        || !ALLOWED_VERDICTS.contains(&verdict.as_str())
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("receipt:{id}"),
            "receipt must bind required family, P01 receipt path, coverage, and verdict",
        ));
    }
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("receipt:{id}"),
            format!("unsupported semantic receipt status {status}"),
        ));
    }
    Ok(SemanticProofReceiptBinding {
        line_number,
        id: id.to_string(),
        family,
        path,
        covers,
        verdict,
        status,
    })
}
fn parse_path(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticProofPathBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let family = required_field(
        &fields,
        "family",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?
    .to_string();
    let path_kind = required_field(
        &fields,
        "kind",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?
    .to_string();
    let entry_receipts = split_list(required_field(
        &fields,
        "entry_receipts",
        line_number,
        ErrorCode::MissingClosureProof,
    )?);
    let challenge_receipts = split_list(required_field(
        &fields,
        "challenge_receipts",
        line_number,
        ErrorCode::MissingClosureProof,
    )?);
    let rollback_receipts = split_list(required_field(
        &fields,
        "rollback_receipts",
        line_number,
        ErrorCode::MissingClosureProof,
    )?);
    let status = required_field(
        &fields,
        "status",
        line_number,
        ErrorCode::UnsupportedClosureStatus,
    )?
    .to_string();
    if !is_required_family(&family)
        || family != path_kind
        || entry_receipts.is_empty()
        || challenge_receipts.is_empty()
        || rollback_receipts.is_empty()
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("path:{id}"),
            "path must bind matching family/kind and all receipt lanes",
        ));
    }
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("path:{id}"),
            format!("unsupported semantic path status {status}"),
        ));
    }
    Ok(SemanticProofPathBinding {
        line_number,
        id: id.to_string(),
        family,
        path_kind,
        entry_receipts,
        challenge_receipts,
        rollback_receipts,
        status,
    })
}
fn validate_families(surface: &SemanticProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for family in &surface.families {
        if family.receipts.len() < 3 {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                family.canonical_identity(),
                "each semantic proof family must bind at least three receipts",
            ));
        }
        for receipt in &family.receipts {
            if surface.receipt_by_id(receipt).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    family.canonical_identity(),
                    format!("unknown family receipt {receipt}"),
                ));
            }
        }
        for target in &family.covers {
            if !is_known_coverage_target(target) {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    family.canonical_identity(),
                    format!("unknown family coverage target {target}"),
                ));
            }
        }
    }
}
fn validate_receipts(surface: &SemanticProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        for target in &receipt.covers {
            if !is_known_coverage_target(target) {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    receipt.canonical_identity(),
                    format!("unknown receipt coverage target {target}"),
                ));
            }
        }
    }
}
fn validate_paths(surface: &SemanticProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for path in &surface.paths {
        for receipt in path
            .entry_receipts
            .iter()
            .chain(path.challenge_receipts.iter())
            .chain(path.rollback_receipts.iter())
        {
            if surface.receipt_by_id(receipt).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    path.canonical_identity(),
                    format!("unknown semantic proof path receipt {receipt}"),
                ));
            }
        }
    }
}
fn validate_family_coverage(
    surface: &SemanticProofFamilyTableSurface,
    errors: &mut Vec<ValidationError>,
) {
    let mut happy_coverage = BTreeSet::new();
    if let Some(happy) = surface.family_by_id("happy_path") {
        for target in &happy.covers {
            happy_coverage.insert(target.as_str());
        }
        for receipt in REQUIRED_HAPPY_RECEIPTS {
            if !happy.receipts.iter().any(|item| item == receipt) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingClosureProof,
                    happy.canonical_identity(),
                    format!("happy path missing required semantic receipt {receipt}"),
                ));
            }
        }
    }
    for target in REQUIRED_COVERAGE_TARGETS {
        if !happy_coverage.contains(target) {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                "proof_family:happy_path",
                format!("happy path missing coverage target {target}"),
            ));
        }
    }
}
fn validate_report(surface: &SemanticProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    let families = surface
        .families
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family_kind.clone(),
                item.receipts.clone(),
                item.covers.clone(),
                item.proofs.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let receipts = surface
        .receipts
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family.clone(),
                item.path.clone(),
                item.covers.clone(),
                item.verdict.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let paths = surface
        .paths
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family.clone(),
                item.path_kind.clone(),
                item.entry_receipts.clone(),
                item.challenge_receipts.clone(),
                item.rollback_receipts.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let report = deterministic_semantic_proof_family_table_report(&families, &receipts, &paths);
    if report.family_count != surface.families.len()
        || report.receipt_count != surface.receipts.len()
        || report.path_count != surface.paths.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_proof_family_table_report",
            "semantic proof family report count mismatch",
        ));
    }
    if report.happy_path_receipt_count == 0
        || report.negative_path_receipt_count == 0
        || report.adversarial_path_receipt_count == 0
        || report.rollback_path_receipt_count == 0
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureProof,
            "k0_semantic_proof_family_table_report",
            "all semantic proof families must have receipt rows",
        ));
    }
    if !report.table_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_proof_family_table_report",
            "semantic proof family report hash must be stable fnv1a128",
        ));
    }
}
fn parse_fields(value: &str) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        let Some((key, field_value)) = segment.split_once(':') else {
            return Err("field segment must contain a key/value separator".to_string());
        };
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
        {
            return Err("field segment sides must be non-empty and trimmed".to_string());
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
            return Err(format!("duplicate field {key}"));
        }
    }
    Ok(fields)
}
fn required_field<'a>(
    fields: &'a BTreeMap<String, String>,
    key: &str,
    line_number: usize,
    code: ErrorCode,
) -> Result<&'a str, ValidationError> {
    fields.get(key).map(String::as_str).ok_or_else(|| {
        ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("missing required field {key}"),
        )
    })
}
fn split_list(value: &str) -> Vec<String> {
    if value == "none" {
        Vec::new()
    } else {
        value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToString::to_string)
            .collect()
    }
}
fn is_required_family(value: &str) -> bool {
    REQUIRED_SEMANTIC_PROOF_FAMILIES.contains(&value)
}
fn is_required_receipt(value: &str) -> bool {
    REQUIRED_SEMANTIC_PROOF_RECEIPTS.contains(&value)
}
fn is_required_path(value: &str) -> bool {
    REQUIRED_SEMANTIC_PROOF_PATHS.contains(&value)
}
fn is_known_coverage_target(value: &str) -> bool {
    REQUIRED_COVERAGE_TARGETS.contains(&value)
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}
fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    for (token, code, detail) in [
        (
            "network_required:true",
            ErrorCode::ClosureNetworkDependency,
            "semantic proof family table cannot require network access",
        ),
        (
            "remote_service_required:true",
            ErrorCode::ClosureNetworkDependency,
            "semantic proof family table cannot require remote services",
        ),
        (
            "docs_only:true",
            ErrorCode::ClosureDocsOnly,
            "semantic proof family table cannot be documentation only",
        ),
        (
            "unreceipted:true",
            ErrorCode::ClosureUnreceipted,
            "semantic proof family table cannot accept unreceipted families",
        ),
        (
            "proof_family_drift:true",
            ErrorCode::ClosureDriftAccepted,
            "semantic proof family table cannot accept drift",
        ),
        (
            "receipt_drift:true",
            ErrorCode::ClosureDriftAccepted,
            "semantic proof family table cannot accept receipt drift",
        ),
        (
            "global_closure:true",
            ErrorCode::UnsupportedGlobalClosure,
            "P01-X02 cannot claim global closure",
        ),
        (
            "phase_closure:true",
            ErrorCode::UnsupportedGlobalClosure,
            "P01-X02 cannot close P01 globally",
        ),
    ] {
        if canonical.contains(token) {
            errors.push(ValidationError::reject(code, "forbidden_text", detail));
        }
    }
}
