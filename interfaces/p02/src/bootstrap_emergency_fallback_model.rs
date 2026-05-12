use std::collections::BTreeMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEmergencyFallbackBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub target_class: String,
    pub failure_state: String,
    pub freeze_action: String,
    pub fallback_action: String,
    pub rollback_path: String,
    pub last_good_receipt: String,
    pub required_challenge: String,
    pub operator_state: String,
    pub closure_claim: String,
    pub status: String,
}
impl BootstrapEmergencyFallbackBinding {
    pub fn canonical_identity(&self) -> String {
        format!("fallback:{}", self.id)
    }
    pub fn holds_phase_open(&self) -> bool {
        self.closure_claim == "phase_open"
    }
    pub fn binds_target(&self, target: &str) -> bool {
        self.target_id == target
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEmergencyRollbackBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub trigger: String,
    pub from_state: String,
    pub to_state: String,
    pub required_receipts: Vec<String>,
    pub replay_gate: String,
    pub frontier_decision: String,
    pub status: String,
}
impl BootstrapEmergencyRollbackBinding {
    pub fn canonical_identity(&self) -> String {
        format!("rollback:{}", self.id)
    }
    pub fn binds_target(&self, target: &str) -> bool {
        self.target_id == target
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEmergencyFallbackReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl BootstrapEmergencyFallbackReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEmergencyFallbackSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub target_matrix_receipt: String,
    pub truth_cleanup_receipt: String,
    pub host_boundary_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub fallbacks: Vec<BootstrapEmergencyFallbackBinding>,
    pub rollbacks: Vec<BootstrapEmergencyRollbackBinding>,
    pub receipts: Vec<BootstrapEmergencyFallbackReceiptBinding>,
}
impl BootstrapEmergencyFallbackSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn fallback_by_target(&self, target: &str) -> Option<&BootstrapEmergencyFallbackBinding> {
        self.fallbacks.iter().find(|x| x.target_id == target)
    }
    pub fn rollback_by_target(&self, target: &str) -> Option<&BootstrapEmergencyRollbackBinding> {
        self.rollbacks.iter().find(|x| x.target_id == target)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapEmergencyFallbackReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
}
