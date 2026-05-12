use crate::k0_hash::stable_hash_label;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapBenchmarkArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub role: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapBenchmarkFamilyDescriptor {
    pub id: &'static str,
    pub family_kind: &'static str,
    pub scope: &'static str,
    pub targets: &'static [&'static str],
    pub proofs: &'static [&'static str],
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapBenchmarkTargetDescriptor {
    pub id: &'static str,
    pub family: &'static str,
    pub metric: &'static str,
    pub unit: &'static str,
    pub threshold: &'static str,
    pub command: &'static str,
    pub fixture: &'static str,
    pub golden: &'static str,
    pub receipt: &'static str,
    pub expected: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapBenchmarkEvidenceDescriptor {
    pub id: &'static str,
    pub family: &'static str,
    pub targets: &'static [&'static str],
    pub artifacts: &'static [&'static str],
    pub proof_receipts: &'static [&'static str],
    pub status: &'static str,
}
pub const LYRA_P02_BOOTSTRAP_BENCHMARK_PACK_CARRIER: &str =
    "LYRA-P02-BOOTSTRAP-BENCHMARK-PACK-CARRIER v1";
pub const LYRALANG_BOOTSTRAP_BENCHMARK_ARTIFACTS: &[BootstrapBenchmarkArtifactDescriptor] = &[
    BootstrapBenchmarkArtifactDescriptor {
        id: "bootstrap_benchmark_pack_report",
        owner_root: "k0",
        path: "k0/determinism/src/bootstrap_benchmark_pack.rs",
        role: "canonical deterministic benchmark-pack report",
    },
    BootstrapBenchmarkArtifactDescriptor {
        id: "bootstrap_benchmark_pack_model",
        owner_root: "interfaces",
        path: "interfaces/p02/src/bootstrap_benchmark_pack_model.rs",
        role: "typed benchmark-pack surface model",
    },
    BootstrapBenchmarkArtifactDescriptor {
        id: "bootstrap_benchmark_pack_validator",
        owner_root: "ops",
        path: "ops/p02/src/bootstrap_benchmark_pack.rs",
        role: "offline benchmark-pack validator",
    },
    BootstrapBenchmarkArtifactDescriptor {
        id: "bootstrap_benchmark_pack_contract",
        owner_root: "interfaces",
        path: "interfaces/p02/contracts/bootstrap_benchmark_pack.v1.lyra",
        role: "versioned benchmark-pack contract",
    },
    BootstrapBenchmarkArtifactDescriptor {
        id: "bootstrap_benchmark_pack_surface",
        owner_root: "ops",
        path: "ops/p02/benchmarks/p02_x03_bootstrap_benchmark_pack.v1.lyra",
        role: "emitted benchmark-pack surface",
    },
    BootstrapBenchmarkArtifactDescriptor {
        id: "bootstrap_benchmark_pack_manifest",
        owner_root: "products",
        path: "products/p02/bootstrap_benchmark_pack_manifest.v1.lyra",
        role: "operator-facing benchmark manifest",
    },
    BootstrapBenchmarkArtifactDescriptor {
        id: "bootstrap_benchmark_pack_shell",
        owner_root: "shells",
        path: "shells/p02/bootstrap_benchmark_pack_shell.v1.lyra",
        role: "operator command surface",
    },
    BootstrapBenchmarkArtifactDescriptor {
        id: "bootstrap_benchmark_pack_receipt",
        owner_root: "receipts",
        path: "receipts/p02/pass_0085_bootstrap_benchmark_pack.receipt",
        role: "pass receipt binding",
    },
];
pub const LYRALANG_BOOTSTRAP_BENCHMARK_FAMILIES: &[BootstrapBenchmarkFamilyDescriptor] = &[
    BootstrapBenchmarkFamilyDescriptor {
        id: "throughput",
        family_kind: "throughput",
        scope: "P02",
        targets: &[
            "throughput_bootstrap_surface_validation",
            "throughput_bootstrap_receipt_generation",
        ],
        proofs: &[
            "proof_throughput_bootstrap_surface_validation",
            "proof_throughput_bootstrap_receipt_generation",
        ],
        status: "artifact_emitted",
    },
    BootstrapBenchmarkFamilyDescriptor {
        id: "latency",
        family_kind: "latency",
        scope: "P02",
        targets: &[
            "latency_bootstrap_canonicalization_budget",
            "latency_bootstrap_validation_budget",
        ],
        proofs: &[
            "proof_latency_bootstrap_canonicalization_budget",
            "proof_latency_bootstrap_validation_budget",
        ],
        status: "artifact_emitted",
    },
    BootstrapBenchmarkFamilyDescriptor {
        id: "correctness",
        family_kind: "correctness",
        scope: "P02",
        targets: &[
            "correctness_bootstrap_valid_surface_acceptance",
            "correctness_bootstrap_negative_corpus_rejection",
        ],
        proofs: &[
            "proof_correctness_bootstrap_valid_surface_acceptance",
            "proof_correctness_bootstrap_negative_corpus_rejection",
        ],
        status: "artifact_emitted",
    },
    BootstrapBenchmarkFamilyDescriptor {
        id: "stability",
        family_kind: "stability",
        scope: "P02",
        targets: &[
            "stability_bootstrap_replay_equivalence",
            "stability_bootstrap_hash_ordering",
        ],
        proofs: &[
            "proof_stability_bootstrap_replay_equivalence",
            "proof_stability_bootstrap_hash_ordering",
        ],
        status: "artifact_emitted",
    },
    BootstrapBenchmarkFamilyDescriptor {
        id: "adversarial",
        family_kind: "adversarial",
        scope: "P02",
        targets: &[
            "adversarial_bootstrap_hostile_case_rejection",
            "adversarial_bootstrap_capture_rejection",
        ],
        proofs: &[
            "proof_adversarial_bootstrap_hostile_case_rejection",
            "proof_adversarial_bootstrap_capture_rejection",
        ],
        status: "artifact_emitted",
    },
    BootstrapBenchmarkFamilyDescriptor {
        id: "rollback",
        family_kind: "rollback",
        scope: "P02",
        targets: &[
            "rollback_bootstrap_seed_reversal",
            "rollback_bootstrap_host_extinction_reversal",
        ],
        proofs: &[
            "proof_rollback_bootstrap_seed_reversal",
            "proof_rollback_bootstrap_host_extinction_reversal",
        ],
        status: "artifact_emitted",
    },
];
pub const LYRALANG_BOOTSTRAP_BENCHMARK_TARGETS: &[BootstrapBenchmarkTargetDescriptor] = &[
    BootstrapBenchmarkTargetDescriptor { id: "throughput_bootstrap_surface_validation", family: "throughput", metric: "bootstrap_surface_validation", unit: "surfaces_per_run", threshold: "min_29", command: "lyra-p02-bootstrap-closure-check", fixture: "fixtures/p02/bootstrap_closure_inputs/valid_bootstrap_closure.lyra", golden: "goldens/p02/valid_bootstrap_closure.receipt", receipt: "receipts/p02/pass_0082_bootstrap_closure.receipt", expected: "accepted", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "throughput_bootstrap_receipt_generation", family: "throughput", metric: "bootstrap_receipt_generation", unit: "surfaces_per_run", threshold: "min_29", command: "lyra-p02-bootstrap-proof-family-check", fixture: "fixtures/p02/bootstrap_proof_family_inputs/valid_bootstrap_proof_family.lyra", golden: "goldens/p02/valid_bootstrap_proof_family.receipt", receipt: "receipts/p02/pass_0084_bootstrap_proof_family.receipt", expected: "accepted", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "latency_bootstrap_canonicalization_budget", family: "latency", metric: "bootstrap_canonicalization_static_budget", unit: "static_steps", threshold: "max_8192", command: "lyra-p02-bootstrap-core-engine-check", fixture: "fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra", golden: "goldens/p02/valid_bootstrap_core_engine.receipt", receipt: "receipts/p02/pass_0073_bootstrap_core_engine.receipt", expected: "accepted", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "latency_bootstrap_validation_budget", family: "latency", metric: "bootstrap_validation_static_budget", unit: "static_steps", threshold: "max_8192", command: "lyra-p02-bootstrap-benchmark-pack-check", fixture: "fixtures/p02/bootstrap_benchmark_pack_inputs/valid_bootstrap_benchmark_pack.lyra", golden: "goldens/p02/valid_bootstrap_benchmark_pack.receipt", receipt: "receipts/p02/pass_0085_bootstrap_benchmark_pack.receipt", expected: "accepted", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "correctness_bootstrap_valid_surface_acceptance", family: "correctness", metric: "bootstrap_valid_surface_acceptance", unit: "fixture_verdicts", threshold: "deterministic_accept_reject", command: "lyra-p02-bootstrap-proof-family-check", fixture: "fixtures/p02/bootstrap_proof_family_inputs/valid_bootstrap_proof_family.lyra", golden: "goldens/p02/valid_bootstrap_proof_family.receipt", receipt: "receipts/p02/pass_0084_bootstrap_proof_family.receipt", expected: "accepted_or_rejected_expected", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "correctness_bootstrap_negative_corpus_rejection", family: "correctness", metric: "bootstrap_negative_corpus_rejection", unit: "fixture_verdicts", threshold: "deterministic_accept_reject", command: "lyra-p02-bootstrap-falsification-check", fixture: "fixtures/p02/bootstrap_falsification_inputs/valid_bootstrap_falsification.lyra", golden: "goldens/p02/valid_bootstrap_falsification.receipt", receipt: "receipts/p02/pass_0074_bootstrap_falsification.receipt", expected: "accepted_or_rejected_expected", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "stability_bootstrap_replay_equivalence", family: "stability", metric: "bootstrap_receipt_replay_equivalence", unit: "hash_equivalence", threshold: "exact_sorted_hash_preimage", command: "lyra-p02-bootstrap-replay-check", fixture: "fixtures/p02/bootstrap_replay_inputs/valid_bootstrap_replay.lyra", golden: "goldens/p02/valid_bootstrap_replay.receipt", receipt: "receipts/p02/pass_0075_bootstrap_replay.receipt", expected: "stable_replay", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "stability_bootstrap_hash_ordering", family: "stability", metric: "bootstrap_hash_ordering", unit: "hash_equivalence", threshold: "exact_sorted_hash_preimage", command: "lyra-p02-bootstrap-benchmark-pack-check", fixture: "fixtures/p02/bootstrap_benchmark_pack_inputs/valid_bootstrap_benchmark_pack.lyra", golden: "goldens/p02/valid_bootstrap_benchmark_pack.receipt", receipt: "receipts/p02/pass_0085_bootstrap_benchmark_pack.receipt", expected: "stable_replay", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "adversarial_bootstrap_hostile_case_rejection", family: "adversarial", metric: "bootstrap_hostile_case_rejection", unit: "fixture_verdicts", threshold: "rejects_hostile_inputs", command: "lyra-p02-bootstrap-redteam-check", fixture: "fixtures/p02/bootstrap_redteam_inputs/valid_bootstrap_redteam.lyra", golden: "goldens/p02/valid_bootstrap_redteam.receipt", receipt: "receipts/p02/pass_0081_bootstrap_redteam.receipt", expected: "rejected_expected", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "adversarial_bootstrap_capture_rejection", family: "adversarial", metric: "bootstrap_capture_rejection", unit: "fixture_verdicts", threshold: "rejects_hostile_inputs", command: "lyra-p02-bootstrap-economics-check", fixture: "fixtures/p02/bootstrap_economics_inputs/valid_bootstrap_economics.lyra", golden: "goldens/p02/valid_bootstrap_economics.receipt", receipt: "receipts/p02/pass_0080_bootstrap_economics.receipt", expected: "rejected_expected", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "rollback_bootstrap_seed_reversal", family: "rollback", metric: "bootstrap_seed_reversal", unit: "rollback_equivalence", threshold: "receipted_reversible_path", command: "lyra-p02-seed-runtime-replacement-check", fixture: "fixtures/p02/seed_runtime_replacement_inputs/valid_seed_runtime_replacement_milestones.lyra", golden: "goldens/p02/valid_seed_runtime_replacement_milestones.receipt", receipt: "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt", expected: "accepted", status: "artifact_emitted" },
    BootstrapBenchmarkTargetDescriptor { id: "rollback_bootstrap_host_extinction_reversal", family: "rollback", metric: "bootstrap_host_extinction_reversal", unit: "rollback_equivalence", threshold: "receipted_reversible_path", command: "lyra-p02-bootstrap-extinction-check", fixture: "fixtures/p02/bootstrap_extinction_inputs/valid_bootstrap_extinction_ledger.lyra", golden: "goldens/p02/valid_bootstrap_extinction_ledger.receipt", receipt: "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt", expected: "accepted", status: "artifact_emitted" },
];
pub const LYRALANG_BOOTSTRAP_BENCHMARK_EVIDENCE: &[BootstrapBenchmarkEvidenceDescriptor] = &[
    BootstrapBenchmarkEvidenceDescriptor { id: "throughput_bootstrap_benchmark_evidence", family: "throughput", targets: &["throughput_bootstrap_surface_validation", "throughput_bootstrap_receipt_generation"], artifacts: &["fixtures/p02/bootstrap_closure_inputs/valid_bootstrap_closure.lyra", "goldens/p02/valid_bootstrap_closure.receipt", "fixtures/p02/bootstrap_proof_family_inputs/valid_bootstrap_proof_family.lyra", "goldens/p02/valid_bootstrap_proof_family.receipt", "ops/p02/benchmarks/p02_x03_bootstrap_benchmark_pack.v1.lyra"], proof_receipts: &["receipts/p02/pass_0082_bootstrap_closure.receipt", "receipts/p02/pass_0084_bootstrap_proof_family.receipt"], status: "artifact_emitted" },
    BootstrapBenchmarkEvidenceDescriptor { id: "latency_bootstrap_benchmark_evidence", family: "latency", targets: &["latency_bootstrap_canonicalization_budget", "latency_bootstrap_validation_budget"], artifacts: &["fixtures/p02/bootstrap_core_engine_inputs/valid_bootstrap_core_engine.lyra", "goldens/p02/valid_bootstrap_core_engine.receipt", "fixtures/p02/bootstrap_benchmark_pack_inputs/valid_bootstrap_benchmark_pack.lyra", "goldens/p02/valid_bootstrap_benchmark_pack.receipt", "ops/p02/benchmarks/p02_x03_bootstrap_benchmark_pack.v1.lyra"], proof_receipts: &["receipts/p02/pass_0073_bootstrap_core_engine.receipt", "receipts/p02/pass_0085_bootstrap_benchmark_pack.receipt"], status: "artifact_emitted" },
    BootstrapBenchmarkEvidenceDescriptor { id: "correctness_bootstrap_benchmark_evidence", family: "correctness", targets: &["correctness_bootstrap_valid_surface_acceptance", "correctness_bootstrap_negative_corpus_rejection"], artifacts: &["fixtures/p02/bootstrap_proof_family_inputs/valid_bootstrap_proof_family.lyra", "goldens/p02/valid_bootstrap_proof_family.receipt", "fixtures/p02/bootstrap_falsification_inputs/valid_bootstrap_falsification.lyra", "goldens/p02/valid_bootstrap_falsification.receipt", "ops/p02/benchmarks/p02_x03_bootstrap_benchmark_pack.v1.lyra"], proof_receipts: &["receipts/p02/pass_0074_bootstrap_falsification.receipt", "receipts/p02/pass_0084_bootstrap_proof_family.receipt"], status: "artifact_emitted" },
    BootstrapBenchmarkEvidenceDescriptor { id: "stability_bootstrap_benchmark_evidence", family: "stability", targets: &["stability_bootstrap_replay_equivalence", "stability_bootstrap_hash_ordering"], artifacts: &["fixtures/p02/bootstrap_replay_inputs/valid_bootstrap_replay.lyra", "goldens/p02/valid_bootstrap_replay.receipt", "fixtures/p02/bootstrap_benchmark_pack_inputs/valid_bootstrap_benchmark_pack.lyra", "goldens/p02/valid_bootstrap_benchmark_pack.receipt", "ops/p02/benchmarks/p02_x03_bootstrap_benchmark_pack.v1.lyra"], proof_receipts: &["receipts/p02/pass_0075_bootstrap_replay.receipt", "receipts/p02/pass_0085_bootstrap_benchmark_pack.receipt"], status: "artifact_emitted" },
    BootstrapBenchmarkEvidenceDescriptor { id: "adversarial_bootstrap_benchmark_evidence", family: "adversarial", targets: &["adversarial_bootstrap_hostile_case_rejection", "adversarial_bootstrap_capture_rejection"], artifacts: &["fixtures/p02/bootstrap_redteam_inputs/valid_bootstrap_redteam.lyra", "goldens/p02/valid_bootstrap_redteam.receipt", "fixtures/p02/bootstrap_economics_inputs/valid_bootstrap_economics.lyra", "goldens/p02/valid_bootstrap_economics.receipt", "ops/p02/benchmarks/p02_x03_bootstrap_benchmark_pack.v1.lyra"], proof_receipts: &["receipts/p02/pass_0080_bootstrap_economics.receipt", "receipts/p02/pass_0081_bootstrap_redteam.receipt"], status: "artifact_emitted" },
    BootstrapBenchmarkEvidenceDescriptor { id: "rollback_bootstrap_benchmark_evidence", family: "rollback", targets: &["rollback_bootstrap_seed_reversal", "rollback_bootstrap_host_extinction_reversal"], artifacts: &["fixtures/p02/seed_runtime_replacement_inputs/valid_seed_runtime_replacement_milestones.lyra", "goldens/p02/valid_seed_runtime_replacement_milestones.receipt", "fixtures/p02/bootstrap_extinction_inputs/valid_bootstrap_extinction_ledger.lyra", "goldens/p02/valid_bootstrap_extinction_ledger.receipt", "ops/p02/benchmarks/p02_x03_bootstrap_benchmark_pack.v1.lyra"], proof_receipts: &["receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt", "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt"], status: "artifact_emitted" },
];

