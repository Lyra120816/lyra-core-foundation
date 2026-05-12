use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticPackageUnitReport {
    pub id: String,
    pub kind: String,
    pub owner_root: String,
    pub artifact_count: usize,
    pub command_count: usize,
    pub receipt_count: usize,
    pub package_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticReleaseBundleReport {
    pub id: String,
    pub order: String,
    pub package_count: usize,
    pub artifact_count: usize,
    pub receipt_count: usize,
    pub check_count: usize,
    pub bundle_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticDistributionCheckReport {
    pub id: String,
    pub scope: String,
    pub target: String,
    pub check_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticPackagingProofReport {
    pub id: String,
    pub scope: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticPackagingSuiteReport {
    pub package_count: usize,
    pub bundle_count: usize,
    pub check_count: usize,
    pub proof_count: usize,
    pub packages: Vec<SemanticPackageUnitReport>,
    pub bundles: Vec<SemanticReleaseBundleReport>,
    pub checks: Vec<SemanticDistributionCheckReport>,
    pub proofs: Vec<SemanticPackagingProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_packaging_suite_report(
    packages: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    bundles: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    checks: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    proofs: &[(
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
) -> SemanticPackagingSuiteReport {
    let mut package_reports: Vec<SemanticPackageUnitReport> = packages
        .iter()
        .map(|item| {
            let preimage = format!(
                "package:{}|kind:{}|owner:{}|artifacts:{}|commands:{}|receipts:{}|status:{}",
                item.0,
                item.1,
                item.2,
                sorted_join(&item.3),
                sorted_join(&item.4),
                sorted_join(&item.5),
                item.6
            );
            SemanticPackageUnitReport {
                id: item.0.clone(),
                kind: item.1.clone(),
                owner_root: item.2.clone(),
                artifact_count: sorted_count(&item.3),
                command_count: sorted_count(&item.4),
                receipt_count: sorted_count(&item.5),
                package_hash: stable_hash_label("lyra.p01.semantic_packaging.package", &preimage),
            }
        })
        .collect();
    package_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut bundle_reports: Vec<SemanticReleaseBundleReport> = bundles.iter().map(|item| {
        let preimage = format!("bundle:{}|order:{}|packages:{}|artifacts:{}|receipts:{}|checks:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), item.7);
        SemanticReleaseBundleReport { id: item.0.clone(), order: item.1.clone(), package_count: sorted_count(&item.2), artifact_count: sorted_count(&item.3), receipt_count: sorted_count(&item.4), check_count: sorted_count(&item.5), bundle_hash: stable_hash_label("lyra.p01.semantic_packaging.bundle", &preimage) }
    }).collect();
    bundle_reports.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));

    let mut check_reports: Vec<SemanticDistributionCheckReport> = checks
        .iter()
        .map(|item| {
            let preimage = format!(
                "check:{}|scope:{}|target:{}|requires:{}|forbids:{}|receipts:{}|status:{}",
                item.0,
                item.1,
                item.2,
                sorted_join(&item.3),
                sorted_join(&item.4),
                sorted_join(&item.5),
                item.6
            );
            SemanticDistributionCheckReport {
                id: item.0.clone(),
                scope: item.1.clone(),
                target: item.2.clone(),
                check_hash: stable_hash_label("lyra.p01.semantic_packaging.check", &preimage),
            }
        })
        .collect();
    check_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<SemanticPackagingProofReport> = proofs.iter().map(|item| {
        let preimage = format!("proof:{}|scope:{}|packages:{}|bundles:{}|checks:{}|receipts:{}|commands:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), sorted_join(&item.7), item.8);
        SemanticPackagingProofReport { id: item.0.clone(), scope: item.1.clone(), proof_hash: stable_hash_label("lyra.p01.semantic_packaging.proof", &preimage) }
    }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_rows = Vec::new();
    for item in &package_reports {
        suite_rows.push(format!("package:{}|hash:{}", item.id, item.package_hash));
    }
    for item in &bundle_reports {
        suite_rows.push(format!("bundle:{}|hash:{}", item.id, item.bundle_hash));
    }
    for item in &check_reports {
        suite_rows.push(format!("check:{}|hash:{}", item.id, item.check_hash));
    }
    for item in &proof_reports {
        suite_rows.push(format!("proof:{}|hash:{}", item.id, item.proof_hash));
    }
    suite_rows.sort();

    SemanticPackagingSuiteReport {
        package_count: package_reports.len(),
        bundle_count: bundle_reports.len(),
        check_count: check_reports.len(),
        proof_count: proof_reports.len(),
        packages: package_reports,
        bundles: bundle_reports,
        checks: check_reports,
        proofs: proof_reports,
        suite_hash: stable_hash_label("lyra.p01.semantic_packaging.suite", &suite_rows.join("\n")),
    }
}

fn sorted_join(items: &[String]) -> String {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy.join(",")
}
fn sorted_count(items: &[String]) -> usize {
    let mut copy = items.to_vec();
    copy.sort();
    copy.dedup();
    copy.len()
}
