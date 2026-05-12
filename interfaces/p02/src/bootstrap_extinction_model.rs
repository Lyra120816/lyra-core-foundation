use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapExtinctionEntryBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub classification: String,
    pub surface_ref: String,
    pub deletion_gate: String,
    pub retirement_trigger: String,
    pub deletion_action: String,
    pub successor: String,
    pub ledger_state: String,
    pub evidence: Vec<String>,
    pub receipt: String,
    pub status: String,
}

impl BootstrapExtinctionEntryBinding {
    pub fn canonical_identity(&self) -> String {
        format!("entry:{}", self.id)
    }
    pub fn is_temporary(&self) -> bool {
        self.classification == "temporary"
    }
    pub fn is_observer(&self) -> bool {
        self.classification == "observer"
    }
    pub fn is_bounded_permanent(&self) -> bool {
        self.classification == "bounded_permanent"
    }
    pub fn is_forbidden(&self) -> bool {
        self.classification == "forbidden"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapExtinctionLedgerGateBinding {
    pub line_number: usize,
    pub id: String,
    pub surface: String,
    pub gate_kind: String,
    pub trigger: String,
    pub allowed_action: String,
    pub blocked_action: String,
    pub evidence: Vec<String>,
    pub status: String,
}

impl BootstrapExtinctionLedgerGateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("gate:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapExtinctionReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}

impl BootstrapExtinctionReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapExtinctionLedgerSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub inventory_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub entries: Vec<BootstrapExtinctionEntryBinding>,
    pub gates: Vec<BootstrapExtinctionLedgerGateBinding>,
    pub receipts: Vec<BootstrapExtinctionReceiptBinding>,
}

impl BootstrapExtinctionLedgerSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn entry_by_id(&self, id: &str) -> Option<&BootstrapExtinctionEntryBinding> {
        self.entries.iter().find(|item| item.id == id)
    }
    pub fn gate_by_id(&self, id: &str) -> Option<&BootstrapExtinctionLedgerGateBinding> {
        self.gates.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapExtinctionReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
    pub fn temporary_entries(&self) -> impl Iterator<Item = &BootstrapExtinctionEntryBinding> {
        self.entries.iter().filter(|item| item.is_temporary())
    }
    pub fn observer_entries(&self) -> impl Iterator<Item = &BootstrapExtinctionEntryBinding> {
        self.entries.iter().filter(|item| item.is_observer())
    }
    pub fn bounded_permanent_entries(
        &self,
    ) -> impl Iterator<Item = &BootstrapExtinctionEntryBinding> {
        self.entries
            .iter()
            .filter(|item| item.is_bounded_permanent())
    }
    pub fn forbidden_entries(&self) -> impl Iterator<Item = &BootstrapExtinctionEntryBinding> {
        self.entries.iter().filter(|item| item.is_forbidden())
    }
}
