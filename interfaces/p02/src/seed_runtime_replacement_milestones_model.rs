use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRuntimeReplacementMilestoneBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub target_class: String,
    pub replacement_unit: String,
    pub foreign_surface_ref: String,
    pub native_successor: String,
    pub entry_gate: String,
    pub proof_gate: String,
    pub extinction_gate: String,
    pub fallback_ref: String,
    pub closure_claim: String,
    pub status: String,
}
impl SeedRuntimeReplacementMilestoneBinding {
    pub fn canonical_identity(&self) -> String {
        format!("milestone:{}", self.id)
    }
    pub fn binds_target(&self, target: &str) -> bool {
        self.target_id == target
    }
    pub fn holds_phase_open(&self) -> bool {
        self.closure_claim == "phase_open"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRuntimeReplacementHandoffBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub operator_role: String,
    pub required_receipts: Vec<String>,
    pub truth_effect: String,
    pub import_gate: String,
    pub status: String,
}
impl SeedRuntimeReplacementHandoffBinding {
    pub fn canonical_identity(&self) -> String {
        format!("handoff:{}", self.id)
    }
    pub fn binds_target(&self, target: &str) -> bool {
        self.target_id == target
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRuntimeReplacementReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl SeedRuntimeReplacementReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRuntimeReplacementMilestoneSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub seed_runtime_contract_receipt: String,
    pub target_matrix_receipt: String,
    pub emergency_fallback_receipt: String,
    pub extinction_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub milestones: Vec<SeedRuntimeReplacementMilestoneBinding>,
    pub handoffs: Vec<SeedRuntimeReplacementHandoffBinding>,
    pub receipts: Vec<SeedRuntimeReplacementReceiptBinding>,
}
impl SeedRuntimeReplacementMilestoneSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn milestone_by_target(
        &self,
        target: &str,
    ) -> Option<&SeedRuntimeReplacementMilestoneBinding> {
        self.milestones.iter().find(|x| x.target_id == target)
    }
    pub fn handoff_by_target(&self, target: &str) -> Option<&SeedRuntimeReplacementHandoffBinding> {
        self.handoffs.iter().find(|x| x.target_id == target)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SeedRuntimeReplacementReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
}
