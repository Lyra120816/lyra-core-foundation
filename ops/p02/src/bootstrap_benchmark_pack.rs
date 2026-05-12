use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_benchmark_pack::deterministic_bootstrap_benchmark_pack_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_benchmark_pack::{
    bootstrap_benchmark_artifacts_bind_paths, bootstrap_benchmark_evidence_bind_registry,
    bootstrap_benchmark_families_bind_targets, bootstrap_benchmark_no_forbidden_descriptor_claims,
    bootstrap_benchmark_registry_hash, bootstrap_benchmark_targets_bind_receipts,
};
use crate::p02_bootstrap_benchmark_pack_model::{
    BootstrapBenchmarkEvidenceBinding, BootstrapBenchmarkFamilyBinding,
    BootstrapBenchmarkPackSurface, BootstrapBenchmarkTargetBinding,
};

pub const P02_BOOTSTRAP_BENCHMARK_PACK_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-BENCHMARK-PACK v1";
pub const REQUIRED_BOOTSTRAP_BENCHMARK_PACK_RULES: &[&str] = &[
    "bootstrap_benchmark_pack_must_cover_required_families",
    "bootstrap_throughput_targets_must_bind_commands_and_receipts",
    "bootstrap_latency_targets_must_use_static_budget_units",
    "bootstrap_correctness_targets_must_bind_fixtures_goldens_and_receipts",
    "bootstrap_stability_targets_must_bind_replay_and_hash_checks",
    "bootstrap_adversarial_targets_must_bind_redteam_and_capture_rejections",
    "bootstrap_rollback_targets_must_bind_seed_and_host_extinction_reversal",
    "bootstrap_benchmark_evidence_must_bind_targets_artifacts_receipts",
    "p02_x03_must_not_close_global_phase",
    "no_network_dependency",
    "no_docs_only_benchmark_pack",
    "no_unreceipted_benchmark_pack",
];
pub const REQUIRED_BOOTSTRAP_BENCHMARK_FAMILIES: &[&str] = &[
    "throughput",
    "latency",
    "correctness",
    "stability",
    "adversarial",
    "rollback",
];
pub const REQUIRED_BOOTSTRAP_BENCHMARK_TARGETS: &[&str] = &[
    "throughput_bootstrap_surface_validation",
    "throughput_bootstrap_receipt_generation",
    "latency_bootstrap_canonicalization_budget",
    "latency_bootstrap_validation_budget",
    "correctness_bootstrap_valid_surface_acceptance",
    "correctness_bootstrap_negative_corpus_rejection",
    "stability_bootstrap_replay_equivalence",
    "stability_bootstrap_hash_ordering",
    "adversarial_bootstrap_hostile_case_rejection",
    "adversarial_bootstrap_capture_rejection",
    "rollback_bootstrap_seed_reversal",
    "rollback_bootstrap_host_extinction_reversal",
];
pub const REQUIRED_BOOTSTRAP_BENCHMARK_EVIDENCE: &[&str] = &[
    "throughput_bootstrap_benchmark_evidence",
    "latency_bootstrap_benchmark_evidence",
    "correctness_bootstrap_benchmark_evidence",
    "stability_bootstrap_benchmark_evidence",
    "adversarial_bootstrap_benchmark_evidence",
    "rollback_bootstrap_benchmark_evidence",
];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "bounded_closed"];
const ALLOWED_EXPECTED: &[&str] = &[
    "accepted",
    "rejected_expected",
    "accepted_or_rejected_expected",
    "stable_replay",
];
const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p02-bootstrap-closure-check",
    "lyra-p02-bootstrap-proof-family-check",
    "lyra-p02-bootstrap-core-engine-check",
    "lyra-p02-bootstrap-benchmark-pack-check",
    "lyra-p02-bootstrap-falsification-check",
    "lyra-p02-bootstrap-replay-check",
    "lyra-p02-bootstrap-redteam-check",
    "lyra-p02-bootstrap-economics-check",
    "lyra-p02-seed-runtime-replacement-check",
    "lyra-p02-bootstrap-extinction-check",
];
const FORBIDDEN_BOOTSTRAP_BENCHMARK_TEXT: &[(&str, ErrorCode)] = &[
    ("network_required:true", ErrorCode::ClosureNetworkDependency),
    (
        "remote_service_required:true",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("network required", ErrorCode::ClosureNetworkDependency),
    (
        "remote service required",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("docs_only:true", ErrorCode::ClosureDocsOnly),
    ("unreceipted:true", ErrorCode::ClosureUnreceipted),
    (
        "global_phase_closed:true",
        ErrorCode::UnsupportedGlobalClosure,
    ),
    ("phase_closed:true", ErrorCode::FakeClosureClaim),
];

pub fn parse_bootstrap_benchmark_pack_surface(
    input: &str,
) -> Result<BootstrapBenchmarkPackSurface, ValidationError> {
    let lines = canonical_lines(input).map_err(|error| {
        ValidationError::reject(
            ErrorCode::CanonicalControlByte,
            "bootstrap_benchmark_pack",
            format!("canonicalization failed: {error:?}"),
        )
    })?;
    if lines.is_empty() {
        return Err(ValidationError::reject(
            ErrorCode::EmptySurface,
            "bootstrap_benchmark_pack",
            "empty bootstrap benchmark surface",
        ));
    }
    if lines[0] != P02_BOOTSTRAP_BENCHMARK_PACK_CONTRACT {
        return Err(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "bootstrap_benchmark_pack",
            format!("expected {P02_BOOTSTRAP_BENCHMARK_PACK_CONTRACT}"),
        ));
    }
    let mut surface = BootstrapBenchmarkPackSurface {
        header: lines[0].clone(),
        phase: String::new(),
        task: String::new(),
        status: String::new(),
        closure_scope: String::new(),
        global_closure: String::new(),
        next_frontier: String::new(),
        rules: BTreeMap::new(),
        families: Vec::new(),
        targets: Vec::new(),
        evidence: Vec::new(),
    };
    let mut seen = BTreeSet::new();
    for (offset, line) in lines.iter().enumerate().skip(1) {
        let line_number = offset + 1;
        let (key, value) = line.split_once('=').ok_or_else(|| {
            ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "missing '='",
            )
        })?;
        if !seen.insert(key.to_string()) {
            return Err(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                key,
                "duplicate bootstrap benchmark entry",
            ));
        }
        if key == "phase" {
            surface.phase = value.to_string();
        } else if key == "task" {
            surface.task = value.to_string();
        } else if key == "status" {
            surface.status = value.to_string();
        } else if key == "closure_scope" {
            surface.closure_scope = value.to_string();
        } else if key == "global_closure" {
            surface.global_closure = value.to_string();
        } else if key == "next_frontier" {
            surface.next_frontier = value.to_string();
        } else if let Some(name) = key.strip_prefix("rule:") {
            surface.rules.insert(name.to_string(), value.to_string());
        } else if let Some(id) = key.strip_prefix("family:") {
            surface.families.push(parse_family(line_number, id, value)?);
        } else if let Some(id) = key.strip_prefix("target:") {
            surface.targets.push(parse_target(line_number, id, value)?);
        } else if let Some(id) = key.strip_prefix("evidence:") {
            surface
                .evidence
                .push(parse_evidence(line_number, id, value)?);
        } else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown bootstrap benchmark key {key}"),
            ));
        }
    }
    Ok(surface)
}

