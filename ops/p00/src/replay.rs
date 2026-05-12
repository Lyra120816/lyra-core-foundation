use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_replay::deterministic_replay_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_replay_model::{
    ReceiptChainLink, ReceiptReplayBinding, ReplayProof, ReplayWitness, ReplayWitnessSurface,
};

pub const P00_REPLAY_WITNESS_CONTRACT: &str = "LYRA-P00-REPLAY-WITNESS v1";

pub const REQUIRED_REPLAY_RULES: &[&str] = &[
    "proof_receipt_required",
    "canonical_preimage_required",
    "replay_witness_required",
    "receipt_chain_required",
    "witness_order_required",
    "deterministic_hash_required",
    "command_binding_required",
    "no_orphan_receipts",
    "no_mutable_replay",
    "phase_open_until_replay_proven",
];

pub const REQUIRED_REPLAY_RECEIPTS: &[&str] = &[
    "constitutional_receipt",
    "authority_receipt",
    "identity_receipt",
    "enforcement_receipt",
    "delivery_receipt",
    "challenge_receipt",
    "control_receipt",
    "owner_root_receipt",
    "benchmark_evidence_receipt",
    "public_interest_receipt",
    "canon_compliance_receipt",
    "acceptance_receipt",
    "formal_semantics_receipt",
    "canonical_model_receipt",
    "deterministic_engine_receipt",
    "falsification_receipt",
    "replay_driver_receipt",
];

pub const REQUIRED_REPLAY_WITNESSES: &[&str] = &[
    "canonical_replay",
    "authority_replay",
    "control_replay",
    "engine_replay",
    "falsification_replay",
    "receipt_chain_replay",
];

pub const REQUIRED_RECEIPT_CHAIN_LINKS: &[&str] = &[
    "constitution_to_authority",
    "authority_to_identity",
    "identity_to_enforcement",
    "enforcement_to_delivery",
    "delivery_to_challenge",
    "challenge_to_control",
    "control_to_owner_root",
    "owner_root_to_benchmark_evidence",
    "benchmark_evidence_to_public_interest",
    "public_interest_to_canon_compliance",
    "canon_compliance_to_acceptance",
    "acceptance_to_formal_semantics",
    "formal_semantics_to_canonical_model",
    "canonical_model_to_engine",
    "engine_to_falsification",
    "falsification_to_replay_driver",
];

pub const REQUIRED_REPLAY_PROOFS: &[&str] = &[
    "replay_local_execution",
    "receipt_chain_integrity",
    "witness_hash_stability",
    "p00_phase_open",
];

const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["task", "chain", "witness", "phase"];
const ALLOWED_RELATIONS: &[&str] = &["precedes", "binds", "replays", "supersedes"];

