use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_operator_interface::deterministic_bootstrap_operator_interface_suite_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_operator_interface::{
    bootstrap_operator_artifact_descriptor, bootstrap_operator_artifact_digest,
    bootstrap_operator_artifact_ids, bootstrap_operator_artifacts_bind_commands,
    bootstrap_operator_carrier_signature, bootstrap_operator_command_descriptor,
    bootstrap_operator_command_digest, bootstrap_operator_command_ids,
    bootstrap_operator_commands_cover_p02_018, bootstrap_operator_example_descriptor,
    bootstrap_operator_example_digest, bootstrap_operator_example_ids,
    bootstrap_operator_examples_bind_known_commands, bootstrap_operator_gate_descriptor,
    bootstrap_operator_gate_digest, bootstrap_operator_gate_ids,
    bootstrap_operator_gates_bind_registry, bootstrap_operator_no_forbidden_descriptor_claims,
    bootstrap_operator_proof_descriptor, bootstrap_operator_proof_digest,
    bootstrap_operator_proof_ids, bootstrap_operator_proofs_bind_registry,
    bootstrap_operator_registry_hash, bootstrap_operator_workflow_descriptor,
    bootstrap_operator_workflow_digest, bootstrap_operator_workflow_ids,
    bootstrap_operator_workflows_bind_known_commands,
    LYRA_P02_BOOTSTRAP_OPERATOR_INTERFACE_CARRIER,
};
use crate::p02_bootstrap_operator_interface_model::{
    BootstrapOperatorAcceptanceGateBinding, BootstrapOperatorArtifactBinding,
    BootstrapOperatorCommandBinding, BootstrapOperatorExampleBinding,
    BootstrapOperatorInterfaceSurface, BootstrapOperatorProofBinding,
    BootstrapOperatorWorkflowBinding,
};

pub const P02_BOOTSTRAP_OPERATOR_INTERFACE_CONTRACT: &str =
    "LYRA-P02-BOOTSTRAP-OPERATOR-INTERFACE v1";

pub const REQUIRED_BOOTSTRAP_OPERATOR_INTERFACE_RULES: &[&str] = &[
    "bootstrap_developer_commands_required",
    "bootstrap_operator_workflows_required",
    "bootstrap_acceptance_gates_required",
    "bootstrap_interface_examples_required",
    "bootstrap_interface_receipts_required",
    "seed_runtime_operator_path_required",
    "host_extinction_operator_path_required",
    "deterministic_interface_report_required",
    "no_human_only_procedure",
    "no_network_interface",
    "no_probabilistic_interface",
    "no_ambient_time_interface",
    "no_unreceipted_interface",
    "no_phase_closure_claim",
];

pub const REQUIRED_BOOTSTRAP_OPERATOR_COMMANDS: &[&str] = &[
    "bootstrap_trust_status",
    "seed_runtime_law_verify",
    "host_extinction_audit",
    "bootstrap_target_preflight",
    "bootstrap_falsification_run",
    "bootstrap_replay_witness_run",
    "bootstrap_interface_report",
    "bootstrap_operator_acceptance",
];
pub const REQUIRED_BOOTSTRAP_OPERATOR_WORKFLOWS: &[&str] = &[
    "developer_bootstrap_trust_review",
    "operator_seed_runtime_replacement",
    "host_extinction_audit_flow",
    "cross_target_preflight_flow",
    "bootstrap_operator_handoff_flow",
];
pub const REQUIRED_BOOTSTRAP_OPERATOR_EXAMPLES: &[&str] = &[
    "bootstrap_trust_cli_example",
    "seed_runtime_law_cli_example",
    "host_extinction_audit_example",
    "cross_target_preflight_example",
    "operator_acceptance_example",
];
pub const REQUIRED_BOOTSTRAP_OPERATOR_GATES: &[&str] = &[
    "bootstrap_trust_operator_gate",
    "seed_runtime_operator_gate",
    "host_extinction_operator_gate",
    "cross_target_operator_gate",
    "p02_operator_acceptance_gate",
];
pub const REQUIRED_BOOTSTRAP_OPERATOR_PROOFS: &[&str] = &[
    "bootstrap_trust_interface_proof",
    "seed_runtime_operator_interface_proof",
    "host_extinction_operator_interface_proof",
    "cross_target_interface_proof",
    "p02_operator_acceptance_proof",
];
pub const REQUIRED_BOOTSTRAP_OPERATOR_ARTIFACTS: &[&str] = &[
    "bootstrap_operator_interface_contract",
    "bootstrap_operator_interface_law",
    "bootstrap_operator_interface_shell",
    "bootstrap_operator_interface_binary",
    "valid_bootstrap_operator_interface_fixture",
    "bootstrap_operator_interface_examples",
    "golden_bootstrap_operator_interface_receipt",
    "execution_bootstrap_operator_interface_receipt",
    "deterministic_bootstrap_operator_interface_report",
];

