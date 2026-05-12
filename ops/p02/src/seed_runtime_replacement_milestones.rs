use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_seed_runtime_replacement_milestones::deterministic_seed_runtime_replacement_milestone_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_seed_runtime_replacement_milestones_model::{
    SeedRuntimeReplacementHandoffBinding, SeedRuntimeReplacementMilestoneBinding,
    SeedRuntimeReplacementMilestoneSurface, SeedRuntimeReplacementReceiptBinding,
};
use std::collections::{BTreeMap, BTreeSet};

pub const P02_SEED_RUNTIME_REPLACEMENT_MILESTONES_CONTRACT: &str =
    "LYRA-P02-SEED-RUNTIME-REPLACEMENT-MILESTONES v1";
pub const REQUIRED_SEED_RUNTIME_REPLACEMENT_RULES: &[&str] = &[
    "seed_runtime_replacement_milestones_must_bind_all_targets",
    "milestone_must_name_foreign_surface_and_native_successor",
    "replacement_before_extinction_requires_execution_receipt",
    "foreign_surface_deletion_requires_native_successor_proven",
    "operator_handoff_cannot_author_truth",
    "target_locality_must_be_preserved",
    "fallback_binding_required_for_unproven_target",
    "post_import_replay_required_before_truth_promotion",
    "milestones_must_preserve_phase_open",
    "no_network_required_replacement",
    "no_probabilistic_replacement_truth",
    "no_hidden_randomness_replacement",
    "no_ambient_time_replacement",
    "no_placeholder_milestone",
    "no_global_closure_claim",
];
pub const REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS: &[&str] = &[
    "target_linux_x86_64",
    "target_linux_aarch64",
    "target_windows_x86_64",
    "target_windows_aarch64",
    "target_android_aarch64",
    "target_ios_aarch64",
    "target_wasm32_wasi",
    "target_wasm32_unknown",
    "target_baremetal_x86_64",
    "target_baremetal_aarch64",
    "target_baremetal_riscv64",
    "target_host_tooling_quarantine",
];
pub const REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGET_CLASSES: &[&str] =
    &["linux", "windows", "mobile", "wasm", "baremetal", "other"];
pub const REQUIRED_SEED_RUNTIME_REPLACEMENT_UNITS: &[&str] = &[
    "native_seed_runtime_linux_x86_64",
    "native_seed_runtime_linux_aarch64",
    "native_seed_runtime_windows_x86_64",
    "native_seed_runtime_windows_aarch64",
    "native_seed_runtime_android_aarch64",
    "native_seed_runtime_ios_aarch64",
    "native_seed_runtime_wasm32_wasi",
    "native_seed_runtime_wasm32_unknown",
    "native_seed_runtime_baremetal_x86_64",
    "native_seed_runtime_baremetal_aarch64",
    "native_seed_runtime_baremetal_riscv64",
    "native_seed_runtime_host_tooling_quarantine",
];
pub const REQUIRED_SEED_RUNTIME_REPLACEMENT_FALLBACKS: &[&str] = &[
    "fallback_linux_x86_64",
    "fallback_linux_aarch64",
    "fallback_windows_x86_64",
    "fallback_windows_aarch64",
    "fallback_android_aarch64",
    "fallback_ios_aarch64",
    "fallback_wasm32_wasi",
    "fallback_wasm32_unknown",
    "fallback_baremetal_x86_64",
    "fallback_baremetal_aarch64",
    "fallback_baremetal_riscv64",
    "fallback_host_tooling_quarantine",
];
pub const REQUIRED_SEED_RUNTIME_REPLACEMENT_RECEIPTS: &[&str] = &[
    "receipt_seed_runtime_contracts",
    "receipt_bootstrap_target_matrix",
    "receipt_bootstrap_emergency_fallback",
    "receipt_seed_runtime_replacement_milestones",
];
const ALLOWED_FOREIGN_SURFACES: &[&str] = &[
    "rust_bootstrap_compiler",
    "rust_std_runtime",
    "host_operating_system",
    "host_filesystem",
    "lyralang_bootstrap_stub_carrier",
];
const ALLOWED_ENTRY_GATES: &[&str] = &["seed_runtime_contract_emitted"];
const ALLOWED_PROOF_GATES: &[&str] = &["native_seed_execution_receipt_required"];
const ALLOWED_EXTINCTION_GATES: &[&str] =
    &["delete_or_reclassify_foreign_surface_after_successor_proven"];
