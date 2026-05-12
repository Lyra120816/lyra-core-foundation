use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityDomainReport {
    pub id: String,
    pub domain: String,
    pub scope: String,
    pub material: String,
    pub canonicalizer: String,
    pub digest: String,
    pub collision: String,
    pub identity_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigestCaseReport {
    pub id: String,
    pub domain: String,
    pub expected_digest: String,
    pub case_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollisionCaseReport {
    pub id: String,
    pub domain: String,
    pub left: String,
    pub right: String,
    pub collision_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIdentityReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticIdentitySuiteReport {
    pub identity_count: usize,
    pub digest_case_count: usize,
    pub collision_case_count: usize,
    pub receipt_count: usize,
    pub stable_digest_count: usize,
    pub identity_reports: Vec<IdentityDomainReport>,
    pub digest_case_reports: Vec<DigestCaseReport>,
    pub collision_case_reports: Vec<CollisionCaseReport>,
    pub receipt_reports: Vec<SemanticIdentityReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_identity_suite_report(
    identities: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    digest_cases: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    collision_cases: &[(String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> SemanticIdentitySuiteReport {
    let mut identity_reports: Vec<IdentityDomainReport> = identities.iter().map(|item| {
        let preimage = format!("identity:{}|domain:{}|scope:{}|material:{}|canonicalizer:{}|digest:{}|collision:{}|status:{}", item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7);
        IdentityDomainReport { id: item.0.clone(), domain: item.1.clone(), scope: item.2.clone(), material: item.3.clone(), canonicalizer: item.4.clone(), digest: item.5.clone(), collision: item.6.clone(), identity_hash: stable_hash_label("lyra.p01.identity.domain", &preimage) }
    }).collect();
    identity_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut digest_case_reports: Vec<DigestCaseReport> = digest_cases.iter().map(|item| {
        let preimage = format!("digest_case:{}|domain:{}|owner:{}|payload:{}|normalization:{}|expected_digest:{}|status:{}|line:{}", item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7);
        DigestCaseReport { id: item.0.clone(), domain: item.1.clone(), expected_digest: item.5.clone(), case_hash: stable_hash_label("lyra.p01.identity.digest_case", &preimage) }
    }).collect();
    digest_case_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut collision_case_reports: Vec<CollisionCaseReport> = collision_cases
        .iter()
        .map(|item| {
            let preimage = format!(
                "collision:{}|domain:{}|left:{}|right:{}|law:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            CollisionCaseReport {
                id: item.0.clone(),
                domain: item.1.clone(),
                left: item.2.clone(),
                right: item.3.clone(),
                collision_hash: stable_hash_label("lyra.p01.identity.collision", &preimage),
            }
        })
        .collect();
    collision_case_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut receipt_reports: Vec<SemanticIdentityReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|path:{}|target:{}|status:{}",
                item.0, item.1, item.2, item.3
            );
            SemanticIdentityReceiptReport {
                id: item.0.clone(),
                path: item.1.clone(),
                target: item.2.clone(),
                receipt_hash: stable_hash_label("lyra.p01.identity.receipt", &preimage),
            }
        })
        .collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let stable_digest_count = digest_case_reports
        .iter()
        .filter(|item| item.expected_digest.starts_with("fnv1a128:"))
        .count();
    let mut suite_preimage = String::new();
    for item in &identity_reports {
        suite_preimage.push_str(&item.identity_hash);
        suite_preimage.push('\n');
    }
    for item in &digest_case_reports {
        suite_preimage.push_str(&item.case_hash);
        suite_preimage.push('\n');
    }
    for item in &collision_case_reports {
        suite_preimage.push_str(&item.collision_hash);
        suite_preimage.push('\n');
    }
    for item in &receipt_reports {
        suite_preimage.push_str(&item.receipt_hash);
        suite_preimage.push('\n');
    }
    let suite_hash = stable_hash_label("lyra.p01.identity.suite", &suite_preimage);
    SemanticIdentitySuiteReport {
        identity_count: identity_reports.len(),
        digest_case_count: digest_case_reports.len(),
        collision_case_count: collision_case_reports.len(),
        receipt_count: receipt_reports.len(),
        stable_digest_count,
        identity_reports,
        digest_case_reports,
        collision_case_reports,
        receipt_reports,
        suite_hash,
    }
}
