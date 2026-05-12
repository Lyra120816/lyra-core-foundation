use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticPackageUnit {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub owner_root: String,
    pub artifacts: Vec<String>,
    pub commands: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}
impl SemanticPackageUnit {
    pub fn canonical_identity(&self) -> String {
        format!("package:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReleaseBundle {
    pub line_number: usize,
    pub id: String,
    pub order: String,
    pub packages: Vec<String>,
    pub artifacts: Vec<String>,
    pub receipts: Vec<String>,
    pub checks: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}
impl SemanticReleaseBundle {
    pub fn canonical_identity(&self) -> String {
        format!("bundle:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDistributionCheck {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub target: String,
    pub requires: Vec<String>,
    pub forbids: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}
impl SemanticDistributionCheck {
    pub fn canonical_identity(&self) -> String {
        format!("check:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticPackagingProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub packages: Vec<String>,
    pub bundles: Vec<String>,
    pub checks: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}
impl SemanticPackagingProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticPackagingSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub packages: Vec<SemanticPackageUnit>,
    pub bundles: Vec<SemanticReleaseBundle>,
    pub checks: Vec<SemanticDistributionCheck>,
    pub proofs: Vec<SemanticPackagingProof>,
}

impl SemanticPackagingSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn package_by_id(&self, id: &str) -> Option<&SemanticPackageUnit> {
        self.packages.iter().find(|item| item.id == id)
    }
    pub fn bundle_by_id(&self, id: &str) -> Option<&SemanticReleaseBundle> {
        self.bundles.iter().find(|item| item.id == id)
    }
    pub fn check_by_id(&self, id: &str) -> Option<&SemanticDistributionCheck> {
        self.checks.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticPackagingProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
