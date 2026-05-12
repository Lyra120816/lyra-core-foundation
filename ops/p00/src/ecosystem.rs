use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_ecosystem::deterministic_ecosystem_suite_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_ecosystem_model::{
    EcosystemDoc, EcosystemExample, EcosystemProof, EcosystemSurface,
};

pub const P00_ECOSYSTEM_CONTRACT: &str = "LYRA-P00-ECOSYSTEM-DOCS-EXAMPLES v1";

pub const REQUIRED_ECOSYSTEM_RULES: &[&str] = &[
    "ecosystem_docs_must_be_receipted",
    "examples_must_be_executable",
    "constitution_people_first_rebuild_coverage",
    "offline_first_distribution",
    "docs_examples_bound_to_proofs",
    "no_remote_service_dependency",
    "no_prose_only_claim",
    "phase_open_until_ecosystem_proven",
];

pub const REQUIRED_ECOSYSTEM_DOCS: &[&str] = &[
    "determinism_constitution_guide",
    "people_first_operator_guide",
    "rebuild_governance_guide",
    "contributor_onboarding",
    "public_review_reference",
];

pub const REQUIRED_ECOSYSTEM_EXAMPLES: &[&str] = &[
    "constitution_surface_walkthrough",
    "people_first_challenge_flow",
    "rebuild_from_receipts_flow",
    "offline_operator_review",
    "negative_drift_rejection",
];

pub const REQUIRED_ECOSYSTEM_PROOFS: &[&str] = &[
    "docs_coverage_proof",
    "executable_examples_proof",
    "receipt_binding_proof",
    "offline_distribution_proof",
    "p00_phase_open",
];

const REQUIRED_COVERAGE_ANCHORS: &[&str] = &["determinism", "people_first", "rebuild_governance"];
const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_AUDIENCES: &[&str] = &["operator", "developer", "contributor", "public", "steward"];
const ALLOWED_EXAMPLE_KINDS: &[&str] = &[
    "walkthrough",
    "challenge_flow",
    "rebuild_flow",
    "review",
    "negative",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["docs", "examples", "receipt", "distribution", "phase"];

const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p00-validate",
    "lyra-p00-authority-check",
    "lyra-p00-identity-check",
    "lyra-p00-enforcement-check",
    "lyra-p00-delivery-check",
    "lyra-p00-challenge-check",
    "lyra-p00-control-check",
    "lyra-p00-owner-root-check",
    "lyra-p00-benchmark-evidence-check",
    "lyra-p00-public-interest-check",
    "lyra-p00-canon-compliance-check",
    "lyra-p00-acceptance-check",
    "lyra-p00-formal-semantics-check",
    "lyra-p00-canonical-model-check",
    "lyra-p00-engine-check",
    "lyra-p00-falsification-check",
    "lyra-p00-replay-check",
    "lyra-p00-interface-check",
    "lyra-p00-packaging-check",
    "lyra-p00-deployment-check",
    "lyra-p00-ecosystem-check",
];

const FORBIDDEN_ECOSYSTEM_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::EcosystemNetworkDependency),
    ("cloud required", ErrorCode::EcosystemNetworkDependency),
    ("online required", ErrorCode::EcosystemNetworkDependency),
    (
        "remote service required",
        ErrorCode::EcosystemNetworkDependency,
    ),
    ("remote fetch", ErrorCode::EcosystemNetworkDependency),
    ("manual only", ErrorCode::EcosystemDocsOnly),
    ("docs only", ErrorCode::EcosystemDocsOnly),
    ("docs_only", ErrorCode::EcosystemDocsOnly),
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

