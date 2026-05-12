use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_formal_semantic_constitution::deterministic_formal_semantic_constitution_suite_report;
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_formal_semantic_constitution::{
    canonical_formal_semantic_constitution_registry_hash, formal_semantic_domain_descriptor,
    formal_semantic_domain_ids, formal_semantic_domains_bind_one_core,
    formal_semantic_invariant_descriptor, formal_semantic_invariant_ids,
    formal_semantic_invariants_reference_admitted_evidence, formal_semantic_law_descriptor,
    formal_semantic_law_ids, formal_semantic_laws_cover_primitive_core,
    formal_semantic_no_forbidden_law_claims, formal_semantic_proof_descriptor,
    formal_semantic_proof_ids, formal_semantic_proofs_bind_known_laws,
    LYRA_P01_FORMAL_SEMANTIC_CORE_REF,
};
use crate::p01_formal_semantic_constitution_model::{
    FormalSemanticConstitutionSurface, FormalSemanticDomainBinding, FormalSemanticInvariantBinding,
    FormalSemanticLawBinding, FormalSemanticProofBinding,
};

pub const P01_FORMAL_SEMANTIC_CONSTITUTION_CONTRACT: &str =
    "LYRA-P01-FORMAL-SEMANTIC-CONSTITUTION v1";
pub const REQUIRED_FORMAL_SEMANTIC_CONSTITUTION_RULES: &[&str] = &[
    "canonical_symbols_have_stable_identity",
    "semantic_atoms_are_closed_and_enumerated",
    "core_ir_is_the_single_cross_phase_semantic_carrier",
    "semantic_objects_bind_identity_and_ir",
    "reference_semantics_are_total_for_admitted_literals",
    "symbolic_equality_requires_canonical_normalization",
    "receipts_require_contract_law_and_verdict_parity",
    "semantic_constitution_rejects_forked_core_claims",
    "no_runtime_network_dependency",
    "no_probabilistic_semantic_truth",
    "no_hidden_randomness",
    "no_placeholder_semantics",
    "no_global_closure_claim",
];
pub const REQUIRED_FORMAL_SEMANTIC_DOMAINS: &[&str] = &[
    "canonical_symbols_domain",
    "semantic_atoms_domain",
    "core_ir_terms_domain",
    "semantic_objects_domain",
    "semantic_identity_domain",
    "reference_semantics_domain",
    "symbolic_equality_domain",
    "receipt_truth_domain",
];
pub const REQUIRED_FORMAL_SEMANTIC_LAWS: &[&str] = &[
    "canonical_symbol_identity_law",
    "semantic_atom_closed_world_law",
    "core_ir_single_carrier_law",
    "semantic_object_identity_law",
    "reference_semantics_totality_law",
    "symbolic_equality_normalization_law",
    "receipt_verdict_parity_law",
    "no_semantic_fork_law",
];
pub const REQUIRED_FORMAL_SEMANTIC_INVARIANTS: &[&str] = &[
    "one_core_invariant",
    "canonical_hash_invariant",
    "atom_reference_invariant",
    "core_ir_reuse_invariant",
    "receipt_parity_invariant",
    "forbidden_semantics_invariant",
];
pub const REQUIRED_FORMAL_SEMANTIC_PROOFS: &[&str] = &[
    "canonical_symbols_proof",
    "semantic_atoms_proof",
    "core_ir_reuse_proof",
    "semantic_object_identity_proof",
    "symbolic_equality_proof",
    "receipt_verdict_parity_proof",
];
const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const FORBIDDEN_FORMAL_SEMANTIC_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic semantic truth",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic semantics allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic semantics", ErrorCode::ProbabilisticTruthAllowed),
    ("random semantics", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder semantics", ErrorCode::PlaceholderAllowed),
    ("stub semantics", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("forked semantic core", ErrorCode::SemanticDriftAccepted),
];

pub fn parse_formal_semantic_constitution_surface(
    input: &str,
) -> Result<FormalSemanticConstitutionSurface, Vec<ValidationError>> {
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
            "empty formal semantic constitution surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P01_FORMAL_SEMANTIC_CONSTITUTION_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_FORMAL_SEMANTIC_CONSTITUTION_CONTRACT}"),
        ));
    }

    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut domains = Vec::new();
    let mut laws = Vec::new();
    let mut invariants = Vec::new();
    let mut proofs = Vec::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
        if let Some(value) = line.strip_prefix("phase=") {
            phase = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("task=") {
            task = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("status=") {
            status = Some(value.to_string());
        } else if let Some((name, value)) = line
            .strip_prefix("rule:")
            .and_then(|value| value.split_once('='))
        {
            if rules.insert(name.to_string(), value.to_string()).is_some() {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    format!("line:{line_number:03}"),
                    format!("duplicate rule {name}"),
                ));
            }
        } else if let Some(value) = line.strip_prefix("domain=") {
            let fields = parse_pipe_fields(value);
            if required_field(&fields, "id").is_none()
                || required_field(&fields, "layer").is_none()
                || required_field(&fields, "owner").is_none()
                || required_field(&fields, "meaning").is_none()
                || required_field(&fields, "core").is_none()
                || required_field(&fields, "status").is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "domain requires id layer owner meaning core status",
                ));
            }
            domains.push(FormalSemanticDomainBinding {
                line_number,
                id: field(&fields, "id"),
                layer: field(&fields, "layer"),
                owner_root: field(&fields, "owner"),
                meaning: field(&fields, "meaning"),
                core_ref: field(&fields, "core"),
                status: field(&fields, "status"),
            });
        } else if let Some(value) = line.strip_prefix("law=") {
            let fields = parse_pipe_fields(value);
            if required_field(&fields, "id").is_none()
                || required_field(&fields, "scope").is_none()
                || required_field(&fields, "rule").is_none()
                || required_field(&fields, "guard").is_none()
                || required_field(&fields, "status").is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "law requires id scope rule guard status",
                ));
            }
            laws.push(FormalSemanticLawBinding {
                line_number,
                id: field(&fields, "id"),
                scope: field(&fields, "scope"),
                rule: field(&fields, "rule"),
                guard: field(&fields, "guard"),
                status: field(&fields, "status"),
            });
        } else if let Some(value) = line.strip_prefix("invariant=") {
            let fields = parse_pipe_fields(value);
            if required_field(&fields, "id").is_none()
                || required_field(&fields, "applies").is_none()
                || required_field(&fields, "assertion").is_none()
                || required_field(&fields, "evidence").is_none()
                || required_field(&fields, "status").is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "invariant requires id applies assertion evidence status",
                ));
            }
            invariants.push(FormalSemanticInvariantBinding {
                line_number,
                id: field(&fields, "id"),
                applies_to: field(&fields, "applies"),
                assertion: field(&fields, "assertion"),
                evidence_ref: field(&fields, "evidence"),
                status: field(&fields, "status"),
            });
        } else if let Some(value) = line.strip_prefix("proof=") {
            let fields = parse_pipe_fields(value);
            if required_field(&fields, "id").is_none()
                || required_field(&fields, "fixture").is_none()
                || required_field(&fields, "golden").is_none()
                || required_field(&fields, "receipt").is_none()
                || required_field(&fields, "law").is_none()
                || required_field(&fields, "status").is_none()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEntrySyntax,
                    format!("line:{line_number:03}"),
                    "proof requires id fixture golden receipt law status",
                ));
            }
            proofs.push(FormalSemanticProofBinding {
                line_number,
                id: field(&fields, "id"),
                fixture: field(&fields, "fixture"),
                golden: field(&fields, "golden"),
                receipt: field(&fields, "receipt"),
                law_ref: field(&fields, "law"),
                status: field(&fields, "status"),
            });
        } else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unrecognized formal semantic constitution line {line}"),
            ));
        }
    }

    if let Some(value) = &phase {
        if value.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingPhase,
                "phase",
                "empty phase",
            ));
        }
    }
    if let Some(value) = &task {
        if value.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingTask,
                "task",
                "empty task",
            ));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(FormalSemanticConstitutionSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        domains,
        laws,
        invariants,
        proofs,
    })
}

