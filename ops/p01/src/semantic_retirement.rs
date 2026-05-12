use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_retirement::deterministic_semantic_retirement_supersession_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_retirement_model::{
    SemanticRetirementGateBinding, SemanticRetirementReceiptBinding,
    SemanticRetirementSupersessionSurface, SemanticRetirementSurfaceBinding,
    SemanticSupersessionBinding,
};

pub const P01_SEMANTIC_RETIREMENT_SUPERSESSION_CONTRACT: &str =
    "LYRA-P01-SEMANTIC-RETIREMENT-SUPERSESSION v1";
pub const REQUIRED_SEMANTIC_RETIREMENT_RULES: &[&str] = &[
    "semantic_retirement_law_must_cover_transitional_surfaces",
    "every_semantic_surface_must_bind_owner_root",
    "every_semantic_surface_must_bind_replacement_or_retention",
    "every_semantic_surface_must_bind_deletion_gate",
    "every_semantic_surface_must_bind_supersession_rule",
    "every_semantic_surface_must_bind_receipt",
    "bootstrap_surfaces_must_have_migration_gate",
    "retained_surfaces_must_have_retention_reason",
    "historical_surfaces_must_archive_as_superseded",
    "semantic_ir_surfaces_must_bind_native_successor",
    "no_network_dependency",
    "no_docs_only_semantic_retirement",
    "no_unreceipted_semantic_retirement",
    "no_drift_accepted",
    "no_ambient_time_gate",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_RETIREMENT_SURFACES: &[&str] = &[
    "rust_semantic_bootstrap_crate",
    "p01_semantic_cli_checks",
    "p01_semantic_text_contracts",
    "p01_semantic_control_plane",
    "p01_semantic_fixture_corpus",
    "p01_semantic_golden_receipts",
    "p01_semantic_docs_examples",
    "p01_semantic_product_surfaces",
    "p01_semantic_receipt_format",
    "p01_semantic_hash_canonicalization",
    "p01_semantic_core_ir_carrier",
    "p01_semantic_object_model",
    "p01_semantic_proof_family",
    "p01_semantic_benchmark_pack",
    "p01_semantic_output_table",
    "p01_semantic_operator_surfaces",
];
pub const REQUIRED_SEMANTIC_RETIREMENT_GATES: &[&str] = &[
    "gate_rust_semantic_bootstrap_crate",
    "gate_p01_semantic_cli_checks",
    "gate_p01_semantic_text_contracts",
    "gate_p01_semantic_control_plane",
    "gate_p01_semantic_fixture_corpus",
    "gate_p01_semantic_golden_receipts",
    "gate_p01_semantic_docs_examples",
    "gate_p01_semantic_product_surfaces",
    "gate_p01_semantic_receipt_format",
    "gate_p01_semantic_hash_canonicalization",
    "gate_p01_semantic_core_ir_carrier",
    "gate_p01_semantic_object_model",
    "gate_p01_semantic_proof_family",
    "gate_p01_semantic_benchmark_pack",
    "gate_p01_semantic_output_table",
    "gate_p01_semantic_operator_surfaces",
];
pub const REQUIRED_SEMANTIC_SUPERSESSIONS: &[&str] = &[
    "supersede_rust_semantic_bootstrap_crate",
    "supersede_p01_semantic_cli_checks",
    "supersede_p01_semantic_text_contracts",
    "supersede_p01_semantic_control_plane",
    "supersede_p01_semantic_fixture_corpus",
    "supersede_p01_semantic_golden_receipts",
    "supersede_p01_semantic_docs_examples",
    "supersede_p01_semantic_product_surfaces",
    "supersede_p01_semantic_receipt_format",
    "supersede_p01_semantic_hash_canonicalization",
    "supersede_p01_semantic_core_ir_carrier",
    "supersede_p01_semantic_object_model",
    "supersede_p01_semantic_proof_family",
    "supersede_p01_semantic_benchmark_pack",
    "supersede_p01_semantic_output_table",
    "supersede_p01_semantic_operator_surfaces",
];
pub const REQUIRED_SEMANTIC_RETIREMENT_RECEIPTS: &[&str] = &[
    "receipt_semantic_retirement_supersession",
    "receipt_semantic_output_table",
    "receipt_semantic_closure_gate",
    "receipt_semantic_replay_witness",
    "receipt_semantic_core_ir",
    "receipt_semantic_bedrock",
    "receipt_semantic_proof_family",
    "receipt_semantic_benchmark_pack",
];

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
const ALLOWED_SURFACE_KINDS: &[&str] = &[
    "bootstrap",
    "cli",
    "contract",
    "control",
    "fixture",
    "golden",
    "doc",
    "example",
    "product",
    "receipt",
    "hash",
    "closure",
    "operator",
    "model",
    "carrier",
    "benchmark",
];
const ALLOWED_SURFACE_STATUSES: &[&str] =
    &["retirement_scheduled", "retained_by_law", "bounded_active"];
const ALLOWED_GATE_ACTIONS: &[&str] = &[
    "retain_until_replaced",
    "retire_after_replacement",
    "archive_after_supersession",
];
const ALLOWED_GATE_TRIGGERS: &[&str] = &[
    "lyralang_native_equivalent",
    "semantic_ir_native_equivalent",
    "semantic_object_native_equivalent",
    "p01_historical_archive",
    "receipt_format_v2",
    "hash_suite_v2",
    "operator_surface_successor",
    "proof_bundle_native_equivalent",
    "benchmark_harness_successor",
    "output_table_successor",
];
const ALLOWED_GATE_STATUSES: &[&str] = &["armed", "retained_by_law", "blocked_until_successor"];
const ALLOWED_ARCHIVE_LANES: &[&str] = &["historical/superseded", "retained/active"];
const ALLOWED_SUPERSESSION_STATUSES: &[&str] = &["armed", "retained_by_law"];
const ALLOWED_RECEIPT_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];

