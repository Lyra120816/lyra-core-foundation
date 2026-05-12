use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_canon_compliance_model::{
    CanonComplianceSurface, CanonSourceBinding, CanonValidationClaim, RoadmapBinding,
};

pub const P00_CANON_COMPLIANCE_CONTRACT: &str = "LYRA-P00-CANON-COMPLIANCE v1";

pub const REQUIRED_CANON_RULES: &[&str] = &[
    "single_file_master_required",
    "roadmap_phase_inventory_required",
    "roadmap_task_inventory_required",
    "authority_order_required",
    "frontier_lock_required",
    "truth_snapshot_required",
    "blocker_index_required",
    "receipt_chain_required",
    "roadmap_to_canon_binding_required",
    "drift_rejection_required",
    "archive_override_forbidden",
    "ambient_authority_forbidden",
];

pub const REQUIRED_CANON_SOURCES: &[&str] = &[
    "single_file_master",
    "p00_phase_inventory",
    "p00_task_inventory",
    "authority_order_contract",
    "frontier_lock",
    "truth_snapshot",
    "blocker_index",
    "receipt_chain",
];

pub const REQUIRED_ROADMAP_BINDINGS: &[&str] = &[
    "P00-001", "P00-002", "P00-003", "P00-004", "P00-005", "P00-006", "P00-007", "P00-008",
    "P00-009", "P00-010", "P00-011",
];

pub const REQUIRED_CANON_VALIDATIONS: &[&str] =
    &["roadmap_canon_chain", "p00_011_local", "p00_phase_open"];

