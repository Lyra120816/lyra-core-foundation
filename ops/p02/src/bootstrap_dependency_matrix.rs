use std::collections::{BTreeMap, BTreeSet};

use crate::k0_bootstrap_dependency_matrix::deterministic_bootstrap_dependency_matrix_report;
use crate::k0_canonical::{canonical_lines, canonical_surface_text};
use crate::k0_receipt::{build_phase_receipt, Receipt};
use crate::k0_verdict::{ErrorCode, ValidationError, Verdict};
use crate::lyralang_bootstrap_dependency_matrix::{
    bootstrap_dependency_blocker_descriptor, bootstrap_dependency_blocker_digest,
    bootstrap_dependency_blockers_bind_required_nodes,
    bootstrap_dependency_matrix_artifacts_bind_paths,
    bootstrap_dependency_matrix_carrier_signature,
    bootstrap_dependency_matrix_no_forbidden_descriptor_claims,
    bootstrap_dependency_matrix_receipts_cover_p02_001_through_p02_x01,
    bootstrap_dependency_matrix_registry_hash, bootstrap_dependency_node_descriptor,
    bootstrap_dependency_node_digest, bootstrap_dependency_nodes_bind_owner_roots,
    bootstrap_dependency_proof_descriptor, bootstrap_dependency_proof_digest,
    bootstrap_dependency_proofs_bind_registry, bootstrap_parallel_lane_descriptor,
    bootstrap_parallel_lane_digest, bootstrap_parallel_lanes_bind_existing_nodes,
    LYRA_P02_BOOTSTRAP_DEPENDENCY_MATRIX_CARRIER,
};
use crate::p02_bootstrap_dependency_matrix_model::{
    BootstrapDependencyBlocker, BootstrapDependencyMatrixSurface, BootstrapDependencyNode,
    BootstrapDependencyProof, BootstrapParallelLane,
};

pub const P02_BOOTSTRAP_DEPENDENCY_MATRIX_CONTRACT: &str =
    "LYRA-P02-BOOTSTRAP-DEPENDENCY-MATRIX v1";

pub const REQUIRED_BOOTSTRAP_DEPENDENCY_RULES: &[&str] = &[
    "dependency_matrix_must_bind_all_primary_tasks",
    "blocker_matrix_must_keep_x_outputs_ordered",
    "parallel_lanes_must_preserve_receipt_dependencies",
    "p02_x01_must_not_close_global_phase",
    "matrix_entries_must_name_owner_roots",
    "x_outputs_must_remain_serial_until_proven",
    "no_network_required_for_matrix",
    "no_unreceipted_parallelization",
    "no_docs_only_matrix",
    "next_frontier_must_be_p02_x02",
];
pub const REQUIRED_BOOTSTRAP_DEPENDENCY_NODES: &[&str] = &[
    "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
    "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
    "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
    "P02-X01", "P02-X02", "P02-X03", "P02-X04", "P02-X05",
];
pub const REQUIRED_BOOTSTRAP_DEPENDENCY_BLOCKERS: &[&str] = &[
    "x02_waits_for_dependency_matrix",
    "x03_waits_for_proof_family",
    "x04_waits_for_benchmark_pack",
    "x05_waits_for_output_table",
    "global_closure_denied_until_x05",
    "parallel_lane_requires_receipts",
    "host_extinction_blocks_retirement",
    "economics_capture_blocks_output_table",
];
pub const REQUIRED_BOOTSTRAP_PARALLEL_LANES: &[&str] = &[
    "lane_bootstrap_trust_core",
    "lane_seed_runtime_replacement",
    "lane_host_extinction",
    "lane_evidence_and_replay",
    "lane_packaging_public_surface",
    "lane_x_outputs_serial",
];
pub const REQUIRED_BOOTSTRAP_DEPENDENCY_PROOFS: &[&str] = &[
    "matrix_primary_node_coverage_proof",
    "matrix_x_output_serial_proof",
    "matrix_parallel_lane_safety_proof",
    "matrix_host_seed_retirement_blocker_proof",
    "matrix_public_surface_blocker_proof",
    "matrix_next_frontier_proof",
];

