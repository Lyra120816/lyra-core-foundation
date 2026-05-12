use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_output_table::deterministic_semantic_output_table_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_output_table_model::{
    SemanticOutputArtifactBinding, SemanticOutputAudienceBinding, SemanticOutputContractBinding,
    SemanticOutputGapBinding, SemanticOutputReceiptBinding, SemanticOutputTableSurface,
};

pub const P01_SEMANTIC_OUTPUT_TABLE_CONTRACT: &str = "LYRA-P01-SEMANTIC-OUTPUT-TABLE v1";
pub const REQUIRED_SEMANTIC_OUTPUT_TABLE_RULES: &[&str] = &[
    "semantic_output_table_must_cover_required_audiences",
    "developer_outputs_must_bind_commands_contracts_fixtures_and_goldens",
    "operator_outputs_must_bind_controls_receipts_blockers_and_closure",
    "product_outputs_must_bind_reference_slice_examples_and_packaging",
    "enterprise_outputs_must_bind_offline_deployment_release_and_compliance",
    "public_interest_outputs_must_bind_access_stewardship_and_people_review",
    "artifact_rows_must_bind_audience_kind_path_status",
    "receipt_rows_must_bind_path_target_status",
    "contract_rows_must_bind_surface_path_status",
    "unresolved_gaps_must_bind_next_frontier_or_blocker",
    "no_network_dependency",
    "no_docs_only_output_table",
    "no_unreceipted_output_table",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_OUTPUT_AUDIENCES: &[&str] = &[
    "developer",
    "operator",
    "product",
    "enterprise",
    "public_interest",
];
pub const REQUIRED_SEMANTIC_OUTPUT_ARTIFACTS: &[&str] = &[
    "developer_semantic_contract_index",
    "developer_semantic_cli_surface",
    "developer_semantic_fixture_corpus",
    "developer_semantic_golden_index",
    "operator_semantic_control_plane",
    "operator_semantic_receipt_index",
    "operator_semantic_blocker_index",
    "operator_semantic_closure_gate",
    "product_semantic_reference_slice",
    "product_semantic_examples",
    "product_semantic_packaging_surface",
    "enterprise_semantic_offline_deployment",
    "enterprise_semantic_release_bundle",
    "enterprise_semantic_compliance_hooks",
    "public_interest_semantic_access_model",
    "public_interest_semantic_stewardship_frame",
    "public_interest_semantic_people_first_review",
];
pub const REQUIRED_SEMANTIC_OUTPUT_RECEIPTS: &[&str] = &[
    "receipt_semantic_atoms",
    "receipt_semantic_deployment",
    "receipt_semantic_closure_gate",
    "receipt_semantic_dependency_matrix",
    "receipt_semantic_proof_family",
    "receipt_semantic_benchmark_pack",
    "receipt_semantic_economics",
    "receipt_semantic_output_table",
];
pub const REQUIRED_SEMANTIC_OUTPUT_CONTRACTS: &[&str] = &[
    "contract_semantic_atoms",
    "contract_semantic_core_ir",
    "contract_semantic_interface",
    "contract_semantic_closure",
    "contract_semantic_dependency_matrix",
    "contract_semantic_proof_family",
    "contract_semantic_benchmark_pack",
    "contract_semantic_output_table",
];
pub const REQUIRED_SEMANTIC_UNRESOLVED_GAPS: &[&str] = &["semantic_retirement_supersession_law"];
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
const FORBIDDEN_SEMANTIC_OUTPUT_TEXT: &[(&str, ErrorCode)] = &[
    ("network_required:true", ErrorCode::ClosureNetworkDependency),
    (
        "remote_service_required:true",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("network required", ErrorCode::ClosureNetworkDependency),
    ("cloud required", ErrorCode::ClosureNetworkDependency),
    ("online required", ErrorCode::ClosureNetworkDependency),
    (
        "remote service required",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("remote fetch", ErrorCode::ClosureNetworkDependency),
    ("docs_only:true", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("unreceipted:true", ErrorCode::ClosureUnreceipted),
    (
        "output table without receipt",
        ErrorCode::ClosureUnreceipted,
    ),
    (
        "unreceipted output table allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    ("output drift accepted", ErrorCode::ClosureDriftAccepted),
    ("artifact drift accepted", ErrorCode::ClosureDriftAccepted),
    ("gap drift accepted", ErrorCode::ClosureDriftAccepted),
    ("global_closure:true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_closure:true", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_semantic_output_table_surface(
    input: &str,
) -> Result<SemanticOutputTableSurface, Vec<ValidationError>> {
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
            "semantic output table surface is empty",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_OUTPUT_TABLE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_OUTPUT_TABLE_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
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
                    "semantic output table rule names must be symbolic and unique",
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
                    format!("invalid semantic output audience {audience_id}"),
                ));
                continue;
            }
            if !seen_audiences.insert(audience_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("audience:{audience_id}"),
                    "semantic output audience identity must be unique",
                ));
                continue;
            }
            match parse_audience(line_number, audience_id, value) {
                Ok(item) => audiences.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(artifact_id) = left.strip_prefix("artifact:") {
            if !is_symbolic_name(artifact_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic output artifact {artifact_id}"),
                ));
                continue;
            }
            if !seen_artifacts.insert(artifact_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("artifact:{artifact_id}"),
                    "semantic output artifact identity must be unique",
                ));
                continue;
            }
            match parse_artifact(line_number, artifact_id, value) {
                Ok(item) => artifacts.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(receipt_id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(receipt_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic output receipt {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("receipt:{receipt_id}"),
                    "semantic output receipt identity must be unique",
                ));
                continue;
            }
            match parse_receipt(line_number, receipt_id, value) {
                Ok(item) => receipts.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(contract_id) = left.strip_prefix("contract:") {
            if !is_symbolic_name(contract_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic output contract {contract_id}"),
                ));
                continue;
            }
            if !seen_contracts.insert(contract_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("contract:{contract_id}"),
                    "semantic output contract identity must be unique",
                ));
                continue;
            }
            match parse_contract(line_number, contract_id, value) {
                Ok(item) => contracts.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(gap_id) = left.strip_prefix("gap:") {
            if !is_symbolic_name(gap_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic output gap {gap_id}"),
                ));
                continue;
            }
            if !seen_gaps.insert(gap_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("gap:{gap_id}"),
                    "semantic output gap identity must be unique",
                ));
                continue;
            }
            match parse_gap(line_number, gap_id, value) {
                Ok(item) => gaps.push(item),
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
                format!("unknown semantic output table key {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticOutputTableSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        audiences,
        artifacts,
        receipts,
        contracts,
        gaps,
    })
}

