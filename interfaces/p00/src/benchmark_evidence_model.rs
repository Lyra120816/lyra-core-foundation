use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkTarget {
    pub line_number: usize,
    pub id: String,
    pub metric: String,
    pub target: String,
    pub method: String,
    pub stability: String,
    pub evidence: Vec<String>,
}

impl BenchmarkTarget {
    pub fn canonical_identity(&self) -> String {
        format!("benchmark:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceBinding {
    pub line_number: usize,
    pub id: String,
    pub family: String,
    pub artifacts: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub status: String,
}

impl EvidenceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("evidence:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionOfDone {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub requires: Vec<String>,
    pub allows: Vec<String>,
    pub forbids: Vec<String>,
    pub evidence: Vec<String>,
}

impl DefinitionOfDone {
    pub fn canonical_identity(&self) -> String {
        format!("definition:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureFormula {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub status: String,
    pub benchmarks: Vec<String>,
    pub evidence: Vec<String>,
    pub definitions: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
}

impl ClosureFormula {
    pub fn canonical_identity(&self) -> String {
        format!("closure:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkEvidenceLawSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub benchmarks: Vec<BenchmarkTarget>,
    pub evidence: Vec<EvidenceBinding>,
    pub definitions: Vec<DefinitionOfDone>,
    pub closures: Vec<ClosureFormula>,
}

impl BenchmarkEvidenceLawSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn benchmark_by_id(&self, id: &str) -> Option<&BenchmarkTarget> {
        self.benchmarks.iter().find(|benchmark| benchmark.id == id)
    }

    pub fn evidence_by_id(&self, id: &str) -> Option<&EvidenceBinding> {
        self.evidence.iter().find(|evidence| evidence.id == id)
    }

    pub fn definition_by_id(&self, id: &str) -> Option<&DefinitionOfDone> {
        self.definitions
            .iter()
            .find(|definition| definition.id == id)
    }

    pub fn closure_by_id(&self, id: &str) -> Option<&ClosureFormula> {
        self.closures.iter().find(|closure| closure.id == id)
    }
}
