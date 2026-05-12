use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputAudienceBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub outputs: Vec<String>,
    pub artifacts: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}
impl OutputAudienceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("audience:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub audience: String,
    pub artifact_kind: String,
    pub path: String,
    pub status: String,
}
impl OutputArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl OutputReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputContractBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub path: String,
    pub status: String,
}
impl OutputContractBinding {
    pub fn canonical_identity(&self) -> String {
        format!("contract:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedGapBinding {
    pub line_number: usize,
    pub id: String,
    pub blocker: String,
    pub next_frontier: String,
    pub owner_root: String,
    pub status: String,
}
impl UnresolvedGapBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gap:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputTableSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub audiences: Vec<OutputAudienceBinding>,
    pub artifacts: Vec<OutputArtifactBinding>,
    pub receipts: Vec<OutputReceiptBinding>,
    pub contracts: Vec<OutputContractBinding>,
    pub gaps: Vec<UnresolvedGapBinding>,
}
impl OutputTableSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn audience_by_id(&self, id: &str) -> Option<&OutputAudienceBinding> {
        self.audiences.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&OutputArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&OutputReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn contract_by_id(&self, id: &str) -> Option<&OutputContractBinding> {
        self.contracts.iter().find(|item| item.id == id)
    }
    pub fn gap_by_id(&self, id: &str) -> Option<&UnresolvedGapBinding> {
        self.gaps.iter().find(|item| item.id == id)
    }
}
