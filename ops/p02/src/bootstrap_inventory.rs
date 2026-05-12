use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_inventory::deterministic_bootstrap_inventory_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_inventory_model::{BootstrapInventorySurface, BootstrapSurfaceBinding};

pub const P02_BOOTSTRAP_INVENTORY_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-SURFACE-INVENTORY v1";

pub const REQUIRED_BOOTSTRAP_INVENTORY_RULES: &[&str] = &[
    "all_foreign_bootstrap_surfaces_must_be_declared",
    "every_surface_must_bind_owner_root",
    "every_surface_must_bind_classification",
    "every_surface_must_bind_boundary",
    "every_surface_must_bind_visibility",
    "every_surface_must_bind_retirement_reference",
    "temporary_surfaces_must_point_to_retirement_law",
    "observer_surfaces_must_not_influence_truth",
    "bounded_permanent_surfaces_must_not_own_semantics",
    "forbidden_surfaces_must_be_blocked",
    "evidence_must_bind_each_surface",
    "no_ambient_network_dependency",
    "no_probabilistic_bootstrap_truth",
    "no_hidden_randomness",
    "no_placeholder_inventory",
    "no_global_phase_closure_claim",
];

pub const REQUIRED_BOOTSTRAP_SURFACES: &[&str] = &[
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

pub fn parse_bootstrap_inventory_surface(
    input: &str,
) -> Result<BootstrapInventorySurface, Vec<ValidationError>> {
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
            "bootstrap inventory surface is empty",
        )]);
    }
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_INVENTORY_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_INVENTORY_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut surfaces = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_surfaces = BTreeSet::new();

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
                    "bootstrap inventory rules must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(surface_id) = left.strip_prefix("surface:") {
            if !is_symbolic_name(surface_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid bootstrap surface identity {surface_id}"),
                ));
                continue;
            }
            if !seen_surfaces.insert(surface_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("surface:{surface_id}"),
                    "bootstrap surface identity must be unique",
                ));
                continue;
            }
            match parse_surface(line_number, surface_id, value) {
                Ok(item) => surfaces.push(item),
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
                format!("unknown bootstrap inventory field {left}"),
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
                "task=P02-001 is required",
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

    if errors.is_empty() {
        Ok(BootstrapInventorySurface {
            header,
            phase,
            task,
            status,
            rules,
            surfaces,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_bootstrap_inventory_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_bootstrap_inventory_surface(input) {
        Ok(surface) => errors.extend(validate_bootstrap_inventory_model(&surface).errors),
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

pub fn validate_bootstrap_inventory_model(surface: &BootstrapInventorySurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "bootstrap inventory must bind to P02",
        ));
    }
    if surface.task != "P02-001" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "bootstrap inventory must bind to P02-001",
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported bootstrap inventory status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_surfaces(surface, &mut errors);
    validate_surfaces(surface, &mut errors);
    validate_classification_coverage(surface, &mut errors);
    validate_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_surface(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapSurfaceBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "bootstrap surface fields must be key:value segments",
        )
    })?;
    Ok(BootstrapSurfaceBinding {
        line_number,
        id: id.to_string(),
        owner_root: required_field(
            &fields,
            "owner_root",
            ErrorCode::InvalidOwnerRoot,
            line_number,
        )?,
        surface_type: required_field(
            &fields,
            "surface_type",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        path: required_field(
            &fields,
            "path",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        role: required_field(
            &fields,
            "role",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        classification: required_field(
            &fields,
            "classification",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        boundary: required_field(
            &fields,
            "boundary",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        target: required_field(
            &fields,
            "target",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        visibility: required_field(
            &fields,
            "visibility",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        retirement_ref: required_field(
            &fields,
            "retirement_ref",
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

fn require_rules(surface: &BootstrapInventorySurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_BOOTSTRAP_INVENTORY_RULES {
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
                "required bootstrap inventory rule is absent",
            )),
        }
    }
}

fn require_surfaces(surface: &BootstrapInventorySurface, errors: &mut Vec<ValidationError>) {
    if surface.surfaces.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "surface:*",
            "at least one bootstrap surface is required",
        ));
    }
    for id in REQUIRED_BOOTSTRAP_SURFACES {
        if surface.surface_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("surface:{id}"),
                "required foreign bootstrap surface is absent",
            ));
        }
    }
}

fn validate_surfaces(surface: &BootstrapInventorySurface, errors: &mut Vec<ValidationError>) {
    let mut seen_paths = BTreeSet::new();
    for item in &surface.surfaces {
        if !allowed_owner_root(&item.owner_root) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                item.canonical_identity(),
                format!("invalid owner root {}", item.owner_root),
            ));
        }
        if !allowed_classification(&item.classification) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                item.canonical_identity(),
                format!("invalid classification {}", item.classification),
            ));
        }
        if !is_symbolic_name(&item.surface_type)
            || !is_symbolic_name(&item.role)
            || !is_symbolic_name(&item.boundary)
            || !is_symbolic_name(&item.target)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                item.canonical_identity(),
                "surface type role boundary and target must be symbolic names",
            ));
        }
        if item.visibility != "declared" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                item.canonical_identity(),
                "foreign surface visibility must be declared",
            ));
        }
        if !seen_paths.insert(item.path.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateClosureOutputGate,
                item.canonical_identity(),
                format!("duplicate path {}", item.path),
            ));
        }
        if item.evidence.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                item.canonical_identity(),
                "surface must bind at least one evidence id",
            ));
        }
        if !item
            .evidence
            .iter()
            .any(|evidence| evidence == "receipt_p02_bootstrap_inventory")
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                item.canonical_identity(),
                "surface must bind receipt_p02_bootstrap_inventory",
            ));
        }
        if item.is_temporary() {
            if item.retirement_ref != "P02-002" && item.retirement_ref != "P02-009" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    item.canonical_identity(),
                    "temporary surfaces must point to P02-002 or P02-009 retirement law",
                ));
            }
            if item.status != "inventoried" && item.status != "blocked_until_explicit_import" {
                errors.push(ValidationError::reject(
                    ErrorCode::UnsupportedClosureStatus,
                    item.canonical_identity(),
                    "temporary surfaces must be inventoried or blocked until explicit import",
                ));
            }
        }
        if item.is_observer() {
            if item.role != "external_observer" && item.role != "operator_handoff_observer" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    item.canonical_identity(),
                    "observer surfaces must use observer roles",
                ));
            }
            if item.boundary.contains("truth_owner") || item.role.contains("truth_owner") {
                errors.push(ValidationError::reject(
                    ErrorCode::AmbientAuthority,
                    item.canonical_identity(),
                    "observer surface must not influence Lyra truth",
                ));
            }
        }
        if item.is_bounded_permanent() {
            if item.role.contains("semantic_owner") || item.boundary.contains("semantic_owner") {
                errors.push(ValidationError::reject(
                    ErrorCode::RootOwnershipViolation,
                    item.canonical_identity(),
                    "bounded permanent foreign surfaces cannot own semantics",
                ));
            }
            if item.retirement_ref != "bounded_by_target_descriptor" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    item.canonical_identity(),
                    "bounded permanent surfaces must be bounded by target descriptor",
                ));
            }
        }
        if item.is_forbidden() {
            if !item.path.starts_with("forbidden:") || item.status != "forbidden_declared" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    item.canonical_identity(),
                    "forbidden surfaces must use forbidden path and forbidden_declared status",
                ));
            }
            if item.retirement_ref != "forbidden_surface_no_import" {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    item.canonical_identity(),
                    "forbidden surfaces must bind no-import law",
                ));
            }
        } else {
            let lower = format!(
                "{} {} {} {}",
                item.path, item.role, item.boundary, item.status
            )
            .to_ascii_lowercase();
            if lower.contains("network_required")
                || lower.contains("remote_truth")
                || lower.contains("download_runtime")
            {
                errors.push(ValidationError::reject(
                    ErrorCode::AmbientNetworkAllowed,
                    item.canonical_identity(),
                    "non-forbidden bootstrap surface cannot require network or remote truth",
                ));
            }
            if lower.contains("probabilistic_truth") || lower.contains("stochastic_truth") {
                errors.push(ValidationError::reject(
                    ErrorCode::ProbabilisticTruthAllowed,
                    item.canonical_identity(),
                    "bootstrap truth cannot be probabilistic",
                ));
            }
            if lower.contains("hidden_random") || lower.contains("rng") {
                errors.push(ValidationError::reject(
                    ErrorCode::HiddenRandomnessAllowed,
                    item.canonical_identity(),
                    "bootstrap boundary cannot hide randomness",
                ));
            }
        }
    }
}

