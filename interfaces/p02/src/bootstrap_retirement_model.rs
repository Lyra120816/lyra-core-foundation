use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRetirementSurfaceBinding {
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
impl BootstrapRetirementSurfaceBinding {
    pub fn canonical_identity(&self) -> String { format!("surface:{}", self.id) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRetirementGateBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub trigger: String,
    pub action: String,
    pub evidence: Vec<String>,
    pub status: String,
}
impl BootstrapRetirementGateBinding {
    pub fn canonical_identity(&self) -> String { format!("gate:{}", self.id) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapSupersessionBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub replaced_by: String,
    pub archive: String,
    pub receipt: String,
    pub status: String,
}
impl BootstrapSupersessionBinding {
    pub fn canonical_identity(&self) -> String { format!("supersession:{}", self.id) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRetirementReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl BootstrapRetirementReceiptBinding {
    pub fn canonical_identity(&self) -> String { format!("receipt:{}", self.id) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRetirementSupersessionSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub global_closure: String,
    pub next_frontier: String,
    pub rules: BTreeMap<String, String>,
    pub surfaces: Vec<BootstrapRetirementSurfaceBinding>,
    pub gates: Vec<BootstrapRetirementGateBinding>,
    pub supersessions: Vec<BootstrapSupersessionBinding>,
    pub receipts: Vec<BootstrapRetirementReceiptBinding>,
}
impl BootstrapRetirementSupersessionSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> { self.rules.get(name).map(String::as_str) }
    pub fn surface_by_id(&self, id: &str) -> Option<&BootstrapRetirementSurfaceBinding> { self.surfaces.iter().find(|item| item.id == id) }
    pub fn gate_by_id(&self, id: &str) -> Option<&BootstrapRetirementGateBinding> { self.gates.iter().find(|item| item.id == id) }
    pub fn supersession_by_id(&self, id: &str) -> Option<&BootstrapSupersessionBinding> { self.supersessions.iter().find(|item| item.id == id) }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapRetirementReceiptBinding> { self.receipts.iter().find(|item| item.id == id) }
}
