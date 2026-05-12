use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceCommandReport {
    pub id: String,
    pub binary: String,
    pub surface: String,
    pub receipt_count: usize,
    pub roles_hash: String,
    pub command_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceManifestReport {
    pub command_count: usize,
    pub workflow_count: usize,
    pub example_count: usize,
    pub proof_count: usize,
    pub manifest_hash: String,
    pub commands: Vec<InterfaceCommandReport>,
}

pub fn deterministic_interface_manifest_report(
    commands: &[(String, String, String, Vec<String>, Vec<String>)],
    workflow_count: usize,
    example_count: usize,
    proof_count: usize,
) -> InterfaceManifestReport {
    let mut normalized = commands.to_vec();
    normalized.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });

    let mut reports = Vec::new();
    let mut manifest_preimage = format!(
        "commands:{}|workflows:{}|examples:{}|proofs:{}",
        normalized.len(),
        workflow_count,
        example_count,
        proof_count
    );

    for (id, binary, surface, receipts, roles) in normalized {
        let mut sorted_receipts = receipts;
        sorted_receipts.sort();
        let mut sorted_roles = roles;
        sorted_roles.sort();
        let roles_hash = stable_hash_label("p00-interface-roles", &sorted_roles.join(","));
        let command_preimage = format!(
            "id:{}|binary:{}|surface:{}|receipts:{}|roles:{}",
            id,
            binary,
            surface,
            sorted_receipts.join(","),
            sorted_roles.join(",")
        );
        let command_hash = stable_hash_label("p00-interface-command", &command_preimage);
        manifest_preimage.push('|');
        manifest_preimage.push_str(&command_preimage);
        reports.push(InterfaceCommandReport {
            id,
            binary,
            surface,
            receipt_count: sorted_receipts.len(),
            roles_hash,
            command_hash,
        });
    }

    InterfaceManifestReport {
        command_count: reports.len(),
        workflow_count,
        example_count,
        proof_count,
        manifest_hash: stable_hash_label("p00-interface-manifest", &manifest_preimage),
        commands: reports,
    }
}