const ALLOWED_SOURCE_KINDS: &[&str] = &["master", "roadmap", "contract", "control", "receipt"];
const ALLOWED_CLAIM_SCOPES: &[&str] = &["task", "phase", "roadmap", "canon"];
const ALLOWED_VALIDATION_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_BINDING_STATUSES: &[&str] = &[
    "admitted",
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_TASKS: &[&str] = &[
    "P00-001", "P00-002", "P00-003", "P00-004", "P00-005", "P00-006", "P00-007", "P00-008",
    "P00-009", "P00-010", "P00-011", "P00-012", "P00-013", "P00-014", "P00-015", "P00-016",
    "P00-017", "P00-018", "P00-019", "P00-020", "P00-021", "P00-022", "P00-023", "P00-024",
    "P00-X01", "P00-X02", "P00-X03", "P00-X04", "P00-X05",
];
const OWNER_ROOTS: &[&str] = &[
    "ops",
    "interfaces",
    "k0",
    "k1",
    "lyralang",
    "shells",
    "slices",
    "products",
    "android",
    "web",
    "fixtures",
    "goldens",
    "receipts",
    "tests",
    "src",
];

const FORBIDDEN_CANON_COMPLIANCE_TEXT: &[(&str, ErrorCode)] = &[
    (
        "archive overrides master",
        ErrorCode::ArchiveAuthorityOverride,
    ),
    ("archive primary", ErrorCode::ArchiveAuthorityOverride),
    ("ambient authority", ErrorCode::AmbientCanonAuthority),
    ("agent memory overrides", ErrorCode::AmbientCanonAuthority),
    (
        "operator override master",
        ErrorCode::AuthorityOrderViolation,
    ),
    ("drift accepted", ErrorCode::CanonDriftAccepted),
    ("roadmap mismatch allowed", ErrorCode::RoadmapCanonMismatch),
    ("unbound source", ErrorCode::CanonSourceUnbound),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_canon_compliance_surface(
    input: &str,
) -> Result<CanonComplianceSurface, Vec<ValidationError>> {
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
            "no canon-compliance lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_CANON_COMPLIANCE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_CANON_COMPLIANCE_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut sources = Vec::new();
    let mut roadmap_bindings = Vec::new();
    let mut validations = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_sources = BTreeSet::new();
    let mut seen_roadmap = BTreeSet::new();
    let mut seen_validations = BTreeSet::new();

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
                    "canon rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(source_id) = left.strip_prefix("source:") {
            if !is_symbolic_name(source_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidCanonSource,
                    format!("line:{line_number:03}"),
                    format!("invalid source identity {source_id}"),
                ));
                continue;
            }
            if !seen_sources.insert(source_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateCanonSource,
                    format!("source:{source_id}"),
                    "source identity must be unique",
                ));
                continue;
            }
            match parse_source(line_number, source_id, value) {
                Ok(source) => sources.push(source),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(binding_id) = left.strip_prefix("roadmap:") {
            if !is_task_identity(binding_id) && !is_closure_identity(binding_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRoadmapBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid roadmap binding identity {binding_id}"),
                ));
                continue;
            }
            if !seen_roadmap.insert(binding_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRoadmapBinding,
                    format!("roadmap:{binding_id}"),
                    "roadmap binding identity must be unique",
                ));
                continue;
            }
            match parse_roadmap_binding(line_number, binding_id, value) {
                Ok(binding) => roadmap_bindings.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(claim_id) = left.strip_prefix("validation:") {
            if !is_symbolic_name(claim_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidCanonValidationClaim,
                    format!("line:{line_number:03}"),
                    format!("invalid validation identity {claim_id}"),
                ));
                continue;
            }
            if !seen_validations.insert(claim_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateCanonValidationClaim,
                    format!("validation:{claim_id}"),
                    "canon validation identity must be unique",
                ));
                continue;
            }
            match parse_validation_claim(line_number, claim_id, value) {
                Ok(validation) => validations.push(validation),
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
                format!("unknown canon-compliance field {left}"),
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
                "task=P00-011 is required",
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
        Ok(CanonComplianceSurface {
            header,
            phase,
            task,
            status,
            rules,
            sources,
            roadmap_bindings,
            validations,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_canon_compliance_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = match canonical_surface_text(input) {
        Ok(text) => text,
        Err(error) => {
            let verdict = Verdict::rejected(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "byte-stream",
                format!("{error:?}"),
            )]);
            let receipt = build_receipt(input, "", verdict.clone());
            return (verdict, receipt);
        }
    };

    let verdict = match parse_canon_compliance_surface(input) {
        Ok(surface) => validate_parsed_canon_compliance_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_source(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CanonSourceBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let path = required_string_field(line_number, &mut fields, "path")?;
    let authority = required_string_field(line_number, &mut fields, "authority")?;
    let hash = required_string_field(line_number, &mut fields, "hash")?;
    let role = required_string_field(line_number, &mut fields, "role")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(CanonSourceBinding {
        line_number,
        id: id.to_string(),
        kind,
        path,
        authority,
        hash,
        role,
    })
}

fn parse_roadmap_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<RoadmapBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let phase = required_string_field(line_number, &mut fields, "phase")?;
    let task = required_string_field(line_number, &mut fields, "task")?;
    let source = required_string_field(line_number, &mut fields, "source")?;
    let owner_roots = required_list_field(line_number, &mut fields, "owner_roots")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(RoadmapBinding {
        line_number,
        id: id.to_string(),
        phase,
        task,
        source,
        owner_roots,
        status,
        receipts,
    })
}

fn parse_validation_claim(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CanonValidationClaim, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let sources = required_list_field(line_number, &mut fields, "sources")?;
    let roadmap_bindings = required_list_field(line_number, &mut fields, "roadmap")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(CanonValidationClaim {
        line_number,
        id: id.to_string(),
        scope,
        sources,
        roadmap_bindings,
        receipts,
        commands,
        status,
        forbids,
    })
}

fn validate_parsed_canon_compliance_surface(
    surface: &CanonComplianceSurface,
    raw_input: &str,
) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            "canon compliance law is scoped to P00",
        ));
    }
    if surface.task != "P00-011" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            "canon compliance law must bind P00-011",
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-011 may only claim working_slice in this pass",
        ));
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_CANON_COMPLIANCE_TEXT {
        if lowered.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "surface:text",
                format!("forbidden canon-compliance token: {token}"),
            ));
        }
    }

    for required in REQUIRED_CANON_RULES {
        match surface.rule_value(required) {
            Some(value) if value.starts_with("required:") || value.starts_with("forbidden:") => {}
            Some(_) => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonRule,
                format!("rule:{required}"),
                "canon rule must be explicit required: or forbidden:",
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingCanonRule,
                format!("rule:{required}"),
                "required canon-compliance rule missing",
            )),
        }
    }

    for required in REQUIRED_CANON_SOURCES {
        if surface.source_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonSource,
                format!("source:{required}"),
                "required canon source binding missing",
            ));
        }
    }

    for required in REQUIRED_ROADMAP_BINDINGS {
        if surface.roadmap_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRoadmapBinding,
                format!("roadmap:{required}"),
                "required roadmap task binding missing",
            ));
        }
    }

    for required in REQUIRED_CANON_VALIDATIONS {
        if surface.validation_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonValidationClaim,
                format!("validation:{required}"),
                "required canon validation claim missing",
            ));
        }
    }

    for source in &surface.sources {
        validate_source(source, &mut errors);
    }

    let source_ids: BTreeSet<String> = surface.sources.iter().map(|item| item.id.clone()).collect();
    let roadmap_ids: BTreeSet<String> = surface
        .roadmap_bindings
        .iter()
        .map(|item| item.id.clone())
        .collect();

    for binding in &surface.roadmap_bindings {
        validate_roadmap_binding(binding, &source_ids, &mut errors);
    }

    for claim in &surface.validations {
        validate_validation_claim(claim, &source_ids, &roadmap_ids, &mut errors);
    }

    match surface.source_by_id("single_file_master") {
        Some(source) if source.authority == "rank_000_master" => {}
        Some(_) => errors.push(ValidationError::reject(
            ErrorCode::MissingMasterAuthority,
            "source:single_file_master",
            "single-file master must carry rank_000_master authority",
        )),
        None => {}
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_source(source: &CanonSourceBinding, errors: &mut Vec<ValidationError>) {
    if !ALLOWED_SOURCE_KINDS.contains(&source.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonSource,
            source.canonical_identity(),
            format!("unknown source kind {}", source.kind),
        ));
    }
    if source.path.starts_with("http://")
        || source.path.starts_with("https://")
        || source.path.contains("cloud")
    {
        errors.push(ValidationError::reject(
            ErrorCode::AmbientCanonAuthority,
            source.canonical_identity(),
            "canon source path must be offline and repository-local",
        ));
    }
    if source.authority == "archive_primary" || source.authority == "archive_override" {
        errors.push(ValidationError::reject(
            ErrorCode::ArchiveAuthorityOverride,
            source.canonical_identity(),
            "archive material cannot override live canon",
        ));
    }
    if source.authority == "ambient"
        || source.authority == "agent_memory"
        || source.authority == "operator_override"
    {
        errors.push(ValidationError::reject(
            ErrorCode::AmbientCanonAuthority,
            source.canonical_identity(),
            "ambient or memory authority cannot serve as canon source",
        ));
    }
    if !(source.hash.starts_with("fnv1a128:") || source.hash.starts_with("sha256:")) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonSource,
            source.canonical_identity(),
            "source hash must be explicit fnv1a128 or sha256 label",
        ));
    }
    if source.role.is_empty() || source.role.contains("drift") && source.role.contains("accept") {
        errors.push(ValidationError::reject(
            ErrorCode::CanonDriftAccepted,
            source.canonical_identity(),
            "source role cannot admit canon drift",
        ));
    }
}

