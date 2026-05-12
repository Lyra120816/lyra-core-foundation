use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRetirementSurfaceReport {
    pub id: String,
    pub owner_root: String,
    pub surface_kind: String,
    pub path: String,
    pub status: String,
    pub surface_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRetirementGateReport {
    pub id: String,
    pub surface: String,
    pub trigger: String,
    pub action: String,
    pub evidence_count: usize,
    pub status: String,
    pub gate_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSupersessionReport {
    pub id: String,
    pub surface: String,
    pub replaced_by: String,
    pub archive: String,
    pub status: String,
    pub supersession_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRetirementReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
    pub receipt_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRetirementSupersessionReport {
    pub surface_count: usize,
    pub gate_count: usize,
    pub supersession_count: usize,
    pub receipt_count: usize,
    pub bootstrap_surface_count: usize,
    pub retained_surface_count: usize,
    pub retirement_scheduled_count: usize,
    pub semantic_native_successor_count: usize,
    pub surface_reports: Vec<SemanticRetirementSurfaceReport>,
    pub gate_reports: Vec<SemanticRetirementGateReport>,
    pub supersession_reports: Vec<SemanticSupersessionReport>,
    pub receipt_reports: Vec<SemanticRetirementReceiptReport>,
    pub law_hash: String,
}

pub fn deterministic_semantic_retirement_supersession_report(
    surfaces: &[(String, String, String, String, String)],
    gates: &[(String, String, String, String, Vec<String>, String)],
    supersessions: &[(String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> SemanticRetirementSupersessionReport {
    let mut sorted_surfaces = surfaces.to_vec();
    sorted_surfaces.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_gates = gates.to_vec();
    sorted_gates.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_supersessions = supersessions.to_vec();
    sorted_supersessions.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_receipts = receipts.to_vec();
    sorted_receipts.sort_by(|left, right| left.0.cmp(&right.0));

    let mut surface_reports = Vec::new();
    let mut gate_reports = Vec::new();
    let mut supersession_reports = Vec::new();
    let mut receipt_reports = Vec::new();
    let mut bootstrap_surface_count = 0usize;
    let mut retained_surface_count = 0usize;
    let mut retirement_scheduled_count = 0usize;
    let mut semantic_native_successor_count = 0usize;
    let mut preimage = format!(
        "surfaces:{}|gates:{}|supersessions:{}|receipts:{}",
        sorted_surfaces.len(),
        sorted_gates.len(),
        sorted_supersessions.len(),
        sorted_receipts.len()
    );

    for (id, owner_root, surface_kind, path, status) in sorted_surfaces {
        if surface_kind == "bootstrap" {
            bootstrap_surface_count += 1;
        }
        if status == "retained_by_law" {
            retained_surface_count += 1;
        }
        if status == "retirement_scheduled" {
            retirement_scheduled_count += 1;
        }
        let surface_preimage = format!("semantic-surface:{id}|owner:{owner_root}|kind:{surface_kind}|path:{path}|status:{status}");
        let surface_hash =
            stable_hash_label("lyra.p01.semantic.retirement.surface", &surface_preimage);
        preimage.push('|');
        preimage.push_str(&surface_preimage);
        surface_reports.push(SemanticRetirementSurfaceReport {
            id,
            owner_root,
            surface_kind,
            path,
            status,
            surface_hash,
        });
    }

    for (id, surface, trigger, action, mut evidence, status) in sorted_gates {
        evidence.sort();
        if trigger == "semantic_ir_native_equivalent"
            || trigger == "semantic_object_native_equivalent"
            || trigger == "lyralang_native_equivalent"
        {
            semantic_native_successor_count += 1;
        }
        let gate_preimage = format!(
            "semantic-gate:{}|surface:{}|trigger:{}|action:{}|evidence:{}|status:{}",
            id,
            surface,
            trigger,
            action,
            evidence.join(","),
            status
        );
        let gate_hash = stable_hash_label("lyra.p01.semantic.retirement.gate", &gate_preimage);
        preimage.push('|');
        preimage.push_str(&gate_preimage);
        gate_reports.push(SemanticRetirementGateReport {
            id,
            surface,
            trigger,
            action,
            evidence_count: evidence.len(),
            status,
            gate_hash,
        });
    }

    for (id, surface, replaced_by, archive, status) in sorted_supersessions {
        let supersession_preimage = format!("semantic-supersession:{id}|surface:{surface}|replaced_by:{replaced_by}|archive:{archive}|status:{status}");
        let supersession_hash = stable_hash_label(
            "lyra.p01.semantic.retirement.supersession",
            &supersession_preimage,
        );
        preimage.push('|');
        preimage.push_str(&supersession_preimage);
        supersession_reports.push(SemanticSupersessionReport {
            id,
            surface,
            replaced_by,
            archive,
            status,
            supersession_hash,
        });
    }

    for (id, path, target, status) in sorted_receipts {
        let receipt_preimage =
            format!("semantic-retirement-receipt:{id}|path:{path}|target:{target}|status:{status}");
        let receipt_hash =
            stable_hash_label("lyra.p01.semantic.retirement.receipt", &receipt_preimage);
        preimage.push('|');
        preimage.push_str(&receipt_preimage);
        receipt_reports.push(SemanticRetirementReceiptReport {
            id,
            path,
            target,
            status,
            receipt_hash,
        });
    }

    SemanticRetirementSupersessionReport {
        surface_count: surface_reports.len(),
        gate_count: gate_reports.len(),
        supersession_count: supersession_reports.len(),
        receipt_count: receipt_reports.len(),
        bootstrap_surface_count,
        retained_surface_count,
        retirement_scheduled_count,
        semantic_native_successor_count,
        surface_reports,
        gate_reports,
        supersession_reports,
        receipt_reports,
        law_hash: stable_hash_label("lyra.p01.semantic.retirement.law", &preimage),
    }
}
