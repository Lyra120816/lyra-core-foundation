use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDataModel {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub source_task: String,
    pub schema: String,
    pub canonical_order: Vec<String>,
    pub status: String,
}

impl CanonicalDataModel {
    pub fn canonical_identity(&self) -> String {
        format!("model:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub model: String,
    pub fields: Vec<String>,
    pub required: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl SchemaBinding {
    pub fn canonical_identity(&self) -> String {
        format!("schema:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldBinding {
    pub line_number: usize,
    pub id: String,
    pub model: String,
    pub kind: String,
    pub required: String,
    pub order: String,
    pub canonical: String,
    pub status: String,
}

impl FieldBinding {
    pub fn canonical_identity(&self) -> String {
        format!("field:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelBinding {
    pub line_number: usize,
    pub id: String,
    pub from: String,
    pub to: String,
    pub through: String,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
}

impl ModelBinding {
    pub fn canonical_identity(&self) -> String {
        format!("binding:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalModelProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub models: Vec<String>,
    pub schemas: Vec<String>,
    pub fields: Vec<String>,
    pub bindings: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
    pub forbids: Vec<String>,
}

impl CanonicalModelProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalModelSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub models: Vec<CanonicalDataModel>,
    pub schemas: Vec<SchemaBinding>,
    pub fields: Vec<FieldBinding>,
    pub bindings: Vec<ModelBinding>,
    pub proofs: Vec<CanonicalModelProof>,
}

impl CanonicalModelSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn model_by_id(&self, id: &str) -> Option<&CanonicalDataModel> {
        self.models.iter().find(|item| item.id == id)
    }

    pub fn schema_by_id(&self, id: &str) -> Option<&SchemaBinding> {
        self.schemas.iter().find(|item| item.id == id)
    }

    pub fn field_by_id(&self, id: &str) -> Option<&FieldBinding> {
        self.fields.iter().find(|item| item.id == id)
    }

    pub fn binding_by_id(&self, id: &str) -> Option<&ModelBinding> {
        self.bindings.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&CanonicalModelProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
