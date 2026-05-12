use crate::k0_hash::stable_hash_label;
use crate::p02_bootstrap_formal_semantics_model::BootstrapFormalSemanticsSurface;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFormalSemanticsReport {
    pub domain_count: usize,
    pub law_count: usize,
    pub transition_count: usize,
    pub invariant_count: usize,
    pub proof_count: usize,
    pub receipt_count: usize,
    pub constitutional_domain_count: usize,
    pub receipt_bound_law_count: usize,
    pub guarded_transition_count: usize,
    pub rejected_token_count: usize,
    pub semantics_hash: String,
}

pub fn deterministic_bootstrap_formal_semantics_report(
    surface: &BootstrapFormalSemanticsSurface,
) -> BootstrapFormalSemanticsReport {
    let mut domain_ids = surface
        .domains
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.owner_root, x.source_task, x.semantic_object, x.constitutional_binding
            )
        })
        .collect::<Vec<_>>();
    let mut laws = surface
        .laws
        .iter()
        .map(|x| {
            let mut forbids = x.forbids.clone();
            forbids.sort();
            format!(
                "{}:{}:{}:{}:{}:{}",
                x.id,
                x.domain_id,
                x.law_class,
                x.governs,
                forbids.join(","),
                x.requires_receipt
            )
        })
        .collect::<Vec<_>>();
    let mut transitions = surface
        .transitions
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.from_state, x.to_state, x.guard, x.receipt
            )
        })
        .collect::<Vec<_>>();
    let mut invariants = surface
        .invariants
        .iter()
        .map(|x| {
            let mut rejects = x.rejects.clone();
            rejects.sort();
            format!(
                "{}:{}:{}:{}:{}",
                x.id,
                x.domain_id,
                x.assertion,
                rejects.join(","),
                x.receipt
            )
        })
        .collect::<Vec<_>>();
    let mut proofs = surface
        .proofs
        .iter()
        .map(|x| {
            let mut domains = x.domains.clone();
            domains.sort();
            let mut laws = x.laws.clone();
            laws.sort();
            let mut transitions = x.transitions.clone();
            transitions.sort();
            let mut invariants = x.invariants.clone();
            invariants.sort();
            let mut receipts = x.receipts.clone();
            receipts.sort();
            format!(
                "{}:{}:{}:{}:{}:{}:{}",
                x.id,
                x.scope,
                domains.join(","),
                laws.join(","),
                transitions.join(","),
                invariants.join(","),
                receipts.join(",")
            )
        })
        .collect::<Vec<_>>();
    let mut receipts = surface
        .receipts
        .iter()
        .map(|x| format!("{}:{}:{}", x.id, x.path, x.binds))
        .collect::<Vec<_>>();
    domain_ids.sort();
    laws.sort();
    transitions.sort();
    invariants.sort();
    proofs.sort();
    receipts.sort();
    let mut rejected = surface
        .invariants
        .iter()
        .flat_map(|x| x.rejects.clone())
        .collect::<Vec<_>>();
    rejected.sort();
    rejected.dedup();
    let preimage = format!(
        "domains={}|laws={}|transitions={}|invariants={}|proofs={}|receipts={}",
        domain_ids.join("|"),
        laws.join("|"),
        transitions.join("|"),
        invariants.join("|"),
        proofs.join("|"),
        receipts.join("|")
    );
    BootstrapFormalSemanticsReport {
        domain_count: surface.domains.len(),
        law_count: surface.laws.len(),
        transition_count: surface.transitions.len(),
        invariant_count: surface.invariants.len(),
        proof_count: surface.proofs.len(),
        receipt_count: surface.receipts.len(),
        constitutional_domain_count: surface
            .domains
            .iter()
            .filter(|x| x.constitutional())
            .count(),
        receipt_bound_law_count: surface.laws.iter().filter(|x| x.receipt_bound()).count(),
        guarded_transition_count: surface
            .transitions
            .iter()
            .filter(|x| x.guarded() && x.receipt_bound())
            .count(),
        rejected_token_count: rejected.len(),
        semantics_hash: stable_hash_label("lyra.p02.bootstrap_formal_semantics.report", &preimage),
    }
}