const ALLOWED_NODE_STATUSES: &[&str] = &[
    "bounded_closed",
    "artifact_emitted",
    "blocked",
    "working_slice",
];
const ALLOWED_BLOCKER_STATUSES: &[&str] = &["active", "cleared", "documented"];
const ALLOWED_LANE_STATUSES: &[&str] = &[
    "available_after_primary_gate",
    "available_after_dependency_matrix",
    "available_after_benchmark_pack",
    "serialized_by_retirement_law",
    "serialized_by_rollback_law",
    "strict_serial_x01_to_x05",
];
const ALLOWED_PROOF_STATUSES: &[&str] = &["artifact_emitted", "execution_proven", "working_slice"];
const ALLOWED_SEVERITIES: &[&str] = &["hard", "review", "info"];
const ALLOWED_PARALLEL_SAFE: &[&str] = &["yes", "no"];

const FORBIDDEN_MATRIX_TEXT: &[(&str, ErrorCode)] = &[
    ("network required", ErrorCode::ClosureNetworkDependency),
    ("cloud required", ErrorCode::ClosureNetworkDependency),
    ("online required", ErrorCode::ClosureNetworkDependency),
    (
        "remote service required",
        ErrorCode::ClosureNetworkDependency,
    ),
    ("remote fetch", ErrorCode::ClosureNetworkDependency),
    ("matrix without receipt", ErrorCode::ClosureUnreceipted),
    (
        "parallelization without receipt",
        ErrorCode::ClosureUnreceipted,
    ),
    ("docs only", ErrorCode::ClosureDocsOnly),
    ("manual only", ErrorCode::ClosureDocsOnly),
    ("global complete", ErrorCode::UnsupportedGlobalClosure),
    ("phase closed", ErrorCode::UnsupportedGlobalClosure),
    ("global_closure=true", ErrorCode::UnsupportedGlobalClosure),
    ("dependency skip allowed", ErrorCode::ClosureProofUnbound),
    ("parallel drift accepted", ErrorCode::ClosureDriftAccepted),
    ("todo", ErrorCode::PlaceholderAllowed),
    ("placeholder", ErrorCode::PlaceholderAllowed),
    ("best effort", ErrorCode::PlaceholderAllowed),
];

