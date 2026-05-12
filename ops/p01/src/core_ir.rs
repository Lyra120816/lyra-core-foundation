use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_core_ir::deterministic_core_ir_suite_report;
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_core_ir::{core_ir_descriptor, core_ir_ids};
use crate::lyralang_semantic_atoms::is_core_atom_id;
use crate::p01_core_ir_model::{
    CoreIrFormBinding, CoreIrParityBinding, CoreIrReceiptBinding, CoreIrSurface,
    CoreIrUpgradeBinding, CoreIrVersionBinding,
};

pub const P01_CORE_IR_CONTRACT: &str = "LYRA-P01-CORE-IR v1";
pub const REQUIRED_CORE_IR_RULES: &[&str] = &[
    "text_ir_header_byte_stable",
    "binary_ir_magic_byte_stable",
    "binary_ir_length_prefix_big_endian",
    "text_binary_round_trip_identity",
    "canonical_surface_sorting_stable",
    "version_edges_explicit",
    "unknown_required_fields_rejected",
    "forward_compatibility_profile_bound",
    "backward_compatibility_profile_bound",
    "no_network_dependency",
    "no_probabilistic_ir_truth",
    "no_placeholder_ir",
    "no_global_closure_claim",
];
pub const REQUIRED_CORE_IR_FORMS: &[&str] = &["text_ir", "binary_ir"];
pub const REQUIRED_CORE_IR_VERSIONS: &[&str] = &["ir_v1"];
pub const REQUIRED_CORE_IR_UPGRADES: &[&str] = &[
    "v1_identity",
    "v1_forward_reserved",
    "v1_backward_reject_unknown_required",
];
pub const REQUIRED_CORE_IR_PARITIES: &[&str] = &[
    "text_minimal_symbol",
    "binary_minimal_symbol",
    "roundtrip_symbol_value",
];
pub const REQUIRED_CORE_IR_RECEIPTS: &[&str] = &["receipt_core_ir"];

const ALLOWED_OWNER_ROOTS: &[&str] = &["lyralang", "interfaces", "k0"];
const ALLOWED_FORM_STATUSES: &[&str] = &["admitted", "contract_bound", "executable_seed"];
const ALLOWED_VERSION_STATUSES: &[&str] = &["admitted", "frozen"];
const ALLOWED_UPGRADE_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];
const ALLOWED_PARITY_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];
const ALLOWED_RECEIPT_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];

const FORBIDDEN_CORE_IR_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic ir truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic ir", ErrorCode::ProbabilisticTruthAllowed),
    ("random ir", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder ir", ErrorCode::PlaceholderAllowed),
    ("stub ir", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_core_ir_surface(input: &str) -> Result<CoreIrSurface, Vec<ValidationError>> {
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
            "no core ir lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_CORE_IR_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_CORE_IR_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut forms = Vec::new();
    let mut versions = Vec::new();
    let mut upgrades = Vec::new();
    let mut parities = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_forms = BTreeSet::new();
    let mut seen_versions = BTreeSet::new();
    let mut seen_upgrades = BTreeSet::new();
    let mut seen_parities = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();

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
                    "core ir rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("form:") {
            if !is_symbolic_name(id) || !seen_forms.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateCanonicalModel,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid core ir form id {id}"),
                ));
                continue;
            }
            match parse_form_binding(line_number, id, value) {
                Ok(form) => forms.push(form),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("version:") {
            if !is_symbolic_name(id) || !seen_versions.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid core ir version id {id}"),
                ));
                continue;
            }
            match parse_version_binding(line_number, id, value) {
                Ok(version) => versions.push(version),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("upgrade:") {
            if !is_symbolic_name(id) || !seen_upgrades.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid core ir upgrade id {id}"),
                ));
                continue;
            }
            match parse_upgrade_binding(line_number, id, value) {
                Ok(upgrade) => upgrades.push(upgrade),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("parity:") {
            if !is_symbolic_name(id) || !seen_parities.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid core ir parity id {id}"),
                ));
                continue;
            }
            match parse_parity_binding(line_number, id, value) {
                Ok(parity) => parities.push(parity),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(id) || !seen_receipts.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateProofBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid core ir receipt id {id}"),
                ));
                continue;
            }
            match parse_receipt_binding(line_number, id, value) {
                Ok(receipt) => receipts.push(receipt),
                Err(error) => errors.push(error),
            }
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
                format!("unknown core ir entry {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(CoreIrSurface {
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
        forms,
        versions,
        upgrades,
        parities,
        receipts,
    })
}

