use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_atom::deterministic_semantic_atom_core_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_atoms::{core_atom_descriptor, core_atom_ids};
use crate::p01_semantic_atom_model::{
    SemanticAtomBinding, SemanticAtomFamilyBinding, SemanticAtomReceiptBinding, SemanticAtomSurface,
};

pub const P01_SEMANTIC_ATOMS_CONTRACT: &str = "LYRA-P01-SEMANTIC-ATOMS v1";
pub const REQUIRED_SEMANTIC_ATOM_RULES: &[&str] = &[
    "all_core_atoms_declared",
    "atom_identity_must_be_canonical",
    "atom_equality_must_be_deterministic",
    "atom_normalization_must_be_explicit",
    "atom_serialization_must_be_byte_stable",
    "atom_owner_root_must_be_lyralang_interfaces_k0",
    "atom_registry_must_match_lyralang_core",
    "no_probabilistic_atom_truth",
    "no_network_dependency",
    "no_placeholder_atoms",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_ATOMS: &[&str] = &[
    "symbol",
    "value",
    "type",
    "effect",
    "capability",
    "proof",
    "receipt",
    "resource",
    "law",
];
pub const REQUIRED_SEMANTIC_ATOM_FAMILIES: &[&str] = &["core_atoms"];
pub const REQUIRED_SEMANTIC_ATOM_RECEIPTS: &[&str] = &["receipt_semantic_atoms"];

const ALLOWED_OWNER_ROOTS: &[&str] = &["lyralang", "interfaces", "k0"];
const ALLOWED_ATOM_STATUSES: &[&str] = &["admitted", "executable_seed", "contract_bound"];
const ALLOWED_FAMILY_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];
const ALLOWED_RECEIPT_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];

const FORBIDDEN_SEMANTIC_ATOM_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic atom truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "stochastic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("random truth", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("status:placeholder", ErrorCode::PlaceholderAllowed),
    ("placeholder atom allowed", ErrorCode::PlaceholderAllowed),
    ("stub atom", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_atom_surface(
    input: &str,
) -> Result<SemanticAtomSurface, Vec<ValidationError>> {
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
            "no semantic atom lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_ATOMS_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_ATOMS_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut atoms = Vec::new();
    let mut families = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_atoms = BTreeSet::new();
    let mut seen_families = BTreeSet::new();
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
                    "semantic atom rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("atom:") {
            if !is_symbolic_name(id) || !seen_atoms.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateCanonicalModel,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid atom id {id}"),
                ));
                continue;
            }
            match parse_atom_binding(line_number, id, value) {
                Ok(atom) => atoms.push(atom),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("family:") {
            if !is_symbolic_name(id) || !seen_families.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid atom family id {id}"),
                ));
                continue;
            }
            match parse_family_binding(line_number, id, value) {
                Ok(family) => families.push(family),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(id) || !seen_receipts.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateProofBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid atom receipt id {id}"),
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
                format!("unknown semantic atom entry {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticAtomSurface {
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
        atoms,
        families,
        receipts,
    })
}

pub fn validate_semantic_atom_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let parsed = parse_semantic_atom_surface(input);
    let mut errors = Vec::new();
    match parsed {
        Ok(surface) => {
            scan_forbidden_text(&canonical, &mut errors);
            validate_surface_scalars(&surface, &mut errors);
            require_rules(&surface, &mut errors);
            require_atoms(&surface, &mut errors);
            require_families(&surface, &mut errors);
            require_receipts(&surface, &mut errors);
            validate_atoms(&surface, &mut errors);
            validate_families(&surface, &mut errors);
            validate_receipts(&surface, &mut errors);
            validate_semantic_atom_report(&surface, &mut errors);
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

fn parse_atom_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticAtomBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            format!("line:{line_number:03}"),
            "invalid atom field map",
        )
    })?;
    let kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidCanonicalModel,
        line_number,
    )?;
    let owner_root = required_field(&fields, "owner", ErrorCode::MissingOwnerRoot, line_number)?;
    let canonical_name = required_field(
        &fields,
        "canonical",
        ErrorCode::InvalidCanonicalModel,
        line_number,
    )?;
    let identity_law = required_field(
        &fields,
        "identity",
        ErrorCode::MissingFieldBinding,
        line_number,
    )?;
    let equality_law = required_field(
        &fields,
        "equality",
        ErrorCode::MissingFieldBinding,
        line_number,
    )?;
    let normalization_law = required_field(
        &fields,
        "normalization",
        ErrorCode::MissingFieldBinding,
        line_number,
    )?;
    let serialization_law = required_field(
        &fields,
        "serialization",
        ErrorCode::MissingFieldBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(SemanticAtomBinding {
        line_number,
        id: id.to_string(),
        kind,
        owner_root,
        canonical_name,
        identity_law,
        equality_law,
        normalization_law,
        serialization_law,
        status,
    })
}

