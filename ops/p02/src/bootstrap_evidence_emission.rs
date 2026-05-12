use crate::k0_bootstrap_evidence_emission::deterministic_bootstrap_evidence_emission_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_evidence_emission_model::{
    BootstrapChallengeReceiptBinding, BootstrapEvidenceEmissionReceiptBinding,
    BootstrapEvidenceEmissionSurface, BootstrapEvidenceFixtureBinding,
    BootstrapTargetMatrixReportBinding,
};
use std::collections::{BTreeMap, BTreeSet};

pub const P02_BOOTSTRAP_EVIDENCE_EMISSION_CONTRACT: &str =
    "LYRA-P02-BOOTSTRAP-EVIDENCE-EMISSION v1";
pub const REQUIRED_BOOTSTRAP_EVIDENCE_RULES: &[&str] = &[
    "extinction_ledger_fixtures_must_include_positive_negative_malformed",
    "fixtures_must_bind_source_receipts",
    "target_matrix_reports_must_cover_all_declared_targets",
    "target_matrix_reports_must_bind_required_proof_families",
    "bootstrap_challenge_receipts_must_cover_host_boundary_suites",
    "challenge_receipts_cannot_promote_truth_without_replay",
    "emitted_receipts_must_bind_p02_002_p02_005_p02_006_p02_009_p02_010",
    "fixture_expected_verdict_must_be_declared",
    "reports_remain_pending_until_local_execution",
    "no_network_required_evidence_emission",
    "no_probabilistic_evidence_truth",
    "no_hidden_randomness_evidence",
    "no_ambient_time_evidence",
    "no_placeholder_evidence",
    "no_global_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_EVIDENCE_FIXTURES: &[&str] = &[
    "fixture_extinction_positive",
    "fixture_extinction_negative_missing_rule",
    "fixture_extinction_negative_duplicate_entry",
    "fixture_target_matrix_positive",
    "fixture_target_matrix_negative_missing_target",
    "fixture_target_matrix_negative_bad_family",
    "fixture_challenge_positive",
    "fixture_challenge_negative_unreceipted",
    "fixture_challenge_negative_foreign_ownership",
    "fixture_replacement_positive",
    "fixture_replacement_negative_missing_handoff",
    "fixture_emission_negative_malformed",
];
pub const REQUIRED_BOOTSTRAP_EVIDENCE_TARGETS: &[&str] = &[
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
pub const REQUIRED_BOOTSTRAP_EVIDENCE_TARGET_CLASSES: &[&str] =
    &["linux", "windows", "mobile", "wasm", "baremetal", "other"];
pub const REQUIRED_BOOTSTRAP_EVIDENCE_PROOF_FAMILIES: &[&str] = &[
    "canonical_io",
    "deterministic_replay",
    "host_boundary",
    "receipt_chain",
    "rollback_lane",
];
pub const REQUIRED_BOOTSTRAP_EVIDENCE_CHALLENGE_SUITES: &[&str] = &[
    "suite_no_ambient_network_import",
    "suite_no_ambient_time_truth",
    "suite_no_hidden_randomness_truth",
    "suite_no_unledgered_host_surface",
    "suite_no_foreign_semantic_ownership",
    "suite_operator_truth_containment",
    "suite_foreign_runtime_quarantine",
];
pub const REQUIRED_BOOTSTRAP_EVIDENCE_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_extinction_ledger",
    "receipt_host_boundary_challenge_suites",
    "receipt_bootstrap_target_matrix",
    "receipt_seed_runtime_replacement_milestones",
    "receipt_bootstrap_evidence_emission",
];
const ALLOWED_FIXTURE_KINDS: &[&str] = &[
    "extinction_ledger_positive",
    "extinction_ledger_negative",
    "target_matrix_positive",
    "target_matrix_negative",
    "bootstrap_challenge_positive",
    "bootstrap_challenge_negative",
    "seed_replacement_positive",
    "seed_replacement_negative",
    "evidence_emission_malformed",
];
const ALLOWED_EXPECTED_VERDICTS: &[&str] = &["accepted", "rejected"];
const ALLOWED_TARGET_REPORT_STATUS: &[&str] = &["pending_local_validation_report_emitted"];
const ALLOWED_CHALLENGE_STATUS: &[&str] = &["challenge_receipt_emitted"];
const ALLOWED_RECEIPT_HASH_STATE: &[&str] = &["stable_hash_bound"];
const ALLOWED_CHALLENGE_KINDS: &[&str] = &["host_boundary"];
const ALLOWED_TRUTH_EFFECTS: &[&str] = &["none_without_local_replay"];
const ALLOWED_SOURCE_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_extinction_ledger",
    "receipt_host_boundary_challenge_suites",
    "receipt_bootstrap_target_matrix",
    "receipt_seed_runtime_replacement_milestones",
    "receipt_bootstrap_evidence_emission",
];
const ALLOWED_SURFACE_REFS: &[&str] = &[
    "surface:git_repository_transport",
    "surface:external_wall_clock",
    "surface:host_operating_system",
    "surface:host_filesystem",
    "surface:rust_bootstrap_compiler",
    "surface:operator_shell_terminal",
    "surface:rust_std_runtime",
];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    (
        "probabilistic evidence truth",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_bootstrap_evidence_emission_surface(
    input: &str,
) -> Result<BootstrapEvidenceEmissionSurface, Vec<ValidationError>> {
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
            "empty bootstrap evidence emission surface",
        )]);
    }
    if lines[0] != P02_BOOTSTRAP_EVIDENCE_EMISSION_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_EVIDENCE_EMISSION_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut extinction_ledger_receipt = None;
    let mut target_matrix_receipt = None;
    let mut host_boundary_receipt = None;
    let mut replacement_milestones_receipt = None;
    let mut rules = BTreeMap::new();
    let mut fixtures = Vec::new();
    let mut target_reports = Vec::new();
    let mut challenge_receipts = Vec::new();
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
        if let Some(id) = left.strip_prefix("fixture:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateAcceptanceGolden,
                    left,
                    "duplicate fixture",
                ));
            } else {
                match parse_fixture(n, id, value) {
                    Ok(x) => fixtures.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("target_report:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeploymentTarget,
                    left,
                    "duplicate target report",
                ));
            } else {
                match parse_target_report(n, id, value) {
                    Ok(x) => target_reports.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("challenge_receipt:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    left,
                    "duplicate challenge receipt",
                ));
            } else {
                match parse_challenge_receipt(n, id, value) {
                    Ok(x) => challenge_receipts.push(x),
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
            "extinction_ledger_receipt" => extinction_ledger_receipt = Some(value.to_string()),
            "target_matrix_receipt" => target_matrix_receipt = Some(value.to_string()),
            "host_boundary_receipt" => host_boundary_receipt = Some(value.to_string()),
            "replacement_milestones_receipt" => {
                replacement_milestones_receipt = Some(value.to_string())
            }
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
    Ok(BootstrapEvidenceEmissionSurface {
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
        extinction_ledger_receipt: extinction_ledger_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "extinction_ledger_receipt",
                "missing extinction ledger receipt",
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
        replacement_milestones_receipt: replacement_milestones_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "replacement_milestones_receipt",
                "missing replacement milestones receipt",
            )]
        })?,
        rules,
        fixtures,
        target_reports,
        challenge_receipts,
        receipts,
    })
}

