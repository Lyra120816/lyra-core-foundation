use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_interface_manifest::deterministic_interface_manifest_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_developer_operator_model::{
    DeveloperOperatorInterfaceSurface, InterfaceCommand, InterfaceExample, InterfaceProof,
    InterfaceWorkflow,
};

pub const P00_DEVELOPER_OPERATOR_INTERFACE_CONTRACT: &str =
    "LYRA-P00-DEVELOPER-OPERATOR-INTERFACE v1";

pub const REQUIRED_INTERFACE_RULES: &[&str] = &[
    "developer_surface_required",
    "operator_surface_required",
    "command_manifest_required",
    "deterministic_usage_required",
    "proof_bound_interface_required",
    "example_bound_interface_required",
    "no_manual_only_interface",
    "no_network_required_interface",
    "phase_open_until_interfaces_proven",
];

pub const REQUIRED_INTERFACE_COMMANDS: &[&str] = &[
    "validate_constitution",
    "validate_authority_order",
    "validate_identity_law",
    "validate_enforcement_law",
    "validate_delivery_protocol",
    "validate_challenge_law",
    "validate_control_surfaces",
    "validate_owner_root_law",
    "validate_benchmark_evidence_law",
    "validate_public_interest_law",
    "validate_canon_compliance",
    "validate_acceptance_proofs",
    "validate_formal_semantics",
    "validate_canonical_model",
    "validate_deterministic_engine",
    "validate_falsification_harness",
    "validate_replay_witness",
    "validate_interface_manifest",
];

pub const REQUIRED_INTERFACE_WORKFLOWS: &[&str] = &[
    "developer_local_slice_validation",
    "operator_truth_gate_review",
    "operator_negative_challenge",
    "replay_receipt_audit",
];

pub const REQUIRED_INTERFACE_EXAMPLES: &[&str] = &[
    "developer_single_surface_check",
    "operator_phase_open_review",
    "negative_fixture_rejection",
    "receipt_replay_audit",
];

pub const REQUIRED_INTERFACE_PROOFS: &[&str] = &[
    "command_manifest_coverage",
    "workflow_determinism",
    "example_receipt_binding",
    "p00_phase_open",
];

const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "blocked",
];
const ALLOWED_ROLES: &[&str] = &["developer", "operator", "red_team", "public_interest"];
const ALLOWED_PROOF_SCOPES: &[&str] = &["command", "workflow", "example", "phase"];

const FORBIDDEN_INTERFACE_TEXT: &[(&str, ErrorCode)] = &[
    ("manual only", ErrorCode::ManualOnlyInterface),
    ("human only", ErrorCode::ManualOnlyInterface),
    ("network required", ErrorCode::InterfaceNetworkDependency),
    ("cloud required", ErrorCode::InterfaceNetworkDependency),
    ("online required", ErrorCode::InterfaceNetworkDependency),
    ("best effort", ErrorCode::PlaceholderAllowed),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    (
        "interface drift accepted",
        ErrorCode::InterfaceDriftAccepted,
    ),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
];