const ALLOWED_OPERATOR_ROLES: &[&str] = &["external_capture_only"];
const ALLOWED_TRUTH_EFFECTS: &[&str] = &["none_without_local_replay"];
const ALLOWED_IMPORT_GATES: &[&str] = &["post_import_replay_required"];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    (
        "probabilistic replacement truth",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_seed_runtime_replacement_milestone_surface(
    input: &str,
) -> Result<SeedRuntimeReplacementMilestoneSurface, Vec<ValidationError>> {
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
            "empty seed runtime replacement milestone surface",
        )]);
    }
    if lines[0] != P02_SEED_RUNTIME_REPLACEMENT_MILESTONES_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_SEED_RUNTIME_REPLACEMENT_MILESTONES_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut seed_runtime_contract_receipt = None;
    let mut target_matrix_receipt = None;
    let mut emergency_fallback_receipt = None;
    let mut extinction_receipt = None;
    let mut rules = BTreeMap::new();
    let mut milestones = Vec::new();
    let mut handoffs = Vec::new();
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
        if left.is_empty() || value.is_empty() || left != left.trim() || value != value.trim() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "untrimmed or empty entry",
            ));
            continue;
        }
        if let Some(id) = left.strip_prefix("rule:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    left,
                    "duplicate rule",
                ));
            } else {
                rules.insert(id.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("milestone:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateControlSurface,
                    left,
                    "duplicate milestone",
                ));
            } else {
                match parse_milestone(n, id, value) {
                    Ok(x) => milestones.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("handoff:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateControlSurface,
                    left,
                    "duplicate handoff",
                ));
            } else {
                match parse_handoff(n, id, value) {
                    Ok(x) => handoffs.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("receipt:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    left,
                    "duplicate receipt",
                ));
            } else {
                match parse_receipt(n, id, value) {
                    Ok(x) => receipts.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if !seen.insert(left.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                left,
                "duplicate scalar",
            ));
            continue;
        }
        match left {
            "phase" => phase = Some(value.to_string()),
            "task" => task = Some(value.to_string()),
            "status" => status = Some(value.to_string()),
            "seed_runtime_contract_receipt" => {
                seed_runtime_contract_receipt = Some(value.to_string())
            }
            "target_matrix_receipt" => target_matrix_receipt = Some(value.to_string()),
            "emergency_fallback_receipt" => emergency_fallback_receipt = Some(value.to_string()),
            "extinction_receipt" => extinction_receipt = Some(value.to_string()),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                format!("unknown key {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SeedRuntimeReplacementMilestoneSurface {
        header: lines[0].clone(),
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
        seed_runtime_contract_receipt: seed_runtime_contract_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "seed_runtime_contract_receipt",
                "missing seed runtime contract receipt",
            )]
        })?,
        target_matrix_receipt: target_matrix_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "target_matrix_receipt",
                "missing target matrix receipt",
            )]
        })?,
        emergency_fallback_receipt: emergency_fallback_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "emergency_fallback_receipt",
                "missing emergency fallback receipt",
            )]
        })?,
        extinction_receipt: extinction_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "extinction_receipt",
                "missing extinction receipt",
            )]
        })?,
        rules,
        milestones,
        handoffs,
        receipts,
    })
}

