use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_formal_semantics_model::{
    FormalSemanticsSurface, InvariantBinding, SemanticDomain, SemanticProof, SemanticRuleBinding,
    TransitionLaw,
};

pub const P00_FORMAL_SEMANTICS_CONTRACT: &str = "LYRA-P00-FORMAL-SEMANTICS v1";

pub const REQUIRED_FORMAL_SEMANTIC_RULES: &[&str] = &[
    "canonical_interpretation_required",
    "deterministic_transition_required",
    "explicit_input_output_required",
    "receipt_bound_semantics_required",
    "constitutional_invariant_required",
    "people_first_invariant_required",
    "rollback_semantics_required",
    "no_probabilistic_truth_semantics",
];

pub const REQUIRED_SEMANTIC_DOMAINS: &[&str] = &[
    "constitution",
    "authority",
    "identity",
    "evidence",
    "challenge",
    "control",
    "public_interest",
    "canon",
];

pub const REQUIRED_SEMANTIC_RULE_BINDINGS: &[&str] = &[
    "canonical_surface_interpretation",
    "verdict_receipt_determinism",
    "authority_precedence_interpretation",
    "task_identity_interpretation",
    "evidence_closure_interpretation",
    "challenge_rollback_interpretation",
    "people_first_interpretation",
    "canon_compliance_interpretation",
];

pub const REQUIRED_TRANSITION_LAWS: &[&str] = &[
    "surface_to_parsed_model",
    "parsed_model_to_verdict",
    "verdict_to_receipt",
    "challenge_to_blocker",
    "rollback_to_truth_snapshot",
];

pub const REQUIRED_INVARIANT_BINDINGS: &[&str] = &[
    "no_ambient_randomness",
    "no_ambient_network_truth",
    "no_probabilistic_truth",
    "no_placeholder_semantics",
    "no_fake_closure_semantics",
    "people_first_floor",
];

pub const REQUIRED_SEMANTIC_PROOFS: &[&str] = &[
    "formal_semantics_local",
    "constitutional_law_driver",
    "rebuild_governance_driver",
    "p00_phase_open",
];

const ALLOWED_DOMAIN_STATUSES: &[&str] = &["admitted", "working_slice", "execution_proven"];
const ALLOWED_RULE_KINDS: &[&str] = &[
    "interpretation",
    "transition",
    "invariant",
    "closure",
    "rollback",
    "rebuild",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["task", "constitution", "governance", "phase"];
const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "execution_proven",
    "artifact_emitted",
    "blocked",
];
const SEMANTIC_OWNER_ROOTS: &[&str] = &["ops", "interfaces", "k0"];
const EXECUTED_TASKS: &[&str] = &[
    "P00-001", "P00-002", "P00-003", "P00-004", "P00-005", "P00-006", "P00-007", "P00-008",
    "P00-009", "P00-010", "P00-011", "P00-012", "P00-013",
];

