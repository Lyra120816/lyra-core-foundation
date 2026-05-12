use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_operator_handoff_automation::deterministic_operator_handoff_automation_report;
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p02_operator_handoff_automation_model::{
    OperatorHandoffAutomationSurface, OperatorHandoffCaptureChannelBinding,
    OperatorHandoffReceiptBinding, OperatorHandoffTargetBinding, OperatorHandoffTruthGateBinding,
    OperatorHandoffWorkflowBinding,
};
use std::collections::{BTreeMap, BTreeSet};

pub const P02_OPERATOR_HANDOFF_AUTOMATION_CONTRACT: &str =
    "LYRA-P02-OPERATOR-HANDOFF-AUTOMATION v1";
pub const REQUIRED_OPERATOR_HANDOFF_RULES: &[&str] = &[
    "external_proof_capture_must_be_operator_acknowledged",
    "captured_artifacts_must_be_canonicalized_before_import",
    "handoff_packets_must_bind_target_channel_workflow_gate",
    "truth_state_cannot_advance_without_local_replay_receipt",
    "operator_automation_must_emit_challenge_bindings",
    "handoff_closeout_must_update_truth_snapshot_only_after_validation",
    "every_declared_target_requires_handoff_row",
    "capture_channels_must_be_offline_or_local_only",
    "no_network_required_operator_handoff",
    "no_probabilistic_handoff_truth",
    "no_hidden_randomness_handoff",
    "no_ambient_time_handoff",
    "no_placeholder_handoff",
    "no_global_phase_closure_claim",
];
pub const REQUIRED_OPERATOR_HANDOFF_WORKFLOWS: &[&str] = &[
    "workflow_preflight_capture_manifest",
    "workflow_digest_pairing",
    "workflow_operator_ack_seal",
    "workflow_challenge_binding",
    "workflow_local_replay_quarantine",
    "workflow_truth_snapshot_closeout",
];
pub const REQUIRED_OPERATOR_HANDOFF_CAPTURE_CHANNELS: &[&str] = &[
    "channel_airgap_media",
    "channel_local_filesystem_drop",
    "channel_terminal_paste_digest",
    "channel_external_drive_manifest",
];
pub const REQUIRED_OPERATOR_HANDOFF_TARGETS: &[&str] = &[
    "target_linux_x86_64",
    "target_linux_aarch64",
    "target_windows_x86_64",
    "target_windows_aarch64",
    "target_android_aarch64",
    "target_ios_aarch64",
    "target_wasm32_wasi",
    "target_wasm32_unknown",
    "target_baremetal_x86_64",
    "target_baremetal_aarch64",
    "target_baremetal_riscv64",
    "target_host_tooling_quarantine",
];
pub const REQUIRED_OPERATOR_HANDOFF_TARGET_CLASSES: &[&str] =
    &["linux", "windows", "mobile", "wasm", "baremetal", "other"];