pub fn validate_seed_runtime_replacement_milestone_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut forbidden = Vec::new();
    scan_forbidden(input, &mut forbidden);
    let verdict = match parse_seed_runtime_replacement_milestone_surface(input) {
        Ok(surface) => {
            let mut v = validate_seed_runtime_replacement_milestone_model(&surface);
            if !forbidden.is_empty() {
                let mut errors = v.errors;
                errors.extend(forbidden);
                v = Verdict::rejected(errors);
            }
            v
        }
        Err(mut errors) => {
            errors.extend(forbidden);
            Verdict::rejected(errors)
        }
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_seed_runtime_replacement_milestone_model(
    surface: &SeedRuntimeReplacementMilestoneSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-009" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-009 got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported {}", surface.status),
        ));
    }
    if surface.seed_runtime_contract_receipt
        != "receipts/p02/pass_0061_seed_runtime_contracts.receipt"
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "seed_runtime_contract_receipt",
            "must bind P02-003",
        ));
    }
    if surface.target_matrix_receipt != "receipts/p02/pass_0064_bootstrap_target_matrix.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "target_matrix_receipt",
            "must bind P02-006",
        ));
    }
    if surface.emergency_fallback_receipt
        != "receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt"
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "emergency_fallback_receipt",
            "must bind P02-008",
        ));
    }
    if surface.extinction_receipt != "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "extinction_receipt",
            "must bind P02-002",
        ));
    }
    for r in REQUIRED_SEED_RUNTIME_REPLACEMENT_RULES {
        match surface.rule_value(r) {
            Some("required") | Some("forbidden") => {}
            Some(v) => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("rule:{r}"),
                format!("bad rule value {v}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{r}"),
                "missing rule",
            )),
        }
    }
    for id in REQUIRED_SEED_RUNTIME_REPLACEMENT_RECEIPTS {
        if surface.receipt_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{id}"),
                "missing receipt",
            ));
        }
    }

    let mut milestone_targets = BTreeSet::new();
    let mut classes = BTreeSet::new();
    for milestone in &surface.milestones {
        if !REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS.contains(&milestone.target_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                milestone.canonical_identity(),
                "unknown milestone target",
            ));
        } else {
            milestone_targets.insert(milestone.target_id.as_str());
        }
        if !REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGET_CLASSES
            .contains(&milestone.target_class.as_str())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                milestone.canonical_identity(),
                "bad target class",
            ));
        } else {
            classes.insert(milestone.target_class.as_str());
        }
        if !REQUIRED_SEED_RUNTIME_REPLACEMENT_UNITS.contains(&milestone.replacement_unit.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidImplementationUnit,
                milestone.canonical_identity(),
                "bad replacement unit",
            ));
        }
        if !ALLOWED_FOREIGN_SURFACES.contains(&milestone.foreign_surface_ref.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                milestone.canonical_identity(),
                "bad foreign surface ref",
            ));
        }
        if !milestone
            .native_successor
            .starts_with("lyra_native_seed_runtime_")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidImplementationUnit,
                milestone.canonical_identity(),
                "bad native successor",
            ));
        }
        if !ALLOWED_ENTRY_GATES.contains(&milestone.entry_gate.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                milestone.canonical_identity(),
                "bad entry gate",
            ));
        }
        if !ALLOWED_PROOF_GATES.contains(&milestone.proof_gate.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEngineProof,
                milestone.canonical_identity(),
                "bad proof gate",
            ));
        }
        if !ALLOWED_EXTINCTION_GATES.contains(&milestone.extinction_gate.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                milestone.canonical_identity(),
                "bad extinction gate",
            ));
        }
        if !REQUIRED_SEED_RUNTIME_REPLACEMENT_FALLBACKS.contains(&milestone.fallback_ref.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                milestone.canonical_identity(),
                "bad fallback ref",
            ));
        }
        if milestone.closure_claim != "phase_open" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                milestone.canonical_identity(),
                "milestone cannot close phase",
            ));
        }
        if milestone.status != "milestone_declared" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                milestone.canonical_identity(),
                "bad milestone status",
            ));
        }
    }
    for id in REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS {
        if !milestone_targets.contains(*id) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingControlSurface,
                format!("milestone_for:{id}"),
                "missing replacement milestone",
            ));
        }
    }
    for class in REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGET_CLASSES {
        if !classes.contains(*class) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target_class:{class}"),
                "missing target class",
            ));
        }
    }

    let mut handoff_targets = BTreeSet::new();
    for handoff in &surface.handoffs {
        if !REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS.contains(&handoff.target_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                handoff.canonical_identity(),
                "unknown handoff target",
            ));
        } else {
            handoff_targets.insert(handoff.target_id.as_str());
        }
        if !ALLOWED_OPERATOR_ROLES.contains(&handoff.operator_role.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                handoff.canonical_identity(),
                "bad operator role",
            ));
        }
        for required in [
            "receipt_seed_runtime_contracts",
            "receipt_bootstrap_target_matrix",
            "receipt_bootstrap_emergency_fallback",
        ] {
            if !handoff.required_receipts.iter().any(|x| x == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingReceiptProof,
                    handoff.canonical_identity(),
                    format!("missing {required}"),
                ));
            }
        }
        if !ALLOWED_TRUTH_EFFECTS.contains(&handoff.truth_effect.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                handoff.canonical_identity(),
                "bad truth effect",
            ));
        }
        if !ALLOWED_IMPORT_GATES.contains(&handoff.import_gate.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReplayProof,
                handoff.canonical_identity(),
                "bad import gate",
            ));
        }
        if handoff.status != "rule_declared" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                handoff.canonical_identity(),
                "bad handoff status",
            ));
        }
    }
    for id in REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS {
        if !handoff_targets.contains(*id) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingControlSurface,
                format!("handoff_for:{id}"),
                "missing operator handoff",
            ));
        }
    }

    for rec in &surface.receipts {
        if !rec.path.starts_with("receipts/p02/") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                rec.canonical_identity(),
                "bad receipt path",
            ));
        }
        if rec.status != "artifact_emitted"
            && rec.status != "execution_proven"
            && rec.status != "pending_local_validation"
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                rec.canonical_identity(),
                "bad receipt status",
            ));
        }
    }

    let report_milestones: Vec<_> = surface
        .milestones
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.target_id.clone(),
                x.target_class.clone(),
                x.replacement_unit.clone(),
                x.foreign_surface_ref.clone(),
                x.native_successor.clone(),
                x.entry_gate.clone(),
                x.proof_gate.clone(),
                x.extinction_gate.clone(),
                x.fallback_ref.clone(),
                x.closure_claim.clone(),
                x.status.clone(),
            )
        })
        .collect();
    let report_handoffs: Vec<_> = surface
        .handoffs
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.target_id.clone(),
                x.operator_role.clone(),
                x.required_receipts.clone(),
                x.truth_effect.clone(),
                x.import_gate.clone(),
                x.status.clone(),
            )
        })
        .collect();
    let report_receipts: Vec<_> = surface
        .receipts
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.path.clone(),
                x.target.clone(),
                x.status.clone(),
            )
        })
        .collect();
    let report = deterministic_seed_runtime_replacement_milestone_report(
        &report_milestones,
        &report_handoffs,
        &report_receipts,
    );
    if report.milestone_count < REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS.len()
        || report.handoff_count < REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS.len()
        || report.target_class_count < REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGET_CLASSES.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlSurface,
            "seed_runtime_replacement_report",
            "insufficient target milestone coverage",
        ));
    }
    if report.native_successor_count < REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidImplementationUnit,
            "seed_runtime_replacement_report",
            "native successor coverage incomplete",
        ));
    }
    if report.fallback_binding_count != surface.milestones.len() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRollbackPath,
            "seed_runtime_replacement_report",
            "milestone without fallback binding",
        ));
    }
    if report.phase_open_count != surface.milestones.len() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "seed_runtime_replacement_report",
            "milestone attempted phase closure",
        ));
    }
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_milestone(
    n: usize,
    id: &str,
    v: &str,
) -> Result<SeedRuntimeReplacementMilestoneBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(SeedRuntimeReplacementMilestoneBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&f, "target_id", n)?,
        target_class: req(&f, "target_class", n)?,
        replacement_unit: req(&f, "replacement_unit", n)?,
        foreign_surface_ref: req(&f, "foreign_surface_ref", n)?,
        native_successor: req(&f, "native_successor", n)?,
        entry_gate: req(&f, "entry_gate", n)?,
        proof_gate: req(&f, "proof_gate", n)?,
        extinction_gate: req(&f, "extinction_gate", n)?,
        fallback_ref: req(&f, "fallback_ref", n)?,
        closure_claim: req(&f, "closure_claim", n)?,
        status: req(&f, "status", n)?,
    })
}
fn parse_handoff(
    n: usize,
    id: &str,
    v: &str,
) -> Result<SeedRuntimeReplacementHandoffBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(SeedRuntimeReplacementHandoffBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&f, "target_id", n)?,
        operator_role: req(&f, "operator_role", n)?,
        required_receipts: csv(&req(&f, "required_receipts", n)?),
        truth_effect: req(&f, "truth_effect", n)?,
        import_gate: req(&f, "import_gate", n)?,
        status: req(&f, "status", n)?,
    })
}
fn parse_receipt(
    n: usize,
    id: &str,
    v: &str,
) -> Result<SeedRuntimeReplacementReceiptBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(SeedRuntimeReplacementReceiptBinding {
        line_number: n,
        id: id.to_string(),
        path: req(&f, "path", n)?,
        target: req(&f, "target", n)?,
        status: req(&f, "status", n)?,
    })
}
fn fields(v: &str, n: usize) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut m = BTreeMap::new();
    for seg in v.split('|') {
        let Some((k, val)) = seg.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "bad field",
            ));
        };
        if k.is_empty() || val.is_empty() || m.insert(k.to_string(), val.to_string()).is_some() {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "bad field",
            ));
        }
    }
    Ok(m)
}
fn req(f: &BTreeMap<String, String>, k: &str, n: usize) -> Result<String, ValidationError> {
    f.get(k).cloned().ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{n:03}"),
            format!("missing {k}"),
        )
    })
}
fn csv(v: &str) -> Vec<String> {
    v.split(',')
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect()
}
fn scan_forbidden(input: &str, errors: &mut Vec<ValidationError>) {
    let lower = input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden_text",
                format!("forbidden replacement milestone phrase {needle}"),
            ));
        }
    }
}
