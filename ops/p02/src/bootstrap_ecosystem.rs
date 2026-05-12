use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_ecosystem::deterministic_bootstrap_ecosystem_suite_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_ecosystem::{
    bootstrap_ecosystem_artifacts_bind_paths, bootstrap_ecosystem_carrier_signature,
    bootstrap_ecosystem_doc_descriptor, bootstrap_ecosystem_doc_digest,
    bootstrap_ecosystem_docs_bind_examples, bootstrap_ecosystem_example_descriptor,
    bootstrap_ecosystem_example_digest, bootstrap_ecosystem_examples_bind_proofs,
    bootstrap_ecosystem_no_forbidden_descriptor_claims, bootstrap_ecosystem_proof_descriptor,
    bootstrap_ecosystem_proof_digest, bootstrap_ecosystem_proofs_bind_registry,
    bootstrap_ecosystem_receipts_cover_p02_001_through_p02_021, bootstrap_ecosystem_registry_hash,
    LYRA_P02_BOOTSTRAP_ECOSYSTEM_CARRIER,
};
use crate::p02_bootstrap_ecosystem_model::{
    BootstrapEcosystemDoc, BootstrapEcosystemExample, BootstrapEcosystemProof,
    BootstrapEcosystemSurface,
};

pub const P02_BOOTSTRAP_ECOSYSTEM_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-ECOSYSTEM-DOCS-EXAMPLES v1";

pub const REQUIRED_BOOTSTRAP_ECOSYSTEM_RULES: &[&str] = &[
    "bootstrap_ecosystem_docs_must_be_receipted",
    "bootstrap_examples_must_be_executable",
    "bootstrap_trust_seed_runtime_host_extinction_coverage_required",
    "offline_first_distribution",
    "ecosystem_docs_bound_to_bootstrap_deployment",
    "docs_examples_bound_to_proofs",
    "no_remote_service_dependency",
    "no_documentation_only_claim",
    "no_ecosystem_drift_acceptance",
    "phase_open_until_closure_gate",
];

pub const REQUIRED_BOOTSTRAP_ECOSYSTEM_DOCS: &[&str] = &[
    "bootstrap_trust_operator_guide",
    "seed_runtime_law_developer_reference",
    "host_extinction_contributor_onboarding",
    "offline_bootstrap_distribution_reference",
    "enterprise_bootstrap_adoption_guide",
    "public_bootstrap_review_reference",
    "bootstrap_deployment_ecosystem_walkthrough",
];

pub const REQUIRED_BOOTSTRAP_ECOSYSTEM_EXAMPLES: &[&str] = &[
    "bootstrap_trust_walkthrough",
    "seed_runtime_replacement_flow",
    "host_extinction_review",
    "offline_airgap_bootstrap_review",
    "enterprise_deployment_to_ecosystem_handoff",
    "negative_bootstrap_doc_drift_rejection",
    "phase_open_ecosystem_review",
];

pub const REQUIRED_BOOTSTRAP_ECOSYSTEM_PROOFS: &[&str] = &[
    "bootstrap_docs_coverage_proof",
    "executable_examples_proof",
    "receipt_binding_proof",
    "offline_distribution_proof",
    "deployment_ecosystem_bridge_proof",
    "p02_phase_open",
];

const REQUIRED_COVERAGE_ANCHORS: &[&str] = &[
    "bootstrap_trust",
    "seed_runtime_law",
    "host_extinction_framework",
];
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
    "lyra-p02-bootstrap-inventory-check",
    "lyra-p02-bootstrap-extinction-check",
    "lyra-p02-host-boundary-check",
    "lyra-p02-target-matrix-check",
    "lyra-p02-truth-cleanup-check",
    "lyra-p02-emergency-fallback-check",
    "lyra-p02-seed-runtime-replacement-check",
    "lyra-p02-bootstrap-evidence-emission-check",
    "lyra-p02-operator-handoff-automation-check",
    "lyra-p02-foreign-surface-closure-check",
    "lyra-p02-bootstrap-formal-semantics-check",
    "lyra-p02-bootstrap-canonical-model-check",
    "lyra-p02-bootstrap-core-engine-check",
    "lyra-p02-bootstrap-falsification-check",
    "lyra-p02-bootstrap-replay-check",
    "lyra-p02-bootstrap-interface-check",
    "lyra-p02-bootstrap-packaging-check",
    "lyra-p02-bootstrap-deployment-check",
    "lyra-p02-bootstrap-ecosystem-check",
];

