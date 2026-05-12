use crate::k0_hash::stable_hash_label;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTruthCleanupReport {
    pub cleanup_count: usize,
    pub frontier_count: usize,
    pub target_class_count: usize,
    pub receipt_count: usize,
    pub phase_open_count: usize,
    pub rollback_count: usize,
    pub cleanup_hash: String,
}
pub fn deterministic_bootstrap_truth_cleanup_report(
    cleanups: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        Vec<String>,
        String,
        String,
    )],
    frontiers: &[(String, String, String, String, String, String, String)],
) -> BootstrapTruthCleanupReport {
    let mut ordered_cleanups = cleanups.to_vec();
    ordered_cleanups.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ordered_frontiers = frontiers.to_vec();
    ordered_frontiers.sort_by(|a, b| a.0.cmp(&b.0));
    let mut classes = Vec::new();
    let mut phase_open = 0usize;
    let mut rollback = 0usize;
    let mut receipt_names = Vec::new();
    let mut preimage = format!(
        "cleanups:{}|frontiers:{}",
        ordered_cleanups.len(),
        ordered_frontiers.len()
    );
    for (
        id,
        target_id,
        class,
        proven,
        retired,
        truth,
        blocker,
        decision,
        mut receipts,
        path,
        status,
    ) in ordered_cleanups
    {
        receipts.sort();
        classes.push(class.clone());
        receipt_names.extend(receipts.clone());
        if path == "rollback_to_target_matrix" {
            rollback += 1;
        }
        preimage.push_str(&format!(
            "|cleanup:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            id,
            target_id,
            class,
            proven,
            retired,
            truth,
            blocker,
            decision,
            receipts.join(","),
            path,
            status
        ));
    }
    for (id, target_id, on_proven, on_retired, next, hold, closure) in ordered_frontiers {
        if closure == "phase_open" {
            phase_open += 1;
        }
        preimage.push_str(&format!(
            "|frontier:{id}:{target_id}:{on_proven}:{on_retired}:{next}:{hold}:{closure}"
        ));
    }
    classes.sort();
    classes.dedup();
    receipt_names.sort();
    receipt_names.dedup();
    BootstrapTruthCleanupReport {
        cleanup_count: cleanups.len(),
        frontier_count: frontiers.len(),
        target_class_count: classes.len(),
        receipt_count: receipt_names.len(),
        phase_open_count: phase_open,
        rollback_count: rollback,
        cleanup_hash: stable_hash_label("lyra.p02.bootstrap_truth_cleanup.report", &preimage),
    }
}
