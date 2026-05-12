use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_interface::deterministic_semantic_interface_suite_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_semantic_interface::{
    semantic_interface_artifact_descriptor, semantic_interface_artifact_digest,
    semantic_interface_artifacts_bind_paths, semantic_interface_command_descriptor,
    semantic_interface_command_digest, semantic_interface_commands_cover_p01_001_through_p01_018,
    semantic_interface_example_descriptor, semantic_interface_example_digest,
    semantic_interface_examples_bind_known_commands,
    semantic_interface_no_forbidden_descriptor_claims, semantic_interface_proof_descriptor,
    semantic_interface_proof_digest, semantic_interface_proofs_bind_registry,
    semantic_interface_registry_hash, semantic_interface_workflow_descriptor,
    semantic_interface_workflow_digest, semantic_interface_workflows_bind_known_commands,
};
use crate::p01_semantic_interface_model::{
    SemanticInterfaceArtifactBinding, SemanticInterfaceCommandBinding,
    SemanticInterfaceExampleBinding, SemanticInterfaceProofBinding, SemanticInterfaceSurface,
    SemanticInterfaceWorkflowBinding,
};

pub const P01_SEMANTIC_INTERFACE_CONTRACT: &str = "LYRA-P01-SEMANTIC-INTERFACE v1";

pub const REQUIRED_SEMANTIC_INTERFACE_RULES: &[&str] = &[
    "semantic_developer_surface_required",
    "semantic_operator_surface_required",
    "command_manifest_required",
    "deterministic_receipt_emission_required",
    "negative_corpus_command_required",
    "replay_witness_command_required",
    "proof_bound_examples_required",
    "artifact_path_binding_required",
    "no_manual_only_semantic_interface",
    "no_network_required_semantic_interface",
    "no_unreceipted_operator_action",
];

pub const REQUIRED_SEMANTIC_INTERFACE_COMMANDS: &[&str] = &[
    "validate_semantic_atoms",
    "validate_core_ir",
    "validate_semantic_objects",
    "validate_semantic_identity",
    "validate_reference_semantics",
    "validate_symbolic_equality",
    "validate_error_challenge_evidence",
    "validate_semantic_serialization_hashing",
    "validate_semantic_adversarial_corpus",
    "validate_core_ir_reuse",
    "validate_semantic_atom_reference",
    "validate_semantic_bedrock_receipts",
    "validate_formal_semantic_constitution",
    "validate_canonical_data_model",
    "validate_semantic_core_engine",
    "validate_semantic_falsification",
    "validate_semantic_replay",
    "validate_semantic_interface",
];

pub const REQUIRED_SEMANTIC_INTERFACE_WORKFLOWS: &[&str] = &[
    "developer_local_semantic_check",
    "operator_core_ir_review",
    "negative_corpus_review",
    "replay_receipt_audit",
    "full_p01_semantic_frontier_review",
];

pub const REQUIRED_SEMANTIC_INTERFACE_EXAMPLES: &[&str] = &[
    "semantic_interface_review",
    "core_ir_operator_review",
    "semantic_core_engine_operator_review",
    "semantic_falsification_operator_review",
    "semantic_replay_operator_review",
    "negative_interface_rejection_review",
];

pub const REQUIRED_SEMANTIC_INTERFACE_PROOFS: &[&str] = &[
    "command_manifest_coverage",
    "workflow_ordering_determinism",
    "example_receipt_binding",
    "negative_rejection_interface",
    "p01_phase_open",
];

pub const REQUIRED_SEMANTIC_INTERFACE_ARTIFACTS: &[&str] = &[
    "deterministic_interface_report",
    "lyralang_interface_registry",
    "interface_model_contract",
    "interface_validator",
    "interface_operator_binary",
    "operator_review_example",
    "product_inspection_surface",
    "interface_tests",
];

