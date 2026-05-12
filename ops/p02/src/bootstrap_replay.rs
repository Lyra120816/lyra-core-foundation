use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_replay::deterministic_bootstrap_replay_suite_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_replay::{
    bootstrap_replay_artifact_descriptor, bootstrap_replay_artifact_digest,
    bootstrap_replay_artifact_ids, bootstrap_replay_artifacts_bind_paths,
    bootstrap_replay_carrier_signature, bootstrap_replay_link_descriptor,
    bootstrap_replay_link_digest, bootstrap_replay_link_ids,
    bootstrap_replay_links_bind_known_receipts, bootstrap_replay_no_forbidden_descriptor_claims,
    bootstrap_replay_proof_descriptor, bootstrap_replay_proof_digest, bootstrap_replay_proof_ids,
    bootstrap_replay_proofs_bind_registry, bootstrap_replay_receipt_descriptor,
    bootstrap_replay_receipt_digest, bootstrap_replay_receipt_ids,
    bootstrap_replay_receipts_cover_p02_001_through_p02_017, bootstrap_replay_registry_hash,
    bootstrap_replay_witness_descriptor, bootstrap_replay_witness_digest,
    bootstrap_replay_witness_ids, bootstrap_replay_witnesses_bind_known_receipts,
    LYRA_P02_BOOTSTRAP_REPLAY_CARRIER,
};
use crate::p02_bootstrap_replay_model::{
    BootstrapReplayArtifactBinding, BootstrapReplayChainLinkBinding, BootstrapReplayProofBinding,
    BootstrapReplayReceiptBinding, BootstrapReplaySurface, BootstrapReplayWitnessBinding,
};

pub const P02_BOOTSTRAP_REPLAY_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-REPLAY-WITNESS v1";