const FORBIDDEN_SEMANTIC_RETIREMENT_TEXT: &[(&str, ErrorCode)] = &[
    ("network_required:true", ErrorCode::ClosureNetworkDependency),
    ("rule:network_required", ErrorCode::ClosureNetworkDependency),
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
    ("retirement without receipt", ErrorCode::ClosureUnreceipted),
    (
        "unreceipted semantic retirement allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    ("drift accepted", ErrorCode::ClosureDriftAccepted),
    ("retirement drift accepted", ErrorCode::ClosureDriftAccepted),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("wall clock", ErrorCode::AmbientTimeAllowed),
    ("global_closure:true", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_semantic_retirement_supersession_surface(
    input: &str,
) -> Result<SemanticRetirementSupersessionSurface, Vec<ValidationError>> {
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
            "semantic retirement supersession surface is empty",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_RETIREMENT_SUPERSESSION_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_RETIREMENT_SUPERSESSION_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut surfaces = Vec::new();
    let mut gates = Vec::new();
    let mut supersessions = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_surfaces = BTreeSet::new();
    let mut seen_gates = BTreeSet::new();
    let mut seen_supersessions = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();

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
                    "semantic retirement rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(surface_id) = left.strip_prefix("surface:") {
            if !is_symbolic_name(surface_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic retirement surface {surface_id}"),
                ));
                continue;
            }
            if !seen_surfaces.insert(surface_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("surface:{surface_id}"),
                    "semantic retirement surface identity must be unique",
                ));
                continue;
            }
            match parse_surface(line_number, surface_id, value) {
                Ok(item) => surfaces.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(gate_id) = left.strip_prefix("gate:") {
            if !is_symbolic_name(gate_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic retirement gate {gate_id}"),
                ));
                continue;
            }
            if !seen_gates.insert(gate_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("gate:{gate_id}"),
                    "semantic retirement gate identity must be unique",
                ));
                continue;
            }
            match parse_gate(line_number, gate_id, value) {
                Ok(item) => gates.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(supersession_id) = left.strip_prefix("supersession:") {
            if !is_symbolic_name(supersession_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic supersession {supersession_id}"),
                ));
                continue;
            }
            if !seen_supersessions.insert(supersession_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("supersession:{supersession_id}"),
                    "semantic supersession identity must be unique",
                ));
                continue;
            }
            match parse_supersession(line_number, supersession_id, value) {
                Ok(item) => supersessions.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(receipt_id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(receipt_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic retirement receipt {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("receipt:{receipt_id}"),
                    "semantic retirement receipt identity must be unique",
                ));
                continue;
            }
            match parse_receipt(line_number, receipt_id, value) {
                Ok(item) => receipts.push(item),
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
                format!("unknown semantic retirement field {left}"),
            )),
        }
    }

    let phase = match phase {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPhase,
                "field:phase",
                "phase=P01 is required",
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
                "task=P01-X05 is required",
            ));
            String::new()
        }
    };
    let status = match status {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                "field:status",
                "status=artifact_emitted is required",
            ));
            String::new()
        }
    };

    if errors.is_empty() {
        Ok(SemanticRetirementSupersessionSurface {
            header,
            phase,
            task,
            status,
            rules,
            surfaces,
            gates,
            supersessions,
            receipts,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_semantic_retirement_supersession_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_semantic_retirement_supersession_surface(input) {
        Ok(surface) => {
            errors.extend(validate_semantic_retirement_supersession_model(&surface).errors)
        }
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

pub fn validate_semantic_retirement_supersession_model(
    surface: &SemanticRetirementSupersessionSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic retirement law must bind to P01",
        ));
    }
    if surface.task != "P01-X05" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic retirement law must bind to P01-X05",
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported semantic retirement status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_surfaces(surface, &mut errors);
    require_gates(surface, &mut errors);
    require_supersessions(surface, &mut errors);
    require_receipts(surface, &mut errors);
    validate_surfaces(surface, &mut errors);
    validate_gates(surface, &mut errors);
    validate_supersessions(surface, &mut errors);
    validate_receipts(surface, &mut errors);
    validate_semantic_retirement_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_surface(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticRetirementSurfaceBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "semantic retirement surface fields must be key:value segments",
        )
    })?;
    Ok(SemanticRetirementSurfaceBinding {
        line_number,
        id: id.to_string(),
        owner_root: required_field(
            &fields,
            "owner_root",
            ErrorCode::InvalidOwnerRoot,
            line_number,
        )?,
        surface_kind: required_field(
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
        replacement: required_field(
            &fields,
            "replacement",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        retirement_gate: required_field(
            &fields,
            "retirement_gate",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        supersession: required_field(
            &fields,
            "supersession",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        receipt: required_field(
            &fields,
            "receipt",
            ErrorCode::InvalidClosureProof,
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

fn parse_gate(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticRetirementGateBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "semantic retirement gate fields must be key:value segments",
        )
    })?;
    Ok(SemanticRetirementGateBinding {
        line_number,
        id: id.to_string(),
        surface: required_field(
            &fields,
            "surface",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        trigger: required_field(
            &fields,
            "trigger",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        action: required_field(
            &fields,
            "action",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        evidence: split_csv(&required_field(
            &fields,
            "evidence",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}

fn parse_supersession(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticSupersessionBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "semantic supersession fields must be key:value segments",
        )
    })?;
    Ok(SemanticSupersessionBinding {
        line_number,
        id: id.to_string(),
        surface: required_field(
            &fields,
            "surface",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        replaced_by: required_field(
            &fields,
            "replaced_by",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        archive: required_field(
            &fields,
            "archive",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        receipt: required_field(
            &fields,
            "receipt",
            ErrorCode::InvalidClosureProof,
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

fn parse_receipt(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticRetirementReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "semantic retirement receipt fields must be key:value segments",
        )
    })?;
    Ok(SemanticRetirementReceiptBinding {
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

fn require_rules(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for rule in REQUIRED_SEMANTIC_RETIREMENT_RULES {
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
                "required semantic retirement rule missing",
            )),
        }
    }
}
fn require_surfaces(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for required in REQUIRED_SEMANTIC_RETIREMENT_SURFACES {
        if surface.surface_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("surface:{required}"),
                "required semantic retirement surface missing",
            ));
        }
    }
}
fn require_gates(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for required in REQUIRED_SEMANTIC_RETIREMENT_GATES {
        if surface.gate_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("gate:{required}"),
                "required semantic retirement gate missing",
            ));
        }
    }
}
fn require_supersessions(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for required in REQUIRED_SEMANTIC_SUPERSESSIONS {
        if surface.supersession_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("supersession:{required}"),
                "required semantic supersession missing",
            ));
        }
    }
}
fn require_receipts(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for required in REQUIRED_SEMANTIC_RETIREMENT_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{required}"),
                "required semantic retirement receipt missing",
            ));
        }
    }
}