const FORBIDDEN_SEMANTIC_TEXT: &[(&str, ErrorCode)] = &[
    ("probabilistic truth", ErrorCode::SemanticDriftAccepted),
    ("stochastic semantics", ErrorCode::SemanticDriftAccepted),
    ("random semantic choice", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::SemanticDriftAccepted),
    ("network authority", ErrorCode::AmbientNetworkAllowed),
    ("placeholder semantics", ErrorCode::PlaceholderAllowed),
    ("semantic todo", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("manual proof only", ErrorCode::InvalidSemanticProof),
];

pub fn parse_formal_semantics_surface(
    input: &str,
) -> Result<FormalSemanticsSurface, Vec<ValidationError>> {
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
            "no formal-semantics lines",
        )]);
    }

    let header = lines[0].clone();
    if header != P00_FORMAL_SEMANTICS_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_FORMAL_SEMANTICS_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut domains = Vec::new();
    let mut semantic_rules = Vec::new();
    let mut transitions = Vec::new();
    let mut invariants = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_domains = BTreeSet::new();
    let mut seen_semantic_rules = BTreeSet::new();
    let mut seen_transitions = BTreeSet::new();
    let mut seen_invariants = BTreeSet::new();
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
                    "formal semantic rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }

        if let Some(domain_id) = left.strip_prefix("domain:") {
            if !is_symbolic_name(domain_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSemanticDomain,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic domain identity {domain_id}"),
                ));
                continue;
            }
            if !seen_domains.insert(domain_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateSemanticDomain,
                    format!("domain:{domain_id}"),
                    "semantic domain identity must be unique",
                ));
                continue;
            }
            match parse_domain(line_number, domain_id, value) {
                Ok(item) => domains.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(binding_id) = left.strip_prefix("semantic_rule:") {
            if !is_symbolic_name(binding_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSemanticRuleBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic rule identity {binding_id}"),
                ));
                continue;
            }
            if !seen_semantic_rules.insert(binding_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateSemanticRuleBinding,
                    format!("semantic_rule:{binding_id}"),
                    "semantic rule identity must be unique",
                ));
                continue;
            }
            match parse_semantic_rule(line_number, binding_id, value) {
                Ok(item) => semantic_rules.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(transition_id) = left.strip_prefix("transition:") {
            if !is_symbolic_name(transition_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidTransitionLaw,
                    format!("line:{line_number:03}"),
                    format!("invalid transition law identity {transition_id}"),
                ));
                continue;
            }
            if !seen_transitions.insert(transition_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateTransitionLaw,
                    format!("transition:{transition_id}"),
                    "transition law identity must be unique",
                ));
                continue;
            }
            match parse_transition(line_number, transition_id, value) {
                Ok(item) => transitions.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(invariant_id) = left.strip_prefix("invariant:") {
            if !is_symbolic_name(invariant_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInvariantBinding,
                    format!("line:{line_number:03}"),
                    format!("invalid invariant identity {invariant_id}"),
                ));
                continue;
            }
            if !seen_invariants.insert(invariant_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInvariantBinding,
                    format!("invariant:{invariant_id}"),
                    "invariant identity must be unique",
                ));
                continue;
            }
            match parse_invariant(line_number, invariant_id, value) {
                Ok(item) => invariants.push(item),
                Err(error) => errors.push(error),
            }
            continue;
        }

        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSemanticProof,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateSemanticProof,
                    format!("proof:{proof_id}"),
                    "semantic proof identity must be unique",
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
                format!("unknown formal-semantics field {left}"),
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
                "task=P00-013 is required",
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
        Ok(FormalSemanticsSurface {
            header,
            phase,
            task,
            status,
            rules,
            domains,
            semantic_rules,
            transitions,
            invariants,
            proofs,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_formal_semantics_surface(input: &str) -> (Verdict, Receipt) {
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

    let verdict = match parse_formal_semantics_surface(input) {
        Ok(surface) => validate_parsed_formal_semantics_surface(&surface, input),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_receipt(input, &canonical_text, verdict.clone());
    (verdict, receipt)
}

fn parse_domain(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticDomain, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let owner_root = required_string_field(line_number, &mut fields, "owner_root")?;
    let source_task = required_string_field(line_number, &mut fields, "source_task")?;
    let contract = required_string_field(line_number, &mut fields, "contract")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(SemanticDomain {
        line_number,
        id: id.to_string(),
        owner_root,
        source_task,
        contract,
        status,
    })
}

fn parse_semantic_rule(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticRuleBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let domain = required_string_field(line_number, &mut fields, "domain")?;
    let kind = required_string_field(line_number, &mut fields, "kind")?;
    let input = required_string_field(line_number, &mut fields, "input")?;
    let output = required_string_field(line_number, &mut fields, "output")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    let receipt = required_string_field(line_number, &mut fields, "receipt")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(SemanticRuleBinding {
        line_number,
        id: id.to_string(),
        domain,
        kind,
        input,
        output,
        forbids,
        receipt,
        status,
    })
}

fn parse_transition(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<TransitionLaw, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let from = required_string_field(line_number, &mut fields, "from")?;
    let to = required_string_field(line_number, &mut fields, "to")?;
    let guard = required_string_field(line_number, &mut fields, "guard")?;
    let receipt = required_string_field(line_number, &mut fields, "receipt")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(TransitionLaw {
        line_number,
        id: id.to_string(),
        from,
        to,
        guard,
        receipt,
        status,
    })
}

fn parse_invariant(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<InvariantBinding, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let domain = required_string_field(line_number, &mut fields, "domain")?;
    let invariant = required_string_field(line_number, &mut fields, "invariant")?;
    let rejects = required_list_field(line_number, &mut fields, "rejects")?;
    let receipt = required_string_field(line_number, &mut fields, "receipt")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(InvariantBinding {
        line_number,
        id: id.to_string(),
        domain,
        invariant,
        rejects,
        receipt,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticProof, ValidationError> {
    let mut fields = parse_fields(line_number, value)?;
    let scope = required_string_field(line_number, &mut fields, "scope")?;
    let domains = required_list_field(line_number, &mut fields, "domains")?;
    let rules = required_list_field(line_number, &mut fields, "rules")?;
    let transitions = required_list_field(line_number, &mut fields, "transitions")?;
    let invariants = required_list_field(line_number, &mut fields, "invariants")?;
    let receipts = required_proof_receipt_list_field(line_number, &mut fields)?;
    let commands = required_list_field(line_number, &mut fields, "commands")?;
    let status = required_string_field(line_number, &mut fields, "status")?;
    let forbids = required_list_field(line_number, &mut fields, "forbids")?;
    reject_unknown_fields(line_number, fields)?;
    Ok(SemanticProof {
        line_number,
        id: id.to_string(),
        scope,
        domains,
        rules,
        transitions,
        invariants,
        receipts,
        commands,
        status,
        forbids,
    })
}

fn validate_parsed_formal_semantics_surface(
    surface: &FormalSemanticsSurface,
    raw_input: &str,
) -> Verdict {
    let mut errors = Vec::new();

    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "field:phase",
            "formal semantics law is scoped to P00",
        ));
    }
    if surface.task != "P00-013" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "field:task",
            "formal semantics law must bind P00-013",
        ));
    }
    if surface.status != "working_slice" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "field:status",
            "P00-013 may only claim working_slice in this pass",
        ));
    }

    let lowered = raw_input.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_SEMANTIC_TEXT {
        if lowered.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "formal_semantics:text",
                format!("forbidden formal semantics phrase detected: {token}"),
            ));
        }
    }

    for required in REQUIRED_FORMAL_SEMANTIC_RULES {
        match surface.rule_value(required) {
            Some(value) if value.starts_with("required:") || value.starts_with("forbidden:") => {}
            Some(_) => errors.push(ValidationError::reject(
                ErrorCode::MissingFormalSemanticRule,
                format!("rule:{required}"),
                "formal semantic rule must be explicit required: or forbidden:",
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingFormalSemanticRule,
                format!("rule:{required}"),
                "required formal semantic rule missing",
            )),
        }
    }

    for required in REQUIRED_SEMANTIC_DOMAINS {
        if surface.domain_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingSemanticDomain,
                format!("domain:{required}"),
                "required semantic domain missing",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_RULE_BINDINGS {
        if surface.semantic_rule_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingSemanticRuleBinding,
                format!("semantic_rule:{required}"),
                "required semantic rule binding missing",
            ));
        }
    }
    for required in REQUIRED_TRANSITION_LAWS {
        if surface.transition_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingTransitionLaw,
                format!("transition:{required}"),
                "required transition law missing",
            ));
        }
    }
    for required in REQUIRED_INVARIANT_BINDINGS {
        if surface.invariant_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingInvariantBinding,
                format!("invariant:{required}"),
                "required invariant binding missing",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_PROOFS {
        if surface.proof_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingSemanticProof,
                format!("proof:{required}"),
                "required semantic proof missing",
            ));
        }
    }

    let domain_ids: BTreeSet<String> = surface.domains.iter().map(|item| item.id.clone()).collect();
    let semantic_rule_ids: BTreeSet<String> = surface
        .semantic_rules
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let transition_ids: BTreeSet<String> = surface
        .transitions
        .iter()
        .map(|item| item.id.clone())
        .collect();
    let invariant_ids: BTreeSet<String> = surface
        .invariants
        .iter()
        .map(|item| item.id.clone())
        .collect();

    for domain in &surface.domains {
        validate_domain(domain, &mut errors);
    }
    for semantic_rule in &surface.semantic_rules {
        validate_semantic_rule(semantic_rule, &domain_ids, &mut errors);
    }
    for transition in &surface.transitions {
        validate_transition(transition, &mut errors);
    }
    for invariant in &surface.invariants {
        validate_invariant(invariant, &domain_ids, &mut errors);
    }
    for proof in &surface.proofs {
        validate_proof(
            proof,
            &domain_ids,
            &semantic_rule_ids,
            &transition_ids,
            &invariant_ids,
            &mut errors,
        );
    }

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_domain(domain: &SemanticDomain, errors: &mut Vec<ValidationError>) {
    let location = domain.canonical_identity();
    if !SEMANTIC_OWNER_ROOTS.contains(&domain.owner_root.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticDomain,
            location.clone(),
            format!(
                "semantic domain owner root must be ops/interfaces/k0: {}",
                domain.owner_root
            ),
        ));
    }
    if !EXECUTED_TASKS.contains(&domain.source_task.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticDomain,
            location.clone(),
            format!(
                "semantic domain source task is not in executed P00 chain: {}",
                domain.source_task
            ),
        ));
    }
    if !domain.contract.starts_with("interfaces/p00/contracts/")
        || !domain.contract.ends_with(".v1.lyra")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticDomain,
            location.clone(),
            format!(
                "domain contract must bind interfaces/p00/contracts/*.v1.lyra: {}",
                domain.contract
            ),
        ));
    }
    if !ALLOWED_DOMAIN_STATUSES.contains(&domain.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticDomain,
            location,
            format!("unsupported semantic domain status {}", domain.status),
        ));
    }
}

