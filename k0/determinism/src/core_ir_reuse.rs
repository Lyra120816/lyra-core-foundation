use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseConsumerReport {
    pub id: String,
    pub surface: String,
    pub target_phase: String,
    pub core_ir_ref: String,
    pub consumer_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseEdgeReport {
    pub id: String,
    pub from_consumer: String,
    pub to_consumer: String,
    pub guard: String,
    pub edge_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseGateReport {
    pub id: String,
    pub scope: String,
    pub law: String,
    pub gate_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseSuiteReport {
    pub consumer_count: usize,
    pub edge_count: usize,
    pub gate_count: usize,
    pub receipt_count: usize,
    pub consumer_reports: Vec<CoreIrReuseConsumerReport>,
    pub edge_reports: Vec<CoreIrReuseEdgeReport>,
    pub gate_reports: Vec<CoreIrReuseGateReport>,
    pub receipt_reports: Vec<CoreIrReuseReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_core_ir_reuse_suite_report(
    consumers: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    edges: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    gates: &[(String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> CoreIrReuseSuiteReport {
    let mut consumer_reports: Vec<CoreIrReuseConsumerReport> = consumers.iter().map(|item| { let preimage = format!("consumer:{}|surface:{}|phase:{}|owner:{}|core_ir_ref:{}|adapter:{}|fixture:{}|status:{}", item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7); CoreIrReuseConsumerReport { id: item.0.clone(), surface: item.1.clone(), target_phase: item.2.clone(), core_ir_ref: item.4.clone(), consumer_hash: stable_hash_label("lyra.p01.core_ir_reuse.consumer", &preimage) } }).collect();
    consumer_reports.sort_by(|left, right| left.id.cmp(&right.id));
    let mut edge_reports: Vec<CoreIrReuseEdgeReport> = edges
        .iter()
        .map(|item| {
            let preimage = format!(
                "edge:{}|from:{}|to:{}|form:{}|guard:{}|rejection:{}|receipt:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7
            );
            CoreIrReuseEdgeReport {
                id: item.0.clone(),
                from_consumer: item.1.clone(),
                to_consumer: item.2.clone(),
                guard: item.4.clone(),
                edge_hash: stable_hash_label("lyra.p01.core_ir_reuse.edge", &preimage),
            }
        })
        .collect();
    edge_reports.sort_by(|left, right| left.id.cmp(&right.id));
    let mut gate_reports: Vec<CoreIrReuseGateReport> = gates
        .iter()
        .map(|item| {
            let preimage = format!(
                "gate:{}|scope:{}|law:{}|evidence:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            CoreIrReuseGateReport {
                id: item.0.clone(),
                scope: item.1.clone(),
                law: item.2.clone(),
                gate_hash: stable_hash_label("lyra.p01.core_ir_reuse.gate", &preimage),
            }
        })
        .collect();
    gate_reports.sort_by(|left, right| left.id.cmp(&right.id));
    let mut receipt_reports: Vec<CoreIrReuseReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|path:{}|target:{}|status:{}",
                item.0, item.1, item.2, item.3
            );
            CoreIrReuseReceiptReport {
                id: item.0.clone(),
                path: item.1.clone(),
                target: item.2.clone(),
                receipt_hash: stable_hash_label("lyra.p01.core_ir_reuse.receipt", &preimage),
            }
        })
        .collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));
    let mut suite_lines = Vec::new();
    for item in &consumer_reports {
        suite_lines.push(format!("consumer:{}|{}", item.id, item.consumer_hash));
    }
    for item in &edge_reports {
        suite_lines.push(format!("edge:{}|{}", item.id, item.edge_hash));
    }
    for item in &gate_reports {
        suite_lines.push(format!("gate:{}|{}", item.id, item.gate_hash));
    }
    for item in &receipt_reports {
        suite_lines.push(format!("receipt:{}|{}", item.id, item.receipt_hash));
    }
    suite_lines.sort();
    CoreIrReuseSuiteReport {
        consumer_count: consumer_reports.len(),
        edge_count: edge_reports.len(),
        gate_count: gate_reports.len(),
        receipt_count: receipt_reports.len(),
        consumer_reports,
        edge_reports,
        gate_reports,
        receipt_reports,
        suite_hash: stable_hash_label("lyra.p01.core_ir_reuse.suite", &suite_lines.join("\n")),
    }
}