pub const REQUIRED_BOOTSTRAP_REPLAY_RULES: &[&str] = &[
    "bootstrap_receipt_chain_required",
    "bootstrap_replay_witness_required",
    "bootstrap_trust_receipt_required",
    "seed_runtime_law_receipt_required",
    "host_extinction_receipt_required",
    "deterministic_bootstrap_replay_hash_required",
    "receipt_hash_parity_required",
    "command_binding_required",
    "no_orphan_bootstrap_receipts",
    "no_mutable_bootstrap_replay",
    "no_network_replay",
    "no_probabilistic_replay",
    "no_ambient_time_replay",
    "no_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_REPLAY_RECEIPTS: &[&str] = &[
    "bootstrap_surface_inventory_receipt",
    "bootstrap_extinction_ledger_receipt",
    "seed_runtime_contracts_receipt",
    "bootstrap_session_rituals_receipt",
    "host_boundary_challenge_receipt",
    "bootstrap_target_matrix_receipt",
    "bootstrap_truth_cleanup_receipt",
    "bootstrap_emergency_fallback_receipt",
    "seed_runtime_replacement_milestones_receipt",
    "bootstrap_evidence_emission_receipt",
    "operator_handoff_automation_receipt",
    "foreign_surface_closure_receipt",
    "bootstrap_formal_semantics_receipt",
    "bootstrap_canonical_model_receipt",
    "bootstrap_core_engine_receipt",
    "bootstrap_falsification_receipt",
    "bootstrap_replay_receipt",
];
pub const REQUIRED_BOOTSTRAP_REPLAY_WITNESSES: &[&str] = &[
    "bootstrap_trust_replay",
    "seed_runtime_law_replay",
    "host_extinction_replay",
    "operator_handoff_replay",
    "bootstrap_engine_replay",
    "fallback_receipt_replay",
    "p02_bootstrap_receipt_chain_replay",
];
pub const REQUIRED_BOOTSTRAP_REPLAY_LINKS: &[&str] = &[
    "bootstrap_surface_inventory_to_bootstrap_extinction_ledger",
    "bootstrap_extinction_ledger_to_seed_runtime_contracts",
    "seed_runtime_contracts_to_bootstrap_session_rituals",
    "bootstrap_session_rituals_to_host_boundary_challenge",
    "host_boundary_challenge_to_bootstrap_target_matrix",
    "bootstrap_target_matrix_to_bootstrap_truth_cleanup",
    "bootstrap_truth_cleanup_to_bootstrap_emergency_fallback",
    "bootstrap_emergency_fallback_to_seed_runtime_replacement_milestones",
    "seed_runtime_replacement_milestones_to_bootstrap_evidence_emission",
    "bootstrap_evidence_emission_to_operator_handoff_automation",
    "operator_handoff_automation_to_foreign_surface_closure",
    "foreign_surface_closure_to_bootstrap_formal_semantics",
    "bootstrap_formal_semantics_to_bootstrap_canonical_model",
    "bootstrap_canonical_model_to_bootstrap_core_engine",
    "bootstrap_core_engine_to_bootstrap_falsification",
    "bootstrap_falsification_to_bootstrap_replay",
];
pub const REQUIRED_BOOTSTRAP_REPLAY_PROOFS: &[&str] = &[
    "bootstrap_trust_replay_proof",
    "seed_runtime_law_replay_proof",
    "host_extinction_replay_proof",
    "p02_bootstrap_receipt_chain_integrity",
    "bootstrap_witness_hash_stability",
];
pub const REQUIRED_BOOTSTRAP_REPLAY_ARTIFACTS: &[&str] = &[
    "bootstrap_replay_contract",
    "bootstrap_replay_law",
    "bootstrap_replay_operator",
    "valid_bootstrap_replay_fixture",
    "golden_bootstrap_replay_receipt",
    "execution_bootstrap_replay_receipt",
    "deterministic_bootstrap_replay_report",
    "bootstrap_replay_suite_report",
];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const ALLOWED_PROOF_SCOPES: &[&str] = &["domain", "chain", "witness", "receipt"];
const ALLOWED_RELATIONS: &[&str] = &["precedes", "binds", "commits"];
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
    "shells",
    "docs",
    "products",
    "examples",
];
const FORBIDDEN_BOOTSTRAP_REPLAY_TEXT: &[(&str, ErrorCode)] = &[
    ("mutable replay allowed", ErrorCode::ReplayDriftAccepted),
    ("network replay required", ErrorCode::AmbientNetworkAllowed),
    ("remote replay", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic replay allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("ambient time allowed", ErrorCode::AmbientTimeAllowed),
    ("manual only", ErrorCode::ReplayDriftAccepted),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_bootstrap_replay_surface(
    input: &str,
) -> Result<BootstrapReplaySurface, Vec<ValidationError>> {
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
            "empty bootstrap replay surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P02_BOOTSTRAP_REPLAY_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_REPLAY_CONTRACT}"),
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
    let mut seen_orders = BTreeSet::new();
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
                    ErrorCode::MissingReplayRule,
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
                    format!("duplicate or invalid bootstrap replay receipt {id}"),
                ));
            }
            receipts.push(BootstrapReplayReceiptBinding {
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
            let order = field(&fields, "order");
            if !is_symbolic_name(&id) || !seen_witnesses.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReplayWitness,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap replay witness {id}"),
                ));
            }
            if order.len() != 3
                || !order.bytes().all(|byte| byte.is_ascii_digit())
                || !seen_orders.insert(order.clone())
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReplayWitness,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid witness order {order}"),
                ));
            }
            witnesses.push(BootstrapReplayWitnessBinding {
                line_number,
                id,
                order,
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
                    format!("duplicate or invalid bootstrap replay link {id}"),
                ));
            }
            links.push(BootstrapReplayChainLinkBinding {
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
                    format!("duplicate or invalid bootstrap replay proof {id}"),
                ));
            }
            proofs.push(BootstrapReplayProofBinding {
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
                    ErrorCode::DuplicateDeliveryArtifact,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap replay artifact {id}"),
                ));
            }
            artifacts.push(BootstrapReplayArtifactBinding {
                line_number,
                id,
                owner_root: field(&fields, "owner"),
                path: field(&fields, "path"),
                artifact_kind: field(&fields, "kind"),
                status: field(&fields, "status"),
            });
            continue;
        }
        match left {
            "phase" => set_scalar(
                &mut phase,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            "task" => set_scalar(
                &mut task,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            "status" => set_scalar(
                &mut status,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown bootstrap replay line {line}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(BootstrapReplaySurface {
        header,
        phase: phase.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingPhase,
                "phase",
                "missing phase",
            )]
        })?,
        task: task.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingTask,
                "task",
                "missing task",
            )]
        })?,
        status: status.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                "status",
                "missing status",
            )]
        })?,
        rules,
        receipts,
        witnesses,
        links,
        proofs,
        artifacts,
    })
}

