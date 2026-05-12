use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicInterestSafeguard {
    pub line_number: usize,
    pub id: String,
    pub category: String,
    pub protects: Vec<String>,
    pub forbids: Vec<String>,
    pub evidence: Vec<String>,
    pub review: String,
}

impl PublicInterestSafeguard {
    pub fn canonical_identity(&self) -> String {
        format!("safeguard:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParticipationRight {
    pub line_number: usize,
    pub id: String,
    pub constituency: String,
    pub rights: Vec<String>,
    pub channels: Vec<String>,
    pub protections: Vec<String>,
    pub evidence: Vec<String>,
}

impl ParticipationRight {
    pub fn canonical_identity(&self) -> String {
        format!("right:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AntiExtractiveDuty {
    pub line_number: usize,
    pub id: String,
    pub duty: String,
    pub requires: Vec<String>,
    pub forbids: Vec<String>,
    pub audit: String,
    pub evidence: Vec<String>,
}

impl AntiExtractiveDuty {
    pub fn canonical_identity(&self) -> String {
        format!("duty:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StewardshipClaim {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub status: String,
    pub safeguards: Vec<String>,
    pub rights: Vec<String>,
    pub duties: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
}

impl StewardshipClaim {
    pub fn canonical_identity(&self) -> String {
        format!("stewardship:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicInterestLawSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub safeguards: Vec<PublicInterestSafeguard>,
    pub rights: Vec<ParticipationRight>,
    pub duties: Vec<AntiExtractiveDuty>,
    pub stewardship: Vec<StewardshipClaim>,
}

impl PublicInterestLawSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn safeguard_by_id(&self, id: &str) -> Option<&PublicInterestSafeguard> {
        self.safeguards.iter().find(|item| item.id == id)
    }

    pub fn right_by_id(&self, id: &str) -> Option<&ParticipationRight> {
        self.rights.iter().find(|item| item.id == id)
    }

    pub fn duty_by_id(&self, id: &str) -> Option<&AntiExtractiveDuty> {
        self.duties.iter().find(|item| item.id == id)
    }

    pub fn stewardship_by_id(&self, id: &str) -> Option<&StewardshipClaim> {
        self.stewardship.iter().find(|item| item.id == id)
    }
}
