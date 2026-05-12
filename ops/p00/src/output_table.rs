use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_output_table::deterministic_output_table_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_output_table_model::{
    OutputArtifactBinding, OutputAudienceBinding, OutputContractBinding, OutputReceiptBinding,
    OutputTableSurface, UnresolvedGapBinding,
};

pub const P00_OUTPUT_TABLE_CONTRACT: &str = "LYRA-P00-OUTPUT-TABLE v1";
pub const REQUIRED_OUTPUT_TABLE_RULES: &[&str] = &[
    "output_table_must_cover_required_audiences",
    "developer_outputs_must_bind_commands_and_contracts",
    "operator_outputs_must_bind_receipts_and_controls",
    "product_outputs_must_bind_examples_and_public_refs",
    "enterprise_outputs_must_bind_offline_deployment_evidence",
    "public_interest_outputs_must_bind_access_and_stewardship_evidence",
    "artifact_rows_must_bind_category_kind_path_status",
    "receipt_rows_must_bind_path_target_status",
    "contract_rows_must_bind_surface_path_status",
    "unresolved_gaps_must_bind_next_frontier_or_blocker",
    "no_network_dependency",
    "no_docs_only_output_table",
    "no_unreceipted_output_table",
    "no_global_closure_claim",
];
pub const REQUIRED_OUTPUT_AUDIENCES: &[&str] = &[
    "developer",
    "operator",
    "product",
    "enterprise",
    "public_interest",
];
pub const REQUIRED_OUTPUT_ARTIFACTS: &[&str] = &[
    "developer_contracts_index",
    "developer_cli_matrix",
    "developer_fixture_corpus",
    "developer_golden_index",
    "operator_control_plane",
    "operator_receipt_index",
    "operator_blocker_index",
    "operator_closure_gate",
    "product_public_reference",
    "product_examples",
    "product_packaging_reference",
    "enterprise_offline_deployment",
    "enterprise_release_bundle",
    "enterprise_compliance_hooks",
    "public_interest_access_model",
    "public_interest_stewardship_frame",
    "public_interest_people_first_review",
];
pub const REQUIRED_OUTPUT_RECEIPTS: &[&str] = &[
    "receipt_constitution",
    "receipt_public_interest",
    "receipt_deployment",
    "receipt_closure_gate",
    "receipt_dependency_matrix",
    "receipt_proof_family",
    "receipt_benchmark_pack",
    "receipt_economics",
    "receipt_output_table",
];
pub const REQUIRED_OUTPUT_CONTRACTS: &[&str] = &[
    "contract_constitution",
    "contract_closure_gate",
    "contract_dependency_matrix",
    "contract_proof_family",
    "contract_benchmark_pack",
    "contract_output_table",
];
pub const REQUIRED_UNRESOLVED_GAPS: &[&str] = &["retirement_supersession_law"];
const ALLOWED_ARTIFACT_STATUSES: &[&str] =
    &["artifact_emitted", "execution_proven", "bounded_closed"];
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
const FORBIDDEN_OUTPUT_TABLE_TEXT: &[(&str, ErrorCode)] = &[
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
    ("rule:docs_only_output_table", ErrorCode::ClosureDocsOnly),
    (
        "output table without receipt",
        ErrorCode::ClosureUnreceipted,
    ),
    (
        "unreceipted output table allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    (
        "rule:unreceipted_output_table_allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    ("output drift accepted", ErrorCode::ClosureDriftAccepted),
    ("artifact drift accepted", ErrorCode::ClosureDriftAccepted),
    ("gap drift accepted", ErrorCode::ClosureDriftAccepted),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("rule:global_complete", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_output_table_surface(input: &str) -> Result<OutputTableSurface, Vec<ValidationError>> {
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
            "no output table lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_OUTPUT_TABLE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_OUTPUT_TABLE_CONTRACT}"),
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
                    "output table rule names must be symbolic and unique",
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
                    format!("invalid output audience {audience_id}"),
                ));
                continue;
            }
            if !seen_audiences.insert(audience_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("audience:{audience_id}"),
                    "output audience identity must be unique",
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
                    format!("invalid output artifact {artifact_id}"),
                ));
                continue;
            }
            if !seen_artifacts.insert(artifact_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("artifact:{artifact_id}"),
                    "output artifact identity must be unique",
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
                    format!("invalid output receipt {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("receipt:{receipt_id}"),
                    "output receipt identity must be unique",
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
                    format!("invalid output contract {contract_id}"),
                ));
                continue;
            }
            if !seen_contracts.insert(contract_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("contract:{contract_id}"),
                    "output contract identity must be unique",
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
                    format!("invalid unresolved gap {gap_id}"),
                ));
                continue;
            }
            if !seen_gaps.insert(gap_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("gap:{gap_id}"),
                    "unresolved gap identity must be unique",
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
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown output table key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(OutputTableSurface {
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

pub fn validate_output_table_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_output_table_surface(input) {
        Ok(surface) => errors.extend(validate_output_table_model(&surface).errors),
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

pub fn validate_output_table_model(surface: &OutputTableSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "output table must bind to P00",
        ));
    }
    if surface.task != "P00-X04" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "output table must bind to P00-X04",
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported output table status {}", surface.status),
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
    validate_output_table_report(surface, &mut errors);
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
) -> Result<OutputAudienceBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "output audience fields must be key:value segments",
        )
    })?;
    Ok(OutputAudienceBinding {
        line_number,
        id: id.to_string(),
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        outputs: split_csv(&required_field(
            &fields,
            "outputs",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        artifacts: split_csv(&required_field(
            &fields,
            "artifacts",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
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

fn parse_artifact(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<OutputArtifactBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "output artifact fields must be key:value segments",
        )
    })?;
    Ok(OutputArtifactBinding {
        line_number,
        id: id.to_string(),
        audience: required_field(
            &fields,
            "audience",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        artifact_kind: required_field(
            &fields,
            "kind",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        path: required_field(
            &fields,
            "path",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
    })
}

fn parse_receipt(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<OutputReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "output receipt fields must be key:value segments",
        )
    })?;
    Ok(OutputReceiptBinding {
        line_number,
        id: id.to_string(),
        path: required_field(&fields, "path", ErrorCode::InvalidClosureProof, line_number)?,
        target: required_field(
            &fields,
            "target",
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

fn parse_contract(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<OutputContractBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "output contract fields must be key:value segments",
        )
    })?;
    Ok(OutputContractBinding {
        line_number,
        id: id.to_string(),
        surface: required_field(
            &fields,
            "surface",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
        path: required_field(&fields, "path", ErrorCode::InvalidClosureProof, line_number)?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
    })
}

fn parse_gap(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<UnresolvedGapBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "unresolved gap fields must be key:value segments",
        )
    })?;
    Ok(UnresolvedGapBinding {
        line_number,
        id: id.to_string(),
        blocker: required_field(
            &fields,
            "blocker",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        next_frontier: required_field(
            &fields,
            "next_frontier",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        owner_root: required_field(
            &fields,
            "owner_root",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
    })
}

fn require_rules(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_OUTPUT_TABLE_RULES {
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
                "required output table rule missing",
            )),
        }
    }
}

fn require_audiences(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for audience in REQUIRED_OUTPUT_AUDIENCES {
        if surface.audience_by_id(audience).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("audience:{audience}"),
                "required output audience missing",
            ));
        }
    }
}

