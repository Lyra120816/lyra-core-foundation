use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticFalsificationCaseDescriptor {
    pub id: &'static str,
    pub target_domain: &'static str,
    pub target_validator: &'static str,
    pub mutation: &'static str,
    pub expected_error: &'static str,
    pub fixture: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticFalsificationHarnessDescriptor {
    pub id: &'static str,
    pub runner: &'static str,
    pub cases: &'static [&'static str],
    pub assertion_mode: &'static str,
    pub receipt_policy: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticRejectionAssertionDescriptor {
    pub id: &'static str,
    pub case_id: &'static str,
    pub expected_error: &'static str,
    pub proof_surface: &'static str,
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticFalsificationArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub artifact_kind: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticFalsificationProofDescriptor {
    pub id: &'static str,
    pub cases: &'static [&'static str],
    pub harnesses: &'static [&'static str],
    pub assertions: &'static [&'static str],
    pub artifacts: &'static [&'static str],
    pub receipt: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticFalsificationError {
    UnknownCase,
    UnknownHarness,
    UnknownAssertion,
    UnknownArtifact,
    UnknownProof,
}

pub const LYRA_P01_SEMANTIC_FALSIFICATION_CARRIER: &str = "lyra_p01_semantic_falsification";

pub const LYRALANG_SEMANTIC_FALSIFICATION_CASES: &[SemanticFalsificationCaseDescriptor] = &[
    SemanticFalsificationCaseDescriptor { id: "canonical_symbol_uppercase_case", target_domain: "canonical_symbols", target_validator: "semantic_core_engine", mutation: "uppercase_symbol_identity", expected_error: "invalid_negative_case", fixture: "fixtures/p01/semantic_falsification_cases/canonical_symbol_uppercase_case.lyra", status: "artifact_emitted" },
    SemanticFalsificationCaseDescriptor { id: "canonical_symbol_duplicate_identity_case", target_domain: "canonical_symbols", target_validator: "semantic_identity", mutation: "duplicate_symbol_identity", expected_error: "duplicate_identity", fixture: "fixtures/p01/semantic_falsification_cases/canonical_symbol_duplicate_identity_case.lyra", status: "artifact_emitted" },
    SemanticFalsificationCaseDescriptor { id: "canonical_symbol_control_byte_case", target_domain: "canonical_symbols", target_validator: "canonical_symbol_ingest", mutation: "control_byte_in_symbol_stream", expected_error: "canonical_control_byte", fixture: "fixtures/p01/semantic_falsification_cases/canonical_symbol_control_byte_case.lyra", status: "artifact_emitted" },
    SemanticFalsificationCaseDescriptor { id: "semantic_atom_unknown_family_case", target_domain: "semantic_atoms", target_validator: "semantic_atoms", mutation: "unknown_atom_family", expected_error: "invalid_semantic_domain", fixture: "fixtures/p01/semantic_falsification_cases/semantic_atom_unknown_family_case.lyra", status: "artifact_emitted" },
    SemanticFalsificationCaseDescriptor { id: "semantic_atom_descriptor_drift_case", target_domain: "semantic_atoms", target_validator: "semantic_atoms", mutation: "descriptor_registry_drift", expected_error: "semantic_drift_accepted", fixture: "fixtures/p01/semantic_falsification_cases/semantic_atom_descriptor_drift_case.lyra", status: "artifact_emitted" },
    SemanticFalsificationCaseDescriptor { id: "semantic_atom_probabilistic_truth_case", target_domain: "semantic_atoms", target_validator: "semantic_atoms", mutation: "probabilistic_atom_truth", expected_error: "probabilistic_truth_allowed", fixture: "fixtures/p01/semantic_falsification_cases/semantic_atom_probabilistic_truth_case.lyra", status: "artifact_emitted" },
    SemanticFalsificationCaseDescriptor { id: "core_ir_version_drift_case", target_domain: "core_ir", target_validator: "core_ir", mutation: "unregistered_ir_version", expected_error: "invalid_model_binding", fixture: "fixtures/p01/semantic_falsification_cases/core_ir_version_drift_case.lyra", status: "artifact_emitted" },
    SemanticFalsificationCaseDescriptor { id: "core_ir_encoding_drift_case", target_domain: "core_ir", target_validator: "core_ir", mutation: "text_binary_parity_break", expected_error: "canonical_model_drift_accepted", fixture: "fixtures/p01/semantic_falsification_cases/core_ir_encoding_drift_case.lyra", status: "artifact_emitted" },
    SemanticFalsificationCaseDescriptor { id: "core_ir_unbound_receipt_case", target_domain: "core_ir", target_validator: "core_ir_reuse", mutation: "unbound_ir_receipt_reference", expected_error: "replay_proof_unbound", fixture: "fixtures/p01/semantic_falsification_cases/core_ir_unbound_receipt_case.lyra", status: "artifact_emitted" },
];

