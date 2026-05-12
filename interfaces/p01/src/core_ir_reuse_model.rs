use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseConsumerBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub target_phase: String,
    pub owner_root: String,
    pub core_ir_ref: String,
    pub adapter: String,
    pub fixture_path: String,
    pub status: String,
}
impl CoreIrReuseConsumerBinding {
    pub fn canonical_identity(&self) -> String {
        format!("consumer:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseEdgeBinding {
    pub line_number: usize,
    pub id: String,
    pub from_consumer: String,
    pub to_consumer: String,
    pub form: String,
    pub guard: String,
    pub rejection: String,
    pub receipt_ref: String,
    pub status: String,
}
impl CoreIrReuseEdgeBinding {
    pub fn canonical_identity(&self) -> String {
        format!("edge:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseGateBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub law: String,
    pub evidence: String,
    pub status: String,
}
impl CoreIrReuseGateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gate:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl CoreIrReuseReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReuseSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub consumers: Vec<CoreIrReuseConsumerBinding>,
    pub edges: Vec<CoreIrReuseEdgeBinding>,
    pub gates: Vec<CoreIrReuseGateBinding>,
    pub receipts: Vec<CoreIrReuseReceiptBinding>,
}
impl CoreIrReuseSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn consumer_by_id(&self, id: &str) -> Option<&CoreIrReuseConsumerBinding> {
        self.consumers.iter().find(|item| item.id == id)
    }
    pub fn edge_by_id(&self, id: &str) -> Option<&CoreIrReuseEdgeBinding> {
        self.edges.iter().find(|item| item.id == id)
    }
    pub fn gate_by_id(&self, id: &str) -> Option<&CoreIrReuseGateBinding> {
        self.gates.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&CoreIrReuseReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