fn require_artifacts(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for artifact in REQUIRED_OUTPUT_ARTIFACTS {
        if surface.artifact_by_id(artifact).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("artifact:{artifact}"),
                "required output artifact missing",
            ));
        }
    }
}

fn require_receipts(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for receipt in REQUIRED_OUTPUT_RECEIPTS {
        if surface.receipt_by_id(receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{receipt}"),
                "required output receipt missing",
            ));
        }
    }
}

fn require_contracts(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for contract in REQUIRED_OUTPUT_CONTRACTS {
        if surface.contract_by_id(contract).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("contract:{contract}"),
                "required output contract missing",
            ));
        }
    }
}

fn require_gaps(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for gap in REQUIRED_UNRESOLVED_GAPS {
        if surface.gap_by_id(gap).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("gap:{gap}"),
                "required unresolved gap missing",
            ));
        }
    }
}

fn validate_audiences(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for audience in &surface.audiences {
        if audience.scope != "P00" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                audience.canonical_identity(),
                "output audience scope must be P00",
            ));
        }
        if !ALLOWED_ARTIFACT_STATUSES.contains(&audience.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                audience.canonical_identity(),
                format!("invalid output audience status {}", audience.status),
            ));
        }
        if audience.outputs.is_empty()
            || audience.artifacts.is_empty()
            || audience.receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                audience.canonical_identity(),
                "output audience must bind outputs, artifacts, and receipts",
            ));
        }
        for output in &audience.outputs {
            if !is_symbolic_name(output) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    audience.canonical_identity(),
                    format!("invalid output label {output}"),
                ));
            }
        }
        for artifact in &audience.artifacts {
            if surface.artifact_by_id(artifact).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    audience.canonical_identity(),
                    format!("unknown output artifact {artifact}"),
                ));
            }
        }
        for receipt in &audience.receipts {
            if surface.receipt_by_id(receipt).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    audience.canonical_identity(),
                    format!("unknown output receipt {receipt}"),
                ));
            }
        }
    }
}

