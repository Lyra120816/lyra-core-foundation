use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIdentityRuleBinding {
    pub line_number: usize,
    pub id: String,
    pub domain: String,
    pub scope: String,
    pub material: String,
    pub canonicalizer: String,
    pub digest: String,
    pub collision: String,
    pub status: String,
}
impl SemanticIdentityRuleBinding {
    pub fn canonical_identity(&self) -> String {
        format!("identity:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDigestCaseBinding {
    pub line_number: usize,
    pub id: String,
    pub domain: String,
    pub owner: String,
    pub payload: String,
    pub normalization: String,
    pub expected_digest: String,
    pub status: String,
}
impl SemanticDigestCaseBinding {
    pub fn canonical_identity(&self) -> String {
        format!("digest_case:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCollisionCaseBinding {
    pub line_number: usize,
    pub id: String,
    pub domain: String,
    pub left: String,
    pub right: String,
    pub law: String,
    pub status: String,
}
impl SemanticCollisionCaseBinding {
    pub fn canonical_identity(&self) -> String {
        format!("collision:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIdentityReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl SemanticIdentityReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIdentitySurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub identities: Vec<SemanticIdentityRuleBinding>,
    pub digest_cases: Vec<SemanticDigestCaseBinding>,
    pub collision_cases: Vec<SemanticCollisionCaseBinding>,
    pub receipts: Vec<SemanticIdentityReceiptBinding>,
}

impl SemanticIdentitySurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn identity_by_id(&self, id: &str) -> Option<&SemanticIdentityRuleBinding> {
        self.identities.iter().find(|item| item.id == id)
    }
    pub fn digest_case_by_id(&self, id: &str) -> Option<&SemanticDigestCaseBinding> {
        self.digest_cases.iter().find(|item| item.id == id)
    }
    pub fn collision_case_by_id(&self, id: &str) -> Option<&SemanticCollisionCaseBinding> {
        self.collision_cases.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticIdentityReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
