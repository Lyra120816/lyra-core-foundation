use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_replay::deterministic_semantic_replay_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_replay::{
    semantic_replay_artifact_descriptor, semantic_replay_artifact_digest,
    semantic_replay_artifacts_bind_paths, semantic_replay_link_descriptor,
    semantic_replay_link_digest, semantic_replay_links_bind_known_receipts,
    semantic_replay_no_forbidden_descriptor_claims, semantic_replay_proof_descriptor,
    semantic_replay_proof_digest, semantic_replay_proofs_bind_registry,
    semantic_replay_receipt_descriptor, semantic_replay_receipt_digest,
    semantic_replay_receipts_cover_p01_001_through_p01_017, semantic_replay_registry_hash,
    semantic_replay_witness_descriptor, semantic_replay_witness_digest,
    semantic_replay_witnesses_bind_known_receipts,
};
use crate::p01_semantic_replay_model::{
    SemanticReplayArtifactBinding, SemanticReplayChainLinkBinding, SemanticReplayProofBinding,
    SemanticReplayReceiptBinding, SemanticReplaySurface, SemanticReplayWitnessBinding,
};

pub const P01_SEMANTIC_REPLAY_CONTRACT: &str = "LYRA-P01-SEMANTIC-REPLAY-WITNESS v1";

pub const REQUIRED_SEMANTIC_REPLAY_RULES: &[&str] = &[
    "semantic_receipt_chain_required",
    "semantic_replay_witness_required",
    "canonical_symbol_receipt_required",
    "semantic_atom_receipt_required",
    "core_ir_receipt_required",
    "deterministic_replay_hash_required",
    "receipt_hash_parity_required",
    "command_binding_required",
    "no_orphan_semantic_receipts",
    "no_mutable_semantic_replay",
    "no_network_replay",
    "no_probabilistic_replay",
    "no_phase_closure_claim",
];

pub const REQUIRED_SEMANTIC_REPLAY_RECEIPTS: &[&str] = &[
    "semantic_atoms_receipt",
    "core_ir_receipt",
    "semantic_objects_receipt",
    "semantic_identity_receipt",
    "reference_semantics_receipt",
    "symbolic_equality_receipt",
    "error_challenge_evidence_receipt",
    "semantic_serialization_hashing_receipt",
    "semantic_adversarial_corpus_receipt",
    "core_ir_reuse_receipt",
    "semantic_atom_reference_receipt",
    "semantic_bedrock_receipts_receipt",
    "formal_semantic_constitution_receipt",
    "canonical_data_model_receipt",
    "semantic_core_engine_receipt",
    "semantic_falsification_receipt",
    "semantic_replay_receipt",
];

pub const REQUIRED_SEMANTIC_REPLAY_WITNESSES: &[&str] = &[
    "canonical_symbols_replay",
    "semantic_atoms_replay",
    "core_ir_replay",
    "semantic_object_model_replay",
    "semantic_core_engine_replay",
    "semantic_falsification_replay",
    "p01_semantic_receipt_chain_replay",
];

pub const REQUIRED_SEMANTIC_REPLAY_LINKS: &[&str] = &[
    "semantic_atoms_to_core_ir",
    "core_ir_to_semantic_objects",
    "semantic_objects_to_semantic_identity",
    "semantic_identity_to_reference_semantics",
    "reference_semantics_to_symbolic_equality",
    "symbolic_equality_to_error_challenge_evidence",
    "error_challenge_evidence_to_semantic_serialization_hashing",
    "semantic_serialization_hashing_to_semantic_adversarial_corpus",
    "semantic_adversarial_corpus_to_core_ir_reuse",
    "core_ir_reuse_to_semantic_atom_reference",
    "semantic_atom_reference_to_semantic_bedrock_receipts",
    "semantic_bedrock_receipts_to_formal_semantic_constitution",
    "formal_semantic_constitution_to_canonical_data_model",
    "canonical_data_model_to_semantic_core_engine",
    "semantic_core_engine_to_semantic_falsification",
    "semantic_falsification_to_semantic_replay",
];