fn parse_family_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticAtomFamilyBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid family field map",
        )
    })?;
    let members = split_csv(&required_field(
        &fields,
        "members",
        ErrorCode::MissingModelBinding,
        line_number,
    )?);
    let phase = required_field(&fields, "phase", ErrorCode::InvalidPhase, line_number)?;
    let work_package =
        required_field(&fields, "work_package", ErrorCode::InvalidTask, line_number)?;
    let receipt = required_field(
        &fields,
        "receipt",
        ErrorCode::MissingProofBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(SemanticAtomFamilyBinding {
        line_number,
        id: id.to_string(),
        members,
        phase,
        work_package,
        receipt,
        status,
    })
}

fn parse_receipt_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticAtomReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            format!("line:{line_number:03}"),
            "invalid receipt field map",
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
    Ok(SemanticAtomReceiptBinding {
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

fn validate_surface_scalars(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01, got {}", surface.phase),
        ));
    }
    if surface.task != "P01-001" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-001, got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported semantic atom status {}", surface.status),
        ));
    }
}

fn require_rules(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_ATOM_RULES {
        if surface.rule_value(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "required semantic atom rule missing",
            ));
        }
    }
}

fn require_atoms(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_ATOMS {
        if surface.atom_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("atom:{required}"),
                "required semantic atom missing",
            ));
        }
    }
}

fn require_families(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_ATOM_FAMILIES {
        if surface.family_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("family:{required}"),
                "required semantic atom family missing",
            ));
        }
    }
}

fn require_receipts(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_ATOM_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "required semantic atom receipt missing",
            ));
        }
    }
}

fn validate_atoms(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    for atom in &surface.atoms {
        if !REQUIRED_SEMANTIC_ATOMS.contains(&atom.id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                atom.canonical_identity(),
                format!("unknown atom {}", atom.id),
            ));
        }
        if atom.kind != atom.id {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                atom.canonical_identity(),
                format!("atom kind {} must match id {}", atom.kind, atom.id),
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&atom.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                atom.canonical_identity(),
                format!("invalid atom owner root {}", atom.owner_root),
            ));
        }
        if !valid_canonical_name(&atom.canonical_name) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                atom.canonical_identity(),
                format!("invalid canonical atom name {}", atom.canonical_name),
            ));
        }
        if !is_symbolic_name(&atom.identity_law)
            || !is_symbolic_name(&atom.equality_law)
            || !is_symbolic_name(&atom.normalization_law)
            || !is_symbolic_name(&atom.serialization_law)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                atom.canonical_identity(),
                "atom laws must be symbolic canonical names",
            ));
        }
        if !ALLOWED_ATOM_STATUSES.contains(&atom.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                atom.canonical_identity(),
                format!("invalid atom status {}", atom.status),
            ));
        }
        if let Some(descriptor) = core_atom_descriptor(&atom.id) {
            if descriptor.kind != atom.kind.as_str()
                || descriptor.canonical_name != atom.canonical_name.as_str()
                || descriptor.identity_law != atom.identity_law.as_str()
                || descriptor.equality_law != atom.equality_law.as_str()
                || descriptor.normalization_law != atom.normalization_law.as_str()
                || descriptor.serialization_law != atom.serialization_law.as_str()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelDriftAccepted,
                    atom.canonical_identity(),
                    "atom row drifts from LyraLang core descriptor",
                ));
            }
        } else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                atom.canonical_identity(),
                "atom id is not in LyraLang core registry",
            ));
        }
    }
}

