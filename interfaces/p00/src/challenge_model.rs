use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewGate {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub required_before: String,
    pub reviewers: Vec<String>,
    pub evidence: Vec<String>,
    pub status: String,
}

impl ReviewGate {
    pub fn canonical_identity(&self) -> String {
        format!("review:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChallengeRight {
    pub line_number: usize,
    pub id: String,
    pub holder: String,
    pub scope: String,
    pub trigger: String,
    pub remedy: String,
    pub evidence: Vec<String>,
    pub protection: String,
}

impl ChallengeRight {
    pub fn canonical_identity(&self) -> String {
        format!("challenge:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackAuthority {
    pub line_number: usize,
    pub id: String,
    pub holder: String,
    pub scope: String,
    pub target: String,
    pub requires: Vec<String>,
    pub evidence: Vec<String>,
    pub status: String,
}

impl RollbackAuthority {
    pub fn canonical_identity(&self) -> String {
        format!("rollback:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendmentGate {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub requires: Vec<String>,
    pub forbids: Vec<String>,
    pub evidence: Vec<String>,
    pub status: String,
}

impl AmendmentGate {
    pub fn canonical_identity(&self) -> String {
        format!("amendment:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChallengeLawSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub reviews: Vec<ReviewGate>,
    pub challenges: Vec<ChallengeRight>,
    pub rollbacks: Vec<RollbackAuthority>,
    pub amendments: Vec<AmendmentGate>,
}

impl ChallengeLawSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn review_by_id(&self, id: &str) -> Option<&ReviewGate> {
        self.reviews.iter().find(|review| review.id == id)
    }

    pub fn challenge_by_id(&self, id: &str) -> Option<&ChallengeRight> {
        self.challenges.iter().find(|challenge| challenge.id == id)
    }

    pub fn rollback_by_id(&self, id: &str) -> Option<&RollbackAuthority> {
        self.rollbacks.iter().find(|rollback| rollback.id == id)
    }

    pub fn amendment_by_id(&self, id: &str) -> Option<&AmendmentGate> {
        self.amendments.iter().find(|amendment| amendment.id == id)
    }
}
