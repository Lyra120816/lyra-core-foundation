use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_falsification::deterministic_falsification_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_falsification_model::{
    FalsificationCorpusSurface, FalsificationHarness, FalsificationProof, NegativeCorpusCase,
    RejectionAssertion,
};

pub const P00_FALSIFICATION_CONTRACT: &str = "LYRA-P00-FALSIFICATION-CORPUS v1";

pub const REQUIRED_FALSIFICATION_RULES: &[&str] = &[
    "negative_corpus_required",
    "falsification_harness_required",
    "deterministic_rejection_required",
    "expected_error_required",
    "challenge_fixture_required",
    "no_accepting_negative",
    "receipt_emission_required",
    "replay_witness_required",
    "phase_open_until_falsification_proven",
];

pub const REQUIRED_NEGATIVE_CASES: &[&str] = &[
    "ambient_time_case",
    "hidden_randomness_case",
    "ambient_network_case",
    "placeholder_case",
    "fake_closure_case",
    "authority_override_case",
    "underbuild_case",
    "canonical_drift_case",
    "engine_drift_case",
];

pub const REQUIRED_FALSIFICATION_HARNESSES: &[&str] = &[
    "corpus_parser",
    "rejection_runner",
    "error_code_matcher",
    "receipt_replay_checker",
];

pub const REQUIRED_REJECTION_ASSERTIONS: &[&str] = &[
    "ambient_time_rejection",
    "hidden_randomness_rejection",
    "ambient_network_rejection",
    "placeholder_rejection",
    "fake_closure_rejection",
    "authority_override_rejection",
    "underbuild_rejection",
    "canonical_drift_rejection",
    "engine_drift_rejection",
];

pub const REQUIRED_FALSIFICATION_PROOFS: &[&str] = &[
    "negative_corpus_execution",
    "expected_error_alignment",
    "replay_receipt_witness",
    "p00_phase_open",
];

const ALLOWED_TARGET_VALIDATORS: &[&str] = &[
    "constitution",
    "authority_order",
    "enforcement",
    "canonical_model",
    "deterministic_engine",
    "falsification_corpus",
];
const ALLOWED_CATEGORIES: &[&str] = &[
    "safety",
    "authority",
    "delivery",
    "canonicalization",
    "engine",
    "governance",
];
const ALLOWED_OWNER_ROOTS: &[&str] = &["k0", "interfaces", "ops", "fixtures", "tests"];
const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["task", "error_alignment", "replay", "phase"];
const REQUIRED_EXPECTED_CODES: &[&str] = &[
    "ambient_time_allowed",
    "hidden_randomness_allowed",
    "ambient_network_allowed",
    "placeholder_allowed",
    "fake_closure_claim",
    "operator_override_constitution",
    "underbuild_violation",
    "canonical_model_drift_accepted",
    "engine_drift_accepted",
];

const FORBIDDEN_FALSIFICATION_TEXT: &[(&str, ErrorCode)] = &[
    ("negative accepted", ErrorCode::NegativeFixtureAccepted),
    ("accept negative", ErrorCode::NegativeFixtureAccepted),
    ("expected pass", ErrorCode::NegativeFixtureAccepted),
    ("corpus drift accepted", ErrorCode::CorpusDriftAccepted),
    ("manual only", ErrorCode::CorpusDriftAccepted),
    ("human only", ErrorCode::CorpusDriftAccepted),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("falsification placeholder", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_falsification_surface(
    input: &str,
) -> Result<FalsificationCorpusSurface, Vec<ValidationError>> {
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
            "no falsification corpus lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_FALSIFICATION_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_FALSIFICATION_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut cases = Vec::new();
    let mut harnesses = Vec::new();
    let mut assertions = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_cases = BTreeSet::new();
    let mut seen_harnesses = BTreeSet::new();
    let mut seen_assertions = BTreeSet::new();
    let mut seen_proofs = BTreeSet::new();

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
                    "falsification rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(case_id) = left.strip_prefix("case:") {
            if !is_symbolic_name(case_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidNegativeCase,
                    format!("line:{line_number:03}"),
                    format!("invalid negative case identity {case_id}"),
                ));
                continue;
            }
            if !seen_cases.insert(case_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateNegativeCase,
                    format!("case:{case_id}"),
                    "negative case identity must be unique",
                ));
                continue;
            }
            match parse_negative_case(line_number, case_id, value) {
                Ok(case) => cases.push(case),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(harness_id) = left.strip_prefix("harness:") {
            if !is_symbolic_name(harness_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidFalsificationHarness,
                    format!("line:{line_number:03}"),
                    format!("invalid harness identity {harness_id}"),
                ));
                continue;
            }
            if !seen_harnesses.insert(harness_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateFalsificationHarness,
                    format!("harness:{harness_id}"),
                    "harness identity must be unique",
                ));
                continue;
            }
            match parse_harness(line_number, harness_id, value) {
                Ok(harness) => harnesses.push(harness),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(assertion_id) = left.strip_prefix("assertion:") {
            if !is_symbolic_name(assertion_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidRejectionAssertion,
                    format!("line:{line_number:03}"),
                    format!("invalid assertion identity {assertion_id}"),
                ));
                continue;
            }
            if !seen_assertions.insert(assertion_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateRejectionAssertion,
                    format!("assertion:{assertion_id}"),
                    "assertion identity must be unique",
                ));
                continue;
            }
            match parse_assertion(line_number, assertion_id, value) {
                Ok(assertion) => assertions.push(assertion),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidFalsificationProof,
                    format!("line:{line_number:03}"),
                    format!("invalid falsification proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateFalsificationProof,
                    format!("proof:{proof_id}"),
                    "falsification proof identity must be unique",
                ));
                continue;
            }
            match parse_proof(line_number, proof_id, value) {
                Ok(proof) => proofs.push(proof),
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
                format!("unknown entry {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(FalsificationCorpusSurface {
        header,
        phase: phase.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingPhase,
                "surface",
                "missing phase",
            )]
        })?,
        task: task.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingTask,
                "surface",
                "missing task",
            )]
        })?,
        status: status.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                "surface",
                "missing status",
            )]
        })?,
        rules,
        cases,
        harnesses,
        assertions,
        proofs,
    })
}

