use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceCommandBinding {
    pub line_number: usize,
    pub id: String,
    pub binary: String,
    pub surface: String,
    pub input: String,
    pub output: String,
    pub receipts: Vec<String>,
    pub roles: Vec<String>,
    pub targets: Vec<String>,
    pub status: String,
}
impl SemanticInterfaceCommandBinding {
    pub fn canonical_identity(&self) -> String {
        format!("command:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceWorkflowBinding {
    pub line_number: usize,
    pub id: String,
    pub order: String,
    pub commands: Vec<String>,
    pub targets: Vec<String>,
    pub examples: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}
impl SemanticInterfaceWorkflowBinding {
    pub fn canonical_identity(&self) -> String {
        format!("workflow:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceExampleBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub commands: Vec<String>,
    pub expected_receipts: Vec<String>,
    pub expected_verdict: String,
    pub status: String,
}
impl SemanticInterfaceExampleBinding {
    pub fn canonical_identity(&self) -> String {
        format!("example:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceProofBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub commands: Vec<String>,
    pub workflows: Vec<String>,
    pub examples: Vec<String>,
    pub receipts: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}
impl SemanticInterfaceProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub artifact_kind: String,
    pub commands: Vec<String>,
    pub status: String,
}
impl SemanticInterfaceArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterfaceSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub commands: Vec<SemanticInterfaceCommandBinding>,
    pub workflows: Vec<SemanticInterfaceWorkflowBinding>,
    pub examples: Vec<SemanticInterfaceExampleBinding>,
    pub proofs: Vec<SemanticInterfaceProofBinding>,
    pub artifacts: Vec<SemanticInterfaceArtifactBinding>,
}

impl SemanticInterfaceSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn command_by_id(&self, id: &str) -> Option<&SemanticInterfaceCommandBinding> {
        self.commands.iter().find(|item| item.id == id)
    }
    pub fn workflow_by_id(&self, id: &str) -> Option<&SemanticInterfaceWorkflowBinding> {
        self.workflows.iter().find(|item| item.id == id)
    }
    pub fn example_by_id(&self, id: &str) -> Option<&SemanticInterfaceExampleBinding> {
        self.examples.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticInterfaceProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&SemanticInterfaceArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
}
