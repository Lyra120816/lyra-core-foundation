use crate::k0_bootstrap_emergency_fallback::deterministic_bootstrap_emergency_fallback_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_emergency_fallback_model::{
    BootstrapEmergencyFallbackBinding, BootstrapEmergencyFallbackReceiptBinding,
    BootstrapEmergencyFallbackSurface, BootstrapEmergencyRollbackBinding,
};
use std::collections::{BTreeMap, BTreeSet};
pub const P02_BOOTSTRAP_EMERGENCY_FALLBACK_CONTRACT: &str =
    "LYRA-P02-BOOTSTRAP-EMERGENCY-FALLBACK v1";
pub const REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_RULES: &[&str] = &[
    "incomplete_target_lane_must_enter_bounded_failure",
    "bounded_failure_must_freeze_truth_promotion",
    "emergency_fallback_must_bind_target_matrix_entry",
    "rollback_requires_last_good_receipt",
    "rollback_must_return_to_truth_cleanup_or_target_matrix",
    "operator_handoff_must_remain_non_authoritative",
    "failure_mode_must_be_deterministic_and_named",
    "quarantine_must_prevent_ambient_host_capture",
    "no_emergency_path_may_claim_phase_closure",
    "local_validation_absence_must_remain_blocker",
    "post_rollback_replay_required_before_frontier_advance",
    "evidence_gap_must_emit_reproducible_challenge",
    "fallback_must_preserve_target_locality",
    "fallback_must_not_delete_prior_good_receipts",
    "no_network_required_fallback",
    "no_probabilistic_fallback_truth",
    "no_hidden_randomness_fallback",
    "no_ambient_time_fallback",
    "no_placeholder_fallback",
    "no_global_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS: &[&str] = &[
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
pub const REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGET_CLASSES: &[&str] =
    &["linux", "windows", "mobile", "wasm", "baremetal", "other"];
pub const REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_target_matrix",
    "receipt_bootstrap_truth_cleanup",
    "receipt_host_boundary_challenge",
    "receipt_bootstrap_emergency_fallback",
];
const ALLOWED_FAILURE_STATES: &[&str] = &[
    "pending_local_validation",
    "host_boundary_violation",
    "target_proof_mismatch",
    "receipt_import_failed",
    "runtime_seed_unavailable",
    "foreign_surface_drift",
    "operator_handoff_incomplete",
    "rollback_receipt_missing",
];
const ALLOWED_FREEZE_ACTIONS: &[&str] = &["freeze_truth_promotion"];
const ALLOWED_FALLBACK_ACTIONS: &[&str] = &["enter_bounded_failure_quarantine"];
const ALLOWED_ROLLBACK_PATHS: &[&str] = &["rollback_to_truth_cleanup", "rollback_to_target_matrix"];
const ALLOWED_REQUIRED_CHALLENGES: &[&str] = &["challenge_replay_target_lane"];
const ALLOWED_OPERATOR_STATES: &[&str] = &["non_authoritative_handoff"];
const ALLOWED_ROLLBACK_TRIGGERS: &[&str] = &["incomplete_target_lane"];
const ALLOWED_TO_STATES: &[&str] = &["bounded_failure_quarantine"];
const ALLOWED_REPLAY_GATES: &[&str] = &["post_rollback_replay_required"];
const ALLOWED_FRONTIER_DECISIONS: &[&str] = &["hold_until_target_proven_or_retired"];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    (
        "probabilistic fallback truth",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
];
pub fn parse_bootstrap_emergency_fallback_surface(
    input: &str,
) -> Result<BootstrapEmergencyFallbackSurface, Vec<ValidationError>> {
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
            "empty bootstrap emergency fallback surface",
        )]);
    }
    if lines[0] != P02_BOOTSTRAP_EMERGENCY_FALLBACK_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_EMERGENCY_FALLBACK_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut target_matrix_receipt = None;
    let mut truth_cleanup_receipt = None;
    let mut host_boundary_receipt = None;
    let mut rules = BTreeMap::new();
    let mut fallbacks = Vec::new();
    let mut rollbacks = Vec::new();
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
        if let Some(id) = left.strip_prefix("fallback:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateControlSurface,
                    left,
                    "duplicate fallback",
                ));
            } else {
                match parse_fallback(n, id, value) {
                    Ok(x) => fallbacks.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("rollback:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRollbackPath,
                    left,
                    "duplicate rollback",
                ));
            } else {
                match parse_rollback(n, id, value) {
                    Ok(x) => rollbacks.push(x),
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
            "target_matrix_receipt" => target_matrix_receipt = Some(value.to_string()),
            "truth_cleanup_receipt" => truth_cleanup_receipt = Some(value.to_string()),
            "host_boundary_receipt" => host_boundary_receipt = Some(value.to_string()),
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
    Ok(BootstrapEmergencyFallbackSurface {
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
        target_matrix_receipt: target_matrix_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "target_matrix_receipt",
                "missing target matrix receipt",
            )]
        })?,
        truth_cleanup_receipt: truth_cleanup_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "truth_cleanup_receipt",
                "missing truth cleanup receipt",
            )]
        })?,
        host_boundary_receipt: host_boundary_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "host_boundary_receipt",
                "missing host boundary receipt",
            )]
        })?,
        rules,
        fallbacks,
        rollbacks,
        receipts,
    })
}
pub fn validate_bootstrap_emergency_fallback_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut forbidden = Vec::new();
    scan_forbidden(input, &mut forbidden);
    let verdict = match parse_bootstrap_emergency_fallback_surface(input) {
        Ok(surface) => {
            let mut v = validate_bootstrap_emergency_fallback_model(&surface);
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
pub fn validate_bootstrap_emergency_fallback_model(
    surface: &BootstrapEmergencyFallbackSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-008" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-008 got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported {}", surface.status),
        ));
    }
    if surface.target_matrix_receipt != "receipts/p02/pass_0064_bootstrap_target_matrix.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "target_matrix_receipt",
            "must bind P02-006",
        ));
    }
    if surface.truth_cleanup_receipt != "receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "truth_cleanup_receipt",
            "must bind P02-007",
        ));
    }
    if surface.host_boundary_receipt
        != "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt"
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "host_boundary_receipt",
            "must bind P02-005",
        ));
    }
    for r in REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_RULES {
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
    for id in REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_RECEIPTS {
        if surface.receipt_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{id}"),
                "missing receipt",
            ));
        }
    }
    let mut fallback_targets = BTreeSet::new();
    let mut classes = BTreeSet::new();
    for fallback in &surface.fallbacks {
        if !REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS.contains(&fallback.target_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                fallback.canonical_identity(),
                "unknown fallback target",
            ));
        } else {
            fallback_targets.insert(fallback.target_id.as_str());
        }
        if !REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGET_CLASSES
            .contains(&fallback.target_class.as_str())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                fallback.canonical_identity(),
                "bad target class",
            ));
        } else {
            classes.insert(fallback.target_class.as_str());
        }
        if !ALLOWED_FAILURE_STATES.contains(&fallback.failure_state.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                fallback.canonical_identity(),
                "bad failure state",
            ));
        }
        if !ALLOWED_FREEZE_ACTIONS.contains(&fallback.freeze_action.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                fallback.canonical_identity(),
                "bad freeze action",
            ));
        }
        if !ALLOWED_FALLBACK_ACTIONS.contains(&fallback.fallback_action.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlSurface,
                fallback.canonical_identity(),
                "bad fallback action",
            ));
        }
        if !ALLOWED_ROLLBACK_PATHS.contains(&fallback.rollback_path.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                fallback.canonical_identity(),
                "bad rollback path",
            ));
        }
        if fallback.last_good_receipt != "receipts/p02/pass_0065_bootstrap_truth_cleanup.receipt" {
            errors.push(ValidationError::reject(
                ErrorCode::RollbackWithoutReceipt,
                fallback.canonical_identity(),
                "bad last good receipt",
            ));
        }
        if !ALLOWED_REQUIRED_CHALLENGES.contains(&fallback.required_challenge.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeFixture,
                fallback.canonical_identity(),
                "bad required challenge",
            ));
        }
        if !ALLOWED_OPERATOR_STATES.contains(&fallback.operator_state.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                fallback.canonical_identity(),
                "bad operator state",
            ));
        }
        if fallback.closure_claim != "phase_open" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                fallback.canonical_identity(),
                "emergency fallback cannot close phase",
            ));
        }
        if fallback.status != "rule_declared" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                fallback.canonical_identity(),
                "bad fallback status",
            ));
        }
    }
    for id in REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS {
        if !fallback_targets.contains(id) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingControlSurface,
                format!("fallback_for:{id}"),
                "missing fallback binding",
            ));
        }
    }
    for class in REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGET_CLASSES {
        if !classes.contains(class) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target_class:{class}"),
                "missing fallback class",
            ));
        }
    }
    let mut rollback_targets = BTreeSet::new();
    for rollback in &surface.rollbacks {
        if !REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS.contains(&rollback.target_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                rollback.canonical_identity(),
                "unknown rollback target",
            ));
        } else {
            rollback_targets.insert(rollback.target_id.as_str());
        }
        if !ALLOWED_ROLLBACK_TRIGGERS.contains(&rollback.trigger.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackAuthority,
                rollback.canonical_identity(),
                "bad rollback trigger",
            ));
        }
        if !ALLOWED_FAILURE_STATES.contains(&rollback.from_state.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                rollback.canonical_identity(),
                "bad rollback source state",
            ));
        }
        if !ALLOWED_TO_STATES.contains(&rollback.to_state.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                rollback.canonical_identity(),
                "bad rollback destination state",
            ));
        }
        for required in [
            "receipt_bootstrap_target_matrix",
            "receipt_bootstrap_truth_cleanup",
        ] {
            if !rollback.required_receipts.iter().any(|x| x == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingReceiptProof,
                    rollback.canonical_identity(),
                    format!("missing {required}"),
                ));
            }
        }
        if !ALLOWED_REPLAY_GATES.contains(&rollback.replay_gate.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReplayProof,
                rollback.canonical_identity(),
                "bad replay gate",
            ));
        }
        if !ALLOWED_FRONTIER_DECISIONS.contains(&rollback.frontier_decision.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlSurface,
                rollback.canonical_identity(),
                "bad frontier decision",
            ));
        }
        if rollback.status != "rule_declared" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                rollback.canonical_identity(),
                "bad rollback status",
            ));
        }
    }
    for id in REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS {
        if !rollback_targets.contains(id) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRollbackPath,
                format!("rollback_for:{id}"),
                "missing rollback binding",
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
    let report_fallbacks: Vec<_> = surface
        .fallbacks
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.target_id.clone(),
                x.target_class.clone(),
                x.failure_state.clone(),
                x.freeze_action.clone(),
                x.fallback_action.clone(),
                x.rollback_path.clone(),
                x.last_good_receipt.clone(),
                x.required_challenge.clone(),
                x.operator_state.clone(),
                x.closure_claim.clone(),
                x.status.clone(),
            )
        })
        .collect();
    let report_rollbacks: Vec<_> = surface
        .rollbacks
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.target_id.clone(),
                x.trigger.clone(),
                x.from_state.clone(),
                x.to_state.clone(),
                x.required_receipts.clone(),
                x.replay_gate.clone(),
                x.frontier_decision.clone(),
                x.status.clone(),
            )
        })
        .collect();
    let report =
        deterministic_bootstrap_emergency_fallback_report(&report_fallbacks, &report_rollbacks);
    if report.fallback_count < REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS.len()
        || report.rollback_count < REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS.len()
        || report.target_class_count < REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGET_CLASSES.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlSurface,
            "emergency_fallback_report",
            "insufficient emergency fallback coverage",
        ));
    }
    if report.phase_open_count != surface.fallbacks.len() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "emergency_fallback_report",
            "emergency fallback attempted phase closure",
        ));
    }
    if report.quarantine_count != surface.fallbacks.len() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlSurface,
            "emergency_fallback_report",
            "fallback without quarantine",
        ));
    }
    if report.replay_gate_count != surface.rollbacks.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReplayProof,
            "emergency_fallback_report",
            "rollback without replay gate",
        ));
    }
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}
fn parse_fallback(
    n: usize,
    id: &str,
    v: &str,
) -> Result<BootstrapEmergencyFallbackBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapEmergencyFallbackBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&f, "target_id", n)?,
        target_class: req(&f, "target_class", n)?,
        failure_state: req(&f, "failure_state", n)?,
        freeze_action: req(&f, "freeze_action", n)?,
        fallback_action: req(&f, "fallback_action", n)?,
        rollback_path: req(&f, "rollback_path", n)?,
        last_good_receipt: req(&f, "last_good_receipt", n)?,
        required_challenge: req(&f, "required_challenge", n)?,
        operator_state: req(&f, "operator_state", n)?,
        closure_claim: req(&f, "closure_claim", n)?,
        status: req(&f, "status", n)?,
    })
}
fn parse_rollback(
    n: usize,
    id: &str,
    v: &str,
) -> Result<BootstrapEmergencyRollbackBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapEmergencyRollbackBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&f, "target_id", n)?,
        trigger: req(&f, "trigger", n)?,
        from_state: req(&f, "from_state", n)?,
        to_state: req(&f, "to_state", n)?,
        required_receipts: csv(&req(&f, "required_receipts", n)?),
        replay_gate: req(&f, "replay_gate", n)?,
        frontier_decision: req(&f, "frontier_decision", n)?,
        status: req(&f, "status", n)?,
    })
}
fn parse_receipt(
    n: usize,
    id: &str,
    v: &str,
) -> Result<BootstrapEmergencyFallbackReceiptBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapEmergencyFallbackReceiptBinding {
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
                format!("forbidden emergency fallback phrase {needle}"),
            ));
        }
    }
}
