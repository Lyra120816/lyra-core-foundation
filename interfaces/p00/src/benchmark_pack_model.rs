use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkFamilyBinding {
    pub line_number: usize,
    pub id: String,
    pub family_kind: String,
    pub scope: String,
    pub targets: Vec<String>,
    pub proofs: Vec<String>,
    pub status: String,
}
impl BenchmarkFamilyBinding {
    pub fn canonical_identity(&self) -> String {
        format!("benchmark_family:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkTargetBinding {
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
impl BenchmarkTargetBinding {
    pub fn canonical_identity(&self) -> String {
        format!("benchmark_target:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkEvidenceBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub targets: Vec<String>,
    pub artifacts: Vec<String>,
    pub proof_receipts: Vec<String>,
    pub status: String,
}
impl BenchmarkEvidenceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("benchmark_evidence:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkPackSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub families: Vec<BenchmarkFamilyBinding>,
    pub targets: Vec<BenchmarkTargetBinding>,
    pub evidence: Vec<BenchmarkEvidenceBinding>,
}
impl BenchmarkPackSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn family_by_id(&self, id: &str) -> Option<&BenchmarkFamilyBinding> {
        self.families.iter().find(|item| item.id == id)
    }
    pub fn target_by_id(&self, id: &str) -> Option<&BenchmarkTargetBinding> {
        self.targets.iter().find(|item| item.id == id)
    }
    pub fn evidence_by_id(&self, id: &str) -> Option<&BenchmarkEvidenceBinding> {
        self.evidence.iter().find(|item| item.id == id)
    }
}
