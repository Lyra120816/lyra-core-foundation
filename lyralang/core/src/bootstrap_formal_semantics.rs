use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_BOOTSTRAP_FORMAL_SEMANTICS_CARRIER: &str =
    "LYRA-P02-BOOTSTRAP-FORMAL-SEMANTICS-CARRIER v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapFormalDomainDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub source_task: &'static str,
    pub semantic_object: &'static str,
    pub constitutional_binding: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapConstitutionalLawDescriptor {
    pub id: &'static str,
    pub domain_id: &'static str,
    pub law_class: &'static str,
    pub governs: &'static str,
    pub forbids: &'static str,
    pub requires_receipt: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapFormalTransitionDescriptor {
    pub id: &'static str,
    pub from_state: &'static str,
    pub to_state: &'static str,
    pub guard: &'static str,
    pub receipt: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapFormalInvariantDescriptor {
    pub id: &'static str,
    pub domain_id: &'static str,
    pub assertion: &'static str,
    pub rejects: &'static str,
    pub receipt: &'static str,
}

pub const LYRALANG_BOOTSTRAP_FORMAL_DOMAINS: &[BootstrapFormalDomainDescriptor] = &[
    BootstrapFormalDomainDescriptor {
        id: "bootstrap_trust",
        owner_root: "k0",
        source_task: "P02-013",
        semantic_object: "trust_floor",
        constitutional_binding: "constitutional_law_bound",
    },
    BootstrapFormalDomainDescriptor {
        id: "seed_runtime_law",
        owner_root: "lyralang",
        source_task: "P02-013",
        semantic_object: "seed_runtime_transition",
        constitutional_binding: "constitutional_law_bound",
    },
    BootstrapFormalDomainDescriptor {
        id: "host_extinction_framework",
        owner_root: "ops",
        source_task: "P02-013",
        semantic_object: "host_extinction_transition",
        constitutional_binding: "constitutional_law_bound",
    },
    BootstrapFormalDomainDescriptor {
        id: "foreign_surface_boundary",
        owner_root: "interfaces",
        source_task: "P02-012",
        semantic_object: "foreign_surface_interface",
        constitutional_binding: "constitutional_law_bound",
    },
    BootstrapFormalDomainDescriptor {
        id: "operator_handoff_truth",
        owner_root: "shells",
        source_task: "P02-011",
        semantic_object: "operator_capture_handoff",
        constitutional_binding: "constitutional_law_bound",
    },
    BootstrapFormalDomainDescriptor {
        id: "emergency_fallback_safety",
        owner_root: "products",
        source_task: "P02-008",
        semantic_object: "rollback_and_freeze",
        constitutional_binding: "constitutional_law_bound",
    },
];

pub const LYRALANG_BOOTSTRAP_CONSTITUTIONAL_LAWS: &[BootstrapConstitutionalLawDescriptor] = &[
    BootstrapConstitutionalLawDescriptor { id: "law_bootstrap_trust_receipt_only", domain_id: "bootstrap_trust", law_class: "trust_law", governs: "truth_advance", forbids: "foreign_truth,unreceipted_truth,ambient_authority", requires_receipt: "receipts/p02/pass_0070_foreign_surface_closure.receipt" },
    BootstrapConstitutionalLawDescriptor { id: "law_seed_runtime_no_silent_ownership", domain_id: "seed_runtime_law", law_class: "runtime_law", governs: "seed_runtime_replacement", forbids: "silent_rust_ownership,placeholder_runtime,unbounded_foreign_language", requires_receipt: "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt" },
    BootstrapConstitutionalLawDescriptor { id: "law_host_extinction_no_global_claim", domain_id: "host_extinction_framework", law_class: "closure_law", governs: "host_extinction", forbids: "phase_complete,global_closure,premature_extinction", requires_receipt: "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt" },
    BootstrapConstitutionalLawDescriptor { id: "law_foreign_surface_challenge_first", domain_id: "foreign_surface_boundary", law_class: "boundary_law", governs: "foreign_surface_truth", forbids: "hidden_surface,unbounded_surface,unchallengeable_surface", requires_receipt: "receipts/p02/pass_0070_foreign_surface_closure.receipt" },
    BootstrapConstitutionalLawDescriptor { id: "law_operator_handoff_no_truth_drift", domain_id: "operator_handoff_truth", law_class: "handoff_law", governs: "external_proof_capture", forbids: "truth_drift,network_required_handoff,manual_truth_promotion", requires_receipt: "receipts/p02/pass_0069_operator_handoff_automation.receipt" },
    BootstrapConstitutionalLawDescriptor { id: "law_emergency_fallback_freeze_before_advance", domain_id: "emergency_fallback_safety", law_class: "safety_law", governs: "fallback_recovery", forbids: "unsafe_advance,unreceipted_rollback,missing_freeze", requires_receipt: "receipts/p02/pass_0066_bootstrap_emergency_fallback.receipt" },
    BootstrapConstitutionalLawDescriptor { id: "law_no_probabilistic_bootstrap_semantics", domain_id: "bootstrap_trust", law_class: "invariant_law", governs: "all_bootstrap_semantics", forbids: "probabilistic_truth,hidden_randomness,ambient_time", requires_receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt" },
    BootstrapConstitutionalLawDescriptor { id: "law_local_validation_blocks_phase_closure", domain_id: "host_extinction_framework", law_class: "validation_law", governs: "phase_closure", forbids: "closure_without_local_validation,closure_without_goldens,closure_without_challenge_receipts", requires_receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt" },
];

pub const LYRALANG_BOOTSTRAP_FORMAL_TRANSITIONS: &[BootstrapFormalTransitionDescriptor] = &[
    BootstrapFormalTransitionDescriptor {
        id: "transition_inventory_to_trust_floor",
        from_state: "foreign_inventory_visible",
        to_state: "bootstrap_trust_floor",
        guard: "gate_inventory_total_and_receipted",
        receipt: "receipts/p02/pass_0062_bootstrap_inventory.receipt",
    },
    BootstrapFormalTransitionDescriptor {
        id: "transition_trust_floor_to_seed_runtime",
        from_state: "bootstrap_trust_floor",
        to_state: "seed_runtime_replacement",
        guard: "gate_seed_runtime_milestones_bound",
        receipt: "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
    },
    BootstrapFormalTransitionDescriptor {
        id: "transition_seed_runtime_to_host_extinction",
        from_state: "seed_runtime_replacement",
        to_state: "host_extinction_candidate",
        guard: "gate_evidence_emitted_for_all_targets",
        receipt: "receipts/p02/pass_0068_bootstrap_evidence_emission.receipt",
    },
    BootstrapFormalTransitionDescriptor {
        id: "transition_host_extinction_to_operator_capture",
        from_state: "host_extinction_candidate",
        to_state: "external_proof_capture",
        guard: "gate_operator_handoff_truth_neutral",
        receipt: "receipts/p02/pass_0069_operator_handoff_automation.receipt",
    },
    BootstrapFormalTransitionDescriptor {
        id: "transition_operator_capture_to_foreign_closure",
        from_state: "external_proof_capture",
        to_state: "foreign_surface_closure",
        guard: "gate_all_foreign_surfaces_challengeable",
        receipt: "receipts/p02/pass_0070_foreign_surface_closure.receipt",
    },
    BootstrapFormalTransitionDescriptor {
        id: "transition_foreign_closure_to_phase_open",
        from_state: "foreign_surface_closure",
        to_state: "p02_phase_open_next_frontier",
        guard: "receipt_gate_no_global_phase_closure",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
];

pub const LYRALANG_BOOTSTRAP_FORMAL_INVARIANTS: &[BootstrapFormalInvariantDescriptor] = &[
    BootstrapFormalInvariantDescriptor {
        id: "invariant_receipt_before_truth",
        domain_id: "bootstrap_trust",
        assertion: "truth_advances_only_after_receipt",
        rejects: "foreign_truth,unreceipted_truth",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
    BootstrapFormalInvariantDescriptor {
        id: "invariant_no_probabilistic_semantics",
        domain_id: "bootstrap_trust",
        assertion: "bootstrap_semantics_are_symbolic",
        rejects: "probabilistic_truth,stochastic_reasoning",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
    BootstrapFormalInvariantDescriptor {
        id: "invariant_no_hidden_randomness",
        domain_id: "seed_runtime_law",
        assertion: "runtime_semantics_have_explicit_inputs",
        rejects: "hidden_randomness,ambient_rng",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
    BootstrapFormalInvariantDescriptor {
        id: "invariant_no_ambient_time",
        domain_id: "seed_runtime_law",
        assertion: "time_is_explicitly_injected",
        rejects: "ambient_time,host_clock_truth",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
    BootstrapFormalInvariantDescriptor {
        id: "invariant_no_network_truth",
        domain_id: "foreign_surface_boundary",
        assertion: "network_never_supplies_truth",
        rejects: "network_required,cloud_required",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
    BootstrapFormalInvariantDescriptor {
        id: "invariant_no_global_closure",
        domain_id: "host_extinction_framework",
        assertion: "phase_closure_remains_blocked_until_local_validation",
        rejects: "phase_complete,global_closure",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
    BootstrapFormalInvariantDescriptor {
        id: "invariant_operator_capture_truth_neutral",
        domain_id: "operator_handoff_truth",
        assertion: "operator_capture_records_evidence_without_promoting_truth",
        rejects: "manual_truth_promotion,truth_drift",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
    BootstrapFormalInvariantDescriptor {
        id: "invariant_fallback_freezes_before_recovery",
        domain_id: "emergency_fallback_safety",
        assertion: "unsafe_bootstrap_paths_freeze_before_recovery",
        rejects: "unsafe_advance,missing_freeze",
        receipt: "receipts/p02/pass_0071_bootstrap_formal_semantics.receipt",
    },
];

pub fn bootstrap_formal_domain_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_FORMAL_DOMAINS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_constitutional_law_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_CONSTITUTIONAL_LAWS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_formal_transition_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_FORMAL_TRANSITIONS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_formal_invariant_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_FORMAL_INVARIANTS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn bootstrap_formal_domain_descriptor(
    id: &str,
) -> Option<&'static BootstrapFormalDomainDescriptor> {
    LYRALANG_BOOTSTRAP_FORMAL_DOMAINS
        .iter()
        .find(|x| x.id == id)
}
pub fn bootstrap_constitutional_law_descriptor(
    id: &str,
) -> Option<&'static BootstrapConstitutionalLawDescriptor> {
    LYRALANG_BOOTSTRAP_CONSTITUTIONAL_LAWS
        .iter()
        .find(|x| x.id == id)
}
pub fn bootstrap_formal_transition_descriptor(
    id: &str,
) -> Option<&'static BootstrapFormalTransitionDescriptor> {
    LYRALANG_BOOTSTRAP_FORMAL_TRANSITIONS
        .iter()
        .find(|x| x.id == id)
}
pub fn bootstrap_formal_invariant_descriptor(
    id: &str,
) -> Option<&'static BootstrapFormalInvariantDescriptor> {
    LYRALANG_BOOTSTRAP_FORMAL_INVARIANTS
        .iter()
        .find(|x| x.id == id)
}
pub fn bootstrap_formal_all_domains_constitutional() -> bool {
    LYRALANG_BOOTSTRAP_FORMAL_DOMAINS
        .iter()
        .all(|x| x.constitutional_binding == "constitutional_law_bound")
}
pub fn bootstrap_formal_all_laws_receipted() -> bool {
    LYRALANG_BOOTSTRAP_CONSTITUTIONAL_LAWS
        .iter()
        .all(|x| x.requires_receipt.starts_with("receipts/p02/"))
}
pub fn bootstrap_formal_all_transitions_guarded() -> bool {
    LYRALANG_BOOTSTRAP_FORMAL_TRANSITIONS
        .iter()
        .all(|x| x.guard.starts_with("gate_") || x.guard.starts_with("receipt_gate_"))
}
pub fn bootstrap_formal_invariants_reject_core_forbidden() -> bool {
    let rejects = LYRALANG_BOOTSTRAP_FORMAL_INVARIANTS
        .iter()
        .map(|x| x.rejects)
        .collect::<Vec<_>>()
        .join(",");
    [
        "probabilistic_truth",
        "hidden_randomness",
        "ambient_time",
        "network_required",
        "global_closure",
        "truth_drift",
    ]
    .iter()
    .all(|token| rejects.contains(token))
}

pub fn bootstrap_formal_semantics_registry_hash() -> String {
    let domains = LYRALANG_BOOTSTRAP_FORMAL_DOMAINS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.owner_root, x.source_task, x.semantic_object, x.constitutional_binding
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let laws = LYRALANG_BOOTSTRAP_CONSTITUTIONAL_LAWS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                x.id, x.domain_id, x.law_class, x.governs, x.forbids, x.requires_receipt
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let transitions = LYRALANG_BOOTSTRAP_FORMAL_TRANSITIONS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.from_state, x.to_state, x.guard, x.receipt
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let invariants = LYRALANG_BOOTSTRAP_FORMAL_INVARIANTS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.domain_id, x.assertion, x.rejects, x.receipt
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    stable_hash_label(
        "lyra.p02.bootstrap_formal_semantics.registry",
        &format!("{domains}|{laws}|{transitions}|{invariants}"),
    )
}

pub fn bootstrap_formal_semantics_registry_signature() -> String {
    format!(
        "{}:{}:{}:{}:{}",
        LYRA_P02_BOOTSTRAP_FORMAL_SEMANTICS_CARRIER,
        LYRALANG_BOOTSTRAP_FORMAL_DOMAINS.len(),
        LYRALANG_BOOTSTRAP_CONSTITUTIONAL_LAWS.len(),
        LYRALANG_BOOTSTRAP_FORMAL_TRANSITIONS.len(),
        LYRALANG_BOOTSTRAP_FORMAL_INVARIANTS.len()
    )
}
