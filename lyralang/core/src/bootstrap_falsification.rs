use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapFalsificationCaseDescriptor {
    pub id: &'static str,
    pub target_domain: &'static str,
    pub target_validator: &'static str,
    pub mutation: &'static str,
    pub expected_error: &'static str,
    pub fixture: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapFalsificationHarnessDescriptor {
    pub id: &'static str,
    pub runner: &'static str,
    pub cases: &'static [&'static str],
    pub assertion_mode: &'static str,
    pub receipt_policy: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapRejectionAssertionDescriptor {
    pub id: &'static str,
    pub case_id: &'static str,
    pub expected_error: &'static str,
    pub proof_surface: &'static str,
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapFalsificationArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub artifact_kind: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapFalsificationProofDescriptor {
    pub id: &'static str,
    pub cases: &'static [&'static str],
    pub harnesses: &'static [&'static str],
    pub assertions: &'static [&'static str],
    pub artifacts: &'static [&'static str],
    pub receipt: &'static str,
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_FALSIFICATION_CARRIER: &str = "lyra_p02_bootstrap_falsification";

pub const LYRALANG_BOOTSTRAP_FALSIFICATION_CASES: &[BootstrapFalsificationCaseDescriptor] = &[
    BootstrapFalsificationCaseDescriptor { id: "bootstrap_authority_missing_master_case", target_domain: "bootstrap_trust", target_validator: "bootstrap_authority_ingest", mutation: "missing_master_authority_layer", expected_error: "missing_master_authority", fixture: "fixtures/p02/bootstrap_falsification_cases/bootstrap_authority_missing_master_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "bootstrap_authority_ambient_override_case", target_domain: "bootstrap_trust", target_validator: "bootstrap_authority_ingest", mutation: "operator_declares_ambient_authority", expected_error: "ambient_authority", fixture: "fixtures/p02/bootstrap_falsification_cases/bootstrap_authority_ambient_override_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "seed_runtime_network_dependency_case", target_domain: "seed_runtime_law", target_validator: "seed_runtime_contract_validator", mutation: "network_dependent_seed_runtime", expected_error: "ambient_network_allowed", fixture: "fixtures/p02/bootstrap_falsification_cases/seed_runtime_network_dependency_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "seed_runtime_probabilistic_seed_case", target_domain: "seed_runtime_law", target_validator: "seed_runtime_contract_validator", mutation: "probabilistic_seed_choice", expected_error: "probabilistic_truth_allowed", fixture: "fixtures/p02/bootstrap_falsification_cases/seed_runtime_probabilistic_seed_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "host_extinction_unledgered_surface_case", target_domain: "host_extinction", target_validator: "host_boundary_challenge_validator", mutation: "foreign_surface_missing_extinction_entry", expected_error: "corpus_drift_accepted", fixture: "fixtures/p02/bootstrap_falsification_cases/host_extinction_unledgered_surface_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "host_extinction_delete_gate_bypass_case", target_domain: "host_extinction", target_validator: "bootstrap_extinction_validator", mutation: "retirement_without_receipt_gate", expected_error: "closure_before_receipt", fixture: "fixtures/p02/bootstrap_falsification_cases/host_extinction_delete_gate_bypass_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "foreign_boundary_hidden_surface_case", target_domain: "foreign_boundary", target_validator: "foreign_surface_closure_validator", mutation: "hidden_unclassified_foreign_surface", expected_error: "unknown_evidence_path", fixture: "fixtures/p02/bootstrap_falsification_cases/foreign_boundary_hidden_surface_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "operator_handoff_truth_drift_case", target_domain: "operator_handoff", target_validator: "operator_handoff_validator", mutation: "operator_capture_rewrites_truth_snapshot", expected_error: "corpus_drift_accepted", fixture: "fixtures/p02/bootstrap_falsification_cases/operator_handoff_truth_drift_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "emergency_fallback_ambient_time_case", target_domain: "emergency_fallback", target_validator: "emergency_fallback_validator", mutation: "fallback_uses_ambient_time", expected_error: "ambient_time_allowed", fixture: "fixtures/p02/bootstrap_falsification_cases/emergency_fallback_ambient_time_case.lyra", status: "artifact_emitted" },
    BootstrapFalsificationCaseDescriptor { id: "receipt_commit_hash_mismatch_case", target_domain: "receipt_commit", target_validator: "bootstrap_receipt_validator", mutation: "receipt_hash_mismatch_in_commit_chain", expected_error: "receipt_hash_mismatch", fixture: "fixtures/p02/bootstrap_falsification_cases/receipt_commit_hash_mismatch_case.lyra", status: "artifact_emitted" },
];

