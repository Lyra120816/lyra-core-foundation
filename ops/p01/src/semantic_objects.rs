use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_object::deterministic_semantic_object_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_core_ir::core_ir_descriptor;
use crate::lyralang_semantic_atoms::is_core_atom_id;
use crate::lyralang_semantic_objects::{semantic_object_descriptor, semantic_object_ids};
use crate::p01_semantic_object_model::{
    SemanticObjectBinding, SemanticObjectConformanceBinding, SemanticObjectInvariantBinding,
    SemanticObjectReceiptBinding, SemanticObjectRelationBinding, SemanticObjectSurface,
};

pub const P01_SEMANTIC_OBJECTS_CONTRACT: &str = "LYRA-P01-SEMANTIC-OBJECTS v1";
pub const REQUIRED_SEMANTIC_OBJECT_RULES: &[&str] = &[
    "all_universal_objects_declared",
    "object_kind_identity_canonical",
    "object_parentage_explicit",
    "module_package_program_world_chain_bound",
    "plan_trace_proof_chain_bound",
    "object_serialization_uses_core_ir",
    "object_comparison_is_canonical",
    "cross_layer_conformance_required",
    "no_orphan_semantic_objects",
    "no_cycle_semantic_objects",
    "no_network_dependency",
    "no_probabilistic_object_truth",
    "no_placeholder_objects",
    "no_global_closure_claim",
];
pub const REQUIRED_SEMANTIC_OBJECTS: &[&str] = &[
    "module", "package", "program", "world", "plan", "trace", "proof",
];
pub const REQUIRED_SEMANTIC_OBJECT_RELATIONS: &[&str] = &[
    "module_belongs_package",
    "package_belongs_program",
    "program_runs_in_world",
    "plan_targets_world",
    "trace_witnesses_plan",
    "proof_binds_trace",
];
pub const REQUIRED_SEMANTIC_OBJECT_INVARIANTS: &[&str] = &[
    "no_orphan_objects",
    "acyclic_parentage",
    "receipt_bound_conformance",
    "core_ir_serialization",
    "deterministic_comparison",
];
pub const REQUIRED_SEMANTIC_OBJECT_CONFORMANCES: &[&str] = &[
    "module_ir_conformance",
    "package_ir_conformance",
    "program_ir_conformance",
    "world_ir_conformance",
    "plan_ir_conformance",
    "trace_ir_conformance",
    "proof_ir_conformance",
];
pub const REQUIRED_SEMANTIC_OBJECT_RECEIPTS: &[&str] = &["receipt_semantic_objects"];

const ALLOWED_OWNER_ROOTS: &[&str] = &["lyralang", "interfaces", "k0"];
const ALLOWED_OBJECT_STATUSES: &[&str] = &["admitted", "contract_bound", "executable_seed"];
const ALLOWED_RELATION_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];
const ALLOWED_INVARIANT_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];
const ALLOWED_CONFORMANCE_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];
const ALLOWED_RECEIPT_STATUSES: &[&str] = &["artifact_emitted", "execution_proven"];

const FORBIDDEN_SEMANTIC_OBJECT_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    (
        "probabilistic object truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    (
        "probabilistic truth allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("stochastic object", ErrorCode::ProbabilisticTruthAllowed),
    ("random object", ErrorCode::HiddenRandomnessAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder object", ErrorCode::PlaceholderAllowed),
    ("stub object", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_object_surface(
    input: &str,
) -> Result<SemanticObjectSurface, Vec<ValidationError>> {
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
            "no semantic object lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_OBJECTS_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_OBJECTS_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut objects = Vec::new();
    let mut relations = Vec::new();
    let mut invariants = Vec::new();
    let mut conformances = Vec::new();
    let mut receipts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_objects = BTreeSet::new();
    let mut seen_relations = BTreeSet::new();
    let mut seen_invariants = BTreeSet::new();
    let mut seen_conformances = BTreeSet::new();
    let mut seen_receipts = BTreeSet::new();

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
                    "semantic object rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("object:") {
            if !is_symbolic_name(id) || !seen_objects.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateCanonicalModel,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic object id {id}"),
                ));
                continue;
            }
            match parse_object_binding(line_number, id, value) {
                Ok(object) => objects.push(object),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("relation:") {
            if !is_symbolic_name(id) || !seen_relations.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic object relation id {id}"),
                ));
                continue;
            }
            match parse_relation_binding(line_number, id, value) {
                Ok(relation) => relations.push(relation),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("invariant:") {
            if !is_symbolic_name(id) || !seen_invariants.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateModelBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic object invariant id {id}"),
                ));
                continue;
            }
            match parse_invariant_binding(line_number, id, value) {
                Ok(invariant) => invariants.push(invariant),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("conformance:") {
            if !is_symbolic_name(id) || !seen_conformances.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateProofBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic object conformance id {id}"),
                ));
                continue;
            }
            match parse_conformance_binding(line_number, id, value) {
                Ok(conformance) => conformances.push(conformance),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("receipt:") {
            if !is_symbolic_name(id) || !seen_receipts.insert(id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateProofBinding,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic object receipt id {id}"),
                ));
                continue;
            }
            match parse_receipt_binding(line_number, id, value) {
                Ok(receipt) => receipts.push(receipt),
                Err(error) => errors.push(error),
            }
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
                format!("unknown semantic object entry {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticObjectSurface {
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
        objects,
        relations,
        invariants,
        conformances,
        receipts,
    })
}

