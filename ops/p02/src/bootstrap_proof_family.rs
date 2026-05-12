use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_proof_family::deterministic_bootstrap_proof_family_table_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_proof_family::{
    bootstrap_proof_family_artifacts_bind_paths, bootstrap_proof_family_families_bind_receipts,
    bootstrap_proof_family_no_forbidden_descriptor_claims,
    bootstrap_proof_family_paths_bind_receipts, bootstrap_proof_family_receipts_bind_families,
    bootstrap_proof_family_receipts_cover_p02_001_through_p02_x01,
    bootstrap_proof_family_registry_hash,
};
use crate::p02_bootstrap_proof_family_model::{
    BootstrapProofFamilyBinding, BootstrapProofFamilyTableSurface, BootstrapProofPathBinding,
    BootstrapProofReceiptBinding,
};

pub const P02_BOOTSTRAP_PROOF_FAMILY_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-PROOF-FAMILY-TABLE v1";
pub const REQUIRED_BOOTSTRAP_PROOF_FAMILY_RULES: &[&str] = &[
    "bootstrap_proof_family_table_must_cover_required_families",
    "happy_path_receipts_must_cover_primary_and_dependency_success",
    "negative_path_receipts_must_bind_rejection_corpus",
    "adversarial_path_receipts_must_bind_hostile_and_capture_cases",
    "rollback_path_receipts_must_bind_seed_and_host_extinction",
    "dependency_path_receipts_must_bind_x01_matrix",
    "proof_paths_must_bind_entry_challenge_and_rollback",
    "p02_x02_must_not_close_global_phase",
    "no_network_dependency",
    "no_docs_only_proof_table",
    "no_unreceipted_proof_family",
];
pub const REQUIRED_BOOTSTRAP_PROOF_FAMILIES: &[&str] = &[
    "happy_path",
    "negative_path",
    "adversarial_path",
    "rollback_path",
    "dependency_path",
];
pub const REQUIRED_BOOTSTRAP_PROOF_PATHS: &[&str] = &[
    "bootstrap_happy_primary_chain",
    "bootstrap_negative_rejection_chain",
    "bootstrap_adversarial_host_chain",
    "bootstrap_rollback_replay_chain",
    "bootstrap_dependency_matrix_chain",
];
pub const REQUIRED_BOOTSTRAP_PROOF_RECEIPTS: &[&str] = &[
    "receipt_happy_p02_001",
    "receipt_happy_p02_002",
    "receipt_happy_p02_003",
    "receipt_happy_p02_004",
    "receipt_happy_p02_005",
    "receipt_happy_p02_006",
    "receipt_happy_p02_007",
    "receipt_happy_p02_008",
    "receipt_happy_p02_009",
    "receipt_happy_p02_010",
    "receipt_happy_p02_011",
    "receipt_happy_p02_012",
    "receipt_happy_p02_013",
    "receipt_happy_p02_014",
    "receipt_happy_p02_015",
    "receipt_happy_p02_016",
    "receipt_happy_p02_017",
    "receipt_happy_p02_018",
    "receipt_happy_p02_019",
    "receipt_happy_p02_020",
    "receipt_happy_p02_021",
    "receipt_happy_p02_022",
    "receipt_happy_p02_023",
    "receipt_happy_p02_024",
    "receipt_happy_p02_x01",
    "receipt_negative_truth_cleanup_rejection",
    "receipt_negative_host_boundary_rejection",
    "receipt_negative_falsification_rejection",
    "receipt_negative_redteam_rejection",
    "receipt_negative_dependency_matrix_rejection",
    "receipt_adversarial_host_boundary_challenge",
    "receipt_adversarial_emergency_fallback_challenge",
    "receipt_adversarial_foreign_surface_challenge",
    "receipt_adversarial_economics_capture_challenge",
    "receipt_adversarial_redteam_attack_challenge",
    "receipt_rollback_extinction_rollback",
    "receipt_rollback_seed_contract_rollback",
    "receipt_rollback_foreign_surface_rollback",
    "receipt_rollback_replay_rollback",
    "receipt_rollback_redteam_rollback",
    "receipt_rollback_closure_gate_rollback",
    "receipt_rollback_dependency_matrix_rollback",
    "receipt_dependency_evidence_emission",
    "receipt_dependency_replay_bridge",
    "receipt_dependency_closure_gate",
    "receipt_dependency_dependency_matrix",
    "receipt_dependency_packaging_deployment",
    "receipt_dependency_economics_redteam",
];
pub const REQUIRED_BOOTSTRAP_PROOF_COVERAGE_TARGETS: &[&str] = &[
    "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
    "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
    "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
    "P02-X01",
];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "bounded_closed"];
const ALLOWED_VERDICTS: &[&str] = &["accepted", "rejected_expected"];

