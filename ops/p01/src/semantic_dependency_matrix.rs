use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_semantic_dependency_matrix::deterministic_semantic_dependency_matrix_report;
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p01_semantic_dependency_matrix_model::{
    SemanticBlockerBinding, SemanticDependencyMatrixSurface, SemanticDependencyNode,
    SemanticParallelLane,
};

pub const P01_SEMANTIC_DEPENDENCY_MATRIX_CONTRACT: &str = "LYRA-P01-SEMANTIC-DEPENDENCY-MATRIX v1";

pub const REQUIRED_SEMANTIC_DEPENDENCY_RULES: &[&str] = &[
    "semantic_dependency_matrix_must_cover_all_p01_tasks",
    "semantic_dependency_matrix_must_cover_all_p01_closure_outputs",
    "semantic_blocker_matrix_must_bind_open_outputs",
    "semantic_parallelization_map_must_preserve_order",
    "semantic_dependency_matrix_must_bind_receipts",
    "no_cycle_in_semantic_dependencies",
    "no_network_dependency",
    "no_docs_only_matrix",
    "no_unreceipted_matrix",
    "no_global_closure_claim",
];

pub const REQUIRED_SEMANTIC_DEPENDENCY_NODES: &[&str] = &[
    "P01-001", "P01-002", "P01-003", "P01-004", "P01-005", "P01-006", "P01-007", "P01-008",
    "P01-009", "P01-010", "P01-011", "P01-012", "P01-013", "P01-014", "P01-015", "P01-016",
    "P01-017", "P01-018", "P01-019", "P01-020", "P01-021", "P01-022", "P01-023", "P01-024",
    "P01-X01", "P01-X02", "P01-X03", "P01-X04", "P01-X05",
];

pub const REQUIRED_SEMANTIC_BLOCKERS: &[&str] = &[
    "local_validation_evidence",
    "P01-X02",
    "P01-X03",
    "P01-X04",
    "P01-X05",
    "P01-GLOBAL",
];
pub const REQUIRED_SEMANTIC_PARALLEL_LANES: &[&str] = &[
    "ontology_ir_chain",
    "proof_engine_chain",
    "operator_release_surfaces",
    "ecosystem_public_interest",
    "closure_output_chain",
];

const ALLOWED_STATUSES: &[&str] = &[
    "working_slice",
    "artifact_emitted",
    "execution_proven",
    "bounded_closed",
    "blocked",
];
const ALLOWED_NODE_KINDS: &[&str] = &["primary", "execution", "closure_output"];
const ALLOWED_LANE_KINDS: &[&str] = &["serial", "parallel", "release"];
const ALLOWED_OWNER_ROOTS: &[&str] = &[
    "k0",
    "k1",
    "lyralang",
    "shells",
    "interfaces",
    "ops",
    "slices",
    "products",
];

pub fn parse_semantic_dependency_matrix_surface(
    input: &str,
) -> Result<SemanticDependencyMatrixSurface, Vec<ValidationError>> {
    let lines = match canonical_lines(input) {
        Ok(lines) => lines,
        Err(error) => {
            return Err(vec![ValidationError::reject(
                ErrorCode::CanonicalControlByte,
                "input",
                format!("canonicalization failed: {error:?}"),
            )])
        }
    };
    if lines.is_empty() {
        return Err(vec![ValidationError::reject(
            ErrorCode::EmptySurface,
            "input",
            "semantic dependency matrix surface is empty",
        )]);
    }

    let header = lines[0].clone();
    let mut errors = Vec::new();
    if header != P01_SEMANTIC_DEPENDENCY_MATRIX_CONTRACT {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P01_SEMANTIC_DEPENDENCY_MATRIX_CONTRACT}"),
        ));
    }

    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut rules = BTreeMap::new();
    let mut dependencies = Vec::new();
    let mut blockers = Vec::new();
    let mut lanes = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_dependencies = BTreeSet::new();
    let mut seen_blockers = BTreeSet::new();
    let mut seen_lanes = BTreeSet::new();

    for (index, line) in lines.iter().enumerate().skip(1) {
        let line_number = index + 1;
        let Some((left, value)) = line.split_once('=') else {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                "entry must contain exactly one key/value separator",
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
                    "semantic dependency rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(node_id) = left.strip_prefix("dependency:") {
            if !is_known_p01_frontier_id(node_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureTask,
                    format!("line:{line_number:03}"),
                    format!("invalid dependency identity {node_id}"),
                ));
                continue;
            }
            if !seen_dependencies.insert(node_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureTask,
                    format!("dependency:{node_id}"),
                    "semantic dependency identity must be unique",
                ));
                continue;
            }
            match parse_dependency(line_number, node_id, value) {
                Ok(node) => dependencies.push(node),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(blocker_id) = left.strip_prefix("blocker:") {
            if !is_known_blocker_id(blocker_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid blocker identity {blocker_id}"),
                ));
                continue;
            }
            if !seen_blockers.insert(blocker_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("blocker:{blocker_id}"),
                    "semantic blocker identity must be unique",
                ));
                continue;
            }
            match parse_blocker(line_number, blocker_id, value) {
                Ok(blocker) => blockers.push(blocker),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(lane_id) = left.strip_prefix("lane:") {
            if !is_symbolic_name(lane_id) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidClosureOutputGate,
                    format!("line:{line_number:03}"),
                    format!("invalid semantic lane identity {lane_id}"),
                ));
                continue;
            }
            if !seen_lanes.insert(lane_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("lane:{lane_id}"),
                    "semantic lane identity must be unique",
                ));
                continue;
            }
            match parse_lane(line_number, lane_id, value) {
                Ok(lane) => lanes.push(lane),
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
                format!("unknown semantic dependency matrix key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(SemanticDependencyMatrixSurface {
        header,
        phase: phase.unwrap_or_default(),
        task: task.unwrap_or_default(),
        status: status.unwrap_or_default(),
        rules,
        dependencies,
        blockers,
        lanes,
    })
}

