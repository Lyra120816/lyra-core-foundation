use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticDomainBinding {
    pub line_number: usize,
    pub id: String,
    pub layer: String,
    pub owner_root: String,
    pub meaning: String,
    pub core_ref: String,
    pub status: String,
}
impl FormalSemanticDomainBinding {
    pub fn canonical_identity(&self) -> String {
        format!("domain:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticLawBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub rule: String,
    pub guard: String,
    pub status: String,
}
impl FormalSemanticLawBinding {
    pub fn canonical_identity(&self) -> String {
        format!("law:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticInvariantBinding {
    pub line_number: usize,
    pub id: String,
    pub applies_to: String,
    pub assertion: String,
    pub evidence_ref: String,
    pub status: String,
}
impl FormalSemanticInvariantBinding {
    pub fn canonical_identity(&self) -> String {
        format!("invariant:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticProofBinding {
    pub line_number: usize,
    pub id: String,
    pub fixture: String,
    pub golden: String,
    pub receipt: String,
    pub law_ref: String,
    pub status: String,
}
impl FormalSemanticProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticConstitutionSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub domains: Vec<FormalSemanticDomainBinding>,
    pub laws: Vec<FormalSemanticLawBinding>,
    pub invariants: Vec<FormalSemanticInvariantBinding>,
    pub proofs: Vec<FormalSemanticProofBinding>,
}

impl FormalSemanticConstitutionSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn domain_by_id(&self, id: &str) -> Option<&FormalSemanticDomainBinding> {
        self.domains.iter().find(|item| item.id == id)
    }
    pub fn law_by_id(&self, id: &str) -> Option<&FormalSemanticLawBinding> {
        self.laws.iter().find(|item| item.id == id)
    }
    pub fn invariant_by_id(&self, id: &str) -> Option<&FormalSemanticInvariantBinding> {
        self.invariants.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&FormalSemanticProofBinding> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