pub fn validate_semantic_object_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let parsed = parse_semantic_object_surface(input);
    let mut errors = Vec::new();
    match parsed {
        Ok(surface) => {
            scan_forbidden_text(&canonical, &mut errors);
            validate_surface_scalars(&surface, &mut errors);
            require_rules(&surface, &mut errors);
            require_objects(&surface, &mut errors);
            require_relations(&surface, &mut errors);
            require_invariants(&surface, &mut errors);
            require_conformances(&surface, &mut errors);
            require_receipts(&surface, &mut errors);
            validate_objects(&surface, &mut errors);
            validate_relations(&surface, &mut errors);
            validate_parentage(&surface, &mut errors);
            validate_invariants(&surface, &mut errors);
            validate_conformances(&surface, &mut errors);
            validate_receipts(&surface, &mut errors);
            validate_semantic_object_report(&surface, &mut errors);
        }
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

fn parse_object_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticObjectBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidCanonicalModel,
            format!("line:{line_number:03}"),
            "invalid semantic object field map",
        )
    })?;
    let kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidCanonicalModel,
        line_number,
    )?;
    let atom = required_field(
        &fields,
        "atom",
        ErrorCode::InvalidCanonicalModel,
        line_number,
    )?;
    let owner_root = required_field(&fields, "owner", ErrorCode::MissingOwnerRoot, line_number)?;
    let parent = required_field(
        &fields,
        "parent",
        ErrorCode::MissingModelBinding,
        line_number,
    )?;
    let ir_form = required_field(&fields, "ir", ErrorCode::MissingModelBinding, line_number)?;
    let serialization = required_field(
        &fields,
        "serialization",
        ErrorCode::InvalidFieldBinding,
        line_number,
    )?;
    let comparison = required_field(
        &fields,
        "comparison",
        ErrorCode::InvalidFieldBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(SemanticObjectBinding {
        line_number,
        id: id.to_string(),
        kind,
        atom,
        owner_root,
        parent,
        ir_form,
        serialization,
        comparison,
        status,
    })
}

fn parse_relation_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticObjectRelationBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid semantic object relation field map",
        )
    })?;
    let from_object = required_field(&fields, "from", ErrorCode::InvalidModelBinding, line_number)?;
    let to_object = required_field(&fields, "to", ErrorCode::InvalidModelBinding, line_number)?;
    let relation_kind =
        required_field(&fields, "kind", ErrorCode::InvalidModelBinding, line_number)?;
    let law = required_field(&fields, "law", ErrorCode::InvalidModelBinding, line_number)?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(SemanticObjectRelationBinding {
        line_number,
        id: id.to_string(),
        from_object,
        to_object,
        relation_kind,
        law,
        status,
    })
}

fn parse_invariant_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticObjectInvariantBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidModelBinding,
            format!("line:{line_number:03}"),
            "invalid semantic object invariant field map",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let law = required_field(&fields, "law", ErrorCode::InvalidModelBinding, line_number)?;
    let requires = required_field(
        &fields,
        "requires",
        ErrorCode::InvalidModelBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(SemanticObjectInvariantBinding {
        line_number,
        id: id.to_string(),
        scope,
        law,
        requires,
        status,
    })
}

fn parse_conformance_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticObjectConformanceBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            format!("line:{line_number:03}"),
            "invalid semantic object conformance field map",
        )
    })?;
    let object = required_field(
        &fields,
        "object",
        ErrorCode::InvalidProofBinding,
        line_number,
    )?;
    let ir_form = required_field(&fields, "ir", ErrorCode::InvalidProofBinding, line_number)?;
    let fixture = required_field(
        &fields,
        "fixture",
        ErrorCode::MissingFixtureProof,
        line_number,
    )?;
    let round_trip = required_field(
        &fields,
        "round_trip",
        ErrorCode::InvalidProofBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(SemanticObjectConformanceBinding {
        line_number,
        id: id.to_string(),
        object,
        ir_form,
        fixture,
        round_trip,
        status,
    })
}

