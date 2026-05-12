use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_benchmark_pack::deterministic_semantic_benchmark_pack_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_benchmark_pack_model::{
    SemanticBenchmarkEvidenceBinding, SemanticBenchmarkFamilyBinding, SemanticBenchmarkPackSurface,
    SemanticBenchmarkTargetBinding,
};

pub const P01_SEMANTIC_BENCHMARK_PACK_CONTRACT: &str = "LYRA-P01-SEMANTIC-BENCHMARK-PACK v1";
pub const REQUIRED_SEMANTIC_BENCHMARK_PACK_RULES: &[&str] = &[
    "semantic_benchmark_pack_must_cover_required_families",
    "semantic_throughput_targets_must_bind_commands_and_receipts",
    "semantic_latency_targets_must_use_static_budget_units",
    "semantic_correctness_targets_must_bind_fixtures_goldens_and_receipts",
    "semantic_stability_targets_must_bind_replay_and_hash_checks",
    "semantic_benchmark_evidence_must_bind_targets_artifacts_receipts",
    "no_network_dependency",
    "no_docs_only_benchmark_pack",
    "no_unreceipted_benchmark_pack",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_BENCHMARK_FAMILIES: &[&str] =
    &["throughput", "latency", "correctness", "stability"];
pub const REQUIRED_SEMANTIC_BENCHMARK_TARGETS: &[&str] = &[
    "throughput_semantic_surface_validation",
    "throughput_semantic_receipt_generation",
    "latency_semantic_canonicalization_budget",
    "latency_semantic_validation_budget",
    "correctness_semantic_valid_surface_acceptance",
    "correctness_semantic_negative_corpus_rejection",
    "stability_semantic_replay_equivalence",
    "stability_semantic_hash_ordering",
];
pub const REQUIRED_SEMANTIC_BENCHMARK_EVIDENCE: &[&str] = &[
    "throughput_semantic_evidence",
    "latency_semantic_evidence",
    "correctness_semantic_evidence",
    "stability_semantic_evidence",
];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "bounded_closed"];
const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p01-ir-check",
    "lyra-p01-semantic-core-engine-check",
    "lyra-p01-semantic-falsification-check",
    "lyra-p01-semantic-replay-check",
    "lyra-p01-semantic-closure-check",
    "lyra-p01-semantic-dependency-matrix-check",
    "lyra-p01-semantic-proof-family-check",
    "lyra-p01-semantic-benchmark-pack-check",
];
const FORBIDDEN_SEMANTIC_BENCHMARK_TEXT: &[(&str, ErrorCode)] = &[
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
    ("remote fetch", ErrorCode::ClosureNetworkDependency),
    ("docs_only:true", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("unreceipted:true", ErrorCode::ClosureUnreceipted),
    (
        "unreceipted benchmark pack allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    ("benchmark_drift:true", ErrorCode::ClosureDriftAccepted),
    ("benchmark drift accepted", ErrorCode::ClosureDriftAccepted),
    ("latency drift accepted", ErrorCode::ClosureDriftAccepted),
    ("stability drift accepted", ErrorCode::ClosureDriftAccepted),
    ("global_closure:true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_closure:true", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_semantic_benchmark_pack_surface(
    input: &str,
) -> Result<SemanticBenchmarkPackSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => {
            return Err(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "input",
                format!("canonicalization failed: {error:?}"),
            )])
        }
    };
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "semantic benchmark pack surface is empty",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_BENCHMARK_PACK_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_BENCHMARK_PACK_CONTRACT}"),
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

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
        let Some((left, value)) = line.split_once('=') else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry must contain exactly one key/value separator",
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
                    "semantic benchmark rule names must be symbolic and unique",
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
                    format!("invalid semantic benchmark family {family_id}"),
                ));
                continue;
            }
            if !seen_families.insert(family_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("family:{family_id}"),
                    "semantic benchmark family identity must be unique",
                ));
                continue;
            }
            match parse_family(line_number, family_id, value) {
                Ok(item) => families.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(target_id) = left.strip_prefix("target:") {
            if !is_symbolic_name(target_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic benchmark target {target_id}"),
                ));
                continue;
            }
            if !seen_targets.insert(target_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("target:{target_id}"),
                    "semantic benchmark target identity must be unique",
                ));
                continue;
            }
            match parse_target(line_number, target_id, value) {
                Ok(item) => targets.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(evidence_id) = left.strip_prefix("evidence:") {
            if !is_symbolic_name(evidence_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic benchmark evidence {evidence_id}"),
                ));
                continue;
            }
            if !seen_evidence.insert(evidence_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("evidence:{evidence_id}"),
                    "semantic benchmark evidence identity must be unique",
                ));
                continue;
            }
            match parse_evidence(line_number, evidence_id, value) {
                Ok(item) => evidence.push(item),
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
                format!("unknown semantic benchmark key {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticBenchmarkPackSurface {
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

pub fn validate_semantic_benchmark_pack_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_semantic_benchmark_pack_surface(input) {
        Ok(surface) => errors.extend(validate_semantic_benchmark_pack_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_semantic_benchmark_pack_model(surface: &SemanticBenchmarkPackSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic benchmark pack must bind phase P01",
        ));
    }
    if surface.task != "P01-X03" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic benchmark pack must bind task P01-X03",
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "P01-X03 must be artifact_emitted",
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
    validate_report(surface, &mut errors);
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
) -> Result<SemanticBenchmarkFamilyBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(SemanticBenchmarkFamilyBinding {
        line_number,
        id: id.to_string(),
        family_kind: required_field(
            &fields,
            "kind",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        scope: required_field(
            &fields,
            "scope",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        targets: split_list(required_field(
            &fields,
            "targets",
            line_number,
            ErrorCode::MissingClosureOutputGate,
        )?),
        proofs: split_list(required_field(
            &fields,
            "proofs",
            line_number,
            ErrorCode::MissingClosureProof,
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
) -> Result<SemanticBenchmarkTargetBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(SemanticBenchmarkTargetBinding {
        line_number,
        id: id.to_string(),
        family: required_field(
            &fields,
            "family",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        metric: required_field(
            &fields,
            "metric",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        unit: required_field(
            &fields,
            "unit",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        threshold: required_field(
            &fields,
            "threshold",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        command: required_field(
            &fields,
            "command",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        fixture: required_field(
            &fields,
            "fixture",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        golden: required_field(
            &fields,
            "golden",
            line_number,
            ErrorCode::InvalidClosureOutputGate,
        )?
        .to_string(),
        receipt: required_field(
            &fields,
            "receipt",
            line_number,
            ErrorCode::InvalidClosureProof,
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
) -> Result<SemanticBenchmarkEvidenceBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    Ok(SemanticBenchmarkEvidenceBinding {
        line_number,
        id: id.to_string(),
        family: required_field(
            &fields,
            "family",
            line_number,
            ErrorCode::InvalidClosureProof,
        )?
        .to_string(),
        targets: split_list(required_field(
            &fields,
            "targets",
            line_number,
            ErrorCode::MissingClosureOutputGate,
        )?),
        artifacts: split_list(required_field(
            &fields,
            "artifacts",
            line_number,
            ErrorCode::MissingClosureProof,
        )?),
        proof_receipts: split_list(required_field(
            &fields,
            "proof_receipts",
            line_number,
            ErrorCode::MissingClosureProof,
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

fn require_rules(surface: &SemanticBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_SEMANTIC_BENCHMARK_PACK_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                format!("unsupported semantic benchmark rule value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "missing required semantic benchmark rule",
            )),
        }
    }
}
fn require_families(surface: &SemanticBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for family in REQUIRED_SEMANTIC_BENCHMARK_FAMILIES {
        if surface.family_by_id(family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("family:{family}"),
                "required semantic benchmark family missing",
            ));
        }
    }
}
fn require_targets(surface: &SemanticBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for target in REQUIRED_SEMANTIC_BENCHMARK_TARGETS {
        if surface.target_by_id(target).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("target:{target}"),
                "required semantic benchmark target missing",
            ));
        }
    }
}
fn require_evidence(surface: &SemanticBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for evidence in REQUIRED_SEMANTIC_BENCHMARK_EVIDENCE {
        if surface.evidence_by_id(evidence).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("evidence:{evidence}"),
                "required semantic benchmark evidence missing",
            ));
        }
    }
}

fn validate_families(surface: &SemanticBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for family in &surface.families {
        if family.family_kind != family.id || !is_required_family(&family.family_kind) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                family.canonical_identity(),
                format!(
                    "invalid semantic benchmark family kind {}",
                    family.family_kind
                ),
            ));
        }
        if family.scope != "P01" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                family.canonical_identity(),
                "semantic benchmark family scope must be P01",
            ));
        }
        if family.targets.is_empty() || family.proofs.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                family.canonical_identity(),
                "semantic benchmark family must bind targets and proof labels",
            ));
        }
        if !ALLOWED_STATUSES.contains(&family.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                family.canonical_identity(),
                format!(
                    "unsupported semantic benchmark family status {}",
                    family.status
                ),
            ));
        }
        for target in &family.targets {
            if surface.target_by_id(target).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    family.canonical_identity(),
                    format!("unknown semantic benchmark target {target}"),
                ));
            }
        }
        for proof in &family.proofs {
            if !is_symbolic_name(proof) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    family.canonical_identity(),
                    format!("invalid semantic benchmark proof label {proof}"),
                ));
            }
        }
    }
}

