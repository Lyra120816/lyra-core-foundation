use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceLibraryBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub registry_ref: String,
    pub atom_ids: String,
    pub library_path: String,
    pub export_contract: String,
    pub status: String,
}
impl SemanticAtomReferenceLibraryBinding {
    pub fn canonical_identity(&self) -> String {
        format!("library:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceExampleBinding {
    pub line_number: usize,
    pub id: String,
    pub library_ref: String,
    pub atom_id: String,
    pub example_path: String,
    pub expected_inspection: String,
    pub status: String,
}
impl SemanticAtomReferenceExampleBinding {
    pub fn canonical_identity(&self) -> String {
        format!("example:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomInspectionToolBinding {
    pub line_number: usize,
    pub id: String,
    pub binary: String,
    pub input_contract: String,
    pub output_contract: String,
    pub fixture_path: String,
    pub receipt_ref: String,
    pub status: String,
}
impl SemanticAtomInspectionToolBinding {
    pub fn canonical_identity(&self) -> String {
        format!("tool:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceGateBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub law: String,
    pub evidence: String,
    pub status: String,
}
impl SemanticAtomReferenceGateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gate:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl SemanticAtomReferenceReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub libraries: Vec<SemanticAtomReferenceLibraryBinding>,
    pub examples: Vec<SemanticAtomReferenceExampleBinding>,
    pub tools: Vec<SemanticAtomInspectionToolBinding>,
    pub gates: Vec<SemanticAtomReferenceGateBinding>,
    pub receipts: Vec<SemanticAtomReferenceReceiptBinding>,
}
impl SemanticAtomReferenceSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn library_by_id(&self, id: &str) -> Option<&SemanticAtomReferenceLibraryBinding> {
        self.libraries.iter().find(|item| item.id == id)
    }
    pub fn example_by_id(&self, id: &str) -> Option<&SemanticAtomReferenceExampleBinding> {
        self.examples.iter().find(|item| item.id == id)
    }
    pub fn tool_by_id(&self, id: &str) -> Option<&SemanticAtomInspectionToolBinding> {
        self.tools.iter().find(|item| item.id == id)
    }
    pub fn gate_by_id(&self, id: &str) -> Option<&SemanticAtomReferenceGateBinding> {
        self.gates.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticAtomReferenceReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