const FORBIDDEN_BOOTSTRAP_ECOSYSTEM_TEXT: &[(&str, ErrorCode)] = &[
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

pub fn parse_bootstrap_ecosystem_surface(
    input: &str,
) -> Result<BootstrapEcosystemSurface, Vec<ValidationError>> {
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
            "no bootstrap ecosystem docs/examples surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_ECOSYSTEM_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_ECOSYSTEM_CONTRACT}"),
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
                    "bootstrap ecosystem rule names must be symbolic and unique",
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
                    format!("duplicate or invalid bootstrap ecosystem doc {doc_id}"),
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
                    format!("duplicate or invalid bootstrap ecosystem example {example_id}"),
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
                    format!("duplicate or invalid bootstrap ecosystem proof {proof_id}"),
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
                format!("unknown bootstrap ecosystem key {left}"),
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
    Ok(BootstrapEcosystemSurface {
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

pub fn validate_bootstrap_ecosystem_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_bootstrap_ecosystem_surface(input) {
        Ok(surface) => validate_bootstrap_ecosystem_model(&surface, &mut errors),
        Err(mut parse_errors) => errors.append(&mut parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_ecosystem_model(
    surface: &BootstrapEcosystemSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "bootstrap ecosystem law must bind to P02",
        ));
    }
    if surface.task != "P02-021" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "bootstrap ecosystem law must bind to P02-021",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported bootstrap ecosystem status {}", surface.status),
        ));
    }
    require_rules(surface, errors);
    require_ids(
        "doc",
        REQUIRED_BOOTSTRAP_ECOSYSTEM_DOCS,
        surface.docs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingEcosystemDoc,
        errors,
    );
    require_ids(
        "example",
        REQUIRED_BOOTSTRAP_ECOSYSTEM_EXAMPLES,
        surface
            .examples
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingEcosystemExample,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_BOOTSTRAP_ECOSYSTEM_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingEcosystemProof,
        errors,
    );
    validate_docs(surface, errors);
    validate_examples(surface, errors);
    validate_proofs(surface, errors);
    validate_coverage(surface, errors);
    validate_registry_bindings(errors);
    validate_bootstrap_ecosystem_report(surface, errors);
}

fn parse_doc(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapEcosystemDoc, ValidationError> {
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
    Ok(BootstrapEcosystemDoc {
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
) -> Result<BootstrapEcosystemExample, ValidationError> {
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
    Ok(BootstrapEcosystemExample {
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
) -> Result<BootstrapEcosystemProof, ValidationError> {
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
    Ok(BootstrapEcosystemProof {
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

fn require_rules(surface: &BootstrapEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_BOOTSTRAP_ECOSYSTEM_RULES {
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
                "required bootstrap ecosystem rule missing",
            )),
        }
    }
}

fn validate_docs(surface: &BootstrapEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for doc in &surface.docs {
        validate_status("doc", &doc.id, doc.line_number, &doc.status, errors);
        if !ALLOWED_AUDIENCES.contains(&doc.audience.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemDoc,
                doc.canonical_identity(),
                format!("invalid doc audience {}", doc.audience),
            ));
        }
        validate_artifact_path(
            &doc.path,
            doc.line_number,
            ErrorCode::InvalidEcosystemDoc,
            errors,
        );
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
        for receipt in &doc.receipts {
            validate_receipt_path(
                receipt,
                doc.line_number,
                ErrorCode::InvalidEcosystemDoc,
                errors,
            );
        }
        if let Some(descriptor) = bootstrap_ecosystem_doc_descriptor(&doc.id) {
            let digest = bootstrap_ecosystem_doc_digest(&doc.id).unwrap_or_default();
            if descriptor.audience != doc.audience
                || descriptor.path != doc.path
                || descriptor.covers != doc.covers.as_slice()
                || descriptor.examples != doc.examples.as_slice()
                || descriptor.receipts != doc.receipts.as_slice()
                || descriptor.status != doc.status
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::EcosystemDriftAccepted,
                    doc.canonical_identity(),
                    "doc descriptor drift",
                ));
            }
        }
    }
}