pub fn parse_developer_operator_interface_surface(
    input: &str,
) -> Result<DeveloperOperatorInterfaceSurface, Vec<ValidationError>> {
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
            "no developer/operator interface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_DEVELOPER_OPERATOR_INTERFACE_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_DEVELOPER_OPERATOR_INTERFACE_CONTRACT}"),
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
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_commands = BTreeSet::new();
    let mut seen_workflows = BTreeSet::new();
    let mut seen_examples = BTreeSet::new();
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
                    "interface rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(command_id) = left.strip_prefix("command:") {
            if !is_symbolic_name(command_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceCommand,
                    format!("line:{line_number:03}"),
                    format!("invalid command identity {command_id}"),
                ));
                continue;
            }
            if !seen_commands.insert(command_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceCommand,
                    format!("command:{command_id}"),
                    "command identity must be unique",
                ));
                continue;
            }
            match parse_command(line_number, command_id, value) {
                Ok(command) => commands.push(command),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(workflow_id) = left.strip_prefix("workflow:") {
            if !is_symbolic_name(workflow_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceWorkflow,
                    format!("line:{line_number:03}"),
                    format!("invalid workflow identity {workflow_id}"),
                ));
                continue;
            }
            if !seen_workflows.insert(workflow_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceWorkflow,
                    format!("workflow:{workflow_id}"),
                    "workflow identity must be unique",
                ));
                continue;
            }
            match parse_workflow(line_number, workflow_id, value) {
                Ok(workflow) => workflows.push(workflow),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(example_id) = left.strip_prefix("example:") {
            if !is_symbolic_name(example_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceExample,
                    format!("line:{line_number:03}"),
                    format!("invalid example identity {example_id}"),
                ));
                continue;
            }
            if !seen_examples.insert(example_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceExample,
                    format!("example:{example_id}"),
                    "example identity must be unique",
                ));
                continue;
            }
            match parse_example(line_number, example_id, value) {
                Ok(example) => examples.push(example),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceProof,
                    format!("line:{line_number:03}"),
                    format!("invalid proof identity {proof_id}"),
                ));
                continue;
            }
            if !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateInterfaceProof,
                    format!("proof:{proof_id}"),
                    "proof identity must be unique",
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
                format!("unknown developer/operator key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(DeveloperOperatorInterfaceSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        commands,
        workflows,
        examples,
        proofs,
    })
}

pub fn validate_developer_operator_interface_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_developer_operator_interface_surface(input) {
        Ok(surface) => errors.extend(validate_developer_operator_interface_model(&surface).errors),
        Err(parse_errors) => errors.extend(parse_errors),
    }

    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_receipt(input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_developer_operator_interface_model(
    surface: &DeveloperOperatorInterfaceSurface,
) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "developer/operator interface law must bind to P00",
        ));
    }
    if surface.task != "P00-018" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "developer/operator interface law must bind to P00-018",
        ));
    }
    if !ALLOWED_STATUSES.contains(&surface.status.as_str()) {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported interface status {}", surface.status),
        ));
    }

    require_rules(surface, &mut errors);
    require_commands(surface, &mut errors);
    require_workflows(surface, &mut errors);
    require_examples(surface, &mut errors);
    require_proofs(surface, &mut errors);
    validate_command_bindings(surface, &mut errors);
    validate_workflow_bindings(surface, &mut errors);
    validate_example_bindings(surface, &mut errors);
    validate_proof_bindings(surface, &mut errors);
    validate_manifest_report(surface, &mut errors);

    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_command(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<InterfaceCommand, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidInterfaceCommand,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let binary = required_field(
        &fields,
        "binary",
        ErrorCode::InvalidInterfaceCommand,
        line_number,
    )?;
    let surface = required_field(
        &fields,
        "surface",
        ErrorCode::InvalidInterfaceCommand,
        line_number,
    )?;
    let input = required_field(
        &fields,
        "input",
        ErrorCode::InvalidInterfaceCommand,
        line_number,
    )?;
    let output = required_field(
        &fields,
        "output",
        ErrorCode::InvalidInterfaceCommand,
        line_number,
    )?;
    let receipts = split_list(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidInterfaceCommand,
        line_number,
    )?);
    let roles = split_list(&required_field(
        &fields,
        "roles",
        ErrorCode::InvalidInterfaceCommand,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidInterfaceCommand,
        line_number,
    )?;
    if receipts.is_empty()
        || roles.is_empty()
        || !ALLOWED_STATUSES.contains(&status.as_str())
        || !is_safe_path(&input)
        || !is_safe_path(&output)
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidInterfaceCommand,
            format!("line:{line_number:03}"),
            "command must bind roles, receipts, safe input/output paths, and allowed status",
        ));
    }
    if !binary.starts_with("lyra-p00-") || !surface.ends_with("v1") {
        return Err(ValidationError::reject(
            ErrorCode::InvalidInterfaceCommand,
            format!("line:{line_number:03}"),
            "command binary and surface contract must be canonical",
        ));
    }
    if roles
        .iter()
        .any(|role| !ALLOWED_ROLES.contains(&role.as_str()))
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidInterfaceCommand,
            format!("line:{line_number:03}"),
            "command role is not admitted",
        ));
    }
    Ok(InterfaceCommand {
        line_number,
        id: id.to_string(),
        binary,
        surface,
        input,
        output,
        receipts,
        roles,
        status,
    })
}

