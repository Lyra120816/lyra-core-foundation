use std::collections::{BTreeMap, BTreeSet};

use crate::k0_benchmark_pack::deterministic_benchmark_pack_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_benchmark_pack_model::{
    BenchmarkEvidenceBinding, BenchmarkFamilyBinding, BenchmarkPackSurface, BenchmarkTargetBinding,
};

pub const P00_BENCHMARK_PACK_CONTRACT: &str = "LYRA-P00-BENCHMARK-PACK v1";
pub const REQUIRED_BENCHMARK_PACK_RULES: &[&str] = &[
    "benchmark_pack_must_cover_required_families",
    "throughput_targets_must_bind_commands_and_receipts",
    "latency_targets_must_use_static_budget_units",
    "correctness_targets_must_bind_fixtures_goldens_and_receipts",
    "stability_targets_must_bind_replay_and_hash_checks",
    "no_network_dependency",
    "no_docs_only_benchmark_pack",
    "no_unreceipted_benchmark_pack",
    "no_global_closure_claim",
];
pub const REQUIRED_BENCHMARK_FAMILIES: &[&str] =
    &["throughput", "latency", "correctness", "stability"];
pub const REQUIRED_BENCHMARK_PACK_TARGETS: &[&str] = &[
    "throughput_control_validation",
    "throughput_receipt_generation",
    "latency_canonicalization_budget",
    "latency_validation_budget",
    "correctness_valid_surface_acceptance",
    "correctness_negative_corpus_rejection",
    "stability_replay_equivalence",
    "stability_hash_ordering",
];
pub const REQUIRED_BENCHMARK_EVIDENCE: &[&str] = &[
    "throughput_evidence",
    "latency_evidence",
    "correctness_evidence",
    "stability_evidence",
];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "bounded_closed"];
const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p00-validate",
    "lyra-p00-control-check",
    "lyra-p00-proof-family-check",
    "lyra-p00-benchmark-pack-check",
    "lyra-p00-falsification-check",
    "lyra-p00-replay-check",
    "lyra-p00-closure-check",
];
const FORBIDDEN_BENCHMARK_PACK_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::ClosureNetworkDependency),
    ("rule:network_required", ErrorCode::ClosureNetworkDependency),
    ("cloud required", ErrorCode::ClosureNetworkDependency),
    ("online required", ErrorCode::ClosureNetworkDependency),
    (
        "remote service required",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("remote fetch", ErrorCode::ClosureNetworkDependency),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("rule:docs_only_benchmark_pack", ErrorCode::ClosureDocsOnly),
    (
        "benchmark pack without receipt",
        ErrorCode::ClosureUnreceipted,
    ),
    (
        "unreceipted benchmark pack allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    (
        "rule:unreceipted_benchmark_pack_allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    ("benchmark drift accepted", ErrorCode::ClosureDriftAccepted),
    ("latency drift accepted", ErrorCode::ClosureDriftAccepted),
    ("stability drift accepted", ErrorCode::ClosureDriftAccepted),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("rule:global_complete", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_benchmark_pack_surface(
    input: &str,
) -> Result<BenchmarkPackSurface, Vec<ValidationError>> {
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
            "no benchmark pack lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_BENCHMARK_PACK_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_BENCHMARK_PACK_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut families = Vec::new();
    let mut targets = Vec::new();
    let mut evidence = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_families = BTreeSet::new();
    let mut seen_targets = BTreeSet::new();
    let mut seen_evidence = BTreeSet::new();

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
                    "benchmark pack rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(family_id) = left.strip_prefix("family:") {
            if !is_required_family(family_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid benchmark family {family_id}"),
                ));
                continue;
            }
            if !seen_families.insert(family_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("family:{family_id}"),
                    "benchmark family identity must be unique",
                ));
                continue;
            }
            match parse_family(line_number, family_id, value) {
                Ok(binding) => families.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(target_id) = left.strip_prefix("target:") {
            if !is_symbolic_name(target_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid benchmark target {target_id}"),
                ));
                continue;
            }
            if !seen_targets.insert(target_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("target:{target_id}"),
                    "benchmark target identity must be unique",
                ));
                continue;
            }
            match parse_target(line_number, target_id, value) {
                Ok(binding) => targets.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(evidence_id) = left.strip_prefix("evidence:") {
            if !is_symbolic_name(evidence_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid benchmark evidence {evidence_id}"),
                ));
                continue;
            }
            if !seen_evidence.insert(evidence_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("evidence:{evidence_id}"),
                    "benchmark evidence identity must be unique",
                ));
                continue;
            }
            match parse_evidence(line_number, evidence_id, value) {
                Ok(binding) => evidence.push(binding),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if !seen_scalars.insert(left.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate scalar {left}"),
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
                format!("unknown benchmark pack key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(BenchmarkPackSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        families,
        targets,
        evidence,
    })
}

