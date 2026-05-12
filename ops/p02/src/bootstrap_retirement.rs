use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_retirement_model::{
    BootstrapRetirementGateBinding, BootstrapRetirementReceiptBinding,
    BootstrapRetirementSupersessionSurface, BootstrapRetirementSurfaceBinding,
    BootstrapSupersessionBinding,
};

pub const P02_BOOTSTRAP_RETIREMENT_SUPERSESSION_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-RETIREMENT-SUPERSESSION v1";
pub const REQUIRED_BOOTSTRAP_RETIREMENT_RULES: &[&str] = &[
    "bootstrap_retirement_law_must_cover_transitional_surfaces",
    "every_bootstrap_surface_must_bind_owner_root",
    "every_bootstrap_surface_must_bind_replacement_or_retention",
    "every_bootstrap_surface_must_bind_deletion_gate",
    "every_bootstrap_surface_must_bind_supersession_rule",
    "every_bootstrap_surface_must_bind_receipt",
    "host_surfaces_must_have_extinction_gate",
    "seed_runtime_surfaces_must_have_replacement_gate",
    "foreign_surface_closure_must_bind_retirement",
    "retained_surfaces_must_have_retention_reason",
    "historical_surfaces_must_archive_as_superseded",
    "p02_x05_must_keep_p03_open",
    "no_network_dependency",
    "no_docs_only_bootstrap_retirement",
    "no_unreceipted_bootstrap_retirement",
    "no_drift_accepted",
    "no_ambient_time_gate",
    "no_global_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_RETIREMENT_SURFACES: &[&str] = &[
    "rust_bootstrap_crate",
    "cargo_build_driver",
    "p02_bootstrap_cli_checks",
    "p02_bootstrap_text_contracts",
    "p02_bootstrap_control_plane",
    "p02_bootstrap_fixture_corpus",
    "p02_bootstrap_golden_receipts",
    "p02_bootstrap_docs_examples",
    "p02_bootstrap_product_surfaces",
    "p02_bootstrap_receipt_format",
    "p02_bootstrap_hash_canonicalization",
    "p02_bootstrap_extinction_ledger",
    "p02_seed_runtime_contracts",
    "p02_foreign_surface_closure",
    "p02_bootstrap_proof_family",
    "p02_bootstrap_benchmark_pack",
    "p02_bootstrap_output_table",
    "p02_bootstrap_operator_surfaces",
];
pub const REQUIRED_BOOTSTRAP_RETIREMENT_GATES: &[&str] = &[
    "gate_rust_bootstrap_crate",
    "gate_cargo_build_driver",
    "gate_p02_bootstrap_cli_checks",
    "gate_p02_bootstrap_text_contracts",
    "gate_p02_bootstrap_control_plane",
    "gate_p02_bootstrap_fixture_corpus",
    "gate_p02_bootstrap_golden_receipts",
    "gate_p02_bootstrap_docs_examples",
    "gate_p02_bootstrap_product_surfaces",
    "gate_p02_bootstrap_receipt_format",
    "gate_p02_bootstrap_hash_canonicalization",
    "gate_p02_bootstrap_extinction_ledger",
    "gate_p02_seed_runtime_contracts",
    "gate_p02_foreign_surface_closure",
    "gate_p02_bootstrap_proof_family",
    "gate_p02_bootstrap_benchmark_pack",
    "gate_p02_bootstrap_output_table",
    "gate_p02_bootstrap_operator_surfaces",
];
pub const REQUIRED_BOOTSTRAP_SUPERSESSIONS: &[&str] = &[
    "supersede_rust_bootstrap_crate",
    "supersede_cargo_build_driver",
    "supersede_p02_bootstrap_cli_checks",
    "supersede_p02_bootstrap_text_contracts",
    "supersede_p02_bootstrap_control_plane",
    "supersede_p02_bootstrap_fixture_corpus",
    "supersede_p02_bootstrap_golden_receipts",
    "supersede_p02_bootstrap_docs_examples",
    "supersede_p02_bootstrap_product_surfaces",
    "supersede_p02_bootstrap_receipt_format",
    "supersede_p02_bootstrap_hash_canonicalization",
    "supersede_p02_bootstrap_extinction_ledger",
    "supersede_p02_seed_runtime_contracts",
    "supersede_p02_foreign_surface_closure",
    "supersede_p02_bootstrap_proof_family",
    "supersede_p02_bootstrap_benchmark_pack",
    "supersede_p02_bootstrap_output_table",
    "supersede_p02_bootstrap_operator_surfaces",
];
pub const REQUIRED_BOOTSTRAP_RETIREMENT_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_retirement_supersession",
    "receipt_bootstrap_output_table",
    "receipt_bootstrap_closure_gate",
    "receipt_bootstrap_replay_witness",
    "receipt_bootstrap_extinction",
    "receipt_seed_runtime_replacement",
    "receipt_foreign_surface_closure",
    "receipt_bootstrap_proof_family",
    "receipt_bootstrap_benchmark_pack",
];