fn parse_workflow(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<InterfaceWorkflow, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidInterfaceWorkflow,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let order = required_field(
        &fields,
        "order",
        ErrorCode::InvalidInterfaceWorkflow,
        line_number,
    )?;
    let commands = split_list(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidInterfaceWorkflow,
        line_number,
    )?);
    let roles = split_list(&required_field(
        &fields,
        "roles",
        ErrorCode::InvalidInterfaceWorkflow,
        line_number,
    )?);
    let artifacts = split_list(&required_field(
        &fields,
        "artifacts",
        ErrorCode::InvalidInterfaceWorkflow,
        line_number,
    )?);
    let forbids = split_list(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidInterfaceWorkflow,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidInterfaceWorkflow,
        line_number,
    )?;
    if !is_order_token(&order)
        || commands.is_empty()
        || roles.is_empty()
        || artifacts.is_empty()
        || forbids.is_empty()
        || !ALLOWED_STATUSES.contains(&status.as_str())
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidInterfaceWorkflow,
            format!("line:{line_number:03}"),
            "workflow must bind order, commands, roles, artifacts, forbids, and allowed status",
        ));
    }
    if roles
        .iter()
        .any(|role| !ALLOWED_ROLES.contains(&role.as_str()))
        || artifacts.iter().any(|path| !is_safe_path(path))
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidInterfaceWorkflow,
            format!("line:{line_number:03}"),
            "workflow role or artifact path is not admitted",
        ));
    }
    Ok(InterfaceWorkflow {
        line_number,
        id: id.to_string(),
        order,
        commands,
        roles,
        artifacts,
        forbids,
        status,
    })
}

fn parse_example(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<InterfaceExample, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidInterfaceExample,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let path = required_field(
        &fields,
        "path",
        ErrorCode::InvalidInterfaceExample,
        line_number,
    )?;
    let commands = split_list(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidInterfaceExample,
        line_number,
    )?);
    let expected_receipts = split_list(&required_field(
        &fields,
        "expected_receipts",
        ErrorCode::InvalidInterfaceExample,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidInterfaceExample,
        line_number,
    )?;
    if !is_safe_path(&path)
        || commands.is_empty()
        || expected_receipts.is_empty()
        || !ALLOWED_STATUSES.contains(&status.as_str())
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidInterfaceExample,
            format!("line:{line_number:03}"),
            "example must bind safe path, commands, receipts, and allowed status",
        ));
    }
    Ok(InterfaceExample {
        line_number,
        id: id.to_string(),
        path,
        commands,
        expected_receipts,
        status,
    })
}

