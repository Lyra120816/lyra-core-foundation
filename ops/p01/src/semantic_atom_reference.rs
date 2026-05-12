use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_atom_reference::deterministic_semantic_atom_reference_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_atom_reference::{
    canonical_semantic_atom_reference_registry_hash, semantic_atom_inspection_tool_descriptor,
    semantic_atom_inspection_tool_ids, semantic_atom_reference_all_atoms_exported,
    semantic_atom_reference_example_descriptor, semantic_atom_reference_example_ids,
    semantic_atom_reference_examples_cover_all_atoms, semantic_atom_reference_gate_descriptor,
    semantic_atom_reference_gate_ids, semantic_atom_reference_library_descriptor,
    semantic_atom_reference_library_exports_atom, semantic_atom_reference_library_ids,
};
use crate::lyralang_semantic_atoms::core_atom_descriptor;
use crate::p01_semantic_atom_reference_model::{
    SemanticAtomInspectionToolBinding, SemanticAtomReferenceExampleBinding,
    SemanticAtomReferenceGateBinding, SemanticAtomReferenceLibraryBinding,
    SemanticAtomReferenceReceiptBinding, SemanticAtomReferenceSurface,
};

pub const P01_SEMANTIC_ATOM_REFERENCE_CONTRACT: &str = "LYRA-P01-SEMANTIC-ATOM-REFERENCE v1";
pub const REQUIRED_SEMANTIC_ATOM_REFERENCE_RULES: &[&str] = &[
    "reference_libraries_export_all_core_atoms",
    "examples_cover_each_core_atom",
    "inspection_tooling_is_read_only",
    "reference_views_bind_semantic_atom_registry",
    "no_reference_registry_drift",
    "no_runtime_network_dependency",
    "no_probabilistic_reference_truth",
    "no_hidden_randomness",
    "no_placeholder_reference_library",
    "receipts_bind_reference_surface",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_ATOM_REFERENCE_LIBRARIES: &[&str] = &[
    "core_atom_reference_library",
    "interface_atom_reference_library",
    "operator_atom_reference_library",
];
pub const REQUIRED_SEMANTIC_ATOM_REFERENCE_EXAMPLES: &[&str] = &[
    "symbol_atom_reference_example",
    "value_atom_reference_example",
    "type_atom_reference_example",
    "effect_atom_reference_example",
    "capability_atom_reference_example",
    "proof_atom_reference_example",
    "receipt_atom_reference_example",
    "resource_atom_reference_example",
    "law_atom_reference_example",
];
pub const REQUIRED_SEMANTIC_ATOM_INSPECTION_TOOLS: &[&str] = &[
    "semantic_atom_reference_cli",
    "semantic_atom_reference_index",
    "semantic_atom_reference_product_surface",
];
pub const REQUIRED_SEMANTIC_ATOM_REFERENCE_GATES: &[&str] = &[
    "all_atoms_exported_gate",
    "examples_cover_atoms_gate",
    "inspection_is_read_only_gate",
];
pub const REQUIRED_SEMANTIC_ATOM_REFERENCE_RECEIPTS: &[&str] = &["receipt_semantic_atom_reference"];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_SEMANTIC_ATOM_REFERENCE_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic reference", ErrorCode::ProbabilisticTruthAllowed),
    ("random reference", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder reference", ErrorCode::PlaceholderAllowed),
    ("stub reference", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_atom_reference_surface(
    input: &str,
) -> Result<SemanticAtomReferenceSurface, Vec<ValidationError>> {
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
            "no semantic atom reference lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_ATOM_REFERENCE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_ATOM_REFERENCE_CONTRACT}"),
        )]);
    }
    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut libraries = Vec::new();
    let mut examples = Vec::new();
    let mut tools = Vec::new();
    let mut gates = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_libraries = BTreeSet::new();
    let mut seen_examples = BTreeSet::new();
    let mut seen_tools = BTreeSet::new();
    let mut seen_gates = BTreeSet::new();
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
            "library" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidCanonicalModel,
                        format!("line:{line_number:03}"),
                        "library missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_libraries.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateCanonicalModel,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid library {id}"),
                    ));
                    continue;
                }
                libraries.push(SemanticAtomReferenceLibraryBinding {
                    line_number,
                    id: id.to_string(),
                    owner_root: field(&fields, "owner"),
                    registry_ref: field(&fields, "registry"),
                    atom_ids: field(&fields, "atoms"),
                    library_path: field(&fields, "path"),
                    export_contract: field(&fields, "export"),
                    status: field(&fields, "status"),
                });
            }
            "example" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "example missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_examples.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid example {id}"),
                    ));
                    continue;
                }
                examples.push(SemanticAtomReferenceExampleBinding {
                    line_number,
                    id: id.to_string(),
                    library_ref: field(&fields, "library"),
                    atom_id: field(&fields, "atom"),
                    example_path: field(&fields, "path"),
                    expected_inspection: field(&fields, "expected"),
                    status: field(&fields, "status"),
                });
            }
            "tool" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "tool missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_tools.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid tool {id}"),
                    ));
                    continue;
                }
                tools.push(SemanticAtomInspectionToolBinding {
                    line_number,
                    id: id.to_string(),
                    binary: field(&fields, "binary"),
                    input_contract: field(&fields, "input"),
                    output_contract: field(&fields, "output"),
                    fixture_path: field(&fields, "fixture"),
                    receipt_ref: field(&fields, "receipt"),
                    status: field(&fields, "status"),
                });
            }
            "gate" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidModelBinding,
                        format!("line:{line_number:03}"),
                        "gate missing id",
                    ));
                    continue;
                };
                if !is_symbolic_name(id) || !seen_gates.insert(id.to_string()) {
                    errors.push(ValidationError::reject(
                        ErrorCode::DuplicateModelBinding,
                        format!("line:{line_number:03}"),
                        format!("duplicate or invalid gate {id}"),
                    ));
                    continue;
                }
                gates.push(SemanticAtomReferenceGateBinding {
                    line_number,
                    id: id.to_string(),
                    scope: field(&fields, "scope"),
                    law: field(&fields, "law"),
                    evidence: field(&fields, "evidence"),
                    status: field(&fields, "status"),
                });
            }
            "receipt" => {
                let fields = parse_pipe_fields(value);
                let Some(id) = required_field(&fields, "id") else {
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
                receipts.push(SemanticAtomReferenceReceiptBinding {
                    line_number,
                    id: id.to_string(),
                    path: field(&fields, "path"),
                    target: field(&fields, "target"),
                    status: field(&fields, "status"),
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
    Ok(SemanticAtomReferenceSurface {
        header,
        phase,
        task,
        status,
        rules,
        libraries,
        examples,
        tools,
        gates,
        receipts,
    })
}

pub fn validate_semantic_atom_reference_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_semantic_atom_reference_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_semantic_atom_reference_surface_model(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_atom_reference_surface_model(
    surface: &SemanticAtomReferenceSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-011" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-011 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }
    for required in REQUIRED_SEMANTIC_ATOM_REFERENCE_RULES {
        match surface.rules.get(*required) {
            Some(value) if value == "required" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                format!("rule:{required}"),
                format!("expected required got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "missing semantic atom reference rule",
            )),
        }
    }
    for required in REQUIRED_SEMANTIC_ATOM_REFERENCE_LIBRARIES {
        if surface.library_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("library:{required}"),
                "missing semantic atom reference library",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_ATOM_REFERENCE_EXAMPLES {
        if surface.example_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("example:{required}"),
                "missing semantic atom reference example",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_ATOM_INSPECTION_TOOLS {
        if surface.tool_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("tool:{required}"),
                "missing semantic atom inspection tool",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_ATOM_REFERENCE_GATES {
        if surface.gate_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("gate:{required}"),
                "missing semantic atom reference gate",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_ATOM_REFERENCE_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "missing semantic atom reference receipt",
            ));
        }
    }
    for binding in &surface.libraries {
        validate_status(
            "library",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_atom_reference_library_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic atom reference library {}", binding.id),
            ));
            continue;
        };
        if binding.owner_root != descriptor.owner_root
            || binding.registry_ref != descriptor.registry_ref
            || binding.atom_ids != descriptor.atom_ids
            || binding.library_path != descriptor.library_path
            || binding.export_contract != descriptor.export_contract
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("library descriptor drift {}", binding.id),
            ));
        }
        for atom_id in binding.atom_ids.split(',') {
            if core_atom_descriptor(atom_id).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    format!("line:{:03}", binding.line_number),
                    format!("unknown atom {atom_id}"),
                ));
            }
        }
    }
    for binding in &surface.examples {
        validate_status(
            "example",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_atom_reference_example_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic atom reference example {}", binding.id),
            ));
            continue;
        };
        if binding.library_ref != descriptor.library_ref
            || binding.atom_id != descriptor.atom_id
            || binding.example_path != descriptor.example_path
            || binding.expected_inspection != descriptor.expected_inspection
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("example descriptor drift {}", binding.id),
            ));
        }
        if !semantic_atom_reference_library_exports_atom(&binding.library_ref, &binding.atom_id) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!(
                    "example atom {} is not exported by {}",
                    binding.atom_id, binding.library_ref
                ),
            ));
        }
    }
    for binding in &surface.tools {
        validate_status(
            "tool",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_atom_inspection_tool_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic atom inspection tool {}", binding.id),
            ));
            continue;
        };
        if binding.binary != descriptor.binary
            || binding.input_contract != descriptor.input_contract
            || binding.output_contract != descriptor.output_contract
            || binding.fixture_path != descriptor.fixture_path
            || binding.receipt_ref != descriptor.receipt_ref
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("tool descriptor drift {}", binding.id),
            ));
        }
    }
    for binding in &surface.gates {
        validate_status(
            "gate",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_atom_reference_gate_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic atom reference gate {}", binding.id),
            ));
            continue;
        };
        if binding.scope != descriptor.scope
            || binding.law != descriptor.law
            || binding.evidence != descriptor.evidence
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("gate descriptor drift {}", binding.id),
            ));
        }
    }
    for binding in &surface.receipts {
        validate_status(
            "receipt",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        if binding.id == "receipt_semantic_atom_reference" {
            if binding.path != "receipts/p01/pass_0040_semantic_atom_reference.receipt" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidProofBinding,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt path {}", binding.path),
                ));
            }
            if binding.target != "P01-011" {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    format!("line:{:03}", binding.line_number),
                    format!("unexpected receipt target {}", binding.target),
                ));
            }
        }
    }
    if !semantic_atom_reference_all_atoms_exported()
        || !semantic_atom_reference_examples_cover_all_atoms()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "coverage",
            "semantic atom reference coverage is incomplete",
        ));
    }
    let library_rows: Vec<(String, String, String, String, String, String, String)> = surface
        .libraries
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.owner_root.clone(),
                item.registry_ref.clone(),
                item.atom_ids.clone(),
                item.library_path.clone(),
                item.export_contract.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let example_rows: Vec<(String, String, String, String, String, String)> = surface
        .examples
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.library_ref.clone(),
                item.atom_id.clone(),
                item.example_path.clone(),
                item.expected_inspection.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let tool_rows: Vec<(String, String, String, String, String, String, String)> = surface
        .tools
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.binary.clone(),
                item.input_contract.clone(),
                item.output_contract.clone(),
                item.fixture_path.clone(),
                item.receipt_ref.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let gate_rows: Vec<(String, String, String, String, String)> = surface
        .gates
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.scope.clone(),
                item.law.clone(),
                item.evidence.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let receipt_rows: Vec<(String, String, String, String)> = surface
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
    let suite = deterministic_semantic_atom_reference_suite_report(
        &library_rows,
        &example_rows,
        &tool_rows,
        &gate_rows,
        &receipt_rows,
    );
    if suite.library_count < semantic_atom_reference_library_ids().len()
        || suite.example_count < semantic_atom_reference_example_ids().len()
        || suite.tool_count < semantic_atom_inspection_tool_ids().len()
        || suite.gate_count < semantic_atom_reference_gate_ids().len()
        || suite.receipt_count < REQUIRED_SEMANTIC_ATOM_REFERENCE_RECEIPTS.len()
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !canonical_semantic_atom_reference_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "suite",
            "semantic atom reference suite report is incomplete or unhashable",
        ));
    }
}

fn validate_status(
    kind: &str,
    id: &str,
    line_number: usize,
    status: &str,
    errors: &mut Vec<ValidationError>,
) {
    if !ALLOWED_STATUSES.contains(&status) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("line:{line_number:03}"),
            format!("{kind} {id} has unsupported status {status}"),
        ));
    }
}
fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_SEMANTIC_ATOM_REFERENCE_TEXT {
        if lower.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden_text",
                format!("forbidden token {token}"),
            ));
        }
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
    fields
        .get(name)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
}
fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    required_field(fields, name).unwrap_or("").to_string()
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && matches!(value.as_bytes().first(), Some(byte) if byte.is_ascii_lowercase())
}