pub fn validate_falsification_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    for (token, code) in FORBIDDEN_FALSIFICATION_TEXT {
        if input.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "surface",
                format!("forbidden falsification token {token}"),
            ));
        }
    }
    match parse_falsification_surface(input) {
        Ok(surface) => errors.extend(validate_falsification_surface_model(&surface).errors),
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

pub fn validate_falsification_surface_model(surface: &FalsificationCorpusSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P00 got {}", surface.phase),
        ));
    }
    if surface.task != "P00-016" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P00-016 got {}", surface.task),
        ));
    }
    if !matches!(
        surface.status.as_str(),
        "working_slice" | "artifact_emitted"
    ) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidNegativeCase,
            "status",
            format!("unsupported falsification status {}", surface.status),
        ));
    }
    for required in REQUIRED_FALSIFICATION_RULES {
        match surface.rule_value(required) {
            Some(value) if strong_required_value(value) => {}
            Some(_) => errors.push(ValidationError::reject(
                ErrorCode::MissingFalsificationRule,
                format!("rule:{required}"),
                "falsification rule must be explicit and enforced",
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingFalsificationRule,
                format!("rule:{required}"),
                "missing required falsification rule",
            )),
        }
    }

    let case_ids: BTreeSet<String> = surface.cases.iter().map(|item| item.id.clone()).collect();
    let harness_ids: BTreeSet<String> = surface
        .harnesses
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let assertion_ids: BTreeSet<String> = surface
        .assertions
        .iter()
        .map(|item| item.id.clone())
        .collect();

    for required in REQUIRED_NEGATIVE_CASES {
        if !case_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingNegativeCase,
                format!("case:{required}"),
                "missing required negative corpus case",
            ));
        }
    }
    for required in REQUIRED_FALSIFICATION_HARNESSES {
        if !harness_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingFalsificationHarness,
                format!("harness:{required}"),
                "missing required falsification harness",
            ));
        }
    }
    for required in REQUIRED_REJECTION_ASSERTIONS {
        if !assertion_ids.contains(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingRejectionAssertion,
                format!("assertion:{required}"),
                "missing required rejection assertion",
            ));
        }
    }
    for required in REQUIRED_FALSIFICATION_PROOFS {
        if surface.proof_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingFalsificationProof,
                format!("proof:{required}"),
                "missing required falsification proof",
            ));
        }
    }

    let mut expected_code_seen = BTreeSet::new();
    let mut order_by_root = BTreeMap::new();
    for case in &surface.cases {
        validate_negative_case(case, &mut expected_code_seen, &mut errors);
    }
    for required_code in REQUIRED_EXPECTED_CODES {
        if !expected_code_seen.contains(*required_code) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidNegativeCase,
                format!("expected_code:{required_code}"),
                "negative corpus must cover required expected error code",
            ));
        }
    }
    for harness in &surface.harnesses {
        validate_harness(harness, &mut order_by_root, &mut errors);
    }
    for assertion in &surface.assertions {
        validate_assertion(assertion, surface, &case_ids, &harness_ids, &mut errors);
    }
    for proof in &surface.proofs {
        validate_proof(proof, &case_ids, &harness_ids, &assertion_ids, &mut errors);
    }

    let suite_cases: Vec<(&str, &str, &str)> = surface
        .cases
        .iter()
        .map(|case| {
            (
                case.id.as_str(),
                case.expected_code.as_str(),
                case.fixture.as_str(),
            )
        })
        .collect();
    let report = deterministic_falsification_report("P00-016", &suite_cases);
    if report.case_count != surface.cases.len() || report.suite_hash.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::CorpusDriftAccepted,
            "falsification_report",
            "deterministic falsification report must bind all cases and emit suite hash",
        ));
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_negative_case(
    case: &NegativeCorpusCase,
    expected_code_seen: &mut BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = case.canonical_identity();
    if !fixture_path(&case.fixture) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidNegativeCase,
            location.clone(),
            format!(
                "negative fixture path must point to fixtures/p00/falsification_inputs: {}",
                case.fixture
            ),
        ));
    }
    if !ALLOWED_TARGET_VALIDATORS.contains(&case.target_validator.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidNegativeCase,
            location.clone(),
            format!("unsupported target validator {}", case.target_validator),
        ));
    }
    if !error_code_token(&case.expected_code) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidNegativeCase,
            location.clone(),
            format!(
                "expected code must be canonical snake-case: {}",
                case.expected_code
            ),
        ));
    } else {
        expected_code_seen.insert(case.expected_code.clone());
    }
    if !ALLOWED_CATEGORIES.contains(&case.category.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidNegativeCase,
            location.clone(),
            format!("unsupported negative case category {}", case.category),
        ));
    }
    if !case.must_reject {
        errors.push(ValidationError::reject(
            ErrorCode::NegativeFixtureAccepted,
            location.clone(),
            "negative corpus cases must reject",
        ));
    }
    if case.receipts.is_empty() || case.receipts.iter().any(|receipt| !receipt_path(receipt)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "negative case must bind canonical P00 receipts",
        ));
    }
    if !ALLOWED_STATUSES.contains(&case.status.as_str()) || case.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidNegativeCase,
            location,
            format!("unsupported negative case status {}", case.status),
        ));
    }
}