pub const LYRALANG_SEMANTIC_FALSIFICATION_HARNESSES: &[SemanticFalsificationHarnessDescriptor] = &[
    SemanticFalsificationHarnessDescriptor {
        id: "semantic_negative_corpus_parser",
        runner: "lyra-p01-semantic-falsification-check",
        cases: &[
            "canonical_symbol_uppercase_case",
            "canonical_symbol_duplicate_identity_case",
            "canonical_symbol_control_byte_case",
            "semantic_atom_unknown_family_case",
            "semantic_atom_descriptor_drift_case",
            "semantic_atom_probabilistic_truth_case",
            "core_ir_version_drift_case",
            "core_ir_encoding_drift_case",
            "core_ir_unbound_receipt_case",
        ],
        assertion_mode: "parse_all_cases_before_accept",
        receipt_policy: "emit_rejected_receipt_per_case",
        status: "artifact_emitted",
    },
    SemanticFalsificationHarnessDescriptor {
        id: "canonical_symbol_falsifier",
        runner: "semantic_core_engine_validator",
        cases: &[
            "canonical_symbol_uppercase_case",
            "canonical_symbol_duplicate_identity_case",
            "canonical_symbol_control_byte_case",
        ],
        assertion_mode: "expected_error_exact",
        receipt_policy: "receipt_bound_replay",
        status: "artifact_emitted",
    },
    SemanticFalsificationHarnessDescriptor {
        id: "semantic_atom_falsifier",
        runner: "semantic_atom_validator",
        cases: &[
            "semantic_atom_unknown_family_case",
            "semantic_atom_descriptor_drift_case",
            "semantic_atom_probabilistic_truth_case",
        ],
        assertion_mode: "expected_error_exact",
        receipt_policy: "receipt_bound_replay",
        status: "artifact_emitted",
    },
    SemanticFalsificationHarnessDescriptor {
        id: "core_ir_falsifier",
        runner: "core_ir_validator",
        cases: &[
            "core_ir_version_drift_case",
            "core_ir_encoding_drift_case",
            "core_ir_unbound_receipt_case",
        ],
        assertion_mode: "expected_error_exact",
        receipt_policy: "receipt_bound_replay",
        status: "artifact_emitted",
    },
    SemanticFalsificationHarnessDescriptor {
        id: "cross_surface_receipt_replay_checker",
        runner: "p01_receipt_replay_checker",
        cases: &[
            "canonical_symbol_duplicate_identity_case",
            "semantic_atom_descriptor_drift_case",
            "core_ir_encoding_drift_case",
            "core_ir_unbound_receipt_case",
        ],
        assertion_mode: "negative_never_accepts",
        receipt_policy: "golden_receipt_parity",
        status: "execution_proven",
    },
];

