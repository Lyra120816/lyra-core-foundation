use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_delivery_model::{DeliveryArtifact, DeliveryClaim, DeliverySurface, ProofBinding};

pub const P00_DELIVERY_PROTOCOL_CONTRACT: &str = "LYRA-P00-DELIVERY-PROTOCOL v1";

pub struct RequiredDeliveryArtifact {
    pub id: &'static str,
    pub kind: &'static str,
    pub root: &'static str,
    pub path: &'static str,
}

pub const REQUIRED_DELIVERY_RULES: &[&str] = &[
    "artifact_only_pass",
    "proof_first_status",
    "receipt_before_closure",
    "truthful_status_only",
    "minimal_reflection_after_artifacts",
    "artifact_inventory_required",
    "negative_corpus_required",
    "golden_required",
    "command_record_required",
    "closure_claim_restricted",
];

pub const REQUIRED_PROOF_FAMILIES: &[&str] = &[
    "p00-execution-receipts",
    "p00-negative-corpus",
    "p00-truth-gate",
];

pub const REQUIRED_DELIVERY_ARTIFACTS: &[RequiredDeliveryArtifact] = &[
    RequiredDeliveryArtifact {
        id: "p00_delivery_runtime",
        kind: "source",
        root: "ops",
        path: "ops/p00/src/delivery.rs",
    },
    RequiredDeliveryArtifact {
        id: "p00_delivery_contract",
        kind: "contract",
        root: "interfaces",
        path: "interfaces/p00/contracts/delivery_protocol.v1.lyra",
    },
    RequiredDeliveryArtifact {
        id: "p00_delivery_model",
        kind: "source",
        root: "interfaces",
        path: "interfaces/p00/src/delivery_model.rs",
    },
    RequiredDeliveryArtifact {
        id: "p00_delivery_tests",
        kind: "test",
        root: "tests",
        path: "tests/p00_delivery_protocol_tests.rs",
    },
    RequiredDeliveryArtifact {
        id: "p00_delivery_fixture",
        kind: "fixture",
        root: "fixtures",
        path: "fixtures/p00/delivery_protocol_inputs/valid_delivery_protocol.lyra",
    },
    RequiredDeliveryArtifact {
        id: "p00_delivery_receipt",
        kind: "receipt",
        root: "receipts",
        path: "receipts/p00/pass_0005_delivery_protocol.receipt",
    },
];

const DELIVERY_ROOTS: &[&str] = &[
    "k0",
    "k1",
    "lyralang",
    "shells",
    "ops",
    "interfaces",
    "slices",
    "products",
    "fixtures",
    "goldens",
    "receipts",
    "tests",
    "src",
];

const DELIVERY_KINDS: &[&str] = &[
    "source", "contract", "fixture", "golden", "receipt", "test", "cli", "control",
];

