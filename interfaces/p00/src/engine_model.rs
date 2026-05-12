use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicEngineUnit {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub module: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub state: String,
    pub order: String,
    pub receipts: Vec<String>,
    pub status: String,
}

impl DeterministicEngineUnit {
    pub fn canonical_identity(&self) -> String {
        format!("engine:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineTransitionBinding {
    pub line_number: usize,
    pub id: String,
    pub from: String,
    pub to: String,
    pub trigger: String,
    pub guard: String,
    pub effect: String,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
}

impl EngineTransitionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("transition:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineExecutionProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub engines: Vec<String>,
    pub transitions: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl EngineExecutionProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicEngineSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub engines: Vec<DeterministicEngineUnit>,
    pub transitions: Vec<EngineTransitionBinding>,
    pub proofs: Vec<EngineExecutionProof>,
}

impl DeterministicEngineSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn engine_by_id(&self, id: &str) -> Option<&DeterministicEngineUnit> {
        self.engines.iter().find(|item| item.id == id)
    }

    pub fn transition_by_id(&self, id: &str) -> Option<&EngineTransitionBinding> {
        self.transitions.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&EngineExecutionProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
