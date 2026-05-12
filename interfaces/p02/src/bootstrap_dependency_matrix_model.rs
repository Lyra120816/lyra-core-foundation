use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyNode {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub status: String,
    pub depends: Vec<String>,
    pub unblocks: Vec<String>,
    pub owner_root: String,
}

impl BootstrapDependencyNode {
    pub fn canonical_identity(&self) -> String {
        format!("node:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyBlocker {
    pub line_number: usize,
    pub id: String,
    pub target: String,
    pub severity: String,
    pub blocks: Vec<String>,
    pub requires: Vec<String>,
    pub status: String,
}

impl BootstrapDependencyBlocker {
    pub fn canonical_identity(&self) -> String {
        format!("blocker:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapParallelLane {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub tasks: Vec<String>,
    pub depends: Vec<String>,
    pub parallel_safe: String,
    pub status: String,
}

impl BootstrapParallelLane {
    pub fn canonical_identity(&self) -> String {
        format!("lane:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyProof {
    pub line_number: usize,
    pub id: String,
    pub nodes: Vec<String>,
    pub blockers: Vec<String>,
    pub lanes: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
    pub permits: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
}

impl BootstrapDependencyProof {
    pub fn canonical_identity(&self) -> String {
        format!("proof:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDependencyMatrixSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub closure_scope: String,
    pub global_closure: String,
    pub next_frontier: String,
    pub rules: BTreeMap<String, String>,
    pub nodes: Vec<BootstrapDependencyNode>,
    pub blockers: Vec<BootstrapDependencyBlocker>,
    pub lanes: Vec<BootstrapParallelLane>,
    pub proofs: Vec<BootstrapDependencyProof>,
}

impl BootstrapDependencyMatrixSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn node_by_id(&self, id: &str) -> Option<&BootstrapDependencyNode> {
        self.nodes.iter().find(|item| item.id == id)
    }
    pub fn blocker_by_id(&self, id: &str) -> Option<&BootstrapDependencyBlocker> {
        self.blockers.iter().find(|item| item.id == id)
    }
    pub fn lane_by_id(&self, id: &str) -> Option<&BootstrapParallelLane> {
        self.lanes.iter().find(|item| item.id == id)
    }
    pub fn proof_by_id(&self, id: &str) -> Option<&BootstrapDependencyProof> {
        self.proofs.iter().find(|item| item.id == id)
    }
}
