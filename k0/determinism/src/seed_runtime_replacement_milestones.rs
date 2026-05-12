use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRuntimeReplacementMilestoneReport {
    pub id: String,
    pub target_id: String,
    pub target_class: String,
    pub replacement_unit: String,
    pub milestone_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRuntimeReplacementHandoffReport {
    pub id: String,
    pub target_id: String,
    pub receipt_count: usize,
    pub handoff_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRuntimeReplacementMilestoneSuiteReport {
    pub milestone_count: usize,
    pub handoff_count: usize,
    pub target_class_count: usize,
    pub native_successor_count: usize,
    pub fallback_binding_count: usize,
    pub phase_open_count: usize,
    pub receipt_count: usize,
    pub milestone_reports: Vec<SeedRuntimeReplacementMilestoneReport>,
    pub handoff_reports: Vec<SeedRuntimeReplacementHandoffReport>,
    pub suite_hash: String,
}

pub fn deterministic_seed_runtime_replacement_milestone_report(
    milestones: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    handoffs: &[(String, String, String, Vec<String>, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> SeedRuntimeReplacementMilestoneSuiteReport {
    let mut ordered_milestones = milestones.to_vec();
    ordered_milestones.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_handoffs = handoffs.to_vec();
    ordered_handoffs.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_receipts = receipts.to_vec();
    ordered_receipts.sort_by(|left, right| left.0.cmp(&right.0));

    let mut classes = Vec::new();
    let mut successors = Vec::new();
    let mut fallback_binding_count = 0usize;
    let mut phase_open_count = 0usize;
    let mut preimage = format!(
        "milestones:{}|handoffs:{}|receipts:{}",
        ordered_milestones.len(),
        ordered_handoffs.len(),
        ordered_receipts.len()
    );
    let mut milestone_reports = Vec::new();
    for (
        id,
        target_id,
        target_class,
        replacement_unit,
        foreign_surface_ref,
        native_successor,
        entry_gate,
        proof_gate,
        extinction_gate,
        fallback_ref,
        closure_claim,
        status,
    ) in ordered_milestones
    {
        classes.push(target_class.clone());
        successors.push(native_successor.clone());
        if fallback_ref.starts_with("fallback_") {
            fallback_binding_count += 1;
        }
        if closure_claim == "phase_open" {
            phase_open_count += 1;
        }
        let row = format!("milestone:{id}:{target_id}:{target_class}:{replacement_unit}:{foreign_surface_ref}:{native_successor}:{entry_gate}:{proof_gate}:{extinction_gate}:{fallback_ref}:{closure_claim}:{status}");
        preimage.push('|');
        preimage.push_str(&row);
        milestone_reports.push(SeedRuntimeReplacementMilestoneReport {
            id,
            target_id,
            target_class,
            replacement_unit,
            milestone_hash: stable_hash_label("lyra.p02.seed_runtime_replacement.milestone", &row),
        });
    }
    let mut handoff_reports = Vec::new();
    for (id, target_id, operator_role, mut required_receipts, truth_effect, import_gate, status) in
        ordered_handoffs
    {
        required_receipts.sort();
        let row = format!(
            "handoff:{}:{}:{}:{}:{}:{}:{}",
            id,
            target_id,
            operator_role,
            required_receipts.join(","),
            truth_effect,
            import_gate,
            status
        );
        preimage.push('|');
        preimage.push_str(&row);
        handoff_reports.push(SeedRuntimeReplacementHandoffReport {
            id,
            target_id,
            receipt_count: required_receipts.len(),
            handoff_hash: stable_hash_label("lyra.p02.seed_runtime_replacement.handoff", &row),
        });
    }
    for (id, path, target, status) in ordered_receipts {
        preimage.push_str(&format!("|receipt:{id}:{path}:{target}:{status}"));
    }
    classes.sort();
    classes.dedup();
    successors.sort();
    successors.dedup();
    SeedRuntimeReplacementMilestoneSuiteReport {
        milestone_count: milestones.len(),
        handoff_count: handoffs.len(),
        target_class_count: classes.len(),
        native_successor_count: successors.len(),
        fallback_binding_count,
        phase_open_count,
        receipt_count: receipts.len(),
        milestone_reports,
        handoff_reports,
        suite_hash: stable_hash_label("lyra.p02.seed_runtime_replacement.suite", &preimage),
    }
}
