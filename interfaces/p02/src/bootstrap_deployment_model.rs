use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDeploymentTarget {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub environment: String,
    pub artifacts: Vec<String>,
    pub commands: Vec<String>,
    pub receipts: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl BootstrapDeploymentTarget {
    pub fn canonical_identity(&self) -> String {
        format!("target:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapComplianceHook {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub target: String,
    pub requires: Vec<String>,
    pub evidence: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl BootstrapComplianceHook {
    pub fn canonical_identity(&self) -> String {
        format!("hook:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReleaseEvidence {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub path: String,
    pub targets: Vec<String>,
    pub hooks: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
}

impl BootstrapReleaseEvidence {
    pub fn canonical_identity(&self) -> String {
        format!("evidence:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDeploymentProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub targets: Vec<String>,
    pub hooks: Vec<String>,
    pub evidence: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl BootstrapDeploymentProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDeploymentSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub targets: Vec<BootstrapDeploymentTarget>,
    pub hooks: Vec<BootstrapComplianceHook>,
    pub evidence: Vec<BootstrapReleaseEvidence>,
    pub proofs: Vec<BootstrapDeploymentProof>,
}

impl BootstrapDeploymentSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn target_by_id(&self, id: &str) -> Option<&BootstrapDeploymentTarget> {
        self.targets.iter().find(|item| item.id == id)
    }
    pub fn hook_by_id(&self, id: &str) -> Option<&BootstrapComplianceHook> {
        self.hooks.iter().find(|item| item.id == id)
    }
    pub fn evidence_by_id(&self, id: &str) -> Option<&BootstrapReleaseEvidence> {
        self.evidence.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapDeploymentProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
