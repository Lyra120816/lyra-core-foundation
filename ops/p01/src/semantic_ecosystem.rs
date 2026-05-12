use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_ecosystem::deterministic_semantic_ecosystem_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_ecosystem_model::{
    SemanticEcosystemDoc, SemanticEcosystemExample, SemanticEcosystemProof,
    SemanticEcosystemSurface,
};

pub const P01_SEMANTIC_ECOSYSTEM_CONTRACT: &str = "LYRA-P01-SEMANTIC-ECOSYSTEM-DOCS-EXAMPLES v1";

pub const REQUIRED_SEMANTIC_ECOSYSTEM_RULES: &[&str] = &[
    "semantic_ecosystem_docs_must_be_receipted",
    "semantic_examples_must_be_executable",
    "canonical_symbols_atoms_ir_coverage_required",
    "offline_first_distribution",
    "docs_examples_bound_to_proofs",
    "semantic_deployment_bridge_required",
    "no_remote_service_dependency",
    "no_documentation_only_claim",
    "phase_open_until_ecosystem_proven",
];

pub const REQUIRED_SEMANTIC_ECOSYSTEM_DOCS: &[&str] = &[
    "semantic_symbol_operator_guide",
    "semantic_atom_developer_reference",
    "core_ir_ecosystem_walkthrough",
    "canonical_semantics_contributor_onboarding",
    "offline_distribution_reference",
    "public_semantic_review_reference",
];

pub const REQUIRED_SEMANTIC_ECOSYSTEM_EXAMPLES: &[&str] = &[
    "canonical_symbol_walkthrough",
    "semantic_atom_extension_flow",
    "core_ir_receipt_review",
    "offline_operator_semantic_review",
    "negative_semantic_doc_drift_rejection",
    "deployment_to_ecosystem_handoff",
];

pub const REQUIRED_SEMANTIC_ECOSYSTEM_PROOFS: &[&str] = &[
    "docs_coverage_proof",
    "executable_examples_proof",
    "receipt_binding_proof",
    "offline_distribution_proof",
    "deployment_bridge_proof",
    "p01_phase_open",
];

const REQUIRED_COVERAGE_ANCHORS: &[&str] = &["canonical_symbols", "semantic_atoms", "core_ir"];
const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_AUDIENCES: &[&str] = &[
    "operator",
    "developer",
    "contributor",
    "public",
    "steward",
    "enterprise",
];
const ALLOWED_EXAMPLE_KINDS: &[&str] = &[
    "walkthrough",
    "extension_flow",
    "review",
    "negative",
    "handoff",
    "corpus_flow",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &[
    "docs",
    "examples",
    "receipt",
    "distribution",
    "deployment",
    "phase",
];

const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p01-atom-check",
    "lyra-p01-ir-check",
    "lyra-p01-object-check",
    "lyra-p01-identity-check",
    "lyra-p01-reference-semantics-check",
    "lyra-p01-symbolic-equality-check",
    "lyra-p01-error-challenge-evidence-check",
    "lyra-p01-semantic-serialization-hashing-check",
    "lyra-p01-semantic-adversarial-corpus-check",
    "lyra-p01-core-ir-reuse-check",
    "lyra-p01-semantic-atom-reference-check",
    "lyra-p01-semantic-bedrock-receipts-check",
    "lyra-p01-formal-semantic-constitution-check",
    "lyra-p01-canonical-data-model-check",
    "lyra-p01-semantic-core-engine-check",
    "lyra-p01-semantic-falsification-check",
    "lyra-p01-semantic-replay-check",
    "lyra-p01-semantic-interface-check",
    "lyra-p01-semantic-packaging-check",
    "lyra-p01-semantic-deployment-check",
    "lyra-p01-semantic-ecosystem-check",
];