pub fn validate_bootstrap_benchmark_pack_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut errors = Vec::new();
    scan_forbidden(input, &mut errors);
    match parse_bootstrap_benchmark_pack_surface(input) {
        Ok(surface) => validate_bootstrap_benchmark_pack_model(&surface, &mut errors),
        Err(error) => errors.push(error),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_benchmark_pack_model(
    surface: &BootstrapBenchmarkPackSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "bootstrap benchmark pack phase must be P02",
        ));
    }
    if surface.task != "P02-X03" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "bootstrap benchmark pack task must be P02-X03",
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "bootstrap benchmark pack status must be artifact_emitted",
        ));
    }
    if surface.closure_scope != "extended_open" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "closure_scope",
            "P02-X03 must remain extended_open",
        ));
    }
    if surface.global_closure != "denied" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "global_closure",
            "P02-X03 must deny global closure",
        ));
    }
    if surface.next_frontier != "P02-X04" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "next_frontier",
            "P02-X03 must point to P02-X04",
        ));
    }
    require_rules(surface, errors);
    require_families(surface, errors);
    require_targets(surface, errors);
    require_evidence(surface, errors);
    validate_families(surface, errors);
    validate_targets(surface, errors);
    validate_evidence(surface, errors);
    validate_family_coverage(surface, errors);
    validate_report(surface, errors);
    if !bootstrap_benchmark_artifacts_bind_paths()
        || !bootstrap_benchmark_families_bind_targets()
        || !bootstrap_benchmark_targets_bind_receipts()
        || !bootstrap_benchmark_evidence_bind_registry()
        || !bootstrap_benchmark_no_forbidden_descriptor_claims()
        || bootstrap_benchmark_registry_hash().is_empty()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureProofUnbound,
            "lyralang_bootstrap_benchmark_registry",
            "bootstrap benchmark descriptor registry is not self-bound",
        ));
    }
}

