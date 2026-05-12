use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRedTeamScenario {
    pub line_number: usize,
    pub id: String,
    pub scenario_kind: String,
    pub path: String,
    pub targets: Vec<String>,
    pub commands: Vec<String>,
    pub receipts: Vec<String>,
    pub rejects: Vec<String>,
    pub status: String,
}

impl SemanticRedTeamScenario {
    pub fn canonical_identity(&self) -> String {
        format!("scenario:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRollbackPath {
    pub line_number: usize,
    pub id: String,
    pub rollback_kind: String,
    pub path: String,
    pub authority: String,
    pub scenarios: Vec<String>,
    pub proofs: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
}

impl SemanticRollbackPath {
    pub fn canonical_identity(&self) -> String {
        format!("rollback:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRedTeamProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub scenarios: Vec<String>,
    pub rollbacks: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl SemanticRedTeamProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRedTeamSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub scenarios: Vec<SemanticRedTeamScenario>,
    pub rollbacks: Vec<SemanticRollbackPath>,
    pub proofs: Vec<SemanticRedTeamProof>,
}

impl SemanticRedTeamSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn scenario_by_id(&self, id: &str) -> Option<&SemanticRedTeamScenario> {
        self.scenarios.iter().find(|item| item.id == id)
    }

    pub fn rollback_by_id(&self, id: &str) -> Option<&SemanticRollbackPath> {
        self.rollbacks.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticRedTeamProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