fn parse_receipt_binding(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticObjectReceiptBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            format!("line:{line_number:03}"),
            "invalid semantic object receipt field map",
        )
    })?;
    let path = required_field(&fields, "path", ErrorCode::InvalidProofBinding, line_number)?;
    let target = required_field(
        &fields,
        "target",
        ErrorCode::InvalidProofBinding,
        line_number,
    )?;
    let status = required_field(
        &fields,
        "status",
        ErrorCode::UnsupportedClosureStatus,
        line_number,
    )?;
    Ok(SemanticObjectReceiptBinding {
        line_number,
        id: id.to_string(),
        path,
        target,
        status,
    })
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

fn validate_surface_scalars(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01, got {}", surface.phase),
        ));
    }
    if surface.task != "P01-003" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-003, got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported semantic object status {}", surface.status),
        ));
    }
}

fn require_rules(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_OBJECT_RULES {
        if surface.rule_value(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModelRule,
                format!("rule:{required}"),
                "required semantic object rule missing",
            ));
        }
    }
}

fn require_objects(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_OBJECTS {
        if surface.object_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                format!("object:{required}"),
                "required semantic object missing",
            ));
        }
    }
}

fn require_relations(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_OBJECT_RELATIONS {
        if surface.relation_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("relation:{required}"),
                "required semantic object relation missing",
            ));
        }
    }
}

fn require_invariants(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_OBJECT_INVARIANTS {
        if surface.invariant_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingModelBinding,
                format!("invariant:{required}"),
                "required semantic object invariant missing",
            ));
        }
    }
}

fn require_conformances(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_OBJECT_CONFORMANCES {
        if surface.conformance_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingFixtureProof,
                format!("conformance:{required}"),
                "required semantic object conformance fixture missing",
            ));
        }
    }
}

fn require_receipts(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for required in REQUIRED_SEMANTIC_OBJECT_RECEIPTS {
        if surface.receipt_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingProofBinding,
                format!("receipt:{required}"),
                "required semantic object receipt missing",
            ));
        }
    }
}

fn validate_objects(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for object in &surface.objects {
        if !REQUIRED_SEMANTIC_OBJECTS.contains(&object.id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                object.canonical_identity(),
                format!("unknown semantic object {}", object.id),
            ));
        }
        if object.kind != object.id {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                object.canonical_identity(),
                "semantic object row id must equal object kind",
            ));
        }
        if !is_core_atom_id(&object.atom) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                object.canonical_identity(),
                format!("unknown semantic atom {}", object.atom),
            ));
        }
        if !ALLOWED_OWNER_ROOTS.contains(&object.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                object.canonical_identity(),
                format!("invalid object owner root {}", object.owner_root),
            ));
        }
        if core_ir_descriptor(&object.ir_form).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                object.canonical_identity(),
                format!("unknown ir form {}", object.ir_form),
            ));
        }
        if !is_symbolic_name(&object.parent)
            || !is_symbolic_name(&object.serialization)
            || !is_symbolic_name(&object.comparison)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidFieldBinding,
                object.canonical_identity(),
                "parent serialization and comparison must be symbolic names",
            ));
        }
        if !ALLOWED_OBJECT_STATUSES.contains(&object.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                object.canonical_identity(),
                format!("invalid object status {}", object.status),
            ));
        }
        if let Some(descriptor) = semantic_object_descriptor(&object.id) {
            if descriptor.primary_atom != object.atom.as_str()
                || descriptor.owner_root != object.owner_root.as_str()
                || descriptor.required_parent != object.parent.as_str()
                || descriptor.ir_form != object.ir_form.as_str()
                || descriptor.serialization_law != object.serialization.as_str()
                || descriptor.comparison_law != object.comparison.as_str()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::CanonicalModelDriftAccepted,
                    object.canonical_identity(),
                    "object row drifts from LyraLang semantic object descriptor",
                ));
            }
        } else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidCanonicalModel,
                object.canonical_identity(),
                "object id is not in LyraLang semantic object registry",
            ));
        }
    }
}

