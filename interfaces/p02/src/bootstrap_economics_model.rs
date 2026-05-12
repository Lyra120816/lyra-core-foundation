use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEconomicsFrame {
    pub line_number: usize,
    pub id: String,
    pub frame_kind: String,
    pub path: String,
    pub covers: Vec<String>,
    pub outputs: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl BootstrapEconomicsFrame {
    pub fn canonical_identity(&self) -> String {
        format!("frame:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPublicInterestOutput {
    pub line_number: usize,
    pub id: String,
    pub output_kind: String,
    pub path: String,
    pub constituencies: Vec<String>,
    pub commands: Vec<String>,
    pub proofs: Vec<String>,
    pub receipts: Vec<String>,
    pub rejects: Vec<String>,
    pub status: String,
}

impl BootstrapPublicInterestOutput {
    pub fn canonical_identity(&self) -> String {
        format!("output:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEconomicsProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub frames: Vec<String>,
    pub outputs: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl BootstrapEconomicsProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEconomicsSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub frames: Vec<BootstrapEconomicsFrame>,
    pub outputs: Vec<BootstrapPublicInterestOutput>,
    pub proofs: Vec<BootstrapEconomicsProof>,
}

impl BootstrapEconomicsSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn frame_by_id(&self, id: &str) -> Option<&BootstrapEconomicsFrame> {
        self.frames.iter().find(|item| item.id == id)
    }
    pub fn output_by_id(&self, id: &str) -> Option<&BootstrapPublicInterestOutput> {
        self.outputs.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapEconomicsProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
