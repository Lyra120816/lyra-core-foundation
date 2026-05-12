use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub input_hash: String,
    pub canonical_hash: String,
    pub verdict_hash: String,
    pub receipt_hash: String,
    pub status: String,
}
impl BootstrapReplayReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayWitnessBinding {
    pub line_number: usize,
    pub id: String,
    pub order: String,
    pub receipts: Vec<String>,
    pub preimage: String,
    pub witness_hash: String,
    pub commands: Vec<String>,
    pub status: String,
}
impl BootstrapReplayWitnessBinding {
    pub fn canonical_identity(&self) -> String {
        format!("witness:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayChainLinkBinding {
    pub line_number: usize,
    pub id: String,
    pub from: String,
    pub to: String,
    pub relation: String,
    pub receipts: Vec<String>,
    pub status: String,
}
impl BootstrapReplayChainLinkBinding {
    pub fn canonical_identity(&self) -> String {
        format!("link:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayProofBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub receipts: Vec<String>,
    pub witnesses: Vec<String>,
    pub links: Vec<String>,
    pub commands: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}
impl BootstrapReplayProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplayArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub artifact_kind: String,
    pub status: String,
}
impl BootstrapReplayArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReplaySurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub receipts: Vec<BootstrapReplayReceiptBinding>,
    pub witnesses: Vec<BootstrapReplayWitnessBinding>,
    pub links: Vec<BootstrapReplayChainLinkBinding>,
    pub proofs: Vec<BootstrapReplayProofBinding>,
    pub artifacts: Vec<BootstrapReplayArtifactBinding>,
}

impl BootstrapReplaySurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapReplayReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn witness_by_id(&self, id: &str) -> Option<&BootstrapReplayWitnessBinding> {
        self.witnesses.iter().find(|item| item.id == id)
    }
    pub fn link_by_id(&self, id: &str) -> Option<&BootstrapReplayChainLinkBinding> {
        self.links.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapReplayProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&BootstrapReplayArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
}
