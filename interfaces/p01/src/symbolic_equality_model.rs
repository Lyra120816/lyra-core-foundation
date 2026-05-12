use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEqualityRuleBinding {
    pub line_number: usize,
    pub id: String,
    pub domain: String,
    pub relation: String,
    pub law: String,
    pub status: String,
}
impl SymbolicEqualityRuleBinding {
    pub fn canonical_identity(&self) -> String {
        format!("equality_rule:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEquivalenceClassBinding {
    pub line_number: usize,
    pub id: String,
    pub members: String,
    pub canonical: String,
    pub normalizer: String,
    pub status: String,
}
impl SymbolicEquivalenceClassBinding {
    pub fn canonical_identity(&self) -> String {
        format!("equivalence_class:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicNormalizationCaseBinding {
    pub line_number: usize,
    pub id: String,
    pub input: String,
    pub output: String,
    pub law: String,
    pub digest: String,
    pub status: String,
}
impl SymbolicNormalizationCaseBinding {
    pub fn canonical_identity(&self) -> String {
        format!("normalization:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicSubstitutionCaseBinding {
    pub line_number: usize,
    pub id: String,
    pub target: String,
    pub replacement: String,
    pub scope: String,
    pub expected: String,
    pub law: String,
    pub digest: String,
    pub status: String,
}
impl SymbolicSubstitutionCaseBinding {
    pub fn canonical_identity(&self) -> String {
        format!("substitution:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEqualityReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl SymbolicEqualityReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEqualitySurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub equality_rules: Vec<SymbolicEqualityRuleBinding>,
    pub equivalence_classes: Vec<SymbolicEquivalenceClassBinding>,
    pub normalizations: Vec<SymbolicNormalizationCaseBinding>,
    pub substitutions: Vec<SymbolicSubstitutionCaseBinding>,
    pub receipts: Vec<SymbolicEqualityReceiptBinding>,
}

impl SymbolicEqualitySurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn equality_rule_by_id(&self, id: &str) -> Option<&SymbolicEqualityRuleBinding> {
        self.equality_rules.iter().find(|item| item.id == id)
    }
    pub fn equivalence_class_by_id(&self, id: &str) -> Option<&SymbolicEquivalenceClassBinding> {
        self.equivalence_classes.iter().find(|item| item.id == id)
    }
    pub fn normalization_by_id(&self, id: &str) -> Option<&SymbolicNormalizationCaseBinding> {
        self.normalizations.iter().find(|item| item.id == id)
    }
    pub fn substitution_by_id(&self, id: &str) -> Option<&SymbolicSubstitutionCaseBinding> {
        self.substitutions.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SymbolicEqualityReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
