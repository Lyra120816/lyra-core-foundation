use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDeploymentTarget {
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

impl SemanticDeploymentTarget {
    pub fn canonical_identity(&self) -> String {
        format!("target:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticComplianceHook {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub target: String,
    pub requires: Vec<String>,
    pub evidence: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl SemanticComplianceHook {
    pub fn canonical_identity(&self) -> String {
        format!("hook:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReleaseEvidence {
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

impl SemanticReleaseEvidence {
    pub fn canonical_identity(&self) -> String {
        format!("evidence:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDeploymentProof {
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

impl SemanticDeploymentProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDeploymentSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub targets: Vec<SemanticDeploymentTarget>,
    pub hooks: Vec<SemanticComplianceHook>,
    pub evidence: Vec<SemanticReleaseEvidence>,
    pub proofs: Vec<SemanticDeploymentProof>,
}

impl SemanticDeploymentSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn target_by_id(&self, id: &str) -> Option<&SemanticDeploymentTarget> {
        self.targets.iter().find(|item| item.id == id)
    }
    pub fn hook_by_id(&self, id: &str) -> Option<&SemanticComplianceHook> {
        self.hooks.iter().find(|item| item.id == id)
    }
    pub fn evidence_by_id(&self, id: &str) -> Option<&SemanticReleaseEvidence> {
        self.evidence.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticDeploymentProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
