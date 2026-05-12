use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRetirementSurfaceBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub surface_kind: String,
    pub path: String,
    pub replacement: String,
    pub retirement_gate: String,
    pub supersession: String,
    pub receipt: String,
    pub status: String,
}
impl SemanticRetirementSurfaceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("surface:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRetirementGateBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub trigger: String,
    pub action: String,
    pub evidence: Vec<String>,
    pub status: String,
}
impl SemanticRetirementGateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gate:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSupersessionBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub replaced_by: String,
    pub archive: String,
    pub receipt: String,
    pub status: String,
}
impl SemanticSupersessionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("supersession:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRetirementReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl SemanticRetirementReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRetirementSupersessionSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub surfaces: Vec<SemanticRetirementSurfaceBinding>,
    pub gates: Vec<SemanticRetirementGateBinding>,
    pub supersessions: Vec<SemanticSupersessionBinding>,
    pub receipts: Vec<SemanticRetirementReceiptBinding>,
}
impl SemanticRetirementSupersessionSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn surface_by_id(&self, id: &str) -> Option<&SemanticRetirementSurfaceBinding> {
        self.surfaces.iter().find(|item| item.id == id)
    }
    pub fn gate_by_id(&self, id: &str) -> Option<&SemanticRetirementGateBinding> {
        self.gates.iter().find(|item| item.id == id)
    }
    pub fn supersession_by_id(&self, id: &str) -> Option<&SemanticSupersessionBinding> {
        self.supersessions.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticRetirementReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
