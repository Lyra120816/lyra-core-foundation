use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormalSemanticDomainDescriptor {
    pub id: &'static str,
    pub layer: &'static str,
    pub owner_root: &'static str,
    pub meaning: &'static str,
    pub core_ref: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormalSemanticLawDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub rule: &'static str,
    pub guard: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormalSemanticInvariantDescriptor {
    pub id: &'static str,
    pub applies_to: &'static str,
    pub assertion: &'static str,
    pub evidence_ref: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormalSemanticProofDescriptor {
    pub id: &'static str,
    pub fixture: &'static str,
    pub golden: &'static str,
    pub receipt: &'static str,
    pub law_ref: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormalSemanticConstitutionError {
    UnknownDomain { id: String },
    UnknownLaw { id: String },
    UnknownInvariant { id: String },
    UnknownProof { id: String },
}

pub const LYRA_P01_FORMAL_SEMANTIC_CORE_REF: &str = "lyra_p01_semantic_core";

pub const LYRALANG_FORMAL_SEMANTIC_DOMAINS: &[FormalSemanticDomainDescriptor] = &[
    FormalSemanticDomainDescriptor {
        id: "canonical_symbols_domain",
        layer: "symbol",
        owner_root: "lyralang",
        meaning: "byte stable names and atom ids define canonical symbolic identity",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    FormalSemanticDomainDescriptor {
        id: "semantic_atoms_domain",
        layer: "atom",
        owner_root: "lyralang",
        meaning: "closed atom inventory anchors primitive meaning",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    FormalSemanticDomainDescriptor {
        id: "core_ir_terms_domain",
        layer: "ir",
        owner_root: "lyralang",
        meaning: "core IR carries canonical symbols and atoms without semantic fork",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    FormalSemanticDomainDescriptor {
        id: "semantic_objects_domain",
        layer: "object",
        owner_root: "lyralang",
        meaning: "semantic object records bind kind identity relation and receipt",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    FormalSemanticDomainDescriptor {
        id: "semantic_identity_domain",
        layer: "identity",
        owner_root: "lyralang",
        meaning: "semantic identity digests are canonical and collision checked",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    FormalSemanticDomainDescriptor {
        id: "reference_semantics_domain",
        layer: "reference",
        owner_root: "lyralang",
        meaning: "reference evaluation gives deterministic meaning to admitted literals",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    FormalSemanticDomainDescriptor {
        id: "symbolic_equality_domain",
        layer: "equality",
        owner_root: "lyralang",
        meaning: "symbolic equality is decided through canonical normalization witnesses",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
    FormalSemanticDomainDescriptor {
        id: "receipt_truth_domain",
        layer: "receipt",
        owner_root: "k0",
        meaning: "receipts record verdict bound truth and cannot replace execution proof",
        core_ref: "lyra_p01_semantic_core",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_FORMAL_SEMANTIC_LAWS: &[FormalSemanticLawDescriptor] = &[
    FormalSemanticLawDescriptor {
        id: "canonical_symbol_identity_law",
        scope: "canonical_symbols",
        rule: "symbol ids must serialize and hash byte identically across all owner roots",
        guard: "canonical_symbol_identity_is_byte_stable",
        status: "artifact_emitted",
    },
    FormalSemanticLawDescriptor {
        id: "semantic_atom_closed_world_law",
        scope: "semantic_atoms",
        rule: "every admitted atom id must be exported by the semantic atom reference library",
        guard: "semantic_atom_reference_all_atoms_exported",
        status: "artifact_emitted",
    },
    FormalSemanticLawDescriptor {
        id: "core_ir_single_carrier_law",
        scope: "core_ir",
        rule:
            "parser checker evaluator vm proof and product surfaces must reuse core IR descriptors",
        guard: "core_ir_reuse_edge_endpoints_are_bound",
        status: "artifact_emitted",
    },
    FormalSemanticLawDescriptor {
        id: "semantic_object_identity_law",
        scope: "semantic_objects",
        rule: "semantic objects must bind object id kind parent identity and receipt evidence",
        guard: "semantic_object_identity_bound",
        status: "artifact_emitted",
    },
    FormalSemanticLawDescriptor {
        id: "reference_semantics_totality_law",
        scope: "reference_semantics",
        rule: "admitted literal and composition descriptors must evaluate deterministically",
        guard: "reference_eval_seed_trace_hash_bound",
        status: "artifact_emitted",
    },
    FormalSemanticLawDescriptor {
        id: "symbolic_equality_normalization_law",
        scope: "symbolic_equality",
        rule: "equality claims must use canonical normal forms and substitution witnesses",
        guard: "symbolic_terms_equal_bound",
        status: "artifact_emitted",
    },
    FormalSemanticLawDescriptor {
        id: "receipt_verdict_parity_law",
        scope: "receipts",
        rule: "receipt truth requires matching contract law fixture golden and verdict hash",
        guard: "semantic_bedrock_parity_fixtures_cover_receipts",
        status: "artifact_emitted",
    },
    FormalSemanticLawDescriptor {
        id: "no_semantic_fork_law",
        scope: "semantic_core",
        rule: "all semantic domains and proofs must bind lyra_p01_semantic_core",
        guard: "formal_semantic_domains_bind_one_core",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_FORMAL_SEMANTIC_INVARIANTS: &[FormalSemanticInvariantDescriptor] = &[
    FormalSemanticInvariantDescriptor { id: "one_core_invariant", applies_to: "semantic_core", assertion: "all domains bind lyra_p01_semantic_core", evidence_ref: "semantic_bedrock_anchors_point_to_one_core", status: "execution_proven" },
    FormalSemanticInvariantDescriptor { id: "canonical_hash_invariant", applies_to: "canonical_symbols", assertion: "all canonical registry hashes use stable fnv1a128 labels", evidence_ref: "canonical_semantic_constitution_registry_hash", status: "execution_proven" },
    FormalSemanticInvariantDescriptor { id: "atom_reference_invariant", applies_to: "semantic_atoms", assertion: "semantic atom reference exports every admitted atom id", evidence_ref: "semantic_atom_reference_all_atoms_exported", status: "execution_proven" },
    FormalSemanticInvariantDescriptor { id: "core_ir_reuse_invariant", applies_to: "core_ir", assertion: "all reuse edges bind known core IR consumers", evidence_ref: "core_ir_reuse_edge_endpoints_are_bound", status: "execution_proven" },
    FormalSemanticInvariantDescriptor { id: "receipt_parity_invariant", applies_to: "receipts", assertion: "P01 receipts contracts and laws remain one to one", evidence_ref: "p01_contract_receipt_parity_tests", status: "execution_proven" },
    FormalSemanticInvariantDescriptor { id: "forbidden_semantics_invariant", applies_to: "semantic_core", assertion: "network probability hidden_entropy_patterns placeholders and global closure are rejected", evidence_ref: "formal_semantic_constitution_forbidden_text_scan", status: "execution_proven" },
];

pub const LYRALANG_FORMAL_SEMANTIC_PROOFS: &[FormalSemanticProofDescriptor] = &[
    FormalSemanticProofDescriptor { id: "canonical_symbols_proof", fixture: "fixtures/p01/formal_semantic_constitution_cases/canonical_symbol_identity_case.lyra", golden: "goldens/p01/valid_formal_semantic_constitution.receipt", receipt: "receipts/p01/pass_0042_formal_semantic_constitution.receipt", law_ref: "canonical_symbol_identity_law", status: "artifact_emitted" },
    FormalSemanticProofDescriptor { id: "semantic_atoms_proof", fixture: "fixtures/p01/formal_semantic_constitution_cases/semantic_atom_closed_world_case.lyra", golden: "goldens/p01/valid_formal_semantic_constitution.receipt", receipt: "receipts/p01/pass_0042_formal_semantic_constitution.receipt", law_ref: "semantic_atom_closed_world_law", status: "artifact_emitted" },
    FormalSemanticProofDescriptor { id: "core_ir_reuse_proof", fixture: "fixtures/p01/formal_semantic_constitution_cases/core_ir_single_carrier_case.lyra", golden: "goldens/p01/valid_formal_semantic_constitution.receipt", receipt: "receipts/p01/pass_0042_formal_semantic_constitution.receipt", law_ref: "core_ir_single_carrier_law", status: "artifact_emitted" },
    FormalSemanticProofDescriptor { id: "semantic_object_identity_proof", fixture: "fixtures/p01/formal_semantic_constitution_cases/semantic_object_identity_case.lyra", golden: "goldens/p01/valid_formal_semantic_constitution.receipt", receipt: "receipts/p01/pass_0042_formal_semantic_constitution.receipt", law_ref: "semantic_object_identity_law", status: "artifact_emitted" },
    FormalSemanticProofDescriptor { id: "symbolic_equality_proof", fixture: "fixtures/p01/formal_semantic_constitution_cases/symbolic_equality_normalization_case.lyra", golden: "goldens/p01/valid_formal_semantic_constitution.receipt", receipt: "receipts/p01/pass_0042_formal_semantic_constitution.receipt", law_ref: "symbolic_equality_normalization_law", status: "artifact_emitted" },
    FormalSemanticProofDescriptor { id: "receipt_verdict_parity_proof", fixture: "fixtures/p01/formal_semantic_constitution_cases/receipt_verdict_parity_case.lyra", golden: "goldens/p01/valid_formal_semantic_constitution.receipt", receipt: "receipts/p01/pass_0042_formal_semantic_constitution.receipt", law_ref: "receipt_verdict_parity_law", status: "artifact_emitted" },
];

pub fn formal_semantic_domain_ids() -> Vec<&'static str> {
    LYRALANG_FORMAL_SEMANTIC_DOMAINS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn formal_semantic_law_ids() -> Vec<&'static str> {
    LYRALANG_FORMAL_SEMANTIC_LAWS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn formal_semantic_invariant_ids() -> Vec<&'static str> {
    LYRALANG_FORMAL_SEMANTIC_INVARIANTS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn formal_semantic_proof_ids() -> Vec<&'static str> {
    LYRALANG_FORMAL_SEMANTIC_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn formal_semantic_domain_descriptor(
    id: &str,
) -> Option<&'static FormalSemanticDomainDescriptor> {
    LYRALANG_FORMAL_SEMANTIC_DOMAINS
        .iter()
        .find(|item| item.id == id)
}

pub fn formal_semantic_law_descriptor(id: &str) -> Option<&'static FormalSemanticLawDescriptor> {
    LYRALANG_FORMAL_SEMANTIC_LAWS
        .iter()
        .find(|item| item.id == id)
}

pub fn formal_semantic_invariant_descriptor(
    id: &str,
) -> Option<&'static FormalSemanticInvariantDescriptor> {
    LYRALANG_FORMAL_SEMANTIC_INVARIANTS
        .iter()
        .find(|item| item.id == id)
}

pub fn formal_semantic_proof_descriptor(
    id: &str,
) -> Option<&'static FormalSemanticProofDescriptor> {
    LYRALANG_FORMAL_SEMANTIC_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn canonical_formal_semantic_domain_signature(item: &FormalSemanticDomainDescriptor) -> String {
    format!(
        "domain:{}|layer:{}|owner:{}|meaning:{}|core:{}|status:{}",
        item.id, item.layer, item.owner_root, item.meaning, item.core_ref, item.status
    )
}

pub fn canonical_formal_semantic_law_signature(item: &FormalSemanticLawDescriptor) -> String {
    format!(
        "law:{}|scope:{}|rule:{}|guard:{}|status:{}",
        item.id, item.scope, item.rule, item.guard, item.status
    )
}

pub fn canonical_formal_semantic_invariant_signature(
    item: &FormalSemanticInvariantDescriptor,
) -> String {
    format!(
        "invariant:{}|applies:{}|assertion:{}|evidence:{}|status:{}",
        item.id, item.applies_to, item.assertion, item.evidence_ref, item.status
    )
}

pub fn canonical_formal_semantic_proof_signature(item: &FormalSemanticProofDescriptor) -> String {
    format!(
        "proof:{}|fixture:{}|golden:{}|receipt:{}|law:{}|status:{}",
        item.id, item.fixture, item.golden, item.receipt, item.law_ref, item.status
    )
}

pub fn canonical_formal_semantic_constitution_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_FORMAL_SEMANTIC_DOMAINS {
        rows.push(canonical_formal_semantic_domain_signature(item));
    }
    for item in LYRALANG_FORMAL_SEMANTIC_LAWS {
        rows.push(canonical_formal_semantic_law_signature(item));
    }
    for item in LYRALANG_FORMAL_SEMANTIC_INVARIANTS {
        rows.push(canonical_formal_semantic_invariant_signature(item));
    }
    for item in LYRALANG_FORMAL_SEMANTIC_PROOFS {
        rows.push(canonical_formal_semantic_proof_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn canonical_formal_semantic_constitution_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.formal_semantic_constitution.registry",
        &canonical_formal_semantic_constitution_registry_signature(),
    )
}

pub fn formal_semantic_domain_digest(id: &str) -> Option<String> {
    formal_semantic_domain_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.formal_semantic_constitution.domain",
            &canonical_formal_semantic_domain_signature(item),
        )
    })
}

pub fn formal_semantic_law_digest(id: &str) -> Option<String> {
    formal_semantic_law_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.formal_semantic_constitution.law",
            &canonical_formal_semantic_law_signature(item),
        )
    })
}

pub fn formal_semantic_invariant_digest(id: &str) -> Option<String> {
    formal_semantic_invariant_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.formal_semantic_constitution.invariant",
            &canonical_formal_semantic_invariant_signature(item),
        )
    })
}

pub fn formal_semantic_proof_digest(id: &str) -> Option<String> {
    formal_semantic_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.formal_semantic_constitution.proof",
            &canonical_formal_semantic_proof_signature(item),
        )
    })
}