fn parse_family(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapBenchmarkFamilyBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidBenchmarkTarget,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(BootstrapBenchmarkFamilyBinding {
        line_number,
        id: id.to_string(),
        family_kind: required_field(
            &fields,
            "kind",
            line_number,
            ErrorCode::InvalidBenchmarkTarget,
        )?
        .to_string(),
        scope: required_field(
            &fields,
            "scope",
            line_number,
            ErrorCode::InvalidBenchmarkTarget,
        )?
        .to_string(),
        targets: split_list(required_field(
            &fields,
            "targets",
            line_number,
            ErrorCode::MissingBenchmarkTarget,
        )?),
        proofs: split_list(required_field(
            &fields,
            "proofs",
            line_number,
            ErrorCode::MissingEvidenceBinding,
        )?),
        status: required_field(
            &fields,
            "status",
            line_number,
            ErrorCode::UnsupportedClosureStatus,
        )?
        .to_string(),
    })
}

fn parse_target(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapBenchmarkTargetBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidBenchmarkTarget,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(BootstrapBenchmarkTargetBinding {
        line_number,
        id: id.to_string(),
        family: required_field(
            &fields,
            "family",
            line_number,
            ErrorCode::InvalidBenchmarkTarget,
        )?
        .to_string(),
        metric: required_field(
            &fields,
            "metric",
            line_number,
            ErrorCode::InvalidBenchmarkTarget,
        )?
        .to_string(),
        unit: required_field(
            &fields,
            "unit",
            line_number,
            ErrorCode::InvalidBenchmarkTarget,
        )?
        .to_string(),
        threshold: required_field(
            &fields,
            "threshold",
            line_number,
            ErrorCode::InvalidBenchmarkTarget,
        )?
        .to_string(),
        command: required_field(
            &fields,
            "command",
            line_number,
            ErrorCode::MissingCommandRecord,
        )?
        .to_string(),
        fixture: required_field(
            &fields,
            "fixture",
            line_number,
            ErrorCode::MissingChallengeFixture,
        )?
        .to_string(),
        golden: required_field(
            &fields,
            "golden",
            line_number,
            ErrorCode::MissingAcceptanceGolden,
        )?
        .to_string(),
        receipt: required_field(
            &fields,
            "receipt",
            line_number,
            ErrorCode::BenchmarkMissingReceipt,
        )?
        .to_string(),
        expected: required_field(
            &fields,
            "expected",
            line_number,
            ErrorCode::InvalidBenchmarkTarget,
        )?
        .to_string(),
        status: required_field(
            &fields,
            "status",
            line_number,
            ErrorCode::UnsupportedClosureStatus,
        )?
        .to_string(),
    })
}

fn parse_evidence(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapBenchmarkEvidenceBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidEvidenceBinding,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(BootstrapBenchmarkEvidenceBinding {
        line_number,
        id: id.to_string(),
        family: required_field(
            &fields,
            "family",
            line_number,
            ErrorCode::InvalidEvidenceBinding,
        )?
        .to_string(),
        targets: split_list(required_field(
            &fields,
            "targets",
            line_number,
            ErrorCode::MissingBenchmarkTarget,
        )?),
        artifacts: split_list(required_field(
            &fields,
            "artifacts",
            line_number,
            ErrorCode::MissingEvidenceBinding,
        )?),
        proof_receipts: split_list(required_field(
            &fields,
            "proof_receipts",
            line_number,
            ErrorCode::BenchmarkMissingReceipt,
        )?),
        status: required_field(
            &fields,
            "status",
            line_number,
            ErrorCode::UnsupportedClosureStatus,
        )?
        .to_string(),
    })
}