pub const REQUIRED_OPERATOR_HANDOFF_GATES: &[&str] = &[
    "gate_canonical_capture_manifest",
    "gate_operator_acknowledgement",
    "gate_digest_pairing",
    "gate_challenge_suite_binding",
    "gate_local_replay_quarantine",
    "gate_truth_snapshot_update",
];
pub const REQUIRED_OPERATOR_HANDOFF_ARTIFACTS: &[&str] = &[
    "capture_manifest",
    "digest_pair",
    "operator_ack",
    "challenge_binding",
    "replay_receipt",
];
pub const REQUIRED_OPERATOR_HANDOFF_RECEIPTS: &[&str] = &[
    "receipt_bootstrap_evidence_emission",
    "receipt_host_boundary_challenge_suites",
    "receipt_bootstrap_target_matrix",
    "receipt_operator_handoff_automation",
];
const ALLOWED_WORKFLOW_STAGES: &[&str] = &[
    "preflight",
    "canonicalization",
    "ack_seal",
    "challenge_binding",
    "replay_quarantine",
    "closeout",
];
const ALLOWED_WORKFLOW_TRIGGERS: &[&str] = &[
    "operator_explicit",
    "imported_artifact_detected",
    "local_replay_completed",
    "local_validation_receipt_present",
];
const ALLOWED_CHANNEL_MEDIA: &[&str] = &[
    "airgap_media",
    "local_filesystem",
    "operator_terminal",
    "external_drive",
];
const ALLOWED_GATE_CLASSES: &[&str] = &["pre_import", "import", "post_import"];
const ALLOWED_TRUTH_EFFECTS: &[&str] = &["none_without_local_replay"];
const ALLOWED_WORKFLOW_STATUS: &[&str] = &["automation_requirement_emitted"];
const ALLOWED_CHANNEL_STATUS: &[&str] = &["capture_channel_bounded"];
const ALLOWED_HANDOFF_STATUS: &[&str] = &["handoff_requirement_emitted"];
const ALLOWED_GATE_STATUS: &[&str] = &["gate_requirement_emitted"];
const FORBIDDEN: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::AmbientNetworkAllowed),
    ("cloud required", ErrorCode::AmbientNetworkAllowed),
    ("online required", ErrorCode::AmbientNetworkAllowed),
    ("hidden randomness", ErrorCode::HiddenRandomnessAllowed),
    ("ambient time", ErrorCode::AmbientTimeAllowed),
    ("probabilistic truth", ErrorCode::ProbabilisticTruthAllowed),
    ("truth drift accepted", ErrorCode::AmbientAuthority),
    ("placeholder=true", ErrorCode::PlaceholderAllowed),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("phase_complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_operator_handoff_automation_surface(
    input: &str,
) -> Result<OperatorHandoffAutomationSurface, Vec<ValidationError>> {
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
            "empty operator handoff automation surface",
        )]);
    }
    if lines[0] != P02_OPERATOR_HANDOFF_AUTOMATION_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_OPERATOR_HANDOFF_AUTOMATION_CONTRACT}"),
        )]);
    }
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut previous_evidence_receipt = None;
    let mut rules = BTreeMap::new();
    let mut workflows = Vec::new();
    let mut capture_channels = Vec::new();
    let mut target_handoffs = Vec::new();
    let mut truth_gates = Vec::new();
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
        if left.is_empty() || value.is_empty() || left != left.trim() || value != value.trim() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "untrimmed or empty entry",
            ));
            continue;
        }
        if let Some(id) = left.strip_prefix("rule:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    left,
                    "duplicate rule",
                ));
            } else {
                rules.insert(id.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("workflow:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceWorkflow,
                    left,
                    "duplicate workflow",
                ));
            } else {
                match parse_workflow(n, id, value) {
                    Ok(x) => workflows.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("capture_channel:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceCommand,
                    left,
                    "duplicate capture channel",
                ));
            } else {
                match parse_channel(n, id, value) {
                    Ok(x) => capture_channels.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("target_handoff:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeploymentTarget,
                    left,
                    "duplicate target handoff",
                ));
            } else {
                match parse_handoff(n, id, value) {
                    Ok(x) => target_handoffs.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("truth_gate:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReviewGate,
                    left,
                    "duplicate truth gate",
                ));
            } else {
                match parse_gate(n, id, value) {
                    Ok(x) => truth_gates.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if let Some(id) = left.strip_prefix("receipt:") {
            if !seen.insert(left.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    left,
                    "duplicate receipt",
                ));
            } else {
                match parse_receipt(n, id, value) {
                    Ok(x) => receipts.push(x),
                    Err(e) => errors.push(e),
                }
            }
            continue;
        }
        if !seen.insert(left.to_string()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateEntry,
                left,
                "duplicate scalar",
            ));
            continue;
        }
        match left {
            "phase" => phase = Some(value.to_string()),
            "task" => task = Some(value.to_string()),
            "status" => status = Some(value.to_string()),
            "previous_evidence_receipt" => previous_evidence_receipt = Some(value.to_string()),
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                format!("unknown key {left}"),
            )),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(OperatorHandoffAutomationSurface {
        header: lines[0].clone(),
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
        previous_evidence_receipt: previous_evidence_receipt.ok_or_else(|| {
            vec![ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                "previous_evidence_receipt",
                "missing P02-010 receipt",
            )]
        })?,
        rules,
        workflows,
        capture_channels,
        target_handoffs,
        truth_gates,
        receipts,
    })
}

