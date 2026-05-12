use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_extinction::deterministic_bootstrap_extinction_ledger_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_extinction_model::{
    BootstrapExtinctionEntryBinding, BootstrapExtinctionLedgerSurface,
    BootstrapExtinctionReceiptBinding, BootstrapRetirementGateBinding,
};

pub const P02_BOOTSTRAP_EXTINCTION_LEDGER_CONTRACT: &str =
    "LYRA-P02-BOOTSTRAP-EXTINCTION-LEDGER v1";

pub const REQUIRED_BOOTSTRAP_EXTINCTION_RULES: &[&str] = &[
    "all_inventory_surfaces_must_have_extinction_entry",
    "temporary_surfaces_must_have_retirement_gate",
    "observer_surfaces_must_have_observer_containment_gate",
    "bounded_permanent_surfaces_must_have_target_descriptor_bound",
    "forbidden_surfaces_must_have_no_import_gate",
    "every_entry_must_bind_owner_root",
    "every_entry_must_bind_deletion_action",
    "every_entry_must_bind_successor_or_retention",
    "every_entry_must_bind_evidence",
    "ledger_must_bind_inventory_receipt",
    "no_ambient_network_dependency",
    "no_probabilistic_extinction_truth",
    "no_hidden_randomness",
    "no_placeholder_ledger",
    "no_ambient_time_gate",
    "no_global_phase_closure_claim",
];

pub const REQUIRED_BOOTSTRAP_EXTINCTION_ENTRIES: &[&str] = &[
    "artifact_generation_python_helper",
    "cargo_build_driver",
    "cursor_codex_assisted_editor",
    "external_sha256sum_tool",
    "external_wall_clock",
    "external_zip_packager",
    "git_repository_transport",
    "host_filesystem",
    "host_operating_system",
    "host_process_launcher",
    "lyra_text_contract_carrier",
    "lyralang_bootstrap_stub_carrier",
    "operator_shell_terminal",
    "physical_cpu_instruction_set",
    "rust_bootstrap_compiler",
    "rust_std_runtime",
    "unbounded_network_bootstrap_fetch",
];

pub const REQUIRED_BOOTSTRAP_RETIREMENT_GATES: &[&str] = &[
    "gate_artifact_generation_python_helper",
    "gate_cargo_build_driver",
    "gate_cursor_codex_assisted_editor",
    "gate_external_sha256sum_tool",
    "gate_external_wall_clock",
    "gate_external_zip_packager",
    "gate_git_repository_transport",
    "gate_host_filesystem",
    "gate_host_operating_system",
    "gate_host_process_launcher",
    "gate_lyra_text_contract_carrier",
    "gate_lyralang_bootstrap_stub_carrier",
    "gate_operator_shell_terminal",
    "gate_physical_cpu_instruction_set",
    "gate_rust_bootstrap_compiler",
    "gate_rust_std_runtime",
    "gate_unbounded_network_bootstrap_fetch",
];

pub const REQUIRED_BOOTSTRAP_EXTINCTION_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_surface_inventory",
    "receipt_bootstrap_extinction_ledger",
    "receipt_retirement_gate_format",
    "receipt_forbidden_surface_no_import",
];

const ALLOWED_OWNER_ROOTS: &[&str] = &[
    "k0",
    "k1",
    "lyralang",
    "shells",
    "interfaces",
    "ops",
    "slices",
    "products",
    "android",
    "web",
];
const ALLOWED_CLASSIFICATIONS: &[&str] =
    &["temporary", "observer", "bounded_permanent", "forbidden"];
const ALLOWED_ACTIONS: &[&str] = &[
    "delete_after_native_artifact_emitter",
    "delete_after_native_build_driver",
    "delete_after_native_packager",
    "delete_after_native_storage_driver",
    "delete_after_native_target_kernel",
    "delete_after_native_process_launcher",
    "delete_after_native_contract_surface",
    "delete_after_self_hosted_lyralang_carrier",
    "delete_after_native_operator_shell",
    "delete_after_lyralang_native_compiler",
    "delete_after_lyra_native_runtime",
    "quarantine_and_discard",
    "retain_as_target_descriptor",
    "deny_import_and_delete_reference",
];
const ALLOWED_LEDGER_STATES: &[&str] = &[
    "deletion_scheduled",
    "contained",
    "retained_by_target_descriptor",
    "forbidden_no_import",
];
const ALLOWED_ENTRY_STATUSES: &[&str] = &[
    "armed",
    "contained",
    "retained_by_law",
    "forbidden_declared",
];
const ALLOWED_GATE_KINDS: &[&str] = &[
    "native_successor_gate",
    "observer_containment_gate",
    "target_descriptor_gate",
    "forbidden_no_import_gate",
];
const ALLOWED_GATE_STATUSES: &[&str] = &[
    "armed",
    "contained",
    "retained_by_law",
    "forbidden_declared",
];
const ALLOWED_RECEIPT_STATUSES: &[&str] = &[
    "artifact_emitted",
    "execution_proven",
    "pending_local_validation",
];