fn require_rules(surface: &BootstrapBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_BOOTSTRAP_BENCHMARK_PACK_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingBenchmarkRule,
                format!("rule:{rule}"),
                format!("unsupported bootstrap benchmark rule value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingBenchmarkRule,
                format!("rule:{rule}"),
                "missing required bootstrap benchmark rule",
            )),
        }
    }
}
fn require_families(surface: &BootstrapBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for family in REQUIRED_BOOTSTRAP_BENCHMARK_FAMILIES {
        if surface.family_by_id(family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingBenchmarkTarget,
                format!("family:{family}"),
                "required bootstrap benchmark family missing",
            ));
        }
    }
}
fn require_targets(surface: &BootstrapBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for target in REQUIRED_BOOTSTRAP_BENCHMARK_TARGETS {
        if surface.target_by_id(target).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingBenchmarkTarget,
                format!("target:{target}"),
                "required bootstrap benchmark target missing",
            ));
        }
    }
}
fn require_evidence(surface: &BootstrapBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for evidence in REQUIRED_BOOTSTRAP_BENCHMARK_EVIDENCE {
        if surface.evidence_by_id(evidence).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                format!("evidence:{evidence}"),
                "required bootstrap benchmark evidence missing",
            ));
        }
    }
}

fn validate_families(surface: &BootstrapBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for family in &surface.families {
        if family.family_kind != family.id || !is_required_family(&family.family_kind) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                family.canonical_identity(),
                format!(
                    "invalid bootstrap benchmark family kind {}",
                    family.family_kind
                ),
            ));
        }
        if family.scope != "P02" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                family.canonical_identity(),
                "bootstrap benchmark family scope must be P02",
            ));
        }
        if family.targets.is_empty() || family.proofs.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                family.canonical_identity(),
                "bootstrap benchmark family must bind targets and proof labels",
            ));
        }
        if !ALLOWED_STATUSES.contains(&family.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                family.canonical_identity(),
                format!(
                    "unsupported bootstrap benchmark family status {}",
                    family.status
                ),
            ));
        }
        for target in &family.targets {
            match surface.target_by_id(target) {
                Some(row) if row.family == family.id => {}
                Some(_) => errors.push(ValidationError::reject(
                    ErrorCode::InvalidBenchmarkTarget,
                    family.canonical_identity(),
                    format!("family target {target} binds wrong family"),
                )),
                None => errors.push(ValidationError::reject(
                    ErrorCode::MissingBenchmarkTarget,
                    family.canonical_identity(),
                    format!("unknown bootstrap benchmark target {target}"),
                )),
            }
        }
        for proof in &family.proofs {
            if !is_symbolic_name(proof) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEvidenceBinding,
                    family.canonical_identity(),
                    format!("invalid bootstrap benchmark proof label {proof}"),
                ));
            }
        }
    }
}

fn validate_targets(surface: &BootstrapBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for target in &surface.targets {
        if !is_required_family(&target.family) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                target.canonical_identity(),
                format!("unknown bootstrap benchmark family {}", target.family),
            ));
        }
        if surface.family_by_id(&target.family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                target.canonical_identity(),
                format!("target family not declared {}", target.family),
            ));
        }
        if !is_symbolic_name(&target.metric) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                target.canonical_identity(),
                format!("invalid bootstrap benchmark metric {}", target.metric),
            ));
        }
        if !valid_family_unit(&target.family, &target.unit) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                target.canonical_identity(),
                format!("invalid unit {} for family {}", target.unit, target.family),
            ));
        }
        if !valid_threshold(&target.family, &target.threshold) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                target.canonical_identity(),
                format!(
                    "invalid threshold {} for family {}",
                    target.threshold, target.family
                ),
            ));
        }
        if !REQUIRED_COMMANDS.contains(&target.command.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCommandRecord,
                target.canonical_identity(),
                format!("unknown bootstrap benchmark command {}", target.command),
            ));
        }
        if !target.fixture.starts_with("fixtures/p02/") || !target.fixture.ends_with(".lyra") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                target.canonical_identity(),
                format!("fixture path must be a P02 fixture: {}", target.fixture),
            ));
        }
        if !target.golden.starts_with("goldens/p02/") || !target.golden.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                target.canonical_identity(),
                format!(
                    "golden path must be a P02 receipt golden: {}",
                    target.golden
                ),
            ));
        }
        if !target.receipt.starts_with("receipts/p02/") || !target.receipt.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::BenchmarkMissingReceipt,
                target.canonical_identity(),
                format!("receipt path must be a P02 receipt: {}", target.receipt),
            ));
        }
        if !ALLOWED_EXPECTED.contains(&target.expected.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidBenchmarkTarget,
                target.canonical_identity(),
                format!(
                    "unsupported bootstrap benchmark expected verdict {}",
                    target.expected
                ),
            ));
        }
        if !ALLOWED_STATUSES.contains(&target.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                target.canonical_identity(),
                format!(
                    "unsupported bootstrap benchmark target status {}",
                    target.status
                ),
            ));
        }
    }
}