fn validate_classification_coverage(
    surface: &BootstrapInventorySurface,
    errors: &mut Vec<ValidationError>,
) {
    let temporary = surface.temporary_surfaces().count();
    let observers = surface.observer_surfaces().count();
    let bounded = surface.bounded_permanent_surfaces().count();
    let forbidden = surface.forbidden_surfaces().count();
    if temporary == 0 {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "classification:temporary",
            "inventory must include temporary foreign surfaces",
        ));
    }
    if observers == 0 {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "classification:observer",
            "inventory must include observer foreign surfaces",
        ));
    }
    if bounded == 0 {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "classification:bounded_permanent",
            "inventory must include bounded permanent foreign surfaces",
        ));
    }
    if forbidden == 0 {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "classification:forbidden",
            "inventory must declare forbidden foreign surfaces",
        ));
    }
}

fn validate_report(surface: &BootstrapInventorySurface, errors: &mut Vec<ValidationError>) {
    let inputs: Vec<_> = surface
        .surfaces
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.owner_root.clone(),
                item.surface_type.clone(),
                item.path.clone(),
                item.role.clone(),
                item.classification.clone(),
                item.boundary.clone(),
                item.target.clone(),
                item.visibility.clone(),
                item.retirement_ref.clone(),
                item.evidence.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let report = deterministic_bootstrap_inventory_report(&inputs);
    if report.surface_count != surface.surfaces.len() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "report:surface_count",
            "report surface count drifted from parsed surface count",
        ));
    }
    if report.receipt_bound_count != surface.surfaces.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureProofUnbound,
            "report:receipt_bound_count",
            "every surface must have evidence",
        ));
    }
    if report.inventory_hash.is_empty() || !report.inventory_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            "report:inventory_hash",
            "inventory report hash must be stable FNV label",
        ));
    }
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for token in [
        "todo=true",
        "tbd=true",
        "placeholder=true",
        "stub=true",
        "note=placeholder",
        "status:placeholder",
    ] {
        if lower.contains(token) {
            errors.push(ValidationError::reject(
                ErrorCode::PlaceholderAllowed,
                "input",
                format!("placeholder token {token} is not allowed"),
            ));
        }
    }
    if lower.contains("global_closure=true")
        || lower.contains("phase_complete=true")
        || lower.contains("p02_closed=true")
    {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "input",
            "P02-001 cannot claim phase closure",
        ));
    }
    if lower.contains("docs_only") || lower.contains("documentation_only") {
        errors.push(ValidationError::reject(
            ErrorCode::DocsOnlyImplementation,
            "input",
            "bootstrap inventory cannot be docs-only",
        ));
    }
    if lower.contains("network_required=true") || lower.contains("remote_truth_required=true") {
        errors.push(ValidationError::reject(
            ErrorCode::AmbientNetworkAllowed,
            "input",
            "ambient network dependency is forbidden",
        ));
    }
    if lower.contains("probabilistic_truth=true") || lower.contains("stochastic_truth=true") {
        errors.push(ValidationError::reject(
            ErrorCode::ProbabilisticTruthAllowed,
            "input",
            "probabilistic truth is forbidden",
        ));
    }
    if lower.contains("hidden_randomness=true") || lower.contains("hidden_rng=true") {
        errors.push(ValidationError::reject(
            ErrorCode::HiddenRandomnessAllowed,
            "input",
            "hidden randomness is forbidden",
        ));
    }
}

