use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageUnit {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub owner: String,
    pub artifacts: Vec<String>,
    pub commands: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl PackageUnit {
    pub fn canonical_identity(&self) -> String {
        format!("package:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseBundle {
    pub line_number: usize,
    pub id: String,
    pub order: String,
    pub packages: Vec<String>,
    pub artifacts: Vec<String>,
    pub receipts: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl ReleaseBundle {
    pub fn canonical_identity(&self) -> String {
        format!("bundle:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributionCheck {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub target: String,
    pub requires: Vec<String>,
    pub forbids: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl DistributionCheck {
    pub fn canonical_identity(&self) -> String {
        format!("check:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagingProof {
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

impl PackagingProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagingSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub packages: Vec<PackageUnit>,
    pub bundles: Vec<ReleaseBundle>,
    pub checks: Vec<DistributionCheck>,
    pub proofs: Vec<PackagingProof>,
}

impl PackagingSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn package_by_id(&self, id: &str) -> Option<&PackageUnit> {
        self.packages.iter().find(|item| item.id == id)
    }

    pub fn bundle_by_id(&self, id: &str) -> Option<&ReleaseBundle> {
        self.bundles.iter().find(|item| item.id == id)
    }

    pub fn check_by_id(&self, id: &str) -> Option<&DistributionCheck> {
        self.checks.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&PackagingProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
