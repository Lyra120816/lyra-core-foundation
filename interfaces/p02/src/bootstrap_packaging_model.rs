use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPackageUnitBinding {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub owner_root: String,
    pub artifacts: Vec<String>,
    pub commands: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}
impl BootstrapPackageUnitBinding {
    pub fn canonical_identity(&self) -> String {
        format!("package:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReleaseBundleBinding {
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
impl BootstrapReleaseBundleBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bundle:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDistributionCheckBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub target: String,
    pub requires: Vec<String>,
    pub forbids: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}
impl BootstrapDistributionCheckBinding {
    pub fn canonical_identity(&self) -> String {
        format!("check:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPackagingProofBinding {
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
impl BootstrapPackagingProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPackagingSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub packages: Vec<BootstrapPackageUnitBinding>,
    pub bundles: Vec<BootstrapReleaseBundleBinding>,
    pub checks: Vec<BootstrapDistributionCheckBinding>,
    pub proofs: Vec<BootstrapPackagingProofBinding>,
}

impl BootstrapPackagingSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn package_by_id(&self, id: &str) -> Option<&BootstrapPackageUnitBinding> {
        self.packages.iter().find(|item| item.id == id)
    }
    pub fn bundle_by_id(&self, id: &str) -> Option<&BootstrapReleaseBundleBinding> {
        self.bundles.iter().find(|item| item.id == id)
    }
    pub fn check_by_id(&self, id: &str) -> Option<&BootstrapDistributionCheckBinding> {
        self.checks.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapPackagingProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
