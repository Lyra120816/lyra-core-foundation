use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_economics::deterministic_semantic_economics_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_economics_model::{
    SemanticEconomicsFrame, SemanticEconomicsProof, SemanticEconomicsSurface,
    SemanticPublicInterestOutput,
};

pub const P01_SEMANTIC_ECONOMICS_CONTRACT: &str = "LYRA-P01-SEMANTIC-ECONOMICS-PUBLIC-INTEREST v1";

pub const REQUIRED_SEMANTIC_ECONOMICS_RULES: &[&str] = &[
    "semantic_economics_must_be_receipted",
    "public_interest_outputs_must_be_executable",
    "canonical_symbols_atoms_ir_public_value_required",
    "offline_first_access_model_required",
    "semantic_infrastructure_must_reject_capture",
    "economics_bound_to_proof_receipts",
    "no_remote_service_dependency",
    "no_extractive_default",
    "phase_open_until_economics_proven",
];

pub const REQUIRED_SEMANTIC_ECONOMICS_FRAMES: &[&str] = &[
    "canonical_semantics_platform_value_frame",
    "public_semantic_access_frame",
    "anti_capture_symbolic_infrastructure_frame",
    "operator_cost_rebuild_frame",
    "commons_science_governance_frame",
    "semantic_labor_participation_frame",
];

pub const REQUIRED_SEMANTIC_PUBLIC_INTEREST_OUTPUTS: &[&str] = &[
    "public_semantic_casebook",
    "operator_cost_benefit_sheet",
    "semantic_stewardship_review_flow",
    "non_extractive_semantic_access_model",
    "negative_capture_rejection",
    "science_and_civic_reuse_pack",
];

pub const REQUIRED_SEMANTIC_ECONOMICS_PROOFS: &[&str] = &[
    "economics_coverage_proof",
    "public_benefit_binding_proof",
    "access_model_proof",
    "anti_capture_receipt_proof",
    "science_civic_reuse_proof",
    "p01_phase_open",
];

const REQUIRED_COVERAGE_ANCHORS: &[&str] = &[
    "canonical_symbols",
    "semantic_atoms",
    "core_ir",
    "public_interest",
    "non_extractive_access",
    "stewardship",
];
const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_FRAME_KINDS: &[&str] = &[
    "platform_value",
    "public_access",
    "anti_capture",
    "operator_cost",
    "commons",
    "labor_participation",
];
const ALLOWED_OUTPUT_KINDS: &[&str] = &[
    "casebook",
    "cost_sheet",
    "review_flow",
    "access_model",
    "negative",
    "reuse_pack",
];
const ALLOWED_CONSTITUENCIES: &[&str] = &[
    "public",
    "operator",
    "developer",
    "contributor",
    "steward",
    "community",
    "labor",
    "science",
    "civic",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &[
    "economics",
    "public_interest",
    "access",
    "anti_capture",
    "reuse",
    "phase",
];

const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p01-atom-check",
    "lyra-p01-ir-check",
    "lyra-p01-semantic-core-engine-check",
    "lyra-p01-semantic-falsification-check",
    "lyra-p01-semantic-replay-check",
    "lyra-p01-semantic-interface-check",
    "lyra-p01-semantic-packaging-check",
    "lyra-p01-semantic-deployment-check",
    "lyra-p01-semantic-ecosystem-check",
    "lyra-p01-semantic-economics-check",
];

