use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProofFamilyBinding {
    pub line_number: usize,
    pub id: String,
    pub family_kind: String,
    pub scope: String,
    pub receipts: Vec<String>,
    pub covers: Vec<String>,
    pub proofs: Vec<String>,
    pub status: String,
}
impl SemanticProofFamilyBinding {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_proof_family:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProofReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub path: String,
    pub covers: Vec<String>,
    pub verdict: String,
    pub status: String,
}
impl SemanticProofReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProofPathBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub path_kind: String,
    pub entry_receipts: Vec<String>,
    pub challenge_receipts: Vec<String>,
    pub rollback_receipts: Vec<String>,
    pub status: String,
}
impl SemanticProofPathBinding {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_path:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProofFamilyTableSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub families: Vec<SemanticProofFamilyBinding>,
    pub receipts: Vec<SemanticProofReceiptBinding>,
    pub paths: Vec<SemanticProofPathBinding>,
}
impl SemanticProofFamilyTableSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn family_by_id(&self, id: &str) -> Option<&SemanticProofFamilyBinding> {
        self.families.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticProofReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn path_by_id(&self, id: &str) -> Option<&SemanticProofPathBinding> {
        self.paths.iter().find(|item| item.id == id)
    }
}