fn validate_artifacts(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for artifact in &surface.artifacts {
        if surface.audience_by_id(&artifact.audience).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                artifact.canonical_identity(),
                format!("unknown artifact audience {}", artifact.audience),
            ));
        }
        if !ALLOWED_ARTIFACT_KINDS.contains(&artifact.artifact_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                artifact.canonical_identity(),
                format!("invalid artifact kind {}", artifact.artifact_kind),
            ));
        }
        if !valid_artifact_path(&artifact.path) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                artifact.canonical_identity(),
                format!("invalid output artifact path {}", artifact.path),
            ));
        }
        if !ALLOWED_ARTIFACT_STATUSES.contains(&artifact.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                artifact.canonical_identity(),
                format!("invalid artifact status {}", artifact.status),
            ));
        }
    }
}

fn validate_receipts(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p00/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("receipt path must be a P00 receipt: {}", receipt.path),
            ));
        }
        if surface.artifact_by_id(&receipt.target).is_none()
            && surface.gap_by_id(&receipt.target).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                receipt.canonical_identity(),
                format!("unknown output receipt target {}", receipt.target),
            ));
        }
        if !ALLOWED_ARTIFACT_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("invalid receipt status {}", receipt.status),
            ));
        }
    }
}

fn validate_contracts(_surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for contract in &_surface.contracts {
        if !is_symbolic_name(&contract.surface) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                contract.canonical_identity(),
                format!("invalid contract surface {}", contract.surface),
            ));
        }
        if !contract.path.starts_with("interfaces/p00/contracts/")
            || !contract.path.ends_with(".lyra")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                contract.canonical_identity(),
                format!(
                    "contract path must be a P00 interface contract: {}",
                    contract.path
                ),
            ));
        }
        if !ALLOWED_ARTIFACT_STATUSES.contains(&contract.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                contract.canonical_identity(),
                format!("invalid contract status {}", contract.status),
            ));
        }
    }
}

fn validate_gaps(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
    for gap in &surface.gaps {
        if gap.blocker != "P00-X05" || gap.next_frontier != "P00-X05" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gap.canonical_identity(),
                "P00-X04 may leave only P00-X05 as the next unresolved closure output",
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&gap.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                gap.canonical_identity(),
                format!("invalid gap owner root {}", gap.owner_root),
            ));
        }
        if !ALLOWED_GAP_STATUSES.contains(&gap.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                gap.canonical_identity(),
                format!("invalid gap status {}", gap.status),
            ));
        }
    }
}

fn validate_output_table_report(surface: &OutputTableSurface, errors: &mut Vec<ValidationError>) {
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
    let report = deterministic_output_table_report(
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
            "k0_output_table_report",
            "output table report count mismatch",
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
            "k0_output_table_report",
            "all required output audiences must have artifact rows",
        ));
    }
    if !report.table_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_output_table_report",
            "output table report hash must be stable fnv1a128",
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

fn is_required_audience(value: &str) -> bool {
    REQUIRED_OUTPUT_AUDIENCES.contains(&value)
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn valid_artifact_path(path: &str) -> bool {
    (path.starts_with("interfaces/p00/contracts/") && path.ends_with(".lyra"))
        || (path.starts_with("src/bin/") && path.ends_with(".rs"))
        || (path.starts_with("ops/p00/") && path.ends_with(".lyra"))
        || (path.starts_with("fixtures/p00/") && path.ends_with(".lyra"))
        || (path.starts_with("goldens/p00/") && path.ends_with(".receipt"))
        || (path.starts_with("receipts/p00/") && path.ends_with(".receipt"))
        || (path.starts_with("docs/p00/") && path.ends_with(".lyra"))
        || (path.starts_with("examples/p00/") && path.ends_with(".lyra"))
        || (path.starts_with("products/p00/") && path.ends_with(".lyra"))
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_OUTPUT_TABLE_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden output table token {needle}"),
            ));
        }
    }
}
