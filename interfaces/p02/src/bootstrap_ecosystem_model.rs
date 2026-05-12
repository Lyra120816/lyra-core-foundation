use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEcosystemDoc {
    pub line_number: usize,
    pub id: String,
    pub audience: String,
    pub path: String,
    pub covers: Vec<String>,
    pub examples: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl BootstrapEcosystemDoc {
    pub fn canonical_identity(&self) -> String {
        format!("doc:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEcosystemExample {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub path: String,
    pub commands: Vec<String>,
    pub proofs: Vec<String>,
    pub receipts: Vec<String>,
    pub rejects: Vec<String>,
    pub status: String,
}

impl BootstrapEcosystemExample {
    pub fn canonical_identity(&self) -> String {
        format!("example:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEcosystemProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub docs: Vec<String>,
    pub examples: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl BootstrapEcosystemProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEcosystemSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub docs: Vec<BootstrapEcosystemDoc>,
    pub examples: Vec<BootstrapEcosystemExample>,
    pub proofs: Vec<BootstrapEcosystemProof>,
}

impl BootstrapEcosystemSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn doc_by_id(&self, id: &str) -> Option<&BootstrapEcosystemDoc> {
        self.docs.iter().find(|item| item.id == id)
    }
    pub fn example_by_id(&self, id: &str) -> Option<&BootstrapEcosystemExample> {
        self.examples.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapEcosystemProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
