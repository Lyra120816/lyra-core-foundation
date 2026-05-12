use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_acceptance_model::{
    AcceptanceGolden, AcceptanceProof, AcceptanceProofSurface, ChallengeFixture,
};

pub const P00_ACCEPTANCE_PROOF_CONTRACT: &str = "LYRA-P00-ACCEPTANCE-PROOF v1";

pub const REQUIRED_ACCEPTANCE_RULES: &[&str] = &[
    "constitutional_goldens_required",
    "challenge_fixtures_required",
    "acceptance_proofs_required",
    "executable_surface_required",
    "receipt_chain_required",
    "negative_path_required",
    "rollback_path_required",
    "no_phase_closure_without_all_tasks",
];

pub const REQUIRED_ACCEPTANCE_GOLDENS: &[&str] = &[
    "constitution_valid",
    "authority_order_valid",
    "identity_law_valid",
    "enforcement_law_valid",
    "delivery_protocol_valid",
    "challenge_law_valid",
    "control_surfaces_valid",
    "owner_root_law_valid",
    "benchmark_evidence_valid",
    "public_interest_valid",
    "canon_compliance_valid",
];

pub const REQUIRED_CHALLENGE_FIXTURES: &[&str] = &[
    "missing_determinism",
    "placeholder_permission",
    "ambient_authority",
    "fake_closure",
    "rollback_without_receipt",
    "canon_drift_accepted",
    "missing_acceptance_receipt",
];

pub const REQUIRED_ACCEPTANCE_PROOFS: &[&str] = &[
    "constitutional_goldens",
    "challenge_fixture_execution",
    "governance_surface_execution",
    "p00_012_local",
    "p00_phase_open",
];

