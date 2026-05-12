use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_owner_root_model::{
    OwnerRootBinding, OwnerRootClaim, OwnerRootLawSurface, RootResponsibility,
};

pub const P00_OWNER_ROOT_LAW_CONTRACT: &str = "LYRA-P00-OWNER-ROOT-LAW v1";

pub const REQUIRED_OWNER_ROOT_RULES: &[&str] = &[
    "owner_root_classification_required",
    "root_specific_responsibility_required",
    "production_logic_owner_root_required",
    "control_plane_mirror_only",
    "platform_root_bounded_adapter_only",
    "product_root_composition_only",
    "evidence_binding_required",
    "no_empty_active_root",
    "no_misplaced_logic",
    "reserved_root_no_empty_shell",
];

pub const REQUIRED_OWNER_ROOTS: &[&str] = &[
    "k0",
    "k1",
    "lyralang",
    "shells",
    "ops",
    "interfaces",
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

pub const ACTIVE_OWNER_ROOTS_FOR_P00_008: &[&str] = &[
    "k0",
    "ops",
    "interfaces",
    "fixtures",
    "goldens",
    "receipts",
    "tests",
    "src",
];

pub struct OwnerRootProfile {
    pub id: &'static str,
    pub domain: &'static str,
    pub required_owns: &'static [&'static str],
    pub required_forbids: &'static [&'static str],
    pub allowed_statuses: &'static [&'static str],
}

pub const OWNER_ROOT_PROFILES: &[OwnerRootProfile] = &[
    OwnerRootProfile {
        id: "k0",
        domain: "determinism_substrate",
        required_owns: &["canonicalization", "hashing", "receipts", "replay_law"],
        required_forbids: &[
            "probabilistic_reasoning",
            "ambient_network",
            "hidden_randomness",
        ],
        allowed_statuses: &["active"],
    },
    OwnerRootProfile {
        id: "k1",
        domain: "symbolic_cognition",
        required_owns: &[
            "symbolic_memory",
            "reasoning",
            "planning",
            "simulation",
            "retrieval",
        ],
        required_forbids: &[
            "probabilistic_reasoning",
            "cloud_truth_path",
            "llm_truth_path",
        ],
        allowed_statuses: &["reserved", "active"],
    },
    OwnerRootProfile {
        id: "lyralang",
        domain: "symbolic_language",
        required_owns: &["parser", "semantic_core", "ir", "evaluator", "compiler"],
        required_forbids: &[
            "foreign_semantic_ownership",
            "stochastic_language_semantics",
            "hidden_network_dependency",
        ],
        allowed_statuses: &["reserved", "active"],
    },
    OwnerRootProfile {
        id: "shells",
        domain: "operator_interaction",
        required_owns: &["cli", "voice_shell", "device_shell", "operator_workflow"],
        required_forbids: &[
            "core_truth_ownership",
            "probabilistic_reasoning",
            "silent_side_effects",
        ],
        allowed_statuses: &["reserved", "active"],
    },
    OwnerRootProfile {
        id: "ops",
        domain: "truth_plane_operations",
        required_owns: &[
            "frontier_lock",
            "truth_snapshot",
            "blocker_index",
            "receipts",
            "release_evidence",
        ],
        required_forbids: &[
            "determinism_substrate",
            "symbolic_cognition",
            "language_semantics",
        ],
        allowed_statuses: &["active"],
    },
    OwnerRootProfile {
        id: "interfaces",
        domain: "contracts_boundaries",
        required_owns: &[
            "schemas",
            "contracts",
            "ffi_boundaries",
            "versioned_surfaces",
        ],
        required_forbids: &[
            "runtime_truth_execution",
            "hidden_semantics",
            "unversioned_boundary",
        ],
        allowed_statuses: &["active"],
    },
    OwnerRootProfile {
        id: "slices",
        domain: "end_to_end_value_slices",
        required_owns: &["vertical_slice", "integration_path", "proof_path"],
        required_forbids: &["core_truth_ownership", "orphan_demo", "unreceipted_flow"],
        allowed_statuses: &["reserved", "active"],
    },
    OwnerRootProfile {
        id: "products",
        domain: "product_composition",
        required_owns: &["packaging", "distribution_surface", "user_composition"],
        required_forbids: &[
            "core_truth_ownership",
            "kernel_semantics",
            "language_semantics",
        ],
        allowed_statuses: &["reserved", "active"],
    },
    OwnerRootProfile {
        id: "android",
        domain: "bounded_platform_surface",
        required_owns: &[
            "bounded_adapter",
            "home_replacement_surface",
            "platform_bridge",
        ],
        required_forbids: &[
            "core_truth_ownership",
            "hidden_network_dependency",
            "platform_drift",
        ],
        allowed_statuses: &["reserved", "active"],
    },
    OwnerRootProfile {
        id: "web",
        domain: "bounded_platform_surface",
        required_owns: &["bounded_adapter", "ui_surface", "platform_bridge"],
        required_forbids: &[
            "core_truth_ownership",
            "hidden_network_dependency",
            "browser_truth_ownership",
        ],
        allowed_statuses: &["reserved", "active"],
    },
    OwnerRootProfile {
        id: "fixtures",
        domain: "proof_input_corpus",
        required_owns: &["positive_corpus", "negative_corpus", "malformed_corpus"],
        required_forbids: &[
            "production_runtime",
            "truth_claim_without_validator",
            "decorative_sample",
        ],
        allowed_statuses: &["active"],
    },
    OwnerRootProfile {
        id: "goldens",
        domain: "canonical_expected_outputs",
        required_owns: &["stable_expected_output", "byte_golden", "canonical_receipt"],
        required_forbids: &[
            "unverified_output",
            "ambient_regeneration",
            "manual_expectation",
        ],
        allowed_statuses: &["active"],
    },
    OwnerRootProfile {
        id: "receipts",
        domain: "proof_receipts",
        required_owns: &["pass_receipt", "replay_witness", "truth_binding"],
        required_forbids: &[
            "unhashed_claim",
            "closure_without_receipt",
            "manual_attestation",
        ],
        allowed_statuses: &["active"],
    },
    OwnerRootProfile {
        id: "tests",
        domain: "proof_execution",
        required_owns: &["unit_tests", "contract_tests", "negative_tests"],
        required_forbids: &["compile_only_claim", "untested_runtime", "decorative_test"],
        allowed_statuses: &["active"],
    },
    OwnerRootProfile {
        id: "src",
        domain: "crate_wiring",
        required_owns: &["binary_entrypoint", "module_exports", "operator_invocation"],
        required_forbids: &[
            "hidden_core_semantics",
            "unowned_runtime_logic",
            "ambient_dependency",
        ],
        allowed_statuses: &["active"],
    },
];