fn parse_field_map(value: &str) -> Option<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();
    for segment in value.split('|') {
        let (key, segment_value) = segment.split_once(':')?;
        if key.is_empty()
            || segment_value.is_empty()
            || key != key.trim()
            || segment_value != segment_value.trim()
        {
            return None;
        }
        if map
            .insert(key.to_string(), segment_value.to_string())
            .is_some()
        {
            return None;
        }
    }
    Some(map)
}

fn required_field(
    fields: &BTreeMap<String, String>,
    name: &str,
    code: ErrorCode,
    line_number: usize,
) -> Result<String, ValidationError> {
    fields.get(name).cloned().ok_or_else(|| {
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
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn allowed_owner_root(value: &str) -> bool {
    matches!(
        value,
        "k0" | "k1"
            | "lyralang"
            | "interfaces"
            | "ops"
            | "shells"
            | "products"
            | "slices"
            | "android"
            | "web"
    )
}

fn allowed_classification(value: &str) -> bool {
    matches!(
        value,
        "temporary" | "observer" | "bounded_permanent" | "forbidden"
    )
}

fn is_symbolic_name(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_lowercase() || first.is_ascii_uppercase() || first == '_') {
        return false;
    }
    chars.all(|ch| {
        ch.is_ascii_lowercase()
            || ch.is_ascii_uppercase()
            || ch.is_ascii_digit()
            || ch == '_'
            || ch == '-'
    })
}