pub fn parse_bootstrap_dependency_matrix_surface(
    input: &str,
) -> Result<BootstrapDependencyMatrixSurface, Vec<ValidationError>> {
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
            "no bootstrap dependency matrix surface lines",
        )]);
    }
    let header = lines[0].clone();
    if header != P02_BOOTSTRAP_DEPENDENCY_MATRIX_CONTRACT {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidHeader,
            "line:001",
            format!("expected {P02_BOOTSTRAP_DEPENDENCY_MATRIX_CONTRACT}"),
        )]);
    }

    let mut errors = Vec::new();
    let mut phase = None;
    let mut task = None;
    let mut status = None;
    let mut closure_scope = None;
    let mut global_closure = None;
    let mut next_frontier = None;
    let mut rules = BTreeMap::new();
    let mut nodes = Vec::new();
    let mut blockers = Vec::new();
    let mut lanes = Vec::new();
    let mut proofs = Vec::new();
    let mut seen_scalars = BTreeSet::new();
    let mut seen_rules = BTreeSet::new();
    let mut seen_nodes = BTreeSet::new();
    let mut seen_blockers = BTreeSet::new();
    let mut seen_lanes = BTreeSet::new();
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
                    "dependency matrix rule names must be symbolic and unique",
                ));
            } else {
                rules.insert(rule_name.to_string(), value.to_string());
            }
            continue;
        }
        if let Some(node_id) = left.strip_prefix("node:") {
            if !is_matrix_id(node_id) || !seen_nodes.insert(node_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureTask,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid dependency node {node_id}"),
                ));
                continue;
            }
            match parse_node(line_number, node_id, value) {
                Ok(node) => nodes.push(node),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(blocker_id) = left.strip_prefix("blocker:") {
            if !is_symbolic_name(blocker_id) || !seen_blockers.insert(blocker_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid blocker {blocker_id}"),
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
            if !is_symbolic_name(lane_id) || !seen_lanes.insert(lane_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateEntry,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid lane {lane_id}"),
                ));
                continue;
            }
            match parse_lane(line_number, lane_id, value) {
                Ok(lane) => lanes.push(lane),
                Err(error) => errors.push(error),
            }
            continue;
        }
        if let Some(proof_id) = left.strip_prefix("proof:") {
            if !is_symbolic_name(proof_id) || !seen_proofs.insert(proof_id.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::DuplicateClosureProof,
                    format!("line:{line_number:03}"),
                    format!("duplicate or invalid dependency proof {proof_id}"),
                ));
                continue;
            }
            match parse_proof(line_number, proof_id, value) {
                Ok(proof) => proofs.push(proof),
                Err(error) => errors.push(error),
            }
            continue;
        }
        match left {
            "phase" | "task" | "status" | "closure_scope" | "global_closure" | "next_frontier" => {
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
                    "closure_scope" => closure_scope = Some(value.to_string()),
                    "global_closure" => global_closure = Some(value.to_string()),
                    "next_frontier" => next_frontier = Some(value.to_string()),
                    _ => unreachable!(),
                }
            }
            _ => errors.push(ValidationError::reject(
                ErrorCode::InvalidEntrySyntax,
                format!("line:{line_number:03}"),
                format!("unknown dependency matrix entry {left}"),
            )),
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    let Some(phase) = phase else {
        return Err(vec![ValidationError::reject(
            ErrorCode::MissingPhase,
            "phase",
            "missing phase",
        )]);
    };
    let Some(task) = task else {
        return Err(vec![ValidationError::reject(
            ErrorCode::MissingTask,
            "task",
            "missing task",
        )]);
    };
    let Some(status) = status else {
        return Err(vec![ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "missing status",
        )]);
    };
    let Some(closure_scope) = closure_scope else {
        return Err(vec![ValidationError::reject(
            ErrorCode::InvalidClosureOutputGate,
            "closure_scope",
            "missing closure scope",
        )]);
    };
    let Some(global_closure) = global_closure else {
        return Err(vec![ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "global_closure",
            "missing global closure assertion",
        )]);
    };
    let Some(next_frontier) = next_frontier else {
        return Err(vec![ValidationError::reject(
            ErrorCode::MissingBlockerBinding,
            "next_frontier",
            "missing next frontier",
        )]);
    };

    Ok(BootstrapDependencyMatrixSurface {
        header,
        phase,
        task,
        status,
        closure_scope,
        global_closure,
        next_frontier,
        rules,
        nodes,
        blockers,
        lanes,
        proofs,
    })
}

pub fn validate_bootstrap_dependency_matrix_surface(input: &str) -> (Verdict, Receipt) {
    let canonical = canonical_surface_text(input).unwrap_or_default();
    let verdict = match parse_bootstrap_dependency_matrix_surface(input) {
        Ok(surface) => validate_bootstrap_dependency_matrix_model(&surface),
        Err(errors) => Verdict::rejected(errors),
    };
    let receipt = build_phase_receipt("P02", input, &canonical, verdict.clone());
    (verdict, receipt)
}