fn validate_roadmap_binding(
    binding: &RoadmapBinding,
    source_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    if binding.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRoadmapBinding,
            binding.canonical_identity(),
            "P00 roadmap binding must declare phase=P00",
        ));
    }
    if binding.task != binding.id {
        errors.push(ValidationError::reject(
            ErrorCode::RoadmapCanonMismatch,
            binding.canonical_identity(),
            "roadmap binding id and task must match exactly",
        ));
    }
    if !ALLOWED_TASKS.contains(&binding.task.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRoadmapBinding,
            binding.canonical_identity(),
            format!("unknown roadmap task {}", binding.task),
        ));
    }
    if !source_ids.contains(&binding.source) {
        errors.push(ValidationError::reject(
            ErrorCode::CanonSourceUnbound,
            binding.canonical_identity(),
            format!(
                "roadmap binding references unknown canon source {}",
                binding.source
            ),
        ));
    }
    if !ALLOWED_BINDING_STATUSES.contains(&binding.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRoadmapBinding,
            binding.canonical_identity(),
            format!("invalid roadmap status {}", binding.status),
        ));
    }
    if binding.owner_roots.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRoadmapBinding,
            binding.canonical_identity(),
            "roadmap binding requires at least one owner root",
        ));
    }
    for root in &binding.owner_roots {
        if !OWNER_ROOTS.contains(&root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                binding.canonical_identity(),
                format!("unknown owner root {root}"),
            ));
        }
    }
    if binding.receipts.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            binding.canonical_identity(),
            "roadmap binding must carry receipt evidence",
        ));
    }
    for receipt in &binding.receipts {
        if !receipt.starts_with("receipts/p00/") || !receipt.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                binding.canonical_identity(),
                format!("receipt path is not canonical: {receipt}"),
            ));
        }
    }
}