const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_ROLES: &[&str] = &["developer", "operator", "red_team", "proof_auditor"];
const ALLOWED_TARGETS: &[&str] = &[
    "canonical_symbols",
    "semantic_atoms",
    "core_ir",
    "semantic_objects",
    "semantic_identity",
    "reference_semantics",
    "symbolic_equality",
    "error_challenge_evidence",
    "semantic_serialization",
    "adversarial_corpus",
    "core_ir_reuse",
    "atom_reference",
    "bedrock_receipts",
    "formal_constitution",
    "canonical_data_model",
    "semantic_core_engine",
    "falsification",
    "replay",
];
const ALLOWED_PROOF_SCOPES: &[&str] = &["command", "workflow", "example", "challenge", "phase"];
const ALLOWED_OWNER_ROOTS: &[&str] = &[
    "k0",
    "lyralang",
    "interfaces",
    "ops",
    "src",
    "docs",
    "examples",
    "products",
    "tests",
    "fixtures",
    "goldens",
    "receipts",
];

const FORBIDDEN_INTERFACE_TEXT: &[(&str, ErrorCode)] = &[
    ("manual only", ErrorCode::ManualOnlyInterface),
    ("human only", ErrorCode::ManualOnlyInterface),
    ("network required", ErrorCode::InterfaceNetworkDependency),
    ("cloud required", ErrorCode::InterfaceNetworkDependency),
    ("online required", ErrorCode::InterfaceNetworkDependency),
    (
        "interface drift accepted",
        ErrorCode::InterfaceDriftAccepted,
    ),
    (
        "unreceipted operator action",
        ErrorCode::InterfaceDriftAccepted,
    ),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_semantic_interface_surface(
    input: &str,
) -> Result<SemanticInterfaceSurface, Vec<ValidationError>> {
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
            "no semantic interface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P01_SEMANTIC_INTERFACE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_INTERFACE_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut commands = Vec::new();
    let mut workflows = Vec::new();
    let mut examples = Vec::new();
    let mut proofs = Vec::new();
    let mut artifacts = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_commands = BTreeSet::new();
    let mut seen_workflows = BTreeSet::new();
    let mut seen_examples = BTreeSet::new();
    let mut seen_proofs = BTreeSet::new();
    let mut seen_artifacts = BTreeSet::new();

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
                    "semantic interface rule names must be symbolic and unique",
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
                    format!("duplicate or invalid semantic interface command {id}"),
                ));
            }
            commands.push(SemanticInterfaceCommandBinding {
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
            if !is_symbolic_name(&id) || !seen_workflows.insert(id.clone()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceWorkflow,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic interface workflow {id}"),
                ));
            }
            workflows.push(SemanticInterfaceWorkflowBinding {
                line_number,
                id,
                order: field(&fields, "order"),
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
                    format!("duplicate or invalid semantic interface example {id}"),
                ));
            }
            examples.push(SemanticInterfaceExampleBinding {
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
                    format!("duplicate or invalid semantic interface proof {id}"),
                ));
            }
            proofs.push(SemanticInterfaceProofBinding {
                line_number,
                id,
                scope: field(&fields, "scope"),
                commands: list_field(&fields, "commands"),
                workflows: list_field(&fields, "workflows"),
                examples: list_field(&fields, "examples"),
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
                    ErrorCode::DuplicateEntry,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid semantic interface artifact {id}"),
                ));
            }
            artifacts.push(SemanticInterfaceArtifactBinding {
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
                format!("unknown semantic interface key {left}"),
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
    Ok(SemanticInterfaceSurface {
        header,
        phase: phase.unwrap(),
        task: task.unwrap(),
        status: status.unwrap(),
        rules,
        commands,
        workflows,
        examples,
        proofs,
        artifacts,
    })
}

pub fn validate_semantic_interface_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| String::new());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);
    let parsed = match parse_semantic_interface_surface(input) {
        Ok(surface) => surface,
        Err(mut parse_errors) => {
            errors.append(&mut parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };
    validate_semantic_interface(&parsed, &mut errors);
    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn validate_semantic_interface(
    surface: &SemanticInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    if surface.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            format!("expected P01 got {}", surface.phase),
        ));
    }
    if surface.task != "P01-018" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            format!("expected P01-018 got {}", surface.task),
        ));
    }
    validate_status("surface", "P01-018", 0, &surface.status, errors);

    for rule in REQUIRED_SEMANTIC_INTERFACE_RULES {
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
                "missing semantic interface rule",
            )),
        }
    }

    require_ids(
        "command",
        REQUIRED_SEMANTIC_INTERFACE_COMMANDS,
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
        REQUIRED_SEMANTIC_INTERFACE_WORKFLOWS,
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
        REQUIRED_SEMANTIC_INTERFACE_EXAMPLES,
        surface
            .examples
            .iter()
            .map(|item| item.id.as_str())
            .collect(),
        ErrorCode::MissingInterfaceExample,
        errors,
    );
    require_ids(
        "proof",
        REQUIRED_SEMANTIC_INTERFACE_PROOFS,
        surface.proofs.iter().map(|item| item.id.as_str()).collect(),
        ErrorCode::MissingInterfaceProof,
        errors,
    );
    require_ids(
        "artifact",
        REQUIRED_SEMANTIC_INTERFACE_ARTIFACTS,
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
    let mut workflow_orders = BTreeSet::new();

    for command in &surface.commands {
        validate_status(
            "command",
            &command.id,
            command.line_number,
            &command.status,
            errors,
        );
        if !command.binary.starts_with("src/bin/lyra-p01-")
            || !command.binary.ends_with(".rs")
            || command.binary.contains("..")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                format!("line:{:03}", command.line_number),
                format!("command {} binary path is invalid", command.id),
            ));
        }
        if command.surface != P01_SEMANTIC_INTERFACE_CONTRACT
            && !command.surface.starts_with("LYRA-P01-")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                format!("line:{:03}", command.line_number),
                format!("command {} surface is not P01-bound", command.id),
            ));
        }
        if !command.input.starts_with("fixtures/p01/")
            || !command.input.ends_with(".lyra")
            || command.input.contains("..")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                format!("line:{:03}", command.line_number),
                format!("command {} input path is invalid", command.id),
            ));
        }
        if !command.output.starts_with("receipts/p01/")
            || !command.output.ends_with(".receipt")
            || command.output.contains("..")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                format!("line:{:03}", command.line_number),
                format!("command {} output path is invalid", command.id),
            ));
        }
        if command.receipts.is_empty()
            || !command
                .receipts
                .iter()
                .any(|receipt| receipt == &command.output)
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                format!("line:{:03}", command.line_number),
                format!("command {} does not bind output receipt", command.id),
            ));
        }
        for receipt in &command.receipts {
            validate_receipt_path(
                receipt,
                command.line_number,
                ErrorCode::InvalidInterfaceCommand,
                errors,
            );
        }
        for role in &command.roles {
            if !ALLOWED_ROLES.contains(&role.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceCommand,
                    format!("line:{:03}", command.line_number),
                    format!("command {} uses invalid role {role}", command.id),
                ));
            }
        }
        for target in &command.targets {
            if !ALLOWED_TARGETS.contains(&target.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceCommand,
                    format!("line:{:03}", command.line_number),
                    format!("command {} uses invalid target {target}", command.id),
                ));
            }
        }
        if let Some(descriptor) = semantic_interface_command_descriptor(&command.id) {
            let digest = semantic_interface_command_digest(&command.id).unwrap_or_default();
            if descriptor.binary != command.binary
                || descriptor.surface != command.surface
                || descriptor.input != command.input
                || descriptor.output != command.output
                || digest.is_empty()
            {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceDriftAccepted,
                    format!("line:{:03}", command.line_number),
                    format!("command {} descriptor drift", command.id),
                ));
            }
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
        if workflow.order.len() != 3
            || !workflow.order.chars().all(|ch| ch.is_ascii_digit())
            || !workflow_orders.insert(workflow.order.clone())
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                format!("line:{:03}", workflow.line_number),
                format!("workflow {} order is invalid", workflow.id),
            ));
        }
        for command_id in &workflow.commands {
            if !command_ids.contains(command_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceWorkflow,
                    format!("line:{:03}", workflow.line_number),
                    format!(
                        "workflow {} references unknown command {command_id}",
                        workflow.id
                    ),
                ));
            }
        }
        for example_id in &workflow.examples {
            if !example_ids.contains(example_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceWorkflow,
                    format!("line:{:03}", workflow.line_number),
                    format!(
                        "workflow {} references unknown example {example_id}",
                        workflow.id
                    ),
                ));
            }
        }
        for target in &workflow.targets {
            if !ALLOWED_TARGETS.contains(&target.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceWorkflow,
                    format!("line:{:03}", workflow.line_number),
                    format!("workflow {} invalid target {target}", workflow.id),
                ));
            }
        }
        if workflow.forbids.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceWorkflow,
                format!("line:{:03}", workflow.line_number),
                format!("workflow {} has no forbidden drift binding", workflow.id),
            ));
        }
        if semantic_interface_workflow_descriptor(&workflow.id).is_some()
            && semantic_interface_workflow_digest(&workflow.id)
                .unwrap_or_default()
                .is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", workflow.line_number),
                format!("workflow {} descriptor digest empty", workflow.id),
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
        if !(example.path.starts_with("examples/p01/") || example.path.starts_with("fixtures/p01/"))
            || !example.path.ends_with(".lyra")
            || example.path.contains("..")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceExample,
                format!("line:{:03}", example.line_number),
                format!("example {} path is invalid", example.id),
            ));
        }
        for command_id in &example.commands {
            if !command_ids.contains(command_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceExample,
                    format!("line:{:03}", example.line_number),
                    format!(
                        "example {} references unknown command {command_id}",
                        example.id
                    ),
                ));
            }
        }
        if example.expected_verdict != "accepted" && example.expected_verdict != "rejected" {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceExample,
                format!("line:{:03}", example.line_number),
                format!("example {} expected verdict is invalid", example.id),
            ));
        }
        if example.expected_receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceExample,
                format!("line:{:03}", example.line_number),
                format!("example {} has no receipt binding", example.id),
            ));
        }
        for receipt in &example.expected_receipts {
            validate_receipt_path(
                receipt,
                example.line_number,
                ErrorCode::InvalidInterfaceExample,
                errors,
            );
        }
        if semantic_interface_example_descriptor(&example.id).is_some()
            && semantic_interface_example_digest(&example.id)
                .unwrap_or_default()
                .is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", example.line_number),
                format!("example {} descriptor digest empty", example.id),
            ));
        }
    }

    for proof in &surface.proofs {
        validate_status("proof", &proof.id, proof.line_number, &proof.status, errors);
        if !ALLOWED_PROOF_SCOPES.contains(&proof.scope.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} invalid scope {}", proof.id, proof.scope),
            ));
        }
        for command_id in &proof.commands {
            if !command_ids.contains(command_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown command {command_id}", proof.id),
                ));
            }
        }
        for workflow_id in &proof.workflows {
            if !workflow_ids.contains(workflow_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!(
                        "proof {} references unknown workflow {workflow_id}",
                        proof.id
                    ),
                ));
            }
        }
        for example_id in &proof.examples {
            if !example_ids.contains(example_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    format!("line:{:03}", proof.line_number),
                    format!("proof {} references unknown example {example_id}", proof.id),
                ));
            }
        }
        if proof.receipts.is_empty() || proof.forbids.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceProof,
                format!("line:{:03}", proof.line_number),
                format!("proof {} needs receipts and forbidden claims", proof.id),
            ));
        }
        for receipt in &proof.receipts {
            validate_receipt_path(
                receipt,
                proof.line_number,
                ErrorCode::InvalidInterfaceProof,
                errors,
            );
        }
        if semantic_interface_proof_descriptor(&proof.id).is_some()
            && semantic_interface_proof_digest(&proof.id)
                .unwrap_or_default()
                .is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", proof.line_number),
                format!("proof {} descriptor digest empty", proof.id),
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
            || !artifact.path.starts_with(&artifact.owner_root)
            || artifact.path.contains("..")
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidDeliveryArtifact,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} path/root mismatch", artifact.id),
            ));
        }
        for command_id in &artifact.commands {
            if !command_ids.contains(command_id.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidDeliveryArtifact,
                    format!("line:{:03}", artifact.line_number),
                    format!(
                        "artifact {} references unknown command {command_id}",
                        artifact.id
                    ),
                ));
            }
        }
        if semantic_interface_artifact_descriptor(&artifact.id).is_some()
            && semantic_interface_artifact_digest(&artifact.id)
                .unwrap_or_default()
                .is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InterfaceDriftAccepted,
                format!("line:{:03}", artifact.line_number),
                format!("artifact {} descriptor digest empty", artifact.id),
            ));
        }
    }

    if !semantic_interface_workflows_bind_known_commands()
        || !semantic_interface_examples_bind_known_commands()
        || !semantic_interface_proofs_bind_registry()
        || !semantic_interface_artifacts_bind_paths()
        || !semantic_interface_commands_cover_p01_001_through_p01_018()
        || !semantic_interface_no_forbidden_descriptor_claims()
        || semantic_interface_registry_hash().is_empty()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InterfaceDriftAccepted,
            "lyralang",
            "semantic interface registry failed binding checks",
        ));
    }

    let report = deterministic_semantic_interface_suite_report(
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
            .proofs
            .iter()
            .map(|item| {
                (
                    item.id.clone(),
                    item.scope.clone(),
                    item.commands.clone(),
                    item.workflows.clone(),
                    item.examples.clone(),
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
    if report.command_count != surface.commands.len()
        || report.workflow_count != surface.workflows.len()
        || report.example_count != surface.examples.len()
        || report.proof_count != surface.proofs.len()
        || report.artifact_count != surface.artifacts.len()
        || !report.suite_hash.starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InterfaceDriftAccepted,
            "deterministic-report",
            "semantic interface report drift",
        ));
    }
}

