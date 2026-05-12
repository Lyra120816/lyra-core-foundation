use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_output_table::deterministic_bootstrap_output_table_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_output_table::{
    bootstrap_output_artifacts_bind_paths, bootstrap_output_audiences_bind_registry,
    bootstrap_output_carrier_signature, bootstrap_output_contracts_bind_paths,
    bootstrap_output_gaps_bind_next_frontier, bootstrap_output_no_forbidden_descriptor_claims,
    bootstrap_output_receipts_bind_paths, bootstrap_output_registry_hash,
};
use crate::p02_bootstrap_output_table_model::{
    BootstrapOutputArtifactBinding, BootstrapOutputAudienceBinding, BootstrapOutputContractBinding,
    BootstrapOutputGapBinding, BootstrapOutputReceiptBinding, BootstrapOutputTableSurface,
};

pub const P02_BOOTSTRAP_OUTPUT_TABLE_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-OUTPUT-TABLE v1";
pub const REQUIRED_BOOTSTRAP_OUTPUT_TABLE_RULES: &[&str] = &[
    "bootstrap_output_table_must_cover_required_audiences",
    "developer_outputs_must_bind_contracts_cli_fixtures_and_goldens",
    "operator_outputs_must_bind_controls_receipts_blockers_and_closure_gate",
    "product_outputs_must_bind_reference_examples_and_packaging",
    "enterprise_outputs_must_bind_offline_deployment_release_and_compliance",
    "public_interest_outputs_must_bind_access_stewardship_people_review_and_economics",
    "artifact_rows_must_bind_audience_kind_owner_path_status",
    "receipt_rows_must_bind_path_target_status",
    "contract_rows_must_bind_surface_path_status",
    "unresolved_gaps_must_bind_next_frontier_or_blocker",
    "p02_x04_must_keep_p02_x05_open",
    "no_network_dependency",
    "no_docs_only_output_table",
    "no_unreceipted_output_table",
    "no_global_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_OUTPUT_AUDIENCES: &[&str] = &[
    "developer",
    "operator",
    "product",
    "enterprise",
    "public_interest",
];
pub const REQUIRED_BOOTSTRAP_OUTPUT_ARTIFACTS: &[&str] = &[
    "developer_bootstrap_contract_index",
    "developer_bootstrap_cli_matrix",
    "developer_bootstrap_fixture_corpus",
    "developer_bootstrap_golden_index",
    "operator_bootstrap_control_plane",
    "operator_bootstrap_receipt_index",
    "operator_bootstrap_blocker_index",
    "operator_bootstrap_closure_gate",
    "product_bootstrap_reference_surface",
    "product_bootstrap_examples",
    "product_bootstrap_packaging_surface",
    "enterprise_bootstrap_offline_deployment",
    "enterprise_bootstrap_release_bundle",
    "enterprise_bootstrap_compliance_hooks",
    "public_interest_bootstrap_access_model",
    "public_interest_bootstrap_stewardship_frame",
    "public_interest_bootstrap_people_first_review",
    "public_interest_bootstrap_economics_surface",
];
pub const REQUIRED_BOOTSTRAP_OUTPUT_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_inventory",
    "receipt_bootstrap_extinction",
    "receipt_seed_runtime_contracts",
    "receipt_seed_runtime_replacement",
    "receipt_bootstrap_packaging",
    "receipt_bootstrap_deployment",
    "receipt_bootstrap_economics",
    "receipt_bootstrap_redteam",
    "receipt_bootstrap_closure",
    "receipt_bootstrap_dependency_matrix",
    "receipt_bootstrap_proof_family",
    "receipt_bootstrap_benchmark_pack",
    "receipt_bootstrap_output_table",
];
pub const REQUIRED_BOOTSTRAP_OUTPUT_CONTRACTS: &[&str] = &[
    "contract_bootstrap_inventory",
    "contract_bootstrap_extinction",
    "contract_seed_runtime_contracts",
    "contract_bootstrap_deployment",
    "contract_bootstrap_economics",
    "contract_bootstrap_closure",
    "contract_bootstrap_dependency_matrix",
    "contract_bootstrap_proof_family",
    "contract_bootstrap_benchmark_pack",
    "contract_bootstrap_output_table",
];
pub const REQUIRED_BOOTSTRAP_OUTPUT_GAPS: &[&str] = &["p02_x05_retirement_supersession_law"];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "bounded_closed"];
const ALLOWED_GAP_STATUSES: &[&str] = &["open"];
const ALLOWED_OWNER_ROOTS: &[&str] = &[
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
    "docs",
    "examples",
];
const ALLOWED_ARTIFACT_KINDS: &[&str] = &[
    "artifact",
    "cli",
    "closure",
    "contract",
    "control",
    "doc",
    "example",
    "fixture",
    "golden",
    "product_ref",
    "receipt",
    "source",
];
const FORBIDDEN_BOOTSTRAP_OUTPUT_TEXT: &[(&str, ErrorCode)] = &[
    ("network_required:true", ErrorCode::ClosureNetworkDependency),
    (
        "remote_service_required:true",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("network required", ErrorCode::ClosureNetworkDependency),
    ("cloud required", ErrorCode::ClosureNetworkDependency),
    ("online required", ErrorCode::ClosureNetworkDependency),
    ("docs_only:true", ErrorCode::ClosureDocsOnly),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("unreceipted:true", ErrorCode::ClosureUnreceipted),
    (
        "output table without receipt",
        ErrorCode::ClosureUnreceipted,
    ),
    ("output drift accepted", ErrorCode::ClosureDriftAccepted),
    ("artifact drift accepted", ErrorCode::ClosureDriftAccepted),
    ("gap drift accepted", ErrorCode::ClosureDriftAccepted),
    ("global_closure:true", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_bootstrap_output_table_surface(
    input: &str,
) -> Result<BootstrapOutputTableSurface, Vec<ValidationError>> {
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
            "bootstrap output table surface is empty",
        )]);
    }
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_OUTPUT_TABLE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_OUTPUT_TABLE_CONTRACT}"),
        )]);
    }
    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut closure_scope = None;
    let mut global_closure = None;
    let mut next_frontier = None;
    let mut rules = BTreeMap::new();
    let mut audiences = Vec::new();
    let mut artifacts = Vec::new();
    let mut receipts = Vec::new();
    let mut contracts = Vec::new();
    let mut gaps = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_audiences = BTreeSet::new();
    let mut seen_artifacts = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();
    let mut seen_contracts = BTreeSet::new();
    let mut seen_gaps = BTreeSet::new();

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
                    "bootstrap output table rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(audience_id) = left.strip_prefix("audience:") {
            if !is_required_audience(audience_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid bootstrap output audience {audience_id}"),
                ));
                continue;
            }
            if !seen_audiences.insert(audience_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("audience:{audience_id}"),
                    "bootstrap output audience identity must be unique",
                ));
                continue;
            }
            match parse_audience(line_number, audience_id, value) {
                Ok(binding) => audiences.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(artifact_id) = left.strip_prefix("artifact:") {
            if !is_symbolic_name(artifact_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid bootstrap output artifact {artifact_id}"),
                ));
                continue;
            }
            if !seen_artifacts.insert(artifact_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("artifact:{artifact_id}"),
                    "bootstrap output artifact identity must be unique",
                ));
                continue;
            }
            match parse_artifact(line_number, artifact_id, value) {
                Ok(binding) => artifacts.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(receipt_id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(receipt_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid bootstrap output receipt {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("receipt:{receipt_id}"),
                    "bootstrap output receipt identity must be unique",
                ));
                continue;
            }
            match parse_receipt(line_number, receipt_id, value) {
                Ok(binding) => receipts.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(contract_id) = left.strip_prefix("contract:") {
            if !is_symbolic_name(contract_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid bootstrap output contract {contract_id}"),
                ));
                continue;
            }
            if !seen_contracts.insert(contract_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("contract:{contract_id}"),
                    "bootstrap output contract identity must be unique",
                ));
                continue;
            }
            match parse_contract(line_number, contract_id, value) {
                Ok(binding) => contracts.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(gap_id) = left.strip_prefix("gap:") {
            if !is_symbolic_name(gap_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid bootstrap output gap {gap_id}"),
                ));
                continue;
            }
            if !seen_gaps.insert(gap_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("gap:{gap_id}"),
                    "bootstrap output gap identity must be unique",
                ));
                continue;
            }
            match parse_gap(line_number, gap_id, value) {
                Ok(binding) => gaps.push(binding),
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
                format!("unknown bootstrap output field {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(BootstrapOutputTableSurface {
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
        closure_scope: closure_scope.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingClosureRule,
                "closure_scope",
                "missing closure scope",
            )]
        })?,
        global_closure: global_closure.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                "global_closure",
                "missing global closure flag",
            )]
        })?,
        next_frontier: next_frontier.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                "next_frontier",
                "missing next frontier",
            )]
        })?,
        rules,
        audiences,
        artifacts,
        receipts,
        contracts,
        gaps,
    })
}

