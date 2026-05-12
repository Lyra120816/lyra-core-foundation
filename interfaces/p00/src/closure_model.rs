use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureTaskBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub evidence: Vec<String>,
    pub status: String,
}

impl ClosureTaskBinding {
    pub fn canonical_identity(&self) -> String {
        format!("task:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureOutputGate {
    pub line_number: usize,
    pub id: String,
    pub output_kind: String,
    pub path: String,
    pub depends: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl ClosureOutputGate {
    pub fn canonical_identity(&self) -> String {
        format!("output:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureProof {
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

impl ClosureProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureGateSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub bounded_closure: String,
    pub global_closure: String,
    pub next_frontier: String,
    pub rules: BTreeMap<String, String>,
    pub tasks: Vec<ClosureTaskBinding>,
    pub outputs: Vec<ClosureOutputGate>,
    pub proofs: Vec<ClosureProof>,
}

impl ClosureGateSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn task_by_id(&self, id: &str) -> Option<&ClosureTaskBinding> {
        self.tasks.iter().find(|item| item.id == id)
    }

    pub fn output_by_id(&self, id: &str) -> Option<&ClosureOutputGate> {
        self.outputs.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&ClosureProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
