use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReport {
    pub id: String,
    pub kind: String,
    pub owner_root: String,
    pub canonical_name: String,
    pub status: String,
    pub atom_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomFamilyReport {
    pub id: String,
    pub member_count: usize,
    pub phase: String,
    pub work_package: String,
    pub status: String,
    pub family_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub status: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomCoreReport {
    pub atom_count: usize,
    pub family_count: usize,
    pub receipt_count: usize,
    pub lyralang_owned_count: usize,
    pub interface_owned_count: usize,
    pub k0_owned_count: usize,
    pub admitted_count: usize,
    pub atom_reports: Vec<SemanticAtomReport>,
    pub family_reports: Vec<SemanticAtomFamilyReport>,
    pub receipt_reports: Vec<SemanticAtomReceiptReport>,
    pub core_hash: String,
}

pub fn deterministic_semantic_atom_core_report(
    atoms: &[(
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
    families: &[(String, Vec<String>, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> SemanticAtomCoreReport {
    let mut sorted_atoms = atoms.to_vec();
    sorted_atoms.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    let mut sorted_families = families.to_vec();
    sorted_families.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_receipts = receipts.to_vec();
    sorted_receipts.sort_by(|left, right| left.0.cmp(&right.0));

    let mut atom_reports = Vec::new();
    let mut family_reports = Vec::new();
    let mut receipt_reports = Vec::new();
    let mut lyralang_owned_count = 0usize;
    let mut interface_owned_count = 0usize;
    let mut k0_owned_count = 0usize;
    let mut admitted_count = 0usize;
    let mut preimage = format!(
        "atoms:{}|families:{}|receipts:{}",
        sorted_atoms.len(),
        sorted_families.len(),
        sorted_receipts.len()
    );

    for (
        id,
        kind,
        owner_root,
        canonical_name,
        identity_law,
        equality_law,
        normalization_law,
        serialization_law,
        status,
    ) in sorted_atoms
    {
        if owner_root == "lyralang" {
            lyralang_owned_count += 1;
        }
        if owner_root == "interfaces" {
            interface_owned_count += 1;
        }
        if owner_root == "k0" {
            k0_owned_count += 1;
        }
        if status == "admitted" {
            admitted_count += 1;
        }
        let atom_preimage = format!(
            "atom:{id}|kind:{kind}|owner:{owner_root}|canonical:{canonical_name}|identity:{identity_law}|equality:{equality_law}|normalization:{normalization_law}|serialization:{serialization_law}|status:{status}"
        );
        let atom_hash = stable_hash_label("lyra.p01.semantic_atom.atom", &atom_preimage);
        preimage.push('|');
        preimage.push_str(&atom_preimage);
        atom_reports.push(SemanticAtomReport {
            id,
            kind,
            owner_root,
            canonical_name,
            status,
            atom_hash,
        });
    }

    for (id, mut members, phase, work_package, status) in sorted_families {
        members.sort();
        let family_preimage = format!(
            "family:{}|members:{}|phase:{}|work_package:{}|status:{}",
            id,
            members.join(","),
            phase,
            work_package,
            status
        );
        let family_hash = stable_hash_label("lyra.p01.semantic_atom.family", &family_preimage);
        preimage.push('|');
        preimage.push_str(&family_preimage);
        family_reports.push(SemanticAtomFamilyReport {
            id,
            member_count: members.len(),
            phase,
            work_package,
            status,
            family_hash,
        });
    }

    for (id, path, target, status) in sorted_receipts {
        let receipt_preimage = format!("receipt:{id}|path:{path}|target:{target}|status:{status}");
        let receipt_hash = stable_hash_label("lyra.p01.semantic_atom.receipt", &receipt_preimage);
        preimage.push('|');
        preimage.push_str(&receipt_preimage);
        receipt_reports.push(SemanticAtomReceiptReport {
            id,
            path,
            target,
            status,
            receipt_hash,
        });
    }

    SemanticAtomCoreReport {
        atom_count: atom_reports.len(),
        family_count: family_reports.len(),
        receipt_count: receipt_reports.len(),
        lyralang_owned_count,
        interface_owned_count,
        k0_owned_count,
        admitted_count,
        atom_reports,
        family_reports,
        receipt_reports,
        core_hash: stable_hash_label("lyra.p01.semantic_atom.core", &preimage),
    }
}