const FORBIDDEN_SEMANTIC_ECOSYSTEM_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::EcosystemNetworkDependency),
    ("cloud required", ErrorCode::EcosystemNetworkDependency),
    ("online required", ErrorCode::EcosystemNetworkDependency),
    (
        "remote service required",
        ErrorCode::EcosystemNetworkDependency,
    ),
    ("remote fetch", ErrorCode::EcosystemNetworkDependency),
    ("manual only", ErrorCode::EcosystemDocsOnly),
    ("documentation alone", ErrorCode::EcosystemDocsOnly),
    (
        "ecosystem drift accepted",
        ErrorCode::EcosystemDriftAccepted,
    ),
    ("doc drift accepted", ErrorCode::EcosystemDriftAccepted),
    ("example drift accepted", ErrorCode::EcosystemDriftAccepted),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_ecosystem_surface(
    input: &str,
) -> Result<SemanticEcosystemSurface, Vec<ValidationError>> {
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
            "no semantic ecosystem docs/examples surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_ECOSYSTEM_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_ECOSYSTEM_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut docs = Vec::new();
    let mut examples = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_docs = BTreeSet::new();
    let mut seen_examples = BTreeSet::new();
    let mut seen_proofs = BTreeSet::new();

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
                    "semantic ecosystem rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(doc_id) = left.strip_prefix("doc:") {
            if !is_symbolic_name(doc_id) || !seen_docs.insert(doc_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEcosystemDoc,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic ecosystem doc {doc_id}"),
                ));
                continue;
            }
            match parse_doc(line_number, doc_id, value) {
                Ok(doc) => docs.push(doc),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(example_id) = left.strip_prefix("example:") {
            if !is_symbolic_name(example_id) || !seen_examples.insert(example_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEcosystemExample,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic ecosystem example {example_id}"),
                ));
                continue;
            }
            match parse_example(line_number, example_id, value) {
                Ok(example) => examples.push(example),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) || !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEcosystemProof,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic ecosystem proof {proof_id}"),
                ));
                continue;
            }
            match parse_proof(line_number, proof_id, value) {
                Ok(proof) => proofs.push(proof),
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
                format!("unknown semantic ecosystem key {left}"),
            )),
        }
    }
    if phase.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingPhase,
            "surface",
            "missing phase",
        ));
    }
    if task.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingTask,
            "surface",
            "missing task",
        ));
    }
    if status.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            "surface",
            "missing status",
        ));
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticEcosystemSurface {
        header,
        phase: phase.unwrap(),
        task: task.unwrap(),
        status: status.unwrap(),
        rules,
        docs,
        examples,
        proofs,
    })
}

pub fn validate_semantic_ecosystem_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_semantic_ecosystem_surface(input) {
        Ok(surface) => validate_semantic_ecosystem_model(&surface, &mut errors),
        Err(mut parse_errors) => errors.append(&mut parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_semantic_ecosystem_model(
    surface: &SemanticEcosystemSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic ecosystem law must bind to P01",
        ));
    }
    if surface.task != "P01-021" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic ecosystem law must bind to P01-021",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported semantic ecosystem status {}", surface.status),
        ));
    }
    require_rules(surface, errors);
    require_docs(surface, errors);
    require_examples(surface, errors);
    require_proofs(surface, errors);
    validate_docs(surface, errors);
    validate_examples(surface, errors);
    validate_proofs(surface, errors);
    validate_coverage(surface, errors);
    validate_semantic_ecosystem_report(surface, errors);
}

fn parse_doc(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticEcosystemDoc, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEcosystemDoc,
            format!("line:{line_number:03}"),
            "doc fields must be key:value segments",
        )
    })?;
    let audience = required_field(
        &fields,
        "audience",
        ErrorCode::InvalidEcosystemDoc,
        line_number,
    )?;
    let path = required_field(&fields, "path", ErrorCode::InvalidEcosystemDoc, line_number)?;
    let covers = split_csv(&required_field(
        &fields,
        "covers",
        ErrorCode::InvalidEcosystemDoc,
        line_number,
    )?);
    let examples = split_csv(&required_field(
        &fields,
        "examples",
        ErrorCode::InvalidEcosystemDoc,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidEcosystemDoc,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidEcosystemDoc,
        line_number,
    )?;
    Ok(SemanticEcosystemDoc {
        line_number,
        id: id.to_string(),
        audience,
        path,
        covers,
        examples,
        receipts,
        status,
    })
}