fn validate_semantic_rule(
    rule: &SemanticRuleBinding,
    domain_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = rule.canonical_identity();
    if !domain_ids.contains(&rule.domain) {
        errors.push(ValidationError::reject(
            ErrorCode::SemanticProofUnbound,
            location.clone(),
            format!("semantic rule references unknown domain {}", rule.domain),
        ));
    }
    if !ALLOWED_RULE_KINDS.contains(&rule.kind.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticRuleBinding,
            location.clone(),
            format!("unsupported semantic rule kind {}", rule.kind),
        ));
    }
    if weak_value(&rule.input) || weak_value(&rule.output) || rule.input == rule.output {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticRuleBinding,
            location.clone(),
            "semantic rule must bind concrete and distinct input/output symbols",
        ));
    }
    if rule.forbids.iter().any(|item| weak_value(item)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticRuleBinding,
            location.clone(),
            "semantic rule forbid list must be concrete",
        ));
    }
    if !receipt_path(&rule.receipt) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "semantic rule must bind canonical P00 receipt",
        ));
    }
    if !ALLOWED_STATUSES.contains(&rule.status.as_str()) || rule.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticRuleBinding,
            location,
            format!(
                "semantic rule status must be executable, got {}",
                rule.status
            ),
        ));
    }
}