pub fn validate_formal_semantic_constitution_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_formal_semantic_constitution_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_formal_semantic_constitution_model(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_formal_semantic_constitution_model(
    surface: &FormalSemanticConstitutionSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-013" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-013 got {}", surface.task),
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }

    for required in REQUIRED_FORMAL_SEMANTIC_CONSTITUTION_RULES {
        match surface.rules.get(*required) {
            Some(value) if value == "required" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingFormalSemanticRule,
                format!("rule:{required}"),
                format!("expected required got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingFormalSemanticRule,
                format!("rule:{required}"),
                "missing formal semantic constitution rule",
            )),
        }
    }

    require_ids(
        "domain",
        REQUIRED_FORMAL_SEMANTIC_DOMAINS,
        surface
            .domains
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingSemanticDomain,
        errors,
    );
    require_ids(
        "law",
        REQUIRED_FORMAL_SEMANTIC_LAWS,
        surface.laws.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingSemanticRuleBinding,
        errors,
    );
    require_ids(
        "invariant",
        REQUIRED_FORMAL_SEMANTIC_INVARIANTS,
        surface
            .invariants
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingInvariantBinding,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_FORMAL_SEMANTIC_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingSemanticProof,
        errors,
    );

    check_duplicate_bindings(
        "domain",
        surface
            .domains
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "law",
        surface
            .laws
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "invariant",
        surface
            .invariants
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "proof",
        surface
            .proofs
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );

    for binding in &surface.domains {
        validate_status(
            "domain",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = formal_semantic_domain_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticDomain,
                format!("line:{:03}", binding.line_number),
                format!("unknown formal semantic domain {}", binding.id),
            ));
            continue;
        };
        if binding.layer != descriptor.layer
            || binding.owner_root != descriptor.owner_root
            || binding.meaning != descriptor.meaning
            || binding.core_ref != descriptor.core_ref
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("domain descriptor drift {}", binding.id),
            ));
        }
        if binding.core_ref != LYRA_P01_FORMAL_SEMANTIC_CORE_REF {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!(
                    "domain {} points to forked core {}",
                    binding.id, binding.core_ref
                ),
            ));
        }
    }

    for binding in &surface.laws {
        validate_status(
            "law",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = formal_semantic_law_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticRuleBinding,
                format!("line:{:03}", binding.line_number),
                format!("unknown formal semantic law {}", binding.id),
            ));
            continue;
        };
        if binding.scope != descriptor.scope
            || binding.rule != descriptor.rule
            || binding.guard != descriptor.guard
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("law descriptor drift {}", binding.id),
            ));
        }
    }

    for binding in &surface.invariants {
        validate_status(
            "invariant",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = formal_semantic_invariant_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInvariantBinding,
                format!("line:{:03}", binding.line_number),
                format!("unknown formal semantic invariant {}", binding.id),
            ));
            continue;
        };
        if binding.applies_to != descriptor.applies_to
            || binding.assertion != descriptor.assertion
            || binding.evidence_ref != descriptor.evidence_ref
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("invariant descriptor drift {}", binding.id),
            ));
        }
    }

    for binding in &surface.proofs {
        validate_status(
            "proof",
            &binding.id,
            binding.line_number,
            &binding.status,
            errors,
        );
        let Some(descriptor) = formal_semantic_proof_descriptor(&binding.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidSemanticProof,
                format!("line:{:03}", binding.line_number),
                format!("unknown formal semantic proof {}", binding.id),
            ));
            continue;
        };
        if binding.fixture != descriptor.fixture
            || binding.golden != descriptor.golden
            || binding.receipt != descriptor.receipt
            || binding.law_ref != descriptor.law_ref
            || binding.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticDriftAccepted,
                format!("line:{:03}", binding.line_number),
                format!("proof descriptor drift {}", binding.id),
            ));
        }
        if formal_semantic_law_descriptor(&binding.law_ref).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::SemanticProofUnbound,
                format!("line:{:03}", binding.line_number),
                format!(
                    "proof {} references unknown law {}",
                    binding.id, binding.law_ref
                ),
            ));
        }
    }

    if !formal_semantic_domains_bind_one_core()
        || !formal_semantic_laws_cover_primitive_core()
        || !formal_semantic_invariants_reference_admitted_evidence()
        || !formal_semantic_proofs_bind_known_laws()
        || !formal_semantic_no_forbidden_law_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::SemanticDriftAccepted,
            "constitution",
            "formal semantic constitution coverage is incomplete or forked",
        ));
    }

    let domain_rows: Vec<(String, String, String, String, String, String)> = surface
        .domains
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.layer.clone(),
                item.owner_root.clone(),
                item.meaning.clone(),
                item.core_ref.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let law_rows: Vec<(String, String, String, String, String)> = surface
        .laws
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.scope.clone(),
                item.rule.clone(),
                item.guard.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let invariant_rows: Vec<(String, String, String, String, String)> = surface
        .invariants
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.applies_to.clone(),
                item.assertion.clone(),
                item.evidence_ref.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let proof_rows: Vec<(String, String, String, String, String, String)> = surface
        .proofs
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.fixture.clone(),
                item.golden.clone(),
                item.receipt.clone(),
                item.law_ref.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let suite = deterministic_formal_semantic_constitution_suite_report(
        &domain_rows,
        &law_rows,
        &invariant_rows,
        &proof_rows,
    );
    if suite.domain_count < formal_semantic_domain_ids().len()
        || suite.law_count < formal_semantic_law_ids().len()
        || suite.invariant_count < formal_semantic_invariant_ids().len()
        || suite.proof_count < formal_semantic_proof_ids().len()
        || !suite.suite_hash.starts_with("fnv1a128:")
        || !canonical_formal_semantic_constitution_registry_hash().starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            "suite",
            "formal semantic constitution suite report is incomplete or unhashable",
        ));
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
                format!("missing required formal semantic {kind} {id}"),
            ));
        }
    }
}

fn check_duplicate_bindings(
    kind: &str,
    items: Vec<(&str, usize)>,
    errors: &mut Vec<ValidationError>,
) {
    let mut seen = BTreeSet::new();
    for (id, line_number) in items {
        if !seen.insert(id.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                format!("line:{line_number:03}"),
                format!("duplicate formal semantic {kind} {id}"),
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
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("line:{line_number:03}"),
            format!("{kind} {id} has unsupported status {status}"),
        ));
    }
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_FORMAL_SEMANTIC_TEXT {
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
        if let Some((key, field_value)) = part.split_once(':') {
            fields.insert(key.to_string(), field_value.to_string());
        }
    }
    fields
}

fn required_field<'a>(fields: &'a BTreeMap<String, String>, name: &str) -> Option<&'a str> {
    fields
        .get(name)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
}

fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    required_field(fields, name).unwrap_or("").to_string()
}
