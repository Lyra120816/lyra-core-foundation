use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEvidenceFixtureBinding {
    pub line_number: usize,
    pub id: String,
    pub fixture_kind: String,
    pub path: String,
    pub binds_task: String,
    pub source_receipt: String,
    pub expected_verdict: String,
    pub status: String,
}

impl BootstrapEvidenceFixtureBinding {
    pub fn canonical_identity(&self) -> String {
        format!("fixture:{}", self.id)
    }
    pub fn expects_acceptance(&self) -> bool {
        self.expected_verdict == "accepted"
    }
    pub fn expects_rejection(&self) -> bool {
        self.expected_verdict == "rejected"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTargetMatrixReportBinding {
    pub line_number: usize,
    pub id: String,
    pub target_id: String,
    pub target_class: String,
    pub proof_count: usize,
    pub required_families: Vec<String>,
    pub matrix_receipt: String,
    pub status: String,
}

impl BootstrapTargetMatrixReportBinding {
    pub fn canonical_identity(&self) -> String {
        format!("target_report:{}", self.id)
    }
    pub fn binds_target(&self, target: &str) -> bool {
        self.target_id == target
    }
    pub fn pending_local_validation(&self) -> bool {
        self.status == "pending_local_validation_report_emitted"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapChallengeReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub suite_id: String,
    pub surface_ref: String,
    pub receipt_path: String,
    pub receipt_hash_state: String,
    pub challenge_kind: String,
    pub truth_effect: String,
    pub status: String,
}

impl BootstrapChallengeReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("challenge_receipt:{}", self.id)
    }
    pub fn binds_suite(&self, suite: &str) -> bool {
        self.suite_id == suite
    }
    pub fn truth_neutral(&self) -> bool {
        self.truth_effect == "none_without_local_replay"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEvidenceEmissionReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
}

impl BootstrapEvidenceEmissionReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEvidenceEmissionSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub extinction_ledger_receipt: String,
    pub target_matrix_receipt: String,
    pub host_boundary_receipt: String,
    pub replacement_milestones_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub fixtures: Vec<BootstrapEvidenceFixtureBinding>,
    pub target_reports: Vec<BootstrapTargetMatrixReportBinding>,
    pub challenge_receipts: Vec<BootstrapChallengeReceiptBinding>,
    pub receipts: Vec<BootstrapEvidenceEmissionReceiptBinding>,
}

impl BootstrapEvidenceEmissionSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn fixture_by_id(&self, id: &str) -> Option<&BootstrapEvidenceFixtureBinding> {
        self.fixtures.iter().find(|x| x.id == id)
    }
    pub fn target_report_by_target(
        &self,
        target: &str,
    ) -> Option<&BootstrapTargetMatrixReportBinding> {
        self.target_reports.iter().find(|x| x.target_id == target)
    }
    pub fn challenge_receipt_by_suite(
        &self,
        suite: &str,
    ) -> Option<&BootstrapChallengeReceiptBinding> {
        self.challenge_receipts.iter().find(|x| x.suite_id == suite)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&BootstrapEvidenceEmissionReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
}
