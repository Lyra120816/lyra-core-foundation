use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDomain {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub source_task: String,
    pub contract: String,
    pub status: String,
}

impl SemanticDomain {
    pub fn canonical_identity(&self) -> String {
        format!("domain:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticRuleBinding {
    pub line_number: usize,
    pub id: String,
    pub domain: String,
    pub kind: String,
    pub input: String,
    pub output: String,
    pub forbids: Vec<String>,
    pub receipt: String,
    pub status: String,
}

impl SemanticRuleBinding {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_rule:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionLaw {
    pub line_number: usize,
    pub id: String,
    pub from: String,
    pub to: String,
    pub guard: String,
    pub receipt: String,
    pub status: String,
}

impl TransitionLaw {
    pub fn canonical_identity(&self) -> String {
        format!("transition:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantBinding {
    pub line_number: usize,
    pub id: String,
    pub domain: String,
    pub invariant: String,
    pub rejects: Vec<String>,
    pub receipt: String,
    pub status: String,
}

impl InvariantBinding {
    pub fn canonical_identity(&self) -> String {
        format!("invariant:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProof {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub domains: Vec<String>,
    pub rules: Vec<String>,
    pub transitions: Vec<String>,
    pub invariants: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
    pub forbids: Vec<String>,
}

impl SemanticProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticsSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub domains: Vec<SemanticDomain>,
    pub semantic_rules: Vec<SemanticRuleBinding>,
    pub transitions: Vec<TransitionLaw>,
    pub invariants: Vec<InvariantBinding>,
    pub proofs: Vec<SemanticProof>,
}

impl FormalSemanticsSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn domain_by_id(&self, id: &str) -> Option<&SemanticDomain> {
        self.domains.iter().find(|item| item.id == id)
    }

    pub fn semantic_rule_by_id(&self, id: &str) -> Option<&SemanticRuleBinding> {
        self.semantic_rules.iter().find(|item| item.id == id)
    }

    pub fn transition_by_id(&self, id: &str) -> Option<&TransitionLaw> {
        self.transitions.iter().find(|item| item.id == id)
    }

    pub fn invariant_by_id(&self, id: &str) -> Option<&InvariantBinding> {
        self.invariants.iter().find(|item| item.id == id)
    }

    pub fn proof_by_id(&self, id: &str) -> Option<&SemanticProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