const ALLOWED_GOLDEN_KINDS: &[&str] = &[
    "constitution",
    "authority",
    "identity",
    "enforcement",
    "delivery",
    "challenge",
    "control",
    "owner_root",
    "benchmark_evidence",
    "public_interest",
    "canon_compliance",
];
const ALLOWED_FIXTURE_KINDS: &[&str] = &[
    "negative",
    "challenge",
    "rollback",
    "adversarial",
    "malformed",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["task", "phase", "governance", "acceptance"];
const ALLOWED_STATUSES: &[&str] = &[
    "accepted",
    "rejected",
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const EXECUTED_TASKS: &[&str] = &[
    "P00-001", "P00-002", "P00-003", "P00-004", "P00-005", "P00-006", "P00-007", "P00-008",
    "P00-009", "P00-010", "P00-011", "P00-012",
];

const FORBIDDEN_ACCEPTANCE_TEXT: &[(&str, ErrorCode)] = &[
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("manual proof only", ErrorCode::InvalidAcceptanceProof),
    ("accept without receipt", ErrorCode::MissingReceiptProof),
    ("challenge optional", ErrorCode::MissingChallengeFixture),
    ("golden optional", ErrorCode::MissingAcceptanceGolden),
    ("negative path optional", ErrorCode::MissingChallengeFixture),
    ("rollback path optional", ErrorCode::RollbackWithoutReceipt),
];

pub fn parse_acceptance_proof_surface(
    input: &str,
) -> Result<AcceptanceProofSurface, Vec<ValidationError>> {
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
            "no acceptance-proof lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_ACCEPTANCE_PROOF_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_ACCEPTANCE_PROOF_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut goldens = Vec::new();
    let mut fixtures = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_goldens = BTreeSet::new();
    let mut seen_fixtures = BTreeSet::new();
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

        if value.is_empty() || value != value.trim() || left.is_empty() || left != left.trim() {
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
                    "acceptance rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(golden_id) = left.strip_prefix("golden:") {
            if !is_symbolic_name(golden_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidAcceptanceGolden,
                    format!("line:{line_number:03}"),
                    format!("invalid golden identity {golden_id}"),
                ));
                continue;
            }
            if !seen_goldens.insert(golden_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateAcceptanceGolden,
                    format!("golden:{golden_id}"),
                    "golden identity must be unique",
                ));
                continue;
            }
            match parse_golden(line_number, golden_id, value) {
                Ok(item) => goldens.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(fixture_id) = left.strip_prefix("fixture:") {
            if !is_symbolic_name(fixture_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidChallengeFixture,
                    format!("line:{line_number:03}"),
                    format!("invalid fixture identity {fixture_id}"),
                ));
                continue;
            }
            if !seen_fixtures.insert(fixture_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateChallengeFixture,
                    format!("fixture:{fixture_id}"),
                    "fixture identity must be unique",
                ));
                continue;
            }
            match parse_fixture(line_number, fixture_id, value) {
                Ok(item) => fixtures.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidAcceptanceProof,
                    format!("line:{line_number:03}"),
                    format!("invalid proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateAcceptanceProof,
                    format!("proof:{proof_id}"),
                    "proof identity must be unique",
                ));
                continue;
            }
            match parse_proof(line_number, proof_id, value) {
                Ok(item) => proofs.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if !seen_scalars.insert(left.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                left.to_string(),
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
                format!("unknown acceptance-proof field {left}"),
            )),
        }
    }

    let phase = match phase {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPhase,
                "field:phase",
                "phase=P00 is required",
            ));
            String::new()
        }
    };
    let task = match task {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::MissingTask,
                "field:task",
                "task=P00-012 is required",
            ));
            String::new()
        }
    };
    let status = match status {
        Some(value) => value,
        None => {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                "field:status",
                "status=working_slice is required",
            ));
            String::new()
        }
    };

    if errors.is_empty() {
        Ok(AcceptanceProofSurface {
            header,
            phase,
            task,
            status,
            rules,
            goldens,
            fixtures,
            proofs,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_acceptance_proof_surface(input: &str) -> (Verdict, Receipt) {
    let canonical_text = match canonical_surface_text(input) {
        Ok(text) => text,
        Err(error) => {
            let verdict = Verdict::rejected(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "byte-stream",
                format!("{error:?}"),
            )]);
            let receipt = build_receipt(input, "", verdict.clone());
            return (verdict, receipt);
        }
    };

    let verdict = match parse_acceptance_proof_surface(input) {
        Ok(surface) => validate_parsed_acceptance_proof_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_golden(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<AcceptanceGolden, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let path = required_string_field(line_number, &mut fields, "path")?;
    let source_task = required_string_field(line_number, &mut fields, "source_task")?;
    let receipt = required_string_field(line_number, &mut fields, "receipt")?;
    let hash = required_string_field(line_number, &mut fields, "hash")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(AcceptanceGolden {
        line_number,
        id: id.to_string(),
        kind,
        path,
        source_task,
        receipt,
        hash,
        status,
    })
}

fn parse_fixture(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<ChallengeFixture, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let path = required_string_field(line_number, &mut fields, "path")?;
    let task = required_string_field(line_number, &mut fields, "task")?;
    let expects = required_string_field(line_number, &mut fields, "expects")?;
    let receipt = required_string_field(line_number, &mut fields, "receipt")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(ChallengeFixture {
        line_number,
        id: id.to_string(),
        kind,
        path,
        task,
        expects,
        receipt,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<AcceptanceProof, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let tasks = required_list_field(line_number, &mut fields, "tasks")?;
    let goldens = required_list_field(line_number, &mut fields, "goldens")?;
    let fixtures = required_list_field(line_number, &mut fields, "fixtures")?;
    let receipts = required_list_field(line_number, &mut fields, "receipts")?;
    let commands = required_commands_field(line_number, &mut fields)?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(AcceptanceProof {
        line_number,
        id: id.to_string(),
        scope,
        tasks,
        goldens,
        fixtures,
        receipts,
        commands,
        status,
        forbids,
    })
}

fn validate_parsed_acceptance_proof_surface(
    surface: &AcceptanceProofSurface,
    raw_input: &str,
) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            "acceptance proof law is scoped to P00",
        ));
    }
    if surface.task != "P00-012" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            "acceptance proof law must bind P00-012",
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-012 may only claim working_slice in this pass",
        ));
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_ACCEPTANCE_TEXT {
        if lowered.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "acceptance:text",
                format!("forbidden acceptance phrase detected: {token}"),
            ));
        }
    }

    for required in REQUIRED_ACCEPTANCE_RULES {
        match surface.rule_value(required) {
            Some(value) if value.starts_with("required:") || value.starts_with("forbidden:") => {}
            Some(_) => errors.push(ValidationError::reject(
                ErrorCode::MissingAcceptanceRule,
                format!("rule:{required}"),
                "acceptance rule must be explicit required: or forbidden:",
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingAcceptanceRule,
                format!("rule:{required}"),
                "required acceptance rule missing",
            )),
        }
    }

    for required in REQUIRED_ACCEPTANCE_GOLDENS {
        if surface.golden_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingAcceptanceGolden,
                format!("golden:{required}"),
                "required acceptance golden missing",
            ));
        }
    }
    for required in REQUIRED_CHALLENGE_FIXTURES {
        if surface.fixture_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingChallengeFixture,
                format!("fixture:{required}"),
                "required challenge fixture missing",
            ));
        }
    }
    for required in REQUIRED_ACCEPTANCE_PROOFS {
        if surface.proof_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingAcceptanceProof,
                format!("proof:{required}"),
                "required acceptance proof missing",
            ));
        }
    }

    for golden in &surface.goldens {
        validate_golden(golden, &mut errors);
    }
    for fixture in &surface.fixtures {
        validate_fixture(fixture, &mut errors);
    }

    let golden_ids: BTreeSet<String> = surface.goldens.iter().map(|item| item.id.clone()).collect();
    let fixture_ids: BTreeSet<String> = surface
        .fixtures
        .iter()
        .map(|item| item.id.clone())
        .collect();
    for proof in &surface.proofs {
        validate_proof(proof, &golden_ids, &fixture_ids, &mut errors);
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_golden(golden: &AcceptanceGolden, errors: &mut Vec<ValidationError>) {
    let location = golden.canonical_identity();
    if !ALLOWED_GOLDEN_KINDS.contains(&golden.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAcceptanceGolden,
            location.clone(),
            format!("unsupported golden kind {}", golden.kind),
        ));
    }
    if !golden.path.starts_with("goldens/p00/") || !golden.path.ends_with(".receipt") {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAcceptanceGolden,
            location.clone(),
            format!("golden path must be goldens/p00/*.receipt: {}", golden.path),
        ));
    }
    if !is_p00_task(&golden.source_task) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAcceptanceGolden,
            location.clone(),
            format!("invalid golden source task {}", golden.source_task),
        ));
    }
    if !golden.receipt.starts_with("receipts/p00/") || !golden.receipt.ends_with(".receipt") {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "golden must bind canonical P00 receipt",
        ));
    }
    if !golden.hash.starts_with("fnv1a128:") && !golden.hash.starts_with("sha256:") {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAcceptanceGolden,
            location.clone(),
            "golden hash must be explicitly labeled",
        ));
    }
    if golden.status != "accepted" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAcceptanceGolden,
            location,
            "golden status must be accepted",
        ));
    }
}

