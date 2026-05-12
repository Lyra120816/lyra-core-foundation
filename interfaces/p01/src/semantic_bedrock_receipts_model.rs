use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub task: String,
    pub surface: String,
    pub path: String,
    pub expected_hash: String,
    pub status: String,
}
impl SemanticBedrockReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockAnchorBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub module: String,
    pub contract: String,
    pub law: String,
    pub receipt_ref: String,
    pub core_ref: String,
    pub status: String,
}
impl SemanticBedrockAnchorBinding {
    pub fn canonical_identity(&self) -> String {
        format!("anchor:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockParityFixtureBinding {
    pub line_number: usize,
    pub id: String,
    pub positive: String,
    pub negative: String,
    pub receipt_ref: String,
    pub golden: String,
    pub status: String,
}
impl SemanticBedrockParityFixtureBinding {
    pub fn canonical_identity(&self) -> String {
        format!("fixture:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockGateBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub law: String,
    pub evidence: String,
    pub status: String,
}
impl SemanticBedrockGateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gate:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBedrockReceiptsSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub receipts: Vec<SemanticBedrockReceiptBinding>,
    pub anchors: Vec<SemanticBedrockAnchorBinding>,
    pub fixtures: Vec<SemanticBedrockParityFixtureBinding>,
    pub gates: Vec<SemanticBedrockGateBinding>,
}
impl SemanticBedrockReceiptsSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticBedrockReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn anchor_by_id(&self, id: &str) -> Option<&SemanticBedrockAnchorBinding> {
        self.anchors.iter().find(|item| item.id == id)
    }
    pub fn fixture_by_id(&self, id: &str) -> Option<&SemanticBedrockParityFixtureBinding> {
        self.fixtures.iter().find(|item| item.id == id)
    }
    pub fn gate_by_id(&self, id: &str) -> Option<&SemanticBedrockGateBinding> {
        self.gates.iter().find(|item| item.id == id)
    }
}