pub const LYRALANG_BOOTSTRAP_FALSIFICATION_HARNESSES: &[BootstrapFalsificationHarnessDescriptor] =
    &[
        BootstrapFalsificationHarnessDescriptor {
            id: "bootstrap_negative_corpus_parser",
            runner: "lyra-p02-bootstrap-falsification-check",
            cases: &[
                "bootstrap_authority_missing_master_case",
                "bootstrap_authority_ambient_override_case",
                "seed_runtime_network_dependency_case",
                "seed_runtime_probabilistic_seed_case",
                "host_extinction_unledgered_surface_case",
                "host_extinction_delete_gate_bypass_case",
                "foreign_boundary_hidden_surface_case",
                "operator_handoff_truth_drift_case",
                "emergency_fallback_ambient_time_case",
                "receipt_commit_hash_mismatch_case",
            ],
            assertion_mode: "parse_all_cases_before_accept",
            receipt_policy: "emit_rejected_receipt_per_case",
            status: "artifact_emitted",
        },
        BootstrapFalsificationHarnessDescriptor {
            id: "bootstrap_trust_falsifier",
            runner: "bootstrap_authority_ingest_validator",
            cases: &[
                "bootstrap_authority_missing_master_case",
                "bootstrap_authority_ambient_override_case",
            ],
            assertion_mode: "expected_error_exact",
            receipt_policy: "receipt_bound_replay",
            status: "artifact_emitted",
        },
        BootstrapFalsificationHarnessDescriptor {
            id: "seed_runtime_law_falsifier",
            runner: "seed_runtime_contract_validator",
            cases: &[
                "seed_runtime_network_dependency_case",
                "seed_runtime_probabilistic_seed_case",
            ],
            assertion_mode: "expected_error_exact",
            receipt_policy: "receipt_bound_replay",
            status: "artifact_emitted",
        },
        BootstrapFalsificationHarnessDescriptor {
            id: "host_extinction_falsifier",
            runner: "host_extinction_challenge_validator",
            cases: &[
                "host_extinction_unledgered_surface_case",
                "host_extinction_delete_gate_bypass_case",
                "foreign_boundary_hidden_surface_case",
            ],
            assertion_mode: "expected_error_exact",
            receipt_policy: "receipt_bound_replay",
            status: "artifact_emitted",
        },
        BootstrapFalsificationHarnessDescriptor {
            id: "handoff_fallback_receipt_falsifier",
            runner: "bootstrap_handoff_fallback_receipt_validator",
            cases: &[
                "operator_handoff_truth_drift_case",
                "emergency_fallback_ambient_time_case",
                "receipt_commit_hash_mismatch_case",
            ],
            assertion_mode: "expected_error_exact",
            receipt_policy: "receipt_bound_replay",
            status: "artifact_emitted",
        },
        BootstrapFalsificationHarnessDescriptor {
            id: "cross_bootstrap_replay_checker",
            runner: "p02_receipt_replay_checker",
            cases: &[
                "bootstrap_authority_ambient_override_case",
                "seed_runtime_network_dependency_case",
                "host_extinction_unledgered_surface_case",
                "operator_handoff_truth_drift_case",
                "receipt_commit_hash_mismatch_case",
            ],
            assertion_mode: "negative_never_accepts",
            receipt_policy: "golden_receipt_parity",
            status: "working_slice",
        },
    ];

