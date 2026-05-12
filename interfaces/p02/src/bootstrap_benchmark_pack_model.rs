use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapBenchmarkFamilyBinding {
    pub line_number: usize,
    pub id: String,
    pub family_kind: String,
    pub scope: String,
    pub targets: Vec<String>,
    pub proofs: Vec<String>,
    pub status: String,
}
impl BootstrapBenchmarkFamilyBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_benchmark_family:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapBenchmarkTargetBinding {
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
    pub expected: String,
    pub status: String,
}
impl BootstrapBenchmarkTargetBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_benchmark_target:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapBenchmarkEvidenceBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub targets: Vec<String>,
    pub artifacts: Vec<String>,
    pub proof_receipts: Vec<String>,
    pub status: String,
}
impl BootstrapBenchmarkEvidenceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("bootstrap_benchmark_evidence:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapBenchmarkPackSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub closure_scope: String,
    pub global_closure: String,
    pub next_frontier: String,
    pub rules: BTreeMap<String, String>,
    pub families: Vec<BootstrapBenchmarkFamilyBinding>,
    pub targets: Vec<BootstrapBenchmarkTargetBinding>,
    pub evidence: Vec<BootstrapBenchmarkEvidenceBinding>,
}
impl BootstrapBenchmarkPackSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn family_by_id(&self, id: &str) -> Option<&BootstrapBenchmarkFamilyBinding> {
        self.families.iter().find(|item| item.id == id)
    }
    pub fn target_by_id(&self, id: &str) -> Option<&BootstrapBenchmarkTargetBinding> {
        self.targets.iter().find(|item| item.id == id)
    }
    pub fn evidence_by_id(&self, id: &str) -> Option<&BootstrapBenchmarkEvidenceBinding> {
        self.evidence.iter().find(|item| item.id == id)
    }
}
