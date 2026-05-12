use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticClosureTaskBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub evidence: Vec<String>,
    pub status: String,
}

impl SemanticClosureTaskBinding {
    pub fn canonical_identity(&self) -> String {
        format!("task:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticClosureOutputGate {
    pub line_number: usize,
    pub id: String,
    pub output_kind: String,
    pub path: String,
    pub depends: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl SemanticClosureOutputGate {
    pub fn canonical_identity(&self) -> String {
        format!("output:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticClosureProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub tasks: Vec<String>,
    pub outputs: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub permits: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl SemanticClosureProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticClosureGateSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub bounded_closure: String,
    pub global_closure: String,
    pub next_frontier: String,
    pub rules: BTreeMap<String, String>,
    pub tasks: Vec<SemanticClosureTaskBinding>,
    pub outputs: Vec<SemanticClosureOutputGate>,
    pub proofs: Vec<SemanticClosureProof>,
}

impl SemanticClosureGateSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn task_by_id(&self, id: &str) -> Option<&SemanticClosureTaskBinding> {
        self.tasks.iter().find(|item| item.id == id)
    }

    pub fn output_by_id(&self, id: &str) -> Option<&SemanticClosureOutputGate> {
        self.outputs.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticClosureProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