fn validate_surfaces(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for binding in &surface.surfaces {
        if !ALLOWED_OWNER_ROOTS.contains(&binding.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                binding.canonical_identity(),
                format!(
                    "invalid semantic retirement owner root {}",
                    binding.owner_root
                ),
            ));
        }
        if !ALLOWED_SURFACE_KINDS.contains(&binding.surface_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                binding.canonical_identity(),
                format!(
                    "invalid semantic retirement surface kind {}",
                    binding.surface_kind
                ),
            ));
        }
        if !valid_surface_path(&binding.path) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                binding.canonical_identity(),
                format!("invalid semantic retirement surface path {}", binding.path),
            ));
        }
        if !is_symbolic_or_archive_name(&binding.replacement) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                binding.canonical_identity(),
                format!("invalid replacement {}", binding.replacement),
            ));
        }
        if surface.gate_by_id(&binding.retirement_gate).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                binding.canonical_identity(),
                format!(
                    "unknown semantic retirement gate {}",
                    binding.retirement_gate
                ),
            ));
        }
        if surface.supersession_by_id(&binding.supersession).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                binding.canonical_identity(),
                format!("unknown semantic supersession {}", binding.supersession),
            ));
        }
        if surface.receipt_by_id(&binding.receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                binding.canonical_identity(),
                format!("unknown semantic retirement receipt {}", binding.receipt),
            ));
        }
        if !ALLOWED_SURFACE_STATUSES.contains(&binding.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                binding.canonical_identity(),
                format!(
                    "invalid semantic retirement surface status {}",
                    binding.status
                ),
            ));
        }
    }
}