const FORBIDDEN_REPLAY_TEXT: &[(&str, ErrorCode)] = &[
    ("manual replay", ErrorCode::ReplayDriftAccepted),
    ("human replay", ErrorCode::ReplayDriftAccepted),
    ("mutable replay", ErrorCode::ReplayDriftAccepted),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("replay placeholder", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("orphan receipt", ErrorCode::OrphanReceiptBinding),
    ("hash mismatch allowed", ErrorCode::ReceiptHashMismatch),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_replay_witness_surface(
    input: &str,
) -> Result<ReplayWitnessSurface, Vec<ValidationError>> {
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
            "no replay-witness lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_REPLAY_WITNESS_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_REPLAY_WITNESS_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut receipts = Vec::new();
    let mut witnesses = Vec::new();
    let mut links = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();
    let mut seen_witnesses = BTreeSet::new();
    let mut seen_links = BTreeSet::new();
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
                    "replay rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(receipt_id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(receipt_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReplayReceipt,
                    format!("line:{line_number:03}"),
                    format!("invalid receipt identity {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReplayReceipt,
                    format!("receipt:{receipt_id}"),
                    "receipt identity must be unique",
                ));
                continue;
            }
            match parse_receipt(line_number, receipt_id, value) {
                Ok(receipt) => receipts.push(receipt),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(witness_id) = left.strip_prefix("witness:") {
            if !is_symbolic_name(witness_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReplayWitness,
                    format!("line:{line_number:03}"),
                    format!("invalid witness identity {witness_id}"),
                ));
                continue;
            }
            if !seen_witnesses.insert(witness_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReplayWitness,
                    format!("witness:{witness_id}"),
                    "witness identity must be unique",
                ));
                continue;
            }
            match parse_witness(line_number, witness_id, value) {
                Ok(witness) => witnesses.push(witness),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(link_id) = left.strip_prefix("link:") {
            if !is_symbolic_name(link_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReceiptChainBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid link identity {link_id}"),
                ));
                continue;
            }
            if !seen_links.insert(link_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReceiptChainBinding,
                    format!("link:{link_id}"),
                    "receipt chain link identity must be unique",
                ));
                continue;
            }
            match parse_link(line_number, link_id, value) {
                Ok(link) => links.push(link),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReplayProof,
                    format!("line:{line_number:03}"),
                    format!("invalid replay proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReplayProof,
                    format!("proof:{proof_id}"),
                    "replay proof identity must be unique",
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
                format!("unknown entry {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(ReplayWitnessSurface {
        header,
        phase: phase.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingPhase,
                "surface",
                "missing phase",
            )]
        })?,
        task: task.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingTask,
                "surface",
                "missing task",
            )]
        })?,
        status: status.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::InvalidReplayWitness,
                "surface",
                "missing status",
            )]
        })?,
        rules,
        receipts,
        witnesses,
        links,
        proofs,
    })
}

pub fn validate_replay_witness_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    for (token, code) in FORBIDDEN_REPLAY_TEXT {
        if input.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "surface",
                format!("forbidden replay token {token}"),
            ));
        }
    }

    match parse_replay_witness_surface(input) {
        Ok(surface) => errors.extend(validate_replay_witness_model(&surface).errors),
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

pub fn validate_replay_witness_model(surface: &ReplayWitnessSurface) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P00 got {}", surface.phase),
        ));
    }
    if surface.task != "P00-017" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P00-017 got {}", surface.task),
        ));
    }
    if !matches!(
        surface.status.as_str(),
        "working_slice" | "artifact_emitted"
    ) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayWitness,
            "status",
            format!("unsupported replay status {}", surface.status),
        ));
    }

    for required in REQUIRED_REPLAY_RULES {
        match surface.rule_value(required) {
            Some(value) if strong_required_value(value) => {}
            Some(_) => errors.push(ValidationError::reject(
                ErrorCode::MissingReplayRule,
                format!("rule:{required}"),
                "replay rule must be explicit and enforced",
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingReplayRule,
                format!("rule:{required}"),
                "missing required replay rule",
            )),
        }
    }

    let receipt_ids: BTreeSet<String> = surface
        .receipts
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let witness_ids: BTreeSet<String> = surface
        .witnesses
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let link_ids: BTreeSet<String> = surface.links.iter().map(|item| item.id.clone()).collect();

    for required in REQUIRED_REPLAY_RECEIPTS {
        if !receipt_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReplayReceipt,
                format!("receipt:{required}"),
                "missing required replay receipt binding",
            ));
        }
    }
    for required in REQUIRED_REPLAY_WITNESSES {
        if !witness_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReplayWitness,
                format!("witness:{required}"),
                "missing required replay witness",
            ));
        }
    }
    for required in REQUIRED_RECEIPT_CHAIN_LINKS {
        if !link_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReceiptChainBinding,
                format!("link:{required}"),
                "missing required receipt-chain link",
            ));
        }
    }
    for required in REQUIRED_REPLAY_PROOFS {
        if surface.proof_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReplayProof,
                format!("proof:{required}"),
                "missing required replay proof",
            ));
        }
    }

    let mut witness_orders = BTreeSet::new();
    let mut referenced_receipts = BTreeSet::new();
    for receipt in &surface.receipts {
        validate_receipt(receipt, &mut errors);
    }
    for witness in &surface.witnesses {
        validate_witness(
            witness,
            &receipt_ids,
            &mut witness_orders,
            &mut referenced_receipts,
            &mut errors,
        );
    }
    for link in &surface.links {
        validate_link(link, &receipt_ids, &mut referenced_receipts, &mut errors);
    }
    for proof in &surface.proofs {
        validate_proof(
            proof,
            &receipt_ids,
            &witness_ids,
            &link_ids,
            &mut referenced_receipts,
            &mut errors,
        );
    }

    for receipt_id in &receipt_ids {
        if !referenced_receipts.contains(receipt_id) {
            errors.push(ValidationError::reject(
                ErrorCode::OrphanReceiptBinding,
                format!("receipt:{receipt_id}"),
                "receipt must be referenced by witness, link, or proof",
            ));
        }
    }

    let report = deterministic_replay_report(
        "P00-017",
        &[
            ("constitutional_receipt", P00_REPLAY_WITNESS_CONTRACT),
            ("replay_driver_receipt", P00_REPLAY_WITNESS_CONTRACT),
        ],
        &[
            ("canonical_replay", "canonical_preimage"),
            ("receipt_chain_replay", "receipt_chain"),
        ],
    );
    if report.receipt_count != 2
        || report.witness_count != 2
        || !report.suite_hash.starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayWitness,
            "replay_report",
            "k0 replay report must deterministically hash receipts and witnesses",
        ));
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_receipt(receipt: &ReceiptReplayBinding, errors: &mut Vec<ValidationError>) {
    let location = receipt.canonical_identity();
    if !receipt_path(&receipt.path) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayReceipt,
            location.clone(),
            format!(
                "receipt path must be receipts/p00/*.receipt: {}",
                receipt.path
            ),
        ));
    }
    for (field, value) in [
        ("input_hash", &receipt.input_hash),
        ("canonical_hash", &receipt.canonical_hash),
        ("verdict_hash", &receipt.verdict_hash),
        ("receipt_hash", &receipt.receipt_hash),
    ] {
        if !hash_token(value) {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                location.clone(),
                format!("{field} must be fnv1a128-bound"),
            ));
        }
    }
    if !ALLOWED_STATUSES.contains(&receipt.status.as_str()) || receipt.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayReceipt,
            location,
            format!("unsupported receipt status {}", receipt.status),
        ));
    }
}