pub fn parse_bootstrap_proof_family_surface(
    input: &str,
) -> Result<BootstrapProofFamilyTableSurface, Vec<ValidationError>> {
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
            "bootstrap proof-family table surface is empty",
        )]);
    }
    let mut errors = Vec::new();
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_PROOF_FAMILY_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_PROOF_FAMILY_CONTRACT}"),
        ));
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut closure_scope = None;
    let mut global_closure = None;
    let mut next_frontier = None;
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
                    "bootstrap proof-family rule names must be symbolic and unique",
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
                    format!("invalid bootstrap proof family identity {family_id}"),
                ));
                continue;
            }
            if !seen_families.insert(family_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("proof_family:{family_id}"),
                    "bootstrap proof family identity must be unique",
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
                    format!("invalid bootstrap proof receipt identity {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("receipt:{receipt_id}"),
                    "bootstrap proof receipt identity must be unique",
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
                    format!("invalid bootstrap proof path identity {path_id}"),
                ));
                continue;
            }
            if !seen_paths.insert(path_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("path:{path_id}"),
                    "bootstrap proof path identity must be unique",
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
            "closure_scope" => closure_scope = Some(value.to_string()),
            "global_closure" => global_closure = Some(value.to_string()),
            "next_frontier" => next_frontier = Some(value.to_string()),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown bootstrap proof-family key {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(BootstrapProofFamilyTableSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        closure_scope: closure_scope.unwrap_or_default(),
        global_closure: global_closure.unwrap_or_default(),
        next_frontier: next_frontier.unwrap_or_default(),
        rules,
        families,
        receipts,
        paths,
    })
}