const FORBIDDEN_EXTINCTION_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote service required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    ("stochastic gate", ErrorCode::ProbabilisticTruthAllowed),
    ("random gate", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("wall clock gate", ErrorCode::AmbientTimeAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("note=placeholder", ErrorCode::PlaceholderAllowed),
    ("status:placeholder", ErrorCode::PlaceholderAllowed),
    ("todo=true", ErrorCode::PlaceholderAllowed),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("unreceipted", ErrorCode::ClosureUnreceipted),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_bootstrap_extinction_ledger_surface(
    input: &str,
) -> Result<BootstrapExtinctionLedgerSurface, Vec<ValidationError>> {
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
            "bootstrap extinction ledger is empty",
        )]);
    }
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_EXTINCTION_LEDGER_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_EXTINCTION_LEDGER_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut inventory_receipt = None;
    let mut rules = BTreeMap::new();
    let mut entries = Vec::new();
    let mut gates = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_entries = BTreeSet::new();
    let mut seen_gates = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
        let Some((left, value)) = line.split_once('=') else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry must contain exactly one key/value separator",
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
                    "bootstrap extinction rules must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(entry_id) = left.strip_prefix("entry:") {
            if !is_symbolic_name(entry_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid extinction entry {entry_id}"),
                ));
                continue;
            }
            if !seen_entries.insert(entry_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("entry:{entry_id}"),
                    "extinction entry identity must be unique",
                ));
                continue;
            }
            match parse_entry(line_number, entry_id, value) {
                Ok(item) => entries.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(gate_id) = left.strip_prefix("gate:") {
            if !is_symbolic_name(gate_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid retirement gate {gate_id}"),
                ));
                continue;
            }
            if !seen_gates.insert(gate_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("gate:{gate_id}"),
                    "retirement gate identity must be unique",
                ));
                continue;
            }
            match parse_gate(line_number, gate_id, value) {
                Ok(item) => gates.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(receipt_id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(receipt_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid extinction receipt {receipt_id}"),
                ));
                continue;
            }
            if !seen_receipts.insert(receipt_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("receipt:{receipt_id}"),
                    "extinction receipt identity must be unique",
                ));
                continue;
            }
            match parse_receipt(line_number, receipt_id, value) {
                Ok(item) => receipts.push(item),
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
            "inventory_receipt" => inventory_receipt = Some(value.to_string()),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown bootstrap extinction field {left}"),
            )),
        }
    }

    let phase = match phase {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPhase,
                "field:phase",
                "phase=P02 is required",
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
                "task=P02-002 is required",
            ));
            String::new()
        }
    };
    let status = match status {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                "field:status",
                "status=artifact_emitted is required",
            ));
            String::new()
        }
    };
    let inventory_receipt = match inventory_receipt {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "field:inventory_receipt",
                "inventory receipt binding is required",
            ));
            String::new()
        }
    };

    if errors.is_empty() {
        Ok(BootstrapExtinctionLedgerSurface {
            header,
            phase,
            task,
            status,
            inventory_receipt,
            rules,
            entries,
            gates,
            receipts,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_bootstrap_extinction_ledger_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_bootstrap_extinction_ledger_surface(input) {
        Ok(surface) => errors.extend(validate_bootstrap_extinction_ledger_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_extinction_ledger_model(
    surface: &BootstrapExtinctionLedgerSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "bootstrap extinction ledger must bind to P02",
        ));
    }
    if surface.task != "P02-002" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "bootstrap extinction ledger must bind to P02-002",
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported bootstrap extinction status {}", surface.status),
        ));
    }
    if surface.inventory_receipt != "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "inventory_receipt",
            "ledger must bind P02-001 inventory receipt",
        ));
    }
    require_rules(surface, &mut errors);
    require_entries(surface, &mut errors);
    require_gates(surface, &mut errors);
    require_receipts(surface, &mut errors);
    validate_entries(surface, &mut errors);
    validate_gates(surface, &mut errors);
    validate_receipts(surface, &mut errors);
    validate_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_entry(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapExtinctionEntryBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "extinction entry fields must be key:value segments",
        )
    })?;
    Ok(BootstrapExtinctionEntryBinding {
        line_number,
        id: id.to_string(),
        owner_root: required_field(
            &fields,
            "owner_root",
            ErrorCode::InvalidOwnerRoot,
            line_number,
        )?,
        classification: required_field(
            &fields,
            "classification",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        surface_ref: required_field(
            &fields,
            "surface_ref",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        deletion_gate: required_field(
            &fields,
            "deletion_gate",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        retirement_trigger: required_field(
            &fields,
            "retirement_trigger",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        deletion_action: required_field(
            &fields,
            "deletion_action",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        successor: required_field(
            &fields,
            "successor",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        ledger_state: required_field(
            &fields,
            "ledger_state",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        evidence: split_csv(&required_field(
            &fields,
            "evidence",
            ErrorCode::MissingEvidenceBinding,
            line_number,
        )?),
        receipt: required_field(
            &fields,
            "receipt",
            ErrorCode::MissingReceiptProof,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}

fn parse_gate(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapRetirementGateBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "retirement gate fields must be key:value segments",
        )
    })?;
    Ok(BootstrapRetirementGateBinding {
        line_number,
        id: id.to_string(),
        surface: required_field(
            &fields,
            "surface",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        gate_kind: required_field(
            &fields,
            "gate_kind",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        trigger: required_field(
            &fields,
            "trigger",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        allowed_action: required_field(
            &fields,
            "allowed_action",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        blocked_action: required_field(
            &fields,
            "blocked_action",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        evidence: split_csv(&required_field(
            &fields,
            "evidence",
            ErrorCode::MissingEvidenceBinding,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}

fn parse_receipt(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapExtinctionReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "extinction receipt fields must be key:value segments",
        )
    })?;
    Ok(BootstrapExtinctionReceiptBinding {
        line_number,
        id: id.to_string(),
        path: required_field(&fields, "path", ErrorCode::InvalidClosureProof, line_number)?,
        target: required_field(
            &fields,
            "target",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::UnsupportedClosureStatus,
            line_number,
        )?,
    })
}

fn require_rules(surface: &BootstrapExtinctionLedgerSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_BOOTSTRAP_EXTINCTION_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("rule:{rule}"),
                format!("unsupported rule value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "required bootstrap extinction rule is absent",
            )),
        }
    }
}

fn require_entries(surface: &BootstrapExtinctionLedgerSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_BOOTSTRAP_EXTINCTION_ENTRIES {
        if surface.entry_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("entry:{id}"),
                "required inventory surface is missing from extinction ledger",
            ));
        }
    }
}

fn require_gates(surface: &BootstrapExtinctionLedgerSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_BOOTSTRAP_RETIREMENT_GATES {
        if surface.gate_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("gate:{id}"),
                "required retirement gate is missing from extinction ledger",
            ));
        }
    }
}

fn require_receipts(surface: &BootstrapExtinctionLedgerSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_BOOTSTRAP_EXTINCTION_RECEIPTS {
        if surface.receipt_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{id}"),
                "required extinction receipt binding is missing",
            ));
        }
    }
}

