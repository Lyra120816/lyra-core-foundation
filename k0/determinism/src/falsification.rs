use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalsificationCaseReport {
    pub order: String,
    pub case_id: String,
    pub expected_code: String,
    pub fixture_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalsificationSuiteReport {
    pub task: String,
    pub case_count: usize,
    pub cases: Vec<FalsificationCaseReport>,
    pub suite_hash: String,
}

pub fn deterministic_falsification_report(
    task: &str,
    cases: &[(&str, &str, &str)],
) -> FalsificationSuiteReport {
    let mut ordered: Vec<(&str, &str, &str)> = cases.to_vec();
    ordered.sort_by(|left, right| {
        left.0
            .cmp(right.0)
            .then(left.1.cmp(right.1))
            .then(left.2.cmp(right.2))
    });
    let mut reports = Vec::with_capacity(ordered.len());
    for (index, (case_id, expected_code, fixture_text)) in ordered.iter().enumerate() {
        reports.push(FalsificationCaseReport {
            order: format!("{:03}", index + 1),
            case_id: (*case_id).to_string(),
            expected_code: (*expected_code).to_string(),
            fixture_hash: stable_hash_label("lyra.p00.falsification.fixture", fixture_text),
        });
    }
    let mut preimage = String::new();
    preimage.push_str(task);
    preimage.push('\n');
    for report in &reports {
        preimage.push_str(&report.order);
        preimage.push('|');
        preimage.push_str(&report.case_id);
        preimage.push('|');
        preimage.push_str(&report.expected_code);
        preimage.push('|');
        preimage.push_str(&report.fixture_hash);
        preimage.push('\n');
    }
    let suite_hash = stable_hash_label("lyra.p00.falsification.suite", &preimage);
    FalsificationSuiteReport {
        task: task.to_string(),
        case_count: reports.len(),
        cases: reports,
        suite_hash,
    }
}