fn parse_proof(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<InterfaceProof, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidInterfaceProof,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let scope = required_field(
        &fields,
        "scope",
        ErrorCode::InvalidInterfaceProof,
        line_number,
    )?;
    let commands = split_list(&required_field(
        &fields,
        "commands",
        ErrorCode::InvalidInterfaceProof,
        line_number,
    )?);
    let workflows = split_list(&required_field(
        &fields,
        "workflows",
        ErrorCode::InvalidInterfaceProof,
        line_number,
    )?);
    let examples = split_list(&required_field(
        &fields,
        "examples",
        ErrorCode::InvalidInterfaceProof,
        line_number,
    )?);
    let receipts = split_list(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidInterfaceProof,
        line_number,
    )?);
    let forbids = split_list(&required_field(
        &fields,
        "forbids",
        ErrorCode::InvalidInterfaceProof,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidInterfaceProof,
        line_number,
    )?;
    if !ALLOWED_PROOF_SCOPES.contains(&scope.as_str())
        || commands.is_empty()
        || workflows.is_empty()
        || examples.is_empty()
        || receipts.is_empty()
        || forbids.is_empty()
        || !ALLOWED_STATUSES.contains(&status.as_str())
    {
        return Err(ValidationError::reject(ErrorCode::InvalidInterfaceProof, format!("line:{line_number:03}"), "proof must bind admitted scope, commands, workflows, examples, receipts, forbids, and allowed status"));
    }
    Ok(InterfaceProof {
        line_number,
        id: id.to_string(),
        scope,
        commands,
        workflows,
        examples,
        receipts,
        forbids,
        status,
    })
}

fn require_rules(surface: &DeveloperOperatorInterfaceSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_INTERFACE_RULES {
        match surface.rule_value(rule) {
            Some("true") | Some("required") | Some("blocked_until_proven") => {}
            Some(other) => errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceRule,
                format!("rule:{rule}"),
                format!("interface rule must be affirmative, got {other}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceRule,
                format!("rule:{rule}"),
                "required interface rule absent",
            )),
        }
    }
}

fn require_commands(
    surface: &DeveloperOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    for id in REQUIRED_INTERFACE_COMMANDS {
        if surface.command_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceCommand,
                format!("command:{id}"),
                "required interface command absent",
            ));
        }
    }
}

fn require_workflows(
    surface: &DeveloperOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    for id in REQUIRED_INTERFACE_WORKFLOWS {
        if surface.workflow_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceWorkflow,
                format!("workflow:{id}"),
                "required interface workflow absent",
            ));
        }
    }
}

fn require_examples(
    surface: &DeveloperOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    for id in REQUIRED_INTERFACE_EXAMPLES {
        if surface.example_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceExample,
                format!("example:{id}"),
                "required interface example absent",
            ));
        }
    }
}

fn require_proofs(surface: &DeveloperOperatorInterfaceSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_INTERFACE_PROOFS {
        if surface.proof_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingInterfaceProof,
                format!("proof:{id}"),
                "required interface proof absent",
            ));
        }
    }
}

fn validate_command_bindings(
    surface: &DeveloperOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    for command in &surface.commands {
        if command
            .receipts
            .iter()
            .any(|path| !path.starts_with("receipts/p00/") || !path.ends_with(".receipt"))
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidInterfaceCommand,
                command.canonical_identity(),
                "command receipt path must bind receipts/p00/*.receipt",
            ));
        }
        if command.output.contains("stdout") || command.output.contains("manual") {
            errors.push(ValidationError::reject(
                ErrorCode::ManualOnlyInterface,
                command.canonical_identity(),
                "command output must be artifact-bound, not manual-only",
            ));
        }
    }
}

fn validate_workflow_bindings(
    surface: &DeveloperOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    let mut orders = BTreeSet::new();
    for workflow in &surface.workflows {
        if !orders.insert(workflow.order.clone()) {
            errors.push(ValidationError::reject(
                ErrorCode::DuplicateInterfaceWorkflow,
                workflow.canonical_identity(),
                "workflow order must be unique",
            ));
        }
        for command in &workflow.commands {
            if surface.command_by_id(command).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceWorkflow,
                    workflow.canonical_identity(),
                    format!("workflow references unknown command {command}"),
                ));
            }
        }
    }
}

