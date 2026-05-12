use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectBinding {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub atom: String,
    pub owner_root: String,
    pub parent: String,
    pub ir_form: String,
    pub serialization: String,
    pub comparison: String,
    pub status: String,
}

impl SemanticObjectBinding {
    pub fn canonical_identity(&self) -> String {
        format!("object:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectRelationBinding {
    pub line_number: usize,
    pub id: String,
    pub from_object: String,
    pub to_object: String,
    pub relation_kind: String,
    pub law: String,
    pub status: String,
}

impl SemanticObjectRelationBinding {
    pub fn canonical_identity(&self) -> String {
        format!("relation:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectInvariantBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub law: String,
    pub requires: String,
    pub status: String,
}

impl SemanticObjectInvariantBinding {
    pub fn canonical_identity(&self) -> String {
        format!("invariant:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectConformanceBinding {
    pub line_number: usize,
    pub id: String,
    pub object: String,
    pub ir_form: String,
    pub fixture: String,
    pub round_trip: String,
    pub status: String,
}

impl SemanticObjectConformanceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("conformance:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}

impl SemanticObjectReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub objects: Vec<SemanticObjectBinding>,
    pub relations: Vec<SemanticObjectRelationBinding>,
    pub invariants: Vec<SemanticObjectInvariantBinding>,
    pub conformances: Vec<SemanticObjectConformanceBinding>,
    pub receipts: Vec<SemanticObjectReceiptBinding>,
}

impl SemanticObjectSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn object_by_id(&self, id: &str) -> Option<&SemanticObjectBinding> {
        self.objects.iter().find(|item| item.id == id)
    }
    pub fn relation_by_id(&self, id: &str) -> Option<&SemanticObjectRelationBinding> {
        self.relations.iter().find(|item| item.id == id)
    }
    pub fn invariant_by_id(&self, id: &str) -> Option<&SemanticObjectInvariantBinding> {
        self.invariants.iter().find(|item| item.id == id)
    }
    pub fn conformance_by_id(&self, id: &str) -> Option<&SemanticObjectConformanceBinding> {
        self.conformances.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticObjectReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
