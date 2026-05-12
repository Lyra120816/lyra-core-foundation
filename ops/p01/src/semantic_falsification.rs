use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_falsification::deterministic_semantic_falsification_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_falsification::{
    semantic_falsification_artifact_descriptor, semantic_falsification_artifact_digest,
    semantic_falsification_artifact_ids, semantic_falsification_artifacts_bind_paths,
    semantic_falsification_case_descriptor, semantic_falsification_case_digest,
    semantic_falsification_case_ids, semantic_falsification_harness_descriptor,
    semantic_falsification_harness_digest, semantic_falsification_harness_ids,
    semantic_falsification_harnesses_bind_known_cases,
    semantic_falsification_no_forbidden_descriptor_claims, semantic_falsification_proof_descriptor,
    semantic_falsification_proof_digest, semantic_falsification_proof_ids,
    semantic_falsification_proofs_bind_registry, semantic_falsification_registry_hash,
    semantic_falsification_targets_all_required_domains, semantic_rejection_assertion_descriptor,
    semantic_rejection_assertion_digest, semantic_rejection_assertion_ids,
    semantic_rejection_assertions_bind_known_cases, LYRA_P01_SEMANTIC_FALSIFICATION_CARRIER,
};
use crate::p01_semantic_falsification_model::{
    SemanticFalsificationArtifactBinding, SemanticFalsificationCaseBinding,
    SemanticFalsificationHarnessBinding, SemanticFalsificationProofBinding,
    SemanticFalsificationSurface, SemanticRejectionAssertionBinding,
};

pub const P01_SEMANTIC_FALSIFICATION_CONTRACT: &str = "LYRA-P01-SEMANTIC-FALSIFICATION-CORPUS v1";

pub const REQUIRED_SEMANTIC_FALSIFICATION_RULES: &[&str] = &[
    "semantic_negative_corpus_required",
    "canonical_symbol_rejection_required",
    "semantic_atom_rejection_required",
    "core_ir_rejection_required",
    "falsification_harness_required",
    "expected_error_exact_match_required",
    "rejected_receipt_required",
    "replay_witness_required",
    "no_accepting_negative",
    "no_manual_only_corpus",
    "no_network_falsification",
    "no_probabilistic_acceptance",
    "no_phase_closure_claim",
];

pub const REQUIRED_SEMANTIC_FALSIFICATION_CASES: &[&str] = &[
    "canonical_symbol_uppercase_case",
    "canonical_symbol_duplicate_identity_case",
    "canonical_symbol_control_byte_case",
    "semantic_atom_unknown_family_case",
    "semantic_atom_descriptor_drift_case",
    "semantic_atom_probabilistic_truth_case",
    "core_ir_version_drift_case",
    "core_ir_encoding_drift_case",
    "core_ir_unbound_receipt_case",
];

pub const REQUIRED_SEMANTIC_FALSIFICATION_HARNESSES: &[&str] = &[
    "semantic_negative_corpus_parser",
    "canonical_symbol_falsifier",
    "semantic_atom_falsifier",
    "core_ir_falsifier",
    "cross_surface_receipt_replay_checker",
];

pub const REQUIRED_SEMANTIC_REJECTION_ASSERTIONS: &[&str] = &[
    "canonical_symbol_uppercase_rejection",
    "canonical_symbol_duplicate_identity_rejection",
    "canonical_symbol_control_byte_rejection",
    "semantic_atom_unknown_family_rejection",
    "semantic_atom_descriptor_drift_rejection",
    "semantic_atom_probabilistic_truth_rejection",
    "core_ir_version_drift_rejection",
    "core_ir_encoding_drift_rejection",
    "core_ir_unbound_receipt_rejection",
];

pub const REQUIRED_SEMANTIC_FALSIFICATION_ARTIFACTS: &[&str] = &[
    "semantic_falsification_contract",
    "semantic_falsification_law",
    "semantic_falsification_operator",
    "valid_semantic_falsification_fixture",
    "golden_semantic_falsification_receipt",
    "execution_semantic_falsification_receipt",
    "deterministic_semantic_falsification_report",
];

