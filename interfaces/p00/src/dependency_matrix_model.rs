use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyNode {
    pub line_number: usize,
    pub id: String,
    pub node_kind: String,
    pub owner_roots: Vec<String>,
    pub depends: Vec<String>,
    pub unlocks: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl DependencyNode {
    pub fn canonical_identity(&self) -> String {
        format!("dependency:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockerBinding {
    pub line_number: usize,
    pub id: String,
    pub blocked_by: Vec<String>,
    pub reason: String,
    pub unblocks: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl BlockerBinding {
    pub fn canonical_identity(&self) -> String {
        format!("blocker:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelLane {
    pub line_number: usize,
    pub id: String,
    pub lane_kind: String,
    pub frontiers: Vec<String>,
    pub after: Vec<String>,
    pub can_parallel_with: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl ParallelLane {
    pub fn canonical_identity(&self) -> String {
        format!("lane:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyMatrixSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub dependencies: Vec<DependencyNode>,
    pub blockers: Vec<BlockerBinding>,
    pub lanes: Vec<ParallelLane>,
}

impl DependencyMatrixSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn dependency_by_id(&self, id: &str) -> Option<&DependencyNode> {
        self.dependencies.iter().find(|item| item.id == id)
    }

    pub fn blocker_by_id(&self, id: &str) -> Option<&BlockerBinding> {
        self.blockers.iter().find(|item| item.id == id)
    }

    pub fn lane_by_id(&self, id: &str) -> Option<&ParallelLane> {
        self.lanes.iter().find(|item| item.id == id)
    }
}