fn validate_targets(surface: &SemanticBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for target in &surface.targets {
        if !is_required_family(&target.family) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("unknown semantic benchmark family {}", target.family),
            ));
        }
        if surface.family_by_id(&target.family).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                target.canonical_identity(),
                format!("target family not declared {}", target.family),
            ));
        }
        if !is_symbolic_name(&target.metric) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("invalid semantic benchmark metric {}", target.metric),
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
                format!("unknown semantic benchmark command {}", target.command),
            ));
        }
        if !target.fixture.starts_with("fixtures/p01/") || !target.fixture.ends_with(".lyra") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!("fixture path must be a P01 fixture: {}", target.fixture),
            ));
        }
        if !target.golden.starts_with("goldens/p01/") || !target.golden.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                target.canonical_identity(),
                format!(
                    "golden path must be a P01 receipt golden: {}",
                    target.golden
                ),
            ));
        }
        if !target.receipt.starts_with("receipts/p01/") || !target.receipt.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                target.canonical_identity(),
                format!("receipt path must be a P01 receipt: {}", target.receipt),
            ));
        }
        if !ALLOWED_STATUSES.contains(&target.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                target.canonical_identity(),
                format!(
                    "unsupported semantic benchmark target status {}",
                    target.status
                ),
            ));
        }
    }
}