pub fn validate_bootstrap_dependency_matrix_model(
    surface: &BootstrapDependencyMatrixSurface,
) -> Verdict {
    let mut errors = Vec::new();
    let lowered = canonical_model_text(surface).to_ascii_lowercase();
    for (needle, code) in FORBIDDEN_MATRIX_TEXT {
        if lowered.contains(needle) {
            errors.push(ValidationError::reject(
                *code,
                "matrix_text",
                format!("forbidden dependency matrix text: {needle}"),
            ));
        }
    }
    if surface.phase != "P02" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidPhase,
            "phase",
            "dependency matrix must bind P02",
        ));
    }
    if surface.task != "P02-X01" {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidTask,
            "task",
            "dependency matrix must bind P02-X01",
        ));
    }
    if surface.status != "artifact_emitted" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "status",
            "P02-X01 dependency matrix must be artifact_emitted",
        ));
    }
    if surface.closure_scope != "bounded_extension_open" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedClosureStatus,
            "closure_scope",
            "dependency matrix must keep bounded extension open",
        ));
    }
    if surface.global_closure != "false" {
        errors.push(ValidationError::reject(
            ErrorCode::UnsupportedGlobalClosure,
            "global_closure",
            "dependency matrix must deny global closure",
        ));
    }
    if surface.next_frontier != "P02-X02" {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBlockerBinding,
            "next_frontier",
            "P02-X01 must name P02-X02 as next frontier",
        ));
    }
    for required in REQUIRED_BOOTSTRAP_DEPENDENCY_RULES {
        if surface.rule_value(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureRule,
                format!("rule:{required}"),
                "missing dependency matrix rule",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_DEPENDENCY_NODES {
        if surface.node_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureTask,
                format!("node:{required}"),
                "missing dependency matrix node",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_DEPENDENCY_BLOCKERS {
        if surface.blocker_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingBlockerBinding,
                format!("blocker:{required}"),
                "missing dependency matrix blocker",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_PARALLEL_LANES {
        if surface.lane_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingBlockerBinding,
                format!("lane:{required}"),
                "missing dependency matrix lane",
            ));
        }
    }
    for required in REQUIRED_BOOTSTRAP_DEPENDENCY_PROOFS {
        if surface.proof_by_id(required).is_none() {
            errors.push(ValidationError::reject(
                ErrorCode::MissingClosureProof,
                format!("proof:{required}"),
                "missing dependency matrix proof",
            ));
        }
    }
    validate_nodes(surface, &mut errors);
    validate_blockers(surface, &mut errors);
    validate_lanes(surface, &mut errors);
    validate_proofs(surface, &mut errors);
    validate_descriptor_registry(surface, &mut errors);
    validate_report(surface, &mut errors);
    if errors.is_empty() {
        Verdict::accepted()
    } else {
        Verdict::rejected(errors)
    }
}

fn validate_nodes(surface: &BootstrapDependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for node in &surface.nodes {
        if !ALLOWED_NODE_STATUSES.contains(&node.status.as_str())
            || node.kind.is_empty()
            || node.owner_root.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                node.canonical_identity(),
                "invalid dependency node fields",
            ));
        }
        if !allowed_owner_root(&node.owner_root) {
            errors.push(ValidationError::reject(
                ErrorCode::MisplacedOwnerRoot,
                node.canonical_identity(),
                "dependency node owner root is outside Lyra owner roots",
            ));
        }
        for dep in &node.depends {
            if surface.node_by_id(dep).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    node.canonical_identity(),
                    format!("node dependency {dep} is unbound"),
                ));
            }
        }
        for next in &node.unblocks {
            if surface.node_by_id(next).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    node.canonical_identity(),
                    format!("node unblock {next} is unbound"),
                ));
            }
        }
    }
    if let Some(x01) = surface.node_by_id("P02-X01") {
        for primary in REQUIRED_BOOTSTRAP_DEPENDENCY_NODES
            .iter()
            .filter(|id| id.starts_with("P02-0"))
        {
            if !x01.depends.contains(&primary.to_string()) {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureFormulaViolation,
                    "node:P02-X01",
                    format!("P02-X01 must depend on {primary}"),
                ));
            }
        }
        if x01.status != "artifact_emitted" {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedClosureStatus,
                "node:P02-X01",
                "P02-X01 node must be artifact_emitted",
            ));
        }
    }
}

fn validate_blockers(
    surface: &BootstrapDependencyMatrixSurface,
    errors: &mut Vec<ValidationError>,
) {
    for blocker in &surface.blockers {
        if !ALLOWED_SEVERITIES.contains(&blocker.severity.as_str())
            || !ALLOWED_BLOCKER_STATUSES.contains(&blocker.status.as_str())
            || blocker.blocks.is_empty()
            || blocker.requires.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingBlockerBinding,
                blocker.canonical_identity(),
                "invalid dependency blocker fields",
            ));
        }
        if surface.node_by_id(&blocker.target).is_none() && blocker.target != "P02" {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureProofUnbound,
                blocker.canonical_identity(),
                "blocker target is unbound",
            ));
        }
        for id in blocker.blocks.iter().chain(blocker.requires.iter()) {
            if surface.node_by_id(id).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    blocker.canonical_identity(),
                    format!("blocker node {id} is unbound"),
                ));
            }
        }
    }
}