fn validate_gates(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for gate in &surface.gates {
        if surface.surface_by_id(&gate.surface).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                gate.canonical_identity(),
                format!("unknown gate surface {}", gate.surface),
            ));
        }
        if !ALLOWED_GATE_TRIGGERS.contains(&gate.trigger.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gate.canonical_identity(),
                format!("invalid semantic gate trigger {}", gate.trigger),
            ));
        }
        if !ALLOWED_GATE_ACTIONS.contains(&gate.action.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gate.canonical_identity(),
                format!("invalid semantic gate action {}", gate.action),
            ));
        }
        if gate.evidence.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                gate.canonical_identity(),
                "semantic retirement gate must bind evidence receipts",
            ));
        }
        for evidence in &gate.evidence {
            if surface.receipt_by_id(evidence).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    gate.canonical_identity(),
                    format!("unknown semantic gate evidence receipt {evidence}"),
                ));
            }
        }
        if !ALLOWED_GATE_STATUSES.contains(&gate.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                gate.canonical_identity(),
                format!("invalid semantic gate status {}", gate.status),
            ));
        }
    }
}

fn validate_supersessions(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for supersession in &surface.supersessions {
        if surface.surface_by_id(&supersession.surface).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                supersession.canonical_identity(),
                format!("unknown supersession surface {}", supersession.surface),
            ));
        }
        if !is_symbolic_or_archive_name(&supersession.replaced_by) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                supersession.canonical_identity(),
                format!(
                    "invalid semantic supersession replacement {}",
                    supersession.replaced_by
                ),
            ));
        }
        if !ALLOWED_ARCHIVE_LANES.contains(&supersession.archive.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                supersession.canonical_identity(),
                format!("invalid semantic archive lane {}", supersession.archive),
            ));
        }
        if surface.receipt_by_id(&supersession.receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                supersession.canonical_identity(),
                format!(
                    "unknown semantic supersession receipt {}",
                    supersession.receipt
                ),
            ));
        }
        if !ALLOWED_SUPERSESSION_STATUSES.contains(&supersession.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                supersession.canonical_identity(),
                format!(
                    "invalid semantic supersession status {}",
                    supersession.status
                ),
            ));
        }
    }
}