const FORBIDDEN_SEMANTIC_ECONOMICS_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::EconomicsNetworkDependency),
    ("cloud required", ErrorCode::EconomicsNetworkDependency),
    ("online required", ErrorCode::EconomicsNetworkDependency),
    (
        "remote service required",
        ErrorCode::EconomicsNetworkDependency,
    ),
    ("remote fetch", ErrorCode::EconomicsNetworkDependency),
    ("capture allowed", ErrorCode::EconomicsCaptureAllowed),
    (
        "platform capture allowed",
        ErrorCode::EconomicsCaptureAllowed,
    ),
    ("paywall default", ErrorCode::EconomicsExtractiveDefault),
    ("extractive default", ErrorCode::EconomicsExtractiveDefault),
    (
        "rent extraction required",
        ErrorCode::EconomicsExtractiveDefault,
    ),
    (
        "economics drift accepted",
        ErrorCode::EconomicsDriftAccepted,
    ),
    (
        "public interest drift accepted",
        ErrorCode::EconomicsDriftAccepted,
    ),
    ("manual only", ErrorCode::DocsOnlyImplementation),
    ("docs only", ErrorCode::DocsOnlyImplementation),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_economics_surface(
    input: &str,
) -> Result<SemanticEconomicsSurface, Vec<ValidationError>> {
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
            "no semantic economics/public-interest surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_ECONOMICS_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_ECONOMICS_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut frames = Vec::new();
    let mut outputs = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_frames = BTreeSet::new();
    let mut seen_outputs = BTreeSet::new();
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
                    "semantic economics rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(frame_id) = left.strip_prefix("frame:") {
            if !is_symbolic_name(frame_id) || !seen_frames.insert(frame_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEconomicsFrame,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic economics frame {frame_id}"),
                ));
                continue;
            }
            match parse_frame(line_number, frame_id, value) {
                Ok(frame) => frames.push(frame),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(output_id) = left.strip_prefix("output:") {
            if !is_symbolic_name(output_id) || !seen_outputs.insert(output_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEconomicsOutput,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic public-interest output {output_id}"),
                ));
                continue;
            }
            match parse_output(line_number, output_id, value) {
                Ok(output) => outputs.push(output),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) || !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEconomicsProof,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic economics proof {proof_id}"),
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
                format!("unknown semantic economics key {left}"),
            )),
        }
    }
    if phase.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingPhase,
            "surface",
            "missing phase",
        ));
    }
    if task.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingTask,
            "surface",
            "missing task",
        ));
    }
    if status.is_none() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            "surface",
            "missing status",
        ));
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(SemanticEconomicsSurface {
        header,
        phase: phase.unwrap(),
        task: task.unwrap(),
        status: status.unwrap(),
        rules,
        frames,
        outputs,
        proofs,
    })
}

pub fn validate_semantic_economics_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    match parse_semantic_economics_surface(input) {
        Ok(surface) => validate_semantic_economics_model(&surface, &mut errors),
        Err(mut parse_errors) => errors.append(&mut parse_errors),
    }
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_semantic_economics_model(
    surface: &SemanticEconomicsSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic economics/public-interest law must bind to P01",
        ));
    }
    if surface.task != "P01-022" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic economics/public-interest law must bind to P01-022",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported semantic economics status {}", surface.status),
        ));
    }
    require_rules(surface, errors);
    require_frames(surface, errors);
    require_outputs(surface, errors);
    require_proofs(surface, errors);
    validate_frames(surface, errors);
    validate_outputs(surface, errors);
    validate_proofs(surface, errors);
    validate_coverage(surface, errors);
    validate_semantic_economics_report(surface, errors);
}

fn parse_frame(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticEconomicsFrame, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEconomicsFrame,
            format!("line:{line_number:03}"),
            "frame fields must be key:value segments",
        )
    })?;
    let frame_kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidEconomicsFrame,
        line_number,
    )?;
    let path = required_field(
        &fields,
        "path",
        ErrorCode::InvalidEconomicsFrame,
        line_number,
    )?;
    let covers = split_csv(&required_field(
        &fields,
        "covers",
        ErrorCode::InvalidEconomicsFrame,
        line_number,
    )?);
    let outputs = split_csv(&required_field(
        &fields,
        "outputs",
        ErrorCode::InvalidEconomicsFrame,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidEconomicsFrame,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidEconomicsFrame,
        line_number,
    )?;
    Ok(SemanticEconomicsFrame {
        line_number,
        id: id.to_string(),
        frame_kind,
        path,
        covers,
        outputs,
        receipts,
        status,
    })
}

fn parse_output(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticPublicInterestOutput, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEconomicsOutput,
            format!("line:{line_number:03}"),
            "output fields must be key:value segments",
        )
    })?;
    let output_kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidEconomicsOutput,
        line_number,
    )?;
    let path = required_field(
        &fields,
        "path",
        ErrorCode::InvalidEconomicsOutput,
        line_number,
    )?;
    let constituencies = split_csv(&required_field(
        &fields,
        "constituencies",
        ErrorCode::InvalidEconomicsOutput,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidEconomicsOutput,
        line_number,
    )?);
    let proofs = split_csv(&required_field(
        &fields,
        "proofs",
        ErrorCode::InvalidEconomicsOutput,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidEconomicsOutput,
        line_number,
    )?);
    let rejects = split_csv(&required_field(
        &fields,
        "rejects",
        ErrorCode::InvalidEconomicsOutput,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidEconomicsOutput,
        line_number,
    )?;
    Ok(SemanticPublicInterestOutput {
        line_number,
        id: id.to_string(),
        output_kind,
        path,
        constituencies,
        commands,
        proofs,
        receipts,
        rejects,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticEconomicsProof, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEconomicsProof,
            format!("line:{line_number:03}"),
            "proof fields must be key:value segments",
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidEconomicsProof,
        line_number,
    )?;
    let frames = split_csv(&required_field(
        &fields,
        "frames",
        ErrorCode::InvalidEconomicsProof,
        line_number,
    )?);
    let outputs = split_csv(&required_field(
        &fields,
        "outputs",
        ErrorCode::InvalidEconomicsProof,
        line_number,
    )?);
    let receipts = split_csv(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidEconomicsProof,
        line_number,
    )?);
    let commands = split_csv(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidEconomicsProof,
        line_number,
    )?);
    let forbids = split_csv(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidEconomicsProof,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidEconomicsProof,
        line_number,
    )?;
    Ok(SemanticEconomicsProof {
        line_number,
        id: id.to_string(),
        scope,
        frames,
        outputs,
        receipts,
        commands,
        forbids,
        status,
    })
}