pub fn parse_ecosystem_surface(input: &str) -> Result<EcosystemSurface, Vec<ValidationError>> {
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
            "no ecosystem docs/examples surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_ECOSYSTEM_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_ECOSYSTEM_CONTRACT}"),
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
                    "ecosystem rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(doc_id) = left.strip_prefix("doc:") {
            if !is_symbolic_name(doc_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEcosystemDoc,
                    format!("line:{line_number:03}"),
                    format!("invalid ecosystem doc identity {doc_id}"),
                ));
                continue;
            }
            if !seen_docs.insert(doc_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEcosystemDoc,
                    format!("doc:{doc_id}"),
                    "ecosystem doc identity must be unique",
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
            if !is_symbolic_name(example_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEcosystemExample,
                    format!("line:{line_number:03}"),
                    format!("invalid ecosystem example identity {example_id}"),
                ));
                continue;
            }
            if !seen_examples.insert(example_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEcosystemExample,
                    format!("example:{example_id}"),
                    "ecosystem example identity must be unique",
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
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEcosystemProof,
                    format!("line:{line_number:03}"),
                    format!("invalid ecosystem proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEcosystemProof,
                    format!("proof:{proof_id}"),
                    "ecosystem proof identity must be unique",
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
                format!("unknown ecosystem key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(EcosystemSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        docs,
        examples,
        proofs,
    })
}

pub fn validate_ecosystem_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_ecosystem_surface(input) {
        Ok(surface) => errors.extend(validate_ecosystem_model(&surface).errors),
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

pub fn validate_ecosystem_model(surface: &EcosystemSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "ecosystem docs/examples law must bind to P00",
        ));
    }
    if surface.task != "P00-021" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "ecosystem docs/examples law must bind to P00-021",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported ecosystem status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_docs(surface, &mut errors);
    require_examples(surface, &mut errors);
    require_proofs(surface, &mut errors);
    validate_docs(surface, &mut errors);
    validate_examples(surface, &mut errors);
    validate_proofs(surface, &mut errors);
    validate_coverage(surface, &mut errors);
    validate_ecosystem_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_doc(line_number: usize, id: &str, value: &str) -> Result<EcosystemDoc, ValidationError> {
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
    Ok(EcosystemDoc {
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
) -> Result<EcosystemExample, ValidationError> {
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
    Ok(EcosystemExample {
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
) -> Result<EcosystemProof, ValidationError> {
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
    Ok(EcosystemProof {
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

fn require_rules(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_ECOSYSTEM_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("blocked_until_proven") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemRule,
                format!("rule:{rule}"),
                "required ecosystem docs/examples rule missing",
            )),
        }
    }
}

fn require_docs(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_ECOSYSTEM_DOCS {
        if surface.doc_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemDoc,
                format!("doc:{id}"),
                "required ecosystem doc missing",
            ));
        }
    }
}

fn require_examples(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_ECOSYSTEM_EXAMPLES {
        if surface.example_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemExample,
                format!("example:{id}"),
                "required ecosystem example missing",
            ));
        }
    }
}

fn require_proofs(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_ECOSYSTEM_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEcosystemProof,
                format!("proof:{id}"),
                "required ecosystem proof missing",
            ));
        }
    }
}

fn validate_docs(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
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

fn validate_examples(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
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
            .any(|command| command == "lyra-p00-ecosystem-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemExample,
                example.canonical_identity(),
                "examples must be checkable by lyra-p00-ecosystem-check",
            ));
        }
        for command in &example.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEcosystemExample,
                    example.canonical_identity(),
                    format!("unknown ecosystem example command {command}"),
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

fn validate_proofs(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
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
                "ecosystem proofs must bind receipts",
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
                "ecosystem proof must keep P00 phase open until closure gate",
            ));
        }
    }
}

fn validate_coverage(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
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
                "ecosystem docs must cover determinism, people-first law, and rebuild governance",
            ));
        }
    }
}

fn validate_ecosystem_report(surface: &EcosystemSurface, errors: &mut Vec<ValidationError>) {
    let doc_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
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
            )
        })
        .collect();
    let report =
        deterministic_ecosystem_suite_report(&doc_inputs, &example_inputs, surface.proofs.len());
    if report.doc_count != surface.docs.len() || report.example_count != surface.examples.len() {
        errors.push(ValidationError::reject(
            ErrorCode::EcosystemDriftAccepted,
            "k0_ecosystem_report",
            "ecosystem report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::EcosystemDriftAccepted,
            "k0_ecosystem_report",
            "ecosystem report hash must be stable fnv1a128",
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
    for (needle, code) in FORBIDDEN_ECOSYSTEM_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden ecosystem token {needle}"),
            ));
        }
    }
}
