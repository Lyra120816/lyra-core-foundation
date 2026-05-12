use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPackageUnitReport {
    pub id: String,
    pub kind: String,
    pub owner_root: String,
    pub artifact_count: usize,
    pub command_count: usize,
    pub receipt_count: usize,
    pub package_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapReleaseBundleReport {
    pub id: String,
    pub order: String,
    pub package_count: usize,
    pub artifact_count: usize,
    pub receipt_count: usize,
    pub check_count: usize,
    pub bundle_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapDistributionCheckReport {
    pub id: String,
    pub scope: String,
    pub target: String,
    pub check_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPackagingProofReport {
    pub id: String,
    pub scope: String,
    pub proof_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPackagingSuiteReport {
    pub package_count: usize,
    pub bundle_count: usize,
    pub check_count: usize,
    pub proof_count: usize,
    pub packages: Vec<BootstrapPackageUnitReport>,
    pub bundles: Vec<BootstrapReleaseBundleReport>,
    pub checks: Vec<BootstrapDistributionCheckReport>,
    pub proofs: Vec<BootstrapPackagingProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_bootstrap_packaging_suite_report(
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
) -> BootstrapPackagingSuiteReport {
    let mut package_reports: Vec<BootstrapPackageUnitReport> = packages
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
            BootstrapPackageUnitReport {
                id: item.0.clone(),
                kind: item.1.clone(),
                owner_root: item.2.clone(),
                artifact_count: sorted_count(&item.3),
                command_count: sorted_count(&item.4),
                receipt_count: sorted_count(&item.5),
                package_hash: stable_hash_label("lyra.p02.bootstrap_packaging.package", &preimage),
            }
        })
        .collect();
    package_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut bundle_reports: Vec<BootstrapReleaseBundleReport> = bundles.iter().map(|item| {
        let preimage = format!("bundle:{}|order:{}|packages:{}|artifacts:{}|receipts:{}|checks:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), item.7);
        BootstrapReleaseBundleReport { id: item.0.clone(), order: item.1.clone(), package_count: sorted_count(&item.2), artifact_count: sorted_count(&item.3), receipt_count: sorted_count(&item.4), check_count: sorted_count(&item.5), bundle_hash: stable_hash_label("lyra.p02.bootstrap_packaging.bundle", &preimage) }
    }).collect();
    bundle_reports.sort_by(|left, right| left.order.cmp(&right.order).then(left.id.cmp(&right.id)));

    let mut check_reports: Vec<BootstrapDistributionCheckReport> = checks
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
            BootstrapDistributionCheckReport {
                id: item.0.clone(),
                scope: item.1.clone(),
                target: item.2.clone(),
                check_hash: stable_hash_label("lyra.p02.bootstrap_packaging.check", &preimage),
            }
        })
        .collect();
    check_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<BootstrapPackagingProofReport> = proofs.iter().map(|item| {
        let preimage = format!("proof:{}|scope:{}|packages:{}|bundles:{}|checks:{}|receipts:{}|commands:{}|forbids:{}|status:{}", item.0, item.1, sorted_join(&item.2), sorted_join(&item.3), sorted_join(&item.4), sorted_join(&item.5), sorted_join(&item.6), sorted_join(&item.7), item.8);
        BootstrapPackagingProofReport { id: item.0.clone(), scope: item.1.clone(), proof_hash: stable_hash_label("lyra.p02.bootstrap_packaging.proof", &preimage) }
    }).collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_lines = Vec::new();
    for item in &package_reports {
        suite_lines.push(format!("package:{}|hash:{}", item.id, item.package_hash));
    }
    for item in &bundle_reports {
        suite_lines.push(format!("bundle:{}|hash:{}", item.id, item.bundle_hash));
    }
    for item in &check_reports {
        suite_lines.push(format!("check:{}|hash:{}", item.id, item.check_hash));
    }
    for item in &proof_reports {
        suite_lines.push(format!("proof:{}|hash:{}", item.id, item.proof_hash));
    }
    suite_lines.sort();
    BootstrapPackagingSuiteReport {
        package_count: package_reports.len(),
        bundle_count: bundle_reports.len(),
        check_count: check_reports.len(),
        proof_count: proof_reports.len(),
        packages: package_reports,
        bundles: bundle_reports,
        checks: check_reports,
        proofs: proof_reports,
        suite_hash: stable_hash_label(
            "lyra.p02.bootstrap_packaging.suite",
            &suite_lines.join("\n"),
        ),
    }
}

fn sorted_join(values: &[String]) -> String {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values.join(",")
}
fn sorted_count(values: &[String]) -> usize {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values.len()
}
