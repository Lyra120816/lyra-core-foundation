use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptanceGolden {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub path: String,
    pub source_task: String,
    pub receipt: String,
    pub hash: String,
    pub status: String,
}

impl AcceptanceGolden {
    pub fn canonical_identity(&self) -> String {
        format!("golden:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChallengeFixture {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub path: String,
    pub task: String,
    pub expects: String,
    pub receipt: String,
    pub status: String,
}

impl ChallengeFixture {
    pub fn canonical_identity(&self) -> String {
        format!("fixture:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptanceProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub tasks: Vec<String>,
    pub goldens: Vec<String>,
    pub fixtures: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
    pub forbids: Vec<String>,
}

impl AcceptanceProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptanceProofSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub goldens: Vec<AcceptanceGolden>,
    pub fixtures: Vec<ChallengeFixture>,
    pub proofs: Vec<AcceptanceProof>,
}

impl AcceptanceProofSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn golden_by_id(&self, id: &str) -> Option<&AcceptanceGolden> {
        self.goldens.iter().find(|item| item.id == id)
    }

    pub fn fixture_by_id(&self, id: &str) -> Option<&ChallengeFixture> {
        self.fixtures.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&AcceptanceProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