pub fn validate_bootstrap_output_table_surface(input: &str) -> (Verdict, Receipt) {
    let mut errors = Vec::new();
    scan_forbidden(input, &mut errors);
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    match parse_bootstrap_output_table_surface(input) {
        Ok(surface) => validate_bootstrap_output_table_model(&surface, &mut errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }
    errors.sort_by_key(|left| left.canonical_line());
    errors.dedup();
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_output_table_model(
    surface: &BootstrapOutputTableSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-X04" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-X04 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported output table status {}", surface.status),
        ));
    }
    if surface.closure_scope != "extended_open" {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureOutputPremature,
            "closure_scope",
            "P02-X04 must remain extended_open",
        ));
    }
    if surface.global_closure != "denied" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "global_closure",
            "P02-X04 must deny global closure",
        ));
    }
    if surface.next_frontier != "P02-X05" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "next_frontier",
            "P02-X04 must hand off to P02-X05",
        ));
    }
    for required in REQUIRED_BOOTSTRAP_OUTPUT_TABLE_RULES {
        if surface.rule_value(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{required}"),
                "missing required bootstrap output table rule",
            ));
        }
    }
    for audience in REQUIRED_BOOTSTRAP_OUTPUT_AUDIENCES {
        if surface.audience_by_id(audience).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("audience:{audience}"),
                "missing required bootstrap output audience",
            ));
        }
    }
    for artifact in REQUIRED_BOOTSTRAP_OUTPUT_ARTIFACTS {
        if surface.artifact_by_id(artifact).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("artifact:{artifact}"),
                "missing required bootstrap output artifact",
            ));
        }
    }
    for receipt in REQUIRED_BOOTSTRAP_OUTPUT_RECEIPTS {
        if surface.receipt_by_id(receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{receipt}"),
                "missing required bootstrap output receipt",
            ));
        }
    }
    for contract in REQUIRED_BOOTSTRAP_OUTPUT_CONTRACTS {
        if surface.contract_by_id(contract).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("contract:{contract}"),
                "missing required bootstrap output contract",
            ));
        }
    }
    for gap in REQUIRED_BOOTSTRAP_OUTPUT_GAPS {
        if surface.gap_by_id(gap).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("gap:{gap}"),
                "missing required bootstrap output gap",
            ));
        }
    }
    validate_audiences(surface, errors);
    validate_artifacts(surface, errors);
    validate_receipts(surface, errors);
    validate_contracts(surface, errors);
    validate_gaps(surface, errors);
    validate_descriptor_registry(errors);
    validate_report(surface, errors);
}

