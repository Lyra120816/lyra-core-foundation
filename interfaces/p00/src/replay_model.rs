use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptReplayBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub input_hash: String,
    pub canonical_hash: String,
    pub verdict_hash: String,
    pub receipt_hash: String,
    pub status: String,
}

impl ReceiptReplayBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayWitness {
    pub line_number: usize,
    pub id: String,
    pub order: String,
    pub receipts: Vec<String>,
    pub preimage: String,
    pub witness_hash: String,
    pub commands: Vec<String>,
    pub status: String,
}

impl ReplayWitness {
    pub fn canonical_identity(&self) -> String {
        format!("witness:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptChainLink {
    pub line_number: usize,
    pub id: String,
    pub from: String,
    pub to: String,
    pub relation: String,
    pub receipts: Vec<String>,
    pub status: String,
}

impl ReceiptChainLink {
    pub fn canonical_identity(&self) -> String {
        format!("link:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayProof {
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

impl ReplayProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayWitnessSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub receipts: Vec<ReceiptReplayBinding>,
    pub witnesses: Vec<ReplayWitness>,
    pub links: Vec<ReceiptChainLink>,
    pub proofs: Vec<ReplayProof>,
}

impl ReplayWitnessSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn receipt_by_id(&self, id: &str) -> Option<&ReceiptReplayBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }

    pub fn witness_by_id(&self, id: &str) -> Option<&ReplayWitness> {
        self.witnesses.iter().find(|item| item.id == id)
    }

    pub fn link_by_id(&self, id: &str) -> Option<&ReceiptChainLink> {
        self.links.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&ReplayProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