pub fn validate_semantic_output_table_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_semantic_output_table_surface(input) {
        Ok(surface) => errors.extend(validate_semantic_output_table_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_semantic_output_table_model(surface: &SemanticOutputTableSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic output table must bind phase P01",
        ));
    }
    if surface.task != "P01-X04" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic output table must bind task P01-X04",
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!(
                "unsupported semantic output table status {}",
                surface.status
            ),
        ));
    }
    require_rules(surface, &mut errors);
    require_audiences(surface, &mut errors);
    require_artifacts(surface, &mut errors);
    require_receipts(surface, &mut errors);
    require_contracts(surface, &mut errors);
    require_gaps(surface, &mut errors);
    validate_audiences(surface, &mut errors);
    validate_artifacts(surface, &mut errors);
    validate_receipts(surface, &mut errors);
    validate_contracts(surface, &mut errors);
    validate_gaps(surface, &mut errors);
    validate_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_audience(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticOutputAudienceBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(SemanticOutputAudienceBinding {
        line_number,
        id: id.to_string(),
        scope: required_field(
            &fields,
            "scope",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
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
) -> Result<SemanticOutputArtifactBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(SemanticOutputArtifactBinding {
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
) -> Result<SemanticOutputReceiptBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(SemanticOutputReceiptBinding {
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
) -> Result<SemanticOutputContractBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(SemanticOutputContractBinding {
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
) -> Result<SemanticOutputGapBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(SemanticOutputGapBinding {
        line_number,
        id: id.to_string(),
        blocker: required_field(
            &fields,
            "blocker",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        next_frontier: required_field(
            &fields,
            "next_frontier",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        owner_root: required_field(
            &fields,
            "owner_root",
            line_number,
            ErrorCode::InvalidOwnerRoot,
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

fn require_rules(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_SEMANTIC_OUTPUT_TABLE_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                format!("unsupported semantic output table rule value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "missing required semantic output table rule",
            )),
        }
    }
}
fn require_audiences(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for audience in REQUIRED_SEMANTIC_OUTPUT_AUDIENCES {
        if surface.audience_by_id(audience).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("audience:{audience}"),
                "required semantic output audience missing",
            ));
        }
    }
}
fn require_artifacts(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for artifact in REQUIRED_SEMANTIC_OUTPUT_ARTIFACTS {
        if surface.artifact_by_id(artifact).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("artifact:{artifact}"),
                "required semantic output artifact missing",
            ));
        }
    }
}
fn require_receipts(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for receipt in REQUIRED_SEMANTIC_OUTPUT_RECEIPTS {
        if surface.receipt_by_id(receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{receipt}"),
                "required semantic output receipt missing",
            ));
        }
    }
}
fn require_contracts(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for contract in REQUIRED_SEMANTIC_OUTPUT_CONTRACTS {
        if surface.contract_by_id(contract).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("contract:{contract}"),
                "required semantic output contract missing",
            ));
        }
    }
}
fn require_gaps(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for gap in REQUIRED_SEMANTIC_UNRESOLVED_GAPS {
        if surface.gap_by_id(gap).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("gap:{gap}"),
                "required semantic output unresolved gap missing",
            ));
        }
    }
}

