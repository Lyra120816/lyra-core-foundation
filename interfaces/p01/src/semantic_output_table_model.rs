use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticOutputAudienceBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub outputs: Vec<String>,
    pub artifacts: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}
impl SemanticOutputAudienceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("audience:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticOutputArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub audience: String,
    pub artifact_kind: String,
    pub path: String,
    pub status: String,
}
impl SemanticOutputArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticOutputReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl SemanticOutputReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticOutputContractBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub path: String,
    pub status: String,
}
impl SemanticOutputContractBinding {
    pub fn canonical_identity(&self) -> String {
        format!("contract:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticOutputGapBinding {
    pub line_number: usize,
    pub id: String,
    pub blocker: String,
    pub next_frontier: String,
    pub owner_root: String,
    pub status: String,
}
impl SemanticOutputGapBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gap:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticOutputTableSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub audiences: Vec<SemanticOutputAudienceBinding>,
    pub artifacts: Vec<SemanticOutputArtifactBinding>,
    pub receipts: Vec<SemanticOutputReceiptBinding>,
    pub contracts: Vec<SemanticOutputContractBinding>,
    pub gaps: Vec<SemanticOutputGapBinding>,
}
impl SemanticOutputTableSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn audience_by_id(&self, id: &str) -> Option<&SemanticOutputAudienceBinding> {
        self.audiences.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&SemanticOutputArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticOutputReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn contract_by_id(&self, id: &str) -> Option<&SemanticOutputContractBinding> {
        self.contracts.iter().find(|item| item.id == id)
    }
    pub fn gap_by_id(&self, id: &str) -> Option<&SemanticOutputGapBinding> {
        self.gaps.iter().find(|item| item.id == id)
    }
}
