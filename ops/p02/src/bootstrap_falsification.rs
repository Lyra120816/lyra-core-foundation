use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_falsification::deterministic_bootstrap_falsification_suite_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_falsification::{
    bootstrap_falsification_artifact_descriptor, bootstrap_falsification_artifact_digest,
    bootstrap_falsification_artifact_ids, bootstrap_falsification_artifacts_bind_paths,
    bootstrap_falsification_case_descriptor, bootstrap_falsification_case_digest,
    bootstrap_falsification_case_ids, bootstrap_falsification_harness_descriptor,
    bootstrap_falsification_harness_digest, bootstrap_falsification_harness_ids,
    bootstrap_falsification_harnesses_bind_known_cases,
    bootstrap_falsification_no_forbidden_descriptor_claims,
    bootstrap_falsification_proof_descriptor, bootstrap_falsification_proof_digest,
    bootstrap_falsification_proof_ids, bootstrap_falsification_proofs_bind_registry,
    bootstrap_falsification_registry_hash, bootstrap_falsification_targets_all_required_domains,
    bootstrap_rejection_assertion_descriptor, bootstrap_rejection_assertion_digest,
    bootstrap_rejection_assertion_ids, bootstrap_rejection_assertions_bind_known_cases,
    LYRA_P02_BOOTSTRAP_FALSIFICATION_CARRIER,
};
use crate::p02_bootstrap_falsification_model::{
    BootstrapFalsificationArtifactBinding, BootstrapFalsificationCaseBinding,
    BootstrapFalsificationHarnessBinding, BootstrapFalsificationProofBinding,
    BootstrapFalsificationSurface, BootstrapRejectionAssertionBinding,
};

pub const P02_BOOTSTRAP_FALSIFICATION_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-FALSIFICATION-CORPUS v1";

