use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBoundaryChallengeReportSet {
    pub suite_count: usize,
    pub probe_count: usize,
    pub covered_surface_count: usize,
    pub rejection_family_count: usize,
    pub containment_gate_count: usize,
    pub rejected_probe_count: usize,
    pub owner_root_count: usize,
    pub suite_set_hash: String,
}

pub fn deterministic_host_boundary_challenge_report(
    suites: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        Vec<String>,
    )],
    probes: &[(String, String, String, String, String, String, Vec<String>)],
) -> HostBoundaryChallengeReportSet {
    let mut ordered_suites = suites.to_vec();
    ordered_suites.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_probes = probes.to_vec();
    ordered_probes.sort_by(|left, right| left.0.cmp(&right.0));
    let mut covered_surfaces = Vec::new();
    let mut rejection_families = Vec::new();
    let mut containment_gates = Vec::new();
    let mut owner_roots = Vec::new();
    let mut rejected_probe_count = 0usize;
    let mut preimage = format!(
        "suites:{}|probes:{}",
        ordered_suites.len(),
        ordered_probes.len()
    );
    for (
        id,
        owner_root,
        boundary_surface,
        suite_kind,
        challenge_scope,
        adversarial_vector,
        expected_rejection,
        mut evidence,
    ) in ordered_suites
    {
        evidence.sort();
        owner_roots.push(owner_root.clone());
        covered_surfaces.push(boundary_surface.clone());
        rejection_families.push(expected_rejection.clone());
        preimage.push_str(&format!(
            "|suite:{}:{}:{}:{}:{}:{}:{}:{}",
            id,
            owner_root,
            boundary_surface,
            suite_kind,
            challenge_scope,
            adversarial_vector,
            expected_rejection,
            evidence.join(",")
        ));
    }
    for (
        id,
        suite_id,
        surface_ref,
        injected_claim,
        expected_error,
        containment_gate,
        mut evidence,
    ) in ordered_probes
    {
        evidence.sort();
        covered_surfaces.push(surface_ref.clone());
        rejection_families.push(expected_error.clone());
        containment_gates.push(containment_gate.clone());
        if expected_error != "none" {
            rejected_probe_count += 1;
        }
        preimage.push_str(&format!(
            "|probe:{}:{}:{}:{}:{}:{}:{}",
            id,
            suite_id,
            surface_ref,
            injected_claim,
            expected_error,
            containment_gate,
            evidence.join(",")
        ));
    }
    covered_surfaces.sort();
    covered_surfaces.dedup();
    rejection_families.sort();
    rejection_families.dedup();
    containment_gates.sort();
    containment_gates.dedup();
    owner_roots.sort();
    owner_roots.dedup();
    HostBoundaryChallengeReportSet {
        suite_count: suites.len(),
        probe_count: probes.len(),
        covered_surface_count: covered_surfaces.len(),
        rejection_family_count: rejection_families.len(),
        containment_gate_count: containment_gates.len(),
        rejected_probe_count,
        owner_root_count: owner_roots.len(),
        suite_set_hash: stable_hash_label("lyra.p02.host_boundary_challenge.report_set", &preimage),
    }
}