fn validate_relations(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for relation in &surface.relations {
        if !is_symbolic_name(&relation.relation_kind) || !is_symbolic_name(&relation.law) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                relation.canonical_identity(),
                "relation kind and law must be symbolic names",
            ));
        }
        if surface.object_by_id(&relation.from_object).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                relation.canonical_identity(),
                format!("unknown relation source {}", relation.from_object),
            ));
        }
        if surface.object_by_id(&relation.to_object).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                relation.canonical_identity(),
                format!("unknown relation target {}", relation.to_object),
            ));
        }
        if !ALLOWED_RELATION_STATUSES.contains(&relation.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                relation.canonical_identity(),
                format!("invalid relation status {}", relation.status),
            ));
        }
        match relation.id.as_str() {
            "module_belongs_package" => require_relation_shape(
                surface,
                relation,
                "module",
                "package",
                "contained_by",
                errors,
            ),
            "package_belongs_program" => require_relation_shape(
                surface,
                relation,
                "package",
                "program",
                "contained_by",
                errors,
            ),
            "program_runs_in_world" => {
                require_relation_shape(surface, relation, "program", "world", "executes_in", errors)
            }
            "plan_targets_world" => {
                require_relation_shape(surface, relation, "plan", "world", "targets", errors)
            }
            "trace_witnesses_plan" => {
                require_relation_shape(surface, relation, "trace", "plan", "witnesses", errors)
            }
            "proof_binds_trace" => {
                require_relation_shape(surface, relation, "proof", "trace", "proves", errors)
            }
            _ => {}
        }
    }
}

fn require_relation_shape(
    surface: &SemanticObjectSurface,
    relation: &SemanticObjectRelationBinding,
    from: &str,
    to: &str,
    kind: &str,
    errors: &mut Vec<ValidationError>,
) {
    if relation.from_object != from || relation.to_object != to || relation.relation_kind != kind {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            relation.canonical_identity(),
            format!("expected {from}->{to} relation kind {kind}"),
        ));
    }
    if surface.object_by_id(from).is_none() || surface.object_by_id(to).is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelUnbound,
            relation.canonical_identity(),
            "required relation endpoint missing",
        ));
    }
}

fn validate_parentage(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for object in &surface.objects {
        if object.parent == "none" {
            continue;
        }
        let has_parent_edge = surface.relations.iter().any(|relation| {
            relation.from_object == object.id
                && surface
                    .object_by_id(&relation.to_object)
                    .map(|parent| parent.kind.as_str())
                    == Some(object.parent.as_str())
        });
        if !has_parent_edge {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                object.canonical_identity(),
                format!("object parent {} is not relation-bound", object.parent),
            ));
        }
    }
    if has_relation_cycle(surface) {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "semantic_object_graph",
            "semantic object relation graph contains a cycle",
        ));
    }
}

fn validate_invariants(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for invariant in &surface.invariants {
        if !is_symbolic_name(&invariant.scope)
            || !is_symbolic_name(&invariant.law)
            || !is_symbolic_name(&invariant.requires)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidModelBinding,
                invariant.canonical_identity(),
                "invariant fields must be symbolic names",
            ));
        }
        if invariant.scope != "semantic_object_graph"
            && surface.object_by_id(&invariant.scope).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                invariant.canonical_identity(),
                format!("unknown invariant scope {}", invariant.scope),
            ));
        }
        if !ALLOWED_INVARIANT_STATUSES.contains(&invariant.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                invariant.canonical_identity(),
                format!("invalid invariant status {}", invariant.status),
            ));
        }
    }
}

fn validate_conformances(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for conformance in &surface.conformances {
        if surface.object_by_id(&conformance.object).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                conformance.canonical_identity(),
                format!("unknown conformance object {}", conformance.object),
            ));
        }
        if core_ir_descriptor(&conformance.ir_form).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                conformance.canonical_identity(),
                format!("unknown conformance ir form {}", conformance.ir_form),
            ));
        }
        if !conformance
            .fixture
            .starts_with("fixtures/p01/semantic_object_inputs/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingFixtureProof,
                conformance.canonical_identity(),
                format!("invalid conformance fixture path {}", conformance.fixture),
            ));
        }
        if !is_symbolic_name(&conformance.round_trip) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                conformance.canonical_identity(),
                format!("invalid round trip law {}", conformance.round_trip),
            ));
        }
        if !ALLOWED_CONFORMANCE_STATUSES.contains(&conformance.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                conformance.canonical_identity(),
                format!("invalid conformance status {}", conformance.status),
            ));
        }
    }
}