const RESPONSIBILITY_KINDS: &[&str] = &[
    "runtime", "model", "contract", "fixture", "golden", "receipt", "test", "cli", "wiring",
];
const CLAIM_STATUSES: &[&str] = &["working_slice", "artifact_emitted", "execution_proven"];

const FORBIDDEN_OWNER_ROOT_TEXT: &[(&str, ErrorCode)] = &[
    ("todo", ErrorCode::ForbiddenToken),
    ("tbd", ErrorCode::ForbiddenToken),
    ("not implemented", ErrorCode::ForbiddenToken),
    ("will add later", ErrorCode::ForbiddenToken),
    ("finish later", ErrorCode::ForbiddenToken),
    ("empty root", ErrorCode::EmptyImplementation),
    ("docs only", ErrorCode::DocsOnlyImplementation),
    (
        "control plane owns runtime",
        ErrorCode::RootOwnershipViolation,
    ),
    ("product owns core", ErrorCode::ProductRootCoreOwnership),
    ("unbounded platform", ErrorCode::PlatformRootUnbounded),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_owner_root_law_surface(
    input: &str,
) -> Result<OwnerRootLawSurface, Vec<ValidationError>> {
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
            "no owner-root law lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_OWNER_ROOT_LAW_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_OWNER_ROOT_LAW_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut roots = Vec::new();
    let mut responsibilities = Vec::new();
    let mut claims = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_roots = BTreeSet::new();
    let mut seen_responsibilities = BTreeSet::new();
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
                    "owner-root rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(root_id) = left.strip_prefix("root:") {
            if !is_root_identity(root_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidOwnerRootBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid owner root identity {root_id}"),
                ));
                continue;
            }
            if !seen_roots.insert(root_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateOwnerRootBinding,
                    format!("root:{root_id}"),
                    "owner-root identity must be unique",
                ));
                continue;
            }
            match parse_root_binding(line_number, root_id, value) {
                Ok(root) => roots.push(root),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(responsibility_id) = left.strip_prefix("responsibility:") {
            if !is_symbolic_name(responsibility_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRootResponsibility,
                    format!("line:{line_number:03}"),
                    format!("invalid root responsibility identity {responsibility_id}"),
                ));
                continue;
            }
            if !seen_responsibilities.insert(responsibility_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRootResponsibility,
                    format!("responsibility:{responsibility_id}"),
                    "root responsibility identity must be unique",
                ));
                continue;
            }
            match parse_responsibility(line_number, responsibility_id, value) {
                Ok(responsibility) => responsibilities.push(responsibility),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(claim_id) = left.strip_prefix("claim:") {
            if !is_symbolic_name(claim_id) || !seen_claims.insert(claim_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "owner-root claim identity must be symbolic and unique",
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
                format!("unknown owner-root field {left}"),
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
                "task=P00-008 is required",
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
        Ok(OwnerRootLawSurface {
            header,
            phase,
            task,
            status,
            rules,
            roots,
            responsibilities,
            claims,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_owner_root_law_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_owner_root_law_surface(input) {
        Ok(surface) => validate_parsed_owner_root_law_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_root_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<OwnerRootBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let domain = required_string_field(line_number, &mut fields, "domain")?;
    let owns = required_list_field(line_number, &mut fields, "owns")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let evidence = required_list_field(line_number, &mut fields, "evidence")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(OwnerRootBinding {
        line_number,
        id: id.to_string(),
        domain,
        owns,
        forbids,
        status,
        evidence,
    })
}

fn parse_responsibility(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<RootResponsibility, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let owner_root = required_string_field(line_number, &mut fields, "owner_root")?;
    let path = required_string_field(line_number, &mut fields, "path")?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let behavior = required_string_field(line_number, &mut fields, "behavior")?;
    let proof = required_string_field(line_number, &mut fields, "proof")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(RootResponsibility {
        line_number,
        id: id.to_string(),
        owner_root,
        path,
        kind,
        behavior,
        proof,
        status,
    })
}

fn parse_claim(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<OwnerRootClaim, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let roots = required_list_field(line_number, &mut fields, "roots")?;
    let responsibilities = required_list_field(line_number, &mut fields, "responsibilities")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(OwnerRootClaim {
        line_number,
        id: id.to_string(),
        scope,
        status,
        roots,
        responsibilities,
        receipts,
        commands,
    })
}

fn parse_fields(
    line_number: usize,
    value: &str,
) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut fields = BTreeMap::new();
    for raw_part in value.split(';') {
        let part = raw_part.trim();
        if part.is_empty() {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "empty field segment is not allowed",
            ));
        }
        let Some((key, field_value)) = part.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field segment must use key:value syntax",
            ));
        };
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
            || !is_symbolic_name(key)
        {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field key/value must be non-empty canonical tokens",
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
            "owner-root surface contains unsupported attributes",
        ))
    }
}

