use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEqualityRuleReport {
    pub id: String,
    pub domain: String,
    pub relation: String,
    pub law: String,
    pub rule_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEquivalenceClassReport {
    pub id: String,
    pub canonical: String,
    pub normalizer: String,
    pub class_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicNormalizationCaseReport {
    pub id: String,
    pub input: String,
    pub output: String,
    pub digest: String,
    pub case_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicSubstitutionCaseReport {
    pub id: String,
    pub target: String,
    pub expected: String,
    pub digest: String,
    pub case_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEqualityReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicEqualitySuiteReport {
    pub equality_rule_count: usize,
    pub equivalence_class_count: usize,
    pub normalization_count: usize,
    pub substitution_count: usize,
    pub receipt_count: usize,
    pub equality_rule_reports: Vec<SymbolicEqualityRuleReport>,
    pub equivalence_class_reports: Vec<SymbolicEquivalenceClassReport>,
    pub normalization_reports: Vec<SymbolicNormalizationCaseReport>,
    pub substitution_reports: Vec<SymbolicSubstitutionCaseReport>,
    pub receipt_reports: Vec<SymbolicEqualityReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_symbolic_equality_suite_report(
    equality_rules: &[(String, String, String, String, String)],
    equivalence_classes: &[(String, String, String, String, String)],
    normalizations: &[(String, String, String, String, String, String)],
    substitutions: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    receipts: &[(String, String, String, String)],
) -> SymbolicEqualitySuiteReport {
    let mut equality_rule_reports: Vec<SymbolicEqualityRuleReport> = equality_rules
        .iter()
        .map(|item| {
            let preimage = format!(
                "equality_rule:{}|domain:{}|relation:{}|law:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            SymbolicEqualityRuleReport {
                id: item.0.clone(),
                domain: item.1.clone(),
                relation: item.2.clone(),
                law: item.3.clone(),
                rule_hash: stable_hash_label("lyra.p01.symbolic_equality.equality_rule", &preimage),
            }
        })
        .collect();
    equality_rule_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut equivalence_class_reports: Vec<SymbolicEquivalenceClassReport> = equivalence_classes
        .iter()
        .map(|item| {
            let preimage = format!(
                "equivalence_class:{}|members:{}|canonical:{}|normalizer:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            SymbolicEquivalenceClassReport {
                id: item.0.clone(),
                canonical: item.2.clone(),
                normalizer: item.3.clone(),
                class_hash: stable_hash_label(
                    "lyra.p01.symbolic_equality.equivalence_class",
                    &preimage,
                ),
            }
        })
        .collect();
    equivalence_class_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut normalization_reports: Vec<SymbolicNormalizationCaseReport> = normalizations
        .iter()
        .map(|item| {
            let preimage = format!(
                "normalization:{}|input:{}|output:{}|law:{}|digest:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            SymbolicNormalizationCaseReport {
                id: item.0.clone(),
                input: item.1.clone(),
                output: item.2.clone(),
                digest: item.4.clone(),
                case_hash: stable_hash_label("lyra.p01.symbolic_equality.normalization", &preimage),
            }
        })
        .collect();
    normalization_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut substitution_reports: Vec<SymbolicSubstitutionCaseReport> = substitutions.iter().map(|item| {
        let preimage = format!(
            "substitution:{}|target:{}|replacement:{}|scope:{}|expected:{}|law:{}|digest:{}|status:{}",
            item.0, item.1, item.2, item.3, item.4, item.5, item.6, item.7
        );
        SymbolicSubstitutionCaseReport {
            id: item.0.clone(),
            target: item.1.clone(),
            expected: item.4.clone(),
            digest: item.6.clone(),
            case_hash: stable_hash_label("lyra.p01.symbolic_equality.substitution", &preimage),
        }
    }).collect();
    substitution_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut receipt_reports: Vec<SymbolicEqualityReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|path:{}|target:{}|status:{}",
                item.0, item.1, item.2, item.3
            );
            SymbolicEqualityReceiptReport {
                id: item.0.clone(),
                path: item.1.clone(),
                target: item.2.clone(),
                receipt_hash: stable_hash_label("lyra.p01.symbolic_equality.receipt", &preimage),
            }
        })
        .collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_lines = Vec::new();
    for item in &equality_rule_reports {
        suite_lines.push(format!("equality_rule:{}|{}", item.id, item.rule_hash));
    }
    for item in &equivalence_class_reports {
        suite_lines.push(format!("equivalence_class:{}|{}", item.id, item.class_hash));
    }
    for item in &normalization_reports {
        suite_lines.push(format!(
            "normalization:{}|{}|{}",
            item.id, item.digest, item.case_hash
        ));
    }
    for item in &substitution_reports {
        suite_lines.push(format!(
            "substitution:{}|{}|{}",
            item.id, item.digest, item.case_hash
        ));
    }
    for item in &receipt_reports {
        suite_lines.push(format!("receipt:{}|{}", item.id, item.receipt_hash));
    }
    suite_lines.sort();

    SymbolicEqualitySuiteReport {
        equality_rule_count: equality_rule_reports.len(),
        equivalence_class_count: equivalence_class_reports.len(),
        normalization_count: normalization_reports.len(),
        substitution_count: substitution_reports.len(),
        receipt_count: receipt_reports.len(),
        equality_rule_reports,
        equivalence_class_reports,
        normalization_reports,
        substitution_reports,
        receipt_reports,
        suite_hash: stable_hash_label("lyra.p01.symbolic_equality.suite", &suite_lines.join("\n")),
    }
}