fn validate_validation_claim(
    claim: &CanonValidationClaim,
    source_ids: &BTreeSet<String>,
    roadmap_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    if !ALLOWED_CLAIM_SCOPES.contains(&claim.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonValidationClaim,
            claim.canonical_identity(),
            format!("invalid validation scope {}", claim.scope),
        ));
    }
    if !ALLOWED_VALIDATION_STATUSES.contains(&claim.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonValidationClaim,
            claim.canonical_identity(),
            format!("invalid validation status {}", claim.status),
        ));
    }
    if claim.receipts.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            claim.canonical_identity(),
            "validation claim must bind receipt evidence",
        ));
    }
    if claim.commands.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            claim.canonical_identity(),
            "validation claim must bind command records",
        ));
    }
    if claim.status == "execution_proven" && claim.receipts.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            claim.canonical_identity(),
            "execution-proven validation requires receipts",
        ));
    }
    for source in &claim.sources {
        if !source_ids.contains(source) {
            errors.push(ValidationError::reject(
                ErrorCode::CanonSourceUnbound,
                claim.canonical_identity(),
                format!("validation references unknown source {source}"),
            ));
        }
    }
    for binding in &claim.roadmap_bindings {
        if !roadmap_ids.contains(binding) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonValidationClaim,
                claim.canonical_identity(),
                format!("validation references unknown roadmap binding {binding}"),
            ));
        }
    }
    for receipt in &claim.receipts {
        if !receipt.starts_with("receipts/p00/") || !receipt.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                claim.canonical_identity(),
                format!("receipt path is not canonical: {receipt}"),
            ));
        }
    }
    if !claim.forbids.iter().any(|item| item == "archive_override") {
        errors.push(ValidationError::reject(
            ErrorCode::ArchiveAuthorityOverride,
            claim.canonical_identity(),
            "validation claim must explicitly forbid archive override",
        ));
    }
    if !claim.forbids.iter().any(|item| item == "ambient_authority") {
        errors.push(ValidationError::reject(
            ErrorCode::AmbientCanonAuthority,
            claim.canonical_identity(),
            "validation claim must explicitly forbid ambient authority",
        ));
    }
    if !claim
        .forbids
        .iter()
        .any(|item| item == "canon_drift_acceptance")
    {
        errors.push(ValidationError::reject(
            ErrorCode::CanonDriftAccepted,
            claim.canonical_identity(),
            "validation claim must explicitly forbid canon drift acceptance",
        ));
    }
    if claim.status == "blocked"
        && claim.id == "p00_phase_open"
        && !claim.forbids.iter().any(|item| item == "phase_closed")
    {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            claim.canonical_identity(),
            "phase-open validation must forbid phase closure",
        ));
    }
    if claim.status == "working_slice" && claim.id.contains("global") {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            claim.canonical_identity(),
            "global validation cannot be working_slice in P00-011",
        ));
    }
}

fn parse_fields(
    line_number: usize,
    value: &str,
) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        let Some((key, field_value)) = segment.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field segment must contain colon",
            ));
        };
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
        {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field key and value must be trimmed and non-empty",
            ));
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
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
    let items: Vec<String> = value
        .split(',')
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
        .collect();
    if items.is_empty() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("list field {key} must be non-empty"),
        ))
    } else {
        Ok(items)
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

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && matches!(value.bytes().next(), Some(byte) if byte.is_ascii_lowercase())
}

fn is_task_identity(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 7
        && &bytes[0..4] == b"P00-"
        && bytes[4].is_ascii_digit()
        && bytes[5].is_ascii_digit()
        && bytes[6].is_ascii_digit()
}

fn is_closure_identity(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 7
        && &bytes[0..5] == b"P00-X"
        && bytes[5].is_ascii_digit()
        && bytes[6].is_ascii_digit()
}