fn validate_families(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    for family in &surface.families {
        if family.phase != "P01" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidPhase,
                family.canonical_identity(),
                format!("family phase must be P01, got {}", family.phase),
            ));
        }
        if family.work_package != "P01-A" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTask,
                family.canonical_identity(),
                format!(
                    "family work package must be P01-A, got {}",
                    family.work_package
                ),
            ));
        }
        if family.members.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                family.canonical_identity(),
                "family must contain atom members",
            ));
        }
        let mut members = family.members.clone();
        members.sort();
        members.dedup();
        if members.len() != family.members.len() {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateModelBinding,
                family.canonical_identity(),
                "family contains duplicate atom members",
            ));
        }
        for required in core_atom_ids() {
            if !members.iter().any(|member| member.as_str() == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingModelBinding,
                    family.canonical_identity(),
                    format!("family missing atom {required}"),
                ));
            }
        }
        for member in &family.members {
            if surface.atom_by_id(member).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelUnbound,
                    family.canonical_identity(),
                    format!("unknown atom member {member}"),
                ));
            }
        }
        if surface.receipt_by_id(&family.receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                family.canonical_identity(),
                format!("unknown family receipt {}", family.receipt),
            ));
        }
        if !ALLOWED_FAMILY_STATUSES.contains(&family.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                family.canonical_identity(),
                format!("invalid family status {}", family.status),
            ));
        }
    }
}

fn validate_receipts(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p01/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                receipt.canonical_identity(),
                format!("receipt path must be a P01 receipt: {}", receipt.path),
            ));
        }
        if surface.atom_by_id(&receipt.target).is_none()
            && surface.family_by_id(&receipt.target).is_none()
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

fn validate_semantic_atom_report(surface: &SemanticAtomSurface, errors: &mut Vec<ValidationError>) {
    let atom_inputs: Vec<(
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
        .atoms
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.kind.clone(),
                item.owner_root.clone(),
                item.canonical_name.clone(),
                item.identity_law.clone(),
                item.equality_law.clone(),
                item.normalization_law.clone(),
                item.serialization_law.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let family_inputs: Vec<(String, Vec<String>, String, String, String)> = surface
        .families
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.members.clone(),
                item.phase.clone(),
                item.work_package.clone(),
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
    let report =
        deterministic_semantic_atom_core_report(&atom_inputs, &family_inputs, &receipt_inputs);
    if report.atom_count != surface.atoms.len()
        || report.family_count != surface.families.len()
        || report.receipt_count != surface.receipts.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_semantic_atom_report",
            "semantic atom report count mismatch",
        ));
    }
    if report.atom_count != REQUIRED_SEMANTIC_ATOMS.len()
        || report.family_count != REQUIRED_SEMANTIC_ATOM_FAMILIES.len()
        || report.receipt_count != REQUIRED_SEMANTIC_ATOM_RECEIPTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCanonicalModel,
            "k0_semantic_atom_report",
            "semantic atom report does not cover required P01-001 bedrock",
        ));
    }
    if report.lyralang_owned_count == 0
        || report.interface_owned_count == 0
        || report.k0_owned_count == 0
    {
        errors.push(ValidationError::reject(
            ErrorCode::MisplacedOwnerRoot,
            "k0_semantic_atom_report",
            "semantic atom report must exercise lyralang, interfaces, and k0 ownership",
        ));
    }
    if !report.core_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_semantic_atom_report",
            "semantic atom core hash must be stable fnv1a128",
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

fn split_csv(value: &str) -> Vec<String> {
    if value == "none" {
        Vec::new()
    } else {
        value
            .split(',')
            .filter(|item| !item.is_empty())
            .map(str::to_string)
            .collect()
    }
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}
fn valid_canonical_name(value: &str) -> bool {
    value.starts_with("lyra.")
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'.'
        })
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_ATOM_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic atom token {needle}"),
            ));
        }
    }
}