pub fn validate_core_ir_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let parsed = parse_core_ir_surface(input);
    let mut errors = Vec::new();
    match parsed {
        Ok(surface) => {
            scan_forbidden_text(&canonical, &mut errors);
            validate_surface_scalars(&surface, &mut errors);
            require_rules(&surface, &mut errors);
            require_forms(&surface, &mut errors);
            require_versions(&surface, &mut errors);
            require_upgrades(&surface, &mut errors);
            require_parities(&surface, &mut errors);
            require_receipts(&surface, &mut errors);
            validate_forms(&surface, &mut errors);
            validate_versions(&surface, &mut errors);
            validate_upgrades(&surface, &mut errors);
            validate_parities(&surface, &mut errors);
            validate_receipts(&surface, &mut errors);
            validate_core_ir_report(&surface, &mut errors);
        }
        Err(parse_errors) => errors.extend(parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn parse_form_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CoreIrFormBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            format!("line:{line_number:03}"),
            "invalid core ir form field map",
        )
    })?;
    let medium = required_field(
        &fields,
        "medium",
        ErrorCode::InvalidCanonicalModel,
        line_number,
    )?;
    let owner_root = required_field(&fields, "owner", ErrorCode::MissingOwnerRoot, line_number)?;
    let version = required_field(
        &fields,
        "version",
        ErrorCode::MissingModelBinding,
        line_number,
    )?;
    let header = required_field(
        &fields,
        "header",
        ErrorCode::InvalidCanonicalModel,
        line_number,
    )?;
    let extension = required_field(
        &fields,
        "extension",
        ErrorCode::InvalidCanonicalModel,
        line_number,
    )?;
    let encoding = required_field(
        &fields,
        "encoding",
        ErrorCode::InvalidFieldBinding,
        line_number,
    )?;
    let canonicalization = required_field(
        &fields,
        "canonicalization",
        ErrorCode::InvalidFieldBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(CoreIrFormBinding {
        line_number,
        id: id.to_string(),
        medium,
        owner_root,
        version,
        header,
        extension,
        encoding,
        canonicalization,
        status,
    })
}

fn parse_version_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CoreIrVersionBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid core ir version field map",
        )
    })?;
    let major = required_field(
        &fields,
        "major",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let minor = required_field(
        &fields,
        "minor",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let stability = required_field(
        &fields,
        "stability",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let upgrade_policy = required_field(
        &fields,
        "upgrade_policy",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(CoreIrVersionBinding {
        line_number,
        id: id.to_string(),
        major,
        minor,
        stability,
        upgrade_policy,
        status,
    })
}

fn parse_upgrade_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CoreIrUpgradeBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid core ir upgrade field map",
        )
    })?;
    let from_version =
        required_field(&fields, "from", ErrorCode::InvalidModelBinding, line_number)?;
    let to_version = required_field(&fields, "to", ErrorCode::InvalidModelBinding, line_number)?;
    let law = required_field(&fields, "law", ErrorCode::InvalidModelBinding, line_number)?;
    let compatibility = required_field(
        &fields,
        "compatibility",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(CoreIrUpgradeBinding {
        line_number,
        id: id.to_string(),
        from_version,
        to_version,
        law,
        compatibility,
        status,
    })
}

fn parse_parity_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CoreIrParityBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid core ir parity field map",
        )
    })?;
    let text_form = required_field(&fields, "text", ErrorCode::InvalidModelBinding, line_number)?;
    let binary_form = required_field(
        &fields,
        "binary",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let fixture = required_field(
        &fields,
        "fixture",
        ErrorCode::MissingFixtureProof,
        line_number,
    )?;
    let atom = required_field(
        &fields,
        "atom",
        ErrorCode::InvalidCanonicalModel,
        line_number,
    )?;
    let round_trip = required_field(
        &fields,
        "round_trip",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(CoreIrParityBinding {
        line_number,
        id: id.to_string(),
        text_form,
        binary_form,
        fixture,
        atom,
        round_trip,
        status,
    })
}

fn parse_receipt_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<CoreIrReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            format!("line:{line_number:03}"),
            "invalid core ir receipt field map",
        )
    })?;
    let path = required_field(&fields, "path", ErrorCode::InvalidProofBinding, line_number)?;
    let target = required_field(
        &fields,
        "target",
        ErrorCode::InvalidProofBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(CoreIrReceiptBinding {
        line_number,
        id: id.to_string(),
        path,
        target,
        status,
    })
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

fn validate_surface_scalars(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01, got {}", surface.phase),
        ));
    }
    if surface.task != "P01-002" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-002, got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported core ir status {}", surface.status),
        ));
    }
}

fn require_rules(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_CORE_IR_RULES {
        if surface.rule_value(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "required core ir rule missing",
            ));
        }
    }
}

fn require_forms(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_CORE_IR_FORMS {
        if surface.form_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("form:{required}"),
                "required core ir form missing",
            ));
        }
    }
}

fn require_versions(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_CORE_IR_VERSIONS {
        if surface.version_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("version:{required}"),
                "required core ir version missing",
            ));
        }
    }
}

