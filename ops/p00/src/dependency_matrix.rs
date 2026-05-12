use std::collections::{BTreeMap, BTreeSet};

use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_dependency_matrix::deterministic_dependency_matrix_report;
use crate::k0_receipt::{build_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::p00_dependency_matrix_model::{
    BlockerBinding, DependencyMatrixSurface, DependencyNode, ParallelLane,
};

pub const P00_DEPENDENCY_MATRIX_CONTRACT: &str = "LYRA-P00-DEPENDENCY-MATRIX v1";

pub const REQUIRED_DEPENDENCY_RULES: &[&str] = &[
    "dependency_matrix_must_cover_all_p00_tasks",
    "dependency_matrix_must_cover_all_closure_outputs",
    "blocker_matrix_must_bind_each_open_closure_output",
    "parallelization_map_must_preserve_dependency_order",
    "no_cycle_in_phase_dependencies",
    "no_network_dependency",
    "no_docs_only_matrix",
    "no_unreceipted_matrix",
    "no_global_closure_claim",
];

pub const REQUIRED_DEPENDENCY_NODES: &[&str] = &[
    "P00-001", "P00-002", "P00-003", "P00-004", "P00-005", "P00-006", "P00-007", "P00-008",
    "P00-009", "P00-010", "P00-011", "P00-012", "P00-013", "P00-014", "P00-015", "P00-016",
    "P00-017", "P00-018", "P00-019", "P00-020", "P00-021", "P00-022", "P00-023", "P00-024",
    "P00-X01", "P00-X02", "P00-X03", "P00-X04", "P00-X05",
];

pub const REQUIRED_BLOCKERS: &[&str] = &["P00-X02", "P00-X03", "P00-X04", "P00-X05", "P00-GLOBAL"];
pub const REQUIRED_PARALLEL_LANES: &[&str] = &[
    "constitution_identity_law",
    "evidence_engine_law",
    "operator_release_surfaces",
    "public_legitimacy_surfaces",
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
const ALLOWED_LANE_KINDS: &[&str] = &["law", "evidence", "surface", "public", "closure"];
const ALLOWED_ROOTS: &[&str] = &[
    "ops",
    "interfaces",
    "k0",
    "fixtures",
    "goldens",
    "receipts",
    "tests",
    "src",
    "shells",
    "examples",
    "products",
    "docs",
];

const REQUIRED_COMMANDS: &[&str] = &[
    "lyra-p00-validate",
    "lyra-p00-authority-check",
    "lyra-p00-identity-check",
    "lyra-p00-enforcement-check",
    "lyra-p00-delivery-check",
    "lyra-p00-challenge-check",
    "lyra-p00-control-check",
    "lyra-p00-owner-root-check",
    "lyra-p00-benchmark-evidence-check",
    "lyra-p00-public-interest-check",
    "lyra-p00-canon-compliance-check",
    "lyra-p00-acceptance-check",
    "lyra-p00-formal-semantics-check",
    "lyra-p00-canonical-model-check",
    "lyra-p00-engine-check",
    "lyra-p00-falsification-check",
    "lyra-p00-replay-check",
    "lyra-p00-interface-check",
    "lyra-p00-packaging-check",
    "lyra-p00-deployment-check",
    "lyra-p00-ecosystem-check",
    "lyra-p00-economics-check",
    "lyra-p00-redteam-check",
    "lyra-p00-closure-check",
    "lyra-p00-dependency-matrix-check",
];

const FORBIDDEN_DEPENDENCY_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::ClosureNetworkDependency),
    ("rule:network_required", ErrorCode::ClosureNetworkDependency),
    ("cloud required", ErrorCode::ClosureNetworkDependency),
    ("online required", ErrorCode::ClosureNetworkDependency),
    (
        "remote service required",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("remote fetch", ErrorCode::ClosureNetworkDependency),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("rule:docs_only_matrix", ErrorCode::ClosureDocsOnly),
    ("unreceipted matrix allowed", ErrorCode::ClosureUnreceipted),
    (
        "rule:unreceipted_matrix_allowed",
        ErrorCode::ClosureUnreceipted,
    ),
    ("matrix without receipt", ErrorCode::ClosureUnreceipted),
    ("dependency drift accepted", ErrorCode::ClosureDriftAccepted),
    ("blocker drift accepted", ErrorCode::ClosureDriftAccepted),
    (
        "parallelization drift accepted",
        ErrorCode::ClosureDriftAccepted,
    ),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("rule:global_complete", ErrorCode::UnsupportedGlobalClosure),
    ("global closure true", ErrorCode::UnsupportedGlobalClosure),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_dependency_matrix_surface(
    input: &str,
) -> Result<DependencyMatrixSurface, Vec<ValidationError>> {
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
            "no dependency matrix surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P00_DEPENDENCY_MATRIX_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P00_DEPENDENCY_MATRIX_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
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
                    "dependency matrix rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(node_id) = left.strip_prefix("dependency:") {
            if !is_known_p00_frontier_id(node_id) {
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
                    "dependency node identity must be unique",
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
                    "blocker identity must be unique",
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
                    format!("invalid parallel lane identity {lane_id}"),
                ));
                continue;
            }
            if !seen_lanes.insert(lane_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureOutputGate,
                    format!("lane:{lane_id}"),
                    "parallel lane identity must be unique",
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
                format!("unknown dependency matrix key {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(DependencyMatrixSurface {
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

pub fn validate_dependency_matrix_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_else(|_| input.to_string());
    let mut errors = Vec::new();
    scan_forbidden_text(&canonical, &mut errors);

    match parse_dependency_matrix_surface(input) {
        Ok(surface) => errors.extend(validate_dependency_matrix_model(&surface).errors),
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

pub fn validate_dependency_matrix_model(surface: &DependencyMatrixSurface) -> Verdict {
    let mut errors = Vec::new();
    if surface.phase != "P00" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "dependency matrix must bind to P00",
        ));
    }
    if surface.task != "P00-X01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "dependency matrix must bind to P00-X01",
        ));
    }
    if surface.status != "artifact_emitted" && surface.status != "execution_proven" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            format!("unsupported dependency matrix status {}", surface.status),
        ));
    }
    require_rules(surface, &mut errors);
    require_dependencies(surface, &mut errors);
    require_blockers(surface, &mut errors);
    require_lanes(surface, &mut errors);
    validate_dependencies(surface, &mut errors);
    validate_blockers(surface, &mut errors);
    validate_lanes(surface, &mut errors);
    validate_acyclic(surface, &mut errors);
    validate_dependency_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn parse_dependency(
    line_number: usize,
    id: &str,
    value: &str,
) -> Result<DependencyNode, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureTask,
            format!("line:{line_number:03}"),
            "dependency fields must be key:value segments",
        )
    })?;
    let node_kind = required_field(&fields, "kind", ErrorCode::InvalidClosureTask, line_number)?;
    let owner_roots = split_csv_allow_none(&required_field(
        &fields,
        "owner_roots",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?);
    let depends = split_csv_allow_none(&required_field(
        &fields,
        "depends",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?);
    let unlocks = split_csv_allow_none(&required_field(
        &fields,
        "unlocks",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?);
    let receipts = split_csv_allow_none(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidClosureTask,
        line_number,
    )?;
    Ok(DependencyNode {
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
) -> Result<BlockerBinding, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "blocker fields must be key:value segments",
        )
    })?;
    let blocked_by = split_csv_allow_none(&required_field(
        &fields,
        "blocked_by",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let reason = required_field(
        &fields,
        "reason",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?;
    let unblocks = split_csv_allow_none(&required_field(
        &fields,
        "unblocks",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let receipts = split_csv_allow_none(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?;
    Ok(BlockerBinding {
        line_number,
        id: id.to_string(),
        blocked_by,
        reason,
        unblocks,
        receipts,
        status,
    })
}

fn parse_lane(line_number: usize, id: &str, value: &str) -> Result<ParallelLane, ValidationError> {
    let fields = parse_field_map(value).ok_or_else(|| {
        ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            format!("line:{line_number:03}"),
            "parallel lane fields must be key:value segments",
        )
    })?;
    let lane_kind = required_field(
        &fields,
        "kind",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?;
    let frontiers = split_csv_allow_none(&required_field(
        &fields,
        "frontiers",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let after = split_csv_allow_none(&required_field(
        &fields,
        "after",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let can_parallel_with = split_csv_allow_none(&required_field(
        &fields,
        "can_parallel_with",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let receipts = split_csv_allow_none(&required_field(
        &fields,
        "receipts",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?);
    let status = required_field(
        &fields,
        "status",
        ErrorCode::InvalidClosureOutputGate,
        line_number,
    )?;
    Ok(ParallelLane {
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

fn require_rules(surface: &DependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for rule in REQUIRED_DEPENDENCY_RULES {
        match surface.rule_value(rule) {
            Some("required") | Some("forbidden") | Some("blocked_until_outputs") => {}
            Some(value) => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                format!("rule has unsupported value {value}"),
            )),
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{rule}"),
                "required dependency matrix rule missing",
            )),
        }
    }
}

fn require_dependencies(surface: &DependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_DEPENDENCY_NODES {
        if surface.dependency_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureTask,
                format!("dependency:{id}"),
                "required dependency node missing",
            ));
        }
    }
}

fn require_blockers(surface: &DependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_BLOCKERS {
        if surface.blocker_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("blocker:{id}"),
                "required blocker binding missing",
            ));
        }
    }
}

fn require_lanes(surface: &DependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for id in REQUIRED_PARALLEL_LANES {
        if surface.lane_by_id(id).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureOutputGate,
                format!("lane:{id}"),
                "required parallel lane missing",
            ));
        }
    }
}

fn validate_dependencies(surface: &DependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for node in &surface.dependencies {
        if !ALLOWED_NODE_KINDS.contains(&node.node_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                node.canonical_identity(),
                format!("invalid dependency kind {}", node.node_kind),
            ));
        }
        if !ALLOWED_STATUSES.contains(&node.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                node.canonical_identity(),
                format!("invalid dependency status {}", node.status),
            ));
        }
        if node.owner_roots.is_empty() || node.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                node.canonical_identity(),
                "dependency nodes must bind owner roots and receipts",
            ));
        }
        for root in &node.owner_roots {
            if !ALLOWED_ROOTS.contains(&root.as_str()) {
                errors.push(ValidationError::reject(
                    ErrorCode::InvalidOwnerRoot,
                    node.canonical_identity(),
                    format!("unknown dependency owner root {root}"),
                ));
            }
        }
        for target in node.depends.iter().chain(node.unlocks.iter()) {
            if target == "none" {
                continue;
            }
            if surface.dependency_by_id(target).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    node.canonical_identity(),
                    format!("unknown dependency reference {target}"),
                ));
            }
        }
        if node.id == "P00-X01" && !node.depends.iter().any(|item| item == "P00-024") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                node.canonical_identity(),
                "P00-X01 must depend on P00-024 closure gate",
            ));
        }
    }
}