fn require_rules(surface: &SemanticEconomicsSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_SEMANTIC_ECONOMICS_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("blocked_until_proven") | Some("forbidden") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingEconomicsRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingEconomicsRule,
                format!("rule:{rule}"),
                "required semantic economics/public-interest rule missing",
            )),
        }
    }
}

fn require_frames(surface: &SemanticEconomicsSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_ECONOMICS_FRAMES {
        if surface.frame_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEconomicsFrame,
                format!("frame:{id}"),
                "required semantic economics frame missing",
            ));
        }
    }
}

fn require_outputs(surface: &SemanticEconomicsSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_PUBLIC_INTEREST_OUTPUTS {
        if surface.output_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEconomicsOutput,
                format!("output:{id}"),
                "required semantic public-interest output missing",
            ));
        }
    }
}

fn require_proofs(surface: &SemanticEconomicsSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_SEMANTIC_ECONOMICS_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingEconomicsProof,
                format!("proof:{id}"),
                "required semantic economics proof missing",
            ));
        }
    }
}

fn validate_frames(surface: &SemanticEconomicsSurface, errors: &mut Vec<ValidationError>) {
    for frame in &surface.frames {
        if !ALLOWED_FRAME_KINDS.contains(&frame.frame_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsFrame,
                frame.canonical_identity(),
                format!("invalid frame kind {}", frame.frame_kind),
            ));
        }
        if !frame.path.starts_with("docs/")
            && !frame.path.starts_with("ops/")
            && !frame.path.starts_with("products/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsFrame,
                frame.canonical_identity(),
                format!("invalid frame path {}", frame.path),
            ));
        }
        if frame.covers.is_empty() || frame.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsFrame,
                frame.canonical_identity(),
                "frames must bind coverage anchors and receipts",
            ));
        }
        if frame.outputs.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsFrame,
                frame.canonical_identity(),
                "frames must bind executable public-interest outputs",
            ));
        }
        for output in &frame.outputs {
            if surface.output_by_id(output).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEconomicsFrame,
                    frame.canonical_identity(),
                    format!("unknown frame output {output}"),
                ));
            }
        }
        if !ALLOWED_STATUSES.contains(&frame.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsFrame,
                frame.canonical_identity(),
                format!("invalid frame status {}", frame.status),
            ));
        }
    }
}

fn validate_outputs(surface: &SemanticEconomicsSurface, errors: &mut Vec<ValidationError>) {
    for output in &surface.outputs {
        if !ALLOWED_OUTPUT_KINDS.contains(&output.output_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsOutput,
                output.canonical_identity(),
                format!("invalid output kind {}", output.output_kind),
            ));
        }
        if !output.path.starts_with("examples/")
            && !output.path.starts_with("fixtures/")
            && !output.path.starts_with("ops/")
            && !output.path.starts_with("products/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsOutput,
                output.canonical_identity(),
                format!("invalid output path {}", output.path),
            ));
        }
        if output.constituencies.is_empty()
            || output.commands.is_empty()
            || output.proofs.is_empty()
            || output.receipts.is_empty()
            || output.rejects.is_empty()
        {
            errors.push(ValidationError::reject(ErrorCode::InvalidEconomicsOutput, output.canonical_identity(), "outputs must bind constituencies, commands, proofs, receipts, and rejection assertions"));
        }
        if !output
            .commands
            .iter()
            .any(|command| command == "lyra-p01-semantic-economics-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsOutput,
                output.canonical_identity(),
                "outputs must be checkable by lyra-p01-semantic-economics-check",
            ));
        }
        for constituency in &output.constituencies {
            if !ALLOWED_CONSTITUENCIES.contains(&constituency.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEconomicsOutput,
                    output.canonical_identity(),
                    format!("unknown constituency {constituency}"),
                ));
            }
        }
        for command in &output.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidEconomicsOutput,
                    output.canonical_identity(),
                    format!("unknown semantic economics output command {command}"),
                ));
            }
        }
        for proof in &output.proofs {
            if surface.proof_by_id(proof).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::EconomicsProofUnbound,
                    output.canonical_identity(),
                    format!("unknown output proof {proof}"),
                ));
            }
        }
        if !ALLOWED_STATUSES.contains(&output.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsOutput,
                output.canonical_identity(),
                format!("invalid output status {}", output.status),
            ));
        }
    }
}