const ALLOWED_OWNER_ROOTS: &[&str] = &["k0", "k1", "lyralang", "shells", "interfaces", "ops", "slices", "products", "android", "web"];
const ALLOWED_SURFACE_KINDS: &[&str] = &["bootstrap", "cli", "contract", "control", "fixture", "golden", "doc", "example", "product", "receipt", "hash", "closure", "operator", "model", "carrier", "benchmark", "host", "tool"];
const ALLOWED_SURFACE_STATUSES: &[&str] = &["retirement_scheduled", "retained_by_law", "bounded_active"];
const ALLOWED_GATE_ACTIONS: &[&str] = &["retain_until_replaced", "retire_after_replacement", "archive_after_supersession"];
const ALLOWED_GATE_TRIGGERS: &[&str] = &["lyralang_native_equivalent", "native_build_driver_proven", "native_target_kernel_proven", "native_storage_driver_proven", "native_operator_shell_proven", "seed_runtime_replacement_complete", "host_extinction_complete", "foreign_surface_closure_complete", "p02_historical_archive", "receipt_format_v2", "hash_suite_v2", "operator_surface_successor", "proof_bundle_native_equivalent", "benchmark_harness_successor", "output_table_successor"];
const ALLOWED_GATE_STATUSES: &[&str] = &["armed", "retained_by_law", "blocked_until_successor"];
const ALLOWED_ARCHIVE_LANES: &[&str] = &["historical/superseded", "retained/active"];
const ALLOWED_SUPERSESSION_STATUSES: &[&str] = &["armed", "retained_by_law"];
const ALLOWED_RECEIPT_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];

const FORBIDDEN_BOOTSTRAP_RETIREMENT_TEXT: &[(&str, ErrorCode)] = &[
    ("network_required:true", ErrorCode::ClosureNetworkDependency),
    ("rule:network_required", ErrorCode::ClosureNetworkDependency),
    ("cloud required", ErrorCode::ClosureNetworkDependency),
    ("online required", ErrorCode::ClosureNetworkDependency),
    ("remote service required", ErrorCode::ClosureNetworkDependency),
    ("remote fetch", ErrorCode::ClosureNetworkDependency),
    ("docs_only:true", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("retirement without receipt", ErrorCode::ClosureUnreceipted),
    ("unreceipted bootstrap retirement allowed", ErrorCode::ClosureUnreceipted),
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

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}
fn parse_fields(value: &str) -> BTreeMap<&str, &str> {
    value.split('|').filter_map(|part| part.split_once(':')).collect()
}
fn require_field<'a>(fields: &'a BTreeMap<&str, &str>, name: &str, line_number: usize, errors: &mut Vec<ValidationError>) -> Option<&'a str> {
    match fields.get(name).copied() {
        Some(value) if !value.is_empty() => Some(value),
        _ => { errors.push(ValidationError::reject(ErrorCode::InvalidEntrySyntax, format!("line:{line_number:03}"), format!("missing field {name}"))); None }
    }
}

