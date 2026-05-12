use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_host_boundary_challenge::deterministic_host_boundary_challenge_report;
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_host_boundary_challenge_model::{
    HostBoundaryChallengeReceiptBinding, HostBoundaryChallengeSuiteBinding,
    HostBoundaryChallengeSurface, HostBoundaryProbeBinding,
};
use std::collections::{BTreeMap, BTreeSet};

pub const P02_HOST_BOUNDARY_CHALLENGE_CONTRACT: &str = "LYRA-P02-HOST-BOUNDARY-CHALLENGE-SUITES v1";
pub const REQUIRED_HOST_BOUNDARY_RULES: &[&str] = &[
    "every_inventory_surface_must_have_host_boundary_probe",
    "foreign_surfaces_must_not_become_ambient_architecture",
    "every_probe_must_bind_expected_rejection",
    "every_suite_must_bind_extinction_ledger_entry",
    "every_probe_must_bind_containment_gate",
    "operator_surfaces_must_not_own_truth",
    "foreign_runtime_surfaces_must_remain_quarantined",
    "unledgered_host_surfaces_must_be_rejected",
    "challenge_surface_must_bind_p02_001_p02_002_p02_004_receipts",
    "target_matrix_lane_must_remain_pending_until_p02_006",
    "no_ambient_network_dependency",
    "no_probabilistic_challenge_truth",
    "no_hidden_randomness",
    "no_ambient_time_challenge",
    "no_placeholder_challenge_suite",
    "no_global_phase_closure_claim",
];
pub const REQUIRED_HOST_BOUNDARY_SUITES: &[&str] = &[
    "suite_no_ambient_network_import",
    "suite_no_ambient_time_truth",
    "suite_no_hidden_randomness_truth",
    "suite_no_unledgered_host_surface",
    "suite_no_foreign_semantic_ownership",
    "suite_operator_truth_containment",
    "suite_foreign_runtime_quarantine",
];
pub const REQUIRED_HOST_BOUNDARY_PROBES: &[&str] = &[
    "probe_artifact_generation_python_helper",
    "probe_cargo_build_driver",
    "probe_cursor_codex_assisted_editor",
    "probe_external_sha256sum_tool",
    "probe_external_wall_clock",
    "probe_external_zip_packager",
    "probe_git_repository_transport",
    "probe_host_filesystem",
    "probe_host_operating_system",
    "probe_host_process_launcher",
    "probe_lyra_text_contract_carrier",
    "probe_lyralang_bootstrap_stub_carrier",
    "probe_operator_shell_terminal",
    "probe_physical_cpu_instruction_set",
    "probe_rust_bootstrap_compiler",
    "probe_rust_std_runtime",
    "probe_unbounded_network_bootstrap_fetch",
];
pub const REQUIRED_HOST_BOUNDARY_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_surface_inventory",
    "receipt_bootstrap_extinction_ledger",
    "receipt_bootstrap_session_rituals",
    "receipt_host_boundary_challenge_suites",
];
const REQUIRED_SURFACE_REFS: &[&str] = &[
    "surface:artifact_generation_python_helper",
    "surface:cargo_build_driver",
    "surface:cursor_codex_assisted_editor",
    "surface:external_sha256sum_tool",
    "surface:external_wall_clock",
    "surface:external_zip_packager",
    "surface:git_repository_transport",
    "surface:host_filesystem",
    "surface:host_operating_system",
    "surface:host_process_launcher",
    "surface:lyra_text_contract_carrier",
    "surface:lyralang_bootstrap_stub_carrier",
    "surface:operator_shell_terminal",
    "surface:physical_cpu_instruction_set",
    "surface:rust_bootstrap_compiler",
    "surface:rust_std_runtime",
    "surface:unbounded_network_bootstrap_fetch",
];
const ALLOWED_OWNER_ROOTS: &[&str] = &["ops", "shells", "k0"];
const ALLOWED_SUITE_KINDS: &[&str] = &[
    "ambient_network_rejection",
    "ambient_time_rejection",
    "hidden_randomness_rejection",
    "unledgered_surface_rejection",
    "semantic_ownership_rejection",
    "operator_truth_containment",
    "foreign_runtime_quarantine",
];
const ALLOWED_REJECTIONS: &[&str] = &[
    "ambient_network_allowed",
    "ambient_time_allowed",
    "hidden_randomness_allowed",
    "ambient_authority",
    "root_ownership_violation",
    "closure_proof_unbound",
    "probabilistic_truth_allowed",
];
const ALLOWED_CONTAINMENT_GATES: &[&str] = &[
    "gate_no_ambient_network",
    "gate_no_ambient_time",
    "gate_no_hidden_randomness",
    "gate_no_unledgered_surface",
    "gate_no_truth_ownership",
    "gate_no_foreign_runtime_import",
    "gate_no_probabilistic_truth",
];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("foreign owner", ErrorCode::RootOwnershipViolation),
    ("host owns truth", ErrorCode::RootOwnershipViolation),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_host_boundary_challenge_surface(
    input: &str,
) -> Result<HostBoundaryChallengeSurface, Vec<ValidationError>> {
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
            "empty host-boundary challenge surface",
        )]);
    }
    if lines[0] != P02_HOST_BOUNDARY_CHALLENGE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_HOST_BOUNDARY_CHALLENGE_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut inventory_receipt = None;
    let mut extinction_receipt = None;
    let mut session_receipt = None;
    let mut rules = BTreeMap::new();
    let mut suites = Vec::new();
    let mut probes = Vec::new();
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
        if let Some(id) = left.strip_prefix("suite:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    left,
                    "duplicate suite",
                ));
            } else {
                match parse_suite(n, id, value) {
                    Ok(x) => suites.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("probe:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateChallengeFixture,
                    left,
                    "duplicate probe",
                ));
            } else {
                match parse_probe(n, id, value) {
                    Ok(x) => probes.push(x),
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
            "session_receipt" => session_receipt = Some(value.to_string()),
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
    Ok(HostBoundaryChallengeSurface {
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
        session_receipt: session_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "session_receipt",
                "missing session receipt",
            )]
        })?,
        rules,
        suites,
        probes,
        receipts,
    })
}