fn validate_audiences(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for audience in &surface.audiences {
        if audience.scope != "P01" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                audience.canonical_identity(),
                "semantic output audience scope must be P01",
            ));
        }
        if !ALLOWED_STATUSES.contains(&audience.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                audience.canonical_identity(),
                format!(
                    "unsupported semantic output audience status {}",
                    audience.status
                ),
            ));
        }
        if audience.outputs.is_empty()
            || audience.artifacts.is_empty()
            || audience.receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                audience.canonical_identity(),
                "semantic output audience must bind outputs, artifacts, and receipts",
            ));
        }
        for output in &audience.outputs {
            if !is_symbolic_name(output) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    audience.canonical_identity(),
                    format!("invalid semantic output label {output}"),
                ));
            }
        }
        for artifact in &audience.artifacts {
            if surface.artifact_by_id(artifact).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    audience.canonical_identity(),
                    format!("unknown semantic output artifact {artifact}"),
                ));
            }
        }
        for receipt in &audience.receipts {
            if surface.receipt_by_id(receipt).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    audience.canonical_identity(),
                    format!("unknown semantic output receipt {receipt}"),
                ));
            }
        }
    }
}

fn validate_artifacts(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for artifact in &surface.artifacts {
        if surface.audience_by_id(&artifact.audience).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                artifact.canonical_identity(),
                format!(
                    "unknown semantic output artifact audience {}",
                    artifact.audience
                ),
            ));
        }
        if !ALLOWED_ARTIFACT_KINDS.contains(&artifact.artifact_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                artifact.canonical_identity(),
                format!(
                    "invalid semantic output artifact kind {}",
                    artifact.artifact_kind
                ),
            ));
        }
        if !valid_artifact_path(&artifact.path) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                artifact.canonical_identity(),
                format!("invalid semantic output artifact path {}", artifact.path),
            ));
        }
        if !ALLOWED_STATUSES.contains(&artifact.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                artifact.canonical_identity(),
                format!(
                    "unsupported semantic output artifact status {}",
                    artifact.status
                ),
            ));
        }
    }
}

fn validate_receipts(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p01/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("receipt path must be a P01 receipt: {}", receipt.path),
            ));
        }
        if surface.artifact_by_id(&receipt.target).is_none()
            && surface.gap_by_id(&receipt.target).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                receipt.canonical_identity(),
                format!("unknown semantic output receipt target {}", receipt.target),
            ));
        }
        if !ALLOWED_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!(
                    "unsupported semantic output receipt status {}",
                    receipt.status
                ),
            ));
        }
    }
}

fn validate_contracts(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for contract in &surface.contracts {
        if !is_symbolic_name(&contract.surface) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                contract.canonical_identity(),
                format!(
                    "invalid semantic output contract surface {}",
                    contract.surface
                ),
            ));
        }
        if !contract.path.starts_with("interfaces/p01/contracts/")
            || !contract.path.ends_with(".lyra")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                contract.canonical_identity(),
                format!(
                    "contract path must be a P01 interface contract: {}",
                    contract.path
                ),
            ));
        }
        if !ALLOWED_STATUSES.contains(&contract.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                contract.canonical_identity(),
                format!(
                    "unsupported semantic output contract status {}",
                    contract.status
                ),
            ));
        }
    }
}

fn validate_gaps(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    for gap in &surface.gaps {
        if gap.blocker != "P01-X05" || gap.next_frontier != "P01-X05" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gap.canonical_identity(),
                "P01-X04 may leave only P01-X05 as the next unresolved closure output",
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&gap.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                gap.canonical_identity(),
                format!("invalid semantic output gap owner root {}", gap.owner_root),
            ));
        }
        if !ALLOWED_GAP_STATUSES.contains(&gap.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                gap.canonical_identity(),
                format!("unsupported semantic output gap status {}", gap.status),
            ));
        }
    }
}