pub const REQUIRED_SEMANTIC_REPLAY_PROOFS: &[&str] = &[
    "canonical_symbols_replay_proof",
    "semantic_atoms_replay_proof",
    "core_ir_replay_proof",
    "p01_semantic_receipt_chain_integrity",
    "semantic_witness_hash_stability",
];

pub const REQUIRED_SEMANTIC_REPLAY_ARTIFACTS: &[&str] = &[
    "semantic_replay_contract",
    "semantic_replay_law",
    "semantic_replay_operator",
    "valid_semantic_replay_fixture",
    "golden_semantic_replay_receipt",
    "execution_semantic_replay_receipt",
    "deterministic_semantic_replay_report",
];

const ALLOWED_STATUSES: &[&str] = &[
    "artifact_emitted",
    "execution_proven",
    "working_slice",
    "blocked",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["domain", "chain", "witness", "task", "phase"];
const ALLOWED_RELATIONS: &[&str] = &["precedes", "binds", "replays"];
const ALLOWED_OWNER_ROOTS: &[&str] = &[
    "lyralang",
    "interfaces",
    "k0",
    "ops",
    "src",
    "fixtures",
    "goldens",
    "receipts",
    "tests",
    "docs",
    "examples",
    "products",
];
const FORBIDDEN_REPLAY_TEXT: &[(&str, ErrorCode)] = &[
    ("mutable replay allowed", ErrorCode::ReplayDriftAccepted),
    ("network replay", ErrorCode::AmbientNetworkAllowed),
    ("cloud replay", ErrorCode::AmbientNetworkAllowed),
    ("remote replay", ErrorCode::AmbientNetworkAllowed),
    ("probabilistic replay", ErrorCode::ProbabilisticTruthAllowed),
    ("random replay", ErrorCode::HiddenRandomnessAllowed),
    ("hash mismatch allowed", ErrorCode::ReceiptHashMismatch),
    ("orphan semantic receipt", ErrorCode::OrphanReceiptBinding),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_replay_surface(
    input: &str,
) -> Result<SemanticReplaySurface, Vec<ValidationError>> {
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
            "empty semantic replay surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P01_SEMANTIC_REPLAY_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_REPLAY_CONTRACT}"),
        ));
    }

    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut receipts = Vec::new();
    let mut witnesses = Vec::new();
    let mut links = Vec::new();
    let mut proofs = Vec::new();
    let mut artifacts = Vec::new();

    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();
    let mut seen_witnesses = BTreeSet::new();
    let mut seen_links = BTreeSet::new();
    let mut seen_proofs = BTreeSet::new();
    let mut seen_artifacts = BTreeSet::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
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
                    "rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if left == "receipt" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id",
                    "path",
                    "input_hash",
                    "canonical_hash",
                    "verdict_hash",
                    "receipt_hash",
                    "status",
                ],
                "receipt",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_receipts.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReplayReceipt,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic replay receipt {id}"),
                ));
            }
            receipts.push(SemanticReplayReceiptBinding {
                line_number,
                id,
                path: field(&fields, "path"),
                input_hash: field(&fields, "input_hash"),
                canonical_hash: field(&fields, "canonical_hash"),
                verdict_hash: field(&fields, "verdict_hash"),
                receipt_hash: field(&fields, "receipt_hash"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "witness" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id",
                    "order",
                    "receipts",
                    "preimage",
                    "witness_hash",
                    "commands",
                    "status",
                ],
                "witness",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_witnesses.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReplayWitness,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic replay witness {id}"),
                ));
            }
            witnesses.push(SemanticReplayWitnessBinding {
                line_number,
                id,
                order: field(&fields, "order"),
                receipts: list_field(&fields, "receipts"),
                preimage: field(&fields, "preimage"),
                witness_hash: field(&fields, "witness_hash"),
                commands: list_field(&fields, "commands"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "link" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &["id", "from", "to", "relation", "receipts", "status"],
                "link",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_links.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReceiptChainBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic replay link {id}"),
                ));
            }
            links.push(SemanticReplayChainLinkBinding {
                line_number,
                id,
                from: field(&fields, "from"),
                to: field(&fields, "to"),
                relation: field(&fields, "relation"),
                receipts: list_field(&fields, "receipts"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "proof" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id",
                    "scope",
                    "receipts",
                    "witnesses",
                    "links",
                    "commands",
                    "forbids",
                    "status",
                ],
                "proof",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_proofs.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReplayProof,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic replay proof {id}"),
                ));
            }
            proofs.push(SemanticReplayProofBinding {
                line_number,
                id,
                scope: field(&fields, "scope"),
                receipts: list_field(&fields, "receipts"),
                witnesses: list_field(&fields, "witnesses"),
                links: list_field(&fields, "links"),
                commands: list_field(&fields, "commands"),
                forbids: list_field(&fields, "forbids"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "artifact" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &["id", "owner", "path", "kind", "status"],
                "artifact",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_artifacts.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic replay artifact {id}"),
                ));
            }
            artifacts.push(SemanticReplayArtifactBinding {
                line_number,
                id,
                owner_root: field(&fields, "owner"),
                path: field(&fields, "path"),
                artifact_kind: field(&fields, "kind"),
                status: field(&fields, "status"),
            });
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
                format!("unknown entry {left}"),
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

    Ok(SemanticReplaySurface {
        header,
        phase: phase.unwrap(),
        task: task.unwrap(),
        status: status.unwrap(),
        rules,
        receipts,
        witnesses,
        links,
        proofs,
        artifacts,
    })
}

