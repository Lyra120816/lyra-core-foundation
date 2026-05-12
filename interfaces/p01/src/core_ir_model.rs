use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrFormBinding {
    pub line_number: usize,
    pub id: String,
    pub medium: String,
    pub owner_root: String,
    pub version: String,
    pub header: String,
    pub extension: String,
    pub encoding: String,
    pub canonicalization: String,
    pub status: String,
}

impl CoreIrFormBinding {
    pub fn canonical_identity(&self) -> String {
        format!("form:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrVersionBinding {
    pub line_number: usize,
    pub id: String,
    pub major: String,
    pub minor: String,
    pub stability: String,
    pub upgrade_policy: String,
    pub status: String,
}

impl CoreIrVersionBinding {
    pub fn canonical_identity(&self) -> String {
        format!("version:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrUpgradeBinding {
    pub line_number: usize,
    pub id: String,
    pub from_version: String,
    pub to_version: String,
    pub law: String,
    pub compatibility: String,
    pub status: String,
}

impl CoreIrUpgradeBinding {
    pub fn canonical_identity(&self) -> String {
        format!("upgrade:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrParityBinding {
    pub line_number: usize,
    pub id: String,
    pub text_form: String,
    pub binary_form: String,
    pub fixture: String,
    pub atom: String,
    pub round_trip: String,
    pub status: String,
}

impl CoreIrParityBinding {
    pub fn canonical_identity(&self) -> String {
        format!("parity:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}

impl CoreIrReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub forms: Vec<CoreIrFormBinding>,
    pub versions: Vec<CoreIrVersionBinding>,
    pub upgrades: Vec<CoreIrUpgradeBinding>,
    pub parities: Vec<CoreIrParityBinding>,
    pub receipts: Vec<CoreIrReceiptBinding>,
}

impl CoreIrSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn form_by_id(&self, id: &str) -> Option<&CoreIrFormBinding> {
        self.forms.iter().find(|item| item.id == id)
    }
    pub fn version_by_id(&self, id: &str) -> Option<&CoreIrVersionBinding> {
        self.versions.iter().find(|item| item.id == id)
    }
    pub fn upgrade_by_id(&self, id: &str) -> Option<&CoreIrUpgradeBinding> {
        self.upgrades.iter().find(|item| item.id == id)
    }
    pub fn parity_by_id(&self, id: &str) -> Option<&CoreIrParityBinding> {
        self.parities.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&CoreIrReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