pub fn validate_benchmark_pack_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_benchmark_pack_surface(input) {
        Ok(surface) => errors.extend(validate_benchmark_pack_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_receipt(input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_benchmark_pack_model(surface: &BenchmarkPackSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "benchmark pack must bind to P00",
        ));
    }
    if surface.task != "P00-X03" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "benchmark pack must bind to P00-X03",
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported benchmark pack status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_families(surface, &mut errors);
    require_targets(surface, &mut errors);
    require_evidence(surface, &mut errors);
    validate_families(surface, &mut errors);
    validate_targets(surface, &mut errors);
    validate_evidence(surface, &mut errors);
    validate_family_coverage(surface, &mut errors);
    validate_benchmark_pack_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_family(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BenchmarkFamilyBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "benchmark family fields must be key:value segments",
        )
    })?;
    Ok(BenchmarkFamilyBinding {
        line_number,
        id: id.to_string(),
        family_kind: required_field(
            &fields,
            "kind",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        targets: split_csv(&required_field(
            &fields,
            "targets",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        proofs: split_csv(&required_field(
            &fields,
            "proofs",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
    })
}

fn parse_target(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BenchmarkTargetBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "benchmark target fields must be key:value segments",
        )
    })?;
    Ok(BenchmarkTargetBinding {
        line_number,
        id: id.to_string(),
        family: required_field(
            &fields,
            "family",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        metric: required_field(
            &fields,
            "metric",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        unit: required_field(
            &fields,
            "unit",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        threshold: required_field(
            &fields,
            "threshold",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        command: required_field(
            &fields,
            "command",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        fixture: required_field(
            &fields,
            "fixture",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        golden: required_field(
            &fields,
            "golden",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        receipt: required_field(
            &fields,
            "receipt",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureOutputGate,
            line_number,
        )?,
    })
}

fn parse_evidence(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<BenchmarkEvidenceBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "benchmark evidence fields must be key:value segments",
        )
    })?;
    Ok(BenchmarkEvidenceBinding {
        line_number,
        id: id.to_string(),
        family: required_field(
            &fields,
            "family",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
        targets: split_csv(&required_field(
            &fields,
            "targets",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        artifacts: split_csv(&required_field(
            &fields,
            "artifacts",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        proof_receipts: split_csv(&required_field(
            &fields,
            "proof_receipts",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
    })
}

fn require_rules(surface: &BenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_BENCHMARK_PACK_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "required benchmark pack rule missing",
            )),
        }
    }
}

fn require_families(surface: &BenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for family in REQUIRED_BENCHMARK_FAMILIES {
        if surface.family_by_id(family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("family:{family}"),
                "required benchmark family missing",
            ));
        }
    }
}

fn require_targets(surface: &BenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for target in REQUIRED_BENCHMARK_PACK_TARGETS {
        if surface.target_by_id(target).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("target:{target}"),
                "required benchmark target missing",
            ));
        }
    }
}

fn require_evidence(surface: &BenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for evidence in REQUIRED_BENCHMARK_EVIDENCE {
        if surface.evidence_by_id(evidence).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("evidence:{evidence}"),
                "required benchmark evidence missing",
            ));
        }
    }
}

