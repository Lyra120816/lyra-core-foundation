use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonSourceBinding {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub path: String,
    pub authority: String,
    pub hash: String,
    pub role: String,
}

impl CanonSourceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("source:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapBinding {
    pub line_number: usize,
    pub id: String,
    pub phase: String,
    pub task: String,
    pub source: String,
    pub owner_roots: Vec<String>,
    pub status: String,
    pub receipts: Vec<String>,
}

impl RoadmapBinding {
    pub fn canonical_identity(&self) -> String {
        format!("roadmap:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonValidationClaim {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub sources: Vec<String>,
    pub roadmap_bindings: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
    pub forbids: Vec<String>,
}

impl CanonValidationClaim {
    pub fn canonical_identity(&self) -> String {
        format!("validation:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonComplianceSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub sources: Vec<CanonSourceBinding>,
    pub roadmap_bindings: Vec<RoadmapBinding>,
    pub validations: Vec<CanonValidationClaim>,
}

impl CanonComplianceSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn source_by_id(&self, id: &str) -> Option<&CanonSourceBinding> {
        self.sources.iter().find(|item| item.id == id)
    }

    pub fn roadmap_by_id(&self, id: &str) -> Option<&RoadmapBinding> {
        self.roadmap_bindings.iter().find(|item| item.id == id)
    }

    pub fn validation_by_id(&self, id: &str) -> Option<&CanonValidationClaim> {
        self.validations.iter().find(|item| item.id == id)
    }
}