fn validate_transition(transition: &TransitionLaw, errors: &mut Vec<ValidationError>) {
    let location = transition.canonical_identity();
    if weak_value(&transition.from)
        || weak_value(&transition.to)
        || weak_value(&transition.guard)
        || transition.from == transition.to
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTransitionLaw,
            location.clone(),
            "transition must bind concrete from/to/guard symbols",
        ));
    }
    if transition.guard.contains("ambient")
        || transition.guard.contains("random")
        || transition.guard.contains("probabilistic")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTransitionLaw,
            location.clone(),
            format!(
                "transition guard admits nondeterminism: {}",
                transition.guard
            ),
        ));
    }
    if !receipt_path(&transition.receipt) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "transition law must bind canonical P00 receipt",
        ));
    }
    if !ALLOWED_STATUSES.contains(&transition.status.as_str()) || transition.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTransitionLaw,
            location,
            format!(
                "transition law status must be executable, got {}",
                transition.status
            ),
        ));
    }
}

fn validate_invariant(
    invariant: &InvariantBinding,
    domain_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = invariant.canonical_identity();
    if !domain_ids.contains(&invariant.domain) {
        errors.push(ValidationError::reject(
            ErrorCode::SemanticProofUnbound,
            location.clone(),
            format!("invariant references unknown domain {}", invariant.domain),
        ));
    }
    if weak_value(&invariant.invariant) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidInvariantBinding,
            location.clone(),
            "invariant must be concrete",
        ));
    }
    if invariant.rejects.is_empty() || invariant.rejects.iter().any(|item| weak_value(item)) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidInvariantBinding,
            location.clone(),
            "invariant must bind concrete rejection codes",
        ));
    }
    if !receipt_path(&invariant.receipt) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "invariant must bind canonical P00 receipt",
        ));
    }
    if !ALLOWED_STATUSES.contains(&invariant.status.as_str()) || invariant.status == "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidInvariantBinding,
            location,
            format!(
                "invariant status must be executable, got {}",
                invariant.status
            ),
        ));
    }
}