pub fn formal_semantic_domains_bind_one_core() -> bool {
    LYRALANG_FORMAL_SEMANTIC_DOMAINS
        .iter()
        .all(|item| item.core_ref == LYRA_P01_FORMAL_SEMANTIC_CORE_REF)
}

pub fn formal_semantic_laws_cover_primitive_core() -> bool {
    ["canonical_symbols", "semantic_atoms", "core_ir"]
        .iter()
        .all(|scope| {
            LYRALANG_FORMAL_SEMANTIC_LAWS
                .iter()
                .any(|item| item.scope == *scope)
        })
}

pub fn formal_semantic_invariants_reference_admitted_evidence() -> bool {
    let admitted = [
        "semantic_bedrock_anchors_point_to_one_core",
        "canonical_semantic_constitution_registry_hash",
        "semantic_atom_reference_all_atoms_exported",
        "core_ir_reuse_edge_endpoints_are_bound",
        "p01_contract_receipt_parity_tests",
        "formal_semantic_constitution_forbidden_text_scan",
    ];
    LYRALANG_FORMAL_SEMANTIC_INVARIANTS
        .iter()
        .all(|item| admitted.contains(&item.evidence_ref))
}

pub fn formal_semantic_proofs_bind_known_laws() -> bool {
    LYRALANG_FORMAL_SEMANTIC_PROOFS
        .iter()
        .all(|item| formal_semantic_law_descriptor(item.law_ref).is_some())
}

pub fn formal_semantic_no_forbidden_law_claims() -> bool {
    LYRALANG_FORMAL_SEMANTIC_LAWS.iter().all(|item| {
        let text = format!("{} {} {}", item.scope, item.rule, item.guard).to_ascii_lowercase();
        !text.contains("probabilistic")
            && !text.contains("stochastic")
            && !text.contains("network required")
            && !text.contains("placeholder")
    })
}