fn validate_entries(surface: &BootstrapExtinctionLedgerSurface, errors: &mut Vec<ValidationError>) {
    for entry in &surface.entries {
        if !ALLOWED_OWNER_ROOTS.contains(&entry.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                entry.canonical_identity(),
                format!("invalid owner root {}", entry.owner_root),
            ));
        }
        if !ALLOWED_CLASSIFICATIONS.contains(&entry.classification.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                entry.canonical_identity(),
                format!("invalid classification {}", entry.classification),
            ));
        }
        if entry.surface_ref != format!("surface:{}", entry.id) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                entry.canonical_identity(),
                "entry surface_ref must point to matching P02-001 surface identity",
            ));
        }
        if !entry.deletion_gate.starts_with("gate_")
            || surface.gate_by_id(&entry.deletion_gate).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                entry.canonical_identity(),
                "entry deletion_gate must reference an emitted gate",
            ));
        }
        if !ALLOWED_ACTIONS.contains(&entry.deletion_action.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                entry.canonical_identity(),
                format!("unsupported deletion action {}", entry.deletion_action),
            ));
        }
        if !ALLOWED_LEDGER_STATES.contains(&entry.ledger_state.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                entry.canonical_identity(),
                format!("unsupported ledger state {}", entry.ledger_state),
            ));
        }
        if !ALLOWED_ENTRY_STATUSES.contains(&entry.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                entry.canonical_identity(),
                format!("unsupported entry status {}", entry.status),
            ));
        }
        if entry.evidence.is_empty() || entry.evidence.iter().any(|item| item.is_empty()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                entry.canonical_identity(),
                "entry evidence must be non-empty",
            ));
        }
        if !entry
            .evidence
            .iter()
            .any(|item| item == "receipt_bootstrap_surface_inventory")
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                entry.canonical_identity(),
                "entry must bind the P02-001 inventory receipt",
            ));
        }
        if surface.receipt_by_id(&entry.receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                entry.canonical_identity(),
                "entry receipt must be declared in ledger receipts",
            ));
        }
        validate_entry_class_law(entry, errors);
    }
}

