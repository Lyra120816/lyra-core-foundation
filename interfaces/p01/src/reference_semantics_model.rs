use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceLiteralBinding {
    pub line_number: usize,
    pub id: String,
    pub atom: String,
    pub canonical: String,
    pub normal: String,
    pub evaluator: String,
    pub proof: String,
    pub status: String,
}
impl ReferenceLiteralBinding {
    pub fn canonical_identity(&self) -> String {
        format!("literal:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceCompositionBinding {
    pub line_number: usize,
    pub id: String,
    pub operator: String,
    pub arity: String,
    pub input_order: String,
    pub output: String,
    pub law: String,
    pub status: String,
}
impl ReferenceCompositionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("composition:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceEvalSeedBinding {
    pub line_number: usize,
    pub id: String,
    pub input: String,
    pub reduction: String,
    pub expected: String,
    pub law: String,
    pub trace: String,
    pub status: String,
}
impl ReferenceEvalSeedBinding {
    pub fn canonical_identity(&self) -> String {
        format!("eval_seed:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceSemanticsReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl ReferenceSemanticsReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceSemanticsSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub literals: Vec<ReferenceLiteralBinding>,
    pub compositions: Vec<ReferenceCompositionBinding>,
    pub eval_seeds: Vec<ReferenceEvalSeedBinding>,
    pub receipts: Vec<ReferenceSemanticsReceiptBinding>,
}

impl ReferenceSemanticsSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn literal_by_id(&self, id: &str) -> Option<&ReferenceLiteralBinding> {
        self.literals.iter().find(|item| item.id == id)
    }
    pub fn composition_by_id(&self, id: &str) -> Option<&ReferenceCompositionBinding> {
        self.compositions.iter().find(|item| item.id == id)
    }
    pub fn eval_seed_by_id(&self, id: &str) -> Option<&ReferenceEvalSeedBinding> {
        self.eval_seeds.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&ReferenceSemanticsReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
