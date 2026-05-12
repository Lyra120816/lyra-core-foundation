use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffWorkflowBinding {
    pub line_number: usize,
    pub id: String,
    pub stage: String,
    pub trigger: String,
    pub input_set: String,
    pub output_set: String,
    pub deterministic_order: String,
    pub truth_effect: String,
    pub status: String,
}

impl OperatorHandoffWorkflowBinding {
    pub fn canonical_identity(&self) -> String {
        format!("workflow:{}", self.id)
    }
    pub fn truth_neutral(&self) -> bool {
        self.truth_effect == "none_without_local_replay"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffCaptureChannelBinding {
    pub line_number: usize,
    pub id: String,
    pub medium: String,
    pub boundary: String,
    pub network_allowed: bool,
    pub operator_ack_required: bool,
    pub capture_hash_required: bool,
    pub status: String,
}

impl OperatorHandoffCaptureChannelBinding {
    pub fn canonical_identity(&self) -> String {
        format!("capture_channel:{}", self.id)
    }
    pub fn offline_only(&self) -> bool {
        !self.network_allowed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffTargetBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub target_class: String,
    pub capture_channel: String,
    pub automation_workflow: String,
    pub required_artifacts: Vec<String>,
    pub pre_import_gate: String,
    pub post_import_truth_effect: String,
    pub receipt_binding: String,
    pub status: String,
}

impl OperatorHandoffTargetBinding {
    pub fn canonical_identity(&self) -> String {
        format!("target_handoff:{}", self.id)
    }
    pub fn binds_target(&self, target: &str) -> bool {
        self.target_id == target
    }
    pub fn truth_neutral(&self) -> bool {
        self.post_import_truth_effect == "none_without_local_replay"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffTruthGateBinding {
    pub line_number: usize,
    pub id: String,
    pub gate_class: String,
    pub required_before: String,
    pub rejects: String,
    pub evidence_path: String,
    pub status: String,
}

impl OperatorHandoffTruthGateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("truth_gate:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}

impl OperatorHandoffReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorHandoffAutomationSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub previous_evidence_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub workflows: Vec<OperatorHandoffWorkflowBinding>,
    pub capture_channels: Vec<OperatorHandoffCaptureChannelBinding>,
    pub target_handoffs: Vec<OperatorHandoffTargetBinding>,
    pub truth_gates: Vec<OperatorHandoffTruthGateBinding>,
    pub receipts: Vec<OperatorHandoffReceiptBinding>,
}

impl OperatorHandoffAutomationSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn workflow_by_id(&self, id: &str) -> Option<&OperatorHandoffWorkflowBinding> {
        self.workflows.iter().find(|x| x.id == id)
    }
    pub fn capture_channel_by_id(&self, id: &str) -> Option<&OperatorHandoffCaptureChannelBinding> {
        self.capture_channels.iter().find(|x| x.id == id)
    }
    pub fn target_handoff_by_target(&self, target: &str) -> Option<&OperatorHandoffTargetBinding> {
        self.target_handoffs.iter().find(|x| x.target_id == target)
    }
    pub fn truth_gate_by_id(&self, id: &str) -> Option<&OperatorHandoffTruthGateBinding> {
        self.truth_gates.iter().find(|x| x.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&OperatorHandoffReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
}
