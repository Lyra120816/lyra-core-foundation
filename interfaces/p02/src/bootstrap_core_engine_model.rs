use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCoreEngineUnitBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub input_model: String,
    pub output_model: String,
    pub stage_order: String,
    pub engine_law: String,
    pub status: String,
}
impl BootstrapCoreEngineUnitBinding {
    pub fn canonical_identity(&self) -> String {
        format!("unit:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCoreEngineTransitionBinding {
    pub line_number: usize,
    pub id: String,
    pub from_unit: String,
    pub to_unit: String,
    pub transition_law: String,
    pub carry: String,
    pub status: String,
}
impl BootstrapCoreEngineTransitionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("transition:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCoreEngineArtifactBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub artifact_kind: String,
    pub status: String,
}
impl BootstrapCoreEngineArtifactBinding {
    pub fn canonical_identity(&self) -> String {
        format!("artifact:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCoreEngineProofBinding {
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
impl BootstrapCoreEngineProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapCoreEngineSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub units: Vec<BootstrapCoreEngineUnitBinding>,
    pub transitions: Vec<BootstrapCoreEngineTransitionBinding>,
    pub artifacts: Vec<BootstrapCoreEngineArtifactBinding>,
    pub proofs: Vec<BootstrapCoreEngineProofBinding>,
}

impl BootstrapCoreEngineSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn unit_by_id(&self, id: &str) -> Option<&BootstrapCoreEngineUnitBinding> {
        self.units.iter().find(|item| item.id == id)
    }
    pub fn transition_by_id(&self, id: &str) -> Option<&BootstrapCoreEngineTransitionBinding> {
        self.transitions.iter().find(|item| item.id == id)
    }
    pub fn artifact_by_id(&self, id: &str) -> Option<&BootstrapCoreEngineArtifactBinding> {
        self.artifacts.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapCoreEngineProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
