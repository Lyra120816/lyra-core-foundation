use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_core_engine::deterministic_bootstrap_core_engine_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_canonical_model::bootstrap_canonical_model_ids;
use crate::lyralang_bootstrap_core_engine::{
    bootstrap_core_engine_artifact_descriptor, bootstrap_core_engine_artifact_digest,
    bootstrap_core_engine_artifact_ids, bootstrap_core_engine_artifacts_bind_paths,
    bootstrap_core_engine_no_forbidden_descriptor_claims, bootstrap_core_engine_proof_descriptor,
    bootstrap_core_engine_proof_digest, bootstrap_core_engine_proof_ids,
    bootstrap_core_engine_proofs_bind_registry, bootstrap_core_engine_registry_hash,
    bootstrap_core_engine_transition_descriptor, bootstrap_core_engine_transition_digest,
    bootstrap_core_engine_transition_ids, bootstrap_core_engine_transitions_bind_known_units,
    bootstrap_core_engine_unit_descriptor, bootstrap_core_engine_unit_digest,
    bootstrap_core_engine_unit_ids, bootstrap_core_engine_units_have_stable_order,
    LYRA_P02_BOOTSTRAP_CORE_ENGINE_CARRIER,
};
use crate::p02_bootstrap_core_engine_model::{
    BootstrapCoreEngineArtifactBinding, BootstrapCoreEngineProofBinding,
    BootstrapCoreEngineSurface, BootstrapCoreEngineTransitionBinding,
    BootstrapCoreEngineUnitBinding,
};

pub const P02_BOOTSTRAP_CORE_ENGINE_CONTRACT: &str = "LYRA-P02-BOOTSTRAP-CORE-ENGINE v1";
pub const REQUIRED_BOOTSTRAP_CORE_ENGINE_RULES: &[&str] = &[
    "bootstrap_trust_engine_total",
    "seed_runtime_law_engine_bound",
    "host_extinction_engine_single_carrier",
    "engine_units_ordered",
    "transitions_total_and_bound",
    "outputs_bind_bootstrap_models",
    "artifacts_bind_owner_roots",
    "proofs_bind_fixture_golden_receipt",
    "receipt_commit_is_deterministic",
    "fallback_freeze_precedes_recovery",
    "no_network_engine_source",
    "no_probabilistic_engine",
    "no_hidden_randomness",
    "no_ambient_time",
    "no_placeholder_engine",
    "no_phase_closure_claim",
];
pub const REQUIRED_BOOTSTRAP_CORE_ENGINE_UNITS: &[&str] = &[
    "bootstrap_authority_ingest_engine",
    "seed_runtime_law_binding_engine",
    "host_surface_inventory_engine",
    "foreign_boundary_projection_engine",
    "operator_handoff_capture_engine",
    "emergency_fallback_freeze_engine",
    "bootstrap_receipt_commit_engine",
];
pub const REQUIRED_BOOTSTRAP_CORE_ENGINE_TRANSITIONS: &[&str] = &[
    "authority_ingest_to_seed_law",
    "seed_law_to_host_inventory",
    "host_inventory_to_boundary_projection",
    "boundary_projection_to_handoff_capture",
    "handoff_capture_to_fallback_freeze",
    "fallback_freeze_to_receipt_commit",
];
pub const REQUIRED_BOOTSTRAP_CORE_ENGINE_ARTIFACTS: &[&str] = &[
    "engine_contract",
    "engine_law",
    "engine_operator",
    "valid_engine_fixture",
    "golden_engine_receipt",
    "execution_engine_receipt",
    "inspection_surface",
    "deterministic_suite_report",
];
pub const REQUIRED_BOOTSTRAP_CORE_ENGINE_PROOFS: &[&str] = &[
    "unit_order_proof",
    "model_binding_proof",
    "transition_totality_proof",
    "artifact_binding_proof",
    "receipt_commit_proof",
    "p02_bootstrap_core_engine_parity_proof",
];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const ALLOWED_SURFACE_STATUS: &[&str] = &["bootstrap_core_engine_artifact_emitted"];
const ALLOWED_OWNER_ROOTS: &[&str] = &[
    "lyralang",
    "interfaces",
    "k0",
    "ops",
    "src",
    "fixtures",
    "goldens",
    "receipts",
    "products",
    "shells",
];
const FORBIDDEN_ENGINE_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("remote fetch", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("probabilistic engine", ErrorCode::ProbabilisticTruthAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    ("stochastic engine", ErrorCode::ProbabilisticTruthAllowed),
    ("weighted engine", ErrorCode::ProbabilisticTruthAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("random engine", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("time now", ErrorCode::AmbientTimeAllowed),
    ("placeholder engine", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("stub engine", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
    ("forked carrier", ErrorCode::EngineDriftAccepted),
    ("engine drift accepted", ErrorCode::EngineDriftAccepted),
];

pub fn parse_bootstrap_core_engine_surface(
    input: &str,
) -> Result<BootstrapCoreEngineSurface, Vec<ValidationError>> {
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
            "empty bootstrap core engine surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P02_BOOTSTRAP_CORE_ENGINE_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_CORE_ENGINE_CONTRACT}"),
        ));
    }

    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut units = Vec::new();
    let mut transitions = Vec::new();
    let mut artifacts = Vec::new();
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
        } else if let Some(value) = line.strip_prefix("unit=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &["id", "owner", "input", "output", "order", "law", "status"],
                "unit",
                line_number,
                &mut errors,
            );
            units.push(BootstrapCoreEngineUnitBinding {
                line_number,
                id: field(&field_map, "id"),
                owner_root: field(&field_map, "owner"),
                input_model: field(&field_map, "input"),
                output_model: field(&field_map, "output"),
                stage_order: field(&field_map, "order"),
                engine_law: field(&field_map, "law"),
                status: field(&field_map, "status"),
            });
        } else if let Some(value) = line.strip_prefix("transition=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &["id", "from", "to", "law", "carry", "status"],
                "transition",
                line_number,
                &mut errors,
            );
            transitions.push(BootstrapCoreEngineTransitionBinding {
                line_number,
                id: field(&field_map, "id"),
                from_unit: field(&field_map, "from"),
                to_unit: field(&field_map, "to"),
                transition_law: field(&field_map, "law"),
                carry: field(&field_map, "carry"),
                status: field(&field_map, "status"),
            });
        } else if let Some(value) = line.strip_prefix("artifact=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &["id", "owner", "path", "kind", "status"],
                "artifact",
                line_number,
                &mut errors,
            );
            artifacts.push(BootstrapCoreEngineArtifactBinding {
                line_number,
                id: field(&field_map, "id"),
                owner_root: field(&field_map, "owner"),
                path: field(&field_map, "path"),
                artifact_kind: field(&field_map, "kind"),
                status: field(&field_map, "status"),
            });
        } else if let Some(value) = line.strip_prefix("proof=") {
            let field_map = parse_pipe_fields(value);
            require_fields(
                &field_map,
                &[
                    "id",
                    "units",
                    "transitions",
                    "artifacts",
                    "fixture",
                    "golden",
                    "receipt",
                    "status",
                ],
                "proof",
                line_number,
                &mut errors,
            );
            proofs.push(BootstrapCoreEngineProofBinding {
                line_number,
                id: field(&field_map, "id"),
                units: list_field(&field_map, "units"),
                transitions: list_field(&field_map, "transitions"),
                artifacts: list_field(&field_map, "artifacts"),
                fixture: field(&field_map, "fixture"),
                golden: field(&field_map, "golden"),
                receipt: field(&field_map, "receipt"),
                status: field(&field_map, "status"),
            });
        } else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unrecognized bootstrap core engine line {line}"),
            ));
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(BootstrapCoreEngineSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        units,
        transitions,
        artifacts,
        proofs,
    })
}