fn validate_parsed_owner_root_law_surface(
    surface: &OwnerRootLawSurface,
    raw_input: &str,
) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            format!("expected P00, found {}", surface.phase),
        ));
    }
    if surface.task != "P00-008" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-008, found {}", surface.task),
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-008 may only declare working_slice status",
        ));
    }

    for rule in REQUIRED_OWNER_ROOT_RULES {
        match surface.rule_value(rule) {
            Some(value) if value.contains("required") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingOwnerRootRule,
                format!("rule:{rule}"),
                format!("rule must contain required, found {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingOwnerRootRule,
                format!("rule:{rule}"),
                "required owner-root rule missing",
            )),
        }
    }

    let root_ids: BTreeSet<String> = surface.roots.iter().map(|root| root.id.clone()).collect();
    for required in REQUIRED_OWNER_ROOTS {
        if !root_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingOwnerRootBinding,
                format!("root:{required}"),
                "required owner root is absent",
            ));
        }
    }

    for root in &surface.roots {
        validate_root_binding(root, &mut errors);
    }

    let responsibility_ids: BTreeSet<String> = surface
        .responsibilities
        .iter()
        .map(|responsibility| responsibility.id.clone())
        .collect();
    let mut active_responsibility_roots = BTreeSet::new();
    for responsibility in &surface.responsibilities {
        validate_responsibility(
            responsibility,
            &root_ids,
            surface,
            &mut active_responsibility_roots,
            &mut errors,
        );
    }
    for required_active_root in ACTIVE_OWNER_ROOTS_FOR_P00_008 {
        if !active_responsibility_roots.contains(*required_active_root) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRootResponsibility,
                format!("root:{required_active_root}"),
                "active owner root lacks an executable responsibility binding",
            ));
        }
    }

    if surface.claims.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            "claim:*",
            "owner-root law must declare a truthful claim",
        ));
    }
    for claim in &surface.claims {
        validate_claim(claim, &root_ids, &responsibility_ids, &mut errors);
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_OWNER_ROOT_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "owner_root:text",
                format!("forbidden owner-root phrase detected: {needle}"),
            ));
        }
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_root_binding(root: &OwnerRootBinding, errors: &mut Vec<ValidationError>) {
    let location = root.canonical_identity();
    let Some(profile) = profile_for(&root.id) else {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidOwnerRootBinding,
            location,
            "unknown owner root identity",
        ));
        return;
    };

    if root.domain != profile.domain {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidOwnerRootBinding,
            location.clone(),
            format!(
                "root domain must be {}, found {}",
                profile.domain, root.domain
            ),
        ));
    }
    if !profile.allowed_statuses.contains(&root.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidOwnerRootBinding,
            location.clone(),
            format!("unsupported owner root status {}", root.status),
        ));
    }
    for required in profile.required_owns {
        if !root.owns.iter().any(|owned| owned == required) {
            errors.push(ValidationError::reject(
                ErrorCode::RootOwnershipViolation,
                location.clone(),
                format!("root {} must own {required}", root.id),
            ));
        }
    }
    for required in profile.required_forbids {
        if !root.forbids.iter().any(|forbidden| forbidden == required) {
            errors.push(ValidationError::reject(
                ErrorCode::RootOwnershipViolation,
                location.clone(),
                format!("root {} must forbid {required}", root.id),
            ));
        }
    }
    if root.owns.iter().any(|item| item == "placeholder")
        || root.forbids.iter().any(|item| item == "placeholder")
    {
        errors.push(ValidationError::reject(
            ErrorCode::PlaceholderAllowed,
            location.clone(),
            "owner root owns/forbids lists must not contain placeholder language",
        ));
    }
    if root.owns.iter().any(|item| weak_owner_value(item))
        || root.forbids.iter().any(|item| weak_owner_value(item))
    {
        errors.push(ValidationError::reject(
            ErrorCode::RootOwnershipViolation,
            location.clone(),
            "owner root owns/forbids lists must bind concrete responsibilities",
        ));
    }
    if root.status == "active"
        && !root
            .evidence
            .iter()
            .any(|path| path_starts_with_root(path, &root.id))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingRequiredEvidence,
            location.clone(),
            format!(
                "active root {} must bind evidence under its owner root",
                root.id
            ),
        ));
    }
    if root.status == "reserved"
        && !root
            .evidence
            .iter()
            .any(|path| path == "interfaces/p00/contracts/owner_root_law.v1.lyra")
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingRequiredEvidence,
            location.clone(),
            "reserved root must bind the owner-root law contract as evidence",
        ));
    }
    if (root.id == "android" || root.id == "web")
        && (!root.owns.iter().any(|owned| owned == "bounded_adapter")
            || !root
                .forbids
                .iter()
                .any(|forbidden| forbidden == "core_truth_ownership"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::PlatformRootUnbounded,
            location.clone(),
            "platform roots must be bounded adapters and forbid core truth ownership",
        ));
    }
    if root.id == "products" && owns_core_semantics(&root.owns) {
        errors.push(ValidationError::reject(
            ErrorCode::ProductRootCoreOwnership,
            location.clone(),
            "products root may compose but must not own Lyra core semantics",
        ));
    }
    if root.id == "ops"
        && root.owns.iter().any(|owned| {
            matches!(
                owned.as_str(),
                "determinism_substrate"
                    | "symbolic_cognition"
                    | "language_semantics"
                    | "core_runtime"
            )
        })
    {
        errors.push(ValidationError::reject(
            ErrorCode::RootOwnershipViolation,
            location.clone(),
            "ops may operate truth-plane control but must not own core runtime semantics",
        ));
    }
}

