use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicsFrame {
    pub line_number: usize,
    pub id: String,
    pub frame_kind: String,
    pub path: String,
    pub covers: Vec<String>,
    pub outputs: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl EconomicsFrame {
    pub fn canonical_identity(&self) -> String {
        format!("frame:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicInterestOutput {
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

impl PublicInterestOutput {
    pub fn canonical_identity(&self) -> String {
        format!("output:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicsProof {
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

impl EconomicsProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicsSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub frames: Vec<EconomicsFrame>,
    pub outputs: Vec<PublicInterestOutput>,
    pub proofs: Vec<EconomicsProof>,
}

impl EconomicsSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn frame_by_id(&self, id: &str) -> Option<&EconomicsFrame> {
        self.frames.iter().find(|item| item.id == id)
    }

    pub fn output_by_id(&self, id: &str) -> Option<&PublicInterestOutput> {
        self.outputs.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&EconomicsProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