pub fn validate_bootstrap_core_engine_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_bootstrap_core_engine_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_bootstrap_core_engine_model(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_core_engine_model(
    surface: &BootstrapCoreEngineSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-015" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-015 got {}", surface.task),
        ));
    }
    if !ALLOWED_SURFACE_STATUS.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            "status",
            format!("unsupported status {}", surface.status),
        ));
    }

    for required in REQUIRED_BOOTSTRAP_CORE_ENGINE_RULES {
        match surface.rules.get(*required) {
            Some(value) if value == "required" || value == "forbidden" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingEngineRule,
                format!("rule:{required}"),
                format!("expected required/forbidden got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingEngineRule,
                format!("rule:{required}"),
                "missing bootstrap core engine rule",
            )),
        }
    }

    require_ids(
        "unit",
        REQUIRED_BOOTSTRAP_CORE_ENGINE_UNITS,
        surface.units.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingEngineUnit,
        errors,
    );
    require_ids(
        "transition",
        REQUIRED_BOOTSTRAP_CORE_ENGINE_TRANSITIONS,
        surface
            .transitions
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingTransitionBinding,
        errors,
    );
    require_ids(
        "artifact",
        REQUIRED_BOOTSTRAP_CORE_ENGINE_ARTIFACTS,
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
        REQUIRED_BOOTSTRAP_CORE_ENGINE_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingEngineProof,
        errors,
    );

    check_duplicate_bindings(
        "unit",
        surface
            .units
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "transition",
        surface
            .transitions
            .iter()
            .map(|item| (item.id.as_str(), item.line_number))
            .collect(),
        errors,
    );
    check_duplicate_bindings(
        "artifact",
        surface
            .artifacts
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

    let unit_ids: BTreeSet<&str> = surface.units.iter().map(|item| item.id.as_str()).collect();
    let transition_ids: BTreeSet<&str> = surface
        .transitions
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let artifact_ids: BTreeSet<&str> = surface
        .artifacts
        .iter()
        .map(|item| item.id.as_str())
        .collect();

    for unit in &surface.units {
        validate_status("unit", &unit.id, unit.line_number, &unit.status, errors);
        if !ALLOWED_OWNER_ROOTS.contains(&unit.owner_root.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidOwnerRoot,
                format!("line:{:03}", unit.line_number),
                format!(
                    "unit {} owner root {} is not allowed",
                    unit.id, unit.owner_root
                ),
            ));
        }
        let Some(descriptor) = bootstrap_core_engine_unit_descriptor(&unit.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEngineUnit,
                format!("line:{:03}", unit.line_number),
                format!("unknown bootstrap core engine unit {}", unit.id),
            ));
            continue;
        };
        if unit.owner_root != descriptor.owner_root
            || unit.input_model != descriptor.input_model
            || unit.output_model != descriptor.output_model
            || unit.stage_order != descriptor.stage_order
            || unit.engine_law != descriptor.engine_law
            || unit.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::EngineDriftAccepted,
                format!("line:{:03}", unit.line_number),
                format!("unit descriptor drift {}", unit.id),
            ));
        }
        if !stable_order(&unit.stage_order) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEngineUnit,
                format!("line:{:03}", unit.line_number),
                format!("unit {} stage order is not stable", unit.id),
            ));
        }
        validate_model_endpoint("input", &unit.input_model, unit.line_number, errors);
        validate_model_endpoint("output", &unit.output_model, unit.line_number, errors);
        if bootstrap_core_engine_unit_digest(&unit.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEngineUnit,
                format!("line:{:03}", unit.line_number),
                format!("unit {} is not digestible", unit.id),
            ));
        }
    }

    for transition in &surface.transitions {
        validate_status(
            "transition",
            &transition.id,
            transition.line_number,
            &transition.status,
            errors,
        );
        let Some(descriptor) = bootstrap_core_engine_transition_descriptor(&transition.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTransitionBinding,
                format!("line:{:03}", transition.line_number),
                format!("unknown bootstrap core engine transition {}", transition.id),
            ));
            continue;
        };
        if transition.from_unit != descriptor.from_unit
            || transition.to_unit != descriptor.to_unit
            || transition.transition_law != descriptor.transition_law
            || transition.carry != descriptor.carry
            || transition.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::EngineDriftAccepted,
                format!("line:{:03}", transition.line_number),
                format!("transition descriptor drift {}", transition.id),
            ));
        }
        if transition.carry != "single_carrier_state" {
            errors.push(ValidationError::reject(
                ErrorCode::EngineDriftAccepted,
                format!("line:{:03}", transition.line_number),
                format!(
                    "transition {} uses forked carrier {}",
                    transition.id, transition.carry
                ),
            ));
        }
        if !unit_ids.contains(transition.from_unit.as_str())
            || !unit_ids.contains(transition.to_unit.as_str())
        {
            errors.push(ValidationError::reject(
                ErrorCode::EngineProofUnbound,
                format!("line:{:03}", transition.line_number),
                format!("transition {} has unbound endpoint", transition.id),
            ));
        }
        if bootstrap_core_engine_transition_digest(&transition.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidTransitionBinding,
                format!("line:{:03}", transition.line_number),
                format!("transition {} is not digestible", transition.id),
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
        let Some(descriptor) = bootstrap_core_engine_artifact_descriptor(&artifact.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", artifact.line_number),
                format!("unknown bootstrap core engine artifact {}", artifact.id),
            ));
            continue;
        };
        if artifact.owner_root != descriptor.owner_root
            || artifact.path != descriptor.path
            || artifact.artifact_kind != descriptor.artifact_kind
            || artifact.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::EngineDriftAccepted,
                format!("line:{:03}", artifact.line_number),
                format!("artifact descriptor drift {}", artifact.id),
            ));
        }
        if artifact.path.contains("..") || artifact.path.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} path is invalid", artifact.id),
            ));
        }
        if bootstrap_core_engine_artifact_digest(&artifact.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} is not digestible", artifact.id),
            ));
        }
    }

    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        let Some(descriptor) = bootstrap_core_engine_proof_descriptor(&proof.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEngineProof,
                format!("line:{:03}", proof.line_number),
                format!("unknown bootstrap core engine proof {}", proof.id),
            ));
            continue;
        };
        if proof.units
            != descriptor
                .units
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
            || proof.transitions
                != descriptor
                    .transitions
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.artifacts
                != descriptor
                    .artifacts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.fixture != descriptor.fixture
            || proof.golden != descriptor.golden
            || proof.receipt != descriptor.receipt
            || proof.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::EngineDriftAccepted,
                format!("line:{:03}", proof.line_number),
                format!("proof descriptor drift {}", proof.id),
            ));
        }
        for unit in &proof.units {
            if !unit_ids.contains(unit.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::EngineProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown unit {}", proof.id, unit),
                ));
            }
        }
        for transition in &proof.transitions {
            if !transition_ids.contains(transition.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::EngineProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!(
                        "proof {} references unknown transition {}",
                        proof.id, transition
                    ),
                ));
            }
        }
        for artifact in &proof.artifacts {
            if !artifact_ids.contains(artifact.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::EngineProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!(
                        "proof {} references unknown artifact {}",
                        proof.id, artifact
                    ),
                ));
            }
        }
        if !proof.fixture.ends_with(".lyra")
            || !proof.golden.ends_with(".receipt")
            || !proof.receipt.ends_with(".receipt")
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} artifact paths are invalid", proof.id),
            ));
        }
        if bootstrap_core_engine_proof_digest(&proof.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEngineProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} is not digestible", proof.id),
            ));
        }
    }

    if !bootstrap_core_engine_units_have_stable_order()
        || !bootstrap_core_engine_transitions_bind_known_units()
        || !bootstrap_core_engine_artifacts_bind_paths()
        || !bootstrap_core_engine_proofs_bind_registry()
        || !bootstrap_core_engine_no_forbidden_descriptor_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::EngineDriftAccepted,
            "bootstrap_core_engine",
            "bootstrap core engine descriptor registry is incomplete or drifted",
        ));
    }

    let unit_rows: Vec<(String, String, String, String, String, String, String)> = surface
        .units
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.owner_root.clone(),
                item.input_model.clone(),
                item.output_model.clone(),
                item.stage_order.clone(),
                item.engine_law.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let transition_rows: Vec<(String, String, String, String, String, String)> = surface
        .transitions
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.from_unit.clone(),
                item.to_unit.clone(),
                item.transition_law.clone(),
                item.carry.clone(),
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
        String,
        String,
        String,
        String,
    )> = surface
        .proofs
        .iter()
        .map(|item| {
            (
                item.id.clone(),
                item.units.clone(),
                item.transitions.clone(),
                item.artifacts.clone(),
                item.fixture.clone(),
                item.golden.clone(),
                item.receipt.clone(),
                item.status.clone(),
            )
        })
        .collect();
    let report = deterministic_bootstrap_core_engine_report(
        &unit_rows,
        &transition_rows,
        &artifact_rows,
        &proof_rows,
    );
    if report.unit_count < bootstrap_core_engine_unit_ids().len()
        || report.transition_count < bootstrap_core_engine_transition_ids().len()
        || report.artifact_count < bootstrap_core_engine_artifact_ids().len()
        || report.proof_count < bootstrap_core_engine_proof_ids().len()
        || report.lyralang_owned_count == 0
        || report.k0_owned_count == 0
        || report.interfaces_owned_count == 0
        || report.ops_owned_count == 0
        || !report.report_hash.starts_with("fnv1a128:")
        || !bootstrap_core_engine_registry_hash().starts_with("fnv1a128:")
        || LYRA_P02_BOOTSTRAP_CORE_ENGINE_CARRIER != "lyra.p02.bootstrap_core_engine.carrier.v1"
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEngineSurface,
            "report",
            "bootstrap core engine report is incomplete or unhashable",
        ));
    }
}

fn validate_model_endpoint(
    kind: &str,
    model: &str,
    line_number: usize,
    errors: &mut Vec<ValidationError>,
) {
    if model == "operator_surface_bytes" || model == "bootstrap_engine_receipt_model" {
        return;
    }
    if !bootstrap_canonical_model_ids().contains(&model) {
        errors.push(ValidationError::reject(
            ErrorCode::EngineProofUnbound,
            format!("line:{line_number:03}"),
            format!("{kind} model {model} is not a P02 bootstrap canonical model endpoint"),
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
                format!("missing required bootstrap core engine {kind} {id}"),
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
                format!("duplicate bootstrap core engine {kind} {id}"),
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
    for (token, code) in FORBIDDEN_ENGINE_TEXT {
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

fn stable_order(value: &str) -> bool {
    value.len() == 3 && value.as_bytes().iter().all(|byte| byte.is_ascii_digit()) && value != "000"
}