fn validate_entry_class_law(
    entry: &BootstrapExtinctionEntryBinding,
    errors: &mut Vec<ValidationError>,
) {
    if entry.is_temporary() {
        if entry.ledger_state != "deletion_scheduled" || entry.status != "armed" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                entry.canonical_identity(),
                "temporary surfaces must be armed for deletion scheduling",
            ));
        }
        if entry.successor.starts_with("none") || entry.successor == "target_descriptor_bound" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                entry.canonical_identity(),
                "temporary surfaces must bind a native successor",
            ));
        }
        if !entry.retirement_trigger.starts_with("native_")
            && !entry.retirement_trigger.starts_with("self_hosted_")
            && !entry.retirement_trigger.starts_with("lyralang_")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                entry.canonical_identity(),
                "temporary retirement trigger must be native, self-hosted, or lyralang-owned",
            ));
        }
    }
    if entry.is_observer()
        && (entry.deletion_action != "quarantine_and_discard"
            || entry.successor != "none_observer_only"
            || entry.ledger_state != "contained"
            || entry.status != "contained")
    {
        errors.push(ValidationError::reject(
            ErrorCode::AmbientAuthority,
            entry.canonical_identity(),
            "observer surfaces must be contained and discarded with no truth successor",
        ));
    }
    if entry.is_bounded_permanent()
        && (entry.deletion_action != "retain_as_target_descriptor"
            || entry.successor != "target_descriptor_bound"
            || entry.ledger_state != "retained_by_target_descriptor"
            || entry.status != "retained_by_law")
    {
        errors.push(ValidationError::reject(
            ErrorCode::RootOwnershipViolation,
            entry.canonical_identity(),
            "bounded permanent surfaces must be retained only as target descriptor substrate",
        ));
    }
    if entry.is_forbidden()
        && (entry.deletion_action != "deny_import_and_delete_reference"
            || entry.successor != "none_forbidden"
            || entry.ledger_state != "forbidden_no_import"
            || entry.status != "forbidden_declared")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            entry.canonical_identity(),
            "forbidden surfaces must bind no-import deletion law",
        ));
    }
}

fn validate_gates(surface: &BootstrapExtinctionLedgerSurface, errors: &mut Vec<ValidationError>) {
    for gate in &surface.gates {
        if !ALLOWED_GATE_KINDS.contains(&gate.gate_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gate.canonical_identity(),
                format!("unsupported gate kind {}", gate.gate_kind),
            ));
        }
        if !ALLOWED_ACTIONS.contains(&gate.allowed_action.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gate.canonical_identity(),
                format!("unsupported gate action {}", gate.allowed_action),
            ));
        }
        if !ALLOWED_GATE_STATUSES.contains(&gate.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                gate.canonical_identity(),
                format!("unsupported gate status {}", gate.status),
            ));
        }
        if gate.blocked_action != "ambient_adoption"
            && gate.blocked_action != "truth_influence"
            && gate.blocked_action != "runtime_import"
            && gate.blocked_action != "semantic_ownership"
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gate.canonical_identity(),
                format!("unsupported blocked action {}", gate.blocked_action),
            ));
        }
        let Some(entry) = surface.entry_by_id(&gate.surface) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gate.canonical_identity(),
                "gate must bind a known extinction entry",
            ));
            continue;
        };
        if entry.deletion_gate != gate.id {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gate.canonical_identity(),
                "gate and entry must be mutually bound",
            ));
        }
        if entry.deletion_action != gate.allowed_action {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                gate.canonical_identity(),
                "gate allowed action must match entry deletion action",
            ));
        }
        if gate.evidence.is_empty()
            || !gate
                .evidence
                .iter()
                .any(|item| item == "receipt_bootstrap_extinction_ledger")
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                gate.canonical_identity(),
                "gate must bind extinction ledger evidence",
            ));
        }
        validate_gate_class_law(entry, gate, errors);
    }
}

