use crate::k0_hash::stable_hash_label;
use crate::lyralang_core_ir::{
    core_ir_descriptor, LYRA_CORE_IR_BINARY_MAGIC, LYRA_CORE_IR_TEXT_HEADER,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoreIrReuseConsumerDescriptor {
    pub id: &'static str,
    pub surface: &'static str,
    pub target_phase: &'static str,
    pub owner_root: &'static str,
    pub core_ir_ref: &'static str,
    pub adapter: &'static str,
    pub fixture_path: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoreIrReuseEdgeDescriptor {
    pub id: &'static str,
    pub from_consumer: &'static str,
    pub to_consumer: &'static str,
    pub form: &'static str,
    pub guard: &'static str,
    pub rejection: &'static str,
    pub receipt_ref: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoreIrReuseGateDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub law: &'static str,
    pub evidence: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreIrReuseError {
    UnknownConsumer { id: String },
    UnknownEdge { id: String },
    UnknownGate { id: String },
    UnknownCoreIrForm { id: String },
}

pub const LYRALANG_CORE_IR_REUSE_CONSUMERS: &[CoreIrReuseConsumerDescriptor] = &[
    CoreIrReuseConsumerDescriptor {
        id: "parser_surface",
        surface: "parser",
        target_phase: "P03",
        owner_root: "lyralang",
        core_ir_ref: "core_ir:text_ir",
        adapter: "parse_tree_to_core_ir",
        fixture_path: "fixtures/p01/core_ir_reuse_cases/parser_surface_reuse.lyra",
        status: "artifact_emitted",
    },
    CoreIrReuseConsumerDescriptor {
        id: "checker_surface",
        surface: "checker",
        target_phase: "P04",
        owner_root: "lyralang",
        core_ir_ref: "core_ir:text_ir",
        adapter: "typed_core_ir_contract",
        fixture_path: "fixtures/p01/core_ir_reuse_cases/checker_surface_reuse.lyra",
        status: "artifact_emitted",
    },
    CoreIrReuseConsumerDescriptor {
        id: "evaluator_surface",
        surface: "evaluator",
        target_phase: "P10",
        owner_root: "lyralang",
        core_ir_ref: "core_ir:text_ir",
        adapter: "reference_eval_core_ir_seed",
        fixture_path: "fixtures/p01/core_ir_reuse_cases/evaluator_surface_reuse.lyra",
        status: "artifact_emitted",
    },
    CoreIrReuseConsumerDescriptor {
        id: "vm_surface",
        surface: "vm",
        target_phase: "P11",
        owner_root: "k0",
        core_ir_ref: "core_ir:binary_ir",
        adapter: "vm_frame_core_ir_contract",
        fixture_path: "fixtures/p01/core_ir_reuse_cases/vm_surface_reuse.lyra",
        status: "artifact_emitted",
    },
    CoreIrReuseConsumerDescriptor {
        id: "proof_surface",
        surface: "proof",
        target_phase: "P22",
        owner_root: "k0",
        core_ir_ref: "core_ir:text_ir",
        adapter: "proof_object_core_ir_witness",
        fixture_path: "fixtures/p01/core_ir_reuse_cases/proof_surface_reuse.lyra",
        status: "artifact_emitted",
    },
    CoreIrReuseConsumerDescriptor {
        id: "product_surface",
        surface: "product",
        target_phase: "P28",
        owner_root: "products",
        core_ir_ref: "core_ir:text_ir",
        adapter: "operator_inspection_core_ir_view",
        fixture_path: "fixtures/p01/core_ir_reuse_cases/product_surface_reuse.lyra",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_CORE_IR_REUSE_EDGES: &[CoreIrReuseEdgeDescriptor] = &[
    CoreIrReuseEdgeDescriptor {
        id: "core_ir_registry_to_parser",
        from_consumer: "core_ir_registry",
        to_consumer: "parser_surface",
        form: "text_ir",
        guard: "core_ir_identity_digest_unchanged",
        rejection: "forked_ir_rejected",
        receipt_ref: "receipt_core_ir_reuse",
        status: "execution_proven",
    },
    CoreIrReuseEdgeDescriptor {
        id: "parser_to_checker_ir_edge",
        from_consumer: "parser_surface",
        to_consumer: "checker_surface",
        form: "text_ir",
        guard: "core_ir_identity_digest_unchanged",
        rejection: "private_checker_ir_rejected",
        receipt_ref: "receipt_core_ir_reuse",
        status: "execution_proven",
    },
    CoreIrReuseEdgeDescriptor {
        id: "checker_to_evaluator_ir_edge",
        from_consumer: "checker_surface",
        to_consumer: "evaluator_surface",
        form: "text_ir",
        guard: "core_ir_identity_digest_unchanged",
        rejection: "semantic_rewrite_without_receipt_rejected",
        receipt_ref: "receipt_core_ir_reuse",
        status: "execution_proven",
    },
    CoreIrReuseEdgeDescriptor {
        id: "evaluator_to_vm_ir_edge",
        from_consumer: "evaluator_surface",
        to_consumer: "vm_surface",
        form: "binary_ir",
        guard: "text_binary_round_trip_identity",
        rejection: "vm_private_bytecode_rejected",
        receipt_ref: "receipt_core_ir_reuse",
        status: "execution_proven",
    },
    CoreIrReuseEdgeDescriptor {
        id: "vm_to_proof_ir_edge",
        from_consumer: "vm_surface",
        to_consumer: "proof_surface",
        form: "binary_ir",
        guard: "execution_trace_binds_core_ir_hash",
        rejection: "unbound_proof_ir_rejected",
        receipt_ref: "receipt_core_ir_reuse",
        status: "execution_proven",
    },
    CoreIrReuseEdgeDescriptor {
        id: "proof_to_product_ir_edge",
        from_consumer: "proof_surface",
        to_consumer: "product_surface",
        form: "text_ir",
        guard: "product_surface_preserves_core_ir_hash",
        rejection: "presentation_only_ir_fork_rejected",
        receipt_ref: "receipt_core_ir_reuse",
        status: "execution_proven",
    },
];

pub const LYRALANG_CORE_IR_REUSE_GATES: &[CoreIrReuseGateDescriptor] = &[
    CoreIrReuseGateDescriptor {
        id: "single_ir_contract_gate",
        scope: "all_consumers",
        law: "no_private_ir_forks",
        evidence: "all_consumers_bind_core_ir_descriptor",
        status: "execution_proven",
    },
    CoreIrReuseGateDescriptor {
        id: "cross_phase_identity_gate",
        scope: "all_edges",
        law: "core_ir_identity_digest_unchanged",
        evidence: "all_edges_bind_receipt_core_ir_reuse",
        status: "execution_proven",
    },
    CoreIrReuseGateDescriptor {
        id: "product_truth_gate",
        scope: "product_surface",
        law: "product_must_expose_core_ir_without_semantic_rewrite",
        evidence: "product_receipt_is_bound_to_core_ir_reuse",
        status: "execution_proven",
    },
];

pub fn core_ir_reuse_consumer_descriptor(
    id: &str,
) -> Option<&'static CoreIrReuseConsumerDescriptor> {
    LYRALANG_CORE_IR_REUSE_CONSUMERS
        .iter()
        .find(|item| item.id == id)
}
pub fn core_ir_reuse_edge_descriptor(id: &str) -> Option<&'static CoreIrReuseEdgeDescriptor> {
    LYRALANG_CORE_IR_REUSE_EDGES
        .iter()
        .find(|item| item.id == id)
}
pub fn core_ir_reuse_gate_descriptor(id: &str) -> Option<&'static CoreIrReuseGateDescriptor> {
    LYRALANG_CORE_IR_REUSE_GATES
        .iter()
        .find(|item| item.id == id)
}
pub fn core_ir_reuse_consumer_ids() -> Vec<&'static str> {
    LYRALANG_CORE_IR_REUSE_CONSUMERS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn core_ir_reuse_edge_ids() -> Vec<&'static str> {
    LYRALANG_CORE_IR_REUSE_EDGES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn core_ir_reuse_gate_ids() -> Vec<&'static str> {
    LYRALANG_CORE_IR_REUSE_GATES
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn core_ir_reuse_ref_is_bound(core_ir_ref: &str) -> bool {
    match core_ir_ref {
        "core_ir:text_ir" => {
            core_ir_descriptor("text_ir").is_some()
                && LYRA_CORE_IR_TEXT_HEADER == "LYRA-CORE-IR-TEXT v1"
        }
        "core_ir:binary_ir" => {
            core_ir_descriptor("binary_ir").is_some() && LYRA_CORE_IR_BINARY_MAGIC == b"LYRAIR01"
        }
        _ => false,
    }
}
pub fn core_ir_reuse_edge_endpoints_are_bound(edge: &CoreIrReuseEdgeDescriptor) -> bool {
    let from_ok = edge.from_consumer == "core_ir_registry"
        || core_ir_reuse_consumer_descriptor(edge.from_consumer).is_some();
    let to_ok = core_ir_reuse_consumer_descriptor(edge.to_consumer).is_some();
    from_ok && to_ok
}

pub fn canonical_core_ir_reuse_consumer_signature(
    descriptor: &CoreIrReuseConsumerDescriptor,
) -> String {
    format!(
        "consumer:{}|surface:{}|phase:{}|owner:{}|core_ir_ref:{}|adapter:{}|fixture:{}|status:{}",
        descriptor.id,
        descriptor.surface,
        descriptor.target_phase,
        descriptor.owner_root,
        descriptor.core_ir_ref,
        descriptor.adapter,
        descriptor.fixture_path,
        descriptor.status
    )
}
pub fn canonical_core_ir_reuse_edge_signature(descriptor: &CoreIrReuseEdgeDescriptor) -> String {
    format!(
        "edge:{}|from:{}|to:{}|form:{}|guard:{}|rejection:{}|receipt:{}|status:{}",
        descriptor.id,
        descriptor.from_consumer,
        descriptor.to_consumer,
        descriptor.form,
        descriptor.guard,
        descriptor.rejection,
        descriptor.receipt_ref,
        descriptor.status
    )
}
pub fn canonical_core_ir_reuse_gate_signature(descriptor: &CoreIrReuseGateDescriptor) -> String {
    format!(
        "gate:{}|scope:{}|law:{}|evidence:{}|status:{}",
        descriptor.id, descriptor.scope, descriptor.law, descriptor.evidence, descriptor.status
    )
}

pub fn core_ir_reuse_consumer_digest(id: &str) -> Result<String, CoreIrReuseError> {
    let descriptor = core_ir_reuse_consumer_descriptor(id)
        .ok_or_else(|| CoreIrReuseError::UnknownConsumer { id: id.to_string() })?;
    Ok(stable_hash_label(
        "lyra.p01.core_ir_reuse.consumer",
        &canonical_core_ir_reuse_consumer_signature(descriptor),
    ))
}
pub fn core_ir_reuse_edge_digest(id: &str) -> Result<String, CoreIrReuseError> {
    let descriptor = core_ir_reuse_edge_descriptor(id)
        .ok_or_else(|| CoreIrReuseError::UnknownEdge { id: id.to_string() })?;
    Ok(stable_hash_label(
        "lyra.p01.core_ir_reuse.edge",
        &canonical_core_ir_reuse_edge_signature(descriptor),
    ))
}
pub fn core_ir_reuse_gate_digest(id: &str) -> Result<String, CoreIrReuseError> {
    let descriptor = core_ir_reuse_gate_descriptor(id)
        .ok_or_else(|| CoreIrReuseError::UnknownGate { id: id.to_string() })?;
    Ok(stable_hash_label(
        "lyra.p01.core_ir_reuse.gate",
        &canonical_core_ir_reuse_gate_signature(descriptor),
    ))
}

pub fn canonical_core_ir_reuse_registry_signature() -> String {
    let mut lines = Vec::new();
    for descriptor in LYRALANG_CORE_IR_REUSE_CONSUMERS {
        lines.push(canonical_core_ir_reuse_consumer_signature(descriptor));
    }
    for descriptor in LYRALANG_CORE_IR_REUSE_EDGES {
        lines.push(canonical_core_ir_reuse_edge_signature(descriptor));
    }
    for descriptor in LYRALANG_CORE_IR_REUSE_GATES {
        lines.push(canonical_core_ir_reuse_gate_signature(descriptor));
    }
    lines.sort();
    lines.join("\n")
}
pub fn canonical_core_ir_reuse_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.core_ir_reuse.registry",
        &canonical_core_ir_reuse_registry_signature(),
    )
}
