use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_proof_family::deterministic_proof_family_table_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_proof_family_model::{
    ProofFamilyBinding, ProofFamilyTableSurface, ProofPathBinding, ProofReceiptBinding,
};

pub const P00_PROOF_FAMILY_TABLE_CONTRACT: &str = "LYRA-P00-PROOF-FAMILY-TABLE v1";
pub const REQUIRED_PROOF_FAMILY_RULES: &[&str] = &[
    "proof_family_table_must_cover_required_families",
    "happy_path_receipts_must_bind_primary_and_closure_success",
    "negative_path_receipts_must_bind_rejection_corpus",
    "adversarial_path_receipts_must_bind_challenge_and_redteam",
    "rollback_path_receipts_must_bind_replay_and_rollback",
    "no_network_dependency",
    "no_docs_only_proof_table",
    "no_unreceipted_proof_family",
    "no_global_closure_claim",
];
pub const REQUIRED_PROOF_FAMILIES: &[&str] = &[
    "happy_path",
    "negative_path",
    "adversarial_path",
    "rollback_path",
];
pub const REQUIRED_PROOF_PATHS: &[&str] = &[
    "happy_primary_chain",
    "negative_rejection_chain",
    "adversarial_challenge_chain",
    "rollback_replay_chain",
];
pub const REQUIRED_HAPPY_RECEIPTS: &[&str] = &[
    "receipt_constitution",
    "receipt_authority",
    "receipt_identity",
    "receipt_enforcement",
    "receipt_delivery",
    "receipt_control",
    "receipt_owner_root",
    "receipt_benchmark_evidence",
    "receipt_public_interest",
    "receipt_canon_compliance",
    "receipt_formal_semantics",
    "receipt_canonical_model",
    "receipt_engine",
    "receipt_interface",
    "receipt_packaging",
    "receipt_deployment",
    "receipt_ecosystem",
    "receipt_economics",
    "receipt_dependency_matrix",
];
pub const REQUIRED_PROOF_RECEIPTS: &[&str] = &[
    "receipt_constitution",
    "receipt_authority",
    "receipt_identity",
    "receipt_enforcement",
    "receipt_delivery",
    "receipt_challenge",
    "receipt_control",
    "receipt_owner_root",
    "receipt_benchmark_evidence",
    "receipt_public_interest",
    "receipt_canon_compliance",
    "receipt_acceptance",
    "receipt_formal_semantics",
    "receipt_canonical_model",
    "receipt_engine",
    "receipt_falsification",
    "receipt_replay",
    "receipt_interface",
    "receipt_packaging",
    "receipt_deployment",
    "receipt_ecosystem",
    "receipt_economics",
    "receipt_redteam",
    "receipt_closure_gate",
    "receipt_dependency_matrix",
];
const REQUIRED_COVERAGE_TARGETS: &[&str] = &[
    "P00-001", "P00-002", "P00-003", "P00-004", "P00-005", "P00-006", "P00-007", "P00-008",
    "P00-009", "P00-010", "P00-011", "P00-012", "P00-013", "P00-014", "P00-015", "P00-016",
    "P00-017", "P00-018", "P00-019", "P00-020", "P00-021", "P00-022", "P00-023", "P00-024",
    "P00-X01",
];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "bounded_closed"];
const ALLOWED_VERDICTS: &[&str] = &["accepted", "rejected_expected"];
const FORBIDDEN_PROOF_FAMILY_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::ClosureNetworkDependency),
    ("rule:network_required", ErrorCode::ClosureNetworkDependency),
    ("cloud required", ErrorCode::ClosureNetworkDependency),
    ("online required", ErrorCode::ClosureNetworkDependency),
    (
        "remote service required",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("remote fetch", ErrorCode::ClosureNetworkDependency),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("rule:docs_only_proof_table", ErrorCode::ClosureDocsOnly),
    (
        "proof family without receipt",
        ErrorCode::ClosureUnreceipted,
    ),
    (
        "unreceipted proof family allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    (
        "rule:unreceipted_proof_family_allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    (
        "proof family drift accepted",
        ErrorCode::ClosureDriftAccepted,
    ),
    ("receipt drift accepted", ErrorCode::ClosureDriftAccepted),
    ("path drift accepted", ErrorCode::ClosureDriftAccepted),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("rule:global_complete", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_proof_family_table_surface(
    input: &str,
) -> Result<ProofFamilyTableSurface, Vec<ValidationError>> {
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
            "no proof family table lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_PROOF_FAMILY_TABLE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_PROOF_FAMILY_TABLE_CONTRACT}"),
        )]);
    }
    let mut errors = Vec::new();
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
                    "proof family rule names must be symbolic and unique",
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
                    format!("invalid proof family identity {family_id}"),
                ));
                continue;
            }
            if !seen_families.insert(family_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("proof_family:{family_id}"),
                    "proof family identity must be unique",
                ));
                continue;
            }
            match parse_family(line_number, family_id, value) {
                Ok(family) => families.push(family),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(receipt_id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(receipt_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid receipt identity {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("receipt:{receipt_id}"),
                    "receipt identity must be unique",
                ));
                continue;
            }
            match parse_receipt(line_number, receipt_id, value) {
                Ok(receipt) => receipts.push(receipt),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(path_id) = left.strip_prefix("path:") {
            if !is_symbolic_name(path_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid proof path identity {path_id}"),
                ));
                continue;
            }
            if !seen_paths.insert(path_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("path:{path_id}"),
                    "proof path identity must be unique",
                ));
                continue;
            }
            match parse_path(line_number, path_id, value) {
                Ok(path) => paths.push(path),
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
                format!("unknown proof family key {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(ProofFamilyTableSurface {
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

pub fn validate_proof_family_table_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_proof_family_table_surface(input) {
        Ok(surface) => errors.extend(validate_proof_family_table_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_receipt(input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_proof_family_table_model(surface: &ProofFamilyTableSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "proof family table must bind to P00",
        ));
    }
    if surface.task != "P00-X02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "proof family table must bind to P00-X02",
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported proof family table status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_families(surface, &mut errors);
    require_receipts(surface, &mut errors);
    require_paths(surface, &mut errors);
    validate_families(surface, &mut errors);
    validate_receipts(surface, &mut errors);
    validate_paths(surface, &mut errors);
    validate_family_coverage(surface, &mut errors);
    validate_proof_family_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_family(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ProofFamilyBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "proof family fields must be key:value segments",
        )
    })?;
    Ok(ProofFamilyBinding {
        line_number,
        id: id.to_string(),
        family_kind: required_field(&fields, "kind", ErrorCode::InvalidClosureProof, line_number)?,
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        covers: split_csv(&required_field(
            &fields,
            "covers",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        proofs: split_csv(&required_field(
            &fields,
            "proofs",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
    })
}
fn parse_receipt(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ProofReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "receipt fields must be key:value segments",
        )
    })?;
    Ok(ProofReceiptBinding {
        line_number,
        id: id.to_string(),
        family: required_field(
            &fields,
            "family",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
        path: required_field(&fields, "path", ErrorCode::InvalidClosureProof, line_number)?,
        covers: split_csv(&required_field(
            &fields,
            "covers",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        verdict: required_field(
            &fields,
            "verdict",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
    })
}
fn parse_path(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ProofPathBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "proof path fields must be key:value segments",
        )
    })?;
    Ok(ProofPathBinding {
        line_number,
        id: id.to_string(),
        family: required_field(
            &fields,
            "family",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        path_kind: required_field(
            &fields,
            "kind",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        entry_receipts: split_csv(&required_field(
            &fields,
            "entry_receipts",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        challenge_receipts: split_csv(&required_field(
            &fields,
            "challenge_receipts",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        rollback_receipts: split_csv(&required_field(
            &fields,
            "rollback_receipts",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
    })
}

fn require_rules(surface: &ProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_PROOF_FAMILY_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "required proof family rule missing",
            )),
        }
    }
}
fn require_families(surface: &ProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for family in REQUIRED_PROOF_FAMILIES {
        if surface.family_by_id(family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("proof_family:{family}"),
                "required proof family missing",
            ));
        }
    }
}
fn require_receipts(surface: &ProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for receipt in REQUIRED_PROOF_RECEIPTS {
        if surface.receipt_by_id(receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{receipt}"),
                "required proof receipt missing",
            ));
        }
    }
}
fn require_paths(surface: &ProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for path in REQUIRED_PROOF_PATHS {
        if surface.path_by_id(path).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("path:{path}"),
                "required proof path missing",
            ));
        }
    }
}

