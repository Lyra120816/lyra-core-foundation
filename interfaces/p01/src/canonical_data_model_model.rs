use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P01CanonicalDataModelBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub owner_root: String,
    pub source_task: String,
    pub schema_ref: String,
    pub canonical_order: Vec<String>,
    pub status: String,
}
impl P01CanonicalDataModelBinding {
    pub fn canonical_identity(&self) -> String {
        format!("model:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P01CanonicalSchemaBinding {
    pub line_number: usize,
    pub id: String,
    pub model_ref: String,
    pub fields: Vec<String>,
    pub required: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}
impl P01CanonicalSchemaBinding {
    pub fn canonical_identity(&self) -> String {
        format!("schema:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P01CanonicalFieldBinding {
    pub line_number: usize,
    pub id: String,
    pub model_ref: String,
    pub kind: String,
    pub order: String,
    pub normalization: String,
    pub status: String,
}
impl P01CanonicalFieldBinding {
    pub fn canonical_identity(&self) -> String {
        format!("field:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P01CanonicalModelBridgeBinding {
    pub line_number: usize,
    pub id: String,
    pub from_model: String,
    pub to_model: String,
    pub carrier: String,
    pub receipt_ref: String,
    pub status: String,
}
impl P01CanonicalModelBridgeBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bridge:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P01CanonicalDataProofBinding {
    pub line_number: usize,
    pub id: String,
    pub models: Vec<String>,
    pub schemas: Vec<String>,
    pub fields: Vec<String>,
    pub bridges: Vec<String>,
    pub fixture: String,
    pub golden: String,
    pub receipt: String,
    pub status: String,
}
impl P01CanonicalDataProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P01CanonicalDataModelSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub models: Vec<P01CanonicalDataModelBinding>,
    pub schemas: Vec<P01CanonicalSchemaBinding>,
    pub fields: Vec<P01CanonicalFieldBinding>,
    pub bridges: Vec<P01CanonicalModelBridgeBinding>,
    pub proofs: Vec<P01CanonicalDataProofBinding>,
}

impl P01CanonicalDataModelSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn model_by_id(&self, id: &str) -> Option<&P01CanonicalDataModelBinding> {
        self.models.iter().find(|item| item.id == id)
    }
    pub fn schema_by_id(&self, id: &str) -> Option<&P01CanonicalSchemaBinding> {
        self.schemas.iter().find(|item| item.id == id)
    }
    pub fn field_by_id(&self, id: &str) -> Option<&P01CanonicalFieldBinding> {
        self.fields.iter().find(|item| item.id == id)
    }
    pub fn bridge_by_id(&self, id: &str) -> Option<&P01CanonicalModelBridgeBinding> {
        self.bridges.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&P01CanonicalDataProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
