use std::collections::BTreeMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTargetBinding {
    pub line_number: usize,
    pub id: String,
    pub target_class: String,
    pub architecture: String,
    pub runtime_lane: String,
    pub proof_mode: String,
    pub owner_root: String,
    pub bootstrap_surface: String,
    pub evidence: Vec<String>,
    pub status: String,
}
impl BootstrapTargetBinding {
    pub fn canonical_identity(&self) -> String {
        format!("target:{}", self.id)
    }
    pub fn binds_bootstrap_surface(&self) -> bool {
        self.bootstrap_surface.starts_with("surface:")
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTargetProofBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub proof_family: String,
    pub required_evidence: Vec<String>,
    pub host_boundary_gate: String,
    pub status: String,
}
impl BootstrapTargetProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTargetReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl BootstrapTargetReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTargetMatrixSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub inventory_receipt: String,
    pub extinction_receipt: String,
    pub seed_runtime_receipt: String,
    pub host_boundary_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub targets: Vec<BootstrapTargetBinding>,
    pub proofs: Vec<BootstrapTargetProofBinding>,
    pub receipts: Vec<BootstrapTargetReceiptBinding>,
}
impl BootstrapTargetMatrixSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn target_by_id(&self, id: &str) -> Option<&BootstrapTargetBinding> {
        self.targets.iter().find(|x| x.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapTargetProofBinding> {
        self.proofs.iter().find(|x| x.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapTargetReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
    pub fn proofs_for_target(&self, target_id: &str) -> Vec<&BootstrapTargetProofBinding> {
        self.proofs
            .iter()
            .filter(|x| x.target_id == target_id)
            .collect()
    }
}