fn validate_blockers(surface: &DependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for blocker in &surface.blockers {
        if !ALLOWED_STATUSES.contains(&blocker.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                blocker.canonical_identity(),
                format!("invalid blocker status {}", blocker.status),
            ));
        }
        if blocker.blocked_by.is_empty() || blocker.receipts.is_empty() || blocker.reason.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                blocker.canonical_identity(),
                "blockers must bind causes, reason, and receipts",
            ));
        }
        for target in blocker.blocked_by.iter().chain(blocker.unblocks.iter()) {
            if target == "none" || target == "global_closure" {
                continue;
            }
            if surface.dependency_by_id(target).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    blocker.canonical_identity(),
                    format!("unknown blocker reference {target}"),
                ));
            }
        }
        if blocker.id != "P00-GLOBAL" && !blocker.id.starts_with("P00-X") {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                blocker.canonical_identity(),
                "blocker must bind closure output or P00-GLOBAL",
            ));
        }
        if blocker.id == "P00-GLOBAL" && blocker.status != "blocked" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                blocker.canonical_identity(),
                "global closure must remain blocked",
            ));
        }
    }
}

fn validate_lanes(surface: &DependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for lane in &surface.lanes {
        if !ALLOWED_LANE_KINDS.contains(&lane.lane_kind.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                lane.canonical_identity(),
                format!("invalid lane kind {}", lane.lane_kind),
            ));
        }
        if !ALLOWED_STATUSES.contains(&lane.status.as_str()) {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                lane.canonical_identity(),
                format!("invalid lane status {}", lane.status),
            ));
        }
        if lane.frontiers.is_empty() || lane.receipts.is_empty() {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureOutputGate,
                lane.canonical_identity(),
                "parallel lanes must bind frontiers and receipts",
            ));
        }
        for frontier in &lane.frontiers {
            if surface.dependency_by_id(frontier).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    lane.canonical_identity(),
                    format!("unknown lane frontier {frontier}"),
                ));
            }
        }
        for after in &lane.after {
            if after == "none" {
                continue;
            }
            if surface.lane_by_id(after).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    lane.canonical_identity(),
                    format!("unknown lane after reference {after}"),
                ));
            }
        }
        for parallel in &lane.can_parallel_with {
            if parallel == "none" {
                continue;
            }
            if surface.lane_by_id(parallel).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    lane.canonical_identity(),
                    format!("unknown lane parallel reference {parallel}"),
                ));
            }
        }
    }
}