fn parse_example(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticEcosystemExample, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEcosystemExample,
            format!("line:{line_number:03}"),
            "example fields must be key:value segments",
        )
    })?;
    let kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidEcosystemExample,
        line_number,
    )?;
    let path = required_field(
        &fields,
        "path",
        ErrorCode::InvalidEcosystemExample,
        line_number,
    )?;
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidEcosystemExample,
        line_number,
    )?);
    let proofs = split_csv(&required_field(
        &fields,
        "proofs",
        ErrorCode::InvalidEcosystemExample,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidEcosystemExample,
        line_number,
    )?);
    let rejects = split_csv(&required_field(
        &fields,
        "rejects",
        ErrorCode::InvalidEcosystemExample,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidEcosystemExample,
        line_number,
    )?;
    Ok(SemanticEcosystemExample {
        line_number,
        id: id.to_string(),
        kind,
        path,
        commands,
        proofs,
        receipts,
        rejects,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticEcosystemProof, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEcosystemProof,
            format!("line:{line_number:03}"),
            "proof fields must be key:value segments",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidEcosystemProof,
        line_number,
    )?;
    let docs = split_csv(&required_field(
        &fields,
        "docs",
        ErrorCode::InvalidEcosystemProof,
        line_number,
    )?);
    let examples = split_csv(&required_field(
        &fields,
        "examples",
        ErrorCode::InvalidEcosystemProof,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidEcosystemProof,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidEcosystemProof,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidEcosystemProof,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidEcosystemProof,
        line_number,
    )?;
    Ok(SemanticEcosystemProof {
        line_number,
        id: id.to_string(),
        scope,
        docs,
        examples,
        receipts,
        commands,
        forbids,
        status,
    })
}

fn require_rules(surface: &SemanticEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_SEMANTIC_ECOSYSTEM_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("blocked_until_proven") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemRule,
                format!("rule:{rule}"),
                "required semantic ecosystem rule missing",
            )),
        }
    }
}

fn require_docs(surface: &SemanticEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_ECOSYSTEM_DOCS {
        if surface.doc_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemDoc,
                format!("doc:{id}"),
                "required semantic ecosystem doc missing",
            ));
        }
    }
}

fn require_examples(surface: &SemanticEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_ECOSYSTEM_EXAMPLES {
        if surface.example_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemExample,
                format!("example:{id}"),
                "required semantic ecosystem example missing",
            ));
        }
    }
}

fn require_proofs(surface: &SemanticEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_ECOSYSTEM_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemProof,
                format!("proof:{id}"),
                "required semantic ecosystem proof missing",
            ));
        }
    }
}

fn validate_docs(surface: &SemanticEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for doc in &surface.docs {
        if !ALLOWED_AUDIENCES.contains(&doc.audience.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemDoc,
                doc.canonical_identity(),
                format!("invalid doc audience {}", doc.audience),
            ));
        }
        if !doc.path.starts_with("docs/")
            && !doc.path.starts_with("examples/")
            && !doc.path.starts_with("products/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemDoc,
                doc.canonical_identity(),
                format!("invalid doc path {}", doc.path),
            ));
        }
        if doc.covers.is_empty() || doc.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemDoc,
                doc.canonical_identity(),
                "docs must bind coverage anchors and receipts",
            ));
        }
        if doc.examples.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::EcosystemDocsOnly,
                doc.canonical_identity(),
                "docs must bind executable examples",
            ));
        }
        for example in &doc.examples {
            if surface.example_by_id(example).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEcosystemDoc,
                    doc.canonical_identity(),
                    format!("unknown doc example {example}"),
                ));
            }
        }
        if !ALLOWED_STATUSES.contains(&doc.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemDoc,
                doc.canonical_identity(),
                format!("invalid doc status {}", doc.status),
            ));
        }
    }
}