pub fn validate_bootstrap_evidence_emission_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut forbidden = Vec::new();
    scan_forbidden(input, &mut forbidden);
    let verdict = match parse_bootstrap_evidence_emission_surface(input) {
        Ok(surface) => {
            let mut v = validate_bootstrap_evidence_emission_model(&surface);
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

pub fn validate_bootstrap_evidence_emission_model(
    surface: &BootstrapEvidenceEmissionSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-010" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-010 got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported {}", surface.status),
        ));
    }

    if surface.extinction_ledger_receipt
        != "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt"
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "extinction_ledger_receipt",
            "must bind P02-002 receipt",
        ));
    }
    if surface.host_boundary_receipt
        != "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt"
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "host_boundary_receipt",
            "must bind P02-005 receipt",
        ));
    }
    if surface.target_matrix_receipt != "receipts/p02/pass_0064_bootstrap_target_matrix.receipt" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "target_matrix_receipt",
            "must bind P02-006 receipt",
        ));
    }
    if surface.replacement_milestones_receipt
        != "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt"
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "replacement_milestones_receipt",
            "must bind P02-009 receipt",
        ));
    }

    for rule in REQUIRED_BOOTSTRAP_EVIDENCE_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") => {}
            Some(v) => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("rule:{rule}"),
                format!("bad rule value {v}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "missing rule",
            )),
        }
    }
    for fixture in REQUIRED_BOOTSTRAP_EVIDENCE_FIXTURES {
        if surface.fixture_by_id(fixture).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingAcceptanceGolden,
                format!("fixture:{fixture}"),
                "missing evidence fixture",
            ));
        }
    }
    for target in REQUIRED_BOOTSTRAP_EVIDENCE_TARGETS {
        if surface.target_report_by_target(target).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target_report:{target}"),
                "missing target matrix report",
            ));
        }
    }
    for suite in REQUIRED_BOOTSTRAP_EVIDENCE_CHALLENGE_SUITES {
        if surface.challenge_receipt_by_suite(suite).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeFixture,
                format!("challenge_receipt:{suite}"),
                "missing challenge receipt",
            ));
        }
    }
    for receipt in REQUIRED_BOOTSTRAP_EVIDENCE_RECEIPTS {
        if surface.receipt_by_id(receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{receipt}"),
                "missing receipt binding",
            ));
        }
    }

    let mut accepted = 0usize;
    let mut rejected = 0usize;
    for fixture in &surface.fixtures {
        if !ALLOWED_FIXTURE_KINDS.contains(&fixture.fixture_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidAcceptanceGolden,
                fixture.canonical_identity(),
                format!("bad fixture kind {}", fixture.fixture_kind),
            ));
        }
        if !ALLOWED_EXPECTED_VERDICTS.contains(&fixture.expected_verdict.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidAcceptanceGolden,
                fixture.canonical_identity(),
                format!("bad expected verdict {}", fixture.expected_verdict),
            ));
        }
        if !ALLOWED_SOURCE_RECEIPTS.contains(&fixture.source_receipt.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                fixture.canonical_identity(),
                format!("unbound source receipt {}", fixture.source_receipt),
            ));
        }
        if !fixture.path.starts_with("fixtures/p02/") {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                fixture.canonical_identity(),
                "fixture path must live in fixtures/p02",
            ));
        }
        if fixture.status != "emitted" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                fixture.canonical_identity(),
                format!("bad fixture status {}", fixture.status),
            ));
        }
        if fixture.expects_acceptance() {
            accepted += 1;
        }
        if fixture.expects_rejection() {
            rejected += 1;
        }
    }
    if accepted < 3 {
        errors.push(ValidationError::reject(
            ErrorCode::MissingAcceptanceGolden,
            "fixtures",
            "must include positive accepted fixtures",
        ));
    }
    if rejected < 6 {
        errors.push(ValidationError::reject(
            ErrorCode::MissingNegativeCase,
            "fixtures",
            "must include negative rejected fixtures",
        ));
    }

    let mut classes = BTreeSet::new();
    for report in &surface.target_reports {
        if !REQUIRED_BOOTSTRAP_EVIDENCE_TARGETS.contains(&report.target_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                report.canonical_identity(),
                format!("unknown target {}", report.target_id),
            ));
        }
        if !REQUIRED_BOOTSTRAP_EVIDENCE_TARGET_CLASSES.contains(&report.target_class.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                report.canonical_identity(),
                format!("bad class {}", report.target_class),
            ));
        }
        classes.insert(report.target_class.as_str());
        if report.proof_count != REQUIRED_BOOTSTRAP_EVIDENCE_PROOF_FAMILIES.len() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                report.canonical_identity(),
                format!("bad proof count {}", report.proof_count),
            ));
        }
        for family in REQUIRED_BOOTSTRAP_EVIDENCE_PROOF_FAMILIES {
            if !report.required_families.iter().any(|x| x == family) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingProofBinding,
                    report.canonical_identity(),
                    format!("missing proof family {family}"),
                ));
            }
        }
        if report.matrix_receipt != "receipt_bootstrap_target_matrix" {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                report.canonical_identity(),
                "target report must bind target matrix receipt",
            ));
        }
        if !ALLOWED_TARGET_REPORT_STATUS.contains(&report.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                report.canonical_identity(),
                format!("bad target report status {}", report.status),
            ));
        }
    }
    for class in REQUIRED_BOOTSTRAP_EVIDENCE_TARGET_CLASSES {
        if !classes.contains(class) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target_class:{class}"),
                "missing target class report",
            ));
        }
    }

    for challenge in &surface.challenge_receipts {
        if !REQUIRED_BOOTSTRAP_EVIDENCE_CHALLENGE_SUITES.contains(&challenge.suite_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                challenge.canonical_identity(),
                format!("unknown suite {}", challenge.suite_id),
            ));
        }
        if !ALLOWED_SURFACE_REFS.contains(&challenge.surface_ref.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                challenge.canonical_identity(),
                format!("unknown surface {}", challenge.surface_ref),
            ));
        }
        if !challenge
            .receipt_path
            .starts_with("receipts/p02/bootstrap_challenges/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                challenge.canonical_identity(),
                "challenge receipt path must live under receipts/p02/bootstrap_challenges",
            ));
        }
        if !ALLOWED_RECEIPT_HASH_STATE.contains(&challenge.receipt_hash_state.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReplayReceipt,
                challenge.canonical_identity(),
                format!("bad receipt hash state {}", challenge.receipt_hash_state),
            ));
        }
        if !ALLOWED_CHALLENGE_KINDS.contains(&challenge.challenge_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidChallengeFixture,
                challenge.canonical_identity(),
                format!("bad challenge kind {}", challenge.challenge_kind),
            ));
        }
        if !ALLOWED_TRUTH_EFFECTS.contains(&challenge.truth_effect.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::AmbientAuthority,
                challenge.canonical_identity(),
                format!("bad truth effect {}", challenge.truth_effect),
            ));
        }
        if !ALLOWED_CHALLENGE_STATUS.contains(&challenge.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                challenge.canonical_identity(),
                format!("bad challenge status {}", challenge.status),
            ));
        }
    }

    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p02/")
            && !receipt.path.starts_with("interfaces/p02/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                receipt.canonical_identity(),
                "receipt path must be receipted or contract path",
            ));
        }
        if receipt.status != "artifact_emitted" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!("bad receipt status {}", receipt.status),
            ));
        }
    }

    if errors.is_empty() {
        let fixtures = surface
            .fixtures
            .iter()
            .map(|x| {
                (
                    x.id.clone(),
                    x.fixture_kind.clone(),
                    x.path.clone(),
                    x.binds_task.clone(),
                    x.source_receipt.clone(),
                    x.expected_verdict.clone(),
                    x.status.clone(),
                )
            })
            .collect::<Vec<_>>();
        let target_reports = surface
            .target_reports
            .iter()
            .map(|x| {
                (
                    x.id.clone(),
                    x.target_id.clone(),
                    x.target_class.clone(),
                    x.proof_count,
                    x.required_families.clone(),
                    x.matrix_receipt.clone(),
                    x.status.clone(),
                )
            })
            .collect::<Vec<_>>();
        let challenge_receipts = surface
            .challenge_receipts
            .iter()
            .map(|x| {
                (
                    x.id.clone(),
                    x.suite_id.clone(),
                    x.surface_ref.clone(),
                    x.receipt_path.clone(),
                    x.receipt_hash_state.clone(),
                    x.challenge_kind.clone(),
                    x.truth_effect.clone(),
                    x.status.clone(),
                )
            })
            .collect::<Vec<_>>();
        let receipts = surface
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
            .collect::<Vec<_>>();
        let _report = deterministic_bootstrap_evidence_emission_report(
            &fixtures,
            &target_reports,
            &challenge_receipts,
            &receipts,
        );
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_fixture(
    n: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapEvidenceFixtureBinding, ValidationError> {
    let fields = fields(value, n)?;
    Ok(BootstrapEvidenceFixtureBinding {
        line_number: n,
        id: id.to_string(),
        fixture_kind: req(&fields, "fixture_kind", n)?,
        path: req(&fields, "path", n)?,
        binds_task: req(&fields, "binds_task", n)?,
        source_receipt: req(&fields, "source_receipt", n)?,
        expected_verdict: req(&fields, "expected_verdict", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn parse_target_report(
    n: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapTargetMatrixReportBinding, ValidationError> {
    let fields = fields(value, n)?;
    let proof_count = req(&fields, "proof_count", n)?
        .parse::<usize>()
        .map_err(|_| {
            ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                format!("line:{n:03}"),
                "bad proof_count",
            )
        })?;
    Ok(BootstrapTargetMatrixReportBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&fields, "target_id", n)?,
        target_class: req(&fields, "target_class", n)?,
        proof_count,
        required_families: csv(&req(&fields, "required_families", n)?),
        matrix_receipt: req(&fields, "matrix_receipt", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn parse_challenge_receipt(
    n: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapChallengeReceiptBinding, ValidationError> {
    let fields = fields(value, n)?;
    Ok(BootstrapChallengeReceiptBinding {
        line_number: n,
        id: id.to_string(),
        suite_id: req(&fields, "suite_id", n)?,
        surface_ref: req(&fields, "surface_ref", n)?,
        receipt_path: req(&fields, "receipt_path", n)?,
        receipt_hash_state: req(&fields, "receipt_hash_state", n)?,
        challenge_kind: req(&fields, "challenge_kind", n)?,
        truth_effect: req(&fields, "truth_effect", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn parse_receipt(
    n: usize,
    id: &str,
    value: &str,
) -> Result<BootstrapEvidenceEmissionReceiptBinding, ValidationError> {
    let fields = fields(value, n)?;
    Ok(BootstrapEvidenceEmissionReceiptBinding {
        line_number: n,
        id: id.to_string(),
        path: req(&fields, "path", n)?,
        target: req(&fields, "target", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn fields(value: &str, n: usize) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut output = BTreeMap::new();
    for segment in value.split('|') {
        let Some((key, val)) = segment.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "bad field",
            ));
        };
        if key.is_empty()
            || val.is_empty()
            || output.insert(key.to_string(), val.to_string()).is_some()
        {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "bad field",
            ));
        }
    }
    Ok(output)
}

fn req(fields: &BTreeMap<String, String>, key: &str, n: usize) -> Result<String, ValidationError> {
    fields.get(key).cloned().ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{n:03}"),
            format!("missing {key}"),
        )
    })
}

fn csv(value: &str) -> Vec<String> {
    value
        .split(',')
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
                format!("forbidden evidence-emission phrase {needle}"),
            ));
        }
    }
}
