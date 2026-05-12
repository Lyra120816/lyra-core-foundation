use std::collections::BTreeMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorObjectBinding {
    pub line_number: usize,
    pub id: String,
    pub severity: String,
    pub domain: String,
    pub subject: String,
    pub message: String,
    pub evidence_ref: String,
    pub digest: String,
    pub status: String,
}
impl ErrorObjectBinding {
    pub fn canonical_identity(&self) -> String {
        format!("error_object:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChallengeObjectBinding {
    pub line_number: usize,
    pub id: String,
    pub target: String,
    pub challenger: String,
    pub claim_ref: String,
    pub counter_evidence_ref: String,
    pub adjudication_law: String,
    pub digest: String,
    pub status: String,
}
impl ChallengeObjectBinding {
    pub fn canonical_identity(&self) -> String {
        format!("challenge_object:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceObjectBinding {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub source: String,
    pub payload_digest: String,
    pub witness: String,
    pub digest: String,
    pub status: String,
}
impl EvidenceObjectBinding {
    pub fn canonical_identity(&self) -> String {
        format!("evidence_object:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLinkBinding {
    pub line_number: usize,
    pub id: String,
    pub from: String,
    pub relation: String,
    pub to: String,
    pub law: String,
    pub digest: String,
    pub status: String,
}
impl ObjectLinkBinding {
    pub fn canonical_identity(&self) -> String {
        format!("object_link:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorChallengeEvidenceReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl ErrorChallengeEvidenceReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorChallengeEvidenceSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub error_objects: Vec<ErrorObjectBinding>,
    pub challenge_objects: Vec<ChallengeObjectBinding>,
    pub evidence_objects: Vec<EvidenceObjectBinding>,
    pub object_links: Vec<ObjectLinkBinding>,
    pub receipts: Vec<ErrorChallengeEvidenceReceiptBinding>,
}
impl ErrorChallengeEvidenceSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn error_object_by_id(&self, id: &str) -> Option<&ErrorObjectBinding> {
        self.error_objects.iter().find(|item| item.id == id)
    }
    pub fn challenge_object_by_id(&self, id: &str) -> Option<&ChallengeObjectBinding> {
        self.challenge_objects.iter().find(|item| item.id == id)
    }
    pub fn evidence_object_by_id(&self, id: &str) -> Option<&EvidenceObjectBinding> {
        self.evidence_objects.iter().find(|item| item.id == id)
    }
    pub fn object_link_by_id(&self, id: &str) -> Option<&ObjectLinkBinding> {
        self.object_links.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&ErrorChallengeEvidenceReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