pub const LYRALANG_BOOTSTRAP_REJECTION_ASSERTIONS: &[BootstrapRejectionAssertionDescriptor] = &[
    BootstrapRejectionAssertionDescriptor {
        id: "bootstrap_authority_missing_master_rejection",
        case_id: "bootstrap_authority_missing_master_case",
        expected_error: "missing_master_authority",
        proof_surface: "bootstrap_authority_ingest",
        forbids: &["negative_fixture_accepted", "ambient_authority"],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "bootstrap_authority_ambient_override_rejection",
        case_id: "bootstrap_authority_ambient_override_case",
        expected_error: "ambient_authority",
        proof_surface: "bootstrap_authority_ingest",
        forbids: &[
            "negative_fixture_accepted",
            "operator_override_constitution",
        ],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "seed_runtime_network_dependency_rejection",
        case_id: "seed_runtime_network_dependency_case",
        expected_error: "ambient_network_allowed",
        proof_surface: "seed_runtime_contract_validator",
        forbids: &[
            "negative_fixture_accepted",
            "network_required_in_seed_runtime",
        ],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "seed_runtime_probabilistic_seed_rejection",
        case_id: "seed_runtime_probabilistic_seed_case",
        expected_error: "probabilistic_truth_allowed",
        proof_surface: "seed_runtime_contract_validator",
        forbids: &["negative_fixture_accepted", "probabilistic_bootstrap_truth"],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "host_extinction_unledgered_surface_rejection",
        case_id: "host_extinction_unledgered_surface_case",
        expected_error: "corpus_drift_accepted",
        proof_surface: "host_boundary_challenge_validator",
        forbids: &["negative_fixture_accepted", "registry_drift_accepted"],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "host_extinction_delete_gate_bypass_rejection",
        case_id: "host_extinction_delete_gate_bypass_case",
        expected_error: "closure_before_receipt",
        proof_surface: "bootstrap_extinction_validator",
        forbids: &["negative_fixture_accepted", "retirement_without_receipt"],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "foreign_boundary_hidden_surface_rejection",
        case_id: "foreign_boundary_hidden_surface_case",
        expected_error: "unknown_evidence_path",
        proof_surface: "foreign_surface_closure_validator",
        forbids: &["negative_fixture_accepted", "hidden_foreign_surface"],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "operator_handoff_truth_drift_rejection",
        case_id: "operator_handoff_truth_drift_case",
        expected_error: "corpus_drift_accepted",
        proof_surface: "operator_handoff_validator",
        forbids: &["negative_fixture_accepted", "registry_drift_accepted"],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "emergency_fallback_ambient_time_rejection",
        case_id: "emergency_fallback_ambient_time_case",
        expected_error: "ambient_time_allowed",
        proof_surface: "emergency_fallback_validator",
        forbids: &["negative_fixture_accepted", "ambient_time_in_fallback"],
        status: "artifact_emitted",
    },
    BootstrapRejectionAssertionDescriptor {
        id: "receipt_commit_hash_mismatch_rejection",
        case_id: "receipt_commit_hash_mismatch_case",
        expected_error: "receipt_hash_mismatch",
        proof_surface: "bootstrap_receipt_validator",
        forbids: &["negative_fixture_accepted", "receipt_mismatch_ignored"],
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_FALSIFICATION_ARTIFACTS:
    &[BootstrapFalsificationArtifactDescriptor] = &[
    BootstrapFalsificationArtifactDescriptor {
        id: "bootstrap_falsification_contract",
        owner_root: "interfaces",
        path: "interfaces/p02/contracts/bootstrap_falsification.v1.lyra",
        artifact_kind: "contract",
        status: "artifact_emitted",
    },
    BootstrapFalsificationArtifactDescriptor {
        id: "bootstrap_falsification_law",
        owner_root: "ops",
        path: "ops/p02/falsification/bootstrap_falsification_harness.v1.lyra",
        artifact_kind: "law_manifest",
        status: "artifact_emitted",
    },
    BootstrapFalsificationArtifactDescriptor {
        id: "bootstrap_falsification_operator",
        owner_root: "src",
        path: "src/bin/lyra-p02-bootstrap-falsification-check.rs",
        artifact_kind: "binary",
        status: "artifact_emitted",
    },
    BootstrapFalsificationArtifactDescriptor {
        id: "valid_bootstrap_falsification_fixture",
        owner_root: "fixtures",
        path: "fixtures/p02/bootstrap_falsification_inputs/valid_bootstrap_falsification.lyra",
        artifact_kind: "valid_fixture",
        status: "artifact_emitted",
    },
    BootstrapFalsificationArtifactDescriptor {
        id: "golden_bootstrap_falsification_receipt",
        owner_root: "goldens",
        path: "goldens/p02/valid_bootstrap_falsification.receipt",
        artifact_kind: "golden_receipt",
        status: "artifact_emitted",
    },
    BootstrapFalsificationArtifactDescriptor {
        id: "execution_bootstrap_falsification_receipt",
        owner_root: "receipts",
        path: "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        artifact_kind: "execution_receipt",
        status: "artifact_emitted",
    },
    BootstrapFalsificationArtifactDescriptor {
        id: "deterministic_bootstrap_falsification_report",
        owner_root: "k0",
        path: "k0/determinism/src/bootstrap_falsification.rs",
        artifact_kind: "report",
        status: "artifact_emitted",
    },
    BootstrapFalsificationArtifactDescriptor {
        id: "bootstrap_falsification_case_pack",
        owner_root: "fixtures",
        path: "fixtures/p02/bootstrap_falsification_cases",
        artifact_kind: "negative_case_pack",
        status: "artifact_emitted",
    },
];

pub const LYRALANG_BOOTSTRAP_FALSIFICATION_PROOFS: &[BootstrapFalsificationProofDescriptor] = &[
    BootstrapFalsificationProofDescriptor {
        id: "bootstrap_trust_negative_proof",
        cases: &[
            "bootstrap_authority_missing_master_case",
            "bootstrap_authority_ambient_override_case",
        ],
        harnesses: &["bootstrap_trust_falsifier"],
        assertions: &[
            "bootstrap_authority_missing_master_rejection",
            "bootstrap_authority_ambient_override_rejection",
        ],
        artifacts: &[
            "bootstrap_falsification_contract",
            "valid_bootstrap_falsification_fixture",
        ],
        receipt: "receipts/p02/bootstrap_falsification/bootstrap_trust_negative.receipt",
        status: "artifact_emitted",
    },
    BootstrapFalsificationProofDescriptor {
        id: "seed_runtime_law_negative_proof",
        cases: &[
            "seed_runtime_network_dependency_case",
            "seed_runtime_probabilistic_seed_case",
        ],
        harnesses: &["seed_runtime_law_falsifier"],
        assertions: &[
            "seed_runtime_network_dependency_rejection",
            "seed_runtime_probabilistic_seed_rejection",
        ],
        artifacts: &[
            "bootstrap_falsification_law",
            "deterministic_bootstrap_falsification_report",
        ],
        receipt: "receipts/p02/bootstrap_falsification/seed_runtime_law_negative.receipt",
        status: "artifact_emitted",
    },
    BootstrapFalsificationProofDescriptor {
        id: "host_extinction_negative_proof",
        cases: &[
            "host_extinction_unledgered_surface_case",
            "host_extinction_delete_gate_bypass_case",
            "foreign_boundary_hidden_surface_case",
        ],
        harnesses: &["host_extinction_falsifier"],
        assertions: &[
            "host_extinction_unledgered_surface_rejection",
            "host_extinction_delete_gate_bypass_rejection",
            "foreign_boundary_hidden_surface_rejection",
        ],
        artifacts: &[
            "bootstrap_falsification_operator",
            "golden_bootstrap_falsification_receipt",
        ],
        receipt: "receipts/p02/bootstrap_falsification/host_extinction_negative.receipt",
        status: "artifact_emitted",
    },
    BootstrapFalsificationProofDescriptor {
        id: "handoff_fallback_receipt_negative_proof",
        cases: &[
            "operator_handoff_truth_drift_case",
            "emergency_fallback_ambient_time_case",
            "receipt_commit_hash_mismatch_case",
        ],
        harnesses: &["handoff_fallback_receipt_falsifier"],
        assertions: &[
            "operator_handoff_truth_drift_rejection",
            "emergency_fallback_ambient_time_rejection",
            "receipt_commit_hash_mismatch_rejection",
        ],
        artifacts: &[
            "bootstrap_falsification_case_pack",
            "execution_bootstrap_falsification_receipt",
        ],
        receipt: "receipts/p02/bootstrap_falsification/handoff_fallback_receipt_negative.receipt",
        status: "artifact_emitted",
    },
    BootstrapFalsificationProofDescriptor {
        id: "p02_bootstrap_falsification_parity_proof",
        cases: &[
            "bootstrap_authority_missing_master_case",
            "bootstrap_authority_ambient_override_case",
            "seed_runtime_network_dependency_case",
            "seed_runtime_probabilistic_seed_case",
            "host_extinction_unledgered_surface_case",
            "host_extinction_delete_gate_bypass_case",
            "foreign_boundary_hidden_surface_case",
            "operator_handoff_truth_drift_case",
            "emergency_fallback_ambient_time_case",
            "receipt_commit_hash_mismatch_case",
        ],
        harnesses: &[
            "bootstrap_negative_corpus_parser",
            "bootstrap_trust_falsifier",
            "seed_runtime_law_falsifier",
            "host_extinction_falsifier",
            "handoff_fallback_receipt_falsifier",
            "cross_bootstrap_replay_checker",
        ],
        assertions: &[
            "bootstrap_authority_missing_master_rejection",
            "bootstrap_authority_ambient_override_rejection",
            "seed_runtime_network_dependency_rejection",
            "seed_runtime_probabilistic_seed_rejection",
            "host_extinction_unledgered_surface_rejection",
            "host_extinction_delete_gate_bypass_rejection",
            "foreign_boundary_hidden_surface_rejection",
            "operator_handoff_truth_drift_rejection",
            "emergency_fallback_ambient_time_rejection",
            "receipt_commit_hash_mismatch_rejection",
        ],
        artifacts: &[
            "bootstrap_falsification_contract",
            "bootstrap_falsification_law",
            "bootstrap_falsification_operator",
            "valid_bootstrap_falsification_fixture",
            "golden_bootstrap_falsification_receipt",
            "execution_bootstrap_falsification_receipt",
            "deterministic_bootstrap_falsification_report",
            "bootstrap_falsification_case_pack",
        ],
        receipt: "receipts/p02/pass_0074_bootstrap_falsification.receipt",
        status: "working_slice",
    },
];

pub fn bootstrap_falsification_case_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_FALSIFICATION_CASES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_falsification_harness_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_FALSIFICATION_HARNESSES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_rejection_assertion_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_REJECTION_ASSERTIONS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_falsification_artifact_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_FALSIFICATION_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_falsification_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_FALSIFICATION_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_falsification_case_descriptor(
    id: &str,
) -> Option<&'static BootstrapFalsificationCaseDescriptor> {
    LYRALANG_BOOTSTRAP_FALSIFICATION_CASES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_falsification_harness_descriptor(
    id: &str,
) -> Option<&'static BootstrapFalsificationHarnessDescriptor> {
    LYRALANG_BOOTSTRAP_FALSIFICATION_HARNESSES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_rejection_assertion_descriptor(
    id: &str,
) -> Option<&'static BootstrapRejectionAssertionDescriptor> {
    LYRALANG_BOOTSTRAP_REJECTION_ASSERTIONS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_falsification_artifact_descriptor(
    id: &str,
) -> Option<&'static BootstrapFalsificationArtifactDescriptor> {
    LYRALANG_BOOTSTRAP_FALSIFICATION_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_falsification_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapFalsificationProofDescriptor> {
    LYRALANG_BOOTSTRAP_FALSIFICATION_PROOFS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_falsification_case_signature(
    item: &BootstrapFalsificationCaseDescriptor,
) -> String {
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
pub fn bootstrap_falsification_harness_signature(
    item: &BootstrapFalsificationHarnessDescriptor,
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
pub fn bootstrap_rejection_assertion_signature(
    item: &BootstrapRejectionAssertionDescriptor,
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
pub fn bootstrap_falsification_artifact_signature(
    item: &BootstrapFalsificationArtifactDescriptor,
) -> String {
    format!(
        "artifact:{}|owner:{}|path:{}|kind:{}|status:{}",
        item.id, item.owner_root, item.path, item.artifact_kind, item.status
    )
}
pub fn bootstrap_falsification_proof_signature(
    item: &BootstrapFalsificationProofDescriptor,
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

fn bootstrap_falsification_registry_raw_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_BOOTSTRAP_FALSIFICATION_CASES {
        rows.push(bootstrap_falsification_case_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_FALSIFICATION_HARNESSES {
        rows.push(bootstrap_falsification_harness_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_REJECTION_ASSERTIONS {
        rows.push(bootstrap_rejection_assertion_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_FALSIFICATION_ARTIFACTS {
        rows.push(bootstrap_falsification_artifact_signature(item));
    }
    for item in LYRALANG_BOOTSTRAP_FALSIFICATION_PROOFS {
        rows.push(bootstrap_falsification_proof_signature(item));
    }
    rows.sort();
    rows.join("\n")
}

pub fn bootstrap_falsification_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_falsification.registry",
        &bootstrap_falsification_registry_raw_signature(),
    )
}
pub fn bootstrap_falsification_registry_signature() -> String {
    format!(
        "{}:{}",
        LYRA_P02_BOOTSTRAP_FALSIFICATION_CARRIER,
        bootstrap_falsification_registry_hash()
    )
}
pub fn bootstrap_falsification_case_digest(id: &str) -> Option<String> {
    bootstrap_falsification_case_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_falsification.case",
            &bootstrap_falsification_case_signature(item),
        )
    })
}
pub fn bootstrap_falsification_harness_digest(id: &str) -> Option<String> {
    bootstrap_falsification_harness_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_falsification.harness",
            &bootstrap_falsification_harness_signature(item),
        )
    })
}
pub fn bootstrap_rejection_assertion_digest(id: &str) -> Option<String> {
    bootstrap_rejection_assertion_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_falsification.assertion",
            &bootstrap_rejection_assertion_signature(item),
        )
    })
}
pub fn bootstrap_falsification_artifact_digest(id: &str) -> Option<String> {
    bootstrap_falsification_artifact_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_falsification.artifact",
            &bootstrap_falsification_artifact_signature(item),
        )
    })
}
pub fn bootstrap_falsification_proof_digest(id: &str) -> Option<String> {
    bootstrap_falsification_proof_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_falsification.proof",
            &bootstrap_falsification_proof_signature(item),
        )
    })
}