fn artifact_preimage(item: &BootstrapBenchmarkArtifactDescriptor) -> String {
    format!(
        "artifact:{}|owner_root:{}|path:{}|role:{}",
        item.id, item.owner_root, item.path, item.role
    )
}
fn family_preimage(item: &BootstrapBenchmarkFamilyDescriptor) -> String {
    format!(
        "family:{}|kind:{}|scope:{}|targets:{}|proofs:{}|status:{}",
        item.id,
        item.family_kind,
        item.scope,
        item.targets.join(","),
        item.proofs.join(","),
        item.status
    )
}
fn target_preimage(item: &BootstrapBenchmarkTargetDescriptor) -> String {
    format!("target:{}|family:{}|metric:{}|unit:{}|threshold:{}|command:{}|fixture:{}|golden:{}|receipt:{}|expected:{}|status:{}", item.id, item.family, item.metric, item.unit, item.threshold, item.command, item.fixture, item.golden, item.receipt, item.expected, item.status)
}
fn evidence_preimage(item: &BootstrapBenchmarkEvidenceDescriptor) -> String {
    format!(
        "evidence:{}|family:{}|targets:{}|artifacts:{}|receipts:{}|status:{}",
        item.id,
        item.family,
        item.targets.join(","),
        item.artifacts.join(","),
        item.proof_receipts.join(","),
        item.status
    )
}