fn validate_harness(
    harness: &FalsificationHarness,
    order_by_root: &mut BTreeMap<String, BTreeSet<String>>,
    errors: &mut Vec<ValidationError>,
) {
    let location = harness.canonical_identity();
    if !ALLOWED_OWNER_ROOTS.contains(&harness.owner_root.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationHarness,
            location.clone(),
            format!("invalid harness owner root {}", harness.owner_root),
        ));
    }
    if !harness
        .module
        .starts_with(&format!("{}/", harness.owner_root))
        || !harness.module.ends_with(".rs")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationHarness,
            location.clone(),
            format!(
                "harness module must live under owner root and be Rust source: {}",
                harness.module
            ),
        ));
    }
    if harness.inputs.is_empty()
        || harness
            .inputs
            .iter()
            .any(|item| weak_value(item) || !is_symbolic_name(item))
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationHarness,
            location.clone(),
            "harness inputs must be concrete symbolic names",
        ));
    }
    if harness.outputs.is_empty()
        || harness
            .outputs
            .iter()
            .any(|item| weak_value(item) || !is_symbolic_name(item))
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationHarness,
            location.clone(),
            "harness outputs must be concrete symbolic names",
        ));
    }
    if !stable_order_token(&harness.order) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationHarness,
            location.clone(),
            format!(
                "harness order must be stable three-digit token: {}",
                harness.order
            ),
        ));
    } else {
        let seen = order_by_root.entry(harness.owner_root.clone()).or_default();
        if !seen.insert(harness.order.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFalsificationHarness,
                location.clone(),
                format!(
                    "duplicate harness order {} in root {}",
                    harness.order, harness.owner_root
                ),
            ));
        }
    }
    if harness.receipts.is_empty()
        || harness
            .receipts
            .iter()
            .any(|receipt| !receipt_path(receipt))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "harness must bind canonical P00 receipts",
        ));
    }
    if harness.commands.is_empty() || harness.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "harness must bind command records",
        ));
    }
    if !ALLOWED_STATUSES.contains(&harness.status.as_str()) || harness.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationHarness,
            location,
            format!("unsupported harness status {}", harness.status),
        ));
    }
}