fn validate_families(surface: &ProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for family in &surface.families {
        if family.family_kind != family.id || !is_required_family(&family.family_kind) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                family.canonical_identity(),
                format!("invalid proof family kind {}", family.family_kind),
            ));
        }
        if family.scope != "P00" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                family.canonical_identity(),
                "proof family scope must be P00",
            ));
        }
        if !ALLOWED_STATUSES.contains(&family.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                family.canonical_identity(),
                format!("invalid proof family status {}", family.status),
            ));
        }
        if family.receipts.is_empty() || family.covers.is_empty() || family.proofs.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                family.canonical_identity(),
                "proof families must bind receipts, coverage, and proof labels",
            ));
        }
        for receipt in &family.receipts {
            if surface.receipt_by_id(receipt).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    family.canonical_identity(),
                    format!("unknown proof family receipt {receipt}"),
                ));
            }
        }
        for target in &family.covers {
            if !is_known_coverage_target(target) {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    family.canonical_identity(),
                    format!("unknown proof family coverage target {target}"),
                ));
            }
        }
    }
}
fn validate_receipts(surface: &ProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !is_required_family(&receipt.family) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("unknown receipt family {}", receipt.family),
            ));
        }
        if !receipt.path.starts_with("receipts/p00/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!(
                    "receipt path must be a P00 receipt artifact: {}",
                    receipt.path
                ),
            ));
        }
        if receipt.covers.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                "receipt must cover at least one target",
            ));
        }
        for target in &receipt.covers {
            if !is_known_coverage_target(target) {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    receipt.canonical_identity(),
                    format!("unknown receipt coverage target {target}"),
                ));
            }
        }
        if !ALLOWED_VERDICTS.contains(&receipt.verdict.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("invalid receipt verdict {}", receipt.verdict),
            ));
        }
        if !ALLOWED_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("invalid receipt status {}", receipt.status),
            ));
        }
    }
}
fn validate_paths(surface: &ProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for path in &surface.paths {
        if !is_required_family(&path.family) || path.path_kind != path.family {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                path.canonical_identity(),
                format!(
                    "proof path family/kind mismatch {} {}",
                    path.family, path.path_kind
                ),
            ));
        }
        if !ALLOWED_STATUSES.contains(&path.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                path.canonical_identity(),
                format!("invalid proof path status {}", path.status),
            ));
        }
        if path.entry_receipts.is_empty()
            || path.challenge_receipts.is_empty()
            || path.rollback_receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                path.canonical_identity(),
                "proof paths must bind entry, challenge, and rollback receipts",
            ));
        }
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
                    format!("unknown proof path receipt {receipt}"),
                ));
            }
        }
    }
}
fn validate_family_coverage(surface: &ProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
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
                    format!("happy path missing required receipt {receipt}"),
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
    for family in REQUIRED_PROOF_FAMILIES {
        if let Some(binding) = surface.family_by_id(family) {
            if binding.receipts.len() < 3 {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingClosureProof,
                    binding.canonical_identity(),
                    "each proof family must bind at least three receipts",
                ));
            }
        }
    }
}
fn validate_proof_family_report(
    surface: &ProofFamilyTableSurface,
    errors: &mut Vec<ValidationError>,
) {
    let family_inputs: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .families
        .iter()
        .map(|family| {
            (
                family.id.clone(),
                family.family_kind.clone(),
                family.receipts.clone(),
                family.covers.clone(),
                family.proofs.clone(),
                family.status.clone(),
            )
        })
        .collect();
    let receipt_inputs: Vec<(String, String, String, Vec<String>, String, String)> = surface
        .receipts
        .iter()
        .map(|receipt| {
            (
                receipt.id.clone(),
                receipt.family.clone(),
                receipt.path.clone(),
                receipt.covers.clone(),
                receipt.verdict.clone(),
                receipt.status.clone(),
            )
        })
        .collect();
    let path_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .paths
        .iter()
        .map(|path| {
            (
                path.id.clone(),
                path.family.clone(),
                path.path_kind.clone(),
                path.entry_receipts.clone(),
                path.challenge_receipts.clone(),
                path.rollback_receipts.clone(),
                path.status.clone(),
            )
        })
        .collect();
    let report =
        deterministic_proof_family_table_report(&family_inputs, &receipt_inputs, &path_inputs);
    if report.family_count != surface.families.len()
        || report.receipt_count != surface.receipts.len()
        || report.path_count != surface.paths.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_proof_family_table_report",
            "proof family report count mismatch",
        ));
    }
    if report.happy_path_receipt_count == 0
        || report.negative_path_receipt_count == 0
        || report.adversarial_path_receipt_count == 0
        || report.rollback_path_receipt_count == 0
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureProof,
            "k0_proof_family_table_report",
            "all proof families must have receipt rows",
        ));
    }
    if !report.table_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_proof_family_table_report",
            "proof family report hash must be stable fnv1a128",
        ));
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
fn split_csv(value: &str) -> Vec<String> {
    if value == "none" {
        Vec::new()
    } else {
        value
            .split(',')
            .filter(|item| !item.is_empty())
            .map(str::to_string)
            .collect()
    }
}
fn is_required_family(value: &str) -> bool {
    REQUIRED_PROOF_FAMILIES.contains(&value)
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
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_PROOF_FAMILY_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden proof family token {needle}"),
            ));
        }
    }
}
