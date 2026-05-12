use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhaseIdentity {
    pub line_number: usize,
    pub id: String,
    pub name: String,
    pub owner_roots: Vec<String>,
    pub status: String,
    pub supersedes: Vec<String>,
    pub requires: Vec<String>,
}

impl PhaseIdentity {
    pub fn canonical_identity(&self) -> String {
        format!("phase:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskIdentity {
    pub line_number: usize,
    pub id: String,
    pub name: String,
    pub kind: String,
    pub phase: String,
    pub owner_roots: Vec<String>,
    pub status: String,
    pub supersedes: Vec<String>,
    pub requires: Vec<String>,
}

impl TaskIdentity {
    pub fn canonical_identity(&self) -> String {
        format!("task:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityLawSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub phases: Vec<PhaseIdentity>,
    pub tasks: Vec<TaskIdentity>,
    pub rules: BTreeMap<String, String>,
}

impl IdentityLawSurface {
    pub fn phase_by_id(&self, id: &str) -> Option<&PhaseIdentity> {
        self.phases.iter().find(|phase| phase.id == id)
    }

    pub fn task_by_id(&self, id: &str) -> Option<&TaskIdentity> {
        self.tasks.iter().find(|task| task.id == id)
    }

    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn sorted_phase_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.phases.iter().map(|phase| phase.id.clone()).collect();
        ids.sort();
        ids
    }

    pub fn sorted_task_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.tasks.iter().map(|task| task.id.clone()).collect();
        ids.sort();
        ids
    }
}