pub fn validate_semantic_dependency_matrix_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    let parsed = match parse_semantic_dependency_matrix_surface(input) {
        Ok(surface) => surface,
        Err(parse_errors) => {
            errors.extend(parse_errors);
            let verdict = Verdict::rejected(errors);
            let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
            return (verdict, receipt);
        }
    };

    if parsed.phase != "P01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "semantic dependency matrix must bind phase P01",
        ));
    }
    if parsed.task != "P01-X01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "semantic dependency matrix must bind task P01-X01",
        ));
    }
    if parsed.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "P01-X01 must be artifact_emitted",
        ));
    }

    for required in REQUIRED_SEMANTIC_DEPENDENCY_RULES {
        if !parsed.rules.contains_key(*required) {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{required}"),
                "missing required semantic dependency matrix rule",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_DEPENDENCY_NODES {
        if parsed.dependency_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureTask,
                format!("dependency:{required}"),
                "missing required P01 dependency node",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_BLOCKERS {
        if parsed.blocker_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("blocker:{required}"),
                "missing required semantic blocker binding",
            ));
        }
    }
    for required in REQUIRED_SEMANTIC_PARALLEL_LANES {
        if parsed.lane_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("lane:{required}"),
                "missing required semantic parallel lane",
            ));
        }
    }

    validate_references(&parsed, &mut errors);
    validate_no_cycles(&parsed, &mut errors);

    let dependencies = parsed
        .dependencies
        .iter()
        .map(|node| {
            (
                node.id.clone(),
                node.node_kind.clone(),
                node.owner_roots.clone(),
                node.depends.clone(),
                node.unlocks.clone(),
                node.receipts.clone(),
                node.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let blockers = parsed
        .blockers
        .iter()
        .map(|blocker| {
            (
                blocker.id.clone(),
                blocker.blocked_by.clone(),
                blocker.reason.clone(),
                blocker.unblocks.clone(),
                blocker.receipts.clone(),
                blocker.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let lanes = parsed
        .lanes
        .iter()
        .map(|lane| {
            (
                lane.id.clone(),
                lane.lane_kind.clone(),
                lane.frontiers.clone(),
                lane.after.clone(),
                lane.can_parallel_with.clone(),
                lane.receipts.clone(),
                lane.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let report = deterministic_semantic_dependency_matrix_report(&dependencies, &blockers, &lanes);
    if report.emitted_output_count != 1 {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureOutputPremature,
            "semantic_dependency_matrix",
            "exactly P01-X01 may be artifact_emitted among P01 closure outputs",
        ));
    }
    if report.open_blocker_count == 0 {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBlockerBinding,
            "semantic_dependency_matrix",
            "matrix must preserve at least one open blocker until local validation evidence exists",
        ));
    }

    let verdict = if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    };
    let receipt = build_phase_receipt("P01", input, &canonical, verdict.clone());
    (verdict, receipt)
}

fn parse_dependency(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticDependencyNode, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureTask,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let node_kind =
        required_field(&fields, "kind", line_number, ErrorCode::InvalidClosureTask)?.to_string();
    let owner_roots = split_list(required_field(
        &fields,
        "roots",
        line_number,
        ErrorCode::InvalidOwnerRoot,
    )?);
    let depends = split_list(required_field(
        &fields,
        "depends",
        line_number,
        ErrorCode::InvalidClosureTask,
    )?);
    let unlocks = split_list(required_field(
        &fields,
        "unlocks",
        line_number,
        ErrorCode::InvalidClosureTask,
    )?);
    let receipts = split_list(required_field(
        &fields,
        "receipts",
        line_number,
        ErrorCode::ClosureUnreceipted,
    )?);
    let status = required_field(
        &fields,
        "status",
        line_number,
        ErrorCode::UnsupportedClosureStatus,
    )?
    .to_string();

    if !ALLOWED_NODE_KINDS.contains(&node_kind.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureTask,
            format!("dependency:{id}"),
            format!("unsupported semantic dependency kind {node_kind}"),
        ));
    }
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("dependency:{id}"),
            format!("unsupported semantic dependency status {status}"),
        ));
    }
    if owner_roots.is_empty()
        || owner_roots
            .iter()
            .any(|root| !ALLOWED_OWNER_ROOTS.contains(&root.as_str()))
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidOwnerRoot,
            format!("dependency:{id}"),
            "semantic dependency owner roots must be explicit owner roots",
        ));
    }
    if receipts.is_empty() {
        return Err(ValidationError::reject(
            ErrorCode::ClosureUnreceipted,
            format!("dependency:{id}"),
            "semantic dependency node must bind at least one receipt",
        ));
    }

    Ok(SemanticDependencyNode {
        line_number,
        id: id.to_string(),
        node_kind,
        owner_roots,
        depends,
        unlocks,
        receipts,
        status,
    })
}

