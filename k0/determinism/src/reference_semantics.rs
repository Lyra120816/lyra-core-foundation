use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceLiteralReport {
    pub id: String,
    pub atom: String,
    pub canonical: String,
    pub normal: String,
    pub literal_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceCompositionReport {
    pub id: String,
    pub operator: String,
    pub arity: String,
    pub law: String,
    pub composition_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceEvalSeedReport {
    pub id: String,
    pub input: String,
    pub expected: String,
    pub trace: String,
    pub seed_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceSemanticsReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceSemanticsSuiteReport {
    pub literal_count: usize,
    pub composition_count: usize,
    pub eval_seed_count: usize,
    pub receipt_count: usize,
    pub traced_eval_seed_count: usize,
    pub literal_reports: Vec<ReferenceLiteralReport>,
    pub composition_reports: Vec<ReferenceCompositionReport>,
    pub eval_seed_reports: Vec<ReferenceEvalSeedReport>,
    pub receipt_reports: Vec<ReferenceSemanticsReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_reference_semantics_suite_report(
    literals: &[(String, String, String, String, String, String, String)],
    compositions: &[(String, String, String, String, String, String, String)],
    eval_seeds: &[(String, String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> ReferenceSemanticsSuiteReport {
    let mut literal_reports: Vec<ReferenceLiteralReport> = literals
        .iter()
        .map(|item| {
            let preimage = format!(
                "literal:{}|atom:{}|canonical:{}|normal:{}|evaluator:{}|proof:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            ReferenceLiteralReport {
                id: item.0.clone(),
                atom: item.1.clone(),
                canonical: item.2.clone(),
                normal: item.3.clone(),
                literal_hash: stable_hash_label("lyra.p01.reference_semantics.literal", &preimage),
            }
        })
        .collect();
    literal_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut composition_reports: Vec<ReferenceCompositionReport> = compositions
        .iter()
        .map(|item| {
            let preimage = format!(
                "composition:{}|operator:{}|arity:{}|input_order:{}|output:{}|law:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            ReferenceCompositionReport {
                id: item.0.clone(),
                operator: item.1.clone(),
                arity: item.2.clone(),
                law: item.5.clone(),
                composition_hash: stable_hash_label(
                    "lyra.p01.reference_semantics.composition",
                    &preimage,
                ),
            }
        })
        .collect();
    composition_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut eval_seed_reports: Vec<ReferenceEvalSeedReport> = eval_seeds
        .iter()
        .map(|item| {
            let preimage = format!(
                "eval_seed:{}|input:{}|reduction:{}|expected:{}|law:{}|trace:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            ReferenceEvalSeedReport {
                id: item.0.clone(),
                input: item.1.clone(),
                expected: item.3.clone(),
                trace: item.5.clone(),
                seed_hash: stable_hash_label("lyra.p01.reference_semantics.seed", &preimage),
            }
        })
        .collect();
    eval_seed_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut receipt_reports: Vec<ReferenceSemanticsReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|path:{}|target:{}|status:{}",
                item.0, item.1, item.2, item.3
            );
            ReferenceSemanticsReceiptReport {
                id: item.0.clone(),
                path: item.1.clone(),
                target: item.2.clone(),
                receipt_hash: stable_hash_label("lyra.p01.reference_semantics.receipt", &preimage),
            }
        })
        .collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let traced_eval_seed_count = eval_seed_reports
        .iter()
        .filter(|item| item.trace.starts_with("fnv1a128:"))
        .count();
    let mut suite_preimage = String::new();
    for item in &literal_reports {
        suite_preimage.push_str(&item.literal_hash);
        suite_preimage.push('\n');
    }
    for item in &composition_reports {
        suite_preimage.push_str(&item.composition_hash);
        suite_preimage.push('\n');
    }
    for item in &eval_seed_reports {
        suite_preimage.push_str(&item.seed_hash);
        suite_preimage.push('\n');
    }
    for item in &receipt_reports {
        suite_preimage.push_str(&item.receipt_hash);
        suite_preimage.push('\n');
    }
    let suite_hash = stable_hash_label("lyra.p01.reference_semantics.suite", &suite_preimage);

    ReferenceSemanticsSuiteReport {
        literal_count: literal_reports.len(),
        composition_count: composition_reports.len(),
        eval_seed_count: eval_seed_reports.len(),
        receipt_count: receipt_reports.len(),
        traced_eval_seed_count,
        literal_reports,
        composition_reports,
        eval_seed_reports,
        receipt_reports,
        suite_hash,
    }
}