pub const LYRALANG_SEMANTIC_REJECTION_ASSERTIONS: &[SemanticRejectionAssertionDescriptor] = &[
    SemanticRejectionAssertionDescriptor {
        id: "canonical_symbol_uppercase_rejection",
        case_id: "canonical_symbol_uppercase_case",
        expected_error: "invalid_negative_case",
        proof_surface: "semantic_core_engine",
        forbids: &["negative_fixture_accepted", "canonical_drift_accepted"],
        status: "artifact_emitted",
    },
    SemanticRejectionAssertionDescriptor {
        id: "canonical_symbol_duplicate_identity_rejection",
        case_id: "canonical_symbol_duplicate_identity_case",
        expected_error: "duplicate_identity",
        proof_surface: "semantic_identity",
        forbids: &["negative_fixture_accepted", "semantic_drift_accepted"],
        status: "artifact_emitted",
    },
    SemanticRejectionAssertionDescriptor {
        id: "canonical_symbol_control_byte_rejection",
        case_id: "canonical_symbol_control_byte_case",
        expected_error: "canonical_control_byte",
        proof_surface: "canonical_ingest",
        forbids: &["negative_fixture_accepted", "manual_only"],
        status: "artifact_emitted",
    },
    SemanticRejectionAssertionDescriptor {
        id: "semantic_atom_unknown_family_rejection",
        case_id: "semantic_atom_unknown_family_case",
        expected_error: "invalid_semantic_domain",
        proof_surface: "semantic_atoms",
        forbids: &["negative_fixture_accepted", "corpus_drift_accepted"],
        status: "artifact_emitted",
    },
    SemanticRejectionAssertionDescriptor {
        id: "semantic_atom_descriptor_drift_rejection",
        case_id: "semantic_atom_descriptor_drift_case",
        expected_error: "semantic_drift_accepted",
        proof_surface: "semantic_atoms",
        forbids: &["negative_fixture_accepted", "registry_drift_accepted"],
        status: "artifact_emitted",
    },
    SemanticRejectionAssertionDescriptor {
        id: "semantic_atom_probabilistic_truth_rejection",
        case_id: "semantic_atom_probabilistic_truth_case",
        expected_error: "probabilistic_truth_allowed",
        proof_surface: "semantic_atoms",
        forbids: &[
            "negative_fixture_accepted",
            "probabilistic_truth_allowed_in_core",
        ],
        status: "artifact_emitted",
    },
    SemanticRejectionAssertionDescriptor {
        id: "core_ir_version_drift_rejection",
        case_id: "core_ir_version_drift_case",
        expected_error: "invalid_model_binding",
        proof_surface: "core_ir",
        forbids: &["negative_fixture_accepted", "unversioned_ir_accepted"],
        status: "artifact_emitted",
    },
    SemanticRejectionAssertionDescriptor {
        id: "core_ir_encoding_drift_rejection",
        case_id: "core_ir_encoding_drift_case",
        expected_error: "canonical_model_drift_accepted",
        proof_surface: "core_ir",
        forbids: &["negative_fixture_accepted", "text_binary_drift_accepted"],
        status: "artifact_emitted",
    },
    SemanticRejectionAssertionDescriptor {
        id: "core_ir_unbound_receipt_rejection",
        case_id: "core_ir_unbound_receipt_case",
        expected_error: "replay_proof_unbound",
        proof_surface: "core_ir_reuse",
        forbids: &["negative_fixture_accepted", "orphan_receipt_binding"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_FALSIFICATION_ARTIFACTS: &[SemanticFalsificationArtifactDescriptor] = &[
    SemanticFalsificationArtifactDescriptor {
        id: "semantic_falsification_contract",
        owner_root: "interfaces",
        path: "interfaces/p01/contracts/semantic_falsification.v1.lyra",
        artifact_kind: "contract",
        status: "artifact_emitted",
    },
    SemanticFalsificationArtifactDescriptor {
        id: "semantic_falsification_law",
        owner_root: "ops",
        path: "ops/p01/control/semantic_falsification_law.v1.lyra",
        artifact_kind: "law",
        status: "artifact_emitted",
    },
    SemanticFalsificationArtifactDescriptor {
        id: "semantic_falsification_operator",
        owner_root: "src",
        path: "src/bin/lyra-p01-semantic-falsification-check.rs",
        artifact_kind: "binary",
        status: "artifact_emitted",
    },
    SemanticFalsificationArtifactDescriptor {
        id: "valid_semantic_falsification_fixture",
        owner_root: "fixtures",
        path: "fixtures/p01/semantic_falsification_inputs/valid_semantic_falsification.lyra",
        artifact_kind: "fixture",
        status: "artifact_emitted",
    },
    SemanticFalsificationArtifactDescriptor {
        id: "golden_semantic_falsification_receipt",
        owner_root: "goldens",
        path: "goldens/p01/valid_semantic_falsification.receipt",
        artifact_kind: "golden",
        status: "artifact_emitted",
    },
    SemanticFalsificationArtifactDescriptor {
        id: "execution_semantic_falsification_receipt",
        owner_root: "receipts",
        path: "receipts/p01/pass_0045_semantic_falsification.receipt",
        artifact_kind: "receipt",
        status: "artifact_emitted",
    },
    SemanticFalsificationArtifactDescriptor {
        id: "deterministic_semantic_falsification_report",
        owner_root: "k0",
        path: "k0/determinism/src/semantic_falsification.rs",
        artifact_kind: "report",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_SEMANTIC_FALSIFICATION_PROOFS: &[SemanticFalsificationProofDescriptor] = &[
    SemanticFalsificationProofDescriptor {
        id: "canonical_symbol_negative_proof",
        cases: &[
            "canonical_symbol_uppercase_case",
            "canonical_symbol_duplicate_identity_case",
            "canonical_symbol_control_byte_case",
        ],
        harnesses: &["canonical_symbol_falsifier"],
        assertions: &[
            "canonical_symbol_uppercase_rejection",
            "canonical_symbol_duplicate_identity_rejection",
            "canonical_symbol_control_byte_rejection",
        ],
        artifacts: &[
            "semantic_falsification_contract",
            "valid_semantic_falsification_fixture",
        ],
        receipt: "receipts/p01/pass_0045_semantic_falsification.receipt",
        status: "artifact_emitted",
    },
    SemanticFalsificationProofDescriptor {
        id: "semantic_atom_negative_proof",
        cases: &[
            "semantic_atom_unknown_family_case",
            "semantic_atom_descriptor_drift_case",
            "semantic_atom_probabilistic_truth_case",
        ],
        harnesses: &["semantic_atom_falsifier"],
        assertions: &[
            "semantic_atom_unknown_family_rejection",
            "semantic_atom_descriptor_drift_rejection",
            "semantic_atom_probabilistic_truth_rejection",
        ],
        artifacts: &[
            "semantic_falsification_law",
            "deterministic_semantic_falsification_report",
        ],
        receipt: "receipts/p01/pass_0045_semantic_falsification.receipt",
        status: "artifact_emitted",
    },
    SemanticFalsificationProofDescriptor {
        id: "core_ir_negative_proof",
        cases: &[
            "core_ir_version_drift_case",
            "core_ir_encoding_drift_case",
            "core_ir_unbound_receipt_case",
        ],
        harnesses: &["core_ir_falsifier"],
        assertions: &[
            "core_ir_version_drift_rejection",
            "core_ir_encoding_drift_rejection",
            "core_ir_unbound_receipt_rejection",
        ],
        artifacts: &[
            "semantic_falsification_operator",
            "golden_semantic_falsification_receipt",
        ],
        receipt: "receipts/p01/pass_0045_semantic_falsification.receipt",
        status: "artifact_emitted",
    },
    SemanticFalsificationProofDescriptor {
        id: "p01_semantic_falsification_parity_proof",
        cases: &[
            "canonical_symbol_uppercase_case",
            "canonical_symbol_duplicate_identity_case",
            "canonical_symbol_control_byte_case",
            "semantic_atom_unknown_family_case",
            "semantic_atom_descriptor_drift_case",
            "semantic_atom_probabilistic_truth_case",
            "core_ir_version_drift_case",
            "core_ir_encoding_drift_case",
            "core_ir_unbound_receipt_case",
        ],
        harnesses: &[
            "semantic_negative_corpus_parser",
            "canonical_symbol_falsifier",
            "semantic_atom_falsifier",
            "core_ir_falsifier",
            "cross_surface_receipt_replay_checker",
        ],
        assertions: &[
            "canonical_symbol_uppercase_rejection",
            "canonical_symbol_duplicate_identity_rejection",
            "canonical_symbol_control_byte_rejection",
            "semantic_atom_unknown_family_rejection",
            "semantic_atom_descriptor_drift_rejection",
            "semantic_atom_probabilistic_truth_rejection",
            "core_ir_version_drift_rejection",
            "core_ir_encoding_drift_rejection",
            "core_ir_unbound_receipt_rejection",
        ],
        artifacts: &[
            "semantic_falsification_contract",
            "semantic_falsification_law",
            "semantic_falsification_operator",
            "valid_semantic_falsification_fixture",
            "golden_semantic_falsification_receipt",
            "execution_semantic_falsification_receipt",
            "deterministic_semantic_falsification_report",
        ],
        receipt: "receipts/p01/pass_0045_semantic_falsification.receipt",
        status: "execution_proven",
    },
];

pub fn semantic_falsification_case_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_FALSIFICATION_CASES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_falsification_harness_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_FALSIFICATION_HARNESSES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_rejection_assertion_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_REJECTION_ASSERTIONS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_falsification_artifact_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_FALSIFICATION_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn semantic_falsification_proof_ids() -> Vec<&'static str> {
    LYRALANG_SEMANTIC_FALSIFICATION_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn semantic_falsification_case_descriptor(
    id: &str,
) -> Option<&'static SemanticFalsificationCaseDescriptor> {
    LYRALANG_SEMANTIC_FALSIFICATION_CASES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_falsification_harness_descriptor(
    id: &str,
) -> Option<&'static SemanticFalsificationHarnessDescriptor> {
    LYRALANG_SEMANTIC_FALSIFICATION_HARNESSES
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_rejection_assertion_descriptor(
    id: &str,
) -> Option<&'static SemanticRejectionAssertionDescriptor> {
    LYRALANG_SEMANTIC_REJECTION_ASSERTIONS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_falsification_artifact_descriptor(
    id: &str,
) -> Option<&'static SemanticFalsificationArtifactDescriptor> {
    LYRALANG_SEMANTIC_FALSIFICATION_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}
pub fn semantic_falsification_proof_descriptor(
    id: &str,
) -> Option<&'static SemanticFalsificationProofDescriptor> {
    LYRALANG_SEMANTIC_FALSIFICATION_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn semantic_falsification_case_signature(item: &SemanticFalsificationCaseDescriptor) -> String {
    format!(
        "case:{}|target:{}|validator:{}|mutation:{}|expected:{}|fixture:{}|status:{}",
        item.id,
        item.target_domain,
        item.target_validator,
        item.mutation,
        item.expected_error,
        item.fixture,
        item.status
    )
}
pub fn semantic_falsification_harness_signature(
    item: &SemanticFalsificationHarnessDescriptor,
) -> String {
    format!(
        "harness:{}|runner:{}|cases:{}|mode:{}|receipt:{}|status:{}",
        item.id,
        item.runner,
        item.cases.join(","),
        item.assertion_mode,
        item.receipt_policy,
        item.status
    )
}
pub fn semantic_rejection_assertion_signature(
    item: &SemanticRejectionAssertionDescriptor,
) -> String {
    format!(
        "assertion:{}|case:{}|expected:{}|surface:{}|forbids:{}|status:{}",
        item.id,
        item.case_id,
        item.expected_error,
        item.proof_surface,
        item.forbids.join(","),
        item.status
    )
}
pub fn semantic_falsification_artifact_signature(
    item: &SemanticFalsificationArtifactDescriptor,
) -> String {
    format!(
        "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
        item.id, item.owner_root, item.path, item.artifact_kind, item.status
    )
}
pub fn semantic_falsification_proof_signature(
    item: &SemanticFalsificationProofDescriptor,
) -> String {
    format!(
        "proof:{}|cases:{}|harnesses:{}|assertions:{}|artifacts:{}|receipt:{}|status:{}",
        item.id,
        item.cases.join(","),
        item.harnesses.join(","),
        item.assertions.join(","),
        item.artifacts.join(","),
        item.receipt,
        item.status
    )
}

pub fn semantic_falsification_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_SEMANTIC_FALSIFICATION_CASES {
        rows.push(semantic_falsification_case_signature(item));
    }
    for item in LYRALANG_SEMANTIC_FALSIFICATION_HARNESSES {
        rows.push(semantic_falsification_harness_signature(item));
    }
    for item in LYRALANG_SEMANTIC_REJECTION_ASSERTIONS {
        rows.push(semantic_rejection_assertion_signature(item));
    }
    for item in LYRALANG_SEMANTIC_FALSIFICATION_ARTIFACTS {
        rows.push(semantic_falsification_artifact_signature(item));
    }
    for item in LYRALANG_SEMANTIC_FALSIFICATION_PROOFS {
        rows.push(semantic_falsification_proof_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn semantic_falsification_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.semantic_falsification.registry",
        &semantic_falsification_registry_signature(),
    )
}
pub fn semantic_falsification_case_digest(id: &str) -> Option<String> {
    semantic_falsification_case_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_falsification.case",
            &semantic_falsification_case_signature(item),
        )
    })
}
pub fn semantic_falsification_harness_digest(id: &str) -> Option<String> {
    semantic_falsification_harness_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_falsification.harness",
            &semantic_falsification_harness_signature(item),
        )
    })
}
pub fn semantic_rejection_assertion_digest(id: &str) -> Option<String> {
    semantic_rejection_assertion_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_falsification.assertion",
            &semantic_rejection_assertion_signature(item),
        )
    })
}
pub fn semantic_falsification_artifact_digest(id: &str) -> Option<String> {
    semantic_falsification_artifact_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_falsification.artifact",
            &semantic_falsification_artifact_signature(item),
        )
    })
}
pub fn semantic_falsification_proof_digest(id: &str) -> Option<String> {
    semantic_falsification_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p01.semantic_falsification.proof",
            &semantic_falsification_proof_signature(item),
        )
    })
}