fn validate_proof(
    proof: &SemanticProof,
    domain_ids: &BTreeSet<String>,
    semantic_rule_ids: &BTreeSet<String>,
    transition_ids: &BTreeSet<String>,
    invariant_ids: &BTreeSet<String>,
    errors: &mut Vec<ValidationError>,
) {
    let location = proof.canonical_identity();
    if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticProof,
            location.clone(),
            format!("unsupported semantic proof scope {}", proof.scope),
        ));
    }
    if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidSemanticProof,
            location.clone(),
            format!("unsupported semantic proof status {}", proof.status),
        ));
    }
    if proof.scope == "phase" && proof.status != "blocked" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            location.clone(),
            "phase semantic proof must remain blocked until all P00 tasks close",
        ));
    }
    for domain in &proof.domains {
        if !domain_ids.contains(domain) {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticProofUnbound,
                location.clone(),
                format!("unknown semantic proof domain {domain}"),
            ));
        }
    }
    for rule in &proof.rules {
        if !semantic_rule_ids.contains(rule) {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticProofUnbound,
                location.clone(),
                format!("unknown semantic rule proof binding {rule}"),
            ));
        }
    }
    for transition in &proof.transitions {
        if !transition_ids.contains(transition) {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticProofUnbound,
                location.clone(),
                format!("unknown transition proof binding {transition}"),
            ));
        }
    }
    for invariant in &proof.invariants {
        if !invariant_ids.contains(invariant) {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticProofUnbound,
                location.clone(),
                format!("unknown invariant proof binding {invariant}"),
            ));
        }
    }
    if proof.receipts.is_empty() || proof.receipts.iter().any(|receipt| !receipt_path(receipt)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            location.clone(),
            "semantic proof must bind canonical P00 receipts",
        ));
    }
    if proof.commands.is_empty() || proof.commands.iter().any(|command| weak_value(command)) {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCommandRecord,
            location.clone(),
            "semantic proof must bind command records",
        ));
    }
    for forbid in &proof.forbids {
        if weak_value(forbid) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticProof,
                location.clone(),
                "forbid entries must be concrete",
            ));
        }
    }
    if proof.id == "constitutional_law_driver" {
        for required in [
            "constitution",
            "authority",
            "identity",
            "evidence",
            "public_interest",
        ] {
            if !proof.domains.iter().any(|value| value == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingSemanticDomain,
                    location.clone(),
                    format!("constitutional driver misses domain {required}"),
                ));
            }
        }
    }
    if proof.id == "rebuild_governance_driver" {
        for required in [
            "surface_to_parsed_model",
            "parsed_model_to_verdict",
            "verdict_to_receipt",
            "rollback_to_truth_snapshot",
        ] {
            if !proof.transitions.iter().any(|value| value == required) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingTransitionLaw,
                    location.clone(),
                    format!("rebuild governance driver misses transition {required}"),
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

fn required_proof_receipt_list_field(
    line_number: usize,
    fields: &mut BTreeMap<String, String>,
) -> Result<Vec<String>, ValidationError> {
    let value = required_string_field(line_number, fields, "receipts")?;
    let trimmed = value.trim();
    if trimmed == "none" || trimmed == "nothing" {
        return Ok(Vec::new());
    }
    let values = split_list(trimmed);
    if values.is_empty() {
        Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{line_number:03}"),
            "field receipts must contain at least one item",
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

fn receipt_path(value: &str) -> bool {
    value.starts_with("receipts/p00/") && value.ends_with(".receipt")
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
    )
}
