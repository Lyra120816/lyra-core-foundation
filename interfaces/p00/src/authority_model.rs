use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityLayer {
    pub line_number: usize,
    pub rank: u16,
    pub name: String,
    pub authority: String,
    pub scope: String,
    pub supersedes: Vec<String>,
    pub requires: Vec<String>,
}

impl AuthorityLayer {
    pub fn supersedes_name(&self, name: &str) -> bool {
        self.supersedes.iter().any(|item| item == name)
    }

    pub fn requires_token(&self, token: &str) -> bool {
        self.requires.iter().any(|item| item == token)
    }

    pub fn canonical_identity(&self) -> String {
        format!("{:03}:{}", self.rank, self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityOrderSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub layers: Vec<AuthorityLayer>,
    pub rules: BTreeMap<String, String>,
}

impl AuthorityOrderSurface {
    pub fn layer_by_name(&self, name: &str) -> Option<&AuthorityLayer> {
        self.layers.iter().find(|layer| layer.name == name)
    }

    pub fn layer_by_rank(&self, rank: u16) -> Option<&AuthorityLayer> {
        self.layers.iter().find(|layer| layer.rank == rank)
    }

    pub fn sorted_layers(&self) -> Vec<&AuthorityLayer> {
        let mut layers: Vec<&AuthorityLayer> = self.layers.iter().collect();
        layers.sort_by(|left, right| left.rank.cmp(&right.rank).then(left.name.cmp(&right.name)));
        layers
    }

    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
}