fn validate_receipts(surface: &SemanticObjectSurface, errors: &mut Vec<ValidationError>) {
    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p01/") || !receipt.path.ends_with(".receipt") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidProofBinding,
                receipt.canonical_identity(),
                format!("receipt path must be a P01 receipt: {}", receipt.path),
            ));
        }
        if receipt.target != "semantic_objects"
            && surface.object_by_id(&receipt.target).is_none()
            && surface.relation_by_id(&receipt.target).is_none()
            && surface.invariant_by_id(&receipt.target).is_none()
            && surface.conformance_by_id(&receipt.target).is_none()
        {
            errors.push(ValidationError::reject(
                ErrorCode::CanonicalModelUnbound,
                receipt.canonical_identity(),
                format!("unknown receipt target {}", receipt.target),
            ));
        }
        if !ALLOWED_RECEIPT_STATUSES.contains(&receipt.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!("invalid receipt status {}", receipt.status),
            ));
        }
    }
}

fn validate_semantic_object_report(
    surface: &SemanticObjectSurface,
    errors: &mut Vec<ValidationError>,
) {
    let object_inputs: Vec<(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )> = surface
        .objects
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.kind.clone(),
                item.atom.clone(),
                item.owner_root.clone(),
                item.parent.clone(),
                item.ir_form.clone(),
                item.serialization.clone(),
                item.comparison.clone(),
                item.status.clone(),
                item.line_number.to_string(),
            )
        })
        .collect();
    let relation_inputs: Vec<(String, String, String, String, String, String)> = surface
        .relations
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.from_object.clone(),
                item.to_object.clone(),
                item.relation_kind.clone(),
                item.law.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let invariant_inputs: Vec<(String, String, String, String, String)> = surface
        .invariants
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.scope.clone(),
                item.law.clone(),
                item.requires.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let conformance_inputs: Vec<(String, String, String, String, String, String)> = surface
        .conformances
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.object.clone(),
                item.ir_form.clone(),
                item.fixture.clone(),
                item.round_trip.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let receipt_inputs: Vec<(String, String, String, String)> = surface
        .receipts
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.path.clone(),
                item.target.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let report = deterministic_semantic_object_suite_report(
        &object_inputs,
        &relation_inputs,
        &invariant_inputs,
        &conformance_inputs,
        &receipt_inputs,
    );
    if report.object_count != surface.objects.len()
        || report.relation_count != surface.relations.len()
        || report.invariant_count != surface.invariants.len()
        || report.conformance_count != surface.conformances.len()
        || report.receipt_count != surface.receipts.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_semantic_object_report",
            "semantic object report count mismatch",
        ));
    }
    if report.object_count != REQUIRED_SEMANTIC_OBJECTS.len()
        || report.relation_count != REQUIRED_SEMANTIC_OBJECT_RELATIONS.len()
        || report.invariant_count != REQUIRED_SEMANTIC_OBJECT_INVARIANTS.len()
        || report.conformance_count != REQUIRED_SEMANTIC_OBJECT_CONFORMANCES.len()
        || report.receipt_count != REQUIRED_SEMANTIC_OBJECT_RECEIPTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingCanonicalModel,
            "k0_semantic_object_report",
            "semantic object report does not cover required P01-003 object model",
        ));
    }
    if report.root_object_count != 1 || report.admitted_object_count == 0 {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_semantic_object_report",
            "semantic object report must include exactly one root and admitted objects",
        ));
    }
    for required in semantic_object_ids() {
        if !surface.objects.iter().any(|object| object.id == required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCanonicalModel,
                "k0_semantic_object_report",
                format!("missing LyraLang semantic object descriptor {required}"),
            ));
        }
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::CanonicalModelDriftAccepted,
            "k0_semantic_object_report",
            "semantic object suite hash must be stable fnv1a128",
        ));
    }
}

fn has_relation_cycle(surface: &SemanticObjectSurface) -> bool {
    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for relation in &surface.relations {
        graph
            .entry(relation.from_object.as_str())
            .or_default()
            .push(relation.to_object.as_str());
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for object in &surface.objects {
        if dfs_cycle(object.id.as_str(), &graph, &mut visiting, &mut visited) {
            return true;
        }
    }
    false
}

fn dfs_cycle<'a>(
    node: &'a str,
    graph: &BTreeMap<&'a str, Vec<&'a str>>,
    visiting: &mut BTreeSet<&'a str>,
    visited: &mut BTreeSet<&'a str>,
) -> bool {
    if visited.contains(node) {
        return false;
    }
    if !visiting.insert(node) {
        return true;
    }
    if let Some(nexts) = graph.get(node) {
        for next in nexts {
            if dfs_cycle(next, graph, visiting, visited) {
                return true;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node);
    false
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

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'.'
        })
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_OBJECT_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic object token {needle}"),
            ));
        }
    }
}
