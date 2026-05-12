use crate::k0_bootstrap_formal_semantics::deterministic_bootstrap_formal_semantics_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_bootstrap_formal_semantics_model::{
    BootstrapConstitutionalLawBinding, BootstrapFormalDomainBinding,
    BootstrapFormalInvariantBinding, BootstrapFormalProofBinding, BootstrapFormalReceiptBinding,
    BootstrapFormalSemanticsSurface, BootstrapFormalTransitionBinding,
};
use std::collections::{BTreeMap, BTreeSet};

pub const P02_BOOTSTRAP_FORMAL_SEMANTICS_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-FORMAL-SEMANTICS v1";
pub const REQUIRED_BOOTSTRAP_FORMAL_RULES: &[&str] = &[
    "bootstrap_truth_must_have_formal_semantics",
    "seed_runtime_law_must_be_constitutional",
    "host_extinction_framework_must_be_formally_guarded",
    "foreign_surface_truth_requires_local_challenge",
    "operator_handoff_must_not_promote_truth",
    "emergency_fallback_freezes_before_recovery",
    "every_transition_requires_receipt_gate",
    "every_domain_requires_constitutional_binding",
    "no_probabilistic_bootstrap_semantics",
    "no_network_bootstrap_truth",
    "no_hidden_randomness_bootstrap_semantics",
    "no_ambient_time_bootstrap_semantics",
    "no_placeholder_bootstrap_semantics",
    "no_global_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_FORMAL_DOMAINS: &[&str] = &[
    "bootstrap_trust",
    "seed_runtime_law",
    "host_extinction_framework",
    "foreign_surface_boundary",
    "operator_handoff_truth",
    "emergency_fallback_safety",
];
pub const REQUIRED_BOOTSTRAP_CONSTITUTIONAL_LAWS: &[&str] = &[
    "law_bootstrap_trust_receipt_only",
    "law_seed_runtime_no_silent_ownership",
    "law_host_extinction_no_global_claim",
    "law_foreign_surface_challenge_first",
    "law_operator_handoff_no_truth_drift",
    "law_emergency_fallback_freeze_before_advance",
    "law_no_probabilistic_bootstrap_semantics",
    "law_local_validation_blocks_phase_closure",
];
pub const REQUIRED_BOOTSTRAP_FORMAL_TRANSITIONS: &[&str] = &[
    "transition_inventory_to_trust_floor",
    "transition_trust_floor_to_seed_runtime",
    "transition_seed_runtime_to_host_extinction",
    "transition_host_extinction_to_operator_capture",
    "transition_operator_capture_to_foreign_closure",
    "transition_foreign_closure_to_phase_open",
];
pub const REQUIRED_BOOTSTRAP_FORMAL_INVARIANTS: &[&str] = &[
    "invariant_receipt_before_truth",
    "invariant_no_probabilistic_semantics",
    "invariant_no_hidden_randomness",
    "invariant_no_ambient_time",
    "invariant_no_network_truth",
    "invariant_no_global_closure",
    "invariant_operator_capture_truth_neutral",
    "invariant_fallback_freezes_before_recovery",
];
pub const REQUIRED_BOOTSTRAP_FORMAL_PROOFS: &[&str] = &[
    "proof_bootstrap_trust_semantics",
    "proof_seed_runtime_law_semantics",
    "proof_host_extinction_semantics",
    "proof_operator_handoff_semantics",
    "proof_p02_phase_open_not_closed",
];
pub const REQUIRED_BOOTSTRAP_FORMAL_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_emergency_fallback",
    "receipt_seed_runtime_replacement",
    "receipt_bootstrap_evidence_emission",
    "receipt_operator_handoff_automation",
    "receipt_foreign_surface_closure",
    "receipt_bootstrap_formal_semantics",
];
pub const REQUIRED_BOOTSTRAP_FORMAL_OWNER_ROOTS: &[&str] =
    &["interfaces", "k0", "lyralang", "ops", "products", "shells"];
