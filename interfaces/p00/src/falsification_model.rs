use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeCorpusCase {
    pub line_number: usize,
    pub id: String,
    pub fixture: String,
    pub target_validator: String,
    pub expected_code: String,
    pub category: String,
    pub must_reject: bool,
    pub receipts: Vec<String>,
    pub status: String,
}

impl NegativeCorpusCase {
    pub fn canonical_identity(&self) -> String {
        format!("case:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalsificationHarness {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub module: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub order: String,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
}

impl FalsificationHarness {
    pub fn canonical_identity(&self) -> String {
        format!("harness:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectionAssertion {
    pub line_number: usize,
    pub id: String,
    pub case: String,
    pub harness: String,
    pub expected_code: String,
    pub expected_verdict: String,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
}

impl RejectionAssertion {
    pub fn canonical_identity(&self) -> String {
        format!("assertion:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalsificationProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub cases: Vec<String>,
    pub harnesses: Vec<String>,
    pub assertions: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl FalsificationProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalsificationCorpusSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub cases: Vec<NegativeCorpusCase>,
    pub harnesses: Vec<FalsificationHarness>,
    pub assertions: Vec<RejectionAssertion>,
    pub proofs: Vec<FalsificationProof>,
}

impl FalsificationCorpusSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn case_by_id(&self, id: &str) -> Option<&NegativeCorpusCase> {
        self.cases.iter().find(|item| item.id == id)
    }
    pub fn harness_by_id(&self, id: &str) -> Option<&FalsificationHarness> {
        self.harnesses.iter().find(|item| item.id == id)
    }
    pub fn assertion_by_id(&self, id: &str) -> Option<&RejectionAssertion> {
        self.assertions.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&FalsificationProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
