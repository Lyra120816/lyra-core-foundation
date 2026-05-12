use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticDomainReport {
    pub id: String,
    pub core_ref: String,
    pub domain_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticLawReport {
    pub id: String,
    pub scope: String,
    pub law_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticInvariantReport {
    pub id: String,
    pub evidence_ref: String,
    pub invariant_hash: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticProofReport {
    pub id: String,
    pub law_ref: String,
    pub proof_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalSemanticConstitutionSuiteReport {
    pub domain_count: usize,
    pub law_count: usize,
    pub invariant_count: usize,
    pub proof_count: usize,
    pub domain_reports: Vec<FormalSemanticDomainReport>,
    pub law_reports: Vec<FormalSemanticLawReport>,
    pub invariant_reports: Vec<FormalSemanticInvariantReport>,
    pub proof_reports: Vec<FormalSemanticProofReport>,
    pub suite_hash: String,
}

pub fn deterministic_formal_semantic_constitution_suite_report(
    domains: &[(String, String, String, String, String, String)],
    laws: &[(String, String, String, String, String)],
    invariants: &[(String, String, String, String, String)],
    proofs: &[(String, String, String, String, String, String)],
) -> FormalSemanticConstitutionSuiteReport {
    let mut domain_reports: Vec<FormalSemanticDomainReport> = domains
        .iter()
        .map(|item| {
            let preimage = format!(
                "domain:{}|layer:{}|owner:{}|meaning:{}|core:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            FormalSemanticDomainReport {
                id: item.0.clone(),
                core_ref: item.4.clone(),
                domain_hash: stable_hash_label(
                    "lyra.p01.formal_semantic_constitution.domain",
                    &preimage,
                ),
            }
        })
        .collect();
    domain_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut law_reports: Vec<FormalSemanticLawReport> = laws
        .iter()
        .map(|item| {
            let preimage = format!(
                "law:{}|scope:{}|rule:{}|guard:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            FormalSemanticLawReport {
                id: item.0.clone(),
                scope: item.1.clone(),
                law_hash: stable_hash_label("lyra.p01.formal_semantic_constitution.law", &preimage),
            }
        })
        .collect();
    law_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut invariant_reports: Vec<FormalSemanticInvariantReport> = invariants
        .iter()
        .map(|item| {
            let preimage = format!(
                "invariant:{}|applies:{}|assertion:{}|evidence:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4
            );
            FormalSemanticInvariantReport {
                id: item.0.clone(),
                evidence_ref: item.3.clone(),
                invariant_hash: stable_hash_label(
                    "lyra.p01.formal_semantic_constitution.invariant",
                    &preimage,
                ),
            }
        })
        .collect();
    invariant_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut proof_reports: Vec<FormalSemanticProofReport> = proofs
        .iter()
        .map(|item| {
            let preimage = format!(
                "proof:{}|fixture:{}|golden:{}|receipt:{}|law:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            FormalSemanticProofReport {
                id: item.0.clone(),
                law_ref: item.4.clone(),
                proof_hash: stable_hash_label(
                    "lyra.p01.formal_semantic_constitution.proof",
                    &preimage,
                ),
            }
        })
        .collect();
    proof_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let preimage = format!(
        "domains:{}|laws:{}|invariants:{}|proofs:{}|domain_hashes:{}|law_hashes:{}|invariant_hashes:{}|proof_hashes:{}",
        domain_reports.len(),
        law_reports.len(),
        invariant_reports.len(),
        proof_reports.len(),
        domain_reports.iter().map(|item| item.domain_hash.as_str()).collect::<Vec<_>>().join(","),
        law_reports.iter().map(|item| item.law_hash.as_str()).collect::<Vec<_>>().join(","),
        invariant_reports.iter().map(|item| item.invariant_hash.as_str()).collect::<Vec<_>>().join(","),
        proof_reports.iter().map(|item| item.proof_hash.as_str()).collect::<Vec<_>>().join(","),
    );
    let suite_hash = stable_hash_label("lyra.p01.formal_semantic_constitution.suite", &preimage);

    FormalSemanticConstitutionSuiteReport {
        domain_count: domain_reports.len(),
        law_count: law_reports.len(),
        invariant_count: invariant_reports.len(),
        proof_count: proof_reports.len(),
        domain_reports,
        law_reports,
        invariant_reports,
        proof_reports,
        suite_hash,
    }
}