pub const REQUIRED_BOOTSTRAP_FALSIFICATION_RULES: &[&str] = &[
    "bootstrap_negative_corpus_required",
    "bootstrap_trust_rejection_required",
    "seed_runtime_rejection_required",
    "host_extinction_rejection_required",
    "foreign_boundary_rejection_required",
    "operator_handoff_rejection_required",
    "fallback_receipt_rejection_required",
    "falsification_harness_required",
    "expected_error_exact_match_required",
    "rejected_receipt_required",
    "replay_witness_required",
    "no_accepting_negative",
    "no_manual_only_corpus",
    "no_network_falsification",
    "no_probabilistic_acceptance",
    "no_ambient_time_acceptance",
    "no_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_FALSIFICATION_CASES: &[&str] = &[
    "bootstrap_authority_missing_master_case",
    "bootstrap_authority_ambient_override_case",
    "seed_runtime_network_dependency_case",
    "seed_runtime_probabilistic_seed_case",
    "host_extinction_unledgered_surface_case",
    "host_extinction_delete_gate_bypass_case",
    "foreign_boundary_hidden_surface_case",
    "operator_handoff_truth_drift_case",
    "emergency_fallback_ambient_time_case",
    "receipt_commit_hash_mismatch_case",
];
pub const REQUIRED_BOOTSTRAP_FALSIFICATION_HARNESSES: &[&str] = &[
    "bootstrap_negative_corpus_parser",
    "bootstrap_trust_falsifier",
    "seed_runtime_law_falsifier",
    "host_extinction_falsifier",
    "handoff_fallback_receipt_falsifier",
    "cross_bootstrap_replay_checker",
];
pub const REQUIRED_BOOTSTRAP_REJECTION_ASSERTIONS: &[&str] = &[
    "bootstrap_authority_missing_master_rejection",
    "bootstrap_authority_ambient_override_rejection",
    "seed_runtime_network_dependency_rejection",
    "seed_runtime_probabilistic_seed_rejection",
    "host_extinction_unledgered_surface_rejection",
    "host_extinction_delete_gate_bypass_rejection",
    "foreign_boundary_hidden_surface_rejection",
    "operator_handoff_truth_drift_rejection",
    "emergency_fallback_ambient_time_rejection",
    "receipt_commit_hash_mismatch_rejection",
];
pub const REQUIRED_BOOTSTRAP_FALSIFICATION_ARTIFACTS: &[&str] = &[
    "bootstrap_falsification_contract",
    "bootstrap_falsification_law",
    "bootstrap_falsification_operator",
    "valid_bootstrap_falsification_fixture",
    "golden_bootstrap_falsification_receipt",
    "execution_bootstrap_falsification_receipt",
    "deterministic_bootstrap_falsification_report",
    "bootstrap_falsification_case_pack",
];
pub const REQUIRED_BOOTSTRAP_FALSIFICATION_PROOFS: &[&str] = &[
    "bootstrap_trust_negative_proof",
    "seed_runtime_law_negative_proof",
    "host_extinction_negative_proof",
    "handoff_fallback_receipt_negative_proof",
    "p02_bootstrap_falsification_parity_proof",
];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const ALLOWED_TARGET_DOMAINS: &[&str] = &[
    "bootstrap_trust",
    "seed_runtime_law",
    "host_extinction",
    "foreign_boundary",
    "operator_handoff",
    "emergency_fallback",
    "receipt_commit",
];
const ALLOWED_TARGET_VALIDATORS: &[&str] = &[
    "bootstrap_authority_ingest",
    "bootstrap_extinction_validator",
    "bootstrap_receipt_validator",
    "emergency_fallback_validator",
    "foreign_surface_closure_validator",
    "host_boundary_challenge_validator",
    "operator_handoff_validator",
    "seed_runtime_contract_validator",
];
const ALLOWED_ASSERTION_MODES: &[&str] = &[
    "parse_all_cases_before_accept",
    "expected_error_exact",
    "negative_never_accepts",
];
const ALLOWED_RECEIPT_POLICIES: &[&str] = &[
    "emit_rejected_receipt_per_case",
    "receipt_bound_replay",
    "golden_receipt_parity",
];
const ALLOWED_OWNER_ROOTS: &[&str] = &[
    "lyralang",
    "interfaces",
    "k0",
    "ops",
    "src",
    "fixtures",
    "goldens",
    "receipts",
    "tests",
    "shells",
    "docs",
    "products",
    "examples",
];
const ALLOWED_EXPECTED_ERRORS: &[&str] = &[
    "ambient_authority",
    "ambient_network_allowed",
    "ambient_time_allowed",
    "closure_before_receipt",
    "corpus_drift_accepted",
    "missing_master_authority",
    "probabilistic_truth_allowed",
    "receipt_hash_mismatch",
    "unknown_evidence_path",
];
const FORBIDDEN_BOOTSTRAP_FALSIFICATION_TEXT: &[(&str, ErrorCode)] = &[
    ("negative accepted", ErrorCode::NegativeFixtureAccepted),
    ("accept negative", ErrorCode::NegativeFixtureAccepted),
    ("expected pass", ErrorCode::NegativeFixtureAccepted),
    ("manual only", ErrorCode::CorpusDriftAccepted),
    ("human only", ErrorCode::CorpusDriftAccepted),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    ("ambient time allowed", ErrorCode::AmbientTimeAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_bootstrap_falsification_surface(
    input: &str,
) -> Result<BootstrapFalsificationSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => {
            return Err(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "input",
                format!("{error:?}"),
            )])
        }
    };
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "empty bootstrap falsification surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P02_BOOTSTRAP_FALSIFICATION_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_FALSIFICATION_CONTRACT}"),
        ));
    }

    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut cases = Vec::new();
    let mut harnesses = Vec::new();
    let mut assertions = Vec::new();
    let mut artifacts = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_cases = BTreeSet::new();
    let mut seen_harnesses = BTreeSet::new();
    let mut seen_assertions = BTreeSet::new();
    let mut seen_artifacts = BTreeSet::new();
    let mut seen_proofs = BTreeSet::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
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
                    "rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if left == "case" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id",
                    "target",
                    "validator",
                    "mutation",
                    "expected",
                    "fixture",
                    "status",
                ],
                "case",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_cases.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateNegativeCase,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap falsification case {id}"),
                ));
            }
            cases.push(BootstrapFalsificationCaseBinding {
                line_number,
                id,
                target_domain: field(&fields, "target"),
                target_validator: field(&fields, "validator"),
                mutation: field(&fields, "mutation"),
                expected_error: field(&fields, "expected"),
                fixture: field(&fields, "fixture"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "harness" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &["id", "runner", "cases", "mode", "receipt", "status"],
                "harness",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_harnesses.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateFalsificationHarness,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap falsification harness {id}"),
                ));
            }
            harnesses.push(BootstrapFalsificationHarnessBinding {
                line_number,
                id,
                runner: field(&fields, "runner"),
                cases: list_field(&fields, "cases"),
                assertion_mode: field(&fields, "mode"),
                receipt_policy: field(&fields, "receipt"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "assertion" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &["id", "case", "expected", "surface", "forbids", "status"],
                "assertion",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_assertions.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRejectionAssertion,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap rejection assertion {id}"),
                ));
            }
            assertions.push(BootstrapRejectionAssertionBinding {
                line_number,
                id,
                case_id: field(&fields, "case"),
                expected_error: field(&fields, "expected"),
                proof_surface: field(&fields, "surface"),
                forbids: list_field(&fields, "forbids"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "artifact" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &["id", "owner", "path", "kind", "status"],
                "artifact",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_artifacts.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap falsification artifact {id}"),
                ));
            }
            artifacts.push(BootstrapFalsificationArtifactBinding {
                line_number,
                id,
                owner_root: field(&fields, "owner"),
                path: field(&fields, "path"),
                artifact_kind: field(&fields, "kind"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "proof" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id",
                    "cases",
                    "harnesses",
                    "assertions",
                    "artifacts",
                    "receipt",
                    "status",
                ],
                "proof",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_proofs.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateFalsificationProof,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap falsification proof {id}"),
                ));
            }
            proofs.push(BootstrapFalsificationProofBinding {
                line_number,
                id,
                cases: list_field(&fields, "cases"),
                harnesses: list_field(&fields, "harnesses"),
                assertions: list_field(&fields, "assertions"),
                artifacts: list_field(&fields, "artifacts"),
                receipt: field(&fields, "receipt"),
                status: field(&fields, "status"),
            });
            continue;
        }
        match left {
            "phase" => set_scalar(
                &mut phase,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            "task" => set_scalar(
                &mut task,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            "status" => set_scalar(
                &mut status,
                value,
                left,
                line_number,
                &mut seen_scalars,
                &mut errors,
            ),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown bootstrap falsification line {line}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(BootstrapFalsificationSurface {
        header,
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
        rules,
        cases,
        harnesses,
        assertions,
        artifacts,
        proofs,
    })
}

pub fn validate_bootstrap_falsification_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_bootstrap_falsification_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_bootstrap_falsification(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_falsification_model(surface: &BootstrapFalsificationSurface) -> Verdict {
    let mut errors = Vec::new();
    validate_bootstrap_falsification(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_bootstrap_falsification(
    surface: &BootstrapFalsificationSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-016" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-016 got {}", surface.task),
        ));
    }
    validate_status("surface", "P02-016", 0, &surface.status, errors);

    for rule in REQUIRED_BOOTSTRAP_FALSIFICATION_RULES {
        match surface.rules.get(*rule) {
            Some(value) if value == "required" || value == "forbidden" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingFalsificationRule,
                format!("rule:{rule}"),
                format!("expected required/forbidden got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingFalsificationRule,
                format!("rule:{rule}"),
                "missing bootstrap falsification rule",
            )),
        }
    }

    require_ids(
        "case",
        REQUIRED_BOOTSTRAP_FALSIFICATION_CASES,
        surface.cases.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingNegativeCase,
        errors,
    );
    require_ids(
        "harness",
        REQUIRED_BOOTSTRAP_FALSIFICATION_HARNESSES,
        surface
            .harnesses
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingFalsificationHarness,
        errors,
    );
    require_ids(
        "assertion",
        REQUIRED_BOOTSTRAP_REJECTION_ASSERTIONS,
        surface
            .assertions
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingRejectionAssertion,
        errors,
    );
    require_ids(
        "artifact",
        REQUIRED_BOOTSTRAP_FALSIFICATION_ARTIFACTS,
        surface
            .artifacts
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingDeliveryArtifact,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_BOOTSTRAP_FALSIFICATION_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingFalsificationProof,
        errors,
    );

    check_duplicate_bindings(
        "case",
        surface
            .cases
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        ErrorCode::DuplicateNegativeCase,
        errors,
    );
    check_duplicate_bindings(
        "harness",
        surface
            .harnesses
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        ErrorCode::DuplicateFalsificationHarness,
        errors,
    );
    check_duplicate_bindings(
        "assertion",
        surface
            .assertions
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        ErrorCode::DuplicateRejectionAssertion,
        errors,
    );
    check_duplicate_bindings(
        "artifact",
        surface
            .artifacts
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        ErrorCode::DuplicateEntry,
        errors,
    );
    check_duplicate_bindings(
        "proof",
        surface
            .proofs
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        ErrorCode::DuplicateFalsificationProof,
        errors,
    );

    let case_ids: BTreeSet<&str> = surface.cases.iter().map(|item| item.id.as_str()).collect();
    let harness_ids: BTreeSet<&str> = surface
        .harnesses
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let assertion_ids: BTreeSet<&str> = surface
        .assertions
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let artifact_ids: BTreeSet<&str> = surface
        .artifacts
        .iter()
        .map(|item| item.id.as_str())
        .collect();

    for case in &surface.cases {
        validate_status("case", &case.id, case.line_number, &case.status, errors);
        if !ALLOWED_TARGET_DOMAINS.contains(&case.target_domain.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("line:{:03}", case.line_number),
                format!(
                    "case {} has invalid target domain {}",
                    case.id, case.target_domain
                ),
            ));
        }
        if !ALLOWED_TARGET_VALIDATORS.contains(&case.target_validator.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("line:{:03}", case.line_number),
                format!(
                    "case {} has invalid target validator {}",
                    case.id, case.target_validator
                ),
            ));
        }
        if !ALLOWED_EXPECTED_ERRORS.contains(&case.expected_error.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("line:{:03}", case.line_number),
                format!(
                    "case {} has invalid expected error {}",
                    case.id, case.expected_error
                ),
            ));
        }
        if !case.fixture.ends_with(".lyra")
            || case.fixture.contains("..")
            || !case
                .fixture
                .starts_with("fixtures/p02/bootstrap_falsification_cases/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("line:{:03}", case.line_number),
                format!("case {} fixture path is invalid", case.id),
            ));
        }
        let Some(descriptor) = bootstrap_falsification_case_descriptor(&case.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("line:{:03}", case.line_number),
                format!("unknown bootstrap falsification case {}", case.id),
            ));
            continue;
        };
        if case.target_domain != descriptor.target_domain
            || case.target_validator != descriptor.target_validator
            || case.mutation != descriptor.mutation
            || case.expected_error != descriptor.expected_error
            || case.fixture != descriptor.fixture
            || case.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CorpusDriftAccepted,
                format!("line:{:03}", case.line_number),
                format!("case descriptor drift {}", case.id),
            ));
        }
        if bootstrap_falsification_case_digest(&case.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("line:{:03}", case.line_number),
                format!("case {} is not digestible", case.id),
            ));
        }
    }

    for harness in &surface.harnesses {
        validate_status(
            "harness",
            &harness.id,
            harness.line_number,
            &harness.status,
            errors,
        );
        if !ALLOWED_ASSERTION_MODES.contains(&harness.assertion_mode.as_str())
            || !ALLOWED_RECEIPT_POLICIES.contains(&harness.receipt_policy.as_str())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationHarness,
                format!("line:{:03}", harness.line_number),
                format!("harness {} has invalid mode or receipt policy", harness.id),
            ));
        }
        let Some(descriptor) = bootstrap_falsification_harness_descriptor(&harness.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationHarness,
                format!("line:{:03}", harness.line_number),
                format!("unknown bootstrap falsification harness {}", harness.id),
            ));
            continue;
        };
        if harness.runner != descriptor.runner
            || harness.cases
                != descriptor
                    .cases
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || harness.assertion_mode != descriptor.assertion_mode
            || harness.receipt_policy != descriptor.receipt_policy
            || harness.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CorpusDriftAccepted,
                format!("line:{:03}", harness.line_number),
                format!("harness descriptor drift {}", harness.id),
            ));
        }
        for case_id in &harness.cases {
            if !case_ids.contains(case_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::FalsificationProofUnbound,
                    format!("line:{:03}", harness.line_number),
                    format!("harness {} references unknown case {}", harness.id, case_id),
                ));
            }
        }
        if bootstrap_falsification_harness_digest(&harness.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationHarness,
                format!("line:{:03}", harness.line_number),
                format!("harness {} is not digestible", harness.id),
            ));
        }
    }

    for assertion in &surface.assertions {
        validate_status(
            "assertion",
            &assertion.id,
            assertion.line_number,
            &assertion.status,
            errors,
        );
        if !case_ids.contains(assertion.case_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRejectionAssertion,
                format!("line:{:03}", assertion.line_number),
                format!(
                    "assertion {} references unknown case {}",
                    assertion.id, assertion.case_id
                ),
            ));
        }
        if !ALLOWED_EXPECTED_ERRORS.contains(&assertion.expected_error.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRejectionAssertion,
                format!("line:{:03}", assertion.line_number),
                format!(
                    "assertion {} has invalid expected error {}",
                    assertion.id, assertion.expected_error
                ),
            ));
        }
        if assertion.forbids.is_empty()
            || !assertion
                .forbids
                .iter()
                .any(|item| item == "negative_fixture_accepted")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRejectionAssertion,
                format!("line:{:03}", assertion.line_number),
                format!(
                    "assertion {} must forbid negative_fixture_accepted",
                    assertion.id
                ),
            ));
        }
        let Some(descriptor) = bootstrap_rejection_assertion_descriptor(&assertion.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRejectionAssertion,
                format!("line:{:03}", assertion.line_number),
                format!("unknown bootstrap rejection assertion {}", assertion.id),
            ));
            continue;
        };
        if assertion.case_id != descriptor.case_id
            || assertion.expected_error != descriptor.expected_error
            || assertion.proof_surface != descriptor.proof_surface
            || assertion.forbids
                != descriptor
                    .forbids
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || assertion.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CorpusDriftAccepted,
                format!("line:{:03}", assertion.line_number),
                format!("assertion descriptor drift {}", assertion.id),
            ));
        }
        if let Some(case) = surface.case_by_id(&assertion.case_id) {
            if case.expected_error != assertion.expected_error {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRejectionAssertion,
                    format!("line:{:03}", assertion.line_number),
                    format!(
                        "assertion {} expected error does not match case {}",
                        assertion.id, assertion.case_id
                    ),
                ));
            }
        }
        if bootstrap_rejection_assertion_digest(&assertion.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRejectionAssertion,
                format!("line:{:03}", assertion.line_number),
                format!("assertion {} is not digestible", assertion.id),
            ));
        }
    }

    for artifact in &surface.artifacts {
        validate_status(
            "artifact",
            &artifact.id,
            artifact.line_number,
            &artifact.status,
            errors,
        );
        if !ALLOWED_OWNER_ROOTS.contains(&artifact.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                format!("line:{:03}", artifact.line_number),
                format!(
                    "artifact {} owner root {} is not allowed",
                    artifact.id, artifact.owner_root
                ),
            ));
        }
        if artifact.path.contains("..") || artifact.path.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} path is invalid", artifact.id),
            ));
        }
        let Some(descriptor) = bootstrap_falsification_artifact_descriptor(&artifact.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", artifact.line_number),
                format!("unknown bootstrap falsification artifact {}", artifact.id),
            ));
            continue;
        };
        if artifact.owner_root != descriptor.owner_root
            || artifact.path != descriptor.path
            || artifact.artifact_kind != descriptor.artifact_kind
            || artifact.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CorpusDriftAccepted,
                format!("line:{:03}", artifact.line_number),
                format!("artifact descriptor drift {}", artifact.id),
            ));
        }
        if bootstrap_falsification_artifact_digest(&artifact.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} is not digestible", artifact.id),
            ));
        }
    }

    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        let Some(descriptor) = bootstrap_falsification_proof_descriptor(&proof.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationProof,
                format!("line:{:03}", proof.line_number),
                format!("unknown bootstrap falsification proof {}", proof.id),
            ));
            continue;
        };
        if proof.cases
            != descriptor
                .cases
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
            || proof.harnesses
                != descriptor
                    .harnesses
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.assertions
                != descriptor
                    .assertions
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.artifacts
                != descriptor
                    .artifacts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.receipt != descriptor.receipt
            || proof.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::CorpusDriftAccepted,
                format!("line:{:03}", proof.line_number),
                format!("proof descriptor drift {}", proof.id),
            ));
        }
        for id in &proof.cases {
            if !case_ids.contains(id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::FalsificationProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown case {}", proof.id, id),
                ));
            }
        }
        for id in &proof.harnesses {
            if !harness_ids.contains(id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::FalsificationProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown harness {}", proof.id, id),
                ));
            }
        }
        for id in &proof.assertions {
            if !assertion_ids.contains(id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::FalsificationProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown assertion {}", proof.id, id),
                ));
            }
        }
        for id in &proof.artifacts {
            if !artifact_ids.contains(id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::FalsificationProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown artifact {}", proof.id, id),
                ));
            }
        }
        if !proof.receipt.ends_with(".receipt")
            || proof.receipt.contains("..")
            || !proof.receipt.starts_with("receipts/p02/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} receipt path is invalid", proof.id),
            ));
        }
        if bootstrap_falsification_proof_digest(&proof.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} is not digestible", proof.id),
            ));
        }
    }

    if !bootstrap_falsification_harnesses_bind_known_cases()
        || !bootstrap_rejection_assertions_bind_known_cases()
        || !bootstrap_falsification_artifacts_bind_paths()
        || !bootstrap_falsification_proofs_bind_registry()
        || !bootstrap_falsification_targets_all_required_domains()
        || !bootstrap_falsification_no_forbidden_descriptor_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::CorpusDriftAccepted,
            "bootstrap_falsification_registry",
            "bootstrap falsification descriptor registry is incomplete or drifted",
        ));
    }

    let case_rows: Vec<(String, String, String, String, String, String, String)> = surface
        .cases
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.target_domain.clone(),
                item.target_validator.clone(),
                item.mutation.clone(),
                item.expected_error.clone(),
                item.fixture.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let harness_rows: Vec<(String, String, Vec<String>, String, String, String)> = surface
        .harnesses
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.runner.clone(),
                item.cases.clone(),
                item.assertion_mode.clone(),
                item.receipt_policy.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let assertion_rows: Vec<(String, String, String, String, Vec<String>, String)> = surface
        .assertions
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.case_id.clone(),
                item.expected_error.clone(),
                item.proof_surface.clone(),
                item.forbids.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let artifact_rows: Vec<(String, String, String, String, String)> = surface
        .artifacts
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.owner_root.clone(),
                item.path.clone(),
                item.artifact_kind.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let proof_rows: Vec<(
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
        String,
    )> = surface
        .proofs
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.cases.clone(),
                item.harnesses.clone(),
                item.assertions.clone(),
                item.artifacts.clone(),
                item.receipt.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let suite = deterministic_bootstrap_falsification_suite_report(
        &case_rows,
        &harness_rows,
        &assertion_rows,
        &artifact_rows,
        &proof_rows,
    );
    if suite.case_count < bootstrap_falsification_case_ids().len()
        || suite.harness_count < bootstrap_falsification_harness_ids().len()
        || suite.assertion_count < bootstrap_rejection_assertion_ids().len()
        || suite.artifact_count < bootstrap_falsification_artifact_ids().len()
        || suite.proof_count < bootstrap_falsification_proof_ids().len()
        || suite.bootstrap_trust_case_count == 0
        || suite.seed_runtime_law_case_count == 0
        || suite.host_extinction_case_count == 0
        || suite.foreign_boundary_case_count == 0
        || suite.operator_handoff_case_count == 0
        || suite.emergency_fallback_case_count == 0
        || suite.receipt_commit_case_count == 0
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !bootstrap_falsification_registry_hash().starts_with("fnv1a128:")
        || LYRA_P02_BOOTSTRAP_FALSIFICATION_CARRIER != "lyra_p02_bootstrap_falsification"
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineSurface,
            "suite",
            "bootstrap falsification suite report is incomplete or unhashable",
        ));
    }
}