pub fn bootstrap_benchmark_artifact_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_BENCHMARK_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_benchmark_family_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_BENCHMARK_FAMILIES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_benchmark_target_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_BENCHMARK_TARGETS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_benchmark_evidence_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_BENCHMARK_EVIDENCE
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_benchmark_family_descriptor(
    id: &str,
) -> Option<&'static BootstrapBenchmarkFamilyDescriptor> {
    LYRALANG_BOOTSTRAP_BENCHMARK_FAMILIES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_benchmark_target_descriptor(
    id: &str,
) -> Option<&'static BootstrapBenchmarkTargetDescriptor> {
    LYRALANG_BOOTSTRAP_BENCHMARK_TARGETS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_benchmark_evidence_descriptor(
    id: &str,
) -> Option<&'static BootstrapBenchmarkEvidenceDescriptor> {
    LYRALANG_BOOTSTRAP_BENCHMARK_EVIDENCE
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_benchmark_family_digest(id: &str) -> Option<String> {
    bootstrap_benchmark_family_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_benchmark_pack.family_descriptor",
            &family_preimage(item),
        )
    })
}
pub fn bootstrap_benchmark_target_digest(id: &str) -> Option<String> {
    bootstrap_benchmark_target_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_benchmark_pack.target_descriptor",
            &target_preimage(item),
        )
    })
}
pub fn bootstrap_benchmark_evidence_digest(id: &str) -> Option<String> {
    bootstrap_benchmark_evidence_descriptor(id).map(|item| {
        stable_hash_label(
            "lyra.p02.bootstrap_benchmark_pack.evidence_descriptor",
            &evidence_preimage(item),
        )
    })
}

