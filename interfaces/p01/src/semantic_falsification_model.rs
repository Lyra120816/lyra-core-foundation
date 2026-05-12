use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationCaseBinding {
    pub line_number: usize,
    pub id: String,
    pub target_domain: String,
    pub target_validator: String,
    pub mutation: String,
    pub expected_error: String,
    pub fixture: String,
    pub status: String,
}
impl SemanticFalsificationCaseBinding {
    pub fn canonical_identity(&self) -> String {
        format!("case:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationHarnessBinding {
    pub line_number: usize,
    pub id: String,
    pub runner: String,
    pub cases: Vec<String>,
    pub assertion_mode: String,
    pub receipt_policy: String,
    pub status: String,
}
impl SemanticFalsificationHarnessBinding {
    pub fn canonical_identity(&self) -> String {
        format!("harness:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRejectionAssertionBinding {
    pub line_number: usize,
    pub id: String,
    pub case_id: String,
    pub expected_error: String,
    pub proof_surface: String,
    pub forbids: Vec<String>,
    pub status: String,
}
impl SemanticRejectionAssertionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("assertion:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub artifact_kind: String,
    pub status: String,
}
impl SemanticFalsificationArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationProofBinding {
    pub line_number: usize,
    pub id: String,
    pub cases: Vec<String>,
    pub harnesses: Vec<String>,
    pub assertions: Vec<String>,
    pub artifacts: Vec<String>,
    pub receipt: String,
    pub status: String,
}
impl SemanticFalsificationProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticFalsificationSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub cases: Vec<SemanticFalsificationCaseBinding>,
    pub harnesses: Vec<SemanticFalsificationHarnessBinding>,
    pub assertions: Vec<SemanticRejectionAssertionBinding>,
    pub artifacts: Vec<SemanticFalsificationArtifactBinding>,
    pub proofs: Vec<SemanticFalsificationProofBinding>,
}

impl SemanticFalsificationSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn case_by_id(&self, id: &str) -> Option<&SemanticFalsificationCaseBinding> {
        self.cases.iter().find(|item| item.id == id)
    }
    pub fn harness_by_id(&self, id: &str) -> Option<&SemanticFalsificationHarnessBinding> {
        self.harnesses.iter().find(|item| item.id == id)
    }
    pub fn assertion_by_id(&self, id: &str) -> Option<&SemanticRejectionAssertionBinding> {
        self.assertions.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&SemanticFalsificationArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticFalsificationProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