fn validate_responsibility(
    responsibility: &RootResponsibility,
    root_ids: &BTreeSet<String>,
    surface: &OwnerRootLawSurface,
    active_responsibility_roots: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = responsibility.canonical_identity();
    if !root_ids.contains(&responsibility.owner_root) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRootResponsibility,
            location.clone(),
            format!("unknown owner root {}", responsibility.owner_root),
        ));
        return;
    }
    let Some(root) = surface.root_by_id(&responsibility.owner_root) else {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRootResponsibility,
            location.clone(),
            format!("missing root {}", responsibility.owner_root),
        ));
        return;
    };
    if root.status != "active" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRootResponsibility,
            location.clone(),
            "reserved roots must not carry implementation responsibility bindings in this slice",
        ));
    }
    if !path_starts_with_root(&responsibility.path, &responsibility.owner_root) {
        errors.push(ValidationError::reject(
            ErrorCode::MisplacedOwnerRoot,
            location.clone(),
            format!(
                "path {} is outside owner root {}",
                responsibility.path, responsibility.owner_root
            ),
        ));
    }
    if !RESPONSIBILITY_KINDS.contains(&responsibility.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRootResponsibility,
            location.clone(),
            format!("unsupported responsibility kind {}", responsibility.kind),
        ));
    }
    if responsibility.status != "active" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRootResponsibility,
            location.clone(),
            format!(
                "responsibility status must be active, found {}",
                responsibility.status
            ),
        ));
    }
    if weak_owner_value(&responsibility.behavior) || weak_owner_value(&responsibility.proof) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingRootResponsibility,
            location.clone(),
            "responsibility behavior and proof must be concrete",
        ));
    }
    if !(responsibility.proof.starts_with("tests/")
        || responsibility.proof.starts_with("receipts/")
        || responsibility.proof.starts_with("goldens/"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingRootResponsibility,
            location.clone(),
            "responsibility proof must bind tests, receipts, or goldens",
        ));
    }
    active_responsibility_roots.insert(responsibility.owner_root.clone());
}

