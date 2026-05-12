use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentTarget {
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

impl DeploymentTarget {
    pub fn canonical_identity(&self) -> String {
        format!("target:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplianceHook {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub target: String,
    pub requires: Vec<String>,
    pub evidence: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl ComplianceHook {
    pub fn canonical_identity(&self) -> String {
        format!("hook:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseEvidence {
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

impl ReleaseEvidence {
    pub fn canonical_identity(&self) -> String {
        format!("evidence:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentProof {
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

impl DeploymentProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub targets: Vec<DeploymentTarget>,
    pub hooks: Vec<ComplianceHook>,
    pub evidence: Vec<ReleaseEvidence>,
    pub proofs: Vec<DeploymentProof>,
}

impl DeploymentSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn target_by_id(&self, id: &str) -> Option<&DeploymentTarget> {
        self.targets.iter().find(|item| item.id == id)
    }

    pub fn hook_by_id(&self, id: &str) -> Option<&ComplianceHook> {
        self.hooks.iter().find(|item| item.id == id)
    }

    pub fn evidence_by_id(&self, id: &str) -> Option<&ReleaseEvidence> {
        self.evidence.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&DeploymentProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