fn require_upgrades(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_CORE_IR_UPGRADES {
        if surface.upgrade_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("upgrade:{required}"),
                "required core ir upgrade law missing",
            ));
        }
    }
}

fn require_parities(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_CORE_IR_PARITIES {
        if surface.parity_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingFixtureProof,
                format!("parity:{required}"),
                "required core ir parity fixture missing",
            ));
        }
    }
}

fn require_receipts(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_CORE_IR_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "required core ir receipt missing",
            ));
        }
    }
}

fn validate_forms(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for form in &surface.forms {
        if !REQUIRED_CORE_IR_FORMS.contains(&form.id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                form.canonical_identity(),
                format!("unknown core ir form {}", form.id),
            ));
        }
        if form.medium != "text" && form.medium != "binary" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                form.canonical_identity(),
                format!("invalid medium {}", form.medium),
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&form.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                form.canonical_identity(),
                format!("invalid form owner root {}", form.owner_root),
            ));
        }
        if surface.version_by_id(&form.version).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                form.canonical_identity(),
                format!("unknown form version {}", form.version),
            ));
        }
        if !is_symbolic_name(&form.extension)
            || !is_symbolic_name(&form.encoding)
            || !is_symbolic_name(&form.canonicalization)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                form.canonical_identity(),
                "form extension encoding and canonicalization must be symbolic names",
            ));
        }
        if !ALLOWED_FORM_STATUSES.contains(&form.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                form.canonical_identity(),
                format!("invalid form status {}", form.status),
            ));
        }
        if let Some(descriptor) = core_ir_descriptor(&form.id) {
            if descriptor.medium != form.medium.as_str()
                || descriptor.version != form.version.as_str()
                || descriptor.canonical_header != form.header.as_str()
                || descriptor.extension != form.extension.as_str()
                || descriptor.encoding_law != form.encoding.as_str()
                || descriptor.canonicalization_law != form.canonicalization.as_str()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelDriftAccepted,
                    form.canonical_identity(),
                    "form row drifts from LyraLang core IR descriptor",
                ));
            }
        } else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                form.canonical_identity(),
                "form id is not in LyraLang core IR registry",
            ));
        }
    }
}

fn validate_versions(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for version in &surface.versions {
        if version.id != "ir_v1" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                version.canonical_identity(),
                format!("unsupported version {}", version.id),
            ));
        }
        if version.major != "1" || version.minor != "0" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                version.canonical_identity(),
                "P01-002 admits only ir_v1 major 1 minor 0",
            ));
        }
        if version.stability != "frozen" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                version.canonical_identity(),
                format!(
                    "version stability must be frozen, got {}",
                    version.stability
                ),
            ));
        }
        if version.upgrade_policy != "explicit_version_edge" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                version.canonical_identity(),
                format!("invalid upgrade policy {}", version.upgrade_policy),
            ));
        }
        if !ALLOWED_VERSION_STATUSES.contains(&version.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                version.canonical_identity(),
                format!("invalid version status {}", version.status),
            ));
        }
    }
}

fn validate_upgrades(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for upgrade in &surface.upgrades {
        if surface.version_by_id(&upgrade.from_version).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                upgrade.canonical_identity(),
                format!("unknown from version {}", upgrade.from_version),
            ));
        }
        if surface.version_by_id(&upgrade.to_version).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                upgrade.canonical_identity(),
                format!("unknown to version {}", upgrade.to_version),
            ));
        }
        if !is_symbolic_name(&upgrade.law) || !is_symbolic_name(&upgrade.compatibility) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                upgrade.canonical_identity(),
                "upgrade law and compatibility must be symbolic names",
            ));
        }
        match upgrade.id.as_str() {
            "v1_identity" => {
                if upgrade.law != "identity_no_rewrite"
                    || upgrade.compatibility != "forward_backward_equal"
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::CanonicalModelDriftAccepted,
                        upgrade.canonical_identity(),
                        "v1 identity upgrade law drift",
                    ));
                }
            }
            "v1_forward_reserved" => {
                if upgrade.law != "reserved_fields_rejected_until_declared"
                    || upgrade.compatibility != "forward_reject_unknown_required"
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::CanonicalModelDriftAccepted,
                        upgrade.canonical_identity(),
                        "v1 forward compatibility law drift",
                    ));
                }
            }
            "v1_backward_reject_unknown_required" => {
                if upgrade.law != "unknown_required_field_rejected"
                    || upgrade.compatibility != "backward_reject_unknown_required"
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::CanonicalModelDriftAccepted,
                        upgrade.canonical_identity(),
                        "v1 backward compatibility law drift",
                    ));
                }
            }
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                upgrade.canonical_identity(),
                format!("unknown required upgrade law {}", upgrade.id),
            )),
        }
        if !ALLOWED_UPGRADE_STATUSES.contains(&upgrade.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                upgrade.canonical_identity(),
                format!("invalid upgrade status {}", upgrade.status),
            ));
        }
    }
}