pub fn parse_bootstrap_retirement_supersession_surface(input: &str) -> Result<BootstrapRetirementSupersessionSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => return Err(vec![ValidationError::reject(ErrorCode::CanonicalControlByte, "input", format!("canonicalization failed: {error:?}"))]),
    };
    if lines.is_empty() { return Err(vec![ValidationError::reject(ErrorCode::EmptySurface, "input", "bootstrap retirement surface is empty")]); }
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_RETIREMENT_SUPERSESSION_CONTRACT {
        return Err(vec![ValidationError::reject(ErrorCode::InvalidHeader, "line:001", format!("expected {P02_BOOTSTRAP_RETIREMENT_SUPERSESSION_CONTRACT}"))]);
    }
    let mut errors = Vec::new();
    let mut phase = None; let mut task = None; let mut status = None; let mut global_closure = None; let mut next_frontier = None;
    let mut rules = BTreeMap::new();
    let mut surfaces = Vec::new(); let mut gates = Vec::new(); let mut supersessions = Vec::new(); let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new(); let mut seen_rules = BTreeSet::new(); let mut seen_surfaces = BTreeSet::new(); let mut seen_gates = BTreeSet::new(); let mut seen_supersessions = BTreeSet::new(); let mut seen_receipts = BTreeSet::new();
    for (offset, line) in lines.iter().enumerate().skip(1) {
        let line_number = offset + 1;
        let Some((left, value)) = line.split_once('=') else { errors.push(ValidationError::reject(ErrorCode::InvalidEntrySyntax, format!("line:{line_number:03}"), "entry must contain one equals separator")); continue; };
        if left.is_empty() || value.is_empty() || left != left.trim() || value != value.trim() { errors.push(ValidationError::reject(ErrorCode::InvalidEntrySyntax, format!("line:{line_number:03}"), "entry sides must be non-empty and trimmed")); continue; }
        if let Some(rule_name) = left.strip_prefix("rule:") {
            if !is_symbolic_name(rule_name) || !seen_rules.insert(rule_name.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateEntry, format!("line:{line_number:03}"), "duplicate or invalid rule")); continue; }
            rules.insert(rule_name.to_string(), value.to_string()); continue;
        }
        if let Some(id) = left.strip_prefix("surface:") {
            if !is_symbolic_name(id) || !seen_surfaces.insert(id.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateClosureOutputGate, format!("line:{line_number:03}"), "duplicate or invalid surface")); continue; }
            let fields = parse_fields(value);
            let Some(owner_root) = require_field(&fields, "owner_root", line_number, &mut errors) else { continue; };
            let Some(surface_kind) = require_field(&fields, "kind", line_number, &mut errors) else { continue; };
            let Some(path) = require_field(&fields, "path", line_number, &mut errors) else { continue; };
            let Some(replacement) = require_field(&fields, "replacement", line_number, &mut errors) else { continue; };
            let Some(retirement_gate) = require_field(&fields, "retirement_gate", line_number, &mut errors) else { continue; };
            let Some(supersession) = require_field(&fields, "supersession", line_number, &mut errors) else { continue; };
            let Some(receipt) = require_field(&fields, "receipt", line_number, &mut errors) else { continue; };
            let Some(surface_status) = require_field(&fields, "status", line_number, &mut errors) else { continue; };
            surfaces.push(BootstrapRetirementSurfaceBinding{ line_number, id:id.to_string(), owner_root:owner_root.to_string(), surface_kind:surface_kind.to_string(), path:path.to_string(), replacement:replacement.to_string(), retirement_gate:retirement_gate.to_string(), supersession:supersession.to_string(), receipt:receipt.to_string(), status:surface_status.to_string() }); continue;
        }
        if let Some(id) = left.strip_prefix("gate:") {
            if !is_symbolic_name(id) || !seen_gates.insert(id.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateClosureOutputGate, format!("line:{line_number:03}"), "duplicate or invalid gate")); continue; }
            let fields = parse_fields(value);
            let Some(surface) = require_field(&fields, "surface", line_number, &mut errors) else { continue; };
            let Some(trigger) = require_field(&fields, "trigger", line_number, &mut errors) else { continue; };
            let Some(action) = require_field(&fields, "action", line_number, &mut errors) else { continue; };
            let Some(evidence) = require_field(&fields, "evidence", line_number, &mut errors) else { continue; };
            let Some(gate_status) = require_field(&fields, "status", line_number, &mut errors) else { continue; };
            let evidence = evidence.split(',').filter(|v| !v.is_empty()).map(str::to_string).collect();
            gates.push(BootstrapRetirementGateBinding{ line_number, id:id.to_string(), surface:surface.to_string(), trigger:trigger.to_string(), action:action.to_string(), evidence, status:gate_status.to_string() }); continue;
        }
        if let Some(id) = left.strip_prefix("supersession:") {
            if !is_symbolic_name(id) || !seen_supersessions.insert(id.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateClosureOutputGate, format!("line:{line_number:03}"), "duplicate or invalid supersession")); continue; }
            let fields = parse_fields(value);
            let Some(surface) = require_field(&fields, "surface", line_number, &mut errors) else { continue; };
            let Some(replaced_by) = require_field(&fields, "replaced_by", line_number, &mut errors) else { continue; };
            let Some(archive) = require_field(&fields, "archive", line_number, &mut errors) else { continue; };
            let Some(receipt) = require_field(&fields, "receipt", line_number, &mut errors) else { continue; };
            let Some(sup_status) = require_field(&fields, "status", line_number, &mut errors) else { continue; };
            supersessions.push(BootstrapSupersessionBinding{ line_number, id:id.to_string(), surface:surface.to_string(), replaced_by:replaced_by.to_string(), archive:archive.to_string(), receipt:receipt.to_string(), status:sup_status.to_string() }); continue;
        }
        if let Some(id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(id) || !seen_receipts.insert(id.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateClosureProof, format!("line:{line_number:03}"), "duplicate or invalid receipt")); continue; }
            let fields = parse_fields(value);
            let Some(path) = require_field(&fields, "path", line_number, &mut errors) else { continue; };
            let Some(target) = require_field(&fields, "target", line_number, &mut errors) else { continue; };
            let Some(receipt_status) = require_field(&fields, "status", line_number, &mut errors) else { continue; };
            receipts.push(BootstrapRetirementReceiptBinding{ line_number, id:id.to_string(), path:path.to_string(), target:target.to_string(), status:receipt_status.to_string() }); continue;
        }
        match left {
            "phase" => { if !seen_scalars.insert(left.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateEntry, format!("line:{line_number:03}"), "duplicate phase")); } phase = Some(value.to_string()); }
            "task" => { if !seen_scalars.insert(left.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateEntry, format!("line:{line_number:03}"), "duplicate task")); } task = Some(value.to_string()); }
            "status" => { if !seen_scalars.insert(left.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateEntry, format!("line:{line_number:03}"), "duplicate status")); } status = Some(value.to_string()); }
            "global_closure" => { if !seen_scalars.insert(left.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateEntry, format!("line:{line_number:03}"), "duplicate global_closure")); } global_closure = Some(value.to_string()); }
            "next_frontier" => { if !seen_scalars.insert(left.to_string()) { errors.push(ValidationError::reject(ErrorCode::DuplicateEntry, format!("line:{line_number:03}"), "duplicate next_frontier")); } next_frontier = Some(value.to_string()); }
            _ => errors.push(ValidationError::reject(ErrorCode::InvalidEntrySyntax, format!("line:{line_number:03}"), format!("unknown entry {left}"))),
        }
    }
    let Some(phase) = phase else { errors.push(ValidationError::reject(ErrorCode::MissingPhase, "phase", "missing phase")); return Err(errors); };
    let Some(task) = task else { errors.push(ValidationError::reject(ErrorCode::MissingTask, "task", "missing task")); return Err(errors); };
    let status = status.unwrap_or_default(); let global_closure = global_closure.unwrap_or_default(); let next_frontier = next_frontier.unwrap_or_default();
    if !errors.is_empty() { return Err(errors); }
    Ok(BootstrapRetirementSupersessionSurface{ header, phase, task, status, global_closure, next_frontier, rules, surfaces, gates, supersessions, receipts })
}