fn validate_gate_class_law(
    entry: &BootstrapExtinctionEntryBinding,
    gate: &BootstrapRetirementGateBinding,
    errors: &mut Vec<ValidationError>,
) {
    if entry.is_temporary()
        && (gate.gate_kind != "native_successor_gate"
            || gate.status != "armed"
            || gate.blocked_action != "ambient_adoption")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            gate.canonical_identity(),
            "temporary gates must require native successor and block ambient adoption",
        ));
    }
    if entry.is_observer()
        && (gate.gate_kind != "observer_containment_gate"
            || gate.status != "contained"
            || gate.blocked_action != "truth_influence")
    {
        errors.push(ValidationError::reject(
            ErrorCode::AmbientAuthority,
            gate.canonical_identity(),
            "observer gates must contain observer influence",
        ));
    }
    if entry.is_bounded_permanent()
        && (gate.gate_kind != "target_descriptor_gate"
            || gate.status != "retained_by_law"
            || gate.blocked_action != "semantic_ownership")
    {
        errors.push(ValidationError::reject(
            ErrorCode::RootOwnershipViolation,
            gate.canonical_identity(),
            "bounded permanent gates must block semantic ownership",
        ));
    }
    if entry.is_forbidden()
        && (gate.gate_kind != "forbidden_no_import_gate"
            || gate.status != "forbidden_declared"
            || gate.blocked_action != "runtime_import")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            gate.canonical_identity(),
            "forbidden gates must block runtime import",
        ));
    }
}

fn validate_receipts(
    surface: &BootstrapExtinctionLedgerSurface,
    errors: &mut Vec<ValidationError>,
) {
    for receipt in &surface.receipts {
        if receipt.path.is_empty() || receipt.target.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                receipt.canonical_identity(),
                "receipt path and target must be non-empty",
            ));
        }
        if !ALLOWED_RECEIPT_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!("unsupported receipt status {}", receipt.status),
            ));
        }
    }
}

fn validate_report(surface: &BootstrapExtinctionLedgerSurface, errors: &mut Vec<ValidationError>) {
    let entry_tuples: Vec<(String, String, String, String, String, Vec<String>)> = surface
        .entries
        .iter()
        .map(|entry| {
            (
                entry.id.clone(),
                entry.owner_root.clone(),
                entry.classification.clone(),
                entry.deletion_action.clone(),
                entry.ledger_state.clone(),
                entry.evidence.clone(),
            )
        })
        .collect();
    let gate_tuples: Vec<(String, String, String, String, String, Vec<String>, String)> = surface
        .gates
        .iter()
        .map(|gate| {
            (
                gate.id.clone(),
                gate.surface.clone(),
                gate.gate_kind.clone(),
                gate.trigger.clone(),
                gate.allowed_action.clone(),
                gate.evidence.clone(),
                gate.status.clone(),
            )
        })
        .collect();
    let receipt_tuples: Vec<(String, String, String, String)> = surface
        .receipts
        .iter()
        .map(|receipt| {
            (
                receipt.id.clone(),
                receipt.path.clone(),
                receipt.target.clone(),
                receipt.status.clone(),
            )
        })
        .collect();
    let report = deterministic_bootstrap_extinction_ledger_report(
        &entry_tuples,
        &gate_tuples,
        &receipt_tuples,
    );
    if report.entry_count != REQUIRED_BOOTSTRAP_EXTINCTION_ENTRIES.len() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "report.entry_count",
            "report must cover every P02-001 inventory surface",
        ));
    }
    if report.gate_count != REQUIRED_BOOTSTRAP_RETIREMENT_GATES.len() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "report.gate_count",
            "report must cover every retirement gate",
        ));
    }
    if report.receipt_count < REQUIRED_BOOTSTRAP_EXTINCTION_RECEIPTS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            "report.receipt_count",
            "report must bind required receipts",
        ));
    }
    if report.temporary_count == 0
        || report.observer_count == 0
        || report.bounded_permanent_count == 0
        || report.forbidden_count == 0
    {
        errors.push(ValidationError::reject(ErrorCode::InvalidClosureOutputGate, "report.classification_coverage", "ledger must cover temporary, observer, bounded_permanent, and forbidden classifications"));
    }
    if report.ledger_hash.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            "report.ledger_hash",
            "ledger hash must be deterministic and non-empty",
        ));
    }
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_EXTINCTION_TEXT {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden_text",
                format!("forbidden extinction phrase {needle}"),
            ));
        }
    }
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

fn parse_field_map(value: &str) -> Option<BTreeMap<String, String>> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        let (key, field_value) = segment.split_once(':')?;
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
            || fields
                .insert(key.to_string(), field_value.to_string())
                .is_some()
        {
            return None;
        }
    }
    Some(fields)
}

fn split_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && !value.starts_with('_')
        && !value.ends_with('_')
}
