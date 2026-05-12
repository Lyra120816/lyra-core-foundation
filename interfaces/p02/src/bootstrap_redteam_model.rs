use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRedTeamScenario {
    pub line_number: usize,
    pub id: String,
    pub attack_kind: String,
    pub path: String,
    pub targets: Vec<String>,
    pub rollback_paths: Vec<String>,
    pub commands: Vec<String>,
    pub receipts: Vec<String>,
    pub rejects: Vec<String>,
    pub status: String,
}

impl BootstrapRedTeamScenario {
    pub fn canonical_identity(&self) -> String {
        format!("scenario:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRollbackPath {
    pub line_number: usize,
    pub id: String,
    pub rollback_kind: String,
    pub path: String,
    pub triggers: Vec<String>,
    pub restores: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub challenge_rights: Vec<String>,
    pub status: String,
}

impl BootstrapRollbackPath {
    pub fn canonical_identity(&self) -> String {
        format!("rollback:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRedTeamProof {
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

impl BootstrapRedTeamProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRedTeamSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub scenarios: Vec<BootstrapRedTeamScenario>,
    pub rollbacks: Vec<BootstrapRollbackPath>,
    pub proofs: Vec<BootstrapRedTeamProof>,
}

impl BootstrapRedTeamSurface {
    pub fn scenario_by_id(&self, id: &str) -> Option<&BootstrapRedTeamScenario> {
        self.scenarios.iter().find(|item| item.id == id)
    }
    pub fn rollback_by_id(&self, id: &str) -> Option<&BootstrapRollbackPath> {
        self.rollbacks.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapRedTeamProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
