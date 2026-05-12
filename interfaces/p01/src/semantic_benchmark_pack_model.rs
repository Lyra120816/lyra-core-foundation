use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBenchmarkFamilyBinding {
    pub line_number: usize,
    pub id: String,
    pub family_kind: String,
    pub scope: String,
    pub targets: Vec<String>,
    pub proofs: Vec<String>,
    pub status: String,
}
impl SemanticBenchmarkFamilyBinding {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_benchmark_family:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBenchmarkTargetBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub metric: String,
    pub unit: String,
    pub threshold: String,
    pub command: String,
    pub fixture: String,
    pub golden: String,
    pub receipt: String,
    pub status: String,
}
impl SemanticBenchmarkTargetBinding {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_benchmark_target:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBenchmarkEvidenceBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub targets: Vec<String>,
    pub artifacts: Vec<String>,
    pub proof_receipts: Vec<String>,
    pub status: String,
}
impl SemanticBenchmarkEvidenceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_benchmark_evidence:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBenchmarkPackSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub families: Vec<SemanticBenchmarkFamilyBinding>,
    pub targets: Vec<SemanticBenchmarkTargetBinding>,
    pub evidence: Vec<SemanticBenchmarkEvidenceBinding>,
}
impl SemanticBenchmarkPackSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn family_by_id(&self, id: &str) -> Option<&SemanticBenchmarkFamilyBinding> {
        self.families.iter().find(|item| item.id == id)
    }
    pub fn target_by_id(&self, id: &str) -> Option<&SemanticBenchmarkTargetBinding> {
        self.targets.iter().find(|item| item.id == id)
    }
    pub fn evidence_by_id(&self, id: &str) -> Option<&SemanticBenchmarkEvidenceBinding> {
        self.evidence.iter().find(|item| item.id == id)
    }
}
