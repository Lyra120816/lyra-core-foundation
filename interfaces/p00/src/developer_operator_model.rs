use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceCommand {
    pub line_number: usize,
    pub id: String,
    pub binary: String,
    pub surface: String,
    pub input: String,
    pub output: String,
    pub receipts: Vec<String>,
    pub roles: Vec<String>,
    pub status: String,
}

impl InterfaceCommand {
    pub fn canonical_identity(&self) -> String {
        format!("command:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceWorkflow {
    pub line_number: usize,
    pub id: String,
    pub order: String,
    pub commands: Vec<String>,
    pub roles: Vec<String>,
    pub artifacts: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl InterfaceWorkflow {
    pub fn canonical_identity(&self) -> String {
        format!("workflow:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceExample {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub commands: Vec<String>,
    pub expected_receipts: Vec<String>,
    pub status: String,
}

impl InterfaceExample {
    pub fn canonical_identity(&self) -> String {
        format!("example:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceProof {
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

impl InterfaceProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeveloperOperatorInterfaceSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub commands: Vec<InterfaceCommand>,
    pub workflows: Vec<InterfaceWorkflow>,
    pub examples: Vec<InterfaceExample>,
    pub proofs: Vec<InterfaceProof>,
}

impl DeveloperOperatorInterfaceSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn command_by_id(&self, id: &str) -> Option<&InterfaceCommand> {
        self.commands.iter().find(|item| item.id == id)
    }

    pub fn workflow_by_id(&self, id: &str) -> Option<&InterfaceWorkflow> {
        self.workflows.iter().find(|item| item.id == id)
    }

    pub fn example_by_id(&self, id: &str) -> Option<&InterfaceExample> {
        self.examples.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&InterfaceProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