fn scan_forbidden_text(input: &str, errors: &mut Vec<ValidationError>) {
    let lowered = input.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_INTERFACE_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "forbidden-text",
                format!("forbidden semantic interface text {needle}"),
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
            format!("{kind}:{id}:line:{line_number:03}"),
            format!("unsupported status {status}"),
        ));
    }
}

fn validate_receipt_path(
    path: &str,
    line_number: usize,
    code: ErrorCode,
    errors: &mut Vec<ValidationError>,
) {
    if !path.starts_with("receipts/p01/") || !path.ends_with(".receipt") || path.contains("..") {
        errors.push(ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("invalid receipt path {path}"),
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
                format!("missing semantic interface {kind} {id}"),
            ));
        }
    }
}

fn parse_pipe_fields(value: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();
    for segment in value.split('|') {
        if let Some((key, item_value)) = segment.split_once(':') {
            fields.insert(key.to_string(), item_value.to_string());
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
    for item in required {
        if !fields.contains_key(*item) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("{kind} missing field {item}"),
            ));
        }
    }
}

fn field(fields: &BTreeMap<String, String>, name: &str) -> String {
    fields.get(name).cloned().unwrap_or_default()
}

fn list_field(fields: &BTreeMap<String, String>, name: &str) -> Vec<String> {
    fields
        .get(name)
        .map(|value| {
            value
                .split(',')
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
        && value.chars().next().unwrap().is_ascii_lowercase()
}
