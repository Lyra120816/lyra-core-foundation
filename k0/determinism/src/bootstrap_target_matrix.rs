use crate::k0_hash::stable_hash_label;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTargetMatrixReport {
    pub target_count: usize,
    pub proof_count: usize,
    pub target_class_count: usize,
    pub architecture_count: usize,
    pub runtime_lane_count: usize,
    pub proof_family_count: usize,
    pub containment_gate_count: usize,
    pub pending_validation_count: usize,
    pub matrix_hash: String,
}
pub fn deterministic_bootstrap_target_matrix_report(
    targets: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        Vec<String>,
    )],
    proofs: &[(String, String, String, Vec<String>, String, String)],
) -> BootstrapTargetMatrixReport {
    let mut ordered_targets = targets.to_vec();
    ordered_targets.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ordered_proofs = proofs.to_vec();
    ordered_proofs.sort_by(|a, b| a.0.cmp(&b.0));
    let mut classes = Vec::new();
    let mut archs = Vec::new();
    let mut lanes = Vec::new();
    let mut families = Vec::new();
    let mut gates = Vec::new();
    let mut pending = 0usize;
    let mut preimage = format!(
        "targets:{}|proofs:{}",
        ordered_targets.len(),
        ordered_proofs.len()
    );
    for (id, class, arch, lane, mode, owner, surface, mut evidence) in ordered_targets {
        evidence.sort();
        classes.push(class.clone());
        archs.push(arch.clone());
        lanes.push(lane.clone());
        preimage.push_str(&format!(
            "|target:{}:{}:{}:{}:{}:{}:{}:{}",
            id,
            class,
            arch,
            lane,
            mode,
            owner,
            surface,
            evidence.join(",")
        ));
    }
    for (id, target_id, family, mut evidence, gate, status) in ordered_proofs {
        evidence.sort();
        families.push(family.clone());
        gates.push(gate.clone());
        if status == "pending_local_validation" {
            pending += 1;
        }
        preimage.push_str(&format!(
            "|proof:{}:{}:{}:{}:{}:{}",
            id,
            target_id,
            family,
            evidence.join(","),
            gate,
            status
        ));
    }
    classes.sort();
    classes.dedup();
    archs.sort();
    archs.dedup();
    lanes.sort();
    lanes.dedup();
    families.sort();
    families.dedup();
    gates.sort();
    gates.dedup();
    BootstrapTargetMatrixReport {
        target_count: targets.len(),
        proof_count: proofs.len(),
        target_class_count: classes.len(),
        architecture_count: archs.len(),
        runtime_lane_count: lanes.len(),
        proof_family_count: families.len(),
        containment_gate_count: gates.len(),
        pending_validation_count: pending,
        matrix_hash: stable_hash_label("lyra.p02.bootstrap_target_matrix.report", &preimage),
    }
}