fn validate_receipts(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p01/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                format!("receipt path must be a P01 receipt: {}", receipt.path),
            ));
        }
        if surface.surface_by_id(&receipt.target).is_none()
            && surface.gate_by_id(&receipt.target).is_none()
            && surface.supersession_by_id(&receipt.target).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                receipt.canonical_identity(),
                format!(
                    "unknown semantic retirement receipt target {}",
                    receipt.target
                ),
            ));
        }
        if !ALLOWED_RECEIPT_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!(
                    "invalid semantic retirement receipt status {}",
                    receipt.status
                ),
            ));
        }
    }
}

fn validate_semantic_retirement_report(
    surface: &SemanticRetirementSupersessionSurface,
    errors: &mut Vec<ValidationError>,
) {
    let surface_inputs: Vec<(String, String, String, String, String)> = surface
        .surfaces
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.owner_root.clone(),
                item.surface_kind.clone(),
                item.path.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let gate_inputs: Vec<(String, String, String, String, Vec<String>, String)> = surface
        .gates
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.surface.clone(),
                item.trigger.clone(),
                item.action.clone(),
                item.evidence.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let supersession_inputs: Vec<(String, String, String, String, String)> = surface
        .supersessions
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.surface.clone(),
                item.replaced_by.clone(),
                item.archive.clone(),
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
    let report = deterministic_semantic_retirement_supersession_report(
        &surface_inputs,
        &gate_inputs,
        &supersession_inputs,
        &receipt_inputs,
    );
    if report.surface_count != surface.surfaces.len()
        || report.gate_count != surface.gates.len()
        || report.supersession_count != surface.supersessions.len()
        || report.receipt_count != surface.receipts.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_retirement_report",
            "semantic retirement report count mismatch",
        ));
    }
    if report.bootstrap_surface_count == 0
        || report.retirement_scheduled_count == 0
        || report.retained_surface_count == 0
        || report.semantic_native_successor_count == 0
    {
        errors.push(ValidationError::reject(ErrorCode::MissingClosureOutputGate, "k0_semantic_retirement_report", "semantic retirement report must include bootstrap, scheduled, retained, and native successor gates"));
    }
    if !report.law_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_retirement_report",
            "semantic retirement report hash must be stable fnv1a128",
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
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}
fn is_symbolic_or_archive_name(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'_'
                || byte == b'/'
                || byte == b'-'
        })
}
fn valid_surface_path(path: &str) -> bool {
    (path == "Cargo.toml")
        || (path == "src/lib.rs")
        || (path.starts_with("src/bin/") && path.ends_with(".rs"))
        || (path.starts_with("k0/determinism/src/") && path.ends_with(".rs"))
        || (path.starts_with("lyralang/core/src/") && path.ends_with(".rs"))
        || (path.starts_with("interfaces/p01/contracts/") && path.ends_with(".lyra"))
        || (path.starts_with("interfaces/p01/src/") && path.ends_with(".rs"))
        || (path.starts_with("ops/p01/control/") && path.ends_with(".lyra"))
        || (path.starts_with("ops/p01/closure/") && path.ends_with(".lyra"))
        || (path.starts_with("fixtures/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("goldens/p01/") && path.ends_with(".receipt"))
        || (path.starts_with("receipts/p01/") && path.ends_with(".receipt"))
        || (path.starts_with("docs/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("examples/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("products/p01/") && path.ends_with(".lyra"))
}
fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_RETIREMENT_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic retirement token {needle}"),
            ));
        }
    }
}