fn validate_audiences(surface: &BootstrapOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for audience in &surface.audiences {
        if audience.outputs.is_empty()
            || audience.artifacts.is_empty()
            || audience.receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                audience.canonical_identity(),
                "audience must bind outputs, artifacts, and receipts",
            ));
        }
        if !ALLOWED_STATUSES.contains(&audience.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                audience.canonical_identity(),
                format!("unsupported audience status {}", audience.status),
            ));
        }
        for artifact in &audience.artifacts {
            match surface.artifact_by_id(artifact) {
                Some(row) if row.audience == audience.id => {}
                Some(_) => errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    audience.canonical_identity(),
                    format!("artifact {artifact} belongs to a different audience"),
                )),
                None => errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    audience.canonical_identity(),
                    format!("unknown audience artifact {artifact}"),
                )),
            }
        }
        for receipt in &audience.receipts {
            if surface.receipt_by_id(receipt).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    audience.canonical_identity(),
                    format!("unknown audience receipt {receipt}"),
                ));
            }
        }
    }
}

fn validate_artifacts(surface: &BootstrapOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for artifact in &surface.artifacts {
        if surface.audience_by_id(&artifact.audience).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                artifact.canonical_identity(),
                format!("artifact references unknown audience {}", artifact.audience),
            ));
        }
        if !ALLOWED_ARTIFACT_KINDS.contains(&artifact.artifact_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                artifact.canonical_identity(),
                format!("unsupported artifact kind {}", artifact.artifact_kind),
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&artifact.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                artifact.canonical_identity(),
                format!("unsupported owner root {}", artifact.owner_root),
            ));
        }
        if !valid_artifact_path(&artifact.path) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                artifact.canonical_identity(),
                format!("invalid artifact path {}", artifact.path),
            ));
        }
        if !ALLOWED_STATUSES.contains(&artifact.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                artifact.canonical_identity(),
                format!("unsupported artifact status {}", artifact.status),
            ));
        }
    }
}