const ALLOWED_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const ALLOWED_ROLES: &[&str] = &["developer", "operator", "auditor", "packager"];
const ALLOWED_TARGETS: &[&str] = &["linux", "windows", "android", "macos", "wasm", "baremetal"];
const ALLOWED_DECISIONS: &[&str] = &[
    "admit_working_slice",
    "require_replay_receipts",
    "block_phase_closure",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["interface", "workflow", "gate", "handoff"];
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
    "shells",
    "docs",
    "products",
    "examples",
];

const FORBIDDEN_BOOTSTRAP_OPERATOR_TEXT: &[(&str, ErrorCode)] = &[
    ("manual only", ErrorCode::ManualOnlyInterface),
    ("manual_only_interface", ErrorCode::ManualOnlyInterface),
    (
        "network interface required",
        ErrorCode::InterfaceNetworkDependency,
    ),
    (
        "remote operator required",
        ErrorCode::InterfaceNetworkDependency,
    ),
    (
        "probabilistic interface allowed",
        ErrorCode::ProbabilisticTruthAllowed,
    ),
    ("ambient time interface", ErrorCode::AmbientTimeAllowed),
    ("unreceipted handoff allowed", ErrorCode::ClosureUnreceipted),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_bootstrap_operator_interface_surface(
    input: &str,
) -> Result<BootstrapOperatorInterfaceSurface, Vec<ValidationError>> {
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
            "empty bootstrap operator interface surface",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P02_BOOTSTRAP_OPERATOR_INTERFACE_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_OPERATOR_INTERFACE_CONTRACT}"),
        ));
    }

    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut commands = Vec::new();
    let mut workflows = Vec::new();
    let mut examples = Vec::new();
    let mut gates = Vec::new();
    let mut proofs = Vec::new();
    let mut artifacts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_commands = BTreeSet::new();
    let mut seen_workflows = BTreeSet::new();
    let mut seen_examples = BTreeSet::new();
    let mut seen_gates = BTreeSet::new();
    let mut seen_proofs = BTreeSet::new();
    let mut seen_artifacts = BTreeSet::new();
    let mut seen_orders = BTreeSet::new();

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
                    ErrorCode::MissingInterfaceRule,
                    format!("line:{line_number:03}"),
                    "rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if left == "command" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id", "binary", "surface", "input", "output", "receipts", "roles", "targets",
                    "status",
                ],
                "command",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_commands.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceCommand,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap operator command {id}"),
                ));
            }
            commands.push(BootstrapOperatorCommandBinding {
                line_number,
                id,
                binary: field(&fields, "binary"),
                surface: field(&fields, "surface"),
                input: field(&fields, "input"),
                output: field(&fields, "output"),
                receipts: list_field(&fields, "receipts"),
                roles: list_field(&fields, "roles"),
                targets: list_field(&fields, "targets"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "workflow" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id", "order", "commands", "targets", "examples", "forbids", "status",
                ],
                "workflow",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            let order = field(&fields, "order");
            if !is_symbolic_name(&id) || !seen_workflows.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceWorkflow,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap operator workflow {id}"),
                ));
            }
            if order.len() != 3
                || !order.bytes().all(|byte| byte.is_ascii_digit())
                || !seen_orders.insert(order.clone())
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceWorkflow,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid workflow order {order}"),
                ));
            }
            workflows.push(BootstrapOperatorWorkflowBinding {
                line_number,
                id,
                order,
                commands: list_field(&fields, "commands"),
                targets: list_field(&fields, "targets"),
                examples: list_field(&fields, "examples"),
                forbids: list_field(&fields, "forbids"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "example" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id",
                    "path",
                    "commands",
                    "expected_receipts",
                    "expected_verdict",
                    "status",
                ],
                "example",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_examples.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceExample,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap operator example {id}"),
                ));
            }
            examples.push(BootstrapOperatorExampleBinding {
                line_number,
                id,
                path: field(&fields, "path"),
                commands: list_field(&fields, "commands"),
                expected_receipts: list_field(&fields, "expected_receipts"),
                expected_verdict: field(&fields, "expected_verdict"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "gate" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &[
                    "id",
                    "workflow",
                    "required_receipts",
                    "required_examples",
                    "decision",
                    "forbids",
                    "status",
                ],
                "gate",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_gates.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateReviewGate,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap operator gate {id}"),
                ));
            }
            gates.push(BootstrapOperatorAcceptanceGateBinding {
                line_number,
                id,
                workflow: field(&fields, "workflow"),
                required_receipts: list_field(&fields, "required_receipts"),
                required_examples: list_field(&fields, "required_examples"),
                decision: field(&fields, "decision"),
                forbids: list_field(&fields, "forbids"),
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
                    "scope",
                    "commands",
                    "workflows",
                    "examples",
                    "gates",
                    "receipts",
                    "forbids",
                    "status",
                ],
                "proof",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_proofs.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceProof,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap operator proof {id}"),
                ));
            }
            proofs.push(BootstrapOperatorProofBinding {
                line_number,
                id,
                scope: field(&fields, "scope"),
                commands: list_field(&fields, "commands"),
                workflows: list_field(&fields, "workflows"),
                examples: list_field(&fields, "examples"),
                gates: list_field(&fields, "gates"),
                receipts: list_field(&fields, "receipts"),
                forbids: list_field(&fields, "forbids"),
                status: field(&fields, "status"),
            });
            continue;
        }
        if left == "artifact" {
            let fields = parse_pipe_fields(value);
            require_fields(
                &fields,
                &["id", "owner", "path", "kind", "commands", "status"],
                "artifact",
                line_number,
                &mut errors,
            );
            let id = field(&fields, "id");
            if !is_symbolic_name(&id) || !seen_artifacts.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateDeliveryArtifact,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid bootstrap operator artifact {id}"),
                ));
            }
            artifacts.push(BootstrapOperatorArtifactBinding {
                line_number,
                id,
                owner_root: field(&fields, "owner"),
                path: field(&fields, "path"),
                artifact_kind: field(&fields, "kind"),
                commands: list_field(&fields, "commands"),
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
                format!("unknown bootstrap operator interface line {line}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(BootstrapOperatorInterfaceSurface {
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
        commands,
        workflows,
        examples,
        gates,
        proofs,
        artifacts,
    })
}

