use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_foreign_surface_closure::deterministic_foreign_surface_closure_report;
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_foreign_surface_closure_model::{
    ForeignSurfaceBinding, ForeignSurfaceChallengeBinding, ForeignSurfaceClosureLawBinding,
    ForeignSurfaceClosureReceiptBinding, ForeignSurfaceClosureSurface,
    ForeignSurfaceVisibilityBinding,
};
use std::collections::{BTreeMap, BTreeSet};

pub const P02_FOREIGN_SURFACE_CLOSURE_CONTRACT: &str = "LYRA-P02-FOREIGN-SURFACE-CLOSURE v1";
pub const REQUIRED_FOREIGN_SURFACE_CLOSURE_RULES: &[&str] = &[
    "foreign_surface_inventory_must_be_total",
    "every_foreign_surface_must_be_visible",
    "every_foreign_surface_must_be_bounded",
    "every_foreign_surface_must_be_challengeable",
    "every_foreign_surface_must_pair_closure_law",
    "closure_law_must_define_deletion_gate",
    "closure_law_must_define_retirement_receipt",
    "truth_cannot_advance_from_foreign_surface_without_local_challenge_receipt",
    "bounded_permanent_surfaces_must_remain_explicit_interfaces",
    "forbidden_surfaces_must_have_rejection_evidence",
    "no_network_required_foreign_closure",
    "no_probabilistic_foreign_truth",
    "no_hidden_randomness_foreign_closure",
    "no_ambient_time_foreign_closure",
    "no_placeholder_foreign_closure",
    "no_global_phase_closure_claim",
];
pub const REQUIRED_FOREIGN_BOOTSTRAP_SURFACES: &[&str] = &[
    "foreign_rust_toolchain",
    "foreign_cargo_runner",
    "foreign_rust_stdlib",
    "foreign_host_os",
    "foreign_filesystem",
    "foreign_terminal",
    "foreign_zip_tool",
    "foreign_sha256sum",
    "foreign_android_bridge",
    "foreign_wasm_toolchain",
    "foreign_mobile_packager",
    "foreign_baremetal_loader",
];
pub const REQUIRED_FOREIGN_BOOTSTRAP_CLASSES: &[&str] = &[
    "bootstrap_language",
    "build_runner",
    "host_runtime",
    "host_os",
    "host_io",
    "operator_tool",
    "archive_tool",
    "digest_tool",
    "platform_bridge",
    "target_toolchain",
    "packager",
    "bootloader",
];
pub const REQUIRED_FOREIGN_SURFACE_OWNER_ROOTS: &[&str] =
    &["interfaces", "k0", "lyralang", "ops", "products", "shells"];
pub const REQUIRED_FOREIGN_SURFACE_CHALLENGE_SUITES: &[&str] = &[
    "suite_visibility_totality",
    "suite_boundary_nonambient",
    "suite_deletion_gate",
    "suite_closure_scope",
];
pub const REQUIRED_FOREIGN_SURFACE_CLOSURE_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_inventory",
    "receipt_bootstrap_extinction",
    "receipt_host_boundary_challenge_suites",
    "receipt_bootstrap_target_matrix",
    "receipt_bootstrap_truth_cleanup",
    "receipt_bootstrap_emergency_fallback",
    "receipt_seed_runtime_replacement_milestones",
    "receipt_bootstrap_evidence_emission",
    "receipt_operator_handoff_automation",
    "receipt_foreign_surface_closure",
];
const ALLOWED_STATUS: &[&str] = &["foreign_surface_closure_artifact_emitted"];
const ALLOWED_SURFACE_STATUS: &[&str] = &["foreign_surface_closure_bound"];
const ALLOWED_CHALLENGE_STATUS: &[&str] = &["challenge_binding_emitted"];
const ALLOWED_CLOSURE_LAW_STATUS: &[&str] = &["closure_law_bound"];
const ALLOWED_VISIBILITY_STATUS: &[&str] = &["visibility_proof_emitted"];
const ALLOWED_RECEIPT_STATUS: &[&str] = &["receipt_bound"];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
    ("foreign truth accepted", ErrorCode::ClosureDriftAccepted),
];