fn validate_parities(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for parity in &surface.parities {
        if surface
            .form_by_id(&parity.text_form)
            .map(|form| form.medium.as_str())
            != Some("text")
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                parity.canonical_identity(),
                format!("unknown text form {}", parity.text_form),
            ));
        }
        if surface
            .form_by_id(&parity.binary_form)
            .map(|form| form.medium.as_str())
            != Some("binary")
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                parity.canonical_identity(),
                format!("unknown binary form {}", parity.binary_form),
            ));
        }
        if !parity.fixture.starts_with("fixtures/p01/core_ir_inputs/") {
            errors.push(ValidationError::reject(
                ErrorCode::MissingFixtureProof,
                parity.canonical_identity(),
                format!("invalid parity fixture path {}", parity.fixture),
            ));
        }
        if !is_core_atom_id(&parity.atom) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                parity.canonical_identity(),
                format!("unknown semantic atom {}", parity.atom),
            ));
        }
        if !is_symbolic_name(&parity.round_trip) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                parity.canonical_identity(),
                format!("invalid round trip law {}", parity.round_trip),
            ));
        }
        if !ALLOWED_PARITY_STATUSES.contains(&parity.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                parity.canonical_identity(),
                format!("invalid parity status {}", parity.status),
            ));
        }
    }
}

fn validate_receipts(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p01/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                receipt.canonical_identity(),
                format!("receipt path must be a P01 receipt: {}", receipt.path),
            ));
        }
        if receipt.target != "core_ir_forms"
            && surface.form_by_id(&receipt.target).is_none()
            && surface.version_by_id(&receipt.target).is_none()
            && surface.upgrade_by_id(&receipt.target).is_none()
            && surface.parity_by_id(&receipt.target).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                receipt.canonical_identity(),
                format!("unknown receipt target {}", receipt.target),
            ));
        }
        if !ALLOWED_RECEIPT_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!("invalid receipt status {}", receipt.status),
            ));
        }
    }
}

fn validate_core_ir_report(surface: &CoreIrSurface, errors: &mut Vec<ValidationError>) {
    let form_inputs: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .forms
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.medium.clone(),
                item.owner_root.clone(),
                item.version.clone(),
                item.header.clone(),
                item.extension.clone(),
                item.encoding.clone(),
                item.canonicalization.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let version_inputs: Vec<(String, String, String, String, String, String)> = surface
        .versions
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.major.clone(),
                item.minor.clone(),
                item.stability.clone(),
                item.upgrade_policy.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let upgrade_inputs: Vec<(String, String, String, String, String, String)> = surface
        .upgrades
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.from_version.clone(),
                item.to_version.clone(),
                item.law.clone(),
                item.compatibility.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let parity_inputs: Vec<(String, String, String, String, String, String, String)> = surface
        .parities
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.text_form.clone(),
                item.binary_form.clone(),
                item.fixture.clone(),
                item.atom.clone(),
                item.round_trip.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let receipt_inputs: Vec<(String, String, String, String)> = surface
        .receipts
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.path.clone(),
                item.target.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let report = deterministic_core_ir_suite_report(
        &form_inputs,
        &version_inputs,
        &upgrade_inputs,
        &parity_inputs,
        &receipt_inputs,
    );
    if report.form_count != surface.forms.len()
        || report.version_count != surface.versions.len()
        || report.upgrade_count != surface.upgrades.len()
        || report.parity_count != surface.parities.len()
        || report.receipt_count != surface.receipts.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_core_ir_report",
            "core ir report count mismatch",
        ));
    }
    if report.form_count != REQUIRED_CORE_IR_FORMS.len()
        || report.version_count != REQUIRED_CORE_IR_VERSIONS.len()
        || report.upgrade_count != REQUIRED_CORE_IR_UPGRADES.len()
        || report.parity_count != REQUIRED_CORE_IR_PARITIES.len()
        || report.receipt_count != REQUIRED_CORE_IR_RECEIPTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCanonicalModel,
            "k0_core_ir_report",
            "core ir report does not cover required P01-002 bedrock",
        ));
    }
    if report.text_form_count == 0 || report.binary_form_count == 0 || report.admitted_count == 0 {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_core_ir_report",
            "core ir report must include text, binary, and admitted forms",
        ));
    }
    for required in core_ir_ids() {
        if !surface.forms.iter().any(|form| form.id == required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                "k0_core_ir_report",
                format!("missing LyraLang core IR descriptor {required}"),
            ));
        }
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_core_ir_report",
            "core ir suite hash must be stable fnv1a128",
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

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_CORE_IR_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden core ir token {needle}"),
            ));
        }
    }
}
