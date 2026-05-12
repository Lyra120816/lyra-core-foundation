use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEconomicsFrameReport {
    pub id: String,
    pub frame_kind: String,
    pub path: String,
    pub cover_count: usize,
    pub output_count: usize,
    pub receipt_count: usize,
    pub frame_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticPublicInterestOutputReport {
    pub id: String,
    pub output_kind: String,
    pub path: String,
    pub constituency_count: usize,
    pub command_count: usize,
    pub proof_count: usize,
    pub rejection_count: usize,
    pub receipt_count: usize,
    pub output_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEconomicsProofReport {
    pub id: String,
    pub scope: String,
    pub frame_count: usize,
    pub output_count: usize,
    pub receipt_count: usize,
    pub command_count: usize,
    pub forbid_count: usize,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticEconomicsSuiteReport {
    pub frame_count: usize,
    pub output_count: usize,
    pub proof_count: usize,
    pub frame_reports: Vec<SemanticEconomicsFrameReport>,
    pub output_reports: Vec<SemanticPublicInterestOutputReport>,
    pub proof_reports: Vec<SemanticEconomicsProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_economics_suite_report(
    frames: &[(
        String,
        String,
        String,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        String,
    )],
    outputs: &[(
        String,
        String,
        String,
        Vec<String>,
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
) -> SemanticEconomicsSuiteReport {
    let mut frame_reports: Vec<_> = frames
        .iter()
        .map(|item| {
            let preimage = format!(
                "frame:{}|kind:{}|path:{}|covers:{}|outputs:{}|receipts:{}|status:{}",
                item.0,
                item.1,
                item.2,
                sorted_join(&item.3),
                sorted_join(&item.4),
                sorted_join(&item.5),
                item.6
            );
            SemanticEconomicsFrameReport {
                id: item.0.clone(),
                frame_kind: item.1.clone(),
                path: item.2.clone(),
                cover_count: sorted_count(&item.3),
                output_count: sorted_count(&item.4),
                receipt_count: sorted_count(&item.5),
                frame_hash: stable_hash_label("lyra.p01.semantic_economics.frame", &preimage),
            }
        })
        .collect();
    frame_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut output_reports: Vec<_> = outputs.iter().map(|item| {
        let preimage = format!(
            "output:{}|kind:{}|path:{}|constituencies:{}|commands:{}|proofs:{}|receipts:{}|rejects:{}|status:{}",
            item.0,
            item.1,
            item.2,
            sorted_join(&item.3),
            sorted_join(&item.4),
            sorted_join(&item.5),
            sorted_join(&item.6),
            sorted_join(&item.7),
            item.8
        );
        SemanticPublicInterestOutputReport {
            id: item.0.clone(),
            output_kind: item.1.clone(),
            path: item.2.clone(),
            constituency_count: sorted_count(&item.3),
            command_count: sorted_count(&item.4),
            proof_count: sorted_count(&item.5),
            receipt_count: sorted_count(&item.6),
            rejection_count: sorted_count(&item.7),
            output_hash: stable_hash_label("lyra.p01.semantic_economics.output", &preimage),
        }
    }).collect();
    output_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<_> = proofs
        .iter()
        .map(|item| {
            let preimage = format!(
            "proof:{}|scope:{}|frames:{}|outputs:{}|receipts:{}|commands:{}|forbids:{}|status:{}",
            item.0,
            item.1,
            sorted_join(&item.2),
            sorted_join(&item.3),
            sorted_join(&item.4),
            sorted_join(&item.5),
            sorted_join(&item.6),
            item.7
        );
            SemanticEconomicsProofReport {
                id: item.0.clone(),
                scope: item.1.clone(),
                frame_count: sorted_count(&item.2),
                output_count: sorted_count(&item.3),
                receipt_count: sorted_count(&item.4),
                command_count: sorted_count(&item.5),
                forbid_count: sorted_count(&item.6),
                proof_hash: stable_hash_label("lyra.p01.semantic_economics.proof", &preimage),
            }
        })
        .collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_rows = Vec::new();
    for item in &frame_reports {
        suite_rows.push(format!("frame:{}|hash:{}", item.id, item.frame_hash));
    }
    for item in &output_reports {
        suite_rows.push(format!("output:{}|hash:{}", item.id, item.output_hash));
    }
    for item in &proof_reports {
        suite_rows.push(format!("proof:{}|hash:{}", item.id, item.proof_hash));
    }
    suite_rows.sort();

    SemanticEconomicsSuiteReport {
        frame_count: frame_reports.len(),
        output_count: output_reports.len(),
        proof_count: proof_reports.len(),
        frame_reports,
        output_reports,
        proof_reports,
        suite_hash: stable_hash_label("lyra.p01.semantic_economics.suite", &suite_rows.join("\n")),
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
