use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDependencyNode {
    pub line_number: usize,
    pub id: String,
    pub node_kind: String,
    pub owner_roots: Vec<String>,
    pub depends: Vec<String>,
    pub unlocks: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl SemanticDependencyNode {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_dependency:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBlockerBinding {
    pub line_number: usize,
    pub id: String,
    pub blocked_by: Vec<String>,
    pub reason: String,
    pub unblocks: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl SemanticBlockerBinding {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_blocker:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticParallelLane {
    pub line_number: usize,
    pub id: String,
    pub lane_kind: String,
    pub frontiers: Vec<String>,
    pub after: Vec<String>,
    pub can_parallel_with: Vec<String>,
    pub receipts: Vec<String>,
    pub status: String,
}

impl SemanticParallelLane {
    pub fn canonical_identity(&self) -> String {
        format!("semantic_lane:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDependencyMatrixSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub dependencies: Vec<SemanticDependencyNode>,
    pub blockers: Vec<SemanticBlockerBinding>,
    pub lanes: Vec<SemanticParallelLane>,
}

impl SemanticDependencyMatrixSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn dependency_by_id(&self, id: &str) -> Option<&SemanticDependencyNode> {
        self.dependencies.iter().find(|item| item.id == id)
    }

    pub fn blocker_by_id(&self, id: &str) -> Option<&SemanticBlockerBinding> {
        self.blockers.iter().find(|item| item.id == id)
    }

    pub fn lane_by_id(&self, id: &str) -> Option<&SemanticParallelLane> {
        self.lanes.iter().find(|item| item.id == id)
    }
}