fn validate_example_bindings(
    surface: &DeveloperOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    for example in &surface.examples {
        for command in &example.commands {
            if surface.command_by_id(command).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceExample,
                    example.canonical_identity(),
                    format!("example references unknown command {command}"),
                ));
            }
        }
        for receipt in &example.expected_receipts {
            if !receipt.starts_with("receipts/p00/") || !receipt.ends_with(".receipt") {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidInterfaceExample,
                    example.canonical_identity(),
                    format!("example receipt {receipt} is not a canonical P00 receipt path"),
                ));
            }
        }
    }
}

fn validate_proof_bindings(
    surface: &DeveloperOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    for proof in &surface.proofs {
        for command in &proof.commands {
            if surface.command_by_id(command).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    proof.canonical_identity(),
                    format!("proof references unknown command {command}"),
                ));
            }
        }
        for workflow in &proof.workflows {
            if surface.workflow_by_id(workflow).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    proof.canonical_identity(),
                    format!("proof references unknown workflow {workflow}"),
                ));
            }
        }
        for example in &proof.examples {
            if surface.example_by_id(example).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    proof.canonical_identity(),
                    format!("proof references unknown example {example}"),
                ));
            }
        }
        for receipt in &proof.receipts {
            if !receipt.starts_with("receipts/p00/") || !receipt.ends_with(".receipt") {
                errors.push(ValidationError::reject(
                    ErrorCode::InterfaceProofUnbound,
                    proof.canonical_identity(),
                    format!("proof receipt {receipt} is not canonical"),
                ));
            }
        }
        if proof.id == "p00_phase_open" && proof.status != "blocked" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "P00 phase-open proof must remain blocked until P00-024",
            ));
        }
    }
}

fn validate_manifest_report(
    surface: &DeveloperOperatorInterfaceSurface,
    errors: &mut Vec<ValidationError>,
) {
    let commands: Vec<(String, String, String, Vec<String>, Vec<String>)> = surface
        .commands
        .iter()
        .map(|command| {
            (
                command.id.clone(),
                command.binary.clone(),
                command.surface.clone(),
                command.receipts.clone(),
                command.roles.clone(),
            )
        })
        .collect();
    let report = deterministic_interface_manifest_report(
        &commands,
        surface.workflows.len(),
        surface.examples.len(),
        surface.proofs.len(),
    );
    if report.command_count != surface.commands.len()
        || report.workflow_count != surface.workflows.len()
        || report.example_count != surface.examples.len()
        || report.proof_count != surface.proofs.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InterfaceDriftAccepted,
            "interface_manifest",
            "manifest report counts drifted from parsed surface",
        ));
    }
    if report.manifest_hash.is_empty() {
        errors.push(ValidationError::reject(
            ErrorCode::InterfaceDriftAccepted,
            "interface_manifest",
            "manifest hash must be deterministic and non-empty",
        ));
    }
}

fn parse_fields(value: &str) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    for part in value.split('|') {
        let Some((key, field_value)) = part.split_once(':') else {
            return Err(format!("field {part} must contain colon"));
        };
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
        {
            return Err("field keys and values must be non-empty and trimmed".to_string());
        }
        if fields
            .insert(key.to_string(), field_value.to_string())
            .is_some()
        {
            return Err(format!("duplicate field {key}"));
        }
    }
    Ok(fields)
}

fn required_field(
    fields: &BTreeMap<String, String>,
    key: &str,
    code: ErrorCode,
    line_number: usize,
) -> Result<String, ValidationError> {
    fields.get(key).cloned().ok_or_else(|| {
        ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("missing field {key}"),
        )
    })
}

fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_order_token(value: &str) -> bool {
    value.len() == 3 && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_safe_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains("..")
        && !value.contains('\\')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-'))
}

fn scan_forbidden_text(text: &str, errors: &mut Vec<ValidationError>) {
    let lower = text.to_ascii_lowercase();
    for (token, code) in FORBIDDEN_INTERFACE_TEXT {
        if lower.contains(token) {
            errors.push(ValidationError::reject(
                *code,
                "developer_operator_interface",
                format!("forbidden interface token {token}"),
            ));
        }
    }
}