const ALLOWED_STATUS: &[&str] = &["bootstrap_formal_semantics_artifact_emitted"];
const ALLOWED_DOMAIN_STATUS: &[&str] = &["semantic_domain_bound"];
const ALLOWED_LAW_STATUS: &[&str] = &["constitutional_law_bound"];
const ALLOWED_TRANSITION_STATUS: &[&str] = &["transition_law_bound"];
const ALLOWED_INVARIANT_STATUS: &[&str] = &["invariant_bound"];
const ALLOWED_PROOF_STATUS: &[&str] = &["formal_proof_bound"];
const ALLOWED_RECEIPT_STATUS: &[&str] = &["receipt_bound"];
const ALLOWED_LAW_CLASSES: &[&str] = &[
    "trust_law",
    "runtime_law",
    "closure_law",
    "boundary_law",
    "handoff_law",
    "safety_law",
    "invariant_law",
    "validation_law",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &[
    "bootstrap_trust",
    "seed_runtime",
    "host_extinction",
    "operator_handoff",
    "phase_open",
];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    ("stochastic semantics", ErrorCode::ProbabilisticTruthAllowed),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("placeholder semantics", ErrorCode::PlaceholderAllowed),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("foreign truth accepted", ErrorCode::ClosureDriftAccepted),
    (
        "operator override constitution",
        ErrorCode::OperatorOverrideConstitution,
    ),
];

pub fn parse_bootstrap_formal_semantics_surface(
    input: &str,
) -> Result<BootstrapFormalSemanticsSurface, Vec<ValidationError>> {
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
            "empty bootstrap formal semantics surface",
        )]);
    }
    if lines[0] != P02_BOOTSTRAP_FORMAL_SEMANTICS_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_FORMAL_SEMANTICS_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut previous_evidence_receipt = None;
    let mut rules = BTreeMap::new();
    let mut domains = Vec::new();
    let mut laws = Vec::new();
    let mut transitions = Vec::new();
    let mut invariants = Vec::new();
    let mut proofs = Vec::new();
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
        let key = left.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() || key != left || value != value.trim() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "entries must be trimmed and non-empty",
            ));
            continue;
        }
        if !seen.insert(key.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{n:03}"),
                key,
            ));
            continue;
        }
        if key == "phase" {
            phase = Some(value.to_string());
            continue;
        }
        if key == "task" {
            task = Some(value.to_string());
            continue;
        }
        if key == "status" {
            status = Some(value.to_string());
            continue;
        }
        if key == "previous_evidence_receipt" {
            previous_evidence_receipt = Some(value.to_string());
            continue;
        }
        if let Some(id) = bracket_id(key, "rule") {
            rules.insert(id.to_string(), value.to_string());
            continue;
        }
        if let Some(id) = bracket_id(key, "domain") {
            let fields = split_fields(value);
            if fields.len() != 5 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSemanticDomain,
                    format!("line:{n:03}"),
                    "domain row requires 5 fields",
                ));
                continue;
            }
            domains.push(BootstrapFormalDomainBinding {
                line_number: n,
                id: id.to_string(),
                owner_root: fields[0].clone(),
                source_task: fields[1].clone(),
                semantic_object: fields[2].clone(),
                constitutional_binding: fields[3].clone(),
                status: fields[4].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "law") {
            let fields = split_fields(value);
            if fields.len() != 6 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSemanticRuleBinding,
                    format!("line:{n:03}"),
                    "law row requires 6 fields",
                ));
                continue;
            }
            laws.push(BootstrapConstitutionalLawBinding {
                line_number: n,
                id: id.to_string(),
                domain_id: fields[0].clone(),
                law_class: fields[1].clone(),
                governs: fields[2].clone(),
                forbids: split_list(&fields[3]),
                requires_receipt: fields[4].clone(),
                status: fields[5].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "transition") {
            let fields = split_fields(value);
            if fields.len() != 5 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidTransitionLaw,
                    format!("line:{n:03}"),
                    "transition row requires 5 fields",
                ));
                continue;
            }
            transitions.push(BootstrapFormalTransitionBinding {
                line_number: n,
                id: id.to_string(),
                from_state: fields[0].clone(),
                to_state: fields[1].clone(),
                guard: fields[2].clone(),
                receipt: fields[3].clone(),
                status: fields[4].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "invariant") {
            let fields = split_fields(value);
            if fields.len() != 5 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInvariantBinding,
                    format!("line:{n:03}"),
                    "invariant row requires 5 fields",
                ));
                continue;
            }
            invariants.push(BootstrapFormalInvariantBinding {
                line_number: n,
                id: id.to_string(),
                domain_id: fields[0].clone(),
                assertion: fields[1].clone(),
                rejects: split_list(&fields[2]),
                receipt: fields[3].clone(),
                status: fields[4].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "proof") {
            let fields = split_fields(value);
            if fields.len() != 7 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidSemanticProof,
                    format!("line:{n:03}"),
                    "proof row requires 7 fields",
                ));
                continue;
            }
            proofs.push(BootstrapFormalProofBinding {
                line_number: n,
                id: id.to_string(),
                scope: fields[0].clone(),
                domains: split_list(&fields[1]),
                laws: split_list(&fields[2]),
                transitions: split_list(&fields[3]),
                invariants: split_list(&fields[4]),
                receipts: split_list(&fields[5]),
                status: fields[6].clone(),
            });
            continue;
        }
        if let Some(id) = bracket_id(key, "receipt") {
            let fields = split_fields(value);
            if fields.len() != 3 {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidReceiptChainBinding,
                    format!("line:{n:03}"),
                    "receipt row requires 3 fields",
                ));
                continue;
            }
            receipts.push(BootstrapFormalReceiptBinding {
                line_number: n,
                id: id.to_string(),
                path: fields[0].clone(),
                binds: fields[1].clone(),
                status: fields[2].clone(),
            });
            continue;
        }
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{n:03}"),
            format!("unknown key {key}"),
        ));
    }
    let phase = require_scalar(phase, ErrorCode::MissingPhase, "phase", &mut errors);
    let task = require_scalar(task, ErrorCode::MissingTask, "task", &mut errors);
    let status = require_scalar(
        status,
        ErrorCode::UnsupportedEvidenceClaim,
        "status",
        &mut errors,
    );
    let previous_evidence_receipt = require_scalar(
        previous_evidence_receipt,
        ErrorCode::MissingReceiptProof,
        "previous_evidence_receipt",
        &mut errors,
    );
    if errors.is_empty() {
        Ok(BootstrapFormalSemanticsSurface {
            header: P02_BOOTSTRAP_FORMAL_SEMANTICS_CONTRACT.to_string(),
            phase,
            task,
            status,
            previous_evidence_receipt,
            rules,
            domains,
            laws,
            transitions,
            invariants,
            proofs,
            receipts,
        })
    } else {
        Err(errors)
    }
}

