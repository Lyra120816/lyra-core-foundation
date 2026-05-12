use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalModelBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub domain_id: String,
    pub canonical_kind: String,
    pub schema_path: String,
    pub hash_policy: String,
    pub status: String,
}
impl BootstrapCanonicalModelBinding {
    pub fn canonical_identity(&self) -> String {
        format!("model:{}", self.id)
    }
    pub fn local_schema(&self) -> bool {
        self.schema_path.starts_with("interfaces/p02/contracts/")
    }
    pub fn deterministic_hash_policy(&self) -> bool {
        self.hash_policy == "stable_hash_label" || self.hash_policy == "canonical_fingerprint"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalSchemaBinding {
    pub line_number: usize,
    pub id: String,
    pub model_id: String,
    pub contract_path: String,
    pub encoding: String,
    pub version: String,
    pub status: String,
}
impl BootstrapCanonicalSchemaBinding {
    pub fn canonical_identity(&self) -> String {
        format!("schema:{}", self.id)
    }
    pub fn local_contract(&self) -> bool {
        self.contract_path.starts_with("interfaces/p02/contracts/")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalFieldBinding {
    pub line_number: usize,
    pub id: String,
    pub model_id: String,
    pub name: String,
    pub field_type: String,
    pub required: String,
    pub canonical_order: String,
    pub status: String,
}
impl BootstrapCanonicalFieldBinding {
    pub fn canonical_identity(&self) -> String {
        format!("field:{}", self.id)
    }
    pub fn required(&self) -> bool {
        self.required == "required"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalRelationBinding {
    pub line_number: usize,
    pub id: String,
    pub from_model: String,
    pub to_model: String,
    pub relation_kind: String,
    pub cardinality: String,
    pub status: String,
}
impl BootstrapCanonicalRelationBinding {
    pub fn canonical_identity(&self) -> String {
        format!("relation:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalInvariantBinding {
    pub line_number: usize,
    pub id: String,
    pub model_id: String,
    pub assertion: String,
    pub rejects: Vec<String>,
    pub receipt: String,
    pub status: String,
}
impl BootstrapCanonicalInvariantBinding {
    pub fn canonical_identity(&self) -> String {
        format!("invariant:{}", self.id)
    }
    pub fn receipt_bound(&self) -> bool {
        self.receipt.starts_with("receipts/p02/")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalProofBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub models: Vec<String>,
    pub schemas: Vec<String>,
    pub fields: Vec<String>,
    pub relations: Vec<String>,
    pub invariants: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}
impl BootstrapCanonicalProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
    pub fn receipt_bound(&self) -> bool {
        self.receipts
            .iter()
            .all(|path| path.starts_with("receipts/p02/"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub binds: String,
    pub status: String,
}
impl BootstrapCanonicalReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCanonicalModelSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub previous_semantics_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub models: Vec<BootstrapCanonicalModelBinding>,
    pub schemas: Vec<BootstrapCanonicalSchemaBinding>,
    pub fields: Vec<BootstrapCanonicalFieldBinding>,
    pub relations: Vec<BootstrapCanonicalRelationBinding>,
    pub invariants: Vec<BootstrapCanonicalInvariantBinding>,
    pub proofs: Vec<BootstrapCanonicalProofBinding>,
    pub receipts: Vec<BootstrapCanonicalReceiptBinding>,
}

impl BootstrapCanonicalModelSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn model_by_id(&self, id: &str) -> Option<&BootstrapCanonicalModelBinding> {
        self.models.iter().find(|x| x.id == id)
    }
    pub fn schema_by_id(&self, id: &str) -> Option<&BootstrapCanonicalSchemaBinding> {
        self.schemas.iter().find(|x| x.id == id)
    }
    pub fn field_by_id(&self, id: &str) -> Option<&BootstrapCanonicalFieldBinding> {
        self.fields.iter().find(|x| x.id == id)
    }
    pub fn relation_by_id(&self, id: &str) -> Option<&BootstrapCanonicalRelationBinding> {
        self.relations.iter().find(|x| x.id == id)
    }
    pub fn invariant_by_id(&self, id: &str) -> Option<&BootstrapCanonicalInvariantBinding> {
        self.invariants.iter().find(|x| x.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapCanonicalProofBinding> {
        self.proofs.iter().find(|x| x.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapCanonicalReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
    pub fn fields_for_model(&self, id: &str) -> Vec<&BootstrapCanonicalFieldBinding> {
        self.fields.iter().filter(|x| x.model_id == id).collect()
    }
    pub fn invariants_for_model(&self, id: &str) -> Vec<&BootstrapCanonicalInvariantBinding> {
        self.invariants
            .iter()
            .filter(|x| x.model_id == id)
            .collect()
    }
}