fn validate_witness(
    witness: &ReplayWitness,
    receipt_ids: &BTreeSet<String>,
    witness_orders: &mut BTreeSet<String>,
    referenced_receipts: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = witness.canonical_identity();
    if !stable_order_token(&witness.order) || !witness_orders.insert(witness.order.clone()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayWitness,
            location.clone(),
            format!(
                "witness order must be unique stable three-digit token: {}",
                witness.order
            ),
        ));
    }
    if weak_value(&witness.preimage) || !witness.preimage.contains("canonical") {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayWitness,
            location.clone(),
            "witness preimage must be concrete and canonical",
        ));
    }
    if !hash_token(&witness.witness_hash) {
        errors.push(ValidationError::reject(
            ErrorCode::ReceiptHashMismatch,
            location.clone(),
            "witness hash must be fnv1a128-bound",
        ));
    }
    for receipt in &witness.receipts {
        if !receipt_ids.contains(receipt) {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayProofUnbound,
                location.clone(),
                format!("witness references unknown receipt {receipt}"),
            ));
        } else {
            referenced_receipts.insert(receipt.clone());
        }
    }
    if witness.receipts.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReplayReceipt,
            location.clone(),
            "witness must bind at least one receipt",
        ));
    }
    if witness.commands.is_empty() || witness.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "witness must bind concrete command records",
        ));
    }
    if !ALLOWED_STATUSES.contains(&witness.status.as_str()) || witness.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayWitness,
            location,
            format!("unsupported witness status {}", witness.status),
        ));
    }
}

