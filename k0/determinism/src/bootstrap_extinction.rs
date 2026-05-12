use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapExtinctionEntryReport {
    pub id: String,
    pub owner_root: String,
    pub classification: String,
    pub deletion_action: String,
    pub ledger_state: String,
    pub evidence_count: usize,
    pub entry_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRetirementGateReport {
    pub id: String,
    pub surface: String,
    pub gate_kind: String,
    pub trigger: String,
    pub allowed_action: String,
    pub status: String,
    pub evidence_count: usize,
    pub gate_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapExtinctionReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapExtinctionLedgerReport {
    pub entry_count: usize,
    pub gate_count: usize,
    pub receipt_count: usize,
    pub temporary_count: usize,
    pub observer_count: usize,
    pub bounded_permanent_count: usize,
    pub forbidden_count: usize,
    pub deletion_scheduled_count: usize,
    pub retained_by_target_descriptor_count: usize,
    pub denied_import_count: usize,
    pub entry_reports: Vec<BootstrapExtinctionEntryReport>,
    pub gate_reports: Vec<BootstrapRetirementGateReport>,
    pub receipt_reports: Vec<BootstrapExtinctionReceiptReport>,
    pub ledger_hash: String,
}

pub fn deterministic_bootstrap_extinction_ledger_report(
    entries: &[(String, String, String, String, String, Vec<String>)],
    gates: &[(String, String, String, String, String, Vec<String>, String)],
    receipts: &[(String, String, String, String)],
) -> BootstrapExtinctionLedgerReport {
    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_gates = gates.to_vec();
    sorted_gates.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_receipts = receipts.to_vec();
    sorted_receipts.sort_by(|left, right| left.0.cmp(&right.0));

    let mut entry_reports = Vec::new();
    let mut gate_reports = Vec::new();
    let mut receipt_reports = Vec::new();
    let mut temporary_count = 0usize;
    let mut observer_count = 0usize;
    let mut bounded_permanent_count = 0usize;
    let mut forbidden_count = 0usize;
    let mut deletion_scheduled_count = 0usize;
    let mut retained_by_target_descriptor_count = 0usize;
    let mut denied_import_count = 0usize;
    let mut preimage = format!(
        "entries:{}|gates:{}|receipts:{}",
        sorted_entries.len(),
        sorted_gates.len(),
        sorted_receipts.len()
    );

    for (id, owner_root, classification, deletion_action, ledger_state, mut evidence) in
        sorted_entries
    {
        evidence.sort();
        match classification.as_str() {
            "temporary" => temporary_count += 1,
            "observer" => observer_count += 1,
            "bounded_permanent" => bounded_permanent_count += 1,
            "forbidden" => forbidden_count += 1,
            _ => {}
        }
        if ledger_state == "deletion_scheduled" {
            deletion_scheduled_count += 1;
        }
        if ledger_state == "retained_by_target_descriptor" {
            retained_by_target_descriptor_count += 1;
        }
        if ledger_state == "forbidden_no_import" {
            denied_import_count += 1;
        }
        let entry_preimage = format!(
            "entry:{}|owner:{}|classification:{}|action:{}|state:{}|evidence:{}",
            id,
            owner_root,
            classification,
            deletion_action,
            ledger_state,
            evidence.join(",")
        );
        let entry_hash = stable_hash_label("lyra.p02.extinction.entry", &entry_preimage);
        preimage.push('|');
        preimage.push_str(&entry_preimage);
        entry_reports.push(BootstrapExtinctionEntryReport {
            id,
            owner_root,
            classification,
            deletion_action,
            ledger_state,
            evidence_count: evidence.len(),
            entry_hash,
        });
    }

    for (id, surface, gate_kind, trigger, allowed_action, mut evidence, status) in sorted_gates {
        evidence.sort();
        let gate_preimage = format!(
            "gate:{}|surface:{}|kind:{}|trigger:{}|action:{}|evidence:{}|status:{}",
            id,
            surface,
            gate_kind,
            trigger,
            allowed_action,
            evidence.join(","),
            status
        );
        let gate_hash = stable_hash_label("lyra.p02.extinction.gate", &gate_preimage);
        preimage.push('|');
        preimage.push_str(&gate_preimage);
        gate_reports.push(BootstrapRetirementGateReport {
            id,
            surface,
            gate_kind,
            trigger,
            allowed_action,
            status,
            evidence_count: evidence.len(),
            gate_hash,
        });
    }

    for (id, path, target, status) in sorted_receipts {
        let receipt_preimage = format!("receipt:{id}|path:{path}|target:{target}|status:{status}");
        let receipt_hash = stable_hash_label("lyra.p02.extinction.receipt", &receipt_preimage);
        preimage.push('|');
        preimage.push_str(&receipt_preimage);
        receipt_reports.push(BootstrapExtinctionReceiptReport {
            id,
            path,
            target,
            status,
            receipt_hash,
        });
    }

    BootstrapExtinctionLedgerReport {
        entry_count: entry_reports.len(),
        gate_count: gate_reports.len(),
        receipt_count: receipt_reports.len(),
        temporary_count,
        observer_count,
        bounded_permanent_count,
        forbidden_count,
        deletion_scheduled_count,
        retained_by_target_descriptor_count,
        denied_import_count,
        entry_reports,
        gate_reports,
        receipt_reports,
        ledger_hash: stable_hash_label("lyra.p02.extinction.ledger", &preimage),
    }
}