pub fn parse_foreign_surface_closure_surface(
    input: &str,
) -> Result<ForeignSurfaceClosureSurface, Vec<ValidationError>> {
    let lines = canonical_lines(input).map_err(|e| {
        vec![ValidationError::reject(
            ErrorCode::CanonicalControlByte,
            "input",
            format!("{e:?}"),
        )]
    })?;
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "empty foreign surface closure surface",
        )]);
    }
    if lines[0] != P02_FOREIGN_SURFACE_CLOSURE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_FOREIGN_SURFACE_CLOSURE_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut previous_evidence_receipt = None;
    let mut rules = BTreeMap::new();
    let mut surfaces = Vec::new();
    let mut challenges = Vec::new();
    let mut closure_laws = Vec::new();
    let mut visibility_proofs = Vec::new();
    let mut receipts = Vec::new();
    let mut seen = BTreeSet::new();
    let mut errors = Vec::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let n = index + 1;
        let Some((left, value)) = line.split_once('=') else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "missing =",
            ));
            continue;
        };
        let key = left.trim();
        let value = value.trim();
        if !seen.insert(key.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{n:03}"),
                key,
            ));
            continue;
        }
        if key == "phase" {
            phase = Some(value.to_string());
            continue;
        }
        if key == "task" {
            task = Some(value.to_string());
            continue;
        }
        if key == "status" {
            status = Some(value.to_string());
            continue;
        }
        if key == "previous_evidence_receipt" {
            previous_evidence_receipt = Some(value.to_string());
            continue;
        }
        if let Some(id) = bracket_id(key, "rule") {
            rules.insert(id.to_string(), value.to_string());
            continue;
        }
        if let Some(id) = bracket_id(key, "surface") {
            let fields = split_fields(value);
            if fields.len() != 9 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDeliveryArtifact,
                    format!("line:{n:03}"),
                    "surface row requires 9 fields",
                ));
                continue;
            }
            surfaces.push(ForeignSurfaceBinding {
                line_number: n,
                id: id.to_string(),
                surface_class: fields[0].clone(),
                owner_root: fields[1].clone(),
                visibility_status: fields[2].clone(),
                boundary_status: fields[3].clone(),
                challenge_suite: fields[4].clone(),
                closure_law: fields[5].clone(),
                retirement_gate: fields[6].clone(),
                truth_effect: fields[7].clone(),
                status: fields[8].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "challenge") {
            let fields = split_fields(value);
            if fields.len() != 6 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidChallengeFixture,
                    format!("line:{n:03}"),
                    "challenge row requires 6 fields",
                ));
                continue;
            }
            challenges.push(ForeignSurfaceChallengeBinding {
                line_number: n,
                id: id.to_string(),
                suite_id: fields[0].clone(),
                surface_id: fields[1].clone(),
                required_fixture: fields[2].clone(),
                negative_case: fields[3].clone(),
                receipt_path: fields[4].clone(),
                status: fields[5].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "closure_law") {
            let fields = split_fields(value);
            if fields.len() != 6 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    format!("line:{n:03}"),
                    "closure law row requires 6 fields",
                ));
                continue;
            }
            closure_laws.push(ForeignSurfaceClosureLawBinding {
                line_number: n,
                id: id.to_string(),
                surface_id: fields[0].clone(),
                closure_gate: fields[1].clone(),
                deletion_gate: fields[2].clone(),
                retirement_receipt: fields[3].clone(),
                allowed_closure_scope: fields[4].clone(),
                status: fields[5].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "visibility") {
            let fields = split_fields(value);
            if fields.len() != 5 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEvidenceBinding,
                    format!("line:{n:03}"),
                    "visibility row requires 5 fields",
                ));
                continue;
            }
            visibility_proofs.push(ForeignSurfaceVisibilityBinding {
                line_number: n,
                id: id.to_string(),
                surface_id: fields[0].clone(),
                inventory_path: fields[1].clone(),
                classification_path: fields[2].clone(),
                evidence_path: fields[3].clone(),
                status: fields[4].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "receipt") {
            let fields = split_fields(value);
            if fields.len() != 3 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{n:03}"),
                    "receipt row requires 3 fields",
                ));
                continue;
            }
            receipts.push(ForeignSurfaceClosureReceiptBinding {
                line_number: n,
                id: id.to_string(),
                path: fields[0].clone(),
                surface_id: fields[1].clone(),
                status: fields[2].clone(),
            });
            continue;
        }
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{n:03}"),
            key,
        ));
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(ForeignSurfaceClosureSurface {
        header: P02_FOREIGN_SURFACE_CLOSURE_CONTRACT.to_string(),
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
        status: status.unwrap_or_default(),
        previous_evidence_receipt: previous_evidence_receipt.unwrap_or_default(),
        rules,
        surfaces,
        challenges,
        closure_laws,
        visibility_proofs,
        receipts,
    })
}