fn set_scalar(
    target: &mut Option<String>,
    value: &str,
    name: &str,
    line_number: usize,
    seen: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    if !seen.insert(name.to_string()) || target.is_some() {
        errors.push(ValidationError::reject(
            ErrorCode::DuplicateEntry,
            format!("line:{line_number:03}"),
            format!("duplicate scalar {name}"),
        ));
    } else {
        *target = Some(value.to_string());
    }
}

fn require_ids(
    kind: &str,
    required: &[&str],
    actual: BTreeSet<&str>,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    for id in required {
        if !actual.contains(id) {
            errors.push(ValidationError::reject(
                code,
                format!("{kind}:{id}"),
                format!("missing required bootstrap falsification {kind} {id}"),
            ));
        }
    }
}

fn check_duplicate_bindings(
    kind: &str,
    items: Vec<(&str, usize)>,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for (id, line_number) in items {
        if !seen.insert(id.to_string()) {
            errors.push(ValidationError::reject(
                code,
                format!("line:{line_number:03}"),
                format!("duplicate bootstrap falsification {kind} {id}"),
            ));
        }
    }
}

fn validate_status(
    kind: &str,
    id: &str,
    line_number: usize,
    status: &str,
    errors: &mut Vec<ValidationError>,
) {
    if !ALLOWED_STATUSES.contains(&status) {
        let locus = if line_number == 0 {
            "status".to_string()
        } else {
            format!("line:{line_number:03}")
        };
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            locus,
            format!("{kind} {id} has unsupported status {status}"),
        ));
    }
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_BOOTSTRAP_FALSIFICATION_TEXT {
        if lower.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden_text",
                format!("forbidden token {token}"),
            ));
        }
    }
}

fn parse_pipe_fields(value: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        if let Some((key, val)) = part.split_once(':') {
            fields.insert(key.to_string(), val.to_string());
        }
    }
    fields
}

fn require_fields(
    fields: &BTreeMap<String, String>,
    required: &[&str],
    kind: &str,
    line_number: usize,
    errors: &mut Vec<ValidationError>,
) {
    for key in required {
        if !fields.contains_key(*key) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("{kind} requires {key}"),
            ));
        }
    }
}

fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    fields.get(name).cloned().unwrap_or_default()
}

fn list_field(fields: &BTreeMap<String, String>, name: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    fields
        .get(name)
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .filter(|item| seen.insert((*item).to_string()))
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        && value.as_bytes()[0].is_ascii_lowercase()
}