fn validate_receipts(surface: &BootstrapOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p02/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("invalid receipt path {}", receipt.path),
            ));
        }
        if !(receipt.target.starts_with("P02-") || receipt.target.starts_with("P02-X")) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("invalid receipt target {}", receipt.target),
            ));
        }
        if !ALLOWED_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!("unsupported receipt status {}", receipt.status),
            ));
        }
    }
}

fn validate_contracts(_surface: &BootstrapOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for contract in &_surface.contracts {
        if contract.surface.is_empty()
            || !contract.path.starts_with("interfaces/p02/contracts/")
            || !contract.path.ends_with(".lyra")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                contract.canonical_identity(),
                format!("invalid contract path {}", contract.path),
            ));
        }
        if !ALLOWED_STATUSES.contains(&contract.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                contract.canonical_identity(),
                format!("unsupported contract status {}", contract.status),
            ));
        }
    }
}

fn validate_gaps(_surface: &BootstrapOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for gap in &_surface.gaps {
        if gap.next_frontier != "P02-X05" {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureOutputPremature,
                gap.canonical_identity(),
                "bootstrap output gap must bind P02-X05 as next frontier",
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&gap.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                gap.canonical_identity(),
                format!("unsupported gap owner root {}", gap.owner_root),
            ));
        }
        if !ALLOWED_GAP_STATUSES.contains(&gap.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                gap.canonical_identity(),
                format!("unsupported gap status {}", gap.status),
            ));
        }
    }
}

fn validate_descriptor_registry(errors: &mut Vec<ValidationError>) {
    if !bootstrap_output_no_forbidden_descriptor_claims() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "bootstrap_output_registry",
            "descriptor carrier must not claim global closure",
        ));
    }
    if !bootstrap_output_artifacts_bind_paths() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "bootstrap_output_registry",
            "artifact descriptors must bind P02 paths",
        ));
    }
    if !bootstrap_output_receipts_bind_paths() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            "bootstrap_output_registry",
            "receipt descriptors must bind P02 receipt paths",
        ));
    }
    if !bootstrap_output_contracts_bind_paths() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            "bootstrap_output_registry",
            "contract descriptors must bind P02 contract paths",
        ));
    }
    if !bootstrap_output_audiences_bind_registry() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureProofUnbound,
            "bootstrap_output_registry",
            "audience descriptors must bind artifact and receipt registries",
        ));
    }
    if !bootstrap_output_gaps_bind_next_frontier() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureOutputPremature,
            "bootstrap_output_registry",
            "gap descriptors must bind P02-X05",
        ));
    }
    if !bootstrap_output_registry_hash().starts_with("fnv1a128:")
        || !bootstrap_output_carrier_signature().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEvidenceBinding,
            "bootstrap_output_registry",
            "registry hashes must use stable fnv1a128 labels",
        ));
    }
}