pub fn semantic_falsification_harnesses_bind_known_cases() -> bool {
    LYRALANG_SEMANTIC_FALSIFICATION_HARNESSES
        .iter()
        .all(|harness| {
            harness
                .cases
                .iter()
                .all(|id| semantic_falsification_case_descriptor(id).is_some())
        })
}

pub fn semantic_rejection_assertions_bind_known_cases() -> bool {
    LYRALANG_SEMANTIC_REJECTION_ASSERTIONS
        .iter()
        .all(|assertion| {
            semantic_falsification_case_descriptor(assertion.case_id)
                .map(|case| case.expected_error == assertion.expected_error)
                .unwrap_or(false)
        })
}

pub fn semantic_falsification_artifacts_bind_paths() -> bool {
    LYRALANG_SEMANTIC_FALSIFICATION_ARTIFACTS
        .iter()
        .all(|artifact| {
            !artifact.path.is_empty()
                && !artifact.path.contains("..")
                && ["lyra", "rs", "receipt"]
                    .iter()
                    .any(|suffix| artifact.path.ends_with(suffix))
        })
}

pub fn semantic_falsification_proofs_bind_registry() -> bool {
    LYRALANG_SEMANTIC_FALSIFICATION_PROOFS.iter().all(|proof| {
        proof
            .cases
            .iter()
            .all(|id| semantic_falsification_case_descriptor(id).is_some())
            && proof
                .harnesses
                .iter()
                .all(|id| semantic_falsification_harness_descriptor(id).is_some())
            && proof
                .assertions
                .iter()
                .all(|id| semantic_rejection_assertion_descriptor(id).is_some())
            && proof
                .artifacts
                .iter()
                .all(|id| semantic_falsification_artifact_descriptor(id).is_some())
            && proof.receipt.ends_with(".receipt")
    })
}

pub fn semantic_falsification_targets_all_required_domains() -> bool {
    let mut domains: Vec<&str> = LYRALANG_SEMANTIC_FALSIFICATION_CASES
        .iter()
        .map(|item| item.target_domain)
        .collect();
    domains.sort();
    domains.dedup();
    domains == vec!["canonical_symbols", "core_ir", "semantic_atoms"]
}

pub fn semantic_falsification_no_forbidden_descriptor_claims() -> bool {
    let lower = semantic_falsification_registry_signature().to_ascii_lowercase();
    !(lower.contains("accept negative")
        || lower.contains("negative accepted")
        || lower.contains("manual only")
        || lower.contains("todo")
        || lower.contains("placeholder")
        || lower.contains("phase closed")
        || lower.contains("network required"))
}