fn validate_link(
    link: &ReceiptChainLink,
    receipt_ids: &BTreeSet<String>,
    referenced_receipts: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = link.canonical_identity();
    if !receipt_ids.contains(&link.from) {
        errors.push(ValidationError::reject(
            ErrorCode::ReplayProofUnbound,
            location.clone(),
            format!("link references unknown from receipt {}", link.from),
        ));
    } else {
        referenced_receipts.insert(link.from.clone());
    }
    if !receipt_ids.contains(&link.to) {
        errors.push(ValidationError::reject(
            ErrorCode::ReplayProofUnbound,
            location.clone(),
            format!("link references unknown to receipt {}", link.to),
        ));
    } else {
        referenced_receipts.insert(link.to.clone());
    }
    if link.from == link.to {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReceiptChainBinding,
            location.clone(),
            "receipt-chain link must join distinct receipts",
        ));
    }
    if !ALLOWED_RELATIONS.contains(&link.relation.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReceiptChainBinding,
            location.clone(),
            format!("unsupported chain relation {}", link.relation),
        ));
    }
    if link.receipts.is_empty()
        || link
            .receipts
            .iter()
            .any(|receipt| !receipt_ids.contains(receipt))
    {
        errors.push(ValidationError::reject(
            ErrorCode::ReplayProofUnbound,
            location.clone(),
            "chain link proof receipts must reference known receipt bindings",
        ));
    }
    for receipt in &link.receipts {
        referenced_receipts.insert(receipt.clone());
    }
    if !ALLOWED_STATUSES.contains(&link.status.as_str()) || link.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReceiptChainBinding,
            location,
            format!("unsupported chain link status {}", link.status),
        ));
    }
}

fn validate_proof(
    proof: &ReplayProof,
    receipt_ids: &BTreeSet<String>,
    witness_ids: &BTreeSet<String>,
    link_ids: &BTreeSet<String>,
    referenced_receipts: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = proof.canonical_identity();
    if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayProof,
            location.clone(),
            format!("unsupported replay proof scope {}", proof.scope),
        ));
    }
    if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayProof,
            location.clone(),
            format!("unsupported replay proof status {}", proof.status),
        ));
    }
    if proof.scope == "phase" && proof.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "phase replay proof must remain blocked until all P00 tasks close",
        ));
    }
    for receipt in &proof.receipts {
        if !receipt_ids.contains(receipt) {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayProofUnbound,
                location.clone(),
                format!("unknown proof receipt {receipt}"),
            ));
        } else {
            referenced_receipts.insert(receipt.clone());
        }
    }
    for witness in &proof.witnesses {
        if !witness_ids.contains(witness) {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayProofUnbound,
                location.clone(),
                format!("unknown proof witness {witness}"),
            ));
        }
    }
    for link in &proof.links {
        if !link_ids.contains(link) {
            errors.push(ValidationError::reject(
                ErrorCode::ReplayProofUnbound,
                location.clone(),
                format!("unknown proof link {link}"),
            ));
        }
    }
    if proof.receipts.is_empty() || proof.witnesses.is_empty() || proof.links.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayProof,
            location.clone(),
            "replay proof must bind receipts, witnesses, and chain links",
        ));
    }
    if proof.commands.is_empty() || proof.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "replay proof must bind command records",
        ));
    }
    if proof.forbids.is_empty() || proof.forbids.iter().any(|item| weak_value(item)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayProof,
            location.clone(),
            "replay proof forbid list must be concrete",
        ));
    }
    if proof.id == "receipt_chain_integrity"
        && !proof.forbids.iter().any(|item| item == "orphan_receipt")
    {
        errors.push(ValidationError::reject(
            ErrorCode::OrphanReceiptBinding,
            location.clone(),
            "receipt-chain integrity proof must forbid orphan receipts",
        ));
    }
    if proof.id == "witness_hash_stability"
        && !proof.forbids.iter().any(|item| item == "hash_mismatch")
    {
        errors.push(ValidationError::reject(
            ErrorCode::ReceiptHashMismatch,
            location,
            "witness hash stability proof must forbid hash mismatch",
        ));
    }
}