fn validate_lanes(surface: &BootstrapDependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for lane in &surface.lanes {
        if !ALLOWED_PARALLEL_SAFE.contains(&lane.parallel_safe.as_str())
            || !ALLOWED_LANE_STATUSES.contains(&lane.status.as_str())
            || lane.tasks.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingBlockerBinding,
                lane.canonical_identity(),
                "invalid parallel lane fields",
            ));
        }
        for id in lane.tasks.iter().chain(lane.depends.iter()) {
            if surface.node_by_id(id).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    lane.canonical_identity(),
                    format!("lane node {id} is unbound"),
                ));
            }
        }
        if lane.id == "lane_x_outputs_serial" && lane.parallel_safe != "no" {
            errors.push(ValidationError::reject(
                ErrorCode::ClosureFormulaViolation,
                lane.canonical_identity(),
                "x output lane must remain serial",
            ));
        }
    }
}

fn validate_proofs(surface: &BootstrapDependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    for proof in &surface.proofs {
        if !ALLOWED_PROOF_STATUSES.contains(&proof.status.as_str())
            || proof.nodes.is_empty()
            || proof.receipts.is_empty()
            || proof.commands.is_empty()
        {
            errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                proof.canonical_identity(),
                "invalid dependency proof fields",
            ));
        }
        if !proof
            .commands
            .iter()
            .any(|command| command == "lyra-p02-bootstrap-dependency-matrix-check")
        {
            errors.push(ValidationError::reject(
                ErrorCode::MissingCommandRecord,
                proof.canonical_identity(),
                "dependency proof must bind dependency matrix command",
            ));
        }
        if !proof
            .forbids
            .iter()
            .any(|forbid| forbid == "global_closure")
        {
            errors.push(ValidationError::reject(
                ErrorCode::UnsupportedGlobalClosure,
                proof.canonical_identity(),
                "dependency proof must forbid global closure",
            ));
        }
        for id in &proof.nodes {
            if surface.node_by_id(id).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    proof.canonical_identity(),
                    format!("proof node {id} is unbound"),
                ));
            }
        }
        for id in &proof.blockers {
            if surface.blocker_by_id(id).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    proof.canonical_identity(),
                    format!("proof blocker {id} is unbound"),
                ));
            }
        }
        for id in &proof.lanes {
            if surface.lane_by_id(id).is_none() {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureProofUnbound,
                    proof.canonical_identity(),
                    format!("proof lane {id} is unbound"),
                ));
            }
        }
        for receipt in &proof.receipts {
            if !receipt.starts_with("receipts/p02/pass_") {
                errors.push(ValidationError::reject(
                    ErrorCode::ClosureUnreceipted,
                    proof.canonical_identity(),
                    format!("receipt {receipt} is outside P02 receipt path"),
                ));
            }
        }
    }
}