fn parse_blocker(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticBlockerBinding, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let blocked_by = split_list(required_field(
        &fields,
        "blocked_by",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?);
    let reason = required_field(
        &fields,
        "reason",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?
    .to_string();
    let unblocks = split_list(required_field(
        &fields,
        "unblocks",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?);
    let receipts = split_list(required_field(
        &fields,
        "receipts",
        line_number,
        ErrorCode::ClosureUnreceipted,
    )?);
    let status = required_field(
        &fields,
        "status",
        line_number,
        ErrorCode::UnsupportedClosureStatus,
    )?
    .to_string();
    if blocked_by.is_empty() || reason.is_empty() || unblocks.is_empty() || receipts.is_empty() {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("blocker:{id}"),
            "blocker must bind sources, reason, unblocks, and receipts",
        ));
    }
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("blocker:{id}"),
            format!("unsupported semantic blocker status {status}"),
        ));
    }
    Ok(SemanticBlockerBinding {
        line_number,
        id: id.to_string(),
        blocked_by,
        reason,
        unblocks,
        receipts,
        status,
    })
}

fn parse_lane(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<SemanticParallelLane, ValidationError> {
    let fields = parse_fields(value).map_err(|detail| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            detail,
        )
    })?;
    let lane_kind = required_field(
        &fields,
        "kind",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?
    .to_string();
    let frontiers = split_list(required_field(
        &fields,
        "frontiers",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?);
    let after = split_list(required_field(
        &fields,
        "after",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?);
    let can_parallel_with = split_list(required_field(
        &fields,
        "parallel",
        line_number,
        ErrorCode::InvalidClosureOutputGate,
    )?);
    let receipts = split_list(required_field(
        &fields,
        "receipts",
        line_number,
        ErrorCode::ClosureUnreceipted,
    )?);
    let status = required_field(
        &fields,
        "status",
        line_number,
        ErrorCode::UnsupportedClosureStatus,
    )?
    .to_string();
    if !ALLOWED_LANE_KINDS.contains(&lane_kind.as_str())
        || frontiers.is_empty()
        || receipts.is_empty()
    {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("lane:{id}"),
            "semantic lane must bind kind, frontiers, and receipts",
        ));
    }
    if !ALLOWED_STATUSES.contains(&status.as_str()) {
        return Err(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            format!("lane:{id}"),
            format!("unsupported semantic lane status {status}"),
        ));
    }
    Ok(SemanticParallelLane {
        line_number,
        id: id.to_string(),
        lane_kind,
        frontiers,
        after,
        can_parallel_with,
        receipts,
        status,
    })
}