pub fn validate_host_boundary_challenge_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut forbidden = Vec::new();
    scan_forbidden(input, &mut forbidden);
    let verdict = match parse_host_boundary_challenge_surface(input) {
        Ok(surface) => {
            let mut v = validate_host_boundary_challenge_model(&surface);
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
pub fn validate_host_boundary_challenge_model(surface: &HostBoundaryChallengeSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-005" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-005 got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported {}", surface.status),
        ));
    }
    for r in REQUIRED_HOST_BOUNDARY_RULES {
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
    for id in REQUIRED_HOST_BOUNDARY_SUITES {
        if surface.suite_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("suite:{id}"),
                "missing suite",
            ));
        }
    }
    for id in REQUIRED_HOST_BOUNDARY_PROBES {
        if surface.probe_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeFixture,
                format!("probe:{id}"),
                "missing probe",
            ));
        }
    }
    for id in REQUIRED_HOST_BOUNDARY_RECEIPTS {
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
    if surface.session_receipt != "receipts/p02/pass_0062_bootstrap_session_rituals.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "session_receipt",
            "must bind P02-004",
        ));
    }
    for suite in &surface.suites {
        if !ALLOWED_OWNER_ROOTS.contains(&suite.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                suite.canonical_identity(),
                "invalid owner root",
            ));
        }
        if !REQUIRED_SURFACE_REFS.contains(&suite.boundary_surface.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                suite.canonical_identity(),
                "unknown boundary surface",
            ));
        }
        if !ALLOWED_SUITE_KINDS.contains(&suite.suite_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                suite.canonical_identity(),
                "bad suite kind",
            ));
        }
        if !ALLOWED_REJECTIONS.contains(&suite.expected_rejection.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                suite.canonical_identity(),
                "bad expected rejection",
            ));
        }
        if suite.evidence.is_empty() || suite.evidence.iter().any(|x| x.is_empty()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                suite.canonical_identity(),
                "missing evidence",
            ));
        }
        if !suite
            .evidence
            .iter()
            .any(|x| x == "receipt_bootstrap_extinction_ledger")
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                suite.canonical_identity(),
                "missing extinction receipt evidence",
            ));
        }
        if surface.receipt_by_id(&suite.receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                suite.canonical_identity(),
                "suite receipt not declared",
            ));
        }
        if suite.status != "challenge_emitted" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                suite.canonical_identity(),
                "bad suite status",
            ));
        }
    }
    let mut suite_refs = BTreeSet::new();
    for probe in &surface.probes {
        if surface.suite_by_id(&probe.suite_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                probe.canonical_identity(),
                "unknown suite id",
            ));
        }
        if !REQUIRED_SURFACE_REFS.contains(&probe.surface_ref.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                probe.canonical_identity(),
                "unknown surface ref",
            ));
        }
        if probe.injected_claim == "ambient_architecture_allowed"
            || probe.injected_claim == "truth_successor_allowed"
        {
            errors.push(ValidationError::reject(
                ErrorCode::NegativeFixtureAccepted,
                probe.canonical_identity(),
                "ambient architecture accepted",
            ));
        }
        if !ALLOWED_REJECTIONS.contains(&probe.expected_error.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                probe.canonical_identity(),
                "bad expected error",
            ));
        }
        if !ALLOWED_CONTAINMENT_GATES.contains(&probe.containment_gate.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                probe.canonical_identity(),
                "bad containment gate",
            ));
        }
        if probe.evidence.is_empty() || probe.evidence.iter().any(|x| x.is_empty()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                probe.canonical_identity(),
                "missing evidence",
            ));
        }
        if !probe
            .evidence
            .iter()
            .any(|x| x == "receipt_host_boundary_challenge_suites")
        {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                probe.canonical_identity(),
                "missing host-boundary receipt",
            ));
        }
        if probe.status != "probe_rejected" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                probe.canonical_identity(),
                "bad probe status",
            ));
        }
        suite_refs.insert(probe.suite_id.as_str());
    }
    for r in REQUIRED_SURFACE_REFS {
        if surface.probe_for_surface(r).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeFixture,
                format!("probe_for:{r}"),
                "missing surface probe",
            ));
        }
    }
    for suite in &surface.suites {
        if !suite_refs.contains(suite.id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeFixture,
                suite.canonical_identity(),
                "suite has no probe",
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
    let suites: Vec<_> = surface
        .suites
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.owner_root.clone(),
                x.boundary_surface.clone(),
                x.suite_kind.clone(),
                x.challenge_scope.clone(),
                x.adversarial_vector.clone(),
                x.expected_rejection.clone(),
                x.evidence.clone(),
            )
        })
        .collect();
    let probes: Vec<_> = surface
        .probes
        .iter()
        .map(|x| {
            (
                x.id.clone(),
                x.suite_id.clone(),
                x.surface_ref.clone(),
                x.injected_claim.clone(),
                x.expected_error.clone(),
                x.containment_gate.clone(),
                x.evidence.clone(),
            )
        })
        .collect();
    let report = deterministic_host_boundary_challenge_report(&suites, &probes);
    if report.suite_count < REQUIRED_HOST_BOUNDARY_SUITES.len()
        || report.probe_count < REQUIRED_HOST_BOUNDARY_PROBES.len()
        || report.covered_surface_count < REQUIRED_SURFACE_REFS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeFixture,
            "host_boundary_report",
            "insufficient coverage",
        ));
    }
    if report.rejected_probe_count != surface.probes.len() {
        errors.push(ValidationError::reject(
            ErrorCode::NegativeFixtureAccepted,
            "host_boundary_report",
            "probe accepted",
        ));
    }
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}
fn parse_suite(
    n: usize,
    id: &str,
    v: &str,
) -> Result<HostBoundaryChallengeSuiteBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(HostBoundaryChallengeSuiteBinding {
        line_number: n,
        id: id.to_string(),
        owner_root: req(&f, "owner_root", n)?,
        boundary_surface: req(&f, "boundary_surface", n)?,
        suite_kind: req(&f, "suite_kind", n)?,
        challenge_scope: req(&f, "challenge_scope", n)?,
        adversarial_vector: req(&f, "adversarial_vector", n)?,
        expected_rejection: req(&f, "expected_rejection", n)?,
        evidence: csv(&req(&f, "evidence", n)?),
        receipt: req(&f, "receipt", n)?,
        status: req(&f, "status", n)?,
    })
}
fn parse_probe(n: usize, id: &str, v: &str) -> Result<HostBoundaryProbeBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(HostBoundaryProbeBinding {
        line_number: n,
        id: id.to_string(),
        suite_id: req(&f, "suite_id", n)?,
        surface_ref: req(&f, "surface_ref", n)?,
        injected_claim: req(&f, "injected_claim", n)?,
        expected_error: req(&f, "expected_error", n)?,
        containment_gate: req(&f, "containment_gate", n)?,
        evidence: csv(&req(&f, "evidence", n)?),
        status: req(&f, "status", n)?,
    })
}
fn parse_receipt(
    n: usize,
    id: &str,
    v: &str,
) -> Result<HostBoundaryChallengeReceiptBinding, ValidationError> {
    let f = fields(v, n)?;
    Ok(HostBoundaryChallengeReceiptBinding {
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
                ErrorCode::InvalidChallengeFixture,
                format!("line:{n:03}"),
                "bad field",
            ));
        };
        if k.is_empty()
            || val.is_empty() && k != "evidence"
            || m.insert(k.to_string(), val.to_string()).is_some()
        {
            return Err(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
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
            ErrorCode::InvalidChallengeFixture,
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
                format!("forbidden host-boundary phrase {needle}"),
            ));
        }
    }
}
