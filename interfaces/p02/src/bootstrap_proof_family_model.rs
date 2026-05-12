use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapProofFamilyBinding {
    pub line_number: usize,
    pub id: String,
    pub family_kind: String,
    pub scope: String,
    pub receipts: Vec<String>,
    pub covers: Vec<String>,
    pub proofs: Vec<String>,
    pub status: String,
}
impl BootstrapProofFamilyBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_proof_family:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapProofReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub path: String,
    pub covers: Vec<String>,
    pub verdict: String,
    pub status: String,
}
impl BootstrapProofReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_proof_receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapProofPathBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub path_kind: String,
    pub entry_receipts: Vec<String>,
    pub challenge_receipts: Vec<String>,
    pub rollback_receipts: Vec<String>,
    pub status: String,
}
impl BootstrapProofPathBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_proof_path:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapProofFamilyTableSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub closure_scope: String,
    pub global_closure: String,
    pub next_frontier: String,
    pub rules: BTreeMap<String, String>,
    pub families: Vec<BootstrapProofFamilyBinding>,
    pub receipts: Vec<BootstrapProofReceiptBinding>,
    pub paths: Vec<BootstrapProofPathBinding>,
}
impl BootstrapProofFamilyTableSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn family_by_id(&self, id: &str) -> Option<&BootstrapProofFamilyBinding> {
        self.families.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapProofReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn path_by_id(&self, id: &str) -> Option<&BootstrapProofPathBinding> {
        self.paths.iter().find(|item| item.id == id)
    }
}