fn validate_fixture(fixture: &ChallengeFixture, errors: &mut Vec<ValidationError>) {
    let location = fixture.canonical_identity();
    if !ALLOWED_FIXTURE_KINDS.contains(&fixture.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeFixture,
            location.clone(),
            format!("unsupported fixture kind {}", fixture.kind),
        ));
    }
    if !fixture.path.starts_with("fixtures/p00/") || !fixture.path.ends_with(".lyra") {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeFixture,
            location.clone(),
            format!("fixture path must be fixtures/p00/*.lyra: {}", fixture.path),
        ));
    }
    if !is_p00_task(&fixture.task) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeFixture,
            location.clone(),
            format!("invalid fixture task {}", fixture.task),
        ));
    }
    if fixture.expects == "accepted" || fixture.expects == "none" || fixture.expects == "unknown" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeFixture,
            location.clone(),
            "challenge fixture must expect a concrete rejection code",
        ));
    }
    if !fixture.receipt.starts_with("receipts/p00/") || !fixture.receipt.ends_with(".receipt") {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "challenge fixture must bind canonical P00 receipt",
        ));
    }
    if fixture.status != "rejected" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidChallengeFixture,
            location,
            "challenge fixture status must be rejected",
        ));
    }
}

fn validate_proof(
    proof: &AcceptanceProof,
    golden_ids: &BTreeSet<String>,
    fixture_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = proof.canonical_identity();
    if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAcceptanceProof,
            location.clone(),
            format!("unsupported proof scope {}", proof.scope),
        ));
    }
    if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAcceptanceProof,
            location.clone(),
            format!("unsupported proof status {}", proof.status),
        ));
    }
    if proof.scope == "phase" && proof.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "phase acceptance proof must remain blocked until all P00 tasks close",
        ));
    }
    if proof.tasks.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidAcceptanceProof,
            location.clone(),
            "proof must bind at least one task",
        ));
    }
    for task in &proof.tasks {
        if !EXECUTED_TASKS.contains(&task.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidAcceptanceProof,
                location.clone(),
                format!("unknown or unexecuted task binding {task}"),
            ));
        }
    }
    for golden in &proof.goldens {
        if !golden_ids.contains(golden) {
            errors.push(ValidationError::reject(
                ErrorCode::AcceptanceProofUnbound,
                location.clone(),
                format!("unknown golden binding {golden}"),
            ));
        }
    }
    for fixture in &proof.fixtures {
        if !fixture_ids.contains(fixture) {
            errors.push(ValidationError::reject(
                ErrorCode::AcceptanceProofUnbound,
                location.clone(),
                format!("unknown fixture binding {fixture}"),
            ));
        }
    }
    if proof.receipts.is_empty()
        || proof
            .receipts
            .iter()
            .any(|receipt| !receipt.starts_with("receipts/p00/") || !receipt.ends_with(".receipt"))
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "acceptance proof must bind canonical P00 receipts",
        ));
    }
    if proof.commands.is_empty() || proof.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "acceptance proof must bind command records",
        ));
    }
    for forbid in &proof.forbids {
        if weak_value(forbid) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidAcceptanceProof,
                location.clone(),
                "forbid entries must be concrete",
            ));
        }
    }
    if proof.id == "constitutional_goldens" {
        for required in REQUIRED_ACCEPTANCE_GOLDENS {
            if !proof.goldens.iter().any(|value| value == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingAcceptanceGolden,
                    location.clone(),
                    format!("constitutional goldens proof misses {required}"),
                ));
            }
        }
    }
    if proof.id == "challenge_fixture_execution" {
        for required in REQUIRED_CHALLENGE_FIXTURES {
            if !proof.fixtures.iter().any(|value| value == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingChallengeFixture,
                    location.clone(),
                    format!("challenge fixture proof misses {required}"),
                ));
            }
        }
    }
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

/// Parse `commands` so explicit `none` / `nothing` yields an empty list and surfaces as `MissingCommandRecord` during proof validation rather than parse syntax error.
fn required_commands_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
) -> Result<Vec<String>, ValidationError> {
    let value = required_string_field(line_number, fields, "commands")?;
    let trimmed = value.trim();
    if trimmed == "none" || trimmed == "nothing" {
        return Ok(Vec::new());
    }
    let values = split_list(trimmed);
    if values.is_empty() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "field commands must contain at least one item",
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

fn is_p00_task(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 7
        && &bytes[0..4] == b"P00-"
        && bytes[4].is_ascii_digit()
        && bytes[5].is_ascii_digit()
        && bytes[6].is_ascii_digit()
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
    )
}