const FORBIDDEN_DELIVERY_TEXT: &[(&str, ErrorCode)] = &[
    ("todo", ErrorCode::ForbiddenToken),
    ("tbd", ErrorCode::ForbiddenToken),
    ("not implemented", ErrorCode::ForbiddenToken),
    ("will add later", ErrorCode::ForbiddenToken),
    ("finish later", ErrorCode::ForbiddenToken),
    ("docs only", ErrorCode::DocsOnlyImplementation),
    ("documentation only", ErrorCode::DocsOnlyImplementation),
    ("artifact later", ErrorCode::MissingDeliveryArtifact),
    ("proof later", ErrorCode::MissingProofBinding),
    ("receipt later", ErrorCode::MissingReceiptProof),
    ("reflection first", ErrorCode::ReflectionBeforeArtifact),
    ("closure before receipt", ErrorCode::ClosureBeforeReceipt),
    ("complete without receipt", ErrorCode::ClosureBeforeReceipt),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_delivery_surface(input: &str) -> Result<DeliverySurface, Vec<ValidationError>> {
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
            "no delivery surface lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_DELIVERY_PROTOCOL_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_DELIVERY_PROTOCOL_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut artifacts = Vec::new();
    let mut proofs = Vec::new();
    let mut claims = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_artifacts = BTreeSet::new();
    let mut seen_proofs = BTreeSet::new();
    let mut seen_claims = BTreeSet::new();

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

        if value.is_empty() || value != value.trim() || left.is_empty() || left != left.trim() {
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
                    "delivery rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(artifact_id) = left.strip_prefix("artifact:") {
            if !is_symbolic_name(artifact_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDeliveryArtifact,
                    format!("line:{line_number:03}"),
                    format!("invalid artifact identity {artifact_id}"),
                ));
                continue;
            }
            if !seen_artifacts.insert(artifact_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeliveryArtifact,
                    format!("artifact:{artifact_id}"),
                    "delivery artifact identity must be unique",
                ));
                continue;
            }
            match parse_artifact(line_number, artifact_id, value) {
                Ok(artifact) => artifacts.push(artifact),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidProofBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateProofBinding,
                    format!("proof:{proof_id}"),
                    "proof identity must be unique",
                ));
                continue;
            }
            match parse_proof(line_number, proof_id, value) {
                Ok(proof) => proofs.push(proof),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(claim_id) = left.strip_prefix("claim:") {
            if !is_symbolic_name(claim_id) || !seen_claims.insert(claim_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "delivery claim identity must be symbolic and unique",
                ));
                continue;
            }
            match parse_claim(line_number, claim_id, value) {
                Ok(claim) => claims.push(claim),
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
                format!("unknown delivery surface field {left}"),
            )),
        }
    }

    let phase = match phase {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPhase,
                "field:phase",
                "phase=P00 is required",
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
                "task=P00-005 is required",
            ));
            String::new()
        }
    };
    let status = match status {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                "field:status",
                "status=working_slice is required",
            ));
            String::new()
        }
    };

    if errors.is_empty() {
        Ok(DeliverySurface {
            header,
            phase,
            task,
            status,
            rules,
            artifacts,
            proofs,
            claims,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_delivery_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_delivery_surface(input) {
        Ok(surface) => validate_parsed_delivery_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_artifact(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<DeliveryArtifact, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let root = required_string_field(line_number, &mut fields, "root")?;
    let path = required_string_field(line_number, &mut fields, "path")?;
    let producer = required_string_field(line_number, &mut fields, "producer")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(DeliveryArtifact {
        line_number,
        id: id.to_string(),
        kind,
        root,
        path,
        producer,
        evidence,
    })
}

fn parse_proof(line_number: usize, id: &str, value: &str) -> Result<ProofBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let family = required_string_field(line_number, &mut fields, "family")?;
    let artifacts = required_list_field(line_number, &mut fields, "artifacts")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ProofBinding {
        line_number,
        id: id.to_string(),
        family,
        artifacts,
        receipts,
        status,
    })
}

fn parse_claim(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<DeliveryClaim, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let artifacts = required_list_field(line_number, &mut fields, "artifacts")?;
    let proofs = required_list_field(line_number, &mut fields, "proofs")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(DeliveryClaim {
        line_number,
        id: id.to_string(),
        scope,
        status,
        artifacts,
        proofs,
        receipts,
        commands,
    })
}

fn parse_fields(
    line_number: usize,
    value: &str,
) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        let Some((key, field_value)) = part.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "delivery attributes must use key:value fields",
            ));
        };
        if !is_symbolic_name(key) || field_value.is_empty() || field_value != field_value.trim() {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field keys must be symbolic and values must be non-empty trimmed text",
            ));
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
            return Err(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate attribute {key}"),
            ));
        }
    }
    Ok(fields)
}

fn required_string_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<String, ValidationError> {
    match fields.remove(key) {
        Some(value) if !value.is_empty() => Ok(value),
        _ => Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("required field {key} is absent or empty"),
        )),
    }
}

fn required_list_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<Vec<String>, ValidationError> {
    let value = required_string_field(line_number, fields, key)?;
    let items = split_list(&value);
    if items.is_empty() {
        Err(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            format!("line:{line_number:03}"),
            format!("required list field {key} must not be empty"),
        ))
    } else {
        Ok(items)
    }
}

fn reject_unknown_fields(
    line_number: usize,
    fields: BTreeMap<String, String>,
) -> Result<(), ValidationError> {
    if fields.is_empty() {
        Ok(())
    } else {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "delivery surface contains unsupported attributes",
        ))
    }
}

fn split_list(value: &str) -> Vec<String> {
    let mut items: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty() && *item != "none" && *item != "nothing")
        .map(ToString::to_string)
        .collect();
    items.sort();
    items.dedup();
    items
}

