use std::collections::BTreeSet;

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_constitution::{P00_CONSTITUTION_CONTRACT, P00_GOVERNANCE_REQUIREMENTS};
use crate::p00_model::{ParsedEntry, ParsedSurface, SurfaceKind};

const FORBIDDEN_TEXT: &[(&str, ErrorCode)] = &[
    ("todo", ErrorCode::ForbiddenToken),
    ("tbd", ErrorCode::ForbiddenToken),
    ("stub", ErrorCode::ForbiddenToken),
    ("mock only", ErrorCode::ForbiddenToken),
    ("not implemented", ErrorCode::ForbiddenToken),
    ("will add later", ErrorCode::ForbiddenToken),
    ("finish later", ErrorCode::ForbiddenToken),
    ("placeholder allowed", ErrorCode::PlaceholderAllowed),
    ("network allowed", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("randomness allowed", ErrorCode::HiddenRandomnessAllowed),
];

pub fn parse_surface(input: &str) -> Result<ParsedSurface, Vec<ValidationError>> {
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
            "no governance surface lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_CONSTITUTION_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_CONSTITUTION_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut entries = Vec::new();
    let mut identities = BTreeSet::new();

    for (offset, line) in lines.iter().enumerate().skip(1) {
        let line_number = offset + 1;
        let Some((left, value)) = line.split_once('=') else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry must contain exactly one equals separator after namespace:name",
            ));
            continue;
        };
        if value.trim().is_empty() || value != value.trim() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry value must be non-empty and already trimmed",
            ));
            continue;
        }

        let (namespace, name) = if let Some((namespace, name)) = left.split_once(':') {
            (namespace.to_string(), name.to_string())
        } else {
            ("field".to_string(), left.to_string())
        };

        if namespace.is_empty()
            || name.is_empty()
            || namespace != namespace.trim()
            || name != name.trim()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry identity must be non-empty and trimmed",
            ));
            continue;
        }

        let identity = format!("{namespace}:{name}");
        if !identities.insert(identity.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                identity,
            ));
            continue;
        }

        entries.push(ParsedEntry {
            line_number,
            namespace,
            name,
            value: value.to_string(),
        });
    }

    if errors.is_empty() {
        Ok(ParsedSurface {
            kind: SurfaceKind::Constitution,
            header,
            entries,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_constitution_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let verdict = match parse_surface(input) {
        Ok(surface) => validate_parsed_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn validate_parsed_surface(surface: &ParsedSurface, raw_input: &str) -> Verdict {
    let mut errors = Vec::new();

    match surface.scalar_value("phase") {
        None => errors.push(ValidationError::reject(
            ErrorCode::MissingPhase,
            "field:phase",
            "phase=P00 is required",
        )),
        Some("P00") => {}
        Some(other) => errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            format!("expected P00, found {other}"),
        )),
    }

    match surface.scalar_value("task") {
        None => errors.push(ValidationError::reject(
            ErrorCode::MissingTask,
            "field:task",
            "task=P00-001 is required for this frontier",
        )),
        Some("P00-001") => {}
        Some(other) => errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            format!("expected P00-001, found {other}"),
        )),
    }

    for requirement in P00_GOVERNANCE_REQUIREMENTS {
        let matching_entry = surface.entries.iter().find(|entry| {
            entry.namespace == requirement.namespace && entry.name == requirement.name
        });

        match matching_entry {
            None => errors.push(missing_requirement_error(
                requirement.namespace,
                requirement.name,
            )),
            Some(entry) => {
                let value_lower = entry.value.to_ascii_lowercase();
                if !value_lower.contains(requirement.value_contains) {
                    errors.push(missing_requirement_error(
                        requirement.namespace,
                        requirement.name,
                    ));
                }
            }
        }
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface:text",
                format!("forbidden token detected: {needle}"),
            ));
        }
    }

    if surface.entries.iter().any(|entry| {
        entry.namespace == "closure" && entry.name == "claim" && entry.value == "global_complete"
    }) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "closure:claim",
            "P00-001 may only claim working_slice until all P00 obligations are proven",
        ));
    }

    if surface.entries.iter().any(|entry| {
        entry.namespace == "closure"
            && entry.name == "truth"
            && entry.value.contains("complete_without_evidence")
    }) {
        errors.push(ValidationError::reject(
            ErrorCode::FakeClosureClaim,
            "closure:truth",
            "completion claims require bound evidence receipts",
        ));
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn missing_requirement_error(namespace: &str, name: &str) -> ValidationError {
    let code = match namespace {
        "principle" => ErrorCode::MissingRequiredPrinciple,
        "duty" => ErrorCode::MissingRequiredDuty,
        "ban" => ErrorCode::MissingRequiredBan,
        "evidence" => ErrorCode::MissingRequiredEvidence,
        "owner_root" => ErrorCode::MissingRequiredOwnerRoot,
        _ => ErrorCode::InvalidEntrySyntax,
    };
    ValidationError::reject(
        code,
        format!("{namespace}:{name}"),
        "required P00 constitutional binding is absent or too weak",
    )
}
