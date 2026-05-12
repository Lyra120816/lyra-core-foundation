use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEcosystemDoc {
    pub line_number: usize,
    pub id: String,
    pub audience: String,
    pub path: String,
    pub covers: Vec<String>,
    pub examples: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl SemanticEcosystemDoc {
    pub fn canonical_identity(&self) -> String {
        format!("doc:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEcosystemExample {
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

impl SemanticEcosystemExample {
    pub fn canonical_identity(&self) -> String {
        format!("example:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEcosystemProof {
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

impl SemanticEcosystemProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEcosystemSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub docs: Vec<SemanticEcosystemDoc>,
    pub examples: Vec<SemanticEcosystemExample>,
    pub proofs: Vec<SemanticEcosystemProof>,
}

impl SemanticEcosystemSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn doc_by_id(&self, id: &str) -> Option<&SemanticEcosystemDoc> {
        self.docs.iter().find(|item| item.id == id)
    }
    pub fn example_by_id(&self, id: &str) -> Option<&SemanticEcosystemExample> {
        self.examples.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticEcosystemProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