pub fn validate_foreign_surface_closure_model(surface: &ForeignSurfaceClosureSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            &surface.phase,
        ));
    }
    if surface.task != "P02-012" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            &surface.task,
        ));
    }
    if !ALLOWED_STATUS.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            &surface.status,
        ));
    }
    for required in REQUIRED_FOREIGN_SURFACE_CLOSURE_RULES {
        if surface.rule_value(required) != Some("active") {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule[{required}]"),
                "required active rule missing",
            ));
        }
    }
    require_unique(
        surface.surfaces.iter().map(|x| x.id.as_str()),
        ErrorCode::DuplicateDeliveryArtifact,
        "surface",
        &mut errors,
    );
    require_unique(
        surface.challenges.iter().map(|x| x.id.as_str()),
        ErrorCode::DuplicateChallengeFixture,
        "challenge",
        &mut errors,
    );
    require_unique(
        surface.closure_laws.iter().map(|x| x.id.as_str()),
        ErrorCode::DuplicateClosureTask,
        "closure_law",
        &mut errors,
    );
    require_unique(
        surface.visibility_proofs.iter().map(|x| x.id.as_str()),
        ErrorCode::DuplicateEvidenceBinding,
        "visibility",
        &mut errors,
    );
    require_unique(
        surface.receipts.iter().map(|x| x.id.as_str()),
        ErrorCode::DuplicateClosureProof,
        "receipt",
        &mut errors,
    );

    for required in REQUIRED_FOREIGN_BOOTSTRAP_SURFACES {
        let Some(row) = surface.surface_by_id(required) else {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeliveryArtifact,
                format!("surface[{required}]"),
                "required foreign surface missing",
            ));
            continue;
        };
        if !REQUIRED_FOREIGN_BOOTSTRAP_CLASSES.contains(&row.surface_class.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                row.canonical_identity(),
                "unknown foreign surface class",
            ));
        }
        if !REQUIRED_FOREIGN_SURFACE_OWNER_ROOTS.contains(&row.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                row.canonical_identity(),
                "unknown owner root",
            ));
        }
        if !ALLOWED_SURFACE_STATUS.contains(&row.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                row.canonical_identity(),
                &row.status,
            ));
        }
        if !row.visible() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                row.canonical_identity(),
                "foreign surface is not visible in inventory",
            ));
        }
        if !row.bounded() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureDriftAccepted,
                row.canonical_identity(),
                "foreign surface boundary is not explicit and bounded",
            ));
        }
        if !row.challengeable()
            || !REQUIRED_FOREIGN_SURFACE_CHALLENGE_SUITES.contains(&row.challenge_suite.as_str())
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeRule,
                row.canonical_identity(),
                "foreign surface is not challengeable",
            ));
        }
        if !row.closure_paired() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureTask,
                row.canonical_identity(),
                "foreign surface lacks closure law or retirement gate",
            ));
        }
        if !row.truth_neutral() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureDriftAccepted,
                row.canonical_identity(),
                "foreign surface can advance truth without local challenge",
            ));
        }
        if surface.visibility_for_surface(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                row.canonical_identity(),
                "visibility proof missing",
            ));
        }
        if surface.challenge_for_surface(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeFixture,
                row.canonical_identity(),
                "challenge binding missing",
            ));
        }
        if surface.closure_law_by_id(&row.closure_law).is_none()
            || surface.closure_law_for_surface(required).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureTask,
                row.canonical_identity(),
                "closure law binding missing",
            ));
        }
    }

    for challenge in &surface.challenges {
        if surface.surface_by_id(&challenge.surface_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                challenge.canonical_identity(),
                "challenge references unknown surface",
            ));
        }
        if !REQUIRED_FOREIGN_SURFACE_CHALLENGE_SUITES.contains(&challenge.suite_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                challenge.canonical_identity(),
                "unknown challenge suite",
            ));
        }
        if !challenge
            .required_fixture
            .starts_with("fixtures/p02/foreign_surface_closure_inputs/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                challenge.canonical_identity(),
                "challenge fixture must be local fixture path",
            ));
        }
        if challenge.negative_case.is_empty() || challenge.negative_case == "none" {
            errors.push(ValidationError::reject(
                ErrorCode::MissingNegativeCase,
                challenge.canonical_identity(),
                "negative case missing",
            ));
        }
        if !challenge
            .receipt_path
            .starts_with("receipts/p02/foreign_surface_challenges/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                challenge.canonical_identity(),
                "challenge receipt path must be local p02 path",
            ));
        }
        if !ALLOWED_CHALLENGE_STATUS.contains(&challenge.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                challenge.canonical_identity(),
                &challenge.status,
            ));
        }
    }

    for law in &surface.closure_laws {
        if surface.surface_by_id(&law.surface_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                law.canonical_identity(),
                "closure law references unknown surface",
            ));
        }
        if law.closure_gate.is_empty() || law.closure_gate == "none" {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                law.canonical_identity(),
                "closure gate missing",
            ));
        }
        if law.deletion_gate.is_empty() || law.deletion_gate == "none" {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                law.canonical_identity(),
                "deletion gate missing",
            ));
        }
        if !law.retirement_receipt.starts_with("receipts/p02/") {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                law.canonical_identity(),
                "retirement receipt path must be local p02 path",
            ));
        }
        if !law.bounded_scope() {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                law.canonical_identity(),
                "closure scope must be per_surface_only",
            ));
        }
        if !ALLOWED_CLOSURE_LAW_STATUS.contains(&law.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                law.canonical_identity(),
                &law.status,
            ));
        }
    }

    for visibility in &surface.visibility_proofs {
        if surface.surface_by_id(&visibility.surface_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEvidenceBinding,
                visibility.canonical_identity(),
                "visibility proof references unknown surface",
            ));
        }
        if !visibility.inventory_path.starts_with("ops/p02/inventory/") {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                visibility.canonical_identity(),
                "inventory path must bind P02 inventory",
            ));
        }
        if !visibility
            .classification_path
            .starts_with("ops/p02/foreign_surface_closure/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                visibility.canonical_identity(),
                "classification path must bind closure packet",
            ));
        }
        if !visibility
            .evidence_path
            .starts_with("fixtures/p02/foreign_surface_closure_inputs/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                visibility.canonical_identity(),
                "visibility evidence must bind local fixture",
            ));
        }
        if !ALLOWED_VISIBILITY_STATUS.contains(&visibility.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                visibility.canonical_identity(),
                &visibility.status,
            ));
        }
    }

    for required in REQUIRED_FOREIGN_SURFACE_CLOSURE_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt[{required}]"),
                "required receipt binding missing",
            ));
        }
    }
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p02/") {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                receipt.canonical_identity(),
                "receipt path must be local p02 path",
            ));
        }
        if !ALLOWED_RECEIPT_STATUS.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                &receipt.status,
            ));
        }
    }
    let report = deterministic_foreign_surface_closure_report(surface);
    if !report.unpaired_surfaces.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureProofUnbound,
            "report.unpaired_surfaces",
            report.unpaired_surfaces.join(","),
        ));
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