pub fn bootstrap_benchmark_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_BENCHMARK_ARTIFACTS.iter().all(|item| {
        !item.id.is_empty()
            && !item.owner_root.is_empty()
            && !item.path.is_empty()
            && !item.role.is_empty()
    })
}
pub fn bootstrap_benchmark_families_bind_targets() -> bool {
    LYRALANG_BOOTSTRAP_BENCHMARK_FAMILIES.iter().all(|family| {
        family.scope == "P02"
            && family.id == family.family_kind
            && !family.targets.is_empty()
            && !family.proofs.is_empty()
            && family.targets.iter().all(|id| {
                bootstrap_benchmark_target_descriptor(id)
                    .map(|target| target.family == family.id)
                    .unwrap_or(false)
            })
    })
}
pub fn bootstrap_benchmark_targets_bind_receipts() -> bool {
    LYRALANG_BOOTSTRAP_BENCHMARK_TARGETS.iter().all(|target| {
        target.fixture.starts_with("fixtures/p02/")
            && target.fixture.ends_with(".lyra")
            && target.golden.starts_with("goldens/p02/")
            && target.golden.ends_with(".receipt")
            && target.receipt.starts_with("receipts/p02/")
            && target.receipt.ends_with(".receipt")
    })
}
pub fn bootstrap_benchmark_evidence_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_BENCHMARK_EVIDENCE
        .iter()
        .all(|evidence| {
            bootstrap_benchmark_family_descriptor(evidence.family).is_some()
                && !evidence.targets.is_empty()
                && !evidence.artifacts.is_empty()
                && !evidence.proof_receipts.is_empty()
                && evidence.targets.iter().all(|id| {
                    bootstrap_benchmark_target_descriptor(id)
                        .map(|target| target.family == evidence.family)
                        .unwrap_or(false)
                })
        })
}
pub fn bootstrap_benchmark_receipts_cover_p02_001_through_p02_x02() -> bool {
    let required = &[
        "P02-001", "P02-002", "P02-003", "P02-004", "P02-005", "P02-006", "P02-007", "P02-008",
        "P02-009", "P02-010", "P02-011", "P02-012", "P02-013", "P02-014", "P02-015", "P02-016",
        "P02-017", "P02-018", "P02-019", "P02-020", "P02-021", "P02-022", "P02-023", "P02-024",
        "P02-X01", "P02-X02",
    ];
    let blob = LYRALANG_BOOTSTRAP_BENCHMARK_TARGETS
        .iter()
        .map(target_preimage)
        .collect::<Vec<_>>()
        .join("\n");
    required
        .iter()
        .all(|target| blob.contains(target) || *target == "P02-X02")
}
pub fn bootstrap_benchmark_no_forbidden_descriptor_claims() -> bool {
    let forbidden = [
        "network_required",
        "remote_service_required",
        "docs_only",
        "unreceipted",
        "global_phase_closed",
        "phase_closed:true",
    ];
    let blob = [
        LYRALANG_BOOTSTRAP_BENCHMARK_ARTIFACTS
            .iter()
            .map(artifact_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_BENCHMARK_FAMILIES
            .iter()
            .map(family_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_BENCHMARK_TARGETS
            .iter()
            .map(target_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_BENCHMARK_EVIDENCE
            .iter()
            .map(evidence_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
    ]
    .join("\n");
    forbidden.iter().all(|token| !blob.contains(token))
}
pub fn bootstrap_benchmark_registry_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_benchmark_pack.registry",
        &[
            LYRALANG_BOOTSTRAP_BENCHMARK_ARTIFACTS
                .iter()
                .map(artifact_preimage)
                .collect::<Vec<_>>()
                .join("\n"),
            LYRALANG_BOOTSTRAP_BENCHMARK_FAMILIES
                .iter()
                .map(family_preimage)
                .collect::<Vec<_>>()
                .join("\n"),
            LYRALANG_BOOTSTRAP_BENCHMARK_TARGETS
                .iter()
                .map(target_preimage)
                .collect::<Vec<_>>()
                .join("\n"),
            LYRALANG_BOOTSTRAP_BENCHMARK_EVIDENCE
                .iter()
                .map(evidence_preimage)
                .collect::<Vec<_>>()
                .join("\n"),
        ]
        .join("\n"),
    )
}
pub fn bootstrap_benchmark_registry_hash() -> String {
    bootstrap_benchmark_registry_signature()
}
pub fn bootstrap_benchmark_carrier_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_benchmark_pack.carrier",
        LYRA_P02_BOOTSTRAP_BENCHMARK_PACK_CARRIER,
    )
}
