use crate::k0_hash::stable_hash_label;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorObjectReport {
    pub id: String,
    pub severity: String,
    pub subject: String,
    pub evidence_ref: String,
    pub object_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChallengeObjectReport {
    pub id: String,
    pub target: String,
    pub counter_evidence_ref: String,
    pub adjudication_law: String,
    pub object_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceObjectReport {
    pub id: String,
    pub kind: String,
    pub payload_digest: String,
    pub witness: String,
    pub object_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectLinkReport {
    pub id: String,
    pub from: String,
    pub relation: String,
    pub to: String,
    pub link_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorChallengeEvidenceReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorChallengeEvidenceSuiteReport {
    pub error_count: usize,
    pub challenge_count: usize,
    pub evidence_count: usize,
    pub link_count: usize,
    pub receipt_count: usize,
    pub error_reports: Vec<ErrorObjectReport>,
    pub challenge_reports: Vec<ChallengeObjectReport>,
    pub evidence_reports: Vec<EvidenceObjectReport>,
    pub link_reports: Vec<ObjectLinkReport>,
    pub receipt_reports: Vec<ErrorChallengeEvidenceReceiptReport>,
    pub suite_hash: String,
}
pub fn deterministic_error_challenge_evidence_suite_report(
    errors: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    challenges: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    evidence: &[(String, String, String, String, String, String, String)],
    links: &[(String, String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> ErrorChallengeEvidenceSuiteReport {
    let mut error_reports: Vec<ErrorObjectReport> = errors.iter().map(|item| { let preimage = format!("error_object:{}|severity:{}|domain:{}|subject:{}|message:{}|evidence_ref:{}|digest:{}|status:{}", item.0,item.1,item.2,item.3,item.4,item.5,item.6,item.7); ErrorObjectReport { id:item.0.clone(), severity:item.1.clone(), subject:item.3.clone(), evidence_ref:item.5.clone(), object_hash:stable_hash_label("lyra.p01.error_challenge_evidence.error_report", &preimage) } }).collect();
    error_reports.sort_by(|l, r| l.id.cmp(&r.id));
    let mut challenge_reports: Vec<ChallengeObjectReport> = challenges.iter().map(|item| { let preimage = format!("challenge_object:{}|target:{}|challenger:{}|claim_ref:{}|counter_evidence_ref:{}|adjudication_law:{}|digest:{}|status:{}", item.0,item.1,item.2,item.3,item.4,item.5,item.6,item.7); ChallengeObjectReport { id:item.0.clone(), target:item.1.clone(), counter_evidence_ref:item.4.clone(), adjudication_law:item.5.clone(), object_hash:stable_hash_label("lyra.p01.error_challenge_evidence.challenge_report", &preimage) } }).collect();
    challenge_reports.sort_by(|l, r| l.id.cmp(&r.id));
    let mut evidence_reports: Vec<EvidenceObjectReport> = evidence.iter().map(|item| { let preimage = format!("evidence_object:{}|kind:{}|source:{}|payload_digest:{}|witness:{}|digest:{}|status:{}", item.0,item.1,item.2,item.3,item.4,item.5,item.6); EvidenceObjectReport { id:item.0.clone(), kind:item.1.clone(), payload_digest:item.3.clone(), witness:item.4.clone(), object_hash:stable_hash_label("lyra.p01.error_challenge_evidence.evidence_report", &preimage) } }).collect();
    evidence_reports.sort_by(|l, r| l.id.cmp(&r.id));
    let mut link_reports: Vec<ObjectLinkReport> = links
        .iter()
        .map(|item| {
            let preimage = format!(
                "object_link:{}|from:{}|relation:{}|to:{}|law:{}|digest:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            ObjectLinkReport {
                id: item.0.clone(),
                from: item.1.clone(),
                relation: item.2.clone(),
                to: item.3.clone(),
                link_hash: stable_hash_label(
                    "lyra.p01.error_challenge_evidence.link_report",
                    &preimage,
                ),
            }
        })
        .collect();
    link_reports.sort_by(|l, r| l.id.cmp(&r.id));
    let mut receipt_reports: Vec<ErrorChallengeEvidenceReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|path:{}|target:{}|status:{}",
                item.0, item.1, item.2, item.3
            );
            ErrorChallengeEvidenceReceiptReport {
                id: item.0.clone(),
                path: item.1.clone(),
                target: item.2.clone(),
                receipt_hash: stable_hash_label(
                    "lyra.p01.error_challenge_evidence.receipt",
                    &preimage,
                ),
            }
        })
        .collect();
    receipt_reports.sort_by(|l, r| l.id.cmp(&r.id));
    let mut suite_lines = Vec::new();
    for item in &error_reports {
        suite_lines.push(format!("error_object:{}|{}", item.id, item.object_hash));
    }
    for item in &challenge_reports {
        suite_lines.push(format!("challenge_object:{}|{}", item.id, item.object_hash));
    }
    for item in &evidence_reports {
        suite_lines.push(format!(
            "evidence_object:{}|{}|{}",
            item.id, item.payload_digest, item.object_hash
        ));
    }
    for item in &link_reports {
        suite_lines.push(format!("object_link:{}|{}", item.id, item.link_hash));
    }
    for item in &receipt_reports {
        suite_lines.push(format!("receipt:{}|{}", item.id, item.receipt_hash));
    }
    suite_lines.sort();
    ErrorChallengeEvidenceSuiteReport {
        error_count: error_reports.len(),
        challenge_count: challenge_reports.len(),
        evidence_count: evidence_reports.len(),
        link_count: link_reports.len(),
        receipt_count: receipt_reports.len(),
        error_reports,
        challenge_reports,
        evidence_reports,
        link_reports,
        receipt_reports,
        suite_hash: stable_hash_label(
            "lyra.p01.error_challenge_evidence.suite",
            &suite_lines.join("\n"),
        ),
    }
}