fn parse_fields(value: &str) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    for segment in value.split(';') {
        let Some((key, field_value)) = segment.split_once(':') else {
            return Err("field segment must contain a key/value separator".to_string());
        };
        if key.is_empty()
            || field_value.is_empty()
            || key != key.trim()
            || field_value != field_value.trim()
        {
            return Err("field segment sides must be non-empty and trimmed".to_string());
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

fn required_field<'a>(
    fields: &'a BTreeMap<String, String>,
    key: &str,
    line_number: usize,
    code: ErrorCode,
) -> Result<&'a str, ValidationError> {
    fields.get(key).map(String::as_str).ok_or_else(|| {
        ValidationError::reject(
            code,
            format!("line:{line_number:03}"),
            format!("missing required field {key}"),
        )
    })
}

fn split_list(value: &str) -> Vec<String> {
    if value == "none" {
        return Vec::new();
    }
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_known_p01_frontier_id(value: &str) -> bool {
    REQUIRED_SEMANTIC_DEPENDENCY_NODES.contains(&value)
}

fn is_known_blocker_id(value: &str) -> bool {
    REQUIRED_SEMANTIC_BLOCKERS.contains(&value)
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    for (token, code, detail) in [
        (
            "network_required:true",
            ErrorCode::ClosureNetworkDependency,
            "semantic dependency matrix cannot require network access",
        ),
        (
            "remote_service_required:true",
            ErrorCode::ClosureNetworkDependency,
            "semantic dependency matrix cannot require remote services",
        ),
        (
            "docs_only:true",
            ErrorCode::ClosureDocsOnly,
            "semantic dependency matrix cannot be documentation only",
        ),
        (
            "unreceipted:true",
            ErrorCode::ClosureUnreceipted,
            "semantic dependency matrix cannot accept unreceipted outputs",
        ),
        (
            "global_closure:true",
            ErrorCode::UnsupportedGlobalClosure,
            "P01-X01 cannot claim global closure",
        ),
        (
            "phase_closure:true",
            ErrorCode::UnsupportedGlobalClosure,
            "P01-X01 cannot close P01 globally",
        ),
    ] {
        if canonical.contains(token) {
            errors.push(ValidationError::reject(code, "forbidden_text", detail));
        }
    }
}

fn validate_references(
    surface: &SemanticDependencyMatrixSurface,
    errors: &mut Vec<ValidationError>,
) {
    let known_nodes = surface
        .dependencies
        .iter()
        .map(|node| node.id.as_str())
        .collect::<BTreeSet<_>>();
    for node in &surface.dependencies {
        for reference in node.depends.iter().chain(node.unlocks.iter()) {
            if !known_nodes.contains(reference.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    format!("dependency:{}", node.id),
                    format!("unknown dependency reference {reference}"),
                ));
            }
        }
    }
    for blocker in &surface.blockers {
        for reference in blocker.unblocks.iter() {
            if !known_nodes.contains(reference.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    format!("blocker:{}", blocker.id),
                    format!("unknown unblock reference {reference}"),
                ));
            }
        }
    }
    for lane in &surface.lanes {
        for reference in lane.frontiers.iter().chain(lane.can_parallel_with.iter()) {
            if !known_nodes.contains(reference.as_str())
                && !REQUIRED_SEMANTIC_PARALLEL_LANES.contains(&reference.as_str())
            {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    format!("lane:{}", lane.id),
                    format!("unknown lane frontier reference {reference}"),
                ));
            }
        }
    }
}

fn validate_no_cycles(
    surface: &SemanticDependencyMatrixSurface,
    errors: &mut Vec<ValidationError>,
) {
    let mut graph = BTreeMap::<String, Vec<String>>::new();
    for node in &surface.dependencies {
        graph.insert(node.id.clone(), node.depends.clone());
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for node in graph.keys() {
        if visits_cycle(node, &graph, &mut visiting, &mut visited) {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureDriftAccepted,
                "semantic_dependency_graph",
                "dependency cycle detected",
            ));
            return;
        }
    }
}

fn visits_cycle(
    node: &str,
    graph: &BTreeMap<String, Vec<String>>,
    visiting: &mut BTreeSet<String>,
    visited: &mut BTreeSet<String>,
) -> bool {
    if visited.contains(node) {
        return false;
    }
    if !visiting.insert(node.to_string()) {
        return true;
    }
    if let Some(dependencies) = graph.get(node) {
        for dependency in dependencies {
            if graph.contains_key(dependency) && visits_cycle(dependency, graph, visiting, visited)
            {
                return true;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node.to_string());
    false
}