fn validate_assertion(
    assertion: &RejectionAssertion,
    surface: &FalsificationCorpusSurface,
    case_ids: &BTreeSet<String>,
    harness_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = assertion.canonical_identity();
    if !case_ids.contains(&assertion.case) {
        errors.push(ValidationError::reject(
            ErrorCode::FalsificationProofUnbound,
            location.clone(),
            format!("assertion references unknown case {}", assertion.case),
        ));
    }
    if !harness_ids.contains(&assertion.harness) {
        errors.push(ValidationError::reject(
            ErrorCode::FalsificationProofUnbound,
            location.clone(),
            format!("assertion references unknown harness {}", assertion.harness),
        ));
    }
    if assertion.expected_verdict != "rejected" {
        errors.push(ValidationError::reject(
            ErrorCode::NegativeFixtureAccepted,
            location.clone(),
            format!(
                "expected verdict must be rejected got {}",
                assertion.expected_verdict
            ),
        ));
    }
    if let Some(case) = surface.case_by_id(&assertion.case) {
        if case.expected_code != assertion.expected_code {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidRejectionAssertion,
                location.clone(),
                format!(
                    "assertion expected code {} mismatches case {}",
                    assertion.expected_code, case.expected_code
                ),
            ));
        }
    }
    if assertion.receipts.is_empty()
        || assertion
            .receipts
            .iter()
            .any(|receipt| !receipt_path(receipt))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "rejection assertion must bind canonical receipts",
        ));
    }
    if assertion.commands.is_empty() || assertion.commands.iter().any(|command| weak_value(command))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "rejection assertion must bind command records",
        ));
    }
    if !ALLOWED_STATUSES.contains(&assertion.status.as_str()) || assertion.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidRejectionAssertion,
            location,
            format!("unsupported assertion status {}", assertion.status),
        ));
    }
}

fn validate_proof(
    proof: &FalsificationProof,
    case_ids: &BTreeSet<String>,
    harness_ids: &BTreeSet<String>,
    assertion_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = proof.canonical_identity();
    if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationProof,
            location.clone(),
            format!("unsupported proof scope {}", proof.scope),
        ));
    }
    if proof.scope == "phase" && proof.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "phase falsification proof must remain blocked until all P00 tasks close",
        ));
    }
    for case in &proof.cases {
        if !case_ids.contains(case) {
            errors.push(ValidationError::reject(
                ErrorCode::FalsificationProofUnbound,
                location.clone(),
                format!("unknown proof case {case}"),
            ));
        }
    }
    for harness in &proof.harnesses {
        if !harness_ids.contains(harness) {
            errors.push(ValidationError::reject(
                ErrorCode::FalsificationProofUnbound,
                location.clone(),
                format!("unknown proof harness {harness}"),
            ));
        }
    }
    for assertion in &proof.assertions {
        if !assertion_ids.contains(assertion) {
            errors.push(ValidationError::reject(
                ErrorCode::FalsificationProofUnbound,
                location.clone(),
                format!("unknown proof assertion {assertion}"),
            ));
        }
    }
    if proof.receipts.is_empty() || proof.receipts.iter().any(|receipt| !receipt_path(receipt)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "falsification proof must bind canonical receipts",
        ));
    }
    if proof.commands.is_empty() || proof.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "falsification proof must bind command records",
        ));
    }
    if proof.forbids.is_empty() || proof.forbids.iter().any(|item| weak_value(item)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationProof,
            location.clone(),
            "falsification proof forbid list must be concrete",
        ));
    }
    if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidFalsificationProof,
            location.clone(),
            format!("unsupported proof status {}", proof.status),
        ));
    }
    if proof.id == "expected_error_alignment" {
        for required in REQUIRED_NEGATIVE_CASES {
            if !proof.cases.iter().any(|case| case == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingNegativeCase,
                    location.clone(),
                    format!("expected error proof misses case {required}"),
                ));
            }
        }
    }
    if proof.id == "p00_phase_open" && !proof.forbids.iter().any(|item| item == "phase_closed") {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location,
            "phase-open proof must forbid phase_closed",
        ));
    }
}

