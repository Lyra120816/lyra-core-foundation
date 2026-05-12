use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_bedrock_receipts::deterministic_semantic_bedrock_receipt_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_bedrock_receipts::{
    canonical_semantic_bedrock_registry_hash, semantic_bedrock_anchor_descriptor,
    semantic_bedrock_anchor_ids, semantic_bedrock_anchors_point_to_one_core,
    semantic_bedrock_gate_descriptor, semantic_bedrock_gate_ids,
    semantic_bedrock_no_forked_core_claims, semantic_bedrock_parity_fixture_descriptor,
    semantic_bedrock_parity_fixture_ids, semantic_bedrock_parity_fixtures_cover_receipts,
    semantic_bedrock_receipt_descriptor, semantic_bedrock_receipt_ids,
    semantic_bedrock_receipts_cover_core_chain, LYRA_P01_SEMANTIC_CORE_REF,
};
use crate::p01_semantic_bedrock_receipts_model::{
    SemanticBedrockAnchorBinding, SemanticBedrockGateBinding, SemanticBedrockParityFixtureBinding,
    SemanticBedrockReceiptBinding, SemanticBedrockReceiptsSurface,
};

pub const P01_SEMANTIC_BEDROCK_RECEIPTS_CONTRACT: &str = "LYRA-P01-SEMANTIC-BEDROCK-RECEIPTS v1";
pub const REQUIRED_SEMANTIC_BEDROCK_RECEIPT_RULES: &[&str] = &[
    "all_admitted_p01_receipts_are_bound",
    "all_semantic_surfaces_anchor_to_one_core",
    "receipt_contract_law_parity_is_enforced",
    "positive_and_negative_fixtures_bind_each_frontier",
    "semantic_bedrock_has_no_forked_core",
    "semantic_bedrock_receipts_are_canonical",
    "no_runtime_network_dependency",
    "no_probabilistic_receipt_truth",
    "no_hidden_randomness",
    "no_placeholder_bedrock_claim",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_BEDROCK_RECEIPTS: &[&str] = &[
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
];
pub const REQUIRED_SEMANTIC_BEDROCK_ANCHORS: &[&str] = &[
    "semantic_atoms_core_anchor",
    "core_ir_core_anchor",
    "semantic_objects_core_anchor",
    "semantic_identity_core_anchor",
    "reference_semantics_core_anchor",
    "symbolic_equality_core_anchor",
    "error_challenge_evidence_core_anchor",
    "semantic_serialization_hashing_core_anchor",
    "semantic_adversarial_corpus_core_anchor",
    "core_ir_reuse_core_anchor",
    "semantic_atom_reference_core_anchor",
];
pub const REQUIRED_SEMANTIC_BEDROCK_PARITY_FIXTURES: &[&str] = &[
    "semantic_atoms_receipt_parity",
    "core_ir_receipt_parity",
    "semantic_objects_receipt_parity",
    "semantic_identity_receipt_parity",
    "reference_semantics_receipt_parity",
    "symbolic_equality_receipt_parity",
    "error_challenge_evidence_receipt_parity",
    "semantic_serialization_hashing_receipt_parity",
    "semantic_adversarial_corpus_receipt_parity",
    "core_ir_reuse_receipt_parity",
    "semantic_atom_reference_receipt_parity",
];
pub const REQUIRED_SEMANTIC_BEDROCK_GATES: &[&str] = &[
    "receipt_chain_complete_gate",
    "one_core_anchor_gate",
    "parity_fixture_gate",
    "no_forked_bedrock_gate",
];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_SEMANTIC_BEDROCK_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic receipt",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic receipt", ErrorCode::ProbabilisticTruthAllowed),
    ("random receipt", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder bedrock", ErrorCode::PlaceholderAllowed),
    ("stub bedrock", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("forked semantic core", ErrorCode::SemanticDriftAccepted),
];

pub fn parse_semantic_bedrock_receipts_surface(
    input: &str,
) -> Result<SemanticBedrockReceiptsSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => {
            return Err(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "input",
                format!("{error:?}"),
            )])
        }
    };
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "empty semantic bedrock receipts surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P01_SEMANTIC_BEDROCK_RECEIPTS_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_BEDROCK_RECEIPTS_CONTRACT}"),
        ));
    }

    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut receipts = Vec::new();
    let mut anchors = Vec::new();
    let mut fixtures = Vec::new();
    let mut gates = Vec::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
        if let Some(value) = line.strip_prefix("phase=") {
            phase = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("task=") {
            task = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("status=") {
            status = Some(value.to_string());
        } else if let Some((name, value)) = line
            .strip_prefix("rule:")
            .and_then(|value| value.split_once('='))
        {
            if rules.insert(name.to_string(), value.to_string()).is_some() {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    format!("line:{line_number:03}"),
                    format!("duplicate rule {name}"),
                ));
            }
        } else if let Some(value) = line.strip_prefix("receipt=") {
            let fields = parse_pipe_fields(value);
            if required_field(&fields, "id").is_none()
                || required_field(&fields, "task").is_none()
                || required_field(&fields, "surface").is_none()
                || required_field(&fields, "path").is_none()
                || required_field(&fields, "expected").is_none()
                || required_field(&fields, "status").is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "receipt requires id task surface path expected status",
                ));
            }
            receipts.push(SemanticBedrockReceiptBinding {
                line_number,
                id: field(&fields, "id"),
                task: field(&fields, "task"),
                surface: field(&fields, "surface"),
                path: field(&fields, "path"),
                expected_hash: field(&fields, "expected"),
                status: field(&fields, "status"),
            });
        } else if let Some(value) = line.strip_prefix("anchor=") {
            let fields = parse_pipe_fields(value);
            if required_field(&fields, "id").is_none()
                || required_field(&fields, "owner").is_none()
                || required_field(&fields, "module").is_none()
                || required_field(&fields, "contract").is_none()
                || required_field(&fields, "law").is_none()
                || required_field(&fields, "receipt").is_none()
                || required_field(&fields, "core").is_none()
                || required_field(&fields, "status").is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "anchor requires id owner module contract law receipt core status",
                ));
            }
            anchors.push(SemanticBedrockAnchorBinding {
                line_number,
                id: field(&fields, "id"),
                owner_root: field(&fields, "owner"),
                module: field(&fields, "module"),
                contract: field(&fields, "contract"),
                law: field(&fields, "law"),
                receipt_ref: field(&fields, "receipt"),
                core_ref: field(&fields, "core"),
                status: field(&fields, "status"),
            });
        } else if let Some(value) = line.strip_prefix("fixture=") {
            let fields = parse_pipe_fields(value);
            if required_field(&fields, "id").is_none()
                || required_field(&fields, "positive").is_none()
                || required_field(&fields, "negative").is_none()
                || required_field(&fields, "receipt").is_none()
                || required_field(&fields, "golden").is_none()
                || required_field(&fields, "status").is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "fixture requires id positive negative receipt golden status",
                ));
            }
            fixtures.push(SemanticBedrockParityFixtureBinding {
                line_number,
                id: field(&fields, "id"),
                positive: field(&fields, "positive"),
                negative: field(&fields, "negative"),
                receipt_ref: field(&fields, "receipt"),
                golden: field(&fields, "golden"),
                status: field(&fields, "status"),
            });
        } else if let Some(value) = line.strip_prefix("gate=") {
            let fields = parse_pipe_fields(value);
            if required_field(&fields, "id").is_none()
                || required_field(&fields, "scope").is_none()
                || required_field(&fields, "law").is_none()
                || required_field(&fields, "evidence").is_none()
                || required_field(&fields, "status").is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "gate requires id scope law evidence status",
                ));
            }
            gates.push(SemanticBedrockGateBinding {
                line_number,
                id: field(&fields, "id"),
                scope: field(&fields, "scope"),
                law: field(&fields, "law"),
                evidence: field(&fields, "evidence"),
                status: field(&fields, "status"),
            });
        } else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unrecognized semantic bedrock receipt line {line}"),
            ));
        }
    }

    if let Some(value) = &phase {
        if value.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPhase,
                "phase",
                "empty phase",
            ));
        }
    }
    if let Some(value) = &task {
        if value.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingTask,
                "task",
                "empty task",
            ));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(SemanticBedrockReceiptsSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        receipts,
        anchors,
        fixtures,
        gates,
    })
}