fn validate_proofs(surface: &SemanticEconomicsSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsProof,
                proof.canonical_identity(),
                format!("invalid proof scope {}", proof.scope),
            ));
        }
        if !ALLOWED_STATUSES.contains(&proof.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsProof,
                proof.canonical_identity(),
                format!("invalid proof status {}", proof.status),
            ));
        }
        for frame in &proof.frames {
            if surface.frame_by_id(frame).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::EconomicsProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof frame {frame}"),
                ));
            }
        }
        for output in &proof.outputs {
            if surface.output_by_id(output).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::EconomicsProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof output {output}"),
                ));
            }
        }
        for command in &proof.commands {
            if !REQUIRED_COMMANDS.contains(&command.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::EconomicsProofUnbound,
                    proof.canonical_identity(),
                    format!("unknown proof command {command}"),
                ));
            }
        }
        if proof.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEconomicsProof,
                proof.canonical_identity(),
                "semantic economics proofs must bind receipts",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|item| item == "phase_closure" || item == "global_complete")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "semantic economics proof must keep P01 phase open until closure gate",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|item| item == "capture" || item == "extractive_default")
        {
            errors.push(ValidationError::reject(
                ErrorCode::EconomicsCaptureAllowed,
                proof.canonical_identity(),
                "semantic economics proof must reject capture and extractive defaults",
            ));
        }
    }
}

fn validate_coverage(surface: &SemanticEconomicsSurface, errors: &mut Vec<ValidationError>) {
    let mut covered = BTreeSet::new();
    for frame in &surface.frames {
        for anchor in &frame.covers {
            covered.insert(anchor.as_str());
        }
    }
    for anchor in REQUIRED_COVERAGE_ANCHORS {
        if !covered.contains(*anchor) {
            errors.push(ValidationError::reject(ErrorCode::InvalidEconomicsFrame, format!("coverage:{anchor}"), "semantic economics frames must cover canonical symbols, semantic atoms, core IR, public-interest, access, and stewardship"));
        }
    }
}

fn validate_semantic_economics_report(
    surface: &SemanticEconomicsSurface,
    errors: &mut Vec<ValidationError>,
) {
    let frame_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .frames
        .iter()
        .map(|frame| {
            (
                frame.id.clone(),
                frame.frame_kind.clone(),
                frame.path.clone(),
                frame.covers.clone(),
                frame.outputs.clone(),
                frame.receipts.clone(),
                frame.status.clone(),
            )
        })
        .collect();
    let output_inputs: Vec<(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .outputs
        .iter()
        .map(|output| {
            (
                output.id.clone(),
                output.output_kind.clone(),
                output.path.clone(),
                output.constituencies.clone(),
                output.commands.clone(),
                output.proofs.clone(),
                output.receipts.clone(),
                output.rejects.clone(),
                output.status.clone(),
            )
        })
        .collect();
    let proof_inputs: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
        .proofs
        .iter()
        .map(|proof| {
            (
                proof.id.clone(),
                proof.scope.clone(),
                proof.frames.clone(),
                proof.outputs.clone(),
                proof.receipts.clone(),
                proof.commands.clone(),
                proof.forbids.clone(),
                proof.status.clone(),
            )
        })
        .collect();
    let report =
        deterministic_semantic_economics_suite_report(&frame_inputs, &output_inputs, &proof_inputs);
    if report.frame_count != surface.frames.len()
        || report.output_count != surface.outputs.len()
        || report.proof_count != surface.proofs.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::EconomicsDriftAccepted,
            "k0_semantic_economics_report",
            "semantic economics report count mismatch",
        ));
    }
    if !report.suite_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::EconomicsDriftAccepted,
            "k0_semantic_economics_report",
            "semantic economics report hash must be stable fnv1a128",
        ));
    }
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

fn split_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_SEMANTIC_ECONOMICS_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden semantic economics token {needle}"),
            ));
        }
    }
}
