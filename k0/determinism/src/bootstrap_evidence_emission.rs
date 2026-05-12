use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEvidenceFixtureReport {
    pub id: String,
    pub fixture_kind: String,
    pub expected_verdict: String,
    pub fixture_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapTargetMatrixReportEmission {
    pub id: String,
    pub target_id: String,
    pub proof_count: usize,
    pub target_report_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapChallengeReceiptReport {
    pub id: String,
    pub suite_id: String,
    pub challenge_receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEvidenceEmissionReport {
    pub fixture_count: usize,
    pub accepted_fixture_count: usize,
    pub rejected_fixture_count: usize,
    pub target_report_count: usize,
    pub target_class_count: usize,
    pub challenge_receipt_count: usize,
    pub truth_neutral_challenge_count: usize,
    pub receipt_count: usize,
    pub fixture_reports: Vec<BootstrapEvidenceFixtureReport>,
    pub target_reports: Vec<BootstrapTargetMatrixReportEmission>,
    pub challenge_reports: Vec<BootstrapChallengeReceiptReport>,
    pub emission_hash: String,
}

pub fn deterministic_bootstrap_evidence_emission_report(
    fixtures: &[(String, String, String, String, String, String, String)],
    target_reports: &[(String, String, String, usize, Vec<String>, String, String)],
    challenge_receipts: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    receipts: &[(String, String, String, String)],
) -> BootstrapEvidenceEmissionReport {
    let mut ordered_fixtures = fixtures.to_vec();
    ordered_fixtures.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_targets = target_reports.to_vec();
    ordered_targets.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_challenges = challenge_receipts.to_vec();
    ordered_challenges.sort_by(|left, right| left.0.cmp(&right.0));
    let mut ordered_receipts = receipts.to_vec();
    ordered_receipts.sort_by(|left, right| left.0.cmp(&right.0));

    let mut accepted_fixture_count = 0usize;
    let mut rejected_fixture_count = 0usize;
    let mut target_classes = Vec::new();
    let mut truth_neutral_challenge_count = 0usize;
    let mut preimage = format!(
        "fixtures:{}|target_reports:{}|challenge_receipts:{}|receipts:{}",
        ordered_fixtures.len(),
        ordered_targets.len(),
        ordered_challenges.len(),
        ordered_receipts.len()
    );

    let mut fixture_reports = Vec::new();
    for (id, fixture_kind, path, binds_task, source_receipt, expected_verdict, status) in
        ordered_fixtures
    {
        if expected_verdict == "accepted" {
            accepted_fixture_count += 1;
        }
        if expected_verdict == "rejected" {
            rejected_fixture_count += 1;
        }
        let row = format!("fixture:{id}:{fixture_kind}:{path}:{binds_task}:{source_receipt}:{expected_verdict}:{status}");
        preimage.push('|');
        preimage.push_str(&row);
        fixture_reports.push(BootstrapEvidenceFixtureReport {
            id,
            fixture_kind,
            expected_verdict,
            fixture_hash: stable_hash_label("lyra.p02.bootstrap_evidence.fixture", &row),
        });
    }

    let mut emitted_target_reports = Vec::new();
    for (id, target_id, target_class, proof_count, mut required_families, matrix_receipt, status) in
        ordered_targets
    {
        required_families.sort();
        target_classes.push(target_class.clone());
        let row = format!(
            "target_report:{}:{}:{}:{}:{}:{}:{}",
            id,
            target_id,
            target_class,
            proof_count,
            required_families.join(","),
            matrix_receipt,
            status
        );
        preimage.push('|');
        preimage.push_str(&row);
        emitted_target_reports.push(BootstrapTargetMatrixReportEmission {
            id,
            target_id,
            proof_count,
            target_report_hash: stable_hash_label(
                "lyra.p02.bootstrap_evidence.target_report",
                &row,
            ),
        });
    }

    let mut emitted_challenge_reports = Vec::new();
    for (
        id,
        suite_id,
        surface_ref,
        receipt_path,
        receipt_hash_state,
        challenge_kind,
        truth_effect,
        status,
    ) in ordered_challenges
    {
        if truth_effect == "none_without_local_replay" {
            truth_neutral_challenge_count += 1;
        }
        let row = format!("challenge_receipt:{id}:{suite_id}:{surface_ref}:{receipt_path}:{receipt_hash_state}:{challenge_kind}:{truth_effect}:{status}");
        preimage.push('|');
        preimage.push_str(&row);
        emitted_challenge_reports.push(BootstrapChallengeReceiptReport {
            id,
            suite_id,
            challenge_receipt_hash: stable_hash_label(
                "lyra.p02.bootstrap_evidence.challenge_receipt",
                &row,
            ),
        });
    }

    for (id, path, target, status) in ordered_receipts {
        preimage.push_str(&format!("|receipt:{id}:{path}:{target}:{status}"));
    }
    target_classes.sort();
    target_classes.dedup();

    BootstrapEvidenceEmissionReport {
        fixture_count: fixtures.len(),
        accepted_fixture_count,
        rejected_fixture_count,
        target_report_count: target_reports.len(),
        target_class_count: target_classes.len(),
        challenge_receipt_count: challenge_receipts.len(),
        truth_neutral_challenge_count,
        receipt_count: receipts.len(),
        fixture_reports,
        target_reports: emitted_target_reports,
        challenge_reports: emitted_challenge_reports,
        emission_hash: stable_hash_label("lyra.p02.bootstrap_evidence.emission", &preimage),
    }
}