fn validate_parsed_delivery_surface(surface: &DeliverySurface, raw_input: &str) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            format!("expected P00, found {}", surface.phase),
        ));
    }
    if surface.task != "P00-005" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-005, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-005 may only declare working_slice status",
        ));
    }

    for rule in REQUIRED_DELIVERY_RULES {
        match surface.rule_value(rule) {
            Some(value) if value.contains("required") => {}
            Some(_) | None => errors.push(ValidationError::reject(
                ErrorCode::MissingDeliveryRule,
                format!("rule:{rule}"),
                "required delivery rule is absent or too weak",
            )),
        }
    }

    if surface.artifacts.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingDeliveryArtifact,
            "artifact:*",
            "at least one delivery artifact is required",
        ));
    }
    for required in REQUIRED_DELIVERY_ARTIFACTS {
        match surface.artifact_by_id(required.id) {
            Some(artifact) => {
                if artifact.kind != required.kind
                    || artifact.root != required.root
                    || artifact.path != required.path
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidDeliveryArtifact,
                        format!("artifact:{}", required.id),
                        "required delivery artifact is not bound to its declared kind root path",
                    ));
                }
            }
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingDeliveryArtifact,
                format!("artifact:{}", required.id),
                "required delivery artifact is absent",
            )),
        }
    }

    let artifact_ids: BTreeSet<String> = surface
        .artifacts
        .iter()
        .map(|artifact| artifact.id.clone())
        .collect();
    for artifact in &surface.artifacts {
        validate_artifact(artifact, &mut errors);
    }

    if surface.proofs.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingProofBinding,
            "proof:*",
            "at least one proof binding is required",
        ));
    }
    for required_family in REQUIRED_PROOF_FAMILIES {
        if !surface
            .proofs
            .iter()
            .any(|proof| proof.family == *required_family)
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("proof-family:{required_family}"),
                "required proof family is not bound",
            ));
        }
    }
    for proof in &surface.proofs {
        validate_proof(proof, &artifact_ids, &mut errors);
    }

    let proof_ids: BTreeSet<String> = surface
        .proofs
        .iter()
        .map(|proof| proof.id.clone())
        .collect();
    if surface.claims.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            "claim:*",
            "at least one truthful delivery claim is required",
        ));
    }
    for claim in &surface.claims {
        validate_claim(claim, &artifact_ids, &proof_ids, &mut errors);
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_DELIVERY_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "delivery:text",
                format!("forbidden delivery phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_artifact(artifact: &DeliveryArtifact, errors: &mut Vec<ValidationError>) {
    let location = artifact.canonical_identity();
    if !DELIVERY_KINDS.contains(&artifact.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidDeliveryArtifact,
            location.clone(),
            format!("unsupported artifact kind {}", artifact.kind),
        ));
    }
    if !DELIVERY_ROOTS.contains(&artifact.root.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidDeliveryArtifact,
            location.clone(),
            format!("unsupported artifact root {}", artifact.root),
        ));
    }
    let expected_prefix = format!("{}/", artifact.root);
    if !artifact.path.starts_with(&expected_prefix) {
        errors.push(ValidationError::reject(
            ErrorCode::MisplacedOwnerRoot,
            location.clone(),
            format!("path {} is not under root {}", artifact.path, artifact.root),
        ));
    }
    if weak_delivery_value(&artifact.producer) {
        errors.push(ValidationError::reject(
            ErrorCode::UnderbuildViolation,
            location.clone(),
            "artifact must name a concrete producer",
        ));
    }
    if artifact.evidence.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            location,
            "artifact must bind evidence",
        ));
    }
}

fn validate_proof(
    proof: &ProofBinding,
    artifact_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = proof.canonical_identity();
    if !REQUIRED_PROOF_FAMILIES.contains(&proof.family.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            location.clone(),
            format!("unsupported proof family {}", proof.family),
        ));
    }
    for artifact_id in &proof.artifacts {
        if !artifact_ids.contains(artifact_id) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("unknown artifact binding {artifact_id}"),
            ));
        }
    }
    if proof
        .receipts
        .iter()
        .all(|receipt| !receipt.ends_with(".receipt"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "proof must bind at least one receipt path",
        ));
    }
    match proof.status.as_str() {
        "working_slice" | "execution_proven" | "artifact_emitted" => {}
        "closed" | "complete" | "global_complete" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location,
            "proof binding cannot close P00",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location,
            format!("unsupported proof status {other}"),
        )),
    }
}

fn validate_claim(
    claim: &DeliveryClaim,
    artifact_ids: &BTreeSet<String>,
    proof_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = claim.canonical_identity();
    match claim.scope.as_str() {
        "task" | "frontier" => {}
        "phase" | "global" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "P00-005 delivery claims cannot close phase or global scope",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            location.clone(),
            format!("unsupported claim scope {other}"),
        )),
    }
    match claim.status.as_str() {
        "working_slice" | "artifact_emitted" | "execution_proven" => {}
        "closed" | "complete" | "global_complete" => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "delivery status cannot claim closure before all P00 gates are proven",
        )),
        other => errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location.clone(),
            format!("unsupported claim status {other}"),
        )),
    }
    for artifact_id in &claim.artifacts {
        if !artifact_ids.contains(artifact_id) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("unknown claimed artifact {artifact_id}"),
            ));
        }
    }
    for proof_id in &claim.proofs {
        if !proof_ids.contains(proof_id) {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                location.clone(),
                format!("unknown claimed proof {proof_id}"),
            ));
        }
    }
    if claim
        .receipts
        .iter()
        .all(|receipt| !receipt.ends_with(".receipt"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureBeforeReceipt,
            location.clone(),
            "claim must bind receipt paths before any status claim",
        ));
    }
    if claim.commands.is_empty()
        || claim
            .commands
            .iter()
            .any(|command| weak_delivery_value(command))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location,
            "claim must bind command records",
        ));
    }
}

fn weak_delivery_value(value: &str) -> bool {
    matches!(
        value,
        "none" | "nothing" | "declared_only" | "documentation_only" | "docs_only" | "thin_patch"
    )
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value == value.trim()
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        && value.as_bytes()[0].is_ascii_lowercase()
}
