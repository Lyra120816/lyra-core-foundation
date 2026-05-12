use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerRootBinding {
    pub line_number: usize,
    pub id: String,
    pub domain: String,
    pub owns: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
    pub evidence: Vec<String>,
}

impl OwnerRootBinding {
    pub fn canonical_identity(&self) -> String {
        format!("root:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootResponsibility {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub path: String,
    pub kind: String,
    pub behavior: String,
    pub proof: String,
    pub status: String,
}

impl RootResponsibility {
    pub fn canonical_identity(&self) -> String {
        format!("responsibility:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerRootClaim {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub status: String,
    pub roots: Vec<String>,
    pub responsibilities: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
}

impl OwnerRootClaim {
    pub fn canonical_identity(&self) -> String {
        format!("claim:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerRootLawSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub roots: Vec<OwnerRootBinding>,
    pub responsibilities: Vec<RootResponsibility>,
    pub claims: Vec<OwnerRootClaim>,
}

impl OwnerRootLawSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn root_by_id(&self, id: &str) -> Option<&OwnerRootBinding> {
        self.roots.iter().find(|root| root.id == id)
    }

    pub fn responsibility_by_id(&self, id: &str) -> Option<&RootResponsibility> {
        self.responsibilities
            .iter()
            .find(|responsibility| responsibility.id == id)
    }

    pub fn claim_by_id(&self, id: &str) -> Option<&OwnerRootClaim> {
        self.claims.iter().find(|claim| claim.id == id)
    }
}