pub fn validate_foreign_surface_closure_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let lower = canonical.to_ascii_lowercase();
    let mut forbidden_errors = Vec::new();
    for (token, code) in FORBIDDEN {
        if lower.contains(token) {
            forbidden_errors.push(ValidationError::reject(*code, "forbidden", *token));
        }
    }
    let verdict = match parse_foreign_surface_closure_surface(input) {
        Ok(surface) => {
            let mut verdict = validate_foreign_surface_closure_model(&surface);
            if !forbidden_errors.is_empty() {
                let mut errors = verdict.errors;
                errors.extend(forbidden_errors);
                verdict = Verdict::rejected(errors);
            }
            verdict
        }
        Err(mut errors) => {
            errors.extend(forbidden_errors);
            Verdict::rejected(errors)
        }
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn bracket_id<'a>(key: &'a str, prefix: &str) -> Option<&'a str> {
    let start = format!("{prefix}[");
    if key.starts_with(&start) && key.ends_with(']') {
        Some(&key[start.len()..key.len() - 1])
    } else {
        None
    }
}

fn split_fields(value: &str) -> Vec<String> {
    value.split('|').map(|x| x.trim().to_string()).collect()
}

fn require_unique<'a>(
    ids: impl Iterator<Item = &'a str>,
    code: ErrorCode,
    label: &str,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id.to_string()) {
            errors.push(ValidationError::reject(
                code,
                format!("{label}[{id}]"),
                "duplicate id",
            ));
        }
    }
}
