use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEcosystemDocReport {
    pub id: String,
    pub audience: String,
    pub path: String,
    pub cover_count: usize,
    pub example_count: usize,
    pub receipt_count: usize,
    pub doc_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEcosystemExampleReport {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub command_count: usize,
    pub proof_count: usize,
    pub receipt_count: usize,
    pub rejection_count: usize,
    pub example_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEcosystemProofReport {
    pub id: String,
    pub scope: String,
    pub doc_count: usize,
    pub example_count: usize,
    pub receipt_count: usize,
    pub command_count: usize,
    pub forbid_count: usize,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapEcosystemSuiteReport {
    pub doc_count: usize,
    pub example_count: usize,
    pub proof_count: usize,
    pub doc_reports: Vec<BootstrapEcosystemDocReport>,
    pub example_reports: Vec<BootstrapEcosystemExampleReport>,
    pub proof_reports: Vec<BootstrapEcosystemProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_bootstrap_ecosystem_suite_report(
    docs: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    examples: &[(
        String,
        String,
        String,
        Vec<String>,
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
        String,
    )],
) -> BootstrapEcosystemSuiteReport {
    let mut doc_reports: Vec<_> = docs
        .iter()
        .map(|item| {
            let preimage = format!(
                "doc:{}|audience:{}|path:{}|covers:{}|examples:{}|receipts:{}|status:{}",
                item.0,
                item.1,
                item.2,
                sorted_join(&item.3),
                sorted_join(&item.4),
                sorted_join(&item.5),
                item.6
            );
            BootstrapEcosystemDocReport {
                id: item.0.clone(),
                audience: item.1.clone(),
                path: item.2.clone(),
                cover_count: sorted_count(&item.3),
                example_count: sorted_count(&item.4),
                receipt_count: sorted_count(&item.5),
                doc_hash: stable_hash_label("lyra.p02.bootstrap_ecosystem.doc", &preimage),
            }
        })
        .collect();
    doc_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut example_reports: Vec<_> = examples
        .iter()
        .map(|item| {
            let preimage = format!(
                "example:{}|kind:{}|path:{}|commands:{}|proofs:{}|receipts:{}|rejects:{}|status:{}",
                item.0,
                item.1,
                item.2,
                sorted_join(&item.3),
                sorted_join(&item.4),
                sorted_join(&item.5),
                sorted_join(&item.6),
                item.7
            );
            BootstrapEcosystemExampleReport {
                id: item.0.clone(),
                kind: item.1.clone(),
                path: item.2.clone(),
                command_count: sorted_count(&item.3),
                proof_count: sorted_count(&item.4),
                receipt_count: sorted_count(&item.5),
                rejection_count: sorted_count(&item.6),
                example_hash: stable_hash_label("lyra.p02.bootstrap_ecosystem.example", &preimage),
            }
        })
        .collect();
    example_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<_> = proofs
        .iter()
        .map(|item| {
            let preimage = format!(
            "proof:{}|scope:{}|docs:{}|examples:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
            item.0,
            item.1,
            sorted_join(&item.2),
            sorted_join(&item.3),
            sorted_join(&item.4),
            sorted_join(&item.5),
            sorted_join(&item.6),
            item.7
        );
            BootstrapEcosystemProofReport {
                id: item.0.clone(),
                scope: item.1.clone(),
                doc_count: sorted_count(&item.2),
                example_count: sorted_count(&item.3),
                receipt_count: sorted_count(&item.4),
                command_count: sorted_count(&item.5),
                forbid_count: sorted_count(&item.6),
                proof_hash: stable_hash_label("lyra.p02.bootstrap_ecosystem.proof", &preimage),
            }
        })
        .collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_rows = Vec::new();
    for item in &doc_reports {
        suite_rows.push(format!("doc:{}|hash:{}", item.id, item.doc_hash));
    }
    for item in &example_reports {
        suite_rows.push(format!("example:{}|hash:{}", item.id, item.example_hash));
    }
    for item in &proof_reports {
        suite_rows.push(format!("proof:{}|hash:{}", item.id, item.proof_hash));
    }
    suite_rows.sort();

    BootstrapEcosystemSuiteReport {
        doc_count: doc_reports.len(),
        example_count: example_reports.len(),
        proof_count: proof_reports.len(),
        doc_reports,
        example_reports,
        proof_reports,
        suite_hash: stable_hash_label("lyra.p02.bootstrap_ecosystem.suite", &suite_rows.join("\n")),
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