fn validate_descriptor_registry(
    surface: &BootstrapDependencyMatrixSurface,
    errors: &mut Vec<ValidationError>,
) {
    if !bootstrap_dependency_nodes_bind_owner_roots() {
        errors.push(ValidationError::reject(
            ErrorCode::MisplacedOwnerRoot,
            "lyralang_registry",
            "dependency nodes do not bind owner roots",
        ));
    }
    if !bootstrap_dependency_blockers_bind_required_nodes() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBlockerBinding,
            "lyralang_registry",
            "dependency blockers do not bind required nodes",
        ));
    }
    if !bootstrap_parallel_lanes_bind_existing_nodes() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingBlockerBinding,
            "lyralang_registry",
            "parallel lanes do not bind existing nodes",
        ));
    }
    if !bootstrap_dependency_proofs_bind_registry() {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            "lyralang_registry",
            "dependency proofs do not bind registry",
        ));
    }
    if !bootstrap_dependency_matrix_artifacts_bind_paths() {
        errors.push(ValidationError::reject(
            ErrorCode::UnknownEvidencePath,
            "lyralang_registry",
            "dependency matrix artifacts are not path-bound",
        ));
    }
    if !bootstrap_dependency_matrix_receipts_cover_p02_001_through_p02_x01() {
        errors.push(ValidationError::reject(
            ErrorCode::MissingReceiptProof,
            "lyralang_registry",
            "dependency matrix receipts do not cover P02-001 through P02-X01",
        ));
    }
    if !bootstrap_dependency_matrix_no_forbidden_descriptor_claims() {
        errors.push(ValidationError::reject(
            ErrorCode::ClosureDriftAccepted,
            "lyralang_registry",
            "dependency matrix descriptors contain forbidden claims",
        ));
    }
    if !LYRA_P02_BOOTSTRAP_DEPENDENCY_MATRIX_CARRIER.contains("bootstrap_dependency_matrix")
        || bootstrap_dependency_matrix_carrier_signature().is_empty()
        || bootstrap_dependency_matrix_registry_hash().is_empty()
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            "lyralang_registry",
            "dependency matrix registry hash missing",
        ));
    }
    for node in &surface.nodes {
        match bootstrap_dependency_node_descriptor(&node.id) {
            Some(descriptor) => {
                if descriptor.kind != node.kind
                    || descriptor.status != node.status
                    || descriptor.owner_root != node.owner_root
                    || bootstrap_dependency_node_digest(&node.id).is_none()
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::ClosureDriftAccepted,
                        node.canonical_identity(),
                        "dependency node descriptor drift",
                    ));
                }
            }
            None => errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureTask,
                node.canonical_identity(),
                "missing dependency node descriptor",
            )),
        }
    }
    for blocker in &surface.blockers {
        match bootstrap_dependency_blocker_descriptor(&blocker.id) {
            Some(descriptor) => {
                if descriptor.target != blocker.target
                    || descriptor.severity != blocker.severity
                    || descriptor.status != blocker.status
                    || bootstrap_dependency_blocker_digest(&blocker.id).is_none()
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::ClosureDriftAccepted,
                        blocker.canonical_identity(),
                        "dependency blocker descriptor drift",
                    ));
                }
            }
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingBlockerBinding,
                blocker.canonical_identity(),
                "missing dependency blocker descriptor",
            )),
        }
    }
    for lane in &surface.lanes {
        match bootstrap_parallel_lane_descriptor(&lane.id) {
            Some(descriptor) => {
                if descriptor.scope != lane.scope
                    || descriptor.parallel_safe != lane.parallel_safe
                    || descriptor.status != lane.status
                    || bootstrap_parallel_lane_digest(&lane.id).is_none()
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::ClosureDriftAccepted,
                        lane.canonical_identity(),
                        "parallel lane descriptor drift",
                    ));
                }
            }
            None => errors.push(ValidationError::reject(
                ErrorCode::MissingBlockerBinding,
                lane.canonical_identity(),
                "missing parallel lane descriptor",
            )),
        }
    }
    for proof in &surface.proofs {
        match bootstrap_dependency_proof_descriptor(&proof.id) {
            Some(descriptor) => {
                if descriptor.status != proof.status
                    || bootstrap_dependency_proof_digest(&proof.id).is_none()
                {
                    errors.push(ValidationError::reject(
                        ErrorCode::ClosureDriftAccepted,
                        proof.canonical_identity(),
                        "dependency proof descriptor drift",
                    ));
                }
            }
            None => errors.push(ValidationError::reject(
                ErrorCode::InvalidClosureProof,
                proof.canonical_identity(),
                "missing dependency proof descriptor",
            )),
        }
    }
}