pub fn validate_semantic_replay_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_semantic_replay_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_semantic_replay(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_replay(surface: &SemanticReplaySurface, errors: &mut Vec<ValidationError>) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-017" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-017 got {}", surface.task),
        ));
    }
    validate_status("surface", "P01-017", 0, &surface.status, errors);

    for rule in REQUIRED_SEMANTIC_REPLAY_RULES {
        match surface.rules.get(*rule) {
            Some(value) if value == "required" || value == "forbidden" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingReplayRule,
                format!("rule:{rule}"),
                format!("expected required/forbidden got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingReplayRule,
                format!("rule:{rule}"),
                "missing semantic replay rule",
            )),
        }
    }

    require_ids(
        "receipt",
        REQUIRED_SEMANTIC_REPLAY_RECEIPTS,
        surface
            .receipts
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingReplayReceipt,
        errors,
    );
    require_ids(
        "witness",
        REQUIRED_SEMANTIC_REPLAY_WITNESSES,
        surface
            .witnesses
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingReplayWitness,
        errors,
    );
    require_ids(
        "link",
        REQUIRED_SEMANTIC_REPLAY_LINKS,
        surface.links.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingReceiptChainBinding,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_SEMANTIC_REPLAY_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingReplayProof,
        errors,
    );
    require_ids(
        "artifact",
        REQUIRED_SEMANTIC_REPLAY_ARTIFACTS,
        surface
            .artifacts
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingDeliveryArtifact,
        errors,
    );

    let receipt_ids: BTreeSet<&str> = surface
        .receipts
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let witness_ids: BTreeSet<&str> = surface
        .witnesses
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let link_ids: BTreeSet<&str> = surface.links.iter().map(|item| item.id.as_str()).collect();
    let mut referenced_receipts = BTreeSet::new();
    let mut witness_orders = BTreeSet::new();

    for receipt in &surface.receipts {
        validate_status(
            "receipt",
            &receipt.id,
            receipt.line_number,
            &receipt.status,
            errors,
        );
        if !receipt.path.starts_with("receipts/p01/")
            || !receipt.path.ends_with(".receipt")
            || receipt.path.contains("..")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayReceipt,
                format!("line:{:03}", receipt.line_number),
                format!("receipt {} path is invalid", receipt.id),
            ));
        }
        for (field_name, field_value) in [
            ("input_hash", &receipt.input_hash),
            ("canonical_hash", &receipt.canonical_hash),
            ("verdict_hash", &receipt.verdict_hash),
            ("receipt_hash", &receipt.receipt_hash),
        ] {
            if !is_hash(field_value) {
                errors.push(ValidationError::reject(
                    ErrorCode::ReceiptHashMismatch,
                    format!("line:{:03}", receipt.line_number),
                    format!("receipt {} has invalid {field_name}", receipt.id),
                ));
            }
        }
        let Some(descriptor) = semantic_replay_receipt_descriptor(&receipt.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayReceipt,
                format!("line:{:03}", receipt.line_number),
                format!("unknown semantic replay receipt {}", receipt.id),
            ));
            continue;
        };
        if receipt.path != descriptor.path
            || receipt.input_hash != descriptor.input_hash
            || receipt.canonical_hash != descriptor.canonical_hash
            || receipt.verdict_hash != descriptor.verdict_hash
            || receipt.receipt_hash != descriptor.receipt_hash
            || receipt.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayDriftAccepted,
                format!("line:{:03}", receipt.line_number),
                format!("receipt descriptor drift {}", receipt.id),
            ));
        }
        if semantic_replay_receipt_digest(&receipt.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayReceipt,
                format!("line:{:03}", receipt.line_number),
                format!("receipt {} is not digestible", receipt.id),
            ));
        }
    }

    for witness in &surface.witnesses {
        validate_status(
            "witness",
            &witness.id,
            witness.line_number,
            &witness.status,
            errors,
        );
        if witness.order.len() != 3
            || witness.order.chars().any(|ch| !ch.is_ascii_digit())
            || !witness_orders.insert(witness.order.clone())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayWitness,
                format!("line:{:03}", witness.line_number),
                format!("witness {} order is invalid or duplicated", witness.id),
            ));
        }
        if witness.receipts.is_empty()
            || witness.commands.is_empty()
            || witness.preimage.is_empty()
            || !is_hash(&witness.witness_hash)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayWitness,
                format!("line:{:03}", witness.line_number),
                format!("witness {} has invalid replay binding", witness.id),
            ));
        }
        for receipt_id in &witness.receipts {
            referenced_receipts.insert(receipt_id.as_str());
            if !receipt_ids.contains(receipt_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ReplayProofUnbound,
                    format!("line:{:03}", witness.line_number),
                    format!(
                        "witness {} references unknown receipt {}",
                        witness.id, receipt_id
                    ),
                ));
            }
        }
        let Some(descriptor) = semantic_replay_witness_descriptor(&witness.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayWitness,
                format!("line:{:03}", witness.line_number),
                format!("unknown semantic replay witness {}", witness.id),
            ));
            continue;
        };
        if witness.order != descriptor.order
            || witness.receipts
                != descriptor
                    .receipts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || witness.preimage != descriptor.preimage
            || witness.witness_hash != descriptor.witness_hash
            || witness.commands
                != descriptor
                    .commands
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || witness.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayDriftAccepted,
                format!("line:{:03}", witness.line_number),
                format!("witness descriptor drift {}", witness.id),
            ));
        }
        if semantic_replay_witness_digest(&witness.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayWitness,
                format!("line:{:03}", witness.line_number),
                format!("witness {} is not digestible", witness.id),
            ));
        }
    }

    for link in &surface.links {
        validate_status("link", &link.id, link.line_number, &link.status, errors);
        if !ALLOWED_RELATIONS.contains(&link.relation.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                format!("line:{:03}", link.line_number),
                format!("link {} has invalid relation {}", link.id, link.relation),
            ));
        }
        if !receipt_ids.contains(link.from.as_str()) || !receipt_ids.contains(link.to.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                format!("line:{:03}", link.line_number),
                format!("link {} has unknown endpoint", link.id),
            ));
        }
        for receipt_id in &link.receipts {
            referenced_receipts.insert(receipt_id.as_str());
            if !receipt_ids.contains(receipt_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ReplayProofUnbound,
                    format!("line:{:03}", link.line_number),
                    format!("link {} references unknown receipt {}", link.id, receipt_id),
                ));
            }
        }
        let Some(descriptor) = semantic_replay_link_descriptor(&link.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                format!("line:{:03}", link.line_number),
                format!("unknown semantic replay link {}", link.id),
            ));
            continue;
        };
        if link.from != descriptor.from
            || link.to != descriptor.to
            || link.relation != descriptor.relation
            || link.receipts
                != descriptor
                    .receipts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || link.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayDriftAccepted,
                format!("line:{:03}", link.line_number),
                format!("link descriptor drift {}", link.id),
            ));
        }
        if semantic_replay_link_digest(&link.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                format!("line:{:03}", link.line_number),
                format!("link {} is not digestible", link.id),
            ));
        }
    }

    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} has invalid scope {}", proof.id, proof.scope),
            ));
        }
        if proof.receipts.is_empty()
            || proof.witnesses.is_empty()
            || proof.commands.is_empty()
            || proof.forbids.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} has empty binding set", proof.id),
            ));
        }
        for receipt_id in &proof.receipts {
            referenced_receipts.insert(receipt_id.as_str());
            if !receipt_ids.contains(receipt_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ReplayProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!(
                        "proof {} references unknown receipt {}",
                        proof.id, receipt_id
                    ),
                ));
            }
        }
        for witness_id in &proof.witnesses {
            if !witness_ids.contains(witness_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ReplayProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!(
                        "proof {} references unknown witness {}",
                        proof.id, witness_id
                    ),
                ));
            }
        }
        for link_id in &proof.links {
            if !link_ids.contains(link_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ReplayProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown link {}", proof.id, link_id),
                ));
            }
        }
        let Some(descriptor) = semantic_replay_proof_descriptor(&proof.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayProof,
                format!("line:{:03}", proof.line_number),
                format!("unknown semantic replay proof {}", proof.id),
            ));
            continue;
        };
        if proof.scope != descriptor.scope
            || proof.receipts
                != descriptor
                    .receipts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.witnesses
                != descriptor
                    .witnesses
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.links
                != descriptor
                    .links
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.commands
                != descriptor
                    .commands
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.forbids
                != descriptor
                    .forbids
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayDriftAccepted,
                format!("line:{:03}", proof.line_number),
                format!("proof descriptor drift {}", proof.id),
            ));
        }
        if semantic_replay_proof_digest(&proof.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} is not digestible", proof.id),
            ));
        }
    }

    for artifact in &surface.artifacts {
        validate_status(
            "artifact",
            &artifact.id,
            artifact.line_number,
            &artifact.status,
            errors,
        );
        if !ALLOWED_OWNER_ROOTS.contains(&artifact.owner_root.as_str())
            || artifact.path.contains("..")
            || artifact.path.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} has invalid owner root or path", artifact.id),
            ));
        }
        let Some(descriptor) = semantic_replay_artifact_descriptor(&artifact.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                format!("line:{:03}", artifact.line_number),
                format!("unknown semantic replay artifact {}", artifact.id),
            ));
            continue;
        };
        if artifact.owner_root != descriptor.owner_root
            || artifact.path != descriptor.path
            || artifact.artifact_kind != descriptor.artifact_kind
            || artifact.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayDriftAccepted,
                format!("line:{:03}", artifact.line_number),
                format!("artifact descriptor drift {}", artifact.id),
            ));
        }
        if semantic_replay_artifact_digest(&artifact.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} is not digestible", artifact.id),
            ));
        }
    }

    for receipt in &surface.receipts {
        if !referenced_receipts.contains(receipt.id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::OrphanReceiptBinding,
                receipt.canonical_identity(),
                "semantic replay receipt is not bound by witness/link/proof",
            ));
        }
    }

    let suite = deterministic_semantic_replay_suite_report(
        &surface
            .receipts
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.path.clone(),
                    item.input_hash.clone(),
                    item.canonical_hash.clone(),
                    item.verdict_hash.clone(),
                    item.receipt_hash.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .witnesses
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.order.clone(),
                    item.receipts.clone(),
                    item.preimage.clone(),
                    item.witness_hash.clone(),
                    item.commands.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .links
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.from.clone(),
                    item.to.clone(),
                    item.relation.clone(),
                    item.receipts.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .proofs
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.scope.clone(),
                    item.receipts.clone(),
                    item.witnesses.clone(),
                    item.links.clone(),
                    item.commands.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .artifacts
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.owner_root.clone(),
                    item.path.clone(),
                    item.artifact_kind.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
    );
    if suite.receipt_count != REQUIRED_SEMANTIC_REPLAY_RECEIPTS.len()
        || suite.witness_count != REQUIRED_SEMANTIC_REPLAY_WITNESSES.len()
        || suite.link_count != REQUIRED_SEMANTIC_REPLAY_LINKS.len()
        || suite.proof_count != REQUIRED_SEMANTIC_REPLAY_PROOFS.len()
        || suite.artifact_count != REQUIRED_SEMANTIC_REPLAY_ARTIFACTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ReplayDriftAccepted,
            "deterministic_report",
            "semantic replay deterministic report count drift",
        ));
    }
    if semantic_replay_registry_hash().is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayProof,
            "registry",
            "semantic replay registry is not hashable",
        ));
    }
    if !semantic_replay_witnesses_bind_known_receipts()
        || !semantic_replay_links_bind_known_receipts()
        || !semantic_replay_proofs_bind_registry()
        || !semantic_replay_artifacts_bind_paths()
        || !semantic_replay_receipts_cover_p01_001_through_p01_017()
        || !semantic_replay_no_forbidden_descriptor_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ReplayProofUnbound,
            "registry",
            "semantic replay registry binding check failed",
        ));
    }
}

