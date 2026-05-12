use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomBinding {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub owner_root: String,
    pub canonical_name: String,
    pub identity_law: String,
    pub equality_law: String,
    pub normalization_law: String,
    pub serialization_law: String,
    pub status: String,
}

impl SemanticAtomBinding {
    pub fn canonical_identity(&self) -> String {
        format!("atom:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomFamilyBinding {
    pub line_number: usize,
    pub id: String,
    pub members: Vec<String>,
    pub phase: String,
    pub work_package: String,
    pub receipt: String,
    pub status: String,
}

impl SemanticAtomFamilyBinding {
    pub fn canonical_identity(&self) -> String {
        format!("family:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}

impl SemanticAtomReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub atoms: Vec<SemanticAtomBinding>,
    pub families: Vec<SemanticAtomFamilyBinding>,
    pub receipts: Vec<SemanticAtomReceiptBinding>,
}

impl SemanticAtomSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn atom_by_id(&self, id: &str) -> Option<&SemanticAtomBinding> {
        self.atoms.iter().find(|item| item.id == id)
    }
    pub fn family_by_id(&self, id: &str) -> Option<&SemanticAtomFamilyBinding> {
        self.families.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticAtomReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