pub const REQUIRED_SEMANTIC_FALSIFICATION_PROOFS: &[&str] = &[
    "canonical_symbol_negative_proof",
    "semantic_atom_negative_proof",
    "core_ir_negative_proof",
    "p01_semantic_falsification_parity_proof",
];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const ALLOWED_TARGET_DOMAINS: &[&str] = &["canonical_symbols", "semantic_atoms", "core_ir"];
const ALLOWED_TARGET_VALIDATORS: &[&str] = &[
    "canonical_symbol_ingest",
    "semantic_core_engine",
    "semantic_identity",
    "semantic_atoms",
    "core_ir",
    "core_ir_reuse",
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
];
const ALLOWED_EXPECTED_ERRORS: &[&str] = &[
    "invalid_negative_case",
    "duplicate_identity",
    "canonical_control_byte",
    "invalid_semantic_domain",
    "semantic_drift_accepted",
    "probabilistic_truth_allowed",
    "invalid_model_binding",
    "canonical_model_drift_accepted",
    "replay_proof_unbound",
];
const FORBIDDEN_FALSIFICATION_TEXT: &[(&str, ErrorCode)] = &[
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
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_falsification_surface(
    input: &str,
) -> Result<SemanticFalsificationSurface, Vec<ValidationError>> {
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
            "empty semantic falsification surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P01_SEMANTIC_FALSIFICATION_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_FALSIFICATION_CONTRACT}"),
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
                    format!("duplicate or invalid semantic falsification case {id}"),
                ));
            }
            cases.push(SemanticFalsificationCaseBinding {
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
                    format!("duplicate or invalid semantic falsification harness {id}"),
                ));
            }
            harnesses.push(SemanticFalsificationHarnessBinding {
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
                    format!("duplicate or invalid semantic rejection assertion {id}"),
                ));
            }
            assertions.push(SemanticRejectionAssertionBinding {
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
                    format!("duplicate or invalid semantic falsification artifact {id}"),
                ));
            }
            artifacts.push(SemanticFalsificationArtifactBinding {
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
                    format!("duplicate or invalid semantic falsification proof {id}"),
                ));
            }
            proofs.push(SemanticFalsificationProofBinding {
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
                format!("unknown semantic falsification line {line}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticFalsificationSurface {
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

pub fn validate_semantic_falsification_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_semantic_falsification_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_semantic_falsification(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_falsification(
    surface: &SemanticFalsificationSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-016" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-016 got {}", surface.task),
        ));
    }
    validate_status("surface", "P01-016", 0, &surface.status, errors);

    for rule in REQUIRED_SEMANTIC_FALSIFICATION_RULES {
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
                "missing semantic falsification rule",
            )),
        }
    }

    require_ids(
        "case",
        REQUIRED_SEMANTIC_FALSIFICATION_CASES,
        surface.cases.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingNegativeCase,
        errors,
    );
    require_ids(
        "harness",
        REQUIRED_SEMANTIC_FALSIFICATION_HARNESSES,
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
        REQUIRED_SEMANTIC_REJECTION_ASSERTIONS,
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
        REQUIRED_SEMANTIC_FALSIFICATION_ARTIFACTS,
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
        REQUIRED_SEMANTIC_FALSIFICATION_PROOFS,
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
                .starts_with("fixtures/p01/semantic_falsification_cases/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("line:{:03}", case.line_number),
                format!("case {} fixture path is invalid", case.id),
            ));
        }
        let Some(descriptor) = semantic_falsification_case_descriptor(&case.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("line:{:03}", case.line_number),
                format!("unknown semantic falsification case {}", case.id),
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
        if semantic_falsification_case_digest(&case.id).is_none() {
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
        let Some(descriptor) = semantic_falsification_harness_descriptor(&harness.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationHarness,
                format!("line:{:03}", harness.line_number),
                format!("unknown semantic falsification harness {}", harness.id),
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
        if semantic_falsification_harness_digest(&harness.id).is_none() {
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
        let Some(descriptor) = semantic_rejection_assertion_descriptor(&assertion.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRejectionAssertion,
                format!("line:{:03}", assertion.line_number),
                format!("unknown semantic rejection assertion {}", assertion.id),
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
        if semantic_rejection_assertion_digest(&assertion.id).is_none() {
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
        let Some(descriptor) = semantic_falsification_artifact_descriptor(&artifact.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", artifact.line_number),
                format!("unknown semantic falsification artifact {}", artifact.id),
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
        if semantic_falsification_artifact_digest(&artifact.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} is not digestible", artifact.id),
            ));
        }
    }

    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        let Some(descriptor) = semantic_falsification_proof_descriptor(&proof.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationProof,
                format!("line:{:03}", proof.line_number),
                format!("unknown semantic falsification proof {}", proof.id),
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
        if !proof.receipt.ends_with(".receipt") || proof.receipt.contains("..") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} receipt path is invalid", proof.id),
            ));
        }
        if semantic_falsification_proof_digest(&proof.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} is not digestible", proof.id),
            ));
        }
    }

    if !semantic_falsification_harnesses_bind_known_cases()
        || !semantic_rejection_assertions_bind_known_cases()
        || !semantic_falsification_artifacts_bind_paths()
        || !semantic_falsification_proofs_bind_registry()
        || !semantic_falsification_targets_all_required_domains()
        || !semantic_falsification_no_forbidden_descriptor_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::CorpusDriftAccepted,
            "semantic_falsification_registry",
            "semantic falsification descriptor registry is incomplete or drifted",
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
    let suite = deterministic_semantic_falsification_suite_report(
        &case_rows,
        &harness_rows,
        &assertion_rows,
        &artifact_rows,
        &proof_rows,
    );
    if suite.case_count < semantic_falsification_case_ids().len()
        || suite.harness_count < semantic_falsification_harness_ids().len()
        || suite.assertion_count < semantic_rejection_assertion_ids().len()
        || suite.artifact_count < semantic_falsification_artifact_ids().len()
        || suite.proof_count < semantic_falsification_proof_ids().len()
        || suite.canonical_symbol_case_count == 0
        || suite.semantic_atom_case_count == 0
        || suite.core_ir_case_count == 0
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !semantic_falsification_registry_hash().starts_with("fnv1a128:")
        || LYRA_P01_SEMANTIC_FALSIFICATION_CARRIER != "lyra_p01_semantic_falsification"
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineSurface,
            "suite",
            "semantic falsification suite report is incomplete or unhashable",
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
                format!("missing required semantic falsification {kind} {id}"),
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
                format!("duplicate semantic falsification {kind} {id}"),
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
    for (token, code) in FORBIDDEN_FALSIFICATION_TEXT {
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
