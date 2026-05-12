use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationCaseBinding {
    pub line_number: usize,
    pub id: String,
    pub target_domain: String,
    pub target_validator: String,
    pub mutation: String,
    pub expected_error: String,
    pub fixture: String,
    pub status: String,
}
impl BootstrapFalsificationCaseBinding {
    pub fn canonical_identity(&self) -> String {
        format!("case:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationHarnessBinding {
    pub line_number: usize,
    pub id: String,
    pub runner: String,
    pub cases: Vec<String>,
    pub assertion_mode: String,
    pub receipt_policy: String,
    pub status: String,
}
impl BootstrapFalsificationHarnessBinding {
    pub fn canonical_identity(&self) -> String {
        format!("harness:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRejectionAssertionBinding {
    pub line_number: usize,
    pub id: String,
    pub case_id: String,
    pub expected_error: String,
    pub proof_surface: String,
    pub forbids: Vec<String>,
    pub status: String,
}
impl BootstrapRejectionAssertionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("assertion:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub artifact_kind: String,
    pub status: String,
}
impl BootstrapFalsificationArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationProofBinding {
    pub line_number: usize,
    pub id: String,
    pub cases: Vec<String>,
    pub harnesses: Vec<String>,
    pub assertions: Vec<String>,
    pub artifacts: Vec<String>,
    pub receipt: String,
    pub status: String,
}
impl BootstrapFalsificationProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFalsificationSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub cases: Vec<BootstrapFalsificationCaseBinding>,
    pub harnesses: Vec<BootstrapFalsificationHarnessBinding>,
    pub assertions: Vec<BootstrapRejectionAssertionBinding>,
    pub artifacts: Vec<BootstrapFalsificationArtifactBinding>,
    pub proofs: Vec<BootstrapFalsificationProofBinding>,
}

impl BootstrapFalsificationSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn case_by_id(&self, id: &str) -> Option<&BootstrapFalsificationCaseBinding> {
        self.cases.iter().find(|item| item.id == id)
    }
    pub fn harness_by_id(&self, id: &str) -> Option<&BootstrapFalsificationHarnessBinding> {
        self.harnesses.iter().find(|item| item.id == id)
    }
    pub fn assertion_by_id(&self, id: &str) -> Option<&BootstrapRejectionAssertionBinding> {
        self.assertions.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&BootstrapFalsificationArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapFalsificationProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