fn validate_families(surface: &BenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for family in &surface.families {
        if family.family_kind != family.id || !is_required_family(&family.family_kind) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                family.canonical_identity(),
                format!("invalid benchmark family kind {}", family.family_kind),
            ));
        }
        if family.scope != "P00" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                family.canonical_identity(),
                "benchmark family scope must be P00",
            ));
        }
        if !ALLOWED_STATUSES.contains(&family.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                family.canonical_identity(),
                format!("invalid benchmark family status {}", family.status),
            ));
        }
        if family.targets.is_empty() || family.proofs.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                family.canonical_identity(),
                "benchmark family must bind targets and proof labels",
            ));
        }
        for target in &family.targets {
            if surface.target_by_id(target).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    family.canonical_identity(),
                    format!("unknown benchmark target {target}"),
                ));
            }
        }
        for proof in &family.proofs {
            if !is_symbolic_name(proof) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    family.canonical_identity(),
                    format!("invalid benchmark proof label {proof}"),
                ));
            }
        }
    }
}

fn validate_targets(surface: &BenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for target in &surface.targets {
        if !is_required_family(&target.family) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("unknown benchmark target family {}", target.family),
            ));
        }
        if surface.family_by_id(&target.family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                target.canonical_identity(),
                format!("target family not declared {}", target.family),
            ));
        }
        if target.metric.is_empty() || !is_symbolic_name(&target.metric) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("invalid benchmark metric {}", target.metric),
            ));
        }
        if !valid_family_unit(&target.family, &target.unit) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("invalid unit {} for family {}", target.unit, target.family),
            ));
        }
        if !valid_threshold(&target.family, &target.threshold) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!(
                    "invalid threshold {} for family {}",
                    target.threshold, target.family
                ),
            ));
        }
        if !REQUIRED_COMMANDS.contains(&target.command.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("unknown benchmark command {}", target.command),
            ));
        }
        if !target.fixture.starts_with("fixtures/p00/") || !target.fixture.ends_with(".lyra") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("fixture path must be a P00 fixture: {}", target.fixture),
            ));
        }
        if !target.golden.starts_with("goldens/p00/") || !target.golden.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!(
                    "golden path must be a P00 receipt golden: {}",
                    target.golden
                ),
            ));
        }
        if !target.receipt.starts_with("receipts/p00/") || !target.receipt.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                target.canonical_identity(),
                format!("receipt path must be a P00 receipt: {}", target.receipt),
            ));
        }
        if !ALLOWED_STATUSES.contains(&target.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("invalid benchmark target status {}", target.status),
            ));
        }
    }
}

fn validate_evidence(surface: &BenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for evidence in &surface.evidence {
        if !is_required_family(&evidence.family) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                evidence.canonical_identity(),
                format!("unknown benchmark evidence family {}", evidence.family),
            ));
        }
        if surface.family_by_id(&evidence.family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                evidence.canonical_identity(),
                format!("evidence family not declared {}", evidence.family),
            ));
        }
        if evidence.targets.is_empty()
            || evidence.artifacts.is_empty()
            || evidence.proof_receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                evidence.canonical_identity(),
                "benchmark evidence must bind targets, artifacts, and proof receipts",
            ));
        }
        for target in &evidence.targets {
            if surface.target_by_id(target).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    evidence.canonical_identity(),
                    format!("unknown benchmark evidence target {target}"),
                ));
            }
        }
        for artifact in &evidence.artifacts {
            if !valid_artifact_path(artifact) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    evidence.canonical_identity(),
                    format!("invalid benchmark evidence artifact {artifact}"),
                ));
            }
        }
        for receipt in &evidence.proof_receipts {
            if !receipt.starts_with("receipts/p00/") || !receipt.ends_with(".receipt") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    evidence.canonical_identity(),
                    format!("invalid benchmark proof receipt {receipt}"),
                ));
            }
        }
        if !ALLOWED_STATUSES.contains(&evidence.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                evidence.canonical_identity(),
                format!("invalid benchmark evidence status {}", evidence.status),
            ));
        }
    }
}

