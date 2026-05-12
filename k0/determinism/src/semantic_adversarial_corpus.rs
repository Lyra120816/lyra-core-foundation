use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticCollisionProbeReport {
    pub id: String,
    pub target_family: String,
    pub expected_error: String,
    pub probe_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAmbiguityProbeReport {
    pub id: String,
    pub target_family: String,
    pub expected_error: String,
    pub probe_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticMalformedObjectReport {
    pub id: String,
    pub target_family: String,
    pub expected_error: String,
    pub object_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAdversarialHarnessReport {
    pub id: String,
    pub validator: String,
    pub coverage: String,
    pub harness_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAdversarialReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAdversarialCorpusSuiteReport {
    pub collision_probe_count: usize,
    pub ambiguity_probe_count: usize,
    pub malformed_object_count: usize,
    pub harness_count: usize,
    pub receipt_count: usize,
    pub collision_reports: Vec<SemanticCollisionProbeReport>,
    pub ambiguity_reports: Vec<SemanticAmbiguityProbeReport>,
    pub malformed_reports: Vec<SemanticMalformedObjectReport>,
    pub harness_reports: Vec<SemanticAdversarialHarnessReport>,
    pub receipt_reports: Vec<SemanticAdversarialReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_adversarial_corpus_suite_report(
    collisions: &[(
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
    ambiguities: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    malformed: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    harnesses: &[(String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> SemanticAdversarialCorpusSuiteReport {
    let mut collision_reports: Vec<SemanticCollisionProbeReport> = collisions.iter().map(|item| {
        let preimage = format!("collision:{}|target_family:{}|left_ref:{}|right_ref:{}|domain:{}|guard:{}|expected_error:{}|fixture:{}|status:{}", item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7, item.8);
        SemanticCollisionProbeReport { id: item.0.clone(), target_family: item.1.clone(), expected_error: item.6.clone(), probe_hash: stable_hash_label("lyra.p01.semantic_adversarial_corpus.collision", &preimage) }
    }).collect();
    collision_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut ambiguity_reports: Vec<SemanticAmbiguityProbeReport> = ambiguities.iter().map(|item| {
        let preimage = format!("ambiguity:{}|target_family:{}|surface:{}|resolution:{}|guard:{}|expected_error:{}|fixture:{}|status:{}", item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7);
        SemanticAmbiguityProbeReport { id: item.0.clone(), target_family: item.1.clone(), expected_error: item.5.clone(), probe_hash: stable_hash_label("lyra.p01.semantic_adversarial_corpus.ambiguity", &preimage) }
    }).collect();
    ambiguity_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut malformed_reports: Vec<SemanticMalformedObjectReport> = malformed.iter().map(|item| {
        let preimage = format!("malformed:{}|target_family:{}|object_ref:{}|field:{}|law:{}|expected_error:{}|fixture:{}|status:{}", item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7);
        SemanticMalformedObjectReport { id: item.0.clone(), target_family: item.1.clone(), expected_error: item.5.clone(), object_hash: stable_hash_label("lyra.p01.semantic_adversarial_corpus.malformed", &preimage) }
    }).collect();
    malformed_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut harness_reports: Vec<SemanticAdversarialHarnessReport> = harnesses
        .iter()
        .map(|item| {
            let preimage = format!(
                "harness:{}|validator:{}|case_ids:{}|coverage:{}|receipt_ref:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            SemanticAdversarialHarnessReport {
                id: item.0.clone(),
                validator: item.1.clone(),
                coverage: item.3.clone(),
                harness_hash: stable_hash_label(
                    "lyra.p01.semantic_adversarial_corpus.harness",
                    &preimage,
                ),
            }
        })
        .collect();
    harness_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut receipt_reports: Vec<SemanticAdversarialReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|path:{}|target:{}|status:{}",
                item.0, item.1, item.2, item.3
            );
            SemanticAdversarialReceiptReport {
                id: item.0.clone(),
                path: item.1.clone(),
                target: item.2.clone(),
                receipt_hash: stable_hash_label(
                    "lyra.p01.semantic_adversarial_corpus.receipt",
                    &preimage,
                ),
            }
        })
        .collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_lines = Vec::new();
    for item in &collision_reports {
        suite_lines.push(format!("collision:{}|{}", item.id, item.probe_hash));
    }
    for item in &ambiguity_reports {
        suite_lines.push(format!("ambiguity:{}|{}", item.id, item.probe_hash));
    }
    for item in &malformed_reports {
        suite_lines.push(format!("malformed:{}|{}", item.id, item.object_hash));
    }
    for item in &harness_reports {
        suite_lines.push(format!("harness:{}|{}", item.id, item.harness_hash));
    }
    for item in &receipt_reports {
        suite_lines.push(format!("receipt:{}|{}", item.id, item.receipt_hash));
    }
    suite_lines.sort();

    SemanticAdversarialCorpusSuiteReport {
        collision_probe_count: collision_reports.len(),
        ambiguity_probe_count: ambiguity_reports.len(),
        malformed_object_count: malformed_reports.len(),
        harness_count: harness_reports.len(),
        receipt_count: receipt_reports.len(),
        collision_reports,
        ambiguity_reports,
        malformed_reports,
        harness_reports,
        receipt_reports,
        suite_hash: stable_hash_label(
            "lyra.p01.semantic_adversarial_corpus.suite",
            &suite_lines.join("\n"),
        ),
    }
}
