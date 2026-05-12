use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineUnitBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub input_model: String,
    pub output_model: String,
    pub stage_order: String,
    pub engine_law: String,
    pub status: String,
}
impl SemanticCoreEngineUnitBinding {
    pub fn canonical_identity(&self) -> String {
        format!("unit:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineTransitionBinding {
    pub line_number: usize,
    pub id: String,
    pub from_unit: String,
    pub to_unit: String,
    pub transition_law: String,
    pub carry: String,
    pub status: String,
}
impl SemanticCoreEngineTransitionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("transition:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub artifact_kind: String,
    pub status: String,
}
impl SemanticCoreEngineArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineProofBinding {
    pub line_number: usize,
    pub id: String,
    pub units: Vec<String>,
    pub transitions: Vec<String>,
    pub artifacts: Vec<String>,
    pub fixture: String,
    pub golden: String,
    pub receipt: String,
    pub status: String,
}
impl SemanticCoreEngineProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCoreEngineSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub units: Vec<SemanticCoreEngineUnitBinding>,
    pub transitions: Vec<SemanticCoreEngineTransitionBinding>,
    pub artifacts: Vec<SemanticCoreEngineArtifactBinding>,
    pub proofs: Vec<SemanticCoreEngineProofBinding>,
}

impl SemanticCoreEngineSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn unit_by_id(&self, id: &str) -> Option<&SemanticCoreEngineUnitBinding> {
        self.units.iter().find(|item| item.id == id)
    }
    pub fn transition_by_id(&self, id: &str) -> Option<&SemanticCoreEngineTransitionBinding> {
        self.transitions.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&SemanticCoreEngineArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticCoreEngineProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