fn validate_report(surface: &BootstrapOutputTableSurface, errors: &mut Vec<ValidationError>) {
    let audiences = surface
        .audiences
        .iter()
        .map(|a| {
            (
                a.id.clone(),
                a.outputs.clone(),
                a.artifacts.clone(),
                a.receipts.clone(),
                a.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let artifacts = surface
        .artifacts
        .iter()
        .map(|a| {
            (
                a.id.clone(),
                a.audience.clone(),
                a.artifact_kind.clone(),
                a.owner_root.clone(),
                a.path.clone(),
                a.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let receipts = surface
        .receipts
        .iter()
        .map(|r| {
            (
                r.id.clone(),
                r.path.clone(),
                r.target.clone(),
                r.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let contracts = surface
        .contracts
        .iter()
        .map(|c| {
            (
                c.id.clone(),
                c.surface.clone(),
                c.path.clone(),
                c.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let gaps = surface
        .gaps
        .iter()
        .map(|g| {
            (
                g.id.clone(),
                g.blocker.clone(),
                g.next_frontier.clone(),
                g.owner_root.clone(),
                g.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let report = deterministic_bootstrap_output_table_report(
        &audiences, &artifacts, &receipts, &contracts, &gaps,
    );
    if report.audience_count < REQUIRED_BOOTSTRAP_OUTPUT_AUDIENCES.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "output_report",
            "missing output audiences",
        ));
    }
    if report.artifact_count < REQUIRED_BOOTSTRAP_OUTPUT_ARTIFACTS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "output_report",
            "missing output artifacts",
        ));
    }
    if report.receipt_count < REQUIRED_BOOTSTRAP_OUTPUT_RECEIPTS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureProof,
            "output_report",
            "missing output receipts",
        ));
    }
    if report.contract_count < REQUIRED_BOOTSTRAP_OUTPUT_CONTRACTS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureProof,
            "output_report",
            "missing output contracts",
        ));
    }
    if report.unresolved_gap_count < REQUIRED_BOOTSTRAP_OUTPUT_GAPS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "output_report",
            "missing unresolved gap rows",
        ));
    }
    if report.developer_artifact_count < 4
        || report.operator_artifact_count < 4
        || report.product_artifact_count < 3
        || report.enterprise_artifact_count < 3
        || report.public_interest_artifact_count < 4
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "output_report",
            "each required audience must carry its output rows",
        ));
    }
    if !report.table_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEvidenceBinding,
            "output_report",
            "bootstrap output table report hash must be stable fnv1a128",
        ));
    }
}

