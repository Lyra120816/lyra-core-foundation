use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryArtifact {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub root: String,
    pub path: String,
    pub producer: String,
    pub evidence: Vec<String>,
}

impl DeliveryArtifact {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub artifacts: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl ProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryClaim {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub status: String,
    pub artifacts: Vec<String>,
    pub proofs: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
}

impl DeliveryClaim {
    pub fn canonical_identity(&self) -> String {
        format!("claim:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliverySurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub artifacts: Vec<DeliveryArtifact>,
    pub proofs: Vec<ProofBinding>,
    pub claims: Vec<DeliveryClaim>,
}

impl DeliverySurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn artifact_by_id(&self, id: &str) -> Option<&DeliveryArtifact> {
        self.artifacts.iter().find(|artifact| artifact.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&ProofBinding> {
        self.proofs.iter().find(|proof| proof.id == id)
    }

    pub fn claim_by_id(&self, id: &str) -> Option<&DeliveryClaim> {
        self.claims.iter().find(|claim| claim.id == id)
    }
}