fn validate_examples(surface: &BootstrapEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for example in &surface.examples {
        validate_status(
            "example",
            &example.id,
            example.line_number,
            &example.status,
            errors,
        );
        if !ALLOWED_EXAMPLE_KINDS.contains(&example.kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemExample,
                example.canonical_identity(),
                format!("invalid example kind {}", example.kind),
            ));
        }
        validate_artifact_path(
            &example.path,
            example.line_number,
            ErrorCode::InvalidEcosystemExample,
            errors,
        );
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
            .any(|command| command == "lyra-p02-bootstrap-ecosystem-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemExample,
                example.canonical_identity(),
                "examples must be checkable by lyra-p02-bootstrap-ecosystem-check",
            ));
        }
        for command in &example.commands {
            validate_command(
                command,
                example.line_number,
                ErrorCode::InvalidEcosystemExample,
                errors,
            );
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
        for receipt in &example.receipts {
            validate_receipt_path(
                receipt,
                example.line_number,
                ErrorCode::InvalidEcosystemExample,
                errors,
            );
        }
        if let Some(descriptor) = bootstrap_ecosystem_example_descriptor(&example.id) {
            let digest = bootstrap_ecosystem_example_digest(&example.id).unwrap_or_default();
            if descriptor.kind != example.kind
                || descriptor.path != example.path
                || descriptor.commands != example.commands.as_slice()
                || descriptor.proofs != example.proofs.as_slice()
                || descriptor.receipts != example.receipts.as_slice()
                || descriptor.rejects != example.rejects.as_slice()
                || descriptor.status != example.status
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::EcosystemDriftAccepted,
                    example.canonical_identity(),
                    "example descriptor drift",
                ));
            }
        }
    }
}

fn validate_proofs(surface: &BootstrapEcosystemSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemProof,
                proof.canonical_identity(),
                format!("invalid proof scope {}", proof.scope),
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
        if proof.receipts.is_empty() || proof.commands.is_empty() || proof.forbids.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEcosystemProof,
                proof.canonical_identity(),
                "proofs must bind receipts, commands, and forbidden claims",
            ));
        }
        for receipt in &proof.receipts {
            validate_receipt_path(
                receipt,
                proof.line_number,
                ErrorCode::InvalidEcosystemProof,
                errors,
            );
        }
        for command in &proof.commands {
            validate_command(
                command,
                proof.line_number,
                ErrorCode::InvalidEcosystemProof,
                errors,
            );
        }
        if proof.id == "p02_phase_open"
            && !proof
                .forbids
                .iter()
                .any(|item| item == "phase_closure" || item == "global_complete")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "phase-open proof must forbid closure claims",
            ));
        }
        if let Some(descriptor) = bootstrap_ecosystem_proof_descriptor(&proof.id) {
            let digest = bootstrap_ecosystem_proof_digest(&proof.id).unwrap_or_default();
            if descriptor.scope != proof.scope
                || descriptor.docs != proof.docs.as_slice()
                || descriptor.examples != proof.examples.as_slice()
                || descriptor.receipts != proof.receipts.as_slice()
                || descriptor.commands != proof.commands.as_slice()
                || descriptor.forbids != proof.forbids.as_slice()
                || descriptor.status != proof.status
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::EcosystemDriftAccepted,
                    proof.canonical_identity(),
                    "proof descriptor drift",
                ));
            }
        }
    }
}