pub fn validate_semantic_bedrock_receipts_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_semantic_bedrock_receipts_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_semantic_bedrock_receipts_model(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_bedrock_receipts_model(
    surface: &SemanticBedrockReceiptsSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-012" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-012 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }

    for required in REQUIRED_SEMANTIC_BEDROCK_RECEIPT_RULES {
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
                "missing semantic bedrock receipt rule",
            )),
        }
    }

    require_ids(
        "receipt",
        REQUIRED_SEMANTIC_BEDROCK_RECEIPTS,
        surface
            .receipts
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingReceiptProof,
        errors,
    );
    require_ids(
        "anchor",
        REQUIRED_SEMANTIC_BEDROCK_ANCHORS,
        surface
            .anchors
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingCanonicalModel,
        errors,
    );
    require_ids(
        "fixture",
        REQUIRED_SEMANTIC_BEDROCK_PARITY_FIXTURES,
        surface
            .fixtures
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingProofBinding,
        errors,
    );
    require_ids(
        "gate",
        REQUIRED_SEMANTIC_BEDROCK_GATES,
        surface.gates.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingModelBinding,
        errors,
    );

    check_duplicate_bindings(
        "receipt",
        surface
            .receipts
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "anchor",
        surface
            .anchors
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "fixture",
        surface
            .fixtures
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "gate",
        surface
            .gates
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );

    for binding in &surface.receipts {
        validate_status(
            "receipt",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_bedrock_receipt_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::OrphanReceiptBinding,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic bedrock receipt {}", binding.id),
            ));
            continue;
        };
        if binding.task != descriptor.task
            || binding.surface != descriptor.surface
            || binding.path != descriptor.path
            || binding.expected_hash != descriptor.expected_hash
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", binding.line_number),
                format!("receipt descriptor drift {}", binding.id),
            ));
        }
        if binding.id == "receipt_semantic_bedrock_receipts" && binding.task != "P01-012" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                format!("line:{:03}", binding.line_number),
                "semantic bedrock self receipt must target P01-012",
            ));
        }
    }

    for binding in &surface.anchors {
        validate_status(
            "anchor",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_bedrock_anchor_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic bedrock anchor {}", binding.id),
            ));
            continue;
        };
        if binding.owner_root != descriptor.owner_root
            || binding.module != descriptor.module
            || binding.contract != descriptor.contract
            || binding.law != descriptor.law
            || binding.receipt_ref != descriptor.receipt_ref
            || binding.core_ref != descriptor.core_ref
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("anchor descriptor drift {}", binding.id),
            ));
        }
        if binding.core_ref != LYRA_P01_SEMANTIC_CORE_REF {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!(
                    "anchor {} points to forked core {}",
                    binding.id, binding.core_ref
                ),
            ));
        }
        if semantic_bedrock_receipt_descriptor(&binding.receipt_ref).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::OrphanReceiptBinding,
                format!("line:{:03}", binding.line_number),
                format!(
                    "anchor {} references unknown receipt {}",
                    binding.id, binding.receipt_ref
                ),
            ));
        }
    }

    for binding in &surface.fixtures {
        validate_status(
            "fixture",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = semantic_bedrock_parity_fixture_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic bedrock fixture {}", binding.id),
            ));
            continue;
        };
        if binding.positive != descriptor.positive
            || binding.negative != descriptor.negative
            || binding.receipt_ref != descriptor.receipt_ref
            || binding.golden != descriptor.golden
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("fixture descriptor drift {}", binding.id),
            ));
        }
        if semantic_bedrock_receipt_descriptor(&binding.receipt_ref).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::OrphanReceiptBinding,
                format!("line:{:03}", binding.line_number),
                format!(
                    "fixture {} references unknown receipt {}",
                    binding.id, binding.receipt_ref
                ),
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
        let Some(descriptor) = semantic_bedrock_gate_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                format!("line:{:03}", binding.line_number),
                format!("unknown semantic bedrock gate {}", binding.id),
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

    if !semantic_bedrock_receipts_cover_core_chain()
        || !semantic_bedrock_anchors_point_to_one_core()
        || !semantic_bedrock_parity_fixtures_cover_receipts()
        || !semantic_bedrock_no_forked_core_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::SemanticDriftAccepted,
            "bedrock",
            "semantic bedrock coverage is incomplete or forked",
        ));
    }

    let receipt_rows: Vec<(String, String, String, String, String, String)> = surface
        .receipts
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.task.clone(),
                item.surface.clone(),
                item.path.clone(),
                item.expected_hash.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let anchor_rows: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .anchors
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.owner_root.clone(),
                item.module.clone(),
                item.contract.clone(),
                item.law.clone(),
                item.receipt_ref.clone(),
                item.core_ref.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let fixture_rows: Vec<(String, String, String, String, String, String)> = surface
        .fixtures
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.positive.clone(),
                item.negative.clone(),
                item.receipt_ref.clone(),
                item.golden.clone(),
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
    let suite = deterministic_semantic_bedrock_receipt_suite_report(
        &receipt_rows,
        &anchor_rows,
        &fixture_rows,
        &gate_rows,
    );
    if suite.receipt_count < semantic_bedrock_receipt_ids().len()
        || suite.anchor_count < semantic_bedrock_anchor_ids().len()
        || suite.fixture_count < semantic_bedrock_parity_fixture_ids().len()
        || suite.gate_count < semantic_bedrock_gate_ids().len()
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !canonical_semantic_bedrock_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "suite",
            "semantic bedrock receipt suite report is incomplete or unhashable",
        ));
    }
}

fn require_ids(
    kind: &str,
    required: &[&str],
    actual: BTreeSet<&str>,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    for id in required {
        if !actual.contains(id) {
            errors.push(ValidationError::reject(
                code,
                format!("{kind}:{id}"),
                format!("missing required semantic bedrock {kind} {id}"),
            ));
        }
    }
}

fn check_duplicate_bindings(
    kind: &str,
    items: Vec<(&str, usize)>,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for (id, line_number) in items {
        if !seen.insert(id.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate semantic bedrock {kind} {id}"),
            ));
        }
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
    for (token, code) in FORBIDDEN_SEMANTIC_BEDROCK_TEXT {
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
