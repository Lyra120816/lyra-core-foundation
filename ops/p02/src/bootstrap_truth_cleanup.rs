use crate::k0_bootstrap_truth_cleanup::deterministic_bootstrap_truth_cleanup_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_truth_cleanup_model::{
    BootstrapCleanupBinding, BootstrapFrontierAdvanceBinding, BootstrapTruthCleanupReceiptBinding,
    BootstrapTruthCleanupSurface,
};
use std::collections::{BTreeMap, BTreeSet};
pub const P02_BOOTSTRAP_TRUTH_CLEANUP_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-TRUTH-CLEANUP v1";
pub const REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_RULES: &[&str] = &[
    "target_cleanup_must_follow_proven_or_retired_status",
    "proven_target_must_have_execution_receipt",
    "retired_target_must_have_retirement_receipt",
    "cleanup_must_update_truth_snapshot",
    "cleanup_must_update_frontier_lock",
    "cleanup_must_clear_target_local_blocker",
    "cleanup_must_preserve_phase_open_until_all_targets_closed",
    "frontier_advance_requires_no_target_drift",
    "frontier_advance_must_bind_next_frontier",
    "frontier_advance_must_preserve_rollback_path",
    "target_evidence_must_bind_matrix_entry",
    "retired_target_must_bind_extinction_ledger",
    "operator_surfaces_must_not_capture_truth",
    "no_ambient_network_dependency",
    "no_probabilistic_cleanup_truth",
    "no_hidden_randomness",
    "no_ambient_time_cleanup",
    "no_placeholder_cleanup",
    "no_global_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS: &[&str] = &[
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
pub const REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGET_CLASSES: &[&str] =
    &["linux", "windows", "mobile", "wasm", "baremetal", "other"];
pub const REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_target_matrix",
    "receipt_bootstrap_truth_cleanup",
];
const ALLOWED_PROVEN_ACTIONS: &[&str] = &["seal_execution_receipt"];
const ALLOWED_RETIRED_ACTIONS: &[&str] = &["bind_retirement_receipt"];
const ALLOWED_TRUTH_UPDATES: &[&str] = &["mark_target_closed"];
const ALLOWED_BLOCKER_UPDATES: &[&str] = &["clear_target_local_blocker"];
const ALLOWED_FRONTIER_DECISIONS: &[&str] = &["advance_when_all_targets_closed"];
const ALLOWED_ROLLBACK_PATHS: &[&str] = &["rollback_to_target_matrix"];
const ALLOWED_ON_PROVEN: &[&str] = &["target_closed_execution_proven"];
const ALLOWED_ON_RETIRED: &[&str] = &["target_closed_retired"];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    (
        "probabilistic cleanup truth",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
];
pub fn parse_bootstrap_truth_cleanup_surface(
    input: &str,
) -> Result<BootstrapTruthCleanupSurface, Vec<ValidationError>> {
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
            "empty bootstrap truth cleanup surface",
        )]);
    }
    if lines[0] != P02_BOOTSTRAP_TRUTH_CLEANUP_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_TRUTH_CLEANUP_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut target_matrix_receipt = None;
    let mut host_boundary_receipt = None;
    let mut extinction_receipt = None;
    let mut rules = BTreeMap::new();
    let mut cleanups = Vec::new();
    let mut frontiers = Vec::new();
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
        if let Some(id) = left.strip_prefix("cleanup:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateControlSurface,
                    left,
                    "duplicate cleanup",
                ));
            } else {
                match parse_cleanup(n, id, value) {
                    Ok(x) => cleanups.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("frontier:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateControlSurface,
                    left,
                    "duplicate frontier",
                ));
            } else {
                match parse_frontier(n, id, value) {
                    Ok(x) => frontiers.push(x),
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
            "host_boundary_receipt" => host_boundary_receipt = Some(value.to_string()),
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
    Ok(BootstrapTruthCleanupSurface {
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
        host_boundary_receipt: host_boundary_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "host_boundary_receipt",
                "missing host boundary receipt",
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
        cleanups,
        frontiers,
        receipts,
    })
}
pub fn validate_bootstrap_truth_cleanup_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut forbidden = Vec::new();
    scan_forbidden(input, &mut forbidden);
    let verdict = match parse_bootstrap_truth_cleanup_surface(input) {
        Ok(surface) => {
            let mut v = validate_bootstrap_truth_cleanup_model(&surface);
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
pub fn validate_bootstrap_truth_cleanup_model(surface: &BootstrapTruthCleanupSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-007" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-007 got {}", surface.task),
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
    if surface.host_boundary_receipt
        != "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt"
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "host_boundary_receipt",
            "must bind P02-005",
        ));
    }
    if surface.extinction_receipt != "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "extinction_receipt",
            "must bind P02-002",
        ));
    }
    for r in REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_RULES {
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
    for id in REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_RECEIPTS {
        if surface.receipt_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{id}"),
                "missing receipt",
            ));
        }
    }
    let mut cleanup_targets = BTreeSet::new();
    let mut classes = BTreeSet::new();
    for cleanup in &surface.cleanups {
        if !REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS.contains(&cleanup.target_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                cleanup.canonical_identity(),
                "unknown cleanup target",
            ));
        } else {
            cleanup_targets.insert(cleanup.target_id.as_str());
        }
        if !REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGET_CLASSES.contains(&cleanup.target_class.as_str())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                cleanup.canonical_identity(),
                "bad target class",
            ));
        } else {
            classes.insert(cleanup.target_class.as_str());
        }
        if !ALLOWED_PROVEN_ACTIONS.contains(&cleanup.proven_action.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTransitionBinding,
                cleanup.canonical_identity(),
                "bad proven action",
            ));
        }
        if !ALLOWED_RETIRED_ACTIONS.contains(&cleanup.retired_action.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                cleanup.canonical_identity(),
                "bad retired action",
            ));
        }
        if !ALLOWED_TRUTH_UPDATES.contains(&cleanup.truth_update.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlField,
                cleanup.canonical_identity(),
                "bad truth update",
            ));
        }
        if !ALLOWED_BLOCKER_UPDATES.contains(&cleanup.blocker_update.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingBlockerBinding,
                cleanup.canonical_identity(),
                "bad blocker update",
            ));
        }
        if !ALLOWED_FRONTIER_DECISIONS.contains(&cleanup.frontier_decision.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlSurface,
                cleanup.canonical_identity(),
                "bad frontier decision",
            ));
        }
        for required in [
            "receipt_bootstrap_target_matrix",
            "receipt_bootstrap_extinction_ledger",
            "receipt_target_execution_or_retirement",
        ] {
            if !cleanup.required_receipts.iter().any(|x| x == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingReceiptProof,
                    cleanup.canonical_identity(),
                    format!("missing {required}"),
                ));
            }
        }
        if !ALLOWED_ROLLBACK_PATHS.contains(&cleanup.rollback_path.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRollbackPath,
                cleanup.canonical_identity(),
                "bad rollback path",
            ));
        }
        if cleanup.status != "rule_declared" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                cleanup.canonical_identity(),
                "bad cleanup status",
            ));
        }
    }
    for id in REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS {
        if !cleanup_targets.contains(*id) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingControlSurface,
                format!("cleanup_for:{id}"),
                "missing cleanup binding",
            ));
        }
    }
    for class in REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGET_CLASSES {
        if !classes.contains(*class) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target_class:{class}"),
                "missing cleanup class",
            ));
        }
    }
    let mut frontier_targets = BTreeSet::new();
    for frontier in &surface.frontiers {
        if !REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS.contains(&frontier.target_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                frontier.canonical_identity(),
                "unknown frontier target",
            ));
        } else {
            frontier_targets.insert(frontier.target_id.as_str());
        }
        if !ALLOWED_ON_PROVEN.contains(&frontier.on_proven.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTransitionBinding,
                frontier.canonical_identity(),
                "bad on_proven",
            ));
        }
        if !ALLOWED_ON_RETIRED.contains(&frontier.on_retired.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTransitionBinding,
                frontier.canonical_identity(),
                "bad on_retired",
            ));
        }
        if frontier.next_frontier != "P02-008" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidControlSurface,
                frontier.canonical_identity(),
                "bad next frontier",
            ));
        }
        if frontier.hold_if_pending != "local_validation_evidence_missing" {
            errors.push(ValidationError::reject(
                ErrorCode::MissingBlockerBinding,
                frontier.canonical_identity(),
                "bad pending blocker",
            ));
        }
        if frontier.closure_claim != "phase_open" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                frontier.canonical_identity(),
                "cleanup cannot close phase",
            ));
        }
        if frontier.status != "rule_declared" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                frontier.canonical_identity(),
                "bad frontier status",
            ));
        }
    }
    for id in REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS {
        if !frontier_targets.contains(*id) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingControlSurface,
                format!("frontier_for:{id}"),
                "missing frontier binding",
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
    let report_cleanups: Vec<_> = surface
        .cleanups
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.target_id.clone(),
                x.target_class.clone(),
                x.proven_action.clone(),
                x.retired_action.clone(),
                x.truth_update.clone(),
                x.blocker_update.clone(),
                x.frontier_decision.clone(),
                x.required_receipts.clone(),
                x.rollback_path.clone(),
                x.status.clone(),
            )
        })
        .collect();
    let report_frontiers: Vec<_> = surface
        .frontiers
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.target_id.clone(),
                x.on_proven.clone(),
                x.on_retired.clone(),
                x.next_frontier.clone(),
                x.hold_if_pending.clone(),
                x.closure_claim.clone(),
            )
        })
        .collect();
    let report = deterministic_bootstrap_truth_cleanup_report(&report_cleanups, &report_frontiers);
    if report.cleanup_count < REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS.len()
        || report.frontier_count < REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS.len()
        || report.target_class_count < REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGET_CLASSES.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidControlSurface,
            "truth_cleanup_report",
            "insufficient cleanup coverage",
        ));
    }
    if report.phase_open_count != surface.frontiers.len() {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "truth_cleanup_report",
            "frontier cleanup attempted phase closure",
        ));
    }
    if report.rollback_count != surface.cleanups.len() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRollbackPath,
            "truth_cleanup_report",
            "cleanup without target-matrix rollback",
        ));
    }
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}
fn parse_cleanup(n: usize, id: &str, v: &str) -> Result<BootstrapCleanupBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapCleanupBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&f, "target_id", n)?,
        target_class: req(&f, "target_class", n)?,
        proven_action: req(&f, "proven_action", n)?,
        retired_action: req(&f, "retired_action", n)?,
        truth_update: req(&f, "truth_update", n)?,
        blocker_update: req(&f, "blocker_update", n)?,
        frontier_decision: req(&f, "frontier_decision", n)?,
        required_receipts: csv(&req(&f, "required_receipts", n)?),
        rollback_path: req(&f, "rollback_path", n)?,
        status: req(&f, "status", n)?,
    })
}
fn parse_frontier(
    n: usize,
    id: &str,
    v: &str,
) -> Result<BootstrapFrontierAdvanceBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapFrontierAdvanceBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&f, "target_id", n)?,
        on_proven: req(&f, "on_proven", n)?,
        on_retired: req(&f, "on_retired", n)?,
        next_frontier: req(&f, "next_frontier", n)?,
        hold_if_pending: req(&f, "hold_if_pending", n)?,
        closure_claim: req(&f, "closure_claim", n)?,
        status: req(&f, "status", n)?,
    })
}
fn parse_receipt(
    n: usize,
    id: &str,
    v: &str,
) -> Result<BootstrapTruthCleanupReceiptBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapTruthCleanupReceiptBinding {
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
                format!("forbidden cleanup phrase {needle}"),
            ));
        }
    }
}