pub fn bootstrap_falsification_harnesses_bind_known_cases() -> bool {
    LYRALANG_BOOTSTRAP_FALSIFICATION_HARNESSES
        .iter()
        .all(|harness| {
            harness
                .cases
                .iter()
                .all(|id| bootstrap_falsification_case_descriptor(id).is_some())
        })
}

pub fn bootstrap_rejection_assertions_bind_known_cases() -> bool {
    LYRALANG_BOOTSTRAP_REJECTION_ASSERTIONS
        .iter()
        .all(|assertion| {
            bootstrap_falsification_case_descriptor(assertion.case_id)
                .map(|case| case.expected_error == assertion.expected_error)
                .unwrap_or(false)
        })
}

pub fn bootstrap_falsification_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_FALSIFICATION_ARTIFACTS
        .iter()
        .all(|artifact| {
            !artifact.path.is_empty()
                && !artifact.path.contains("..")
                && ["lyra", "rs", "receipt"]
                    .iter()
                    .any(|suffix| artifact.path.ends_with(suffix))
                || artifact.path == "fixtures/p02/bootstrap_falsification_cases"
        })
}

pub fn bootstrap_falsification_proofs_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_FALSIFICATION_PROOFS.iter().all(|proof| {
        proof
            .cases
            .iter()
            .all(|id| bootstrap_falsification_case_descriptor(id).is_some())
            && proof
                .harnesses
                .iter()
                .all(|id| bootstrap_falsification_harness_descriptor(id).is_some())
            && proof
                .assertions
                .iter()
                .all(|id| bootstrap_rejection_assertion_descriptor(id).is_some())
            && proof
                .artifacts
                .iter()
                .all(|id| bootstrap_falsification_artifact_descriptor(id).is_some())
            && proof.receipt.starts_with("receipts/p02/")
            && proof.receipt.ends_with(".receipt")
    })
}

pub fn bootstrap_falsification_targets_all_required_domains() -> bool {
    let mut domains: Vec<&str> = LYRALANG_BOOTSTRAP_FALSIFICATION_CASES
        .iter()
        .map(|item| item.target_domain)
        .collect();
    domains.sort();
    domains.dedup();
    domains
        == vec![
            "bootstrap_trust",
            "emergency_fallback",
            "foreign_boundary",
            "host_extinction",
            "operator_handoff",
            "receipt_commit",
            "seed_runtime_law",
        ]
}

pub fn bootstrap_falsification_no_forbidden_descriptor_claims() -> bool {
    let lower = bootstrap_falsification_registry_raw_signature().to_ascii_lowercase();
    !(lower.contains("accept negative")
        || lower.contains("negative accepted")
        || lower.contains("manual only")
        || lower.contains("todo")
        || lower.contains("placeholder")
        || lower.contains("phase closed")
        || lower.contains("network required")
        || lower.contains("cloud required")
        || lower.contains("global complete"))
}