pub fn validate_bootstrap_proof_family_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_bootstrap_proof_family_surface(input) {
        Ok(surface) => surface,
        Err(parse_errors) => {
            errors.extend(parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    errors.extend(validate_bootstrap_proof_family_model(&parsed).errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_proof_family_model(
    surface: &BootstrapProofFamilyTableSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "bootstrap proof-family table must bind phase P02",
        ));
    }
    if surface.task != "P02-X02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "bootstrap proof-family table must bind task P02-X02",
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "P02-X02 must be artifact_emitted",
        ));
    }
    if surface.closure_scope != "extended_open" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "closure_scope",
            "P02-X02 must remain extended_open",
        ));
    }
    if surface.global_closure != "denied" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "global_closure",
            "P02-X02 cannot claim global closure",
        ));
    }
    if surface.next_frontier != "P02-X03" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "next_frontier",
            "P02-X02 must set P02-X03 as next frontier",
        ));
    }
    for required in REQUIRED_BOOTSTRAP_PROOF_FAMILY_RULES {
        match surface.rule_value(required) {
            Some("required") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{required}"),
                format!("unsupported rule value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{required}"),
                "missing required bootstrap proof-family rule",
            )),
        }
    }
    for required in REQUIRED_BOOTSTRAP_PROOF_FAMILIES {
        if surface.family_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("proof_family:{required}"),
                "missing required bootstrap proof family",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_PROOF_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{required}"),
                "missing required bootstrap proof receipt",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_PROOF_PATHS {
        if surface.path_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("path:{required}"),
                "missing required bootstrap proof path",
            ));
        }
    }
    validate_families(surface, &mut errors);
    validate_receipts(surface, &mut errors);
    validate_paths(surface, &mut errors);
    validate_family_coverage(surface, &mut errors);
    validate_report(surface, &mut errors);
    validate_descriptor_registry(&mut errors);
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
) -> Result<BootstrapProofFamilyBinding, ValidationError> {
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
        || scope != "P02"
        || receipts.is_empty()
        || covers.is_empty()
        || proofs.is_empty()
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("proof_family:{id}"),
            "family must bind kind, P02 scope, receipts, coverage, and proofs",
        ));
    }
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("proof_family:{id}"),
            format!("unsupported bootstrap proof family status {status}"),
        ));
    }
    Ok(BootstrapProofFamilyBinding {
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
) -> Result<BootstrapProofReceiptBinding, ValidationError> {
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
        || !path.starts_with("receipts/p02/")
        || !path.ends_with(".receipt")
        || covers.is_empty()
        || !ALLOWED_VERDICTS.contains(&verdict.as_str())
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("receipt:{id}"),
            "receipt must bind required family, P02 receipt path, coverage, and verdict",
        ));
    }
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("receipt:{id}"),
            format!("unsupported bootstrap receipt status {status}"),
        ));
    }
    Ok(BootstrapProofReceiptBinding {
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
) -> Result<BootstrapProofPathBinding, ValidationError> {
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
            format!("unsupported bootstrap path status {status}"),
        ));
    }
    Ok(BootstrapProofPathBinding {
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

fn validate_families(
    surface: &BootstrapProofFamilyTableSurface,
    errors: &mut Vec<ValidationError>,
) {
    for family in &surface.families {
        if family.receipts.len() < 3 {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                family.canonical_identity(),
                "each bootstrap proof family must bind at least three receipts",
            ));
        }
        for receipt_id in &family.receipts {
            match surface.receipt_by_id(receipt_id) {
                Some(receipt) if receipt.family == family.id => {}
                Some(_) => errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    family.canonical_identity(),
                    format!("family receipt {receipt_id} binds the wrong family"),
                )),
                None => errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    family.canonical_identity(),
                    format!("unknown family receipt {receipt_id}"),
                )),
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

fn validate_receipts(
    surface: &BootstrapProofFamilyTableSurface,
    errors: &mut Vec<ValidationError>,
) {
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

fn validate_paths(surface: &BootstrapProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
    for path in &surface.paths {
        for receipt_id in path
            .entry_receipts
            .iter()
            .chain(path.challenge_receipts.iter())
            .chain(path.rollback_receipts.iter())
        {
            match surface.receipt_by_id(receipt_id) {
                Some(receipt) if receipt.family == path.family => {}
                Some(_) => errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    path.canonical_identity(),
                    format!("path receipt {receipt_id} binds the wrong family"),
                )),
                None => errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    path.canonical_identity(),
                    format!("unknown bootstrap proof path receipt {receipt_id}"),
                )),
            }
        }
    }
}

fn validate_family_coverage(
    surface: &BootstrapProofFamilyTableSurface,
    errors: &mut Vec<ValidationError>,
) {
    let mut all_coverage = BTreeSet::new();
    if let Some(happy) = surface.family_by_id("happy_path") {
        for target in &happy.covers {
            all_coverage.insert(target.as_str());
        }
    }
    for target in REQUIRED_BOOTSTRAP_PROOF_COVERAGE_TARGETS {
        if !all_coverage.contains(target) {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                "proof_family:happy_path",
                format!("happy path missing coverage target {target}"),
            ));
        }
    }
}

