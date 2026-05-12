#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    Constitution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedEntry {
    pub line_number: usize,
    pub namespace: String,
    pub name: String,
    pub value: String,
}

impl ParsedEntry {
    pub fn identity(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSurface {
    pub kind: SurfaceKind,
    pub header: String,
    pub entries: Vec<ParsedEntry>,
}

impl ParsedSurface {
    pub fn values_for(&self, namespace: &str) -> Vec<&ParsedEntry> {
        let mut matches: Vec<&ParsedEntry> = self
            .entries
            .iter()
            .filter(|entry| entry.namespace == namespace)
            .collect();
        matches.sort_by(|left, right| {
            left.name
                .cmp(&right.name)
                .then(left.value.cmp(&right.value))
        });
        matches
    }

    pub fn has_value(&self, namespace: &str, name: &str) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.namespace == namespace && entry.name == name)
    }

    pub fn scalar_value(&self, name: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|entry| entry.namespace == "field" && entry.name == name)
            .map(|entry| entry.value.as_str())
    }
}
