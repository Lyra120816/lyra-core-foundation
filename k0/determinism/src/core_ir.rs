use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrFormReport {
    pub id: String,
    pub medium: String,
    pub owner_root: String,
    pub version: String,
    pub status: String,
    pub form_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrVersionReport {
    pub id: String,
    pub major: String,
    pub minor: String,
    pub stability: String,
    pub version_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrUpgradeReport {
    pub id: String,
    pub from_version: String,
    pub to_version: String,
    pub compatibility: String,
    pub upgrade_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrParityReport {
    pub id: String,
    pub text_form: String,
    pub binary_form: String,
    pub atom: String,
    pub parity_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreIrSuiteReport {
    pub form_count: usize,
    pub version_count: usize,
    pub upgrade_count: usize,
    pub parity_count: usize,
    pub receipt_count: usize,
    pub text_form_count: usize,
    pub binary_form_count: usize,
    pub admitted_count: usize,
    pub form_reports: Vec<CoreIrFormReport>,
    pub version_reports: Vec<CoreIrVersionReport>,
    pub upgrade_reports: Vec<CoreIrUpgradeReport>,
    pub parity_reports: Vec<CoreIrParityReport>,
    pub receipt_reports: Vec<CoreIrReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_core_ir_suite_report(
    forms: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    versions: &[(String, String, String, String, String, String)],
    upgrades: &[(String, String, String, String, String, String)],
    parities: &[(String, String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> CoreIrSuiteReport {
    let mut sorted_forms = forms.to_vec();
    sorted_forms.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_versions = versions.to_vec();
    sorted_versions.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_upgrades = upgrades.to_vec();
    sorted_upgrades.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_parities = parities.to_vec();
    sorted_parities.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_receipts = receipts.to_vec();
    sorted_receipts.sort_by(|left, right| left.0.cmp(&right.0));

    let mut text_form_count = 0usize;
    let mut binary_form_count = 0usize;
    let mut admitted_count = 0usize;
    let mut form_reports = Vec::new();
    let mut version_reports = Vec::new();
    let mut upgrade_reports = Vec::new();
    let mut parity_reports = Vec::new();
    let mut receipt_reports = Vec::new();
    let mut preimage = format!(
        "forms:{}|versions:{}|upgrades:{}|parities:{}|receipts:{}",
        sorted_forms.len(),
        sorted_versions.len(),
        sorted_upgrades.len(),
        sorted_parities.len(),
        sorted_receipts.len()
    );

    for (id, medium, owner_root, version, header, extension, encoding, canonicalization, status) in
        sorted_forms
    {
        if medium == "text" {
            text_form_count += 1;
        }
        if medium == "binary" {
            binary_form_count += 1;
        }
        if status == "admitted" {
            admitted_count += 1;
        }
        let form_preimage = format!("form:{id}|medium:{medium}|owner:{owner_root}|version:{version}|header:{header}|extension:{extension}|encoding:{encoding}|canonicalization:{canonicalization}|status:{status}");
        let form_hash = stable_hash_label("lyra.p01.core_ir.form", &form_preimage);
        preimage.push('|');
        preimage.push_str(&form_preimage);
        form_reports.push(CoreIrFormReport {
            id,
            medium,
            owner_root,
            version,
            status,
            form_hash,
        });
    }

    for (id, major, minor, stability, upgrade_policy, status) in sorted_versions {
        let version_preimage = format!("version:{id}|major:{major}|minor:{minor}|stability:{stability}|upgrade_policy:{upgrade_policy}|status:{status}");
        let version_hash = stable_hash_label("lyra.p01.core_ir.version", &version_preimage);
        preimage.push('|');
        preimage.push_str(&version_preimage);
        version_reports.push(CoreIrVersionReport {
            id,
            major,
            minor,
            stability,
            version_hash,
        });
    }

    for (id, from_version, to_version, law, compatibility, status) in sorted_upgrades {
        let upgrade_preimage = format!("upgrade:{id}|from:{from_version}|to:{to_version}|law:{law}|compatibility:{compatibility}|status:{status}");
        let upgrade_hash = stable_hash_label("lyra.p01.core_ir.upgrade", &upgrade_preimage);
        preimage.push('|');
        preimage.push_str(&upgrade_preimage);
        upgrade_reports.push(CoreIrUpgradeReport {
            id,
            from_version,
            to_version,
            compatibility,
            upgrade_hash,
        });
    }

    for (id, text_form, binary_form, fixture, atom, round_trip, status) in sorted_parities {
        let parity_preimage = format!("parity:{id}|text:{text_form}|binary:{binary_form}|fixture:{fixture}|atom:{atom}|round_trip:{round_trip}|status:{status}");
        let parity_hash = stable_hash_label("lyra.p01.core_ir.parity", &parity_preimage);
        preimage.push('|');
        preimage.push_str(&parity_preimage);
        parity_reports.push(CoreIrParityReport {
            id,
            text_form,
            binary_form,
            atom,
            parity_hash,
        });
    }

    for (id, path, target, status) in sorted_receipts {
        let receipt_preimage = format!("receipt:{id}|path:{path}|target:{target}|status:{status}");
        let receipt_hash = stable_hash_label("lyra.p01.core_ir.receipt", &receipt_preimage);
        preimage.push('|');
        preimage.push_str(&receipt_preimage);
        receipt_reports.push(CoreIrReceiptReport {
            id,
            path,
            target,
            receipt_hash,
        });
    }

    let suite_hash = stable_hash_label("lyra.p01.core_ir.suite", &preimage);
    CoreIrSuiteReport {
        form_count: form_reports.len(),
        version_count: version_reports.len(),
        upgrade_count: upgrade_reports.len(),
        parity_count: parity_reports.len(),
        receipt_count: receipt_reports.len(),
        text_form_count,
        binary_form_count,
        admitted_count,
        form_reports,
        version_reports,
        upgrade_reports,
        parity_reports,
        receipt_reports,
        suite_hash,
    }
}