fn validate_evidence(surface: &BootstrapBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for evidence in &surface.evidence {
        if !is_required_family(&evidence.family) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEvidenceBinding,
                evidence.canonical_identity(),
                format!(
                    "unknown bootstrap benchmark evidence family {}",
                    evidence.family
                ),
            ));
        }
        if surface.family_by_id(&evidence.family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEvidenceBinding,
                evidence.canonical_identity(),
                format!("evidence family not declared {}", evidence.family),
            ));
        }
        if evidence.targets.is_empty()
            || evidence.artifacts.is_empty()
            || evidence.proof_receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEvidenceBinding,
                evidence.canonical_identity(),
                "bootstrap benchmark evidence must bind targets, artifacts, and proof receipts",
            ));
        }
        for target in &evidence.targets {
            match surface.target_by_id(target) {
                Some(row) if row.family == evidence.family => {}
                Some(_) => errors.push(ValidationError::reject(
                    ErrorCode::InvalidEvidenceBinding,
                    evidence.canonical_identity(),
                    format!("evidence target {target} binds wrong family"),
                )),
                None => errors.push(ValidationError::reject(
                    ErrorCode::MissingBenchmarkTarget,
                    evidence.canonical_identity(),
                    format!("unknown bootstrap benchmark evidence target {target}"),
                )),
            }
        }
        for artifact in &evidence.artifacts {
            if !valid_artifact_path(artifact) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEvidenceBinding,
                    evidence.canonical_identity(),
                    format!("invalid bootstrap benchmark evidence artifact {artifact}"),
                ));
            }
        }
        for receipt in &evidence.proof_receipts {
            if !receipt.starts_with("receipts/p02/") || !receipt.ends_with(".receipt") {
                errors.push(ValidationError::reject(
                    ErrorCode::BenchmarkMissingReceipt,
                    evidence.canonical_identity(),
                    format!("invalid bootstrap benchmark proof receipt {receipt}"),
                ));
            }
        }
        if !ALLOWED_STATUSES.contains(&evidence.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                evidence.canonical_identity(),
                format!(
                    "unsupported bootstrap benchmark evidence status {}",
                    evidence.status
                ),
            ));
        }
    }
}

fn validate_family_coverage(
    surface: &BootstrapBenchmarkPackSurface,
    errors: &mut Vec<ValidationError>,
) {
    for family_name in REQUIRED_BOOTSTRAP_BENCHMARK_FAMILIES {
        if let Some(family) = surface.family_by_id(family_name) {
            let family_targets = family
                .targets
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>();
            let actual_targets = surface
                .targets
                .iter()
                .filter(|target| target.family == *family_name)
                .map(|target| target.id.as_str())
                .collect::<BTreeSet<_>>();
            for actual in &actual_targets {
                if !family_targets.contains(actual) {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidBenchmarkTarget,
                        family.canonical_identity(),
                        format!("family omits declared bootstrap benchmark target {actual}"),
                    ));
                }
            }
            for listed in &family_targets {
                if !actual_targets.contains(listed) {
                    errors.push(ValidationError::reject(
                        ErrorCode::InvalidBenchmarkTarget,
                        family.canonical_identity(),
                        format!(
                            "family lists target outside its bootstrap benchmark family {listed}"
                        ),
                    ));
                }
            }
        }
    }
}