fn parse_receipt(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReceiptReplayBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let item = ReceiptReplayBinding {
        line_number,
        id: id.to_string(),
        path: required_string_field(line_number, &mut fields, "path")?,
        input_hash: required_string_field(line_number, &mut fields, "input_hash")?,
        canonical_hash: required_string_field(line_number, &mut fields, "canonical_hash")?,
        verdict_hash: required_string_field(line_number, &mut fields, "verdict_hash")?,
        receipt_hash: required_string_field(line_number, &mut fields, "receipt_hash")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(item)
}

fn parse_witness(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReplayWitness, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let item = ReplayWitness {
        line_number,
        id: id.to_string(),
        order: required_string_field(line_number, &mut fields, "order")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        preimage: required_string_field(line_number, &mut fields, "preimage")?,
        witness_hash: required_string_field(line_number, &mut fields, "witness_hash")?,
        commands: required_list_field(line_number, &mut fields, "commands")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(item)
}

fn parse_link(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ReceiptChainLink, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let item = ReceiptChainLink {
        line_number,
        id: id.to_string(),
        from: required_string_field(line_number, &mut fields, "from")?,
        to: required_string_field(line_number, &mut fields, "to")?,
        relation: required_string_field(line_number, &mut fields, "relation")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(item)
}

fn parse_proof(line_number: usize, id: &str, value: &str) -> Result<ReplayProof, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let item = ReplayProof {
        line_number,
        id: id.to_string(),
        scope: required_string_field(line_number, &mut fields, "scope")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        witnesses: required_list_field(line_number, &mut fields, "witnesses")?,
        links: required_list_field(line_number, &mut fields, "links")?,
        commands: required_list_field(line_number, &mut fields, "commands")?,
        forbids: required_list_field(line_number, &mut fields, "forbids")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(item)
}

fn parse_fields(
    line_number: usize,
    value: &str,
) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        let Some((key, val)) = part.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field must use key:value syntax",
            ));
        };
        if key.is_empty() || val.is_empty() || key != key.trim() || val != val.trim() {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field key/value must be non-empty and trimmed",
            ));
        }
        if fields.insert(key.to_string(), val.to_string()).is_some() {
            return Err(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate field {key}"),
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
    fields.remove(key).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("missing field {key}"),
        )
    })
}

fn required_list_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<Vec<String>, ValidationError> {
    let value = required_string_field(line_number, fields, key)?;
    let values = split_list(&value);
    if values.is_empty() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("field {key} must contain at least one item"),
        ))
    } else {
        Ok(values)
    }
}

fn reject_unknown_fields(
    line_number: usize,
    fields: BTreeMap<String, String>,
) -> Result<(), ValidationError> {
    if let Some(key) = fields.keys().next() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("unknown field {key}"),
        ))
    } else {
        Ok(())
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

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value == value.trim()
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        && value.as_bytes()[0].is_ascii_lowercase()
}

fn stable_order_token(value: &str) -> bool {
    value.len() == 3 && value.as_bytes().iter().all(|byte| byte.is_ascii_digit()) && value != "000"
}

fn receipt_path(value: &str) -> bool {
    value.starts_with("receipts/p00/") && value.ends_with(".receipt")
}

fn hash_token(value: &str) -> bool {
    value.starts_with("fnv1a128:")
        && value.len() == "fnv1a128:".len() + 32
        && value.as_bytes()["fnv1a128:".len()..]
            .iter()
            .all(|byte| byte.is_ascii_hexdigit())
}

fn strong_required_value(value: &str) -> bool {
    matches!(
        value,
        "required" | "enforced" | "forbidden" | "blocked" | "receipt_bound" | "replay_required"
    )
}

fn weak_value(value: &str) -> bool {
    matches!(
        value,
        "none"
            | "nothing"
            | "declared_only"
            | "manual_only"
            | "human_only"
            | "unbound"
            | "empty"
            | "future"
            | "later"
            | "best_effort"
            | "placeholder"
            | "todo"
    )
}