pub fn validate_bootstrap_replay_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_bootstrap_replay_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_bootstrap_replay(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_replay_model(surface: &BootstrapReplaySurface) -> Verdict {
    let mut errors = Vec::new();
    validate_bootstrap_replay(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_bootstrap_replay(surface: &BootstrapReplaySurface, errors: &mut Vec<ValidationError>) {
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-017" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-017 got {}", surface.task),
        ));
    }
    validate_status("surface", "P02-017", 0, &surface.status, errors);

    for rule in REQUIRED_BOOTSTRAP_REPLAY_RULES {
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
                "missing bootstrap replay rule",
            )),
        }
    }

    require_ids(
        "receipt",
        REQUIRED_BOOTSTRAP_REPLAY_RECEIPTS,
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
        REQUIRED_BOOTSTRAP_REPLAY_WITNESSES,
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
        REQUIRED_BOOTSTRAP_REPLAY_LINKS,
        surface.links.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingReceiptChainBinding,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_BOOTSTRAP_REPLAY_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingReplayProof,
        errors,
    );
    require_ids(
        "artifact",
        REQUIRED_BOOTSTRAP_REPLAY_ARTIFACTS,
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
    let artifact_ids: BTreeSet<&str> = surface
        .artifacts
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let mut referenced_receipts: BTreeSet<&str> = BTreeSet::new();

    for receipt in &surface.receipts {
        validate_status(
            "receipt",
            &receipt.id,
            receipt.line_number,
            &receipt.status,
            errors,
        );
        if !receipt.path.starts_with("receipts/p02/")
            || !receipt.path.ends_with(".receipt")
            || receipt.path.contains("..")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayReceipt,
                format!("line:{:03}", receipt.line_number),
                format!("receipt {} path is invalid", receipt.id),
            ));
        }
        if !is_hash(&receipt.input_hash)
            || !is_hash(&receipt.canonical_hash)
            || !is_hash(&receipt.verdict_hash)
            || !is_hash(&receipt.receipt_hash)
        {
            errors.push(ValidationError::reject(
                ErrorCode::ReceiptHashMismatch,
                format!("line:{:03}", receipt.line_number),
                format!("receipt {} has invalid hash", receipt.id),
            ));
        }
        let Some(descriptor) = bootstrap_replay_receipt_descriptor(&receipt.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayReceipt,
                format!("line:{:03}", receipt.line_number),
                format!("unknown bootstrap replay receipt {}", receipt.id),
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
        if bootstrap_replay_receipt_digest(&receipt.id).is_none() {
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
        if witness.receipts.is_empty()
            || witness.commands.is_empty()
            || !is_hash(&witness.witness_hash)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayWitness,
                format!("line:{:03}", witness.line_number),
                format!("witness {} has incomplete replay binding", witness.id),
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
        let Some(descriptor) = bootstrap_replay_witness_descriptor(&witness.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayWitness,
                format!("line:{:03}", witness.line_number),
                format!("unknown bootstrap replay witness {}", witness.id),
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
        if bootstrap_replay_witness_digest(&witness.id).is_none() {
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
        if !receipt_ids.contains(link.from.as_str())
            || !receipt_ids.contains(link.to.as_str())
            || link.receipts.len() < 2
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                format!("line:{:03}", link.line_number),
                format!("link {} does not bind known receipt endpoints", link.id),
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
        let Some(descriptor) = bootstrap_replay_link_descriptor(&link.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                format!("line:{:03}", link.line_number),
                format!("unknown bootstrap replay link {}", link.id),
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
        if bootstrap_replay_link_digest(&link.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                format!("line:{:03}", link.line_number),
                format!("link {} is not digestible", link.id),
            ));
        }
    }

    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str())
            || proof.receipts.is_empty()
            || proof.witnesses.is_empty()
            || proof.commands.is_empty()
            || proof.forbids.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} has invalid scope or empty binding set", proof.id),
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
        let Some(descriptor) = bootstrap_replay_proof_descriptor(&proof.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayProof,
                format!("line:{:03}", proof.line_number),
                format!("unknown bootstrap replay proof {}", proof.id),
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
        if bootstrap_replay_proof_digest(&proof.id).is_none() {
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
        let Some(descriptor) = bootstrap_replay_artifact_descriptor(&artifact.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                format!("line:{:03}", artifact.line_number),
                format!("unknown bootstrap replay artifact {}", artifact.id),
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
        if bootstrap_replay_artifact_digest(&artifact.id).is_none() {
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
                "bootstrap replay receipt is not bound by witness link or proof",
            ));
        }
    }

    let suite = deterministic_bootstrap_replay_suite_report(
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
    if suite.receipt_count != REQUIRED_BOOTSTRAP_REPLAY_RECEIPTS.len()
        || suite.witness_count != REQUIRED_BOOTSTRAP_REPLAY_WITNESSES.len()
        || suite.link_count != REQUIRED_BOOTSTRAP_REPLAY_LINKS.len()
        || suite.proof_count != REQUIRED_BOOTSTRAP_REPLAY_PROOFS.len()
        || suite.artifact_count != REQUIRED_BOOTSTRAP_REPLAY_ARTIFACTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ReplayDriftAccepted,
            "deterministic_report",
            "bootstrap replay deterministic report count drift",
        ));
    }
    if receipt_ids
        != bootstrap_replay_receipt_ids()
            .into_iter()
            .collect::<BTreeSet<_>>()
        || witness_ids
            != bootstrap_replay_witness_ids()
                .into_iter()
                .collect::<BTreeSet<_>>()
        || link_ids
            != bootstrap_replay_link_ids()
                .into_iter()
                .collect::<BTreeSet<_>>()
        || surface
            .proofs
            .iter()
            .map(|item| item.id.as_str())
            .collect::<BTreeSet<_>>()
            != bootstrap_replay_proof_ids()
                .into_iter()
                .collect::<BTreeSet<_>>()
        || artifact_ids
            != bootstrap_replay_artifact_ids()
                .into_iter()
                .collect::<BTreeSet<_>>()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ReplayDriftAccepted,
            "registry",
            "bootstrap replay surface ids diverge from descriptor registry",
        ));
    }
    if bootstrap_replay_registry_hash().is_empty()
        || !bootstrap_replay_carrier_signature().starts_with(LYRA_P02_BOOTSTRAP_REPLAY_CARRIER)
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidReplayProof,
            "registry",
            "bootstrap replay registry is not hashable",
        ));
    }
    if !bootstrap_replay_witnesses_bind_known_receipts()
        || !bootstrap_replay_links_bind_known_receipts()
        || !bootstrap_replay_proofs_bind_registry()
        || !bootstrap_replay_artifacts_bind_paths()
        || !bootstrap_replay_receipts_cover_p02_001_through_p02_017()
        || !bootstrap_replay_no_forbidden_descriptor_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ReplayProofUnbound,
            "registry",
            "bootstrap replay registry binding check failed",
        ));
    }
}

fn set_scalar(
    target: &mut Option<String>,
    value: &str,
    name: &str,
    line_number: usize,
    seen: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    if !seen.insert(name.to_string()) || target.is_some() {
        errors.push(ValidationError::reject(
            ErrorCode::DuplicateEntry,
            format!("line:{line_number:03}"),
            format!("duplicate scalar {name}"),
        ));
    } else {
        *target = Some(value.to_string());
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
    let mut seen = BTreeSet::new();
    fields
        .get(name)
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .filter(|item| seen.insert((*item).to_string()))
                .map(ToString::to_string)
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
                format!("missing required bootstrap replay {kind} {id}"),
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
        && value.as_bytes()[0].is_ascii_lowercase()
}

fn is_hash(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("fnv1a128:") else {
        return false;
    };
    rest.len() == 32 && rest.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_BOOTSTRAP_REPLAY_TEXT {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "bootstrap_replay_text",
                format!("forbidden bootstrap replay text {needle}"),
            ));
        }
    }
}