fn parse_pipe_fields(value: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        if let Some((key, val)) = part.split_once(':') {
            fields.insert(key.to_string(), val.to_string());
        }
    }
    fields
}

fn require_fields(
    fields: &BTreeMap<String, String>,
    names: &[&str],
    kind: &str,
    line_number: usize,
    errors: &mut Vec<ValidationError>,
) {
    for name in names {
        if !fields.contains_key(*name)
            || fields
                .get(*name)
                .map(|value| value.is_empty())
                .unwrap_or(true)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("{kind} missing field {name}"),
            ));
        }
    }
}

fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    fields.get(name).cloned().unwrap_or_default()
}

fn list_field(fields: &BTreeMap<String, String>, name: &str) -> Vec<String> {
    fields
        .get(name)
        .map(|value| {
            value
                .split(',')
                .filter(|item| !item.is_empty())
                .map(|item| item.to_string())
                .collect()
        })
        .unwrap_or_default()
}

fn require_ids(
    kind: &str,
    required: &[&str],
    actual: Vec<&str>,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    let actual: BTreeSet<&str> = actual.into_iter().collect();
    for id in required {
        if !actual.contains(id) {
            errors.push(ValidationError::reject(
                code,
                format!("{kind}:{id}"),
                format!("missing required semantic replay {kind} {id}"),
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
        let location = if line_number == 0 {
            kind.to_string()
        } else {
            format!("line:{line_number:03}")
        };
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            location,
            format!("{kind} {id} has invalid status {status}"),
        ));
    }
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_hash(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("fnv1a128:") else {
        return false;
    };
    rest.len() == 32 && rest.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_REPLAY_TEXT {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "semantic_replay_text",
                format!("forbidden replay text {needle}"),
            ));
        }
    }
}