fn validate_evidence(surface: &SemanticBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
    for evidence in &surface.evidence {
        if !is_required_family(&evidence.family) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                evidence.canonical_identity(),
                format!(
                    "unknown semantic benchmark evidence family {}",
                    evidence.family
                ),
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
                "semantic benchmark evidence must bind targets, artifacts, and proof receipts",
            ));
        }
        for target in &evidence.targets {
            if surface.target_by_id(target).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    evidence.canonical_identity(),
                    format!("unknown semantic benchmark evidence target {target}"),
                ));
            }
        }
        for artifact in &evidence.artifacts {
            if !valid_artifact_path(artifact) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    evidence.canonical_identity(),
                    format!("invalid semantic benchmark evidence artifact {artifact}"),
                ));
            }
        }
        for receipt in &evidence.proof_receipts {
            if !receipt.starts_with("receipts/p01/") || !receipt.ends_with(".receipt") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureProof,
                    evidence.canonical_identity(),
                    format!("invalid semantic benchmark proof receipt {receipt}"),
                ));
            }
        }
        if !ALLOWED_STATUSES.contains(&evidence.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                evidence.canonical_identity(),
                format!(
                    "unsupported semantic benchmark evidence status {}",
                    evidence.status
                ),
            ));
        }
    }
}

fn validate_family_coverage(
    surface: &SemanticBenchmarkPackSurface,
    errors: &mut Vec<ValidationError>,
) {
    for family_name in REQUIRED_SEMANTIC_BENCHMARK_FAMILIES {
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
                        ErrorCode::ClosureProofUnbound,
                        family.canonical_identity(),
                        format!("family omits declared semantic benchmark target {actual}"),
                    ));
                }
            }
            for listed in &family_targets {
                if !actual_targets.contains(listed) {
                    errors.push(ValidationError::reject(
                        ErrorCode::ClosureProofUnbound,
                        family.canonical_identity(),
                        format!(
                            "family lists target outside its semantic benchmark family {listed}"
                        ),
                    ));
                }
            }
        }
    }
}

fn validate_report(surface: &SemanticBenchmarkPackSurface, errors: &mut Vec<ValidationError>) {
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
    let report = deterministic_semantic_benchmark_pack_report(&families, &targets, &evidence);
    if report.family_count != surface.families.len()
        || report.target_count != surface.targets.len()
        || report.evidence_count != surface.evidence.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_benchmark_pack_report",
            "semantic benchmark report count mismatch",
        ));
    }
    if report.throughput_target_count == 0
        || report.latency_target_count == 0
        || report.correctness_target_count == 0
        || report.stability_target_count == 0
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingClosureOutputGate,
            "k0_semantic_benchmark_pack_report",
            "all semantic benchmark families must have target rows",
        ));
    }
    if !report.pack_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_semantic_benchmark_pack_report",
            "semantic benchmark report hash must be stable fnv1a128",
        ));
    }
}

fn parse_fields(value: &str) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        let Some((key, field_value)) = segment.split_once(':') else {
            return Err("field segment must contain a key/value separator".to_string());
        };
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
        {
            return Err("field segment sides must be non-empty and trimmed".to_string());
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
            return Err(format!("duplicate field {key}"));
        }
    }
    Ok(fields)
}
fn required_field<'a>(
    fields: &'a BTreeMap<String, String>,
    key: &str,
    line_number: usize,
    code: ErrorCode,
) -> Result<&'a str, ValidationError> {
    fields.get(key).map(String::as_str).ok_or_else(|| {
        ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("missing required field {key}"),
        )
    })
}
fn split_list(value: &str) -> Vec<String> {
    if value == "none" {
        Vec::new()
    } else {
        value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToString::to_string)
            .collect()
    }
}
fn is_required_family(value: &str) -> bool {
    REQUIRED_SEMANTIC_BENCHMARK_FAMILIES.contains(&value)
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
    (path.starts_with("fixtures/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("goldens/p01/") && path.ends_with(".receipt"))
        || (path.starts_with("receipts/p01/") && path.ends_with(".receipt"))
        || (path.starts_with("ops/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("products/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("interfaces/p01/") && path.ends_with(".lyra"))
        || (path.starts_with("docs/p01/") && path.ends_with(".lyra"))
}
fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_BENCHMARK_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic benchmark token {needle}"),
            ));
        }
    }
}
