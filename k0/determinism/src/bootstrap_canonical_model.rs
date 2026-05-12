use crate::k0_hash::stable_hash_label;
use crate::p02_bootstrap_canonical_model_model::BootstrapCanonicalModelSurface;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalModelReport {
    pub model_count: usize,
    pub schema_count: usize,
    pub field_count: usize,
    pub relation_count: usize,
    pub invariant_count: usize,
    pub proof_count: usize,
    pub receipt_count: usize,
    pub local_schema_count: usize,
    pub required_field_count: usize,
    pub receipt_bound_invariant_count: usize,
    pub canonical_model_hash: String,
}

pub fn deterministic_bootstrap_canonical_model_report(
    surface: &BootstrapCanonicalModelSurface,
) -> BootstrapCanonicalModelReport {
    let mut models = surface
        .models
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                x.id, x.owner_root, x.domain_id, x.canonical_kind, x.schema_path, x.hash_policy
            )
        })
        .collect::<Vec<_>>();
    let mut schemas = surface
        .schemas
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.model_id, x.contract_path, x.encoding, x.version
            )
        })
        .collect::<Vec<_>>();
    let mut fields = surface
        .fields
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                x.id, x.model_id, x.name, x.field_type, x.required, x.canonical_order
            )
        })
        .collect::<Vec<_>>();
    let mut relations = surface
        .relations
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}",
                x.id, x.from_model, x.to_model, x.relation_kind, x.cardinality
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
                x.model_id,
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
            let mut models = x.models.clone();
            models.sort();
            let mut schemas = x.schemas.clone();
            schemas.sort();
            let mut fields = x.fields.clone();
            fields.sort();
            let mut relations = x.relations.clone();
            relations.sort();
            let mut invariants = x.invariants.clone();
            invariants.sort();
            let mut receipts = x.receipts.clone();
            receipts.sort();
            format!(
                "{}:{}:{}:{}:{}:{}:{}:{}",
                x.id,
                x.scope,
                models.join(","),
                schemas.join(","),
                fields.join(","),
                relations.join(","),
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
    models.sort();
    schemas.sort();
    fields.sort();
    relations.sort();
    invariants.sort();
    proofs.sort();
    receipts.sort();
    let preimage = format!(
        "models={}|schemas={}|fields={}|relations={}|invariants={}|proofs={}|receipts={}",
        models.join("|"),
        schemas.join("|"),
        fields.join("|"),
        relations.join("|"),
        invariants.join("|"),
        proofs.join("|"),
        receipts.join("|")
    );
    BootstrapCanonicalModelReport {
        model_count: surface.models.len(),
        schema_count: surface.schemas.len(),
        field_count: surface.fields.len(),
        relation_count: surface.relations.len(),
        invariant_count: surface.invariants.len(),
        proof_count: surface.proofs.len(),
        receipt_count: surface.receipts.len(),
        local_schema_count: surface.models.iter().filter(|x| x.local_schema()).count(),
        required_field_count: surface.fields.iter().filter(|x| x.required()).count(),
        receipt_bound_invariant_count: surface
            .invariants
            .iter()
            .filter(|x| x.receipt_bound())
            .count(),
        canonical_model_hash: stable_hash_label(
            "lyra.p02.bootstrap_canonical_model.report",
            &preimage,
        ),
    }
}