pub fn validate_operator_handoff_automation_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let mut forbidden = Vec::new();
    scan_forbidden(input, &mut forbidden);
    let verdict = match parse_operator_handoff_automation_surface(input) {
        Ok(surface) => {
            let mut v = validate_operator_handoff_automation_model(&surface);
            if !forbidden.is_empty() {
                let mut errors = v.errors;
                errors.extend(forbidden);
                v = Verdict::rejected(errors);
            }
            v
        }
        Err(mut errors) => {
            errors.extend(forbidden);
            Verdict::rejected(errors)
        }
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_operator_handoff_automation_model(
    surface: &OperatorHandoffAutomationSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-011" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-011 got {}", surface.task),
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported {}", surface.status),
        ));
    }
    if surface.previous_evidence_receipt
        != "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt"
    {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "previous_evidence_receipt",
            "must bind P02-010 receipt",
        ));
    }

    for rule in REQUIRED_OPERATOR_HANDOFF_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") => {}
            Some(v) => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("rule:{rule}"),
                format!("bad rule value {v}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "missing handoff rule",
            )),
        }
    }
    for workflow in REQUIRED_OPERATOR_HANDOFF_WORKFLOWS {
        if surface.workflow_by_id(workflow).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceWorkflow,
                format!("workflow:{workflow}"),
                "missing operator workflow",
            ));
        }
    }
    for channel in REQUIRED_OPERATOR_HANDOFF_CAPTURE_CHANNELS {
        if surface.capture_channel_by_id(channel).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceCommand,
                format!("capture_channel:{channel}"),
                "missing capture channel",
            ));
        }
    }
    for target in REQUIRED_OPERATOR_HANDOFF_TARGETS {
        if surface.target_handoff_by_target(target).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target_handoff:{target}"),
                "missing target handoff",
            ));
        }
    }
    for gate in REQUIRED_OPERATOR_HANDOFF_GATES {
        if surface.truth_gate_by_id(gate).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReviewGate,
                format!("truth_gate:{gate}"),
                "missing truth gate",
            ));
        }
    }
    for receipt in REQUIRED_OPERATOR_HANDOFF_RECEIPTS {
        if surface.receipt_by_id(receipt).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("receipt:{receipt}"),
                "missing receipt binding",
            ));
        }
    }

    for workflow in &surface.workflows {
        if !ALLOWED_WORKFLOW_STAGES.contains(&workflow.stage.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                workflow.canonical_identity(),
                format!("bad stage {}", workflow.stage),
            ));
        }
        if !ALLOWED_WORKFLOW_TRIGGERS.contains(&workflow.trigger.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                workflow.canonical_identity(),
                format!("bad trigger {}", workflow.trigger),
            ));
        }
        if workflow.deterministic_order != "lexicographic_by_id" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                workflow.canonical_identity(),
                "workflow order must be lexicographic_by_id",
            ));
        }
        if !ALLOWED_TRUTH_EFFECTS.contains(&workflow.truth_effect.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::AmbientAuthority,
                workflow.canonical_identity(),
                format!("bad truth effect {}", workflow.truth_effect),
            ));
        }
        if !ALLOWED_WORKFLOW_STATUS.contains(&workflow.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                workflow.canonical_identity(),
                format!("bad workflow status {}", workflow.status),
            ));
        }
    }

    for channel in &surface.capture_channels {
        if !ALLOWED_CHANNEL_MEDIA.contains(&channel.medium.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                channel.canonical_identity(),
                format!("bad medium {}", channel.medium),
            ));
        }
        if channel.boundary != "external_proof_capture" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                channel.canonical_identity(),
                "channel boundary must be external_proof_capture",
            ));
        }
        if channel.network_allowed {
            errors.push(ValidationError::reject(
                ErrorCode::AmbientNetworkAllowed,
                channel.canonical_identity(),
                "capture channel cannot allow network",
            ));
        }
        if !channel.operator_ack_required {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReviewGate,
                channel.canonical_identity(),
                "operator ack must be required",
            ));
        }
        if !channel.capture_hash_required {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReceiptProof,
                channel.canonical_identity(),
                "capture hash must be required",
            ));
        }
        if !ALLOWED_CHANNEL_STATUS.contains(&channel.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                channel.canonical_identity(),
                format!("bad channel status {}", channel.status),
            ));
        }
    }

    let mut classes = BTreeSet::new();
    for handoff in &surface.target_handoffs {
        if !REQUIRED_OPERATOR_HANDOFF_TARGETS.contains(&handoff.target_id.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                handoff.canonical_identity(),
                format!("unknown target {}", handoff.target_id),
            ));
        }
        if !REQUIRED_OPERATOR_HANDOFF_TARGET_CLASSES.contains(&handoff.target_class.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeploymentTarget,
                handoff.canonical_identity(),
                format!("bad class {}", handoff.target_class),
            ));
        }
        classes.insert(handoff.target_class.as_str());
        if !REQUIRED_OPERATOR_HANDOFF_CAPTURE_CHANNELS.contains(&handoff.capture_channel.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                handoff.canonical_identity(),
                format!("unknown channel {}", handoff.capture_channel),
            ));
        }
        if !REQUIRED_OPERATOR_HANDOFF_WORKFLOWS.contains(&handoff.automation_workflow.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                handoff.canonical_identity(),
                format!("unknown workflow {}", handoff.automation_workflow),
            ));
        }
        for artifact in REQUIRED_OPERATOR_HANDOFF_ARTIFACTS {
            if !handoff.required_artifacts.iter().any(|x| x == artifact) {
                errors.push(ValidationError::reject(
                    ErrorCode::MissingDeliveryArtifact,
                    handoff.canonical_identity(),
                    format!("missing required artifact {artifact}"),
                ));
            }
        }
        if !REQUIRED_OPERATOR_HANDOFF_GATES.contains(&handoff.pre_import_gate.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingReviewGate,
                handoff.canonical_identity(),
                format!("unknown gate {}", handoff.pre_import_gate),
            ));
        }
        if !ALLOWED_TRUTH_EFFECTS.contains(&handoff.post_import_truth_effect.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::AmbientAuthority,
                handoff.canonical_identity(),
                format!("bad truth effect {}", handoff.post_import_truth_effect),
            ));
        }
        if handoff.receipt_binding != "receipt_operator_handoff_automation" {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                handoff.canonical_identity(),
                "handoff must bind P02-011 receipt",
            ));
        }
        if !ALLOWED_HANDOFF_STATUS.contains(&handoff.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                handoff.canonical_identity(),
                format!("bad handoff status {}", handoff.status),
            ));
        }
    }
    for class in REQUIRED_OPERATOR_HANDOFF_TARGET_CLASSES {
        if !classes.contains(class) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingDeploymentTarget,
                format!("target_class:{class}"),
                "missing target class handoff",
            ));
        }
    }

    for gate in &surface.truth_gates {
        if !ALLOWED_GATE_CLASSES.contains(&gate.gate_class.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReviewGate,
                gate.canonical_identity(),
                format!("bad gate class {}", gate.gate_class),
            ));
        }
        if !REQUIRED_OPERATOR_HANDOFF_WORKFLOWS.contains(&gate.required_before.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReviewGate,
                gate.canonical_identity(),
                format!("unknown workflow {}", gate.required_before),
            ));
        }
        if gate.rejects.is_empty() || gate.rejects == "allow_truth_drift" {
            errors.push(ValidationError::reject(
                ErrorCode::AmbientAuthority,
                gate.canonical_identity(),
                "gate must reject truth drift conditions",
            ));
        }
        if !gate.evidence_path.starts_with("ops/p02/handoff/")
            && !gate.evidence_path.starts_with("receipts/p02/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                gate.canonical_identity(),
                "gate evidence path must live in ops/p02/handoff or receipts/p02",
            ));
        }
        if !ALLOWED_GATE_STATUS.contains(&gate.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                gate.canonical_identity(),
                format!("bad gate status {}", gate.status),
            ));
        }
    }

    for receipt in &surface.receipts {
        if !receipt.path.starts_with("receipts/p02/")
            && !receipt.path.starts_with("interfaces/p02/")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnknownEvidencePath,
                receipt.canonical_identity(),
                "receipt path must be P02 receipt or contract path",
            ));
        }
        if receipt.status != "artifact_emitted" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                receipt.canonical_identity(),
                format!("bad receipt status {}", receipt.status),
            ));
        }
    }

    if errors.is_empty() {
        let workflows = surface
            .workflows
            .iter()
            .map(|x| {
                (
                    x.id.clone(),
                    x.stage.clone(),
                    x.trigger.clone(),
                    x.input_set.clone(),
                    x.output_set.clone(),
                    x.deterministic_order.clone(),
                    x.truth_effect.clone(),
                    x.status.clone(),
                )
            })
            .collect::<Vec<_>>();
        let channels = surface
            .capture_channels
            .iter()
            .map(|x| {
                (
                    x.id.clone(),
                    x.medium.clone(),
                    x.boundary.clone(),
                    x.network_allowed,
                    x.operator_ack_required,
                    x.capture_hash_required,
                    x.status.clone(),
                )
            })
            .collect::<Vec<_>>();
        let handoffs = surface
            .target_handoffs
            .iter()
            .map(|x| {
                (
                    x.id.clone(),
                    x.target_id.clone(),
                    x.target_class.clone(),
                    x.capture_channel.clone(),
                    x.automation_workflow.clone(),
                    x.required_artifacts.clone(),
                    x.pre_import_gate.clone(),
                    x.post_import_truth_effect.clone(),
                    x.receipt_binding.clone(),
                    x.status.clone(),
                )
            })
            .collect::<Vec<_>>();
        let gates = surface
            .truth_gates
            .iter()
            .map(|x| {
                (
                    x.id.clone(),
                    x.gate_class.clone(),
                    x.required_before.clone(),
                    x.rejects.clone(),
                    x.evidence_path.clone(),
                    x.status.clone(),
                )
            })
            .collect::<Vec<_>>();
        let receipts = surface
            .receipts
            .iter()
            .map(|x| {
                (
                    x.id.clone(),
                    x.path.clone(),
                    x.target.clone(),
                    x.status.clone(),
                )
            })
            .collect::<Vec<_>>();
        let _report = deterministic_operator_handoff_automation_report(
            &workflows, &channels, &handoffs, &gates, &receipts,
        );
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_workflow(
    n: usize,
    id: &str,
    value: &str,
) -> Result<OperatorHandoffWorkflowBinding, ValidationError> {
    let fields = fields(value, n)?;
    Ok(OperatorHandoffWorkflowBinding {
        line_number: n,
        id: id.to_string(),
        stage: req(&fields, "stage", n)?,
        trigger: req(&fields, "trigger", n)?,
        input_set: req(&fields, "input_set", n)?,
        output_set: req(&fields, "output_set", n)?,
        deterministic_order: req(&fields, "deterministic_order", n)?,
        truth_effect: req(&fields, "truth_effect", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn parse_channel(
    n: usize,
    id: &str,
    value: &str,
) -> Result<OperatorHandoffCaptureChannelBinding, ValidationError> {
    let fields = fields(value, n)?;
    Ok(OperatorHandoffCaptureChannelBinding {
        line_number: n,
        id: id.to_string(),
        medium: req(&fields, "medium", n)?,
        boundary: req(&fields, "boundary", n)?,
        network_allowed: bool_field(&fields, "network_allowed", n)?,
        operator_ack_required: bool_field(&fields, "operator_ack_required", n)?,
        capture_hash_required: bool_field(&fields, "capture_hash_required", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn parse_handoff(
    n: usize,
    id: &str,
    value: &str,
) -> Result<OperatorHandoffTargetBinding, ValidationError> {
    let fields = fields(value, n)?;
    Ok(OperatorHandoffTargetBinding {
        line_number: n,
        id: id.to_string(),
        target_id: req(&fields, "target_id", n)?,
        target_class: req(&fields, "target_class", n)?,
        capture_channel: req(&fields, "capture_channel", n)?,
        automation_workflow: req(&fields, "automation_workflow", n)?,
        required_artifacts: csv(&req(&fields, "required_artifacts", n)?),
        pre_import_gate: req(&fields, "pre_import_gate", n)?,
        post_import_truth_effect: req(&fields, "post_import_truth_effect", n)?,
        receipt_binding: req(&fields, "receipt_binding", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn parse_gate(
    n: usize,
    id: &str,
    value: &str,
) -> Result<OperatorHandoffTruthGateBinding, ValidationError> {
    let fields = fields(value, n)?;
    Ok(OperatorHandoffTruthGateBinding {
        line_number: n,
        id: id.to_string(),
        gate_class: req(&fields, "gate_class", n)?,
        required_before: req(&fields, "required_before", n)?,
        rejects: req(&fields, "rejects", n)?,
        evidence_path: req(&fields, "evidence_path", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn parse_receipt(
    n: usize,
    id: &str,
    value: &str,
) -> Result<OperatorHandoffReceiptBinding, ValidationError> {
    let fields = fields(value, n)?;
    Ok(OperatorHandoffReceiptBinding {
        line_number: n,
        id: id.to_string(),
        path: req(&fields, "path", n)?,
        target: req(&fields, "target", n)?,
        status: req(&fields, "status", n)?,
    })
}

fn fields(value: &str, n: usize) -> Result<BTreeMap<String, String>, ValidationError> {
    let mut output = BTreeMap::new();
    for segment in value.split('|') {
        let Some((key, val)) = segment.split_once(':') else {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "bad field",
            ));
        };
        if key.is_empty()
            || val.is_empty()
            || output.insert(key.to_string(), val.to_string()).is_some()
        {
            return Err(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{n:03}"),
                "bad field",
            ));
        }
    }
    Ok(output)
}

fn req(fields: &BTreeMap<String, String>, key: &str, n: usize) -> Result<String, ValidationError> {
    fields.get(key).cloned().ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{n:03}"),
            format!("missing {key}"),
        )
    })
}

fn bool_field(
    fields: &BTreeMap<String, String>,
    key: &str,
    n: usize,
) -> Result<bool, ValidationError> {
    match req(fields, key, n)?.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ValidationError::reject(
            ErrorCode::InvalidEntrySyntax,
            format!("line:{n:03}"),
            format!("bad bool {key}"),
        )),
    }
}

fn csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect()
}

fn scan_forbidden(input: &str, errors: &mut Vec<ValidationError>) {
    let lower = input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden_text",
                format!("forbidden operator-handoff phrase {needle}"),
            ));
        }
    }
}
