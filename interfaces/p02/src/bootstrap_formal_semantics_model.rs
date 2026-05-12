use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFormalDomainBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub source_task: String,
    pub semantic_object: String,
    pub constitutional_binding: String,
    pub status: String,
}

impl BootstrapFormalDomainBinding {
    pub fn canonical_identity(&self) -> String {
        format!("domain:{}", self.id)
    }
    pub fn constitutional(&self) -> bool {
        self.constitutional_binding == "constitutional_law_bound"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapConstitutionalLawBinding {
    pub line_number: usize,
    pub id: String,
    pub domain_id: String,
    pub law_class: String,
    pub governs: String,
    pub forbids: Vec<String>,
    pub requires_receipt: String,
    pub status: String,
}

impl BootstrapConstitutionalLawBinding {
    pub fn canonical_identity(&self) -> String {
        format!("law:{}", self.id)
    }
    pub fn receipt_bound(&self) -> bool {
        self.requires_receipt.starts_with("receipts/p02/")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFormalTransitionBinding {
    pub line_number: usize,
    pub id: String,
    pub from_state: String,
    pub to_state: String,
    pub guard: String,
    pub receipt: String,
    pub status: String,
}

impl BootstrapFormalTransitionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("transition:{}", self.id)
    }
    pub fn guarded(&self) -> bool {
        self.guard.starts_with("gate_") || self.guard.starts_with("receipt_gate_")
    }
    pub fn receipt_bound(&self) -> bool {
        self.receipt.starts_with("receipts/p02/")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFormalInvariantBinding {
    pub line_number: usize,
    pub id: String,
    pub domain_id: String,
    pub assertion: String,
    pub rejects: Vec<String>,
    pub receipt: String,
    pub status: String,
}

impl BootstrapFormalInvariantBinding {
    pub fn canonical_identity(&self) -> String {
        format!("invariant:{}", self.id)
    }
    pub fn rejects_token(&self, token: &str) -> bool {
        self.rejects.iter().any(|item| item == token)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFormalProofBinding {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub domains: Vec<String>,
    pub laws: Vec<String>,
    pub transitions: Vec<String>,
    pub invariants: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl BootstrapFormalProofBinding {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
    pub fn receipt_bound(&self) -> bool {
        self.receipts
            .iter()
            .all(|path| path.starts_with("receipts/p02/"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFormalReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub binds: String,
    pub status: String,
}

impl BootstrapFormalReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapFormalSemanticsSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub previous_evidence_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub domains: Vec<BootstrapFormalDomainBinding>,
    pub laws: Vec<BootstrapConstitutionalLawBinding>,
    pub transitions: Vec<BootstrapFormalTransitionBinding>,
    pub invariants: Vec<BootstrapFormalInvariantBinding>,
    pub proofs: Vec<BootstrapFormalProofBinding>,
    pub receipts: Vec<BootstrapFormalReceiptBinding>,
}

impl BootstrapFormalSemanticsSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn domain_by_id(&self, id: &str) -> Option<&BootstrapFormalDomainBinding> {
        self.domains.iter().find(|x| x.id == id)
    }
    pub fn law_by_id(&self, id: &str) -> Option<&BootstrapConstitutionalLawBinding> {
        self.laws.iter().find(|x| x.id == id)
    }
    pub fn transition_by_id(&self, id: &str) -> Option<&BootstrapFormalTransitionBinding> {
        self.transitions.iter().find(|x| x.id == id)
    }
    pub fn invariant_by_id(&self, id: &str) -> Option<&BootstrapFormalInvariantBinding> {
        self.invariants.iter().find(|x| x.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapFormalProofBinding> {
        self.proofs.iter().find(|x| x.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapFormalReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
    pub fn laws_for_domain(&self, domain_id: &str) -> Vec<&BootstrapConstitutionalLawBinding> {
        self.laws
            .iter()
            .filter(|x| x.domain_id == domain_id)
            .collect()
    }
    pub fn invariants_for_domain(&self, domain_id: &str) -> Vec<&BootstrapFormalInvariantBinding> {
        self.invariants
            .iter()
            .filter(|x| x.domain_id == domain_id)
            .collect()
    }
}