pub fn validate_bootstrap_retirement_supersession_model(surface: &BootstrapRetirementSupersessionSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" { errors.push(ValidationError::reject(ErrorCode::InvalidPhase, "phase", "expected P02")); }
    if surface.task != "P02-X05" { errors.push(ValidationError::reject(ErrorCode::InvalidTask, "task", "expected P02-X05")); }
    if surface.status != "artifact_emitted" { errors.push(ValidationError::reject(ErrorCode::UnsupportedClosureStatus, "status", "expected artifact_emitted")); }
    if surface.global_closure != "denied" { errors.push(ValidationError::reject(ErrorCode::UnsupportedGlobalClosure, "global_closure", "P02 global closure remains denied without local validation evidence")); }
    if surface.next_frontier != "P03" { errors.push(ValidationError::reject(ErrorCode::ClosureOutputPremature, "next_frontier", "P02-X05 must point to P03 as next frontier without claiming global closure")); }
    for required in REQUIRED_BOOTSTRAP_RETIREMENT_RULES { match surface.rule_value(required) { Some("required")|Some("forbidden") => {}, _ => errors.push(ValidationError::reject(ErrorCode::MissingClosureRule, *required, "missing required retirement rule")), } }
    for required in REQUIRED_BOOTSTRAP_RETIREMENT_SURFACES { if surface.surface_by_id(required).is_none() { errors.push(ValidationError::reject(ErrorCode::MissingClosureOutputGate, *required, "missing required surface")); } }
    for required in REQUIRED_BOOTSTRAP_RETIREMENT_GATES { if surface.gate_by_id(required).is_none() { errors.push(ValidationError::reject(ErrorCode::MissingClosureOutputGate, *required, "missing required gate")); } }
    for required in REQUIRED_BOOTSTRAP_SUPERSESSIONS { if surface.supersession_by_id(required).is_none() { errors.push(ValidationError::reject(ErrorCode::MissingClosureOutputGate, *required, "missing required supersession")); } }
    for required in REQUIRED_BOOTSTRAP_RETIREMENT_RECEIPTS { if surface.receipt_by_id(required).is_none() { errors.push(ValidationError::reject(ErrorCode::MissingClosureProof, *required, "missing required receipt")); } }
    for item in &surface.surfaces {
        if !ALLOWED_OWNER_ROOTS.contains(&item.owner_root.as_str()) { errors.push(ValidationError::reject(ErrorCode::InvalidOwnerRoot, item.canonical_identity(), format!("invalid owner root {}", item.owner_root))); }
        if !ALLOWED_SURFACE_KINDS.contains(&item.surface_kind.as_str()) { errors.push(ValidationError::reject(ErrorCode::InvalidClosureOutputGate, item.canonical_identity(), format!("invalid surface kind {}", item.surface_kind))); }
        if !ALLOWED_SURFACE_STATUSES.contains(&item.status.as_str()) { errors.push(ValidationError::reject(ErrorCode::InvalidClosureOutputGate, item.canonical_identity(), format!("invalid surface status {}", item.status))); }
        if item.path.contains("//") || item.path.starts_with('/') { errors.push(ValidationError::reject(ErrorCode::InvalidClosureOutputGate, item.canonical_identity(), "surface path must be repo-relative")); }
        if surface.gate_by_id(&item.retirement_gate).is_none() || surface.supersession_by_id(&item.supersession).is_none() || surface.receipt_by_id(&item.receipt).is_none() { errors.push(ValidationError::reject(ErrorCode::ClosureProofUnbound, item.canonical_identity(), "surface must bind existing gate, supersession, and receipt")); }
        if item.status == "retained_by_law" && item.replacement.is_empty() { errors.push(ValidationError::reject(ErrorCode::InvalidClosureOutputGate, item.canonical_identity(), "retained surface must bind retention replacement/reason")); }
    }
    for gate in &surface.gates {
        if surface.surface_by_id(&gate.surface).is_none() { errors.push(ValidationError::reject(ErrorCode::ClosureProofUnbound, gate.canonical_identity(), "gate references unknown surface")); }
        if !ALLOWED_GATE_TRIGGERS.contains(&gate.trigger.as_str()) || !ALLOWED_GATE_ACTIONS.contains(&gate.action.as_str()) || !ALLOWED_GATE_STATUSES.contains(&gate.status.as_str()) { errors.push(ValidationError::reject(ErrorCode::InvalidClosureOutputGate, gate.canonical_identity(), "invalid gate trigger/action/status")); }
        if gate.evidence.is_empty() || gate.evidence.iter().any(|id| surface.receipt_by_id(id).is_none()) { errors.push(ValidationError::reject(ErrorCode::ClosureProofUnbound, gate.canonical_identity(), "gate evidence must bind existing receipts")); }
    }
    for supersession in &surface.supersessions {
        if surface.surface_by_id(&supersession.surface).is_none() || surface.receipt_by_id(&supersession.receipt).is_none() { errors.push(ValidationError::reject(ErrorCode::ClosureProofUnbound, supersession.canonical_identity(), "supersession must bind known surface and receipt")); }
        if !ALLOWED_ARCHIVE_LANES.contains(&supersession.archive.as_str()) || !ALLOWED_SUPERSESSION_STATUSES.contains(&supersession.status.as_str()) { errors.push(ValidationError::reject(ErrorCode::InvalidClosureOutputGate, supersession.canonical_identity(), "invalid archive lane or supersession status")); }
    }
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p02/") || !ALLOWED_RECEIPT_STATUSES.contains(&receipt.status.as_str()) { errors.push(ValidationError::reject(ErrorCode::InvalidClosureProof, receipt.canonical_identity(), "invalid receipt path/status")); }
    }
    if errors.is_empty() { Verdict::accepted() } else { Verdict::rejected(errors) }
}

pub fn validate_bootstrap_retirement_supersession_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let parse_result = parse_bootstrap_retirement_supersession_surface(input);
    let mut errors = Vec::new();
    let verdict = match parse_result {
        Ok(surface) => {
            for (needle, code) in FORBIDDEN_BOOTSTRAP_RETIREMENT_TEXT { if input.contains(needle) { errors.push(ValidationError::reject(*code, "surface", format!("forbidden text {needle}"))); } }
            let model_verdict = validate_bootstrap_retirement_supersession_model(&surface);
            if !model_verdict.accepted { errors.extend(model_verdict.errors); }
            if errors.is_empty() { Verdict::accepted() } else { Verdict::rejected(errors) }
        }
        Err(parse_errors) => Verdict::rejected(parse_errors),
    };
    let receipt = build_phase_receipt("P02", input, &canonical_text, verdict.clone());
    (verdict, receipt)
}