fn validate_examples(surface: &SemanticEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for example in &surface.examples {
        if !ALLOWED_EXAMPLE_KINDS.contains(&example.kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemExample,
                example.canonical_identity(),
                format!("invalid example kind {}", example.kind),
            ));
        }
        if !example.path.starts_with("examples/")
            && !example.path.starts_with("fixtures/")
            && !example.path.starts_with("products/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemExample,
                example.canonical_identity(),
                format!("invalid example path {}", example.path),
            ));
        }
        if example.commands.is_empty()
            || example.proofs.is_empty()
            || example.receipts.is_empty()
            || example.rejects.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemExample,
                example.canonical_identity(),
                "examples must bind commands, proofs, receipts, and rejection assertions",
            ));
        }
        if !example
            .commands
            .iter()
            .any(|command| command == "lyra-p01-semantic-ecosystem-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemExample,
                example.canonical_identity(),
                "examples must be checkable by lyra-p01-semantic-ecosystem-check",
            ));
        }
        for command in &example.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEcosystemExample,
                    example.canonical_identity(),
                    format!("unknown semantic ecosystem example command {command}"),
                ));
            }
        }
        for proof in &example.proofs {
            if surface.proof_by_id(proof).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::EcosystemProofUnbound,
                    example.canonical_identity(),
                    format!("unknown example proof {proof}"),
                ));
            }
        }
        if !ALLOWED_STATUSES.contains(&example.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemExample,
                example.canonical_identity(),
                format!("invalid example status {}", example.status),
            ));
        }
    }
}

fn validate_proofs(surface: &SemanticEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemProof,
                proof.canonical_identity(),
                format!("invalid proof scope {}", proof.scope),
            ));
        }
        if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemProof,
                proof.canonical_identity(),
                format!("invalid proof status {}", proof.status),
            ));
        }
        for doc in &proof.docs {
            if surface.doc_by_id(doc).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::EcosystemProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof doc {doc}"),
                ));
            }
        }
        for example in &proof.examples {
            if surface.example_by_id(example).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::EcosystemProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof example {example}"),
                ));
            }
        }
        for command in &proof.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::EcosystemProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof command {command}"),
                ));
            }
        }
        if proof.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemProof,
                proof.canonical_identity(),
                "semantic ecosystem proofs must bind receipts",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|item| item == "phase_closure" || item == "global_complete")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "semantic ecosystem proof must keep P01 phase open until closure gate",
            ));
        }
    }
}

fn validate_coverage(surface: &SemanticEcosystemSurface, errors: &mut Vec<ValidationError>) {
    let mut covered = BTreeSet::new();
    for doc in &surface.docs {
        for anchor in &doc.covers {
            covered.insert(anchor.as_str());
        }
    }
    for anchor in REQUIRED_COVERAGE_ANCHORS {
        if !covered.contains(*anchor) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemDoc,
                format!("coverage:{anchor}"),
                "semantic ecosystem docs must cover canonical symbols, semantic atoms, and core IR",
            ));
        }
    }
}

fn validate_semantic_ecosystem_report(
    surface: &SemanticEcosystemSurface,
    errors: &mut Vec<ValidationError>,
) {
    let doc_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .docs
        .iter()
        .map(|doc| {
            (
                doc.id.clone(),
                doc.audience.clone(),
                doc.path.clone(),
                doc.covers.clone(),
                doc.examples.clone(),
                doc.receipts.clone(),
                doc.status.clone(),
            )
        })
        .collect();
    let example_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .examples
        .iter()
        .map(|example| {
            (
                example.id.clone(),
                example.kind.clone(),
                example.path.clone(),
                example.commands.clone(),
                example.proofs.clone(),
                example.receipts.clone(),
                example.rejects.clone(),
                example.status.clone(),
            )
        })
        .collect();
    let proof_inputs: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .proofs
        .iter()
        .map(|proof| {
            (
                proof.id.clone(),
                proof.scope.clone(),
                proof.docs.clone(),
                proof.examples.clone(),
                proof.receipts.clone(),
                proof.commands.clone(),
                proof.forbids.clone(),
                proof.status.clone(),
            )
        })
        .collect();
    let report =
        deterministic_semantic_ecosystem_suite_report(&doc_inputs, &example_inputs, &proof_inputs);
    if report.doc_count != surface.docs.len()
        || report.example_count != surface.examples.len()
        || report.proof_count != surface.proofs.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::EcosystemDriftAccepted,
            "k0_semantic_ecosystem_report",
            "semantic ecosystem report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::EcosystemDriftAccepted,
            "k0_semantic_ecosystem_report",
            "semantic ecosystem report hash must be stable fnv1a128",
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
    value
        .split(',')
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_ECOSYSTEM_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic ecosystem token {needle}"),
            ));
        }
    }
}
