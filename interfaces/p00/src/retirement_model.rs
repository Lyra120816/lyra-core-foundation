use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetirementSurfaceBinding {
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
impl RetirementSurfaceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("surface:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetirementGateBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub trigger: String,
    pub action: String,
    pub evidence: Vec<String>,
    pub status: String,
}
impl RetirementGateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gate:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupersessionBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub replaced_by: String,
    pub archive: String,
    pub receipt: String,
    pub status: String,
}
impl SupersessionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("supersession:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetirementReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl RetirementReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetirementSupersessionSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub surfaces: Vec<RetirementSurfaceBinding>,
    pub gates: Vec<RetirementGateBinding>,
    pub supersessions: Vec<SupersessionBinding>,
    pub receipts: Vec<RetirementReceiptBinding>,
}
impl RetirementSupersessionSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn surface_by_id(&self, id: &str) -> Option<&RetirementSurfaceBinding> {
        self.surfaces.iter().find(|item| item.id == id)
    }
    pub fn gate_by_id(&self, id: &str) -> Option<&RetirementGateBinding> {
        self.gates.iter().find(|item| item.id == id)
    }
    pub fn supersession_by_id(&self, id: &str) -> Option<&SupersessionBinding> {
        self.supersessions.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&RetirementReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
