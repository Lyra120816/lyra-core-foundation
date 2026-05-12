use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcosystemDocReport {
    pub id: String,
    pub audience: String,
    pub path: String,
    pub cover_count: usize,
    pub example_count: usize,
    pub receipt_count: usize,
    pub doc_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcosystemExampleReport {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub command_count: usize,
    pub proof_count: usize,
    pub rejection_count: usize,
    pub receipt_count: usize,
    pub example_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcosystemSuiteReport {
    pub doc_count: usize,
    pub example_count: usize,
    pub proof_count: usize,
    pub doc_reports: Vec<EcosystemDocReport>,
    pub example_reports: Vec<EcosystemExampleReport>,
    pub suite_hash: String,
}

pub fn deterministic_ecosystem_suite_report(
    docs: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )],
    examples: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    )],
    proof_count: usize,
) -> EcosystemSuiteReport {
    let mut sorted_docs = docs.to_vec();
    sorted_docs.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });
    let mut sorted_examples = examples.to_vec();
    sorted_examples.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });

    let mut doc_reports = Vec::new();
    let mut example_reports = Vec::new();
    let mut preimage = format!(
        "docs:{}|examples:{}|proofs:{}",
        sorted_docs.len(),
        sorted_examples.len(),
        proof_count
    );

    for (id, audience, path, mut covers, mut examples, mut receipts) in sorted_docs {
        covers.sort();
        examples.sort();
        receipts.sort();
        let doc_preimage = format!(
            "doc:{}|audience:{}|path:{}|covers:{}|examples:{}|receipts:{}",
            id,
            audience,
            path,
            covers.join(","),
            examples.join(","),
            receipts.join(",")
        );
        let doc_hash = stable_hash_label("lyra.p00.ecosystem.doc", &doc_preimage);
        preimage.push('|');
        preimage.push_str(&doc_preimage);
        doc_reports.push(EcosystemDocReport {
            id,
            audience,
            path,
            cover_count: covers.len(),
            example_count: examples.len(),
            receipt_count: receipts.len(),
            doc_hash,
        });
    }

    for (id, kind, path, mut commands, mut proofs, mut receipts, mut rejects) in sorted_examples {
        commands.sort();
        proofs.sort();
        receipts.sort();
        rejects.sort();
        let example_preimage = format!(
            "example:{}|kind:{}|path:{}|commands:{}|proofs:{}|receipts:{}|rejects:{}",
            id,
            kind,
            path,
            commands.join(","),
            proofs.join(","),
            receipts.join(","),
            rejects.join(",")
        );
        let example_hash = stable_hash_label("lyra.p00.ecosystem.example", &example_preimage);
        preimage.push('|');
        preimage.push_str(&example_preimage);
        example_reports.push(EcosystemExampleReport {
            id,
            kind,
            path,
            command_count: commands.len(),
            proof_count: proofs.len(),
            rejection_count: rejects.len(),
            receipt_count: receipts.len(),
            example_hash,
        });
    }

    EcosystemSuiteReport {
        doc_count: doc_reports.len(),
        example_count: example_reports.len(),
        proof_count,
        doc_reports,
        example_reports,
        suite_hash: stable_hash_label("lyra.p00.ecosystem.suite", &preimage),
    }
}