fn validate_family_coverage(surface: &BenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for family_name in REQUIRED_BENCHMARK_FAMILIES {
        if let Some(family) = surface.family_by_id(family_name) {
            let mut family_targets = BTreeSet::new();
            for target in &family.targets {
                family_targets.insert(target.as_str());
            }
            let mut actual_targets = BTreeSet::new();
            for target in &surface.targets {
                if target.family == *family_name {
                    actual_targets.insert(target.id.as_str());
                }
            }
            for actual in &actual_targets {
                if !family_targets.contains(actual) {
                    errors.push(ValidationError::reject(
                        ErrorCode::ClosureProofUnbound,
                        family.canonical_identity(),
                        format!("family omits declared target {actual}"),
                    ));
                }
            }
            for listed in &family_targets {
                if !actual_targets.contains(listed) {
                    errors.push(ValidationError::reject(
                        ErrorCode::ClosureProofUnbound,
                        family.canonical_identity(),
                        format!("family lists target outside its family {listed}"),
                    ));
                }
            }
        }
    }
}

fn validate_benchmark_pack_report(
    surface: &BenchmarkPackSurface,
    errors: &mut Vec<ValidationError>,
) {
    let family_inputs: Vec<(String, String, Vec<String>, Vec<String>, String)> = surface
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
        .collect();
    let target_inputs: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
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
                target.receipt.clone(),
                target.status.clone(),
            )
        })
        .collect();
    let evidence_inputs: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .evidence
        .iter()
        .map(|evidence| {
            (
                evidence.id.clone(),
                evidence.family.clone(),
                evidence.targets.clone(),
                evidence.artifacts.clone(),
                evidence.proof_receipts.clone(),
                evidence.status.clone(),
            )
        })
        .collect();
    let report =
        deterministic_benchmark_pack_report(&family_inputs, &target_inputs, &evidence_inputs);
    if report.family_count != surface.families.len()
        || report.target_count != surface.targets.len()
        || report.evidence_count != surface.evidence.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_benchmark_pack_report",
            "benchmark pack report count mismatch",
        ));
    }
    if report.throughput_target_count == 0
        || report.latency_target_count == 0
        || report.correctness_target_count == 0
        || report.stability_target_count == 0
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "k0_benchmark_pack_report",
            "all benchmark families must have target rows",
        ));
    }
    if !report.pack_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_benchmark_pack_report",
            "benchmark pack report hash must be stable fnv1a128",
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

fn is_required_family(value: &str) -> bool {
    REQUIRED_BENCHMARK_FAMILIES.contains(&value)
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn valid_family_unit(family: &str, unit: &str) -> bool {
    match family {
        "throughput" => unit == "surfaces_per_run" || unit == "receipts_per_run",
        "latency" => unit == "static_steps" || unit == "milliseconds_static_budget",
        "correctness" => unit == "fixture_verdicts",
        "stability" => unit == "replay_equivalence" || unit == "hash_equivalence",
        _ => false,
    }
}

fn valid_threshold(family: &str, threshold: &str) -> bool {
    match family {
        "throughput" => threshold.starts_with("min_") && threshold.len() > 4,
        "latency" => threshold.starts_with("max_") && threshold.len() > 4,
        "correctness" => threshold.starts_with("accepts_") || threshold.starts_with("rejects_"),
        "stability" => threshold.starts_with("stable_") || threshold.starts_with("exact_"),
        _ => false,
    }
}

fn valid_artifact_path(path: &str) -> bool {
    (path.starts_with("fixtures/p00/") && path.ends_with(".lyra"))
        || (path.starts_with("goldens/p00/") && path.ends_with(".receipt"))
        || (path.starts_with("receipts/p00/") && path.ends_with(".receipt"))
        || (path.starts_with("ops/p00/") && path.ends_with(".lyra"))
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_BENCHMARK_PACK_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden benchmark pack token {needle}"),
            ));
        }
    }
}
