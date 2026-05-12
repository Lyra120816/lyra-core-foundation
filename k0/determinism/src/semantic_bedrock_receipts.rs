use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockReceiptReport {
    pub id: String,
    pub task: String,
    pub receipt_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockAnchorReport {
    pub id: String,
    pub core_ref: String,
    pub anchor_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockParityFixtureReport {
    pub id: String,
    pub receipt_ref: String,
    pub fixture_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockGateReport {
    pub id: String,
    pub law: String,
    pub gate_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockReceiptSuiteReport {
    pub receipt_count: usize,
    pub anchor_count: usize,
    pub fixture_count: usize,
    pub gate_count: usize,
    pub receipt_reports: Vec<SemanticBedrockReceiptReport>,
    pub anchor_reports: Vec<SemanticBedrockAnchorReport>,
    pub fixture_reports: Vec<SemanticBedrockParityFixtureReport>,
    pub gate_reports: Vec<SemanticBedrockGateReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_bedrock_receipt_suite_report(
    receipts: &[(String, String, String, String, String, String)],
    anchors: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    fixtures: &[(String, String, String, String, String, String)],
    gates: &[(String, String, String, String, String)],
) -> SemanticBedrockReceiptSuiteReport {
    let mut receipt_reports: Vec<SemanticBedrockReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|task:{}|surface:{}|path:{}|expected:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            SemanticBedrockReceiptReport {
                id: item.0.clone(),
                task: item.1.clone(),
                receipt_hash: stable_hash_label(
                    "lyra.p01.semantic_bedrock_receipts.receipt",
                    &preimage,
                ),
            }
        })
        .collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut anchor_reports: Vec<SemanticBedrockAnchorReport> = anchors
        .iter()
        .map(|item| {
            let preimage = format!(
                "anchor:{}|owner:{}|module:{}|contract:{}|law:{}|receipt:{}|core:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7
            );
            SemanticBedrockAnchorReport {
                id: item.0.clone(),
                core_ref: item.6.clone(),
                anchor_hash: stable_hash_label(
                    "lyra.p01.semantic_bedrock_receipts.anchor",
                    &preimage,
                ),
            }
        })
        .collect();
    anchor_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut fixture_reports: Vec<SemanticBedrockParityFixtureReport> = fixtures
        .iter()
        .map(|item| {
            let preimage = format!(
                "fixture:{}|positive:{}|negative:{}|receipt:{}|golden:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            SemanticBedrockParityFixtureReport {
                id: item.0.clone(),
                receipt_ref: item.3.clone(),
                fixture_hash: stable_hash_label(
                    "lyra.p01.semantic_bedrock_receipts.fixture",
                    &preimage,
                ),
            }
        })
        .collect();
    fixture_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut gate_reports: Vec<SemanticBedrockGateReport> = gates
        .iter()
        .map(|item| {
            let preimage = format!(
                "gate:{}|scope:{}|law:{}|evidence:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            SemanticBedrockGateReport {
                id: item.0.clone(),
                law: item.2.clone(),
                gate_hash: stable_hash_label("lyra.p01.semantic_bedrock_receipts.gate", &preimage),
            }
        })
        .collect();
    gate_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let preimage = format!(
        "receipts:{}|anchors:{}|fixtures:{}|gates:{}|receipt_hashes:{}|anchor_hashes:{}|fixture_hashes:{}|gate_hashes:{}",
        receipt_reports.len(),
        anchor_reports.len(),
        fixture_reports.len(),
        gate_reports.len(),
        receipt_reports.iter().map(|item| item.receipt_hash.as_str()).collect::<Vec<_>>().join(","),
        anchor_reports.iter().map(|item| item.anchor_hash.as_str()).collect::<Vec<_>>().join(","),
        fixture_reports.iter().map(|item| item.fixture_hash.as_str()).collect::<Vec<_>>().join(","),
        gate_reports.iter().map(|item| item.gate_hash.as_str()).collect::<Vec<_>>().join(","),
    );
    let suite_hash = stable_hash_label("lyra.p01.semantic_bedrock_receipts.suite", &preimage);

    SemanticBedrockReceiptSuiteReport {
        receipt_count: receipt_reports.len(),
        anchor_count: anchor_reports.len(),
        fixture_count: fixture_reports.len(),
        gate_count: gate_reports.len(),
        receipt_reports,
        anchor_reports,
        fixture_reports,
        gate_reports,
        suite_hash,
    }
}
