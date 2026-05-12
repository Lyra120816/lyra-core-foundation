use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceLibraryReport {
    pub id: String,
    pub owner_root: String,
    pub atom_count: usize,
    pub library_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceExampleReport {
    pub id: String,
    pub atom_id: String,
    pub example_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomInspectionToolReport {
    pub id: String,
    pub output_contract: String,
    pub tool_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceGateReport {
    pub id: String,
    pub law: String,
    pub gate_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticAtomReferenceSuiteReport {
    pub library_count: usize,
    pub example_count: usize,
    pub tool_count: usize,
    pub gate_count: usize,
    pub receipt_count: usize,
    pub library_reports: Vec<SemanticAtomReferenceLibraryReport>,
    pub example_reports: Vec<SemanticAtomReferenceExampleReport>,
    pub tool_reports: Vec<SemanticAtomInspectionToolReport>,
    pub gate_reports: Vec<SemanticAtomReferenceGateReport>,
    pub receipt_reports: Vec<SemanticAtomReferenceReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_atom_reference_suite_report(
    libraries: &[(String, String, String, String, String, String, String)],
    examples: &[(String, String, String, String, String, String)],
    tools: &[(String, String, String, String, String, String, String)],
    gates: &[(String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> SemanticAtomReferenceSuiteReport {
    let mut library_reports: Vec<SemanticAtomReferenceLibraryReport> = libraries
        .iter()
        .map(|item| {
            let preimage = format!(
                "library:{}|owner:{}|registry:{}|atoms:{}|path:{}|export:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            SemanticAtomReferenceLibraryReport {
                id: item.0.clone(),
                owner_root: item.1.clone(),
                atom_count: item.3.split(',').filter(|part| !part.is_empty()).count(),
                library_hash: stable_hash_label(
                    "lyra.p01.semantic_atom_reference.library",
                    &preimage,
                ),
            }
        })
        .collect();
    library_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut example_reports: Vec<SemanticAtomReferenceExampleReport> = examples
        .iter()
        .map(|item| {
            let preimage = format!(
                "example:{}|library:{}|atom:{}|path:{}|expected:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            SemanticAtomReferenceExampleReport {
                id: item.0.clone(),
                atom_id: item.2.clone(),
                example_hash: stable_hash_label(
                    "lyra.p01.semantic_atom_reference.example",
                    &preimage,
                ),
            }
        })
        .collect();
    example_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut tool_reports: Vec<SemanticAtomInspectionToolReport> = tools
        .iter()
        .map(|item| {
            let preimage = format!(
                "tool:{}|binary:{}|input:{}|output:{}|fixture:{}|receipt:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5, item.6
            );
            SemanticAtomInspectionToolReport {
                id: item.0.clone(),
                output_contract: item.3.clone(),
                tool_hash: stable_hash_label("lyra.p01.semantic_atom_reference.tool", &preimage),
            }
        })
        .collect();
    tool_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut gate_reports: Vec<SemanticAtomReferenceGateReport> = gates
        .iter()
        .map(|item| {
            let preimage = format!(
                "gate:{}|scope:{}|law:{}|evidence:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            SemanticAtomReferenceGateReport {
                id: item.0.clone(),
                law: item.2.clone(),
                gate_hash: stable_hash_label("lyra.p01.semantic_atom_reference.gate", &preimage),
            }
        })
        .collect();
    gate_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut receipt_reports: Vec<SemanticAtomReferenceReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|path:{}|target:{}|status:{}",
                item.0, item.1, item.2, item.3
            );
            SemanticAtomReferenceReceiptReport {
                id: item.0.clone(),
                path: item.1.clone(),
                target: item.2.clone(),
                receipt_hash: stable_hash_label(
                    "lyra.p01.semantic_atom_reference.receipt",
                    &preimage,
                ),
            }
        })
        .collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_lines = Vec::new();
    for item in &library_reports {
        suite_lines.push(format!("library:{}:{}", item.id, item.library_hash));
    }
    for item in &example_reports {
        suite_lines.push(format!("example:{}:{}", item.id, item.example_hash));
    }
    for item in &tool_reports {
        suite_lines.push(format!("tool:{}:{}", item.id, item.tool_hash));
    }
    for item in &gate_reports {
        suite_lines.push(format!("gate:{}:{}", item.id, item.gate_hash));
    }
    for item in &receipt_reports {
        suite_lines.push(format!("receipt:{}:{}", item.id, item.receipt_hash));
    }
    suite_lines.sort();
    let suite_hash = stable_hash_label(
        "lyra.p01.semantic_atom_reference.suite",
        &suite_lines.join("\n"),
    );
    SemanticAtomReferenceSuiteReport {
        library_count: library_reports.len(),
        example_count: example_reports.len(),
        tool_count: tool_reports.len(),
        gate_count: gate_reports.len(),
        receipt_count: receipt_reports.len(),
        library_reports,
        example_reports,
        tool_reports,
        gate_reports,
        receipt_reports,
        suite_hash,
    }
}