fn validate_report(surface: &SemanticOutputTableSurface, errors: &mut Vec<ValidationError>) {
    let audience_inputs: Vec<(String, Vec<String>, Vec<String>, Vec<String>, String)> = surface
        .audiences
        .iter()
        .map(|audience| {
            (
                audience.id.clone(),
                audience.outputs.clone(),
                audience.artifacts.clone(),
                audience.receipts.clone(),
                audience.status.clone(),
            )
        })
        .collect();
    let artifact_inputs: Vec<(String, String, String, String, String)> = surface
        .artifacts
        .iter()
        .map(|artifact| {
            (
                artifact.id.clone(),
                artifact.audience.clone(),
                artifact.artifact_kind.clone(),
                artifact.path.clone(),
                artifact.status.clone(),
            )
        })
        .collect();
    let receipt_inputs: Vec<(String, String, String, String)> = surface
        .receipts
        .iter()
        .map(|receipt| {
            (
                receipt.id.clone(),
                receipt.path.clone(),
                receipt.target.clone(),
                receipt.status.clone(),
            )
        })
        .collect();
    let contract_inputs: Vec<(String, String, String, String)> = surface
        .contracts
        .iter()
        .map(|contract| {
            (
                contract.id.clone(),
                contract.surface.clone(),
                contract.path.clone(),
                contract.status.clone(),
            )
        })
        .collect();
    let gap_inputs: Vec<(String, String, String, String, String)> = surface
        .gaps
        .iter()
        .map(|gap| {
            (
                gap.id.clone(),
                gap.blocker.clone(),
                gap.next_frontier.clone(),
                gap.owner_root.clone(),
                gap.status.clone(),
            )
        })
        .collect();
    let report = deterministic_semantic_output_table_report(
        &audience_inputs,
        &artifact_inputs,
        &receipt_inputs,
        &contract_inputs,
        &gap_inputs,
    );
    if report.audience_count != surface.audiences.len()
        || report.artifact_count != surface.artifacts.len()
        || report.receipt_count != surface.receipts.len()
        || report.contract_count != surface.contracts.len()
        || report.unresolved_gap_count != surface.gaps.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_output_table_report",
            "semantic output table report count mismatch",
        ));
    }
    if report.developer_artifact_count == 0
        || report.operator_artifact_count == 0
        || report.product_artifact_count == 0
        || report.enterprise_artifact_count == 0
        || report.public_interest_artifact_count == 0
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "k0_semantic_output_table_report",
            "all semantic output audiences must have artifact rows",
        ));
    }
    if !report.table_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_output_table_report",
            "semantic output table report hash must be stable fnv1a128",
        ));
    }
}

fn parse_fields(value: &str) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        let Some((key, val)) = segment.split_once(':') else {
            return Err(format!("field segment lacks colon: {segment}"));
        };
        if key.is_empty() || val.is_empty() || key != key.trim() || val != val.trim() {
            return Err(format!("field segment is not canonical: {segment}"));
        }
        if fields.insert(key.to_string(), val.to_string()).is_some() {
            return Err(format!("duplicate field key: {key}"));
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
    fields
        .get(name)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ValidationError::reject(
                code,
                format!("line:{line_number:03}"),
                format!("missing field {name}"),
            )
        })
}

fn split_list(value: &str) -> Vec<String> {
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
fn is_required_audience(value: &str) -> bool {
    REQUIRED_SEMANTIC_OUTPUT_AUDIENCES.contains(&value)
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn valid_artifact_path(path: &str) -> bool {
    (path.starts_with("interfaces/p01/contracts/") && path.ends_with(".lyra"))
        || (path.starts_with("src/bin/") && path.ends_with(".rs"))
        || (path.starts_with("ops/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("fixtures/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("goldens/p01/") && path.ends_with(".receipt"))
        || (path.starts_with("receipts/p01/") && path.ends_with(".receipt"))
        || (path.starts_with("docs/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("examples/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("products/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("lyralang/") && path.ends_with(".rs"))
        || (path.starts_with("k0/") && path.ends_with(".rs"))
        || (path.starts_with("interfaces/p01/src/") && path.ends_with(".rs"))
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_OUTPUT_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic output table token {needle}"),
            ));
        }
    }
}
