use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofFamilyBinding {
    pub line_number: usize,
    pub id: String,
    pub family_kind: String,
    pub scope: String,
    pub receipts: Vec<String>,
    pub covers: Vec<String>,
    pub proofs: Vec<String>,
    pub status: String,
}
impl ProofFamilyBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof_family:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub path: String,
    pub covers: Vec<String>,
    pub verdict: String,
    pub status: String,
}
impl ProofReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPathBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub path_kind: String,
    pub entry_receipts: Vec<String>,
    pub challenge_receipts: Vec<String>,
    pub rollback_receipts: Vec<String>,
    pub status: String,
}
impl ProofPathBinding {
    pub fn canonical_identity(&self) -> String {
        format!("path:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofFamilyTableSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub families: Vec<ProofFamilyBinding>,
    pub receipts: Vec<ProofReceiptBinding>,
    pub paths: Vec<ProofPathBinding>,
}
impl ProofFamilyTableSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn family_by_id(&self, id: &str) -> Option<&ProofFamilyBinding> {
        self.families.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&ProofReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn path_by_id(&self, id: &str) -> Option<&ProofPathBinding> {
        self.paths.iter().find(|item| item.id == id)
    }
}
