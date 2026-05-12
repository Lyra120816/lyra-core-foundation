use crate::k0_bootstrap_target_matrix::deterministic_bootstrap_target_matrix_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_target_matrix_model::{
    BootstrapTargetBinding, BootstrapTargetMatrixSurface, BootstrapTargetProofBinding,
    BootstrapTargetReceiptBinding,
};
use std::collections::{BTreeMap, BTreeSet};
pub const P02_BOOTSTRAP_TARGET_MATRIX_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-TARGET-MATRIX v1";
pub const REQUIRED_BOOTSTRAP_TARGET_MATRIX_RULES: &[&str] = &[
    "every_declared_target_must_have_target_matrix_entry",
    "linux_windows_mobile_wasm_baremetal_must_be_represented",
    "each_target_must_bind_seed_runtime_contract",
    "each_target_must_bind_host_boundary_challenge",
    "each_target_must_declare_architecture_and_runtime_lane",
    "each_target_must_have_all_required_proof_families",
    "proofs_must_bind_containment_gate",
    "proofs_must_remain_pending_until_local_execution",
    "unproven_targets_must_not_close_phase",
    "target_matrix_must_bind_p02_001_p02_002_p02_003_p02_005_receipts",
    "operator_surfaces_must_not_capture_truth",
    "no_ambient_network_dependency",
    "no_probabilistic_target_truth",
    "no_hidden_randomness",
    "no_ambient_time_target_matrix",
    "no_placeholder_target_matrix",
    "no_global_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_TARGETS: &[&str] = &[
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
pub const REQUIRED_BOOTSTRAP_TARGET_CLASSES: &[&str] =
    &["linux", "windows", "mobile", "wasm", "baremetal", "other"];
pub const REQUIRED_BOOTSTRAP_TARGET_PROOF_FAMILIES: &[&str] = &[
    "canonical_io",
    "deterministic_replay",
    "host_boundary",
    "receipt_chain",
    "rollback_lane",
];
pub const REQUIRED_BOOTSTRAP_TARGET_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_surface_inventory",
    "receipt_bootstrap_extinction_ledger",
    "receipt_seed_runtime_contracts",
    "receipt_host_boundary_challenge_suites",
    "receipt_bootstrap_target_matrix",
];
const ALLOWED_ARCHITECTURES: &[&str] = &["x86_64", "aarch64", "wasm32", "riscv64", "host_abstract"];
const ALLOWED_RUNTIME_LANES: &[&str] = &[
    "seed_runtime_posix",
    "seed_runtime_win32",
    "seed_runtime_mobile_sandbox",
    "seed_runtime_wasm_component",
    "seed_runtime_baremetal",
    "seed_runtime_quarantine",
];
const ALLOWED_PROOF_MODES: &[&str] = &[
    "native_bootstrap",
    "bounded_platform_bootstrap",
    "wasm_bootstrap",
    "baremetal_bootstrap",
    "bounded_observer_bootstrap",
];
const ALLOWED_OWNER_ROOTS: &[&str] = &["k0", "ops", "shells", "lyralang"];
const ALLOWED_BOOTSTRAP_SURFACES: &[&str] = &[
    "surface:host_operating_system",
    "surface:physical_cpu_instruction_set",
    "surface:lyra_text_contract_carrier",
    "surface:host_process_launcher",
];
const ALLOWED_GATES: &[&str] = &[
    "gate_no_ambient_network",
    "gate_no_ambient_time",
    "gate_no_foreign_runtime_import",
    "gate_no_unledgered_surface",
    "gate_no_truth_ownership",
];
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
];
pub fn parse_bootstrap_target_matrix_surface(
    input: &str,
) -> Result<BootstrapTargetMatrixSurface, Vec<ValidationError>> {
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
            "empty bootstrap target matrix surface",
        )]);
    }
    if lines[0] != P02_BOOTSTRAP_TARGET_MATRIX_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_TARGET_MATRIX_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut inventory_receipt = None;
    let mut extinction_receipt = None;
    let mut seed_runtime_receipt = None;
    let mut host_boundary_receipt = None;
    let mut rules = BTreeMap::new();
    let mut targets = Vec::new();
    let mut proofs = Vec::new();
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
        if let Some(id) = left.strip_prefix("target:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeploymentTarget,
                    left,
                    "duplicate target",
                ));
            } else {
                match parse_target(n, id, value) {
                    Ok(x) => targets.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("proof:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateProofBinding,
                    left,
                    "duplicate proof",
                ));
            } else {
                match parse_proof(n, id, value) {
                    Ok(x) => proofs.push(x),
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
            "inventory_receipt" => inventory_receipt = Some(value.to_string()),
            "extinction_receipt" => extinction_receipt = Some(value.to_string()),
            "seed_runtime_receipt" => seed_runtime_receipt = Some(value.to_string()),
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
    Ok(BootstrapTargetMatrixSurface {
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
        inventory_receipt: inventory_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "inventory_receipt",
                "missing inventory receipt",
            )]
        })?,
        extinction_receipt: extinction_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "extinction_receipt",
                "missing extinction receipt",
            )]
        })?,
        seed_runtime_receipt: seed_runtime_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "seed_runtime_receipt",
                "missing seed-runtime receipt",
            )]
        })?,
        host_boundary_receipt: host_boundary_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "host_boundary_receipt",
                "missing host-boundary receipt",
            )]
        })?,
        rules,
        targets,
        proofs,
        receipts,
    })
}
pub fn validate_bootstrap_target_matrix_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut forbidden = Vec::new();
    scan_forbidden(input, &mut forbidden);
    let verdict = match parse_bootstrap_target_matrix_surface(input) {
        Ok(surface) => {
            let mut v = validate_bootstrap_target_matrix_model(&surface);
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
pub fn validate_bootstrap_target_matrix_model(surface: &BootstrapTargetMatrixSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-006" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-006 got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported {}", surface.status),
        ));
    }
    for r in REQUIRED_BOOTSTRAP_TARGET_MATRIX_RULES {
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
    for id in REQUIRED_BOOTSTRAP_TARGETS {
        if surface.target_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target:{id}"),
                "missing target",
            ));
        }
    }
    for id in REQUIRED_BOOTSTRAP_TARGET_RECEIPTS {
        if surface.receipt_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{id}"),
                "missing receipt",
            ));
        }
    }
    if surface.inventory_receipt != "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "inventory_receipt",
            "must bind P02-001",
        ));
    }
    if surface.extinction_receipt != "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "extinction_receipt",
            "must bind P02-002",
        ));
    }
    if surface.seed_runtime_receipt != "receipts/p02/pass_0061_seed_runtime_contracts.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "seed_runtime_receipt",
            "must bind P02-003",
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
    let mut classes = BTreeSet::new();
    for target in &surface.targets {
        if !REQUIRED_BOOTSTRAP_TARGET_CLASSES.contains(&target.target_class.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                "bad target class",
            ));
        } else {
            classes.insert(target.target_class.as_str());
        }
        if !ALLOWED_ARCHITECTURES.contains(&target.architecture.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                "bad architecture",
            ));
        }
        if !ALLOWED_RUNTIME_LANES.contains(&target.runtime_lane.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                "bad runtime lane",
            ));
        }
        if !ALLOWED_PROOF_MODES.contains(&target.proof_mode.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                "bad proof mode",
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&target.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                target.canonical_identity(),
                "bad owner root",
            ));
        }
        if !ALLOWED_BOOTSTRAP_SURFACES.contains(&target.bootstrap_surface.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                target.canonical_identity(),
                "bad bootstrap surface",
            ));
        }
        if !target
            .evidence
            .iter()
            .any(|x| x == "receipt_seed_runtime_contracts")
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                target.canonical_identity(),
                "missing seed-runtime receipt evidence",
            ));
        }
        if !target
            .evidence
            .iter()
            .any(|x| x == "receipt_host_boundary_challenge_suites")
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                target.canonical_identity(),
                "missing host-boundary receipt evidence",
            ));
        }
        if target.status != "matrix_declared" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                target.canonical_identity(),
                "bad target status",
            ));
        }
    }
    for class in REQUIRED_BOOTSTRAP_TARGET_CLASSES {
        if !classes.contains(*class) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target_class:{class}"),
                "missing target class",
            ));
        }
    }
    let mut proof_pairs = BTreeSet::new();
    for proof in &surface.proofs {
        if surface.target_by_id(&proof.target_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                proof.canonical_identity(),
                "unknown target id",
            ));
        }
        if !REQUIRED_BOOTSTRAP_TARGET_PROOF_FAMILIES.contains(&proof.proof_family.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                proof.canonical_identity(),
                "bad proof family",
            ));
        }
        if !ALLOWED_GATES.contains(&proof.host_boundary_gate.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                proof.canonical_identity(),
                "bad host-boundary gate",
            ));
        }
        for required in [
            "fixture_positive",
            "fixture_negative",
            "receipt_replay",
            "receipt_host_boundary_challenge_suites",
        ] {
            if !proof.required_evidence.iter().any(|x| x == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingEvidenceBinding,
                    proof.canonical_identity(),
                    format!("missing {required}"),
                ));
            }
        }
        if proof.status != "pending_local_validation" && proof.status != "proof_declared" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                proof.canonical_identity(),
                "bad proof status",
            ));
        }
        proof_pairs.insert((proof.target_id.as_str(), proof.proof_family.as_str()));
    }
    for target in &surface.targets {
        for family in REQUIRED_BOOTSTRAP_TARGET_PROOF_FAMILIES {
            if !proof_pairs.contains(&(target.id.as_str(), *family)) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingProofBinding,
                    format!("target:{}:{family}", target.id),
                    "missing target proof family",
                ));
            }
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
    let report_targets: Vec<_> = surface
        .targets
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.target_class.clone(),
                x.architecture.clone(),
                x.runtime_lane.clone(),
                x.proof_mode.clone(),
                x.owner_root.clone(),
                x.bootstrap_surface.clone(),
                x.evidence.clone(),
            )
        })
        .collect();
    let report_proofs: Vec<_> = surface
        .proofs
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.target_id.clone(),
                x.proof_family.clone(),
                x.required_evidence.clone(),
                x.host_boundary_gate.clone(),
                x.status.clone(),
            )
        })
        .collect();
    let report = deterministic_bootstrap_target_matrix_report(&report_targets, &report_proofs);
    if report.target_count < REQUIRED_BOOTSTRAP_TARGETS.len()
        || report.proof_family_count < REQUIRED_BOOTSTRAP_TARGET_PROOF_FAMILIES.len()
        || report.target_class_count < REQUIRED_BOOTSTRAP_TARGET_CLASSES.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidDeploymentTarget,
            "target_matrix_report",
            "insufficient coverage",
        ));
    }
    if report.pending_validation_count != surface.proofs.len() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureOutputPremature,
            "target_matrix_report",
            "target proof prematurely closed",
        ));
    }
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}
fn parse_target(n: usize, id: &str, v: &str) -> Result<BootstrapTargetBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapTargetBinding {
        line_number: n,
        id: id.to_string(),
        target_class: req(&f, "target_class", n)?,
        architecture: req(&f, "architecture", n)?,
        runtime_lane: req(&f, "runtime_lane", n)?,
        proof_mode: req(&f, "proof_mode", n)?,
        owner_root: req(&f, "owner_root", n)?,
        bootstrap_surface: req(&f, "bootstrap_surface", n)?,
        evidence: csv(&req(&f, "evidence", n)?),
        status: req(&f, "status", n)?,
    })
}
fn parse_proof(
    n: usize,
    id: &str,
    v: &str,
) -> Result<BootstrapTargetProofBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapTargetProofBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&f, "target_id", n)?,
        proof_family: req(&f, "proof_family", n)?,
        required_evidence: csv(&req(&f, "required_evidence", n)?),
        host_boundary_gate: req(&f, "host_boundary_gate", n)?,
        status: req(&f, "status", n)?,
    })
}
fn parse_receipt(
    n: usize,
    id: &str,
    v: &str,
) -> Result<BootstrapTargetReceiptBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(BootstrapTargetReceiptBinding {
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
                format!("forbidden target-matrix phrase {needle}"),
            ));
        }
    }
}