fn validate_report(surface: &BootstrapDependencyMatrixSurface, errors: &mut Vec<ValidationError>) {
    let nodes = surface
        .nodes
        .iter()
        .map(|node| {
            (
                node.id.clone(),
                node.kind.clone(),
                node.status.clone(),
                node.depends.clone(),
                node.unblocks.clone(),
                node.owner_root.clone(),
            )
        })
        .collect::<Vec<_>>();
    let blockers = surface
        .blockers
        .iter()
        .map(|blocker| {
            (
                blocker.id.clone(),
                blocker.target.clone(),
                blocker.severity.clone(),
                blocker.blocks.clone(),
                blocker.requires.clone(),
                blocker.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let lanes = surface
        .lanes
        .iter()
        .map(|lane| {
            (
                lane.id.clone(),
                lane.scope.clone(),
                lane.tasks.clone(),
                lane.depends.clone(),
                lane.parallel_safe.clone(),
                lane.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let proofs = surface
        .proofs
        .iter()
        .map(|proof| {
            (
                proof.id.clone(),
                proof.nodes.clone(),
                proof.blockers.clone(),
                proof.lanes.clone(),
                proof.receipts.clone(),
                proof.commands.clone(),
                proof.status.clone(),
            )
        })
        .collect::<Vec<_>>();
    let report =
        deterministic_bootstrap_dependency_matrix_report(&nodes, &blockers, &lanes, &proofs);
    if report.node_count != surface.nodes.len()
        || report.blocker_count != surface.blockers.len()
        || report.lane_count != surface.lanes.len()
        || report.proof_count != surface.proofs.len()
        || !report.matrix_hash.starts_with("fnv1a128:")
    {
        errors.push(ValidationError::reject(
            ErrorCode::InvalidProofBinding,
            "dependency_matrix_report",
            "deterministic dependency matrix report mismatch",
        ));
    }
}

fn parse_node(
    line_number: usize,
    node_id: &str,
    value: &str,
) -> Result<BootstrapDependencyNode, ValidationError> {
    let Some(fields) = parse_field_map(value) else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureTask,
            format!("line:{line_number:03}"),
            "invalid dependency node field map",
        ));
    };
    Ok(BootstrapDependencyNode {
        line_number,
        id: node_id.to_string(),
        kind: required_field(&fields, "kind", ErrorCode::InvalidClosureTask, line_number)?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureTask,
            line_number,
        )?,
        depends: split_csv(&required_field(
            &fields,
            "depends",
            ErrorCode::InvalidClosureTask,
            line_number,
        )?),
        unblocks: split_csv(&required_field(
            &fields,
            "unblocks",
            ErrorCode::InvalidClosureTask,
            line_number,
        )?),
        owner_root: required_field(
            &fields,
            "owner_root",
            ErrorCode::InvalidClosureTask,
            line_number,
        )?,
    })
}
fn parse_blocker(
    line_number: usize,
    blocker_id: &str,
    value: &str,
) -> Result<BootstrapDependencyBlocker, ValidationError> {
    let Some(fields) = parse_field_map(value) else {
        return Err(ValidationError::reject(
            ErrorCode::MissingBlockerBinding,
            format!("line:{line_number:03}"),
            "invalid blocker field map",
        ));
    };
    Ok(BootstrapDependencyBlocker {
        line_number,
        id: blocker_id.to_string(),
        target: required_field(
            &fields,
            "target",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?,
        severity: required_field(
            &fields,
            "severity",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?,
        blocks: split_csv(&required_field(
            &fields,
            "blocks",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?),
        requires: split_csv(&required_field(
            &fields,
            "requires",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?,
    })
}
fn parse_lane(
    line_number: usize,
    lane_id: &str,
    value: &str,
) -> Result<BootstrapParallelLane, ValidationError> {
    let Some(fields) = parse_field_map(value) else {
        return Err(ValidationError::reject(
            ErrorCode::MissingBlockerBinding,
            format!("line:{line_number:03}"),
            "invalid lane field map",
        ));
    };
    Ok(BootstrapParallelLane {
        line_number,
        id: lane_id.to_string(),
        scope: required_field(
            &fields,
            "scope",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?,
        tasks: split_csv(&required_field(
            &fields,
            "tasks",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?),
        depends: split_csv(&required_field(
            &fields,
            "depends",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?),
        parallel_safe: required_field(
            &fields,
            "parallel_safe",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?,
        status: required_field(
            &fields,
            "status",
            ErrorCode::MissingBlockerBinding,
            line_number,
        )?,
    })
}
fn parse_proof(
    line_number: usize,
    proof_id: &str,
    value: &str,
) -> Result<BootstrapDependencyProof, ValidationError> {
    let Some(fields) = parse_field_map(value) else {
        return Err(ValidationError::reject(
            ErrorCode::InvalidClosureProof,
            format!("line:{line_number:03}"),
            "invalid dependency proof field map",
        ));
    };
    Ok(BootstrapDependencyProof {
        line_number,
        id: proof_id.to_string(),
        nodes: split_csv(&required_field(
            &fields,
            "nodes",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        blockers: split_csv(&required_field(
            &fields,
            "blockers",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        lanes: split_csv(&required_field(
            &fields,
            "lanes",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        receipts: split_csv(&required_field(
            &fields,
            "receipts",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        commands: split_csv(&required_field(
            &fields,
            "commands",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        permits: split_csv(&required_field(
            &fields,
            "permits",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        forbids: split_csv(&required_field(
            &fields,
            "forbids",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?),
        status: required_field(
            &fields,
            "status",
            ErrorCode::InvalidClosureProof,
            line_number,
        )?,
    })
}

fn parse_field_map(value: &str) -> Option<BTreeMap<String, String>> {
    let mut fields = BTreeMap::new();
    if value.is_empty() || value.contains("||") {
        return None;
    }
    for part in value.split('|') {
        let (key, field_value) = part.split_once(':')?;
        if key.is_empty()
            || field_value.is_empty()
            || !is_symbolic_name(key)
            || fields
                .insert(key.to_string(), field_value.to_string())
                .is_some()
        {
            return None;
        }
    }
    Some(fields)
}
fn required_field(
    fields: &BTreeMap<String, String>,
    key: &str,
    code: ErrorCode,
    line_number: usize,
) -> Result<String, ValidationError> {
    fields
        .get(key)
        .cloned()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ValidationError::reject(
                code,
                format!("line:{line_number:03}"),
                format!("missing field {key}"),
            )
        })
}
fn split_csv(value: &str) -> Vec<String> {
    if value == "-" {
        Vec::new()
    } else {
        value
            .split(',')
            .filter(|item| !item.is_empty())
            .map(str::to_string)
            .collect()
    }
}
fn is_symbolic_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}
fn is_matrix_id(value: &str) -> bool {
    value.starts_with("P02-") || value.starts_with("P02-X")
}
fn allowed_owner_root(path: &str) -> bool {
    path.starts_with("k0/")
        || path.starts_with("k1/")
        || path.starts_with("interfaces/")
        || path.starts_with("lyralang/")
        || path.starts_with("ops/")
        || path.starts_with("shells/")
        || path.starts_with("products/")
        || path.starts_with("docs/")
}
fn canonical_model_text(surface: &BootstrapDependencyMatrixSurface) -> String {
    let mut rows = Vec::new();
    rows.push(format!("phase={}", surface.phase));
    rows.push(format!("task={}", surface.task));
    rows.push(format!("status={}", surface.status));
    rows.push(format!("closure_scope={}", surface.closure_scope));
    rows.push(format!("global_closure={}", surface.global_closure));
    rows.push(format!("next_frontier={}", surface.next_frontier));
    rows.extend(
        surface
            .rules
            .iter()
            .map(|(key, value)| format!("rule:{key}={value}")),
    );
    rows.extend(surface.nodes.iter().map(|node| {
        format!(
            "node:{}|kind:{}|status:{}|depends:{}|unblocks:{}|owner_root:{}",
            node.id,
            node.kind,
            node.status,
            node.depends.join(","),
            node.unblocks.join(","),
            node.owner_root
        )
    }));
    rows.extend(surface.blockers.iter().map(|blocker| {
        format!(
            "blocker:{}|target:{}|severity:{}|blocks:{}|requires:{}|status:{}",
            blocker.id,
            blocker.target,
            blocker.severity,
            blocker.blocks.join(","),
            blocker.requires.join(","),
            blocker.status
        )
    }));
    rows.extend(surface.lanes.iter().map(|lane| {
        format!(
            "lane:{}|scope:{}|tasks:{}|depends:{}|parallel_safe:{}|status:{}",
            lane.id,
            lane.scope,
            lane.tasks.join(","),
            lane.depends.join(","),
            lane.parallel_safe,
            lane.status
        )
    }));
    rows.extend(surface.proofs.iter().map(|proof| format!("proof:{}|nodes:{}|blockers:{}|lanes:{}|receipts:{}|commands:{}|permits:{}|forbids:{}|status:{}", proof.id, proof.nodes.join(","), proof.blockers.join(","), proof.lanes.join(","), proof.receipts.join(","), proof.commands.join(","), proof.permits.join(","), proof.forbids.join(","), proof.status)));
    rows.join(
        "
",
    )
}
