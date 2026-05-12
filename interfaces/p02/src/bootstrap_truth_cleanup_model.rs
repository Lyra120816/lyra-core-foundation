use std::collections::BTreeMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCleanupBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub target_class: String,
    pub proven_action: String,
    pub retired_action: String,
    pub truth_update: String,
    pub blocker_update: String,
    pub frontier_decision: String,
    pub required_receipts: Vec<String>,
    pub rollback_path: String,
    pub status: String,
}
impl BootstrapCleanupBinding {
    pub fn canonical_identity(&self) -> String {
        format!("cleanup:{}", self.id)
    }
    pub fn binds_target(&self, target: &str) -> bool {
        self.target_id == target
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFrontierAdvanceBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub on_proven: String,
    pub on_retired: String,
    pub next_frontier: String,
    pub hold_if_pending: String,
    pub closure_claim: String,
    pub status: String,
}
impl BootstrapFrontierAdvanceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("frontier:{}", self.id)
    }
    pub fn holds_phase_open(&self) -> bool {
        self.closure_claim == "phase_open"
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTruthCleanupReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl BootstrapTruthCleanupReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTruthCleanupSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub target_matrix_receipt: String,
    pub host_boundary_receipt: String,
    pub extinction_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub cleanups: Vec<BootstrapCleanupBinding>,
    pub frontiers: Vec<BootstrapFrontierAdvanceBinding>,
    pub receipts: Vec<BootstrapTruthCleanupReceiptBinding>,
}
impl BootstrapTruthCleanupSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn cleanup_by_target(&self, target: &str) -> Option<&BootstrapCleanupBinding> {
        self.cleanups.iter().find(|x| x.target_id == target)
    }
    pub fn frontier_by_target(&self, target: &str) -> Option<&BootstrapFrontierAdvanceBinding> {
        self.frontiers.iter().find(|x| x.target_id == target)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapTruthCleanupReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
}
