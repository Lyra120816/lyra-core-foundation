use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDataModelReport {
    pub id: String,
    pub model_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDataSchemaReport {
    pub id: String,
    pub schema_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDataFieldReport {
    pub id: String,
    pub field_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDataBridgeReport {
    pub id: String,
    pub bridge_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDataProofReport {
    pub id: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDataModelSuiteReport {
    pub model_count: usize,
    pub schema_count: usize,
    pub field_count: usize,
    pub bridge_count: usize,
    pub proof_count: usize,
    pub model_reports: Vec<CanonicalDataModelReport>,
    pub schema_reports: Vec<CanonicalDataSchemaReport>,
    pub field_reports: Vec<CanonicalDataFieldReport>,
    pub bridge_reports: Vec<CanonicalDataBridgeReport>,
    pub proof_reports: Vec<CanonicalDataProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_canonical_data_model_suite_report(
    models: &[(String, String, String, String, String, Vec<String>, String)],
    schemas: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    fields: &[(String, String, String, String, String, String)],
    bridges: &[(String, String, String, String, String, String)],
    proofs: &[(
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
        String,
        String,
        String,
    )],
) -> CanonicalDataModelSuiteReport {
    let mut model_reports: Vec<CanonicalDataModelReport> = models
        .iter()
        .map(|item| {
            let preimage = format!(
                "model:{}|scope:{}|owner:{}|source:{}|schema:{}|order:{}|status:{}",
                item.0,
                item.1,
                item.2,
                item.3,
                item.4,
                sorted_join(&item.5),
                item.6
            );
            CanonicalDataModelReport {
                id: item.0.clone(),
                model_hash: stable_hash_label("lyra.p01.canonical_data_model.model", &preimage),
            }
        })
        .collect();
    model_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut schema_reports: Vec<CanonicalDataSchemaReport> = schemas
        .iter()
        .map(|item| {
            let preimage = format!(
                "schema:{}|model:{}|fields:{}|required:{}|forbids:{}|status:{}",
                item.0,
                item.1,
                sorted_join(&item.2),
                sorted_join(&item.3),
                sorted_join(&item.4),
                item.5
            );
            CanonicalDataSchemaReport {
                id: item.0.clone(),
                schema_hash: stable_hash_label("lyra.p01.canonical_data_model.schema", &preimage),
            }
        })
        .collect();
    schema_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut field_reports: Vec<CanonicalDataFieldReport> = fields
        .iter()
        .map(|item| {
            let preimage = format!(
                "field:{}|model:{}|kind:{}|order:{}|normalization:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            CanonicalDataFieldReport {
                id: item.0.clone(),
                field_hash: stable_hash_label("lyra.p01.canonical_data_model.field", &preimage),
            }
        })
        .collect();
    field_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut bridge_reports: Vec<CanonicalDataBridgeReport> = bridges
        .iter()
        .map(|item| {
            let preimage = format!(
                "bridge:{}|from:{}|to:{}|carrier:{}|receipt:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            CanonicalDataBridgeReport {
                id: item.0.clone(),
                bridge_hash: stable_hash_label("lyra.p01.canonical_data_model.bridge", &preimage),
            }
        })
        .collect();
    bridge_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<CanonicalDataProofReport> = proofs.iter().map(|item| {
        let preimage = format!("proof:{}|models:{}|schemas:{}|fields:{}|bridges:{}|fixture:{}|golden:{}|receipt:{}|status:{}", item.0, sorted_join(&item.1), sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), item.5, item.6, item.7, item.8);
        CanonicalDataProofReport { id: item.0.clone(), proof_hash: stable_hash_label("lyra.p01.canonical_data_model.proof", &preimage) }
    }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let preimage = format!(
        "models:{}|schemas:{}|fields:{}|bridges:{}|proofs:{}|model_hashes:{}|schema_hashes:{}|field_hashes:{}|bridge_hashes:{}|proof_hashes:{}",
        model_reports.len(), schema_reports.len(), field_reports.len(), bridge_reports.len(), proof_reports.len(),
        model_reports.iter().map(|item| item.model_hash.as_str()).collect::<Vec<_>>().join(","),
        schema_reports.iter().map(|item| item.schema_hash.as_str()).collect::<Vec<_>>().join(","),
        field_reports.iter().map(|item| item.field_hash.as_str()).collect::<Vec<_>>().join(","),
        bridge_reports.iter().map(|item| item.bridge_hash.as_str()).collect::<Vec<_>>().join(","),
        proof_reports.iter().map(|item| item.proof_hash.as_str()).collect::<Vec<_>>().join(","),
    );
    let suite_hash = stable_hash_label("lyra.p01.canonical_data_model.suite", &preimage);

    CanonicalDataModelSuiteReport {
        model_count: model_reports.len(),
        schema_count: schema_reports.len(),
        field_count: field_reports.len(),
        bridge_count: bridge_reports.len(),
        proof_count: proof_reports.len(),
        model_reports,
        schema_reports,
        field_reports,
        bridge_reports,
        proof_reports,
        suite_hash,
    }
}

fn sorted_join(values: &[String]) -> String {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values.join(",")
}
