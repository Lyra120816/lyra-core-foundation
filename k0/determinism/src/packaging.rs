use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageUnitReport {
    pub id: String,
    pub kind: String,
    pub artifact_count: usize,
    pub command_count: usize,
    pub receipt_count: usize,
    pub package_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseBundleReport {
    pub id: String,
    pub order: String,
    pub package_count: usize,
    pub artifact_count: usize,
    pub receipt_count: usize,
    pub bundle_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackagingSuiteReport {
    pub package_count: usize,
    pub bundle_count: usize,
    pub check_count: usize,
    pub proof_count: usize,
    pub package_reports: Vec<PackageUnitReport>,
    pub bundle_reports: Vec<ReleaseBundleReport>,
    pub suite_hash: String,
}

pub fn deterministic_packaging_suite_report(
    packages: &[(String, String, Vec<String>, Vec<String>, Vec<String>)],
    bundles: &[(String, String, Vec<String>, Vec<String>, Vec<String>)],
    check_count: usize,
    proof_count: usize,
) -> PackagingSuiteReport {
    let mut sorted_packages = packages.to_vec();
    sorted_packages.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_bundles = bundles.to_vec();
    sorted_bundles.sort_by(|left, right| left.1.cmp(&right.1).then(left.0.cmp(&right.0)));

    let mut package_reports = Vec::new();
    let mut bundle_reports = Vec::new();
    let mut preimage = format!(
        "packages:{}|bundles:{}|checks:{}|proofs:{}",
        sorted_packages.len(),
        sorted_bundles.len(),
        check_count,
        proof_count
    );

    for (id, kind, mut artifacts, mut commands, mut receipts) in sorted_packages {
        artifacts.sort();
        commands.sort();
        receipts.sort();
        let package_preimage = format!(
            "package:{}|kind:{}|artifacts:{}|commands:{}|receipts:{}",
            id,
            kind,
            artifacts.join(","),
            commands.join(","),
            receipts.join(",")
        );
        let package_hash = stable_hash_label("lyra.p00.packaging.package", &package_preimage);
        preimage.push('|');
        preimage.push_str(&package_preimage);
        package_reports.push(PackageUnitReport {
            id,
            kind,
            artifact_count: artifacts.len(),
            command_count: commands.len(),
            receipt_count: receipts.len(),
            package_hash,
        });
    }

    for (id, order, mut packages, mut artifacts, mut receipts) in sorted_bundles {
        packages.sort();
        artifacts.sort();
        receipts.sort();
        let bundle_preimage = format!(
            "bundle:{}|order:{}|packages:{}|artifacts:{}|receipts:{}",
            id,
            order,
            packages.join(","),
            artifacts.join(","),
            receipts.join(",")
        );
        let bundle_hash = stable_hash_label("lyra.p00.packaging.bundle", &bundle_preimage);
        preimage.push('|');
        preimage.push_str(&bundle_preimage);
        bundle_reports.push(ReleaseBundleReport {
            id,
            order,
            package_count: packages.len(),
            artifact_count: artifacts.len(),
            receipt_count: receipts.len(),
            bundle_hash,
        });
    }

    PackagingSuiteReport {
        package_count: package_reports.len(),
        bundle_count: bundle_reports.len(),
        check_count,
        proof_count,
        package_reports,
        bundle_reports,
        suite_hash: stable_hash_label("lyra.p00.packaging.suite", &preimage),
    }
}
