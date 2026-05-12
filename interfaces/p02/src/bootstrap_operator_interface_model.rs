use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorCommandBinding {
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
impl BootstrapOperatorCommandBinding {
    pub fn canonical_identity(&self) -> String {
        format!("command:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorWorkflowBinding {
    pub line_number: usize,
    pub id: String,
    pub order: String,
    pub commands: Vec<String>,
    pub targets: Vec<String>,
    pub examples: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}
impl BootstrapOperatorWorkflowBinding {
    pub fn canonical_identity(&self) -> String {
        format!("workflow:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorExampleBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub commands: Vec<String>,
    pub expected_receipts: Vec<String>,
    pub expected_verdict: String,
    pub status: String,
}
impl BootstrapOperatorExampleBinding {
    pub fn canonical_identity(&self) -> String {
        format!("example:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorAcceptanceGateBinding {
    pub line_number: usize,
    pub id: String,
    pub workflow: String,
    pub required_receipts: Vec<String>,
    pub required_examples: Vec<String>,
    pub decision: String,
    pub forbids: Vec<String>,
    pub status: String,
}
impl BootstrapOperatorAcceptanceGateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gate:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorProofBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub commands: Vec<String>,
    pub workflows: Vec<String>,
    pub examples: Vec<String>,
    pub gates: Vec<String>,
    pub receipts: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}
impl BootstrapOperatorProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub artifact_kind: String,
    pub commands: Vec<String>,
    pub status: String,
}
impl BootstrapOperatorArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOperatorInterfaceSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub commands: Vec<BootstrapOperatorCommandBinding>,
    pub workflows: Vec<BootstrapOperatorWorkflowBinding>,
    pub examples: Vec<BootstrapOperatorExampleBinding>,
    pub gates: Vec<BootstrapOperatorAcceptanceGateBinding>,
    pub proofs: Vec<BootstrapOperatorProofBinding>,
    pub artifacts: Vec<BootstrapOperatorArtifactBinding>,
}

impl BootstrapOperatorInterfaceSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn command_by_id(&self, id: &str) -> Option<&BootstrapOperatorCommandBinding> {
        self.commands.iter().find(|item| item.id == id)
    }
    pub fn workflow_by_id(&self, id: &str) -> Option<&BootstrapOperatorWorkflowBinding> {
        self.workflows.iter().find(|item| item.id == id)
    }
    pub fn example_by_id(&self, id: &str) -> Option<&BootstrapOperatorExampleBinding> {
        self.examples.iter().find(|item| item.id == id)
    }
    pub fn gate_by_id(&self, id: &str) -> Option<&BootstrapOperatorAcceptanceGateBinding> {
        self.gates.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapOperatorProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&BootstrapOperatorArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
}
