use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSerializationFamilyBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub serializer: String,
    pub hash_domain: String,
    pub registry: String,
    pub status: String,
}
impl SemanticSerializationFamilyBinding {
    pub fn canonical_identity(&self) -> String {
        format!("serializer:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectHashBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub object_ref: String,
    pub payload_hash: String,
    pub record_hash: String,
    pub comparison_key: String,
    pub status: String,
}
impl SemanticObjectHashBinding {
    pub fn canonical_identity(&self) -> String {
        format!("object_hash:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRoundTripBinding {
    pub line_number: usize,
    pub id: String,
    pub object_ref: String,
    pub text_identity: String,
    pub hash_identity: String,
    pub law: String,
    pub status: String,
}
impl SemanticRoundTripBinding {
    pub fn canonical_identity(&self) -> String {
        format!("round_trip:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSerializationReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl SemanticSerializationReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSerializationHashingSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub serializers: Vec<SemanticSerializationFamilyBinding>,
    pub object_hashes: Vec<SemanticObjectHashBinding>,
    pub round_trips: Vec<SemanticRoundTripBinding>,
    pub receipts: Vec<SemanticSerializationReceiptBinding>,
}
impl SemanticSerializationHashingSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn serializer_by_id(&self, id: &str) -> Option<&SemanticSerializationFamilyBinding> {
        self.serializers.iter().find(|item| item.id == id)
    }
    pub fn object_hash_by_ref(&self, object_ref: &str) -> Option<&SemanticObjectHashBinding> {
        self.object_hashes
            .iter()
            .find(|item| item.object_ref == object_ref)
    }
    pub fn round_trip_by_id(&self, id: &str) -> Option<&SemanticRoundTripBinding> {
        self.round_trips.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticSerializationReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
