use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReplayReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub input_hash: String,
    pub canonical_hash: String,
    pub verdict_hash: String,
    pub receipt_hash: String,
    pub status: String,
}
impl SemanticReplayReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReplayWitnessBinding {
    pub line_number: usize,
    pub id: String,
    pub order: String,
    pub receipts: Vec<String>,
    pub preimage: String,
    pub witness_hash: String,
    pub commands: Vec<String>,
    pub status: String,
}
impl SemanticReplayWitnessBinding {
    pub fn canonical_identity(&self) -> String {
        format!("witness:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReplayChainLinkBinding {
    pub line_number: usize,
    pub id: String,
    pub from: String,
    pub to: String,
    pub relation: String,
    pub receipts: Vec<String>,
    pub status: String,
}
impl SemanticReplayChainLinkBinding {
    pub fn canonical_identity(&self) -> String {
        format!("link:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReplayProofBinding {
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
impl SemanticReplayProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReplayArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub artifact_kind: String,
    pub status: String,
}
impl SemanticReplayArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReplaySurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub receipts: Vec<SemanticReplayReceiptBinding>,
    pub witnesses: Vec<SemanticReplayWitnessBinding>,
    pub links: Vec<SemanticReplayChainLinkBinding>,
    pub proofs: Vec<SemanticReplayProofBinding>,
    pub artifacts: Vec<SemanticReplayArtifactBinding>,
}

impl SemanticReplaySurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticReplayReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn witness_by_id(&self, id: &str) -> Option<&SemanticReplayWitnessBinding> {
        self.witnesses.iter().find(|item| item.id == id)
    }
    pub fn link_by_id(&self, id: &str) -> Option<&SemanticReplayChainLinkBinding> {
        self.links.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticReplayProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&SemanticReplayArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
}
