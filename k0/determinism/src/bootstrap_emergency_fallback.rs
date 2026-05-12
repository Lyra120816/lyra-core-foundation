use crate::k0_hash::stable_hash_label;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEmergencyFallbackReport {
    pub fallback_count: usize,
    pub rollback_count: usize,
    pub target_class_count: usize,
    pub phase_open_count: usize,
    pub quarantine_count: usize,
    pub replay_gate_count: usize,
    pub receipt_reference_count: usize,
    pub emergency_hash: String,
}
pub fn deterministic_bootstrap_emergency_fallback_report(
    fallbacks: &[(
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
    rollbacks: &[(
        String,
        String,
        String,
        String,
        String,
        Vec<String>,
        String,
        String,
        String,
    )],
) -> BootstrapEmergencyFallbackReport {
    let mut ordered_fallbacks = fallbacks.to_vec();
    ordered_fallbacks.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ordered_rollbacks = rollbacks.to_vec();
    ordered_rollbacks.sort_by(|a, b| a.0.cmp(&b.0));
    let mut classes = Vec::new();
    let mut phase_open = 0usize;
    let mut quarantine = 0usize;
    let mut replay = 0usize;
    let mut receipt_refs = Vec::new();
    let mut preimage = format!(
        "fallbacks:{}|rollbacks:{}",
        ordered_fallbacks.len(),
        ordered_rollbacks.len()
    );
    for (
        id,
        target_id,
        class,
        failure,
        freeze,
        action,
        path,
        last_good,
        challenge,
        operator,
        closure,
        status,
    ) in ordered_fallbacks
    {
        classes.push(class.clone());
        if closure == "phase_open" {
            phase_open += 1;
        }
        if action == "enter_bounded_failure_quarantine" {
            quarantine += 1;
        }
        receipt_refs.push(last_good.clone());
        preimage.push_str(&format!("|fallback:{id}:{target_id}:{class}:{failure}:{freeze}:{action}:{path}:{last_good}:{challenge}:{operator}:{closure}:{status}"));
    }
    for (id, target_id, trigger, from_state, to_state, mut receipts, gate, decision, status) in
        ordered_rollbacks
    {
        receipts.sort();
        receipt_refs.extend(receipts.clone());
        if gate == "post_rollback_replay_required" {
            replay += 1;
        }
        preimage.push_str(&format!(
            "|rollback:{}:{}:{}:{}:{}:{}:{}:{}",
            id,
            target_id,
            trigger,
            from_state,
            to_state,
            receipts.join(","),
            gate,
            decision
        ));
        preimage.push_str(&format!(":{status}"));
    }
    classes.sort();
    classes.dedup();
    receipt_refs.sort();
    receipt_refs.dedup();
    BootstrapEmergencyFallbackReport {
        fallback_count: fallbacks.len(),
        rollback_count: rollbacks.len(),
        target_class_count: classes.len(),
        phase_open_count: phase_open,
        quarantine_count: quarantine,
        replay_gate_count: replay,
        receipt_reference_count: receipt_refs.len(),
        emergency_hash: stable_hash_label(
            "lyra.p02.bootstrap_emergency_fallback.report",
            &preimage,
        ),
    }
}