fn parse_audience(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapOutputAudienceBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|msg| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            msg,
        )
    })?;
    Ok(BootstrapOutputAudienceBinding {
        line_number,
        id: id.to_string(),
        outputs: split_list(required_field(
            &fields,
            "outputs",
            line_number,
            ErrorCode::MissingClosureOutputGate,
        )?),
        artifacts: split_list(required_field(
            &fields,
            "artifacts",
            line_number,
            ErrorCode::MissingClosureOutputGate,
        )?),
        receipts: split_list(required_field(
            &fields,
            "receipts",
            line_number,
            ErrorCode::MissingClosureProof,
        )?),
        status: required_field(
            &fields,
            "status",
            line_number,
            ErrorCode::UnsupportedClosureStatus,
        )?
        .to_string(),
    })
}
fn parse_artifact(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapOutputArtifactBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|msg| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            msg,
        )
    })?;
    Ok(BootstrapOutputArtifactBinding {
        line_number,
        id: id.to_string(),
        audience: required_field(
            &fields,
            "audience",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        artifact_kind: required_field(
            &fields,
            "kind",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        owner_root: required_field(&fields, "owner", line_number, ErrorCode::InvalidOwnerRoot)?
            .to_string(),
        path: required_field(
            &fields,
            "path",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        status: required_field(
            &fields,
            "status",
            line_number,
            ErrorCode::UnsupportedClosureStatus,
        )?
        .to_string(),
    })
}
fn parse_receipt(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapOutputReceiptBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|msg| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            msg,
        )
    })?;
    Ok(BootstrapOutputReceiptBinding {
        line_number,
        id: id.to_string(),
        path: required_field(&fields, "path", line_number, ErrorCode::InvalidClosureProof)?
            .to_string(),
        target: required_field(
            &fields,
            "target",
            line_number,
            ErrorCode::InvalidClosureProof,
        )?
        .to_string(),
        status: required_field(
            &fields,
            "status",
            line_number,
            ErrorCode::UnsupportedClosureStatus,
        )?
        .to_string(),
    })
}
fn parse_contract(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapOutputContractBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|msg| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            msg,
        )
    })?;
    Ok(BootstrapOutputContractBinding {
        line_number,
        id: id.to_string(),
        surface: required_field(
            &fields,
            "surface",
            line_number,
            ErrorCode::InvalidClosureProof,
        )?
        .to_string(),
        path: required_field(&fields, "path", line_number, ErrorCode::InvalidClosureProof)?
            .to_string(),
        status: required_field(
            &fields,
            "status",
            line_number,
            ErrorCode::UnsupportedClosureStatus,
        )?
        .to_string(),
    })
}
fn parse_gap(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapOutputGapBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|msg| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            msg,
        )
    })?;
    Ok(BootstrapOutputGapBinding {
        line_number,
        id: id.to_string(),
        blocker: required_field(
            &fields,
            "blocker",
            line_number,
            ErrorCode::MissingBlockerBinding,
        )?
        .to_string(),
        next_frontier: required_field(
            &fields,
            "next",
            line_number,
            ErrorCode::MissingClosureOutputGate,
        )?
        .to_string(),
        owner_root: required_field(&fields, "owner", line_number, ErrorCode::InvalidOwnerRoot)?
            .to_string(),
        status: required_field(
            &fields,
            "status",
            line_number,
            ErrorCode::UnsupportedClosureStatus,
        )?
        .to_string(),
    })
}

fn scan_forbidden(input: &str, errors: &mut Vec<ValidationError>) {
    let lower = input.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_BOOTSTRAP_OUTPUT_TEXT {
        if lower.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "bootstrap_output_forbidden_text",
                format!("forbidden token {token}"),
            ));
        }
    }
}
fn parse_fields(value: &str) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        let (key, val) = segment
            .split_once(':')
            .ok_or_else(|| format!("field segment missing ':' -> {segment}"))?;
        if key.is_empty() || val.is_empty() {
            return Err(format!("empty field in segment {segment}"));
        }
        if fields.insert(key.to_string(), val.to_string()).is_some() {
            return Err(format!("duplicate field {key}"));
        }
    }
    Ok(fields)
}
fn required_field<'a>(
    fields: &'a BTreeMap<String, String>,
    name: &str,
    line_number: usize,
    code: ErrorCode,
) -> Result<&'a str, ValidationError> {
    fields.get(name).map(String::as_str).ok_or_else(|| {
        ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("missing field {name}"),
        )
    })
}
fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}
fn is_required_audience(value: &str) -> bool {
    REQUIRED_BOOTSTRAP_OUTPUT_AUDIENCES.contains(&value)
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
}
fn valid_artifact_path(path: &str) -> bool {
    (path.starts_with("interfaces/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("fixtures/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("goldens/p02/") && path.ends_with(".receipt"))
        || (path.starts_with("ops/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("products/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("shells/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("docs/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("examples/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("receipts/p02/") && path.ends_with(".receipt"))
}