fn validate_coverage(surface: &BootstrapEcosystemSurface, errors: &mut Vec<ValidationError>) {
    let mut covered = BTreeSet::new();
    for doc in &surface.docs {
        for anchor in &doc.covers {
            covered.insert(anchor.as_str());
        }
    }
    for anchor in REQUIRED_COVERAGE_ANCHORS {
        if !covered.contains(*anchor) {
            errors.push(ValidationError::reject(ErrorCode::InvalidEcosystemDoc, format!("coverage:{anchor}"), "bootstrap ecosystem docs must cover bootstrap trust, seed runtime law, and host-extinction framework"));
        }
    }
}

fn validate_registry_bindings(errors: &mut Vec<ValidationError>) {
    if LYRA_P02_BOOTSTRAP_ECOSYSTEM_CARRIER != "lyra.p02.bootstrap_ecosystem.carrier.v1"
        || bootstrap_ecosystem_registry_hash().is_empty()
        || bootstrap_ecosystem_carrier_signature().is_empty()
    {
        errors.push(ValidationError::reject(
            ErrorCode::EcosystemDriftAccepted,
            "lyralang_bootstrap_ecosystem_registry",
            "bootstrap ecosystem carrier registry drift",
        ));
    }
    if !bootstrap_ecosystem_docs_bind_examples()
        || !bootstrap_ecosystem_examples_bind_proofs()
        || !bootstrap_ecosystem_proofs_bind_registry()
        || !bootstrap_ecosystem_artifacts_bind_paths()
        || !bootstrap_ecosystem_no_forbidden_descriptor_claims()
        || !bootstrap_ecosystem_receipts_cover_p02_001_through_p02_021()
    {
        errors.push(ValidationError::reject(
            ErrorCode::EcosystemDriftAccepted,
            "lyralang_bootstrap_ecosystem_registry",
            "bootstrap ecosystem descriptor registry is not fully bound",
        ));
    }
}

fn validate_bootstrap_ecosystem_report(
    surface: &BootstrapEcosystemSurface,
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
        deterministic_bootstrap_ecosystem_suite_report(&doc_inputs, &example_inputs, &proof_inputs);
    if report.doc_count != surface.docs.len()
        || report.example_count != surface.examples.len()
        || report.proof_count != surface.proofs.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::EcosystemDriftAccepted,
            "k0_bootstrap_ecosystem_report",
            "bootstrap ecosystem report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::EcosystemDriftAccepted,
            "k0_bootstrap_ecosystem_report",
            "bootstrap ecosystem report hash must be stable fnv1a128",
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
            format!("{kind}:{id}:line:{line_number:03}"),
            format!("unsupported status {status}"),
        ));
    }
}

fn validate_receipt_path(
    path: &str,
    line_number: usize,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    if !path.starts_with("receipts/p02/") || !path.ends_with(".receipt") || path.contains("..") {
        errors.push(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("invalid receipt path {path}"),
        ));
    }
}

fn validate_artifact_path(
    path: &str,
    line_number: usize,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    let allowed = [
        "src/",
        "ops/",
        "interfaces/",
        "goldens/",
        "receipts/",
        "products/",
        "examples/",
        "docs/",
        "fixtures/",
        "tests/",
        "shells/",
    ];
    if path.contains("..") || !allowed.iter().any(|prefix| path.starts_with(prefix)) {
        errors.push(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("invalid artifact path {path}"),
        ));
    }
}

fn validate_command(
    command: &str,
    line_number: usize,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    if !REQUIRED_COMMANDS.contains(&command) {
        errors.push(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("unknown bootstrap ecosystem command {command}"),
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
                format!("missing bootstrap ecosystem {kind} {id}"),
            ));
        }
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
    for (needle, code) in FORBIDDEN_BOOTSTRAP_ECOSYSTEM_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden bootstrap ecosystem token {needle}"),
            ));
        }
    }
}