fn validate_claim(
    claim: &OwnerRootClaim,
    root_ids: &BTreeSet<String>,
    responsibility_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = claim.canonical_identity();
    if claim.scope != "task" && claim.scope != "frontier" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location.clone(),
            format!("unsupported claim scope {}", claim.scope),
        ));
    }
    if !CLAIM_STATUSES.contains(&claim.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            location.clone(),
            format!("unsupported claim status {}", claim.status),
        ));
    }
    for required in REQUIRED_OWNER_ROOTS {
        if !claim.roots.iter().any(|root| root == required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingOwnerRootBinding,
                location.clone(),
                format!("claim does not bind required owner root {required}"),
            ));
        }
    }
    for root in &claim.roots {
        if !root_ids.contains(root) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRootBinding,
                location.clone(),
                format!("claim references unknown owner root {root}"),
            ));
        }
    }
    for responsibility in &claim.responsibilities {
        if !responsibility_ids.contains(responsibility) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRootResponsibility,
                location.clone(),
                format!("claim references unknown responsibility {responsibility}"),
            ));
        }
    }
    if claim
        .receipts
        .iter()
        .all(|receipt| !receipt.ends_with(".receipt"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "claim must bind at least one receipt path",
        ));
    }
    if claim.commands.is_empty()
        || claim
            .commands
            .iter()
            .any(|command| weak_owner_value(command))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location,
            "claim must bind command records",
        ));
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

fn weak_owner_value(value: &str) -> bool {
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
    )
}

fn owns_core_semantics(owns: &[String]) -> bool {
    owns.iter().any(|owned| {
        matches!(
            owned.as_str(),
            "core_truth"
                | "core_runtime"
                | "kernel_semantics"
                | "language_semantics"
                | "determinism_substrate"
                | "symbolic_cognition"
        )
    })
}

fn path_starts_with_root(path: &str, root: &str) -> bool {
    let expected = format!("{root}/");
    path.starts_with(&expected)
}

fn profile_for(id: &str) -> Option<&'static OwnerRootProfile> {
    OWNER_ROOT_PROFILES.iter().find(|profile| profile.id == id)
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

fn is_root_identity(value: &str) -> bool {
    is_symbolic_name(value)
}
