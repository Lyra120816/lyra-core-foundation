use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputAudienceBinding {
    pub line_number: usize,
    pub id: String,
    pub outputs: Vec<String>,
    pub artifacts: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}
impl BootstrapOutputAudienceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_output_audience:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub audience: String,
    pub artifact_kind: String,
    pub owner_root: String,
    pub path: String,
    pub status: String,
}
impl BootstrapOutputArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_output_artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl BootstrapOutputReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_output_receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputContractBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub path: String,
    pub status: String,
}
impl BootstrapOutputContractBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_output_contract:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputGapBinding {
    pub line_number: usize,
    pub id: String,
    pub blocker: String,
    pub next_frontier: String,
    pub owner_root: String,
    pub status: String,
}
impl BootstrapOutputGapBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_output_gap:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapOutputTableSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub closure_scope: String,
    pub global_closure: String,
    pub next_frontier: String,
    pub rules: BTreeMap<String, String>,
    pub audiences: Vec<BootstrapOutputAudienceBinding>,
    pub artifacts: Vec<BootstrapOutputArtifactBinding>,
    pub receipts: Vec<BootstrapOutputReceiptBinding>,
    pub contracts: Vec<BootstrapOutputContractBinding>,
    pub gaps: Vec<BootstrapOutputGapBinding>,
}
impl BootstrapOutputTableSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn audience_by_id(&self, id: &str) -> Option<&BootstrapOutputAudienceBinding> {
        self.audiences.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&BootstrapOutputArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapOutputReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn contract_by_id(&self, id: &str) -> Option<&BootstrapOutputContractBinding> {
        self.contracts.iter().find(|item| item.id == id)
    }
    pub fn gap_by_id(&self, id: &str) -> Option<&BootstrapOutputGapBinding> {
        self.gaps.iter().find(|item| item.id == id)
    }
}
