use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplementationUnit {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub responsibility: String,
    pub behavior: String,
    pub tests: String,
    pub fixtures: String,
    pub receipts: String,
    pub status: String,
}

impl ImplementationUnit {
    pub fn canonical_identity(&self) -> String {
        format!("unit:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureClaim {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub status: String,
    pub evidence: Vec<String>,
}

impl ClosureClaim {
    pub fn canonical_identity(&self) -> String {
        format!("claim:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnforcementSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub units: Vec<ImplementationUnit>,
    pub claims: Vec<ClosureClaim>,
}

impl EnforcementSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn unit_by_id(&self, id: &str) -> Option<&ImplementationUnit> {
        self.units.iter().find(|unit| unit.id == id)
    }

    pub fn claim_by_id(&self, id: &str) -> Option<&ClosureClaim> {
        self.claims.iter().find(|claim| claim.id == id)
    }
}