pub fn validate_bootstrap_operator_interface_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_bootstrap_operator_interface_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_bootstrap_operator_interface(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_operator_interface_model(
    surface: &BootstrapOperatorInterfaceSurface,
) -> Verdict {
    let mut errors = Vec::new();
    validate_bootstrap_operator_interface(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_bootstrap_operator_interface(
    surface: &BootstrapOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P02 got {}", surface.phase),
        ));
    }
    if surface.task != "P02-018" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P02-018 got {}", surface.task),
        ));
    }
    validate_status("surface", "P02-018", 0, &surface.status, errors);

    for rule in REQUIRED_BOOTSTRAP_OPERATOR_INTERFACE_RULES {
        match surface.rules.get(*rule) {
            Some(value) if value == "required" || value == "forbidden" => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceRule,
                format!("rule:{rule}"),
                format!("expected required/forbidden got {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceRule,
                format!("rule:{rule}"),
                "missing bootstrap operator interface rule",
            )),
        }
    }

    require_ids(
        "command",
        REQUIRED_BOOTSTRAP_OPERATOR_COMMANDS,
        surface
            .commands
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingInterfaceCommand,
        errors,
    );
    require_ids(
        "workflow",
        REQUIRED_BOOTSTRAP_OPERATOR_WORKFLOWS,
        surface
            .workflows
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingInterfaceWorkflow,
        errors,
    );
    require_ids(
        "example",
        REQUIRED_BOOTSTRAP_OPERATOR_EXAMPLES,
        surface
            .examples
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingInterfaceExample,
        errors,
    );
    require_ids(
        "gate",
        REQUIRED_BOOTSTRAP_OPERATOR_GATES,
        surface.gates.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingReviewGate,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_BOOTSTRAP_OPERATOR_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingInterfaceProof,
        errors,
    );
    require_ids(
        "artifact",
        REQUIRED_BOOTSTRAP_OPERATOR_ARTIFACTS,
        surface
            .artifacts
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingDeliveryArtifact,
        errors,
    );

    let command_ids: BTreeSet<&str> = surface
        .commands
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let workflow_ids: BTreeSet<&str> = surface
        .workflows
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let example_ids: BTreeSet<&str> = surface
        .examples
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    let gate_ids: BTreeSet<&str> = surface.gates.iter().map(|item| item.id.as_str()).collect();

    for command in &surface.commands {
        validate_status(
            "command",
            &command.id,
            command.line_number,
            &command.status,
            errors,
        );
        if !command.binary.starts_with("lyra-p02-")
            || command.input.contains("..")
            || command.output.contains("..")
            || !command.output.starts_with("receipts/p02/")
            || command.receipts.is_empty()
            || command.roles.is_empty()
            || command.targets.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                format!("line:{:03}", command.line_number),
                format!("command {} has invalid command binding", command.id),
            ));
        }
        for role in &command.roles {
            if !ALLOWED_ROLES.contains(&role.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceCommand,
                    format!("line:{:03}", command.line_number),
                    format!("command {} has invalid role {}", command.id, role),
                ));
            }
        }
        for target in &command.targets {
            if !ALLOWED_TARGETS.contains(&target.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceCommand,
                    format!("line:{:03}", command.line_number),
                    format!("command {} has invalid target {}", command.id, target),
                ));
            }
        }
        let Some(descriptor) = bootstrap_operator_command_descriptor(&command.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                format!("line:{:03}", command.line_number),
                format!("unknown bootstrap operator command {}", command.id),
            ));
            continue;
        };
        if command.binary != descriptor.binary
            || command.surface != descriptor.surface
            || command.input != descriptor.input
            || command.output != descriptor.output
            || command.receipts
                != descriptor
                    .receipts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || command.roles
                != descriptor
                    .roles
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || command.targets
                != descriptor
                    .targets
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || command.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", command.line_number),
                format!("command descriptor drift {}", command.id),
            ));
        }
        if bootstrap_operator_command_digest(&command.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                format!("line:{:03}", command.line_number),
                format!("command {} is not digestible", command.id),
            ));
        }
    }

    for workflow in &surface.workflows {
        validate_status(
            "workflow",
            &workflow.id,
            workflow.line_number,
            &workflow.status,
            errors,
        );
        if workflow.commands.is_empty()
            || workflow.targets.is_empty()
            || workflow.examples.is_empty()
            || workflow.forbids.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                format!("line:{:03}", workflow.line_number),
                format!(
                    "workflow {} has empty command/target/example binding",
                    workflow.id
                ),
            ));
        }
        for command_id in &workflow.commands {
            if !command_ids.contains(command_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", workflow.line_number),
                    format!(
                        "workflow {} references unknown command {}",
                        workflow.id, command_id
                    ),
                ));
            }
        }
        for target in &workflow.targets {
            if !ALLOWED_TARGETS.contains(&target.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceWorkflow,
                    format!("line:{:03}", workflow.line_number),
                    format!("workflow {} has invalid target {}", workflow.id, target),
                ));
            }
        }
        let Some(descriptor) = bootstrap_operator_workflow_descriptor(&workflow.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                format!("line:{:03}", workflow.line_number),
                format!("unknown bootstrap operator workflow {}", workflow.id),
            ));
            continue;
        };
        if workflow.order != descriptor.order
            || workflow.commands
                != descriptor
                    .commands
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || workflow.targets
                != descriptor
                    .targets
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || workflow.examples
                != descriptor
                    .examples
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || workflow.forbids
                != descriptor
                    .forbids
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || workflow.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", workflow.line_number),
                format!("workflow descriptor drift {}", workflow.id),
            ));
        }
        if bootstrap_operator_workflow_digest(&workflow.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                format!("line:{:03}", workflow.line_number),
                format!("workflow {} is not digestible", workflow.id),
            ));
        }
    }

    for example in &surface.examples {
        validate_status(
            "example",
            &example.id,
            example.line_number,
            &example.status,
            errors,
        );
        if !example.path.starts_with("examples/p02/")
            || example.path.contains("..")
            || example.commands.is_empty()
            || example.expected_receipts.is_empty()
            || example.expected_verdict != "accepted"
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceExample,
                format!("line:{:03}", example.line_number),
                format!("example {} has invalid example binding", example.id),
            ));
        }
        for command_id in &example.commands {
            if !command_ids.contains(command_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", example.line_number),
                    format!(
                        "example {} references unknown command {}",
                        example.id, command_id
                    ),
                ));
            }
        }
        let Some(descriptor) = bootstrap_operator_example_descriptor(&example.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceExample,
                format!("line:{:03}", example.line_number),
                format!("unknown bootstrap operator example {}", example.id),
            ));
            continue;
        };
        if example.path != descriptor.path
            || example.commands
                != descriptor
                    .commands
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || example.expected_receipts
                != descriptor
                    .expected_receipts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || example.expected_verdict != descriptor.expected_verdict
            || example.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", example.line_number),
                format!("example descriptor drift {}", example.id),
            ));
        }
        if bootstrap_operator_example_digest(&example.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceExample,
                format!("line:{:03}", example.line_number),
                format!("example {} is not digestible", example.id),
            ));
        }
    }

    for gate in &surface.gates {
        validate_status("gate", &gate.id, gate.line_number, &gate.status, errors);
        if !workflow_ids.contains(gate.workflow.as_str())
            || gate.required_receipts.is_empty()
            || gate.required_examples.is_empty()
            || !ALLOWED_DECISIONS.contains(&gate.decision.as_str())
            || gate.forbids.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReviewGate,
                format!("line:{:03}", gate.line_number),
                format!("gate {} has invalid acceptance binding", gate.id),
            ));
        }
        for example_id in &gate.required_examples {
            if !example_ids.contains(example_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", gate.line_number),
                    format!("gate {} references unknown example {}", gate.id, example_id),
                ));
            }
        }
        let Some(descriptor) = bootstrap_operator_gate_descriptor(&gate.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReviewGate,
                format!("line:{:03}", gate.line_number),
                format!("unknown bootstrap operator gate {}", gate.id),
            ));
            continue;
        };
        if gate.workflow != descriptor.workflow
            || gate.required_receipts
                != descriptor
                    .required_receipts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || gate.required_examples
                != descriptor
                    .required_examples
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || gate.decision != descriptor.decision
            || gate.forbids
                != descriptor
                    .forbids
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || gate.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", gate.line_number),
                format!("gate descriptor drift {}", gate.id),
            ));
        }
        if bootstrap_operator_gate_digest(&gate.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidReviewGate,
                format!("line:{:03}", gate.line_number),
                format!("gate {} is not digestible", gate.id),
            ));
        }
    }

    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str())
            || proof.commands.is_empty()
            || proof.workflows.is_empty()
            || proof.examples.is_empty()
            || proof.gates.is_empty()
            || proof.receipts.is_empty()
            || proof.forbids.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} has invalid proof binding", proof.id),
            ));
        }
        for command_id in &proof.commands {
            if !command_ids.contains(command_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!(
                        "proof {} references unknown command {}",
                        proof.id, command_id
                    ),
                ));
            }
        }
        for workflow_id in &proof.workflows {
            if !workflow_ids.contains(workflow_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!(
                        "proof {} references unknown workflow {}",
                        proof.id, workflow_id
                    ),
                ));
            }
        }
        for example_id in &proof.examples {
            if !example_ids.contains(example_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!(
                        "proof {} references unknown example {}",
                        proof.id, example_id
                    ),
                ));
            }
        }
        for gate_id in &proof.gates {
            if !gate_ids.contains(gate_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown gate {}", proof.id, gate_id),
                ));
            }
        }
        let Some(descriptor) = bootstrap_operator_proof_descriptor(&proof.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceProof,
                format!("line:{:03}", proof.line_number),
                format!("unknown bootstrap operator proof {}", proof.id),
            ));
            continue;
        };
        if proof.scope != descriptor.scope
            || proof.commands
                != descriptor
                    .commands
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.workflows
                != descriptor
                    .workflows
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.examples
                != descriptor
                    .examples
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.gates
                != descriptor
                    .gates
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.receipts
                != descriptor
                    .receipts
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.forbids
                != descriptor
                    .forbids
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || proof.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", proof.line_number),
                format!("proof descriptor drift {}", proof.id),
            ));
        }
        if bootstrap_operator_proof_digest(&proof.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} is not digestible", proof.id),
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
        if !ALLOWED_OWNER_ROOTS.contains(&artifact.owner_root.as_str())
            || artifact.path.contains("..")
            || artifact.path.is_empty()
            || artifact.commands.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} has invalid artifact binding", artifact.id),
            ));
        }
        for command_id in &artifact.commands {
            if !command_ids.contains(command_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", artifact.line_number),
                    format!(
                        "artifact {} references unknown command {}",
                        artifact.id, command_id
                    ),
                ));
            }
        }
        let Some(descriptor) = bootstrap_operator_artifact_descriptor(&artifact.id) else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                format!("line:{:03}", artifact.line_number),
                format!("unknown bootstrap operator artifact {}", artifact.id),
            ));
            continue;
        };
        if artifact.owner_root != descriptor.owner_root
            || artifact.path != descriptor.path
            || artifact.artifact_kind != descriptor.artifact_kind
            || artifact.commands
                != descriptor
                    .commands
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
            || artifact.status != descriptor.status
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", artifact.line_number),
                format!("artifact descriptor drift {}", artifact.id),
            ));
        }
        if bootstrap_operator_artifact_digest(&artifact.id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} is not digestible", artifact.id),
            ));
        }
    }

    let suite = deterministic_bootstrap_operator_interface_suite_report(
        &surface
            .commands
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.binary.clone(),
                    item.surface.clone(),
                    item.input.clone(),
                    item.output.clone(),
                    item.receipts.clone(),
                    item.roles.clone(),
                    item.targets.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .workflows
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.order.clone(),
                    item.commands.clone(),
                    item.targets.clone(),
                    item.examples.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .examples
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.path.clone(),
                    item.commands.clone(),
                    item.expected_receipts.clone(),
                    item.expected_verdict.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .gates
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.workflow.clone(),
                    item.required_receipts.clone(),
                    item.required_examples.clone(),
                    item.decision.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .proofs
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.scope.clone(),
                    item.commands.clone(),
                    item.workflows.clone(),
                    item.examples.clone(),
                    item.gates.clone(),
                    item.receipts.clone(),
                    item.forbids.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
        &surface
            .artifacts
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.owner_root.clone(),
                    item.path.clone(),
                    item.artifact_kind.clone(),
                    item.commands.clone(),
                    item.status.clone(),
                )
            })
            .collect::<Vec<_>>(),
    );
    if suite.command_count != REQUIRED_BOOTSTRAP_OPERATOR_COMMANDS.len()
        || suite.workflow_count != REQUIRED_BOOTSTRAP_OPERATOR_WORKFLOWS.len()
        || suite.example_count != REQUIRED_BOOTSTRAP_OPERATOR_EXAMPLES.len()
        || suite.gate_count != REQUIRED_BOOTSTRAP_OPERATOR_GATES.len()
        || suite.proof_count != REQUIRED_BOOTSTRAP_OPERATOR_PROOFS.len()
        || suite.artifact_count != REQUIRED_BOOTSTRAP_OPERATOR_ARTIFACTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InterfaceDriftAccepted,
            "deterministic_report",
            "bootstrap operator interface deterministic report count drift",
        ));
    }
    if bootstrap_operator_registry_hash().is_empty()
        || !bootstrap_operator_carrier_signature()
            .starts_with(LYRA_P02_BOOTSTRAP_OPERATOR_INTERFACE_CARRIER)
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidInterfaceProof,
            "registry",
            "bootstrap operator interface registry is not hashable",
        ));
    }
    if !bootstrap_operator_workflows_bind_known_commands()
        || !bootstrap_operator_examples_bind_known_commands()
        || !bootstrap_operator_gates_bind_registry()
        || !bootstrap_operator_proofs_bind_registry()
        || !bootstrap_operator_artifacts_bind_commands()
        || !bootstrap_operator_commands_cover_p02_018()
        || !bootstrap_operator_no_forbidden_descriptor_claims()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InterfaceProofUnbound,
            "registry",
            "bootstrap operator interface registry binding check failed",
        ));
    }
    if bootstrap_operator_command_ids().len() != REQUIRED_BOOTSTRAP_OPERATOR_COMMANDS.len()
        || bootstrap_operator_workflow_ids().len() != REQUIRED_BOOTSTRAP_OPERATOR_WORKFLOWS.len()
        || bootstrap_operator_example_ids().len() != REQUIRED_BOOTSTRAP_OPERATOR_EXAMPLES.len()
        || bootstrap_operator_gate_ids().len() != REQUIRED_BOOTSTRAP_OPERATOR_GATES.len()
        || bootstrap_operator_proof_ids().len() != REQUIRED_BOOTSTRAP_OPERATOR_PROOFS.len()
        || bootstrap_operator_artifact_ids().len() != REQUIRED_BOOTSTRAP_OPERATOR_ARTIFACTS.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InterfaceDriftAccepted,
            "registry",
            "bootstrap operator interface registry size drift",
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
    names: &[&str],
    kind: &str,
    line_number: usize,
    errors: &mut Vec<ValidationError>,
) {
    for name in names {
        if !fields.contains_key(*name)
            || fields
                .get(*name)
                .map(|value| value.is_empty())
                .unwrap_or(true)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("{kind} missing field {name}"),
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
                .collect()
        })
        .unwrap_or_default()
}

fn require_ids(
    kind: &str,
    required: &[&str],
    actual: Vec<&str>,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    let actual: BTreeSet<&str> = actual.into_iter().collect();
    for id in required {
        if !actual.contains(id) {
            errors.push(ValidationError::reject(
                code,
                format!("{kind}:{id}"),
                format!("missing required bootstrap operator {kind} {id}"),
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
        let location = if line_number == 0 {
            kind.to_string()
        } else {
            format!("line:{line_number:03}")
        };
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedEvidenceClaim,
            location,
            format!("{kind} {id} has invalid status {status}"),
        ));
    }
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && value.as_bytes()[0].is_ascii_lowercase()
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lower = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_BOOTSTRAP_OPERATOR_TEXT {
        if lower.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "bootstrap_operator_interface_text",
                format!("forbidden bootstrap operator interface text {needle}"),
            ));
        }
    }
}