fn validate_acyclic(surface: &DependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for node in &surface.dependencies {
        graph.insert(
            node.id.as_str(),
            node.depends
                .iter()
                .filter(|dep| dep.as_str() != "none")
                .map(String::as_str)
                .collect(),
        );
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for node_id in graph.keys() {
        if has_cycle(node_id, &graph, &mut visiting, &mut visited) {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureDriftAccepted,
                format!("dependency:{node_id}"),
                "dependency matrix cycle detected",
            ));
            return;
        }
    }
}

fn has_cycle<'a>(
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
    if let Some(edges) = graph.get(node) {
        for edge in edges {
            if graph.contains_key(*edge) && has_cycle(edge, graph, visiting, visited) {
                return true;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node);
    false
}

fn validate_dependency_report(
    surface: &DependencyMatrixSurface,
    errors: &mut Vec<ValidationError>,
) {
    let dependency_inputs: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
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
        .collect();
    let blocker_inputs: Vec<(
        String,
        Vec<String>,
        String,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
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
        .collect();
    let lane_inputs: Vec<(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )> = surface
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
        .collect();
    let report =
        deterministic_dependency_matrix_report(&dependency_inputs, &blocker_inputs, &lane_inputs);
    if report.dependency_count != surface.dependencies.len()
        || report.blocker_count != surface.blockers.len()
        || report.lane_count != surface.lanes.len()
    {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_dependency_matrix_report",
            "dependency matrix report count mismatch",
        ));
    }
    if report.emitted_output_count == 0 {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "k0_dependency_matrix_report",
            "dependency matrix must mark at least P00-X01 artifact emitted",
        ));
    }
    if !report.matrix_hash.starts_with("fnv1a128:") {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "k0_dependency_matrix_report",
            "dependency matrix report hash must be stable fnv1a128",
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

fn split_csv_allow_none(value: &str) -> Vec<String> {
    if value == "none" {
        Vec::new()
    } else {
        value
            .split(',')
            .filter(|item| !item.is_empty())
            .map(str::to_string)
            .collect()
    }
}

fn is_known_p00_frontier_id(value: &str) -> bool {
    REQUIRED_DEPENDENCY_NODES.contains(&value)
}

fn is_known_blocker_id(value: &str) -> bool {
    REQUIRED_BLOCKERS.contains(&value)
}

fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn scan_forbidden_text(canonical: &str, errors: &mut Vec<ValidationError>) {
    let lowered = canonical.to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_DEPENDENCY_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "surface_text",
                format!("forbidden dependency matrix token {needle}"),
            ));
        }
    }
}

#[allow(dead_code)]
fn _known_command(command: &str) -> bool {
    REQUIRED_COMMANDS.contains(&command)
}
