use std::collections::BTreeMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBoundaryChallengeSuiteBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub boundary_surface: String,
    pub suite_kind: String,
    pub challenge_scope: String,
    pub adversarial_vector: String,
    pub expected_rejection: String,
    pub evidence: Vec<String>,
    pub receipt: String,
    pub status: String,
}
impl HostBoundaryChallengeSuiteBinding {
    pub fn canonical_identity(&self) -> String {
        format!("suite:{}", self.id)
    }
    pub fn binds_boundary_surface(&self) -> bool {
        self.boundary_surface.starts_with("surface:")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBoundaryProbeBinding {
    pub line_number: usize,
    pub id: String,
    pub suite_id: String,
    pub surface_ref: String,
    pub injected_claim: String,
    pub expected_error: String,
    pub containment_gate: String,
    pub evidence: Vec<String>,
    pub status: String,
}
impl HostBoundaryProbeBinding {
    pub fn canonical_identity(&self) -> String {
        format!("probe:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBoundaryChallengeReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl HostBoundaryChallengeReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostBoundaryChallengeSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub inventory_receipt: String,
    pub extinction_receipt: String,
    pub session_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub suites: Vec<HostBoundaryChallengeSuiteBinding>,
    pub probes: Vec<HostBoundaryProbeBinding>,
    pub receipts: Vec<HostBoundaryChallengeReceiptBinding>,
}
impl HostBoundaryChallengeSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn suite_by_id(&self, id: &str) -> Option<&HostBoundaryChallengeSuiteBinding> {
        self.suites.iter().find(|x| x.id == id)
    }
    pub fn probe_by_id(&self, id: &str) -> Option<&HostBoundaryProbeBinding> {
        self.probes.iter().find(|x| x.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&HostBoundaryChallengeReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
    pub fn probe_for_surface(&self, surface_ref: &str) -> Option<&HostBoundaryProbeBinding> {
        self.probes.iter().find(|x| x.surface_ref == surface_ref)
    }
}
