use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCollisionProbeBinding {
    pub line_number: usize,
    pub id: String,
    pub target_family: String,
    pub left_ref: String,
    pub right_ref: String,
    pub collision_domain: String,
    pub guard: String,
    pub expected_error: String,
    pub fixture_path: String,
    pub status: String,
}
impl SemanticCollisionProbeBinding {
    pub fn canonical_identity(&self) -> String {
        format!("collision_probe:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAmbiguityProbeBinding {
    pub line_number: usize,
    pub id: String,
    pub target_family: String,
    pub ambiguous_surface: String,
    pub deterministic_resolution: String,
    pub guard: String,
    pub expected_error: String,
    pub fixture_path: String,
    pub status: String,
}
impl SemanticAmbiguityProbeBinding {
    pub fn canonical_identity(&self) -> String {
        format!("ambiguity_probe:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticMalformedObjectBinding {
    pub line_number: usize,
    pub id: String,
    pub target_family: String,
    pub object_ref: String,
    pub malformed_field: String,
    pub rejection_law: String,
    pub expected_error: String,
    pub fixture_path: String,
    pub status: String,
}
impl SemanticMalformedObjectBinding {
    pub fn canonical_identity(&self) -> String {
        format!("malformed_object:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAdversarialHarnessBinding {
    pub line_number: usize,
    pub id: String,
    pub validator: String,
    pub case_ids: String,
    pub coverage: String,
    pub receipt_ref: String,
    pub status: String,
}
impl SemanticAdversarialHarnessBinding {
    pub fn canonical_identity(&self) -> String {
        format!("harness:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAdversarialReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}
impl SemanticAdversarialReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAdversarialCorpusSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub collision_probes: Vec<SemanticCollisionProbeBinding>,
    pub ambiguity_probes: Vec<SemanticAmbiguityProbeBinding>,
    pub malformed_objects: Vec<SemanticMalformedObjectBinding>,
    pub harnesses: Vec<SemanticAdversarialHarnessBinding>,
    pub receipts: Vec<SemanticAdversarialReceiptBinding>,
}
impl SemanticAdversarialCorpusSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn collision_probe_by_id(&self, id: &str) -> Option<&SemanticCollisionProbeBinding> {
        self.collision_probes.iter().find(|item| item.id == id)
    }
    pub fn ambiguity_probe_by_id(&self, id: &str) -> Option<&SemanticAmbiguityProbeBinding> {
        self.ambiguity_probes.iter().find(|item| item.id == id)
    }
    pub fn malformed_object_by_id(&self, id: &str) -> Option<&SemanticMalformedObjectBinding> {
        self.malformed_objects.iter().find(|item| item.id == id)
    }
    pub fn harness_by_id(&self, id: &str) -> Option<&SemanticAdversarialHarnessBinding> {
        self.harnesses.iter().find(|item| item.id == id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&SemanticAdversarialReceiptBinding> {
        self.receipts.iter().find(|item| item.id == id)
    }
}