fn validate_report(surface: &BootstrapBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    let families = surface
        .families
        .iter()
        .map(|family| {
            (
                family.id.clone(),
                family.family_kind.clone(),
                family.targets.clone(),
                family.proofs.clone(),
                family.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let targets = surface
        .targets
        .iter()
        .map(|target| {
            (
                target.id.clone(),
                target.family.clone(),
                target.metric.clone(),
                target.unit.clone(),
                target.threshold.clone(),
                target.command.clone(),
                target.fixture.clone(),
                target.golden.clone(),
                target.receipt.clone(),
                target.expected.clone(),
                target.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let evidence = surface
        .evidence
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.family.clone(),
                item.targets.clone(),
                item.artifacts.clone(),
                item.proof_receipts.clone(),
                item.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let report = deterministic_bootstrap_benchmark_pack_report(&families, &targets, &evidence);
    if report.family_count < REQUIRED_BOOTSTRAP_BENCHMARK_FAMILIES.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBenchmarkTarget,
            "benchmark_report",
            "bootstrap benchmark report is missing families",
        ));
    }
    if report.target_count < REQUIRED_BOOTSTRAP_BENCHMARK_TARGETS.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBenchmarkTarget,
            "benchmark_report",
            "bootstrap benchmark report is missing targets",
        ));
    }
    if report.evidence_count < REQUIRED_BOOTSTRAP_BENCHMARK_EVIDENCE.len() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingEvidenceBinding,
            "benchmark_report",
            "bootstrap benchmark report is missing evidence",
        ));
    }
    if report.throughput_target_count < 2
        || report.latency_target_count < 2
        || report.correctness_target_count < 2
        || report.stability_target_count < 2
        || report.adversarial_target_count < 2
        || report.rollback_target_count < 2
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBenchmarkTarget,
            "benchmark_report",
            "each bootstrap benchmark family must expose at least two targets",
        ));
    }
    if !report.pack_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEvidenceBinding,
            "benchmark_report",
            "bootstrap benchmark report hash must be stable fnv1a128",
        ));
    }
}

fn scan_forbidden(input: &str, errors: &mut Vec<ValidationError>) {
    let lower = input.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_BOOTSTRAP_BENCHMARK_TEXT {
        if lower.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "bootstrap_benchmark_forbidden_text",
                format!("forbidden token {token}"),
            ));
        }
    }
}

fn parse_fields(value: &str) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        let (key, val) = segment
            .split_once(':')
            .ok_or_else(|| format!("field segment missing ':' -> {segment}"))?;
        if key.is_empty() || val.is_empty() {
            return Err(format!("empty field in segment {segment}"));
        }
        if fields.insert(key.to_string(), val.to_string()).is_some() {
            return Err(format!("duplicate field {key}"));
        }
    }
    Ok(fields)
}
fn required_field<'a>(
    fields: &'a BTreeMap<String, String>,
    name: &str,
    line_number: usize,
    code: ErrorCode,
) -> Result<&'a str, ValidationError> {
    fields.get(name).map(String::as_str).ok_or_else(|| {
        ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("missing field {name}"),
        )
    })
}
fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}
fn is_required_family(value: &str) -> bool {
    REQUIRED_BOOTSTRAP_BENCHMARK_FAMILIES.contains(&value)
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-')
}
fn valid_family_unit(family: &str, unit: &str) -> bool {
    matches!(
        (family, unit),
        ("throughput", "surfaces_per_run")
            | ("latency", "static_steps")
            | ("latency", "milliseconds_static_budget")
            | ("correctness", "fixture_verdicts")
            | ("stability", "hash_equivalence")
            | ("stability", "replay_equivalence")
            | ("adversarial", "fixture_verdicts")
            | ("rollback", "rollback_equivalence")
    )
}
fn valid_threshold(family: &str, threshold: &str) -> bool {
    match family {
        "throughput" => threshold.starts_with("min_"),
        "latency" => threshold.starts_with("max_"),
        "correctness" => threshold == "deterministic_accept_reject",
        "stability" => {
            threshold == "exact_sorted_hash_preimage"
                || threshold == "stable_bootstrap_replay_receipts"
        }
        "adversarial" => threshold == "rejects_hostile_inputs",
        "rollback" => threshold == "receipted_reversible_path",
        _ => false,
    }
}
fn valid_artifact_path(path: &str) -> bool {
    (path.starts_with("fixtures/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("goldens/p02/") && path.ends_with(".receipt"))
        || (path.starts_with("receipts/p02/") && path.ends_with(".receipt"))
        || (path.starts_with("ops/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("products/p02/") && path.ends_with(".lyra"))
        || (path.starts_with("docs/p02/") && path.ends_with(".lyra"))
}