pub fn validate_bootstrap_formal_semantics_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let verdict = match parse_bootstrap_formal_semantics_surface(input) {
        Ok(surface) => validate_bootstrap_formal_semantics_model(&surface),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_formal_semantics_model(
    surface: &BootstrapFormalSemanticsSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "expected P02",
        ));
    }
    if surface.task != "P02-013" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "expected P02-013",
        ));
    }
    if !ALLOWED_STATUS.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            "status",
            surface.status.as_str(),
        ));
    }
    if !surface
        .previous_evidence_receipt
        .starts_with("receipts/p02/")
    {
        errors.push(ValidationError::reject(
            ErrorCode::UnknownEvidencePath,
            "previous_evidence_receipt",
            surface.previous_evidence_receipt.as_str(),
        ));
    }
    scan_forbidden(surface, &mut errors);
    require_rules(&surface.rules, &mut errors);
    require_named(
        REQUIRED_BOOTSTRAP_FORMAL_DOMAINS,
        surface.domains.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingSemanticDomain,
        "domain",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_CONSTITUTIONAL_LAWS,
        surface.laws.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingSemanticRuleBinding,
        "law",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_FORMAL_TRANSITIONS,
        surface.transitions.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingTransitionLaw,
        "transition",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_FORMAL_INVARIANTS,
        surface.invariants.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingInvariantBinding,
        "invariant",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_FORMAL_PROOFS,
        surface.proofs.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingSemanticProof,
        "proof",
        &mut errors,
    );
    require_named(
        REQUIRED_BOOTSTRAP_FORMAL_RECEIPTS,
        surface.receipts.iter().map(|x| x.id.as_str()).collect(),
        ErrorCode::MissingReceiptProof,
        "receipt",
        &mut errors,
    );

    for domain in &surface.domains {
        if !REQUIRED_BOOTSTRAP_FORMAL_OWNER_ROOTS.contains(&domain.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                domain.canonical_identity(),
                domain.owner_root.as_str(),
            ));
        }
        if !ALLOWED_DOMAIN_STATUS.contains(&domain.status.as_str())
            || domain.constitutional_binding != "constitutional_law_bound"
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticDomain,
                domain.canonical_identity(),
                "domain must be constitutionally bound",
            ));
        }
    }
    for law in &surface.laws {
        if surface.domain_by_id(&law.domain_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticRuleBinding,
                law.canonical_identity(),
                format!("unknown domain {}", law.domain_id),
            ));
        }
        if !ALLOWED_LAW_CLASSES.contains(&law.law_class.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticRuleBinding,
                law.canonical_identity(),
                law.law_class.as_str(),
            ));
        }
        if law.forbids.is_empty() || law.forbids.iter().any(|x| x == "none") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticRuleBinding,
                law.canonical_identity(),
                "law must forbid concrete unsafe states",
            ));
        }
        if !law.receipt_bound() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                law.canonical_identity(),
                law.requires_receipt.as_str(),
            ));
        }
        if !ALLOWED_LAW_STATUS.contains(&law.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticRuleBinding,
                law.canonical_identity(),
                law.status.as_str(),
            ));
        }
    }
    for transition in &surface.transitions {
        if transition.from_state == transition.to_state {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTransitionLaw,
                transition.canonical_identity(),
                "transition cannot be self-loop",
            ));
        }
        if !transition.guarded() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTransitionLaw,
                transition.canonical_identity(),
                transition.guard.as_str(),
            ));
        }
        if !transition.receipt_bound() {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticProofUnbound,
                transition.canonical_identity(),
                transition.receipt.as_str(),
            ));
        }
        if !ALLOWED_TRANSITION_STATUS.contains(&transition.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTransitionLaw,
                transition.canonical_identity(),
                transition.status.as_str(),
            ));
        }
    }
    for invariant in &surface.invariants {
        if surface.domain_by_id(&invariant.domain_id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInvariantBinding,
                invariant.canonical_identity(),
                format!("unknown domain {}", invariant.domain_id),
            ));
        }
        if invariant.rejects.is_empty() || invariant.rejects.iter().any(|x| x == "none") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInvariantBinding,
                invariant.canonical_identity(),
                "invariant must reject concrete unsafe states",
            ));
        }
        if !invariant.receipt.starts_with("receipts/p02/") {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                invariant.canonical_identity(),
                invariant.receipt.as_str(),
            ));
        }
        if !ALLOWED_INVARIANT_STATUS.contains(&invariant.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInvariantBinding,
                invariant.canonical_identity(),
                invariant.status.as_str(),
            ));
        }
    }
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticProof,
                proof.canonical_identity(),
                proof.scope.as_str(),
            ));
        }
        if proof.domains.is_empty()
            || proof.laws.is_empty()
            || proof.transitions.is_empty()
            || proof.invariants.is_empty()
            || proof.receipts.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticProofUnbound,
                proof.canonical_identity(),
                "proof must bind domains laws transitions invariants receipts",
            ));
        }
        for domain in &proof.domains {
            if surface.domain_by_id(domain).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::SemanticProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown domain {domain}"),
                ));
            }
        }
        for law in &proof.laws {
            if surface.law_by_id(law).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::SemanticProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown law {law}"),
                ));
            }
        }
        for transition in &proof.transitions {
            if surface.transition_by_id(transition).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::SemanticProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown transition {transition}"),
                ));
            }
        }
        for invariant in &proof.invariants {
            if surface.invariant_by_id(invariant).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::SemanticProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown invariant {invariant}"),
                ));
            }
        }
        if !proof.receipt_bound() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                proof.canonical_identity(),
                "proof receipts must be local p02 receipts",
            ));
        }
        if !ALLOWED_PROOF_STATUS.contains(&proof.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticProof,
                proof.canonical_identity(),
                proof.status.as_str(),
            ));
        }
    }
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p02/") {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                receipt.canonical_identity(),
                receipt.path.as_str(),
            ));
        }
        if !ALLOWED_RECEIPT_STATUS.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReceiptChainBinding,
                receipt.canonical_identity(),
                receipt.status.as_str(),
            ));
        }
    }
    require_core_invariant_coverage(surface, &mut errors);
    let report = deterministic_bootstrap_formal_semantics_report(surface);
    if report.domain_count < REQUIRED_BOOTSTRAP_FORMAL_DOMAINS.len()
        || report.law_count < REQUIRED_BOOTSTRAP_CONSTITUTIONAL_LAWS.len()
        || report.transition_count < REQUIRED_BOOTSTRAP_FORMAL_TRANSITIONS.len()
        || report.invariant_count < REQUIRED_BOOTSTRAP_FORMAL_INVARIANTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::UnderbuildViolation,
            "report",
            report.semantics_hash,
        ));
    }
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn require_scalar(
    value: Option<String>,
    code: ErrorCode,
    location: &str,
    errors: &mut Vec<ValidationError>,
) -> String {
    match value {
        Some(v) => v,
        None => {
            errors.push(ValidationError::reject(
                code,
                location,
                "required scalar missing",
            ));
            String::new()
        }
    }
}
fn bracket_id<'a>(key: &'a str, prefix: &str) -> Option<&'a str> {
    let start = format!("{prefix}[");
    if key.starts_with(&start) && key.ends_with(']') {
        Some(&key[start.len()..key.len() - 1])
    } else {
        None
    }
}
fn split_fields(value: &str) -> Vec<String> {
    value.split('|').map(|x| x.trim().to_string()).collect()
}
fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}
fn require_rules(rules: &BTreeMap<String, String>, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_BOOTSTRAP_FORMAL_RULES {
        if rules.get(*required).map(String::as_str) != Some("required") {
            errors.push(ValidationError::reject(
                ErrorCode::MissingFormalSemanticRule,
                format!("rule[{required}]"),
                "required formal bootstrap semantic rule missing",
            ));
        }
    }
}
fn require_named(
    required: &[&str],
    present: Vec<&str>,
    code: ErrorCode,
    prefix: &str,
    errors: &mut Vec<ValidationError>,
) {
    let set = present.into_iter().collect::<BTreeSet<_>>();
    for item in required {
        if !set.contains(item) {
            errors.push(ValidationError::reject(
                code,
                format!("{prefix}[{item}]"),
                "required binding missing",
            ));
        }
    }
}
fn require_core_invariant_coverage(
    surface: &BootstrapFormalSemanticsSurface,
    errors: &mut Vec<ValidationError>,
) {
    let rejects = surface
        .invariants
        .iter()
        .flat_map(|x| x.rejects.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();
    for token in [
        "probabilistic_truth",
        "hidden_randomness",
        "ambient_time",
        "network_required",
        "global_closure",
        "truth_drift",
    ] {
        if !rejects.contains(token) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInvariantBinding,
                format!("rejects:{token}"),
                "core forbidden token not covered by invariants",
            ));
        }
    }
}
fn scan_forbidden(surface: &BootstrapFormalSemanticsSurface, errors: &mut Vec<ValidationError>) {
    let mut text = format!(
        "{} {} {} {}",
        surface.phase, surface.task, surface.status, surface.previous_evidence_receipt
    );
    for value in surface.rules.values() {
        text.push(' ');
        text.push_str(value);
    }
    for domain in &surface.domains {
        text.push(' ');
        text.push_str(&domain.semantic_object);
        text.push(' ');
        text.push_str(&domain.status);
    }
    for law in &surface.laws {
        text.push(' ');
        text.push_str(&law.governs);
        text.push(' ');
        text.push_str(law.status.as_str());
    }
    for transition in &surface.transitions {
        text.push(' ');
        text.push_str(&transition.from_state);
        text.push(' ');
        text.push_str(&transition.to_state);
        text.push(' ');
        text.push_str(transition.status.as_str());
    }
    for invariant in &surface.invariants {
        text.push(' ');
        text.push_str(&invariant.assertion);
        text.push(' ');
        text.push_str(invariant.status.as_str());
    }
    for proof in &surface.proofs {
        text.push(' ');
        text.push_str(proof.scope.as_str());
        text.push(' ');
        text.push_str(proof.status.as_str());
    }
    let lower = text.to_ascii_lowercase().replace('_', " ");
    let raw_lower = text.to_ascii_lowercase();
    for (token, code) in FORBIDDEN {
        if lower.contains(token) || raw_lower.contains(token) {
            errors.push(ValidationError::reject(*code, "forbidden", *token));
        }
    }
}