fn parse_negative_case(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<NegativeCorpusCase, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let must_reject = parse_bool(
        line_number,
        required_string_field(line_number, &mut fields, "must_reject")?,
    )?;
    let case = NegativeCorpusCase {
        line_number,
        id: id.to_string(),
        fixture: required_string_field(line_number, &mut fields, "fixture")?,
        target_validator: required_string_field(line_number, &mut fields, "target_validator")?,
        expected_code: required_string_field(line_number, &mut fields, "expected_code")?,
        category: required_string_field(line_number, &mut fields, "category")?,
        must_reject,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(case)
}

fn parse_harness(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<FalsificationHarness, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let harness = FalsificationHarness {
        line_number,
        id: id.to_string(),
        owner_root: required_string_field(line_number, &mut fields, "owner_root")?,
        module: required_string_field(line_number, &mut fields, "module")?,
        inputs: required_list_field(line_number, &mut fields, "inputs")?,
        outputs: required_list_field(line_number, &mut fields, "outputs")?,
        order: required_string_field(line_number, &mut fields, "order")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        commands: required_list_field(line_number, &mut fields, "commands")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(harness)
}

fn parse_assertion(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<RejectionAssertion, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let assertion = RejectionAssertion {
        line_number,
        id: id.to_string(),
        case: required_string_field(line_number, &mut fields, "case")?,
        harness: required_string_field(line_number, &mut fields, "harness")?,
        expected_code: required_string_field(line_number, &mut fields, "expected_code")?,
        expected_verdict: required_string_field(line_number, &mut fields, "expected_verdict")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        commands: required_list_field(line_number, &mut fields, "commands")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(assertion)
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<FalsificationProof, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let proof = FalsificationProof {
        line_number,
        id: id.to_string(),
        scope: required_string_field(line_number, &mut fields, "scope")?,
        cases: required_list_field(line_number, &mut fields, "cases")?,
        harnesses: required_list_field(line_number, &mut fields, "harnesses")?,
        assertions: required_list_field(line_number, &mut fields, "assertions")?,
        receipts: required_list_field(line_number, &mut fields, "receipts")?,
        commands: required_list_field(line_number, &mut fields, "commands")?,
        forbids: required_list_field(line_number, &mut fields, "forbids")?,
        status: required_string_field(line_number, &mut fields, "status")?,
    };
    reject_unknown_fields(line_number, fields)?;
    Ok(proof)
}

fn parse_fields(
    line_number: usize,
    value: &str,
) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        let Some((key, val)) = part.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field must use key:value syntax",
            ));
        };
        if key.is_empty() || val.is_empty() || key != key.trim() || val != val.trim() {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "field key/value must be non-empty and trimmed",
            ));
        }
        if fields.insert(key.to_string(), val.to_string()).is_some() {
            return Err(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate field {key}"),
            ));
        }
    }
    Ok(fields)
}

fn required_string_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<String, ValidationError> {
    fields.remove(key).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("missing field {key}"),
        )
    })
}

fn required_list_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
    key: &str,
) -> Result<Vec<String>, ValidationError> {
    let value = required_string_field(line_number, fields, key)?;
    let values = split_list(&value);
    if values.is_empty() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("field {key} must contain at least one item"),
        ))
    } else {
        Ok(values)
    }
}

fn reject_unknown_fields(
    line_number: usize,
    fields: BTreeMap<String, String>,
) -> Result<(), ValidationError> {
    if let Some(key) = fields.keys().next() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            format!("unknown field {key}"),
        ))
    } else {
        Ok(())
    }
}

fn parse_bool(line_number: usize, value: String) -> Result<bool, ValidationError> {
    match value.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ValidationError::reject(
            ErrorCode::InvalidNegativeCase,
            format!("line:{line_number:03}"),
            format!("invalid boolean {value}"),
        )),
    }
}

fn split_list(value: &str) -> Vec<String> {
    let mut items: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty() && *item != "none" && *item != "nothing")
        .map(ToString::to_string)
        .collect();
    items.sort();
    items.dedup();
    items
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value == value.trim()
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        && value.as_bytes()[0].is_ascii_lowercase()
}

fn stable_order_token(value: &str) -> bool {
    value.len() == 3 && value.as_bytes().iter().all(|byte| byte.is_ascii_digit()) && value != "000"
}

fn fixture_path(value: &str) -> bool {
    value.starts_with("fixtures/p00/falsification_inputs/") && value.ends_with(".lyra")
}

fn receipt_path(value: &str) -> bool {
    value.starts_with("receipts/p00/") && value.ends_with(".receipt")
}

fn error_code_token(value: &str) -> bool {
    !value.is_empty()
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        && value.contains('_')
}

fn strong_required_value(value: &str) -> bool {
    matches!(
        value,
        "required" | "enforced" | "forbidden" | "blocked" | "receipt_bound" | "replay_required"
    )
}

fn weak_value(value: &str) -> bool {
    matches!(
        value,
        "none"
            | "nothing"
            | "declared_only"
            | "manual_only"
            | "human_only"
            | "unbound"
            | "empty"
            | "future"
            | "later"
            | "best_effort"
            | "placeholder"
            | "todo"
    )
}