fn validate_report(surface: &BootstrapProofFamilyTableSurface, errors: &mut Vec<ValidationError>) {
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
    let report = deterministic_bootstrap_proof_family_table_report(&families, &receipts, &paths);
    if report.family_count != surface.families.len()
        || report.receipt_count != surface.receipts.len()
        || report.path_count != surface.paths.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_bootstrap_proof_family_table_report",
            "bootstrap proof-family report count mismatch",
        ));
    }
    if report.happy_path_receipt_count == 0
        || report.negative_path_receipt_count == 0
        || report.adversarial_path_receipt_count == 0
        || report.rollback_path_receipt_count == 0
        || report.dependency_path_receipt_count == 0
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureProof,
            "k0_bootstrap_proof_family_table_report",
            "all bootstrap proof families must have receipt rows",
        ));
    }
    if !report.table_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_bootstrap_proof_family_table_report",
            "bootstrap proof-family report hash must be stable fnv1a128",
        ));
    }
}

fn validate_descriptor_registry(errors: &mut Vec<ValidationError>) {
    if !bootstrap_proof_family_artifacts_bind_paths() {
        errors.push(ValidationError::reject(
            ErrorCode::UnknownEvidencePath,
            "lyralang_bootstrap_proof_family",
            "artifact descriptor paths must be bound",
        ));
    }
    if !bootstrap_proof_family_families_bind_receipts() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureProofUnbound,
            "lyralang_bootstrap_proof_family",
            "family descriptors must bind receipts",
        ));
    }
    if !bootstrap_proof_family_receipts_bind_families() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureProofUnbound,
            "lyralang_bootstrap_proof_family",
            "receipt descriptors must bind families",
        ));
    }
    if !bootstrap_proof_family_paths_bind_receipts() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureProofUnbound,
            "lyralang_bootstrap_proof_family",
            "path descriptors must bind receipts",
        ));
    }
    if !bootstrap_proof_family_receipts_cover_p02_001_through_p02_x01() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureProof,
            "lyralang_bootstrap_proof_family",
            "descriptors must cover P02-001 through P02-X01",
        ));
    }
    if !bootstrap_proof_family_no_forbidden_descriptor_claims() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "lyralang_bootstrap_proof_family",
            "descriptor registry contains forbidden closure claim",
        ));
    }
    if !bootstrap_proof_family_registry_hash().starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "lyralang_bootstrap_proof_family",
            "registry hash must be stable fnv1a128",
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
    REQUIRED_BOOTSTRAP_PROOF_FAMILIES.contains(&value)
}
fn is_required_receipt(value: &str) -> bool {
    REQUIRED_BOOTSTRAP_PROOF_RECEIPTS.contains(&value)
}
fn is_required_path(value: &str) -> bool {
    REQUIRED_BOOTSTRAP_PROOF_PATHS.contains(&value)
}
fn is_known_coverage_target(value: &str) -> bool {
    REQUIRED_BOOTSTRAP_PROOF_COVERAGE_TARGETS.contains(&value)
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
            "bootstrap proof-family table cannot require network access",
        ),
        (
            "remote_service_required:true",
            ErrorCode::ClosureNetworkDependency,
            "bootstrap proof-family table cannot require remote services",
        ),
        (
            "docs_only:true",
            ErrorCode::ClosureDocsOnly,
            "bootstrap proof-family table cannot be documentation only",
        ),
        (
            "unreceipted:true",
            ErrorCode::ClosureUnreceipted,
            "bootstrap proof-family table cannot accept unreceipted families",
        ),
        (
            "proof_family_drift:true",
            ErrorCode::ClosureDriftAccepted,
            "bootstrap proof-family table cannot accept proof-family drift",
        ),
        (
            "receipt_drift:true",
            ErrorCode::ClosureDriftAccepted,
            "bootstrap proof-family table cannot accept receipt drift",
        ),
        (
            "global_closure:true",
            ErrorCode::UnsupportedGlobalClosure,
            "P02-X02 cannot claim global closure",
        ),
        (
            "phase_closure:true",
            ErrorCode::UnsupportedGlobalClosure,
            "P02-X02 cannot close P02 globally",
        ),
    ] {
        if canonical.contains(token) {
            errors.push(ValidationError::reject(code, "forbidden_text", detail));
        }
    }
}
