use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOperatorCommandDescriptor {
    pub id: &'static str,
    pub binary: &'static str,
    pub surface: &'static str,
    pub input: &'static str,
    pub output: &'static str,
    pub receipts: &'static [&'static str],
    pub roles: &'static [&'static str],
    pub targets: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOperatorWorkflowDescriptor {
    pub id: &'static str,
    pub order: &'static str,
    pub commands: &'static [&'static str],
    pub targets: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOperatorExampleDescriptor {
    pub id: &'static str,
    pub path: &'static str,
    pub commands: &'static [&'static str],
    pub expected_receipts: &'static [&'static str],
    pub expected_verdict: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOperatorAcceptanceGateDescriptor {
    pub id: &'static str,
    pub workflow: &'static str,
    pub required_receipts: &'static [&'static str],
    pub required_examples: &'static [&'static str],
    pub decision: &'static str,
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOperatorProofDescriptor {
    pub id: &'static str,
    pub scope: &'static str,
    pub commands: &'static [&'static str],
    pub workflows: &'static [&'static str],
    pub examples: &'static [&'static str],
    pub gates: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub forbids: &'static [&'static str],
    pub status: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOperatorArtifactDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub artifact_kind: &'static str,
    pub commands: &'static [&'static str],
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_OPERATOR_INTERFACE_CARRIER: &str =
    "lyra.p02.bootstrap_operator_interface.carrier.v1";

pub const LYRALANG_BOOTSTRAP_OPERATOR_COMMANDS: &[BootstrapOperatorCommandDescriptor] = &[
    BootstrapOperatorCommandDescriptor { id: "bootstrap_trust_status", binary: "lyra-p02-bootstrap-interface-check", surface: "bootstrap_trust", input: "ops/p02/control/truth_snapshot.v1.lyra", output: "receipts/p02/bootstrap_interface/bootstrap_trust_status.receipt", receipts: &["receipts/p02/pass_0059_bootstrap_surface_inventory.receipt", "receipts/p02/pass_0075_bootstrap_replay.receipt"], roles: &["developer", "operator"], targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"], status: "artifact_emitted" },
    BootstrapOperatorCommandDescriptor { id: "seed_runtime_law_verify", binary: "lyra-p02-bootstrap-interface-check", surface: "seed_runtime_law", input: "interfaces/p02/contracts/seed_runtime_contracts.v1.lyra", output: "receipts/p02/bootstrap_interface/seed_runtime_law_verify.receipt", receipts: &["receipts/p02/pass_0061_seed_runtime_contracts.receipt", "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt"], roles: &["developer", "operator"], targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"], status: "artifact_emitted" },
    BootstrapOperatorCommandDescriptor { id: "host_extinction_audit", binary: "lyra-p02-bootstrap-interface-check", surface: "host_extinction", input: "ops/p02/extinction/bootstrap_extinction_ledger.v1.lyra", output: "receipts/p02/bootstrap_interface/host_extinction_audit.receipt", receipts: &["receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt", "receipts/p02/pass_0070_foreign_surface_closure.receipt"], roles: &["developer", "operator", "auditor"], targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"], status: "artifact_emitted" },
    BootstrapOperatorCommandDescriptor { id: "bootstrap_target_preflight", binary: "lyra-p02-bootstrap-interface-check", surface: "cross_target_preflight", input: "ops/p02/target_matrix/bootstrap_target_matrix.v1.lyra", output: "receipts/p02/bootstrap_interface/bootstrap_target_preflight.receipt", receipts: &["receipts/p02/pass_0064_bootstrap_target_matrix.receipt", "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt"], roles: &["developer", "operator"], targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"], status: "artifact_emitted" },
    BootstrapOperatorCommandDescriptor { id: "bootstrap_falsification_run", binary: "lyra-p02-bootstrap-falsification-check", surface: "negative_corpus", input: "fixtures/p02/bootstrap_falsification_inputs/valid_bootstrap_falsification.lyra", output: "receipts/p02/bootstrap_interface/bootstrap_falsification_run.receipt", receipts: &["receipts/p02/pass_0074_bootstrap_falsification.receipt", "receipts/p02/bootstrap_falsification/bootstrap_falsification_suite.report"], roles: &["developer", "auditor"], targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"], status: "artifact_emitted" },
    BootstrapOperatorCommandDescriptor { id: "bootstrap_replay_witness_run", binary: "lyra-p02-bootstrap-replay-check", surface: "replay_witness", input: "fixtures/p02/bootstrap_replay_inputs/valid_bootstrap_replay.lyra", output: "receipts/p02/bootstrap_interface/bootstrap_replay_witness_run.receipt", receipts: &["receipts/p02/pass_0075_bootstrap_replay.receipt", "receipts/p02/bootstrap_replay/bootstrap_replay_suite.report"], roles: &["developer", "operator", "auditor"], targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"], status: "artifact_emitted" },
    BootstrapOperatorCommandDescriptor { id: "bootstrap_interface_report", binary: "lyra-p02-bootstrap-interface-check", surface: "interface_report", input: "ops/p02/interface/bootstrap_operator_interface.v1.lyra", output: "receipts/p02/bootstrap_interface/bootstrap_interface_report.receipt", receipts: &["receipts/p02/pass_0076_bootstrap_operator_interface.receipt", "receipts/p02/bootstrap_interface/bootstrap_operator_interface_suite.report"], roles: &["developer", "operator", "auditor"], targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"], status: "working_slice" },
    BootstrapOperatorCommandDescriptor { id: "bootstrap_operator_acceptance", binary: "lyra-p02-bootstrap-interface-check", surface: "operator_acceptance", input: "fixtures/p02/bootstrap_operator_interface_inputs/valid_bootstrap_operator_interface.lyra", output: "receipts/p02/bootstrap_interface/bootstrap_operator_acceptance.receipt", receipts: &["receipts/p02/pass_0076_bootstrap_operator_interface.receipt", "goldens/p02/valid_bootstrap_operator_interface.receipt"], roles: &["operator", "auditor"], targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"], status: "working_slice" },
];

pub const LYRALANG_BOOTSTRAP_OPERATOR_WORKFLOWS: &[BootstrapOperatorWorkflowDescriptor] = &[
    BootstrapOperatorWorkflowDescriptor {
        id: "developer_bootstrap_trust_review",
        order: "001",
        commands: &["bootstrap_trust_status", "bootstrap_interface_report"],
        targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"],
        examples: &["bootstrap_trust_cli_example"],
        forbids: &["ambient_authority", "manual_only_review"],
        status: "artifact_emitted",
    },
    BootstrapOperatorWorkflowDescriptor {
        id: "operator_seed_runtime_replacement",
        order: "002",
        commands: &[
            "seed_runtime_law_verify",
            "bootstrap_target_preflight",
            "bootstrap_replay_witness_run",
        ],
        targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"],
        examples: &["seed_runtime_law_cli_example"],
        forbids: &["network_runtime_gate", "probabilistic_runtime_selection"],
        status: "artifact_emitted",
    },
    BootstrapOperatorWorkflowDescriptor {
        id: "host_extinction_audit_flow",
        order: "003",
        commands: &[
            "host_extinction_audit",
            "bootstrap_target_preflight",
            "bootstrap_falsification_run",
        ],
        targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"],
        examples: &["host_extinction_audit_example"],
        forbids: &["unledgered_foreign_surface", "retirement_without_receipt"],
        status: "artifact_emitted",
    },
    BootstrapOperatorWorkflowDescriptor {
        id: "cross_target_preflight_flow",
        order: "004",
        commands: &[
            "bootstrap_target_preflight",
            "bootstrap_replay_witness_run",
            "bootstrap_interface_report",
        ],
        targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"],
        examples: &["cross_target_preflight_example"],
        forbids: &["target_specific_truth_drift", "ambient_time_preflight"],
        status: "artifact_emitted",
    },
    BootstrapOperatorWorkflowDescriptor {
        id: "bootstrap_operator_handoff_flow",
        order: "005",
        commands: &[
            "bootstrap_trust_status",
            "seed_runtime_law_verify",
            "host_extinction_audit",
            "bootstrap_falsification_run",
            "bootstrap_replay_witness_run",
            "bootstrap_operator_acceptance",
        ],
        targets: &["linux", "windows", "android", "macos", "wasm", "baremetal"],
        examples: &["operator_acceptance_example"],
        forbids: &["phase_closed", "unreceipted_handoff"],
        status: "working_slice",
    },
];

pub const LYRALANG_BOOTSTRAP_OPERATOR_EXAMPLES: &[BootstrapOperatorExampleDescriptor] = &[
    BootstrapOperatorExampleDescriptor {
        id: "bootstrap_trust_cli_example",
        path: "examples/p02/bootstrap_operator_interface/bootstrap_trust_status.lyra",
        commands: &["bootstrap_trust_status"],
        expected_receipts: &["receipts/p02/bootstrap_interface/bootstrap_trust_status.receipt"],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapOperatorExampleDescriptor {
        id: "seed_runtime_law_cli_example",
        path: "examples/p02/bootstrap_operator_interface/seed_runtime_law_verify.lyra",
        commands: &["seed_runtime_law_verify"],
        expected_receipts: &["receipts/p02/bootstrap_interface/seed_runtime_law_verify.receipt"],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapOperatorExampleDescriptor {
        id: "host_extinction_audit_example",
        path: "examples/p02/bootstrap_operator_interface/host_extinction_audit.lyra",
        commands: &["host_extinction_audit"],
        expected_receipts: &["receipts/p02/bootstrap_interface/host_extinction_audit.receipt"],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapOperatorExampleDescriptor {
        id: "cross_target_preflight_example",
        path: "examples/p02/bootstrap_operator_interface/cross_target_preflight.lyra",
        commands: &["bootstrap_target_preflight", "bootstrap_replay_witness_run"],
        expected_receipts: &[
            "receipts/p02/bootstrap_interface/bootstrap_target_preflight.receipt",
            "receipts/p02/bootstrap_interface/bootstrap_replay_witness_run.receipt",
        ],
        expected_verdict: "accepted",
        status: "artifact_emitted",
    },
    BootstrapOperatorExampleDescriptor {
        id: "operator_acceptance_example",
        path: "examples/p02/bootstrap_operator_interface/operator_acceptance.lyra",
        commands: &[
            "bootstrap_operator_acceptance",
            "bootstrap_interface_report",
        ],
        expected_receipts: &[
            "receipts/p02/bootstrap_interface/bootstrap_operator_acceptance.receipt",
            "receipts/p02/bootstrap_interface/bootstrap_interface_report.receipt",
        ],
        expected_verdict: "accepted",
        status: "working_slice",
    },
];

pub const LYRALANG_BOOTSTRAP_OPERATOR_GATES: &[BootstrapOperatorAcceptanceGateDescriptor] = &[
    BootstrapOperatorAcceptanceGateDescriptor {
        id: "bootstrap_trust_operator_gate",
        workflow: "developer_bootstrap_trust_review",
        required_receipts: &[
            "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
        ],
        required_examples: &["bootstrap_trust_cli_example"],
        decision: "admit_working_slice",
        forbids: &["operator_override_constitution", "ambient_authority"],
        status: "artifact_emitted",
    },
    BootstrapOperatorAcceptanceGateDescriptor {
        id: "seed_runtime_operator_gate",
        workflow: "operator_seed_runtime_replacement",
        required_receipts: &[
            "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
            "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        ],
        required_examples: &["seed_runtime_law_cli_example"],
        decision: "require_replay_receipts",
        forbids: &["network_runtime_gate", "probabilistic_runtime_selection"],
        status: "artifact_emitted",
    },
    BootstrapOperatorAcceptanceGateDescriptor {
        id: "host_extinction_operator_gate",
        workflow: "host_extinction_audit_flow",
        required_receipts: &[
            "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
            "receipts/p02/pass_0070_foreign_surface_closure.receipt",
        ],
        required_examples: &["host_extinction_audit_example"],
        decision: "require_replay_receipts",
        forbids: &["unledgered_foreign_surface", "retirement_without_receipt"],
        status: "artifact_emitted",
    },
    BootstrapOperatorAcceptanceGateDescriptor {
        id: "cross_target_operator_gate",
        workflow: "cross_target_preflight_flow",
        required_receipts: &[
            "receipts/p02/pass_0064_bootstrap_target_matrix.receipt",
            "receipts/p02/pass_0063_host_boundary_challenge_suites.receipt",
        ],
        required_examples: &["cross_target_preflight_example"],
        decision: "admit_working_slice",
        forbids: &["target_specific_truth_drift", "ambient_time_preflight"],
        status: "artifact_emitted",
    },
    BootstrapOperatorAcceptanceGateDescriptor {
        id: "p02_operator_acceptance_gate",
        workflow: "bootstrap_operator_handoff_flow",
        required_receipts: &[
            "receipts/p02/pass_0074_bootstrap_falsification.receipt",
            "receipts/p02/pass_0075_bootstrap_replay.receipt",
            "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
        ],
        required_examples: &["operator_acceptance_example"],
        decision: "block_phase_closure",
        forbids: &["phase_closed", "unreceipted_handoff"],
        status: "working_slice",
    },
];

pub const LYRALANG_BOOTSTRAP_OPERATOR_PROOFS: &[BootstrapOperatorProofDescriptor] = &[
    BootstrapOperatorProofDescriptor {
        id: "bootstrap_trust_interface_proof",
        scope: "interface",
        commands: &["bootstrap_trust_status"],
        workflows: &["developer_bootstrap_trust_review"],
        examples: &["bootstrap_trust_cli_example"],
        gates: &["bootstrap_trust_operator_gate"],
        receipts: &["receipts/p02/bootstrap_interface/bootstrap_trust_status.receipt"],
        forbids: &["ambient_authority", "manual_only_review"],
        status: "artifact_emitted",
    },
    BootstrapOperatorProofDescriptor {
        id: "seed_runtime_operator_interface_proof",
        scope: "workflow",
        commands: &["seed_runtime_law_verify", "bootstrap_target_preflight"],
        workflows: &["operator_seed_runtime_replacement"],
        examples: &["seed_runtime_law_cli_example"],
        gates: &["seed_runtime_operator_gate"],
        receipts: &[
            "receipts/p02/bootstrap_interface/seed_runtime_law_verify.receipt",
            "receipts/p02/bootstrap_interface/bootstrap_target_preflight.receipt",
        ],
        forbids: &["network_runtime_gate", "probabilistic_runtime_selection"],
        status: "artifact_emitted",
    },
    BootstrapOperatorProofDescriptor {
        id: "host_extinction_operator_interface_proof",
        scope: "workflow",
        commands: &["host_extinction_audit", "bootstrap_falsification_run"],
        workflows: &["host_extinction_audit_flow"],
        examples: &["host_extinction_audit_example"],
        gates: &["host_extinction_operator_gate"],
        receipts: &[
            "receipts/p02/bootstrap_interface/host_extinction_audit.receipt",
            "receipts/p02/bootstrap_interface/bootstrap_falsification_run.receipt",
        ],
        forbids: &["unledgered_foreign_surface", "retirement_without_receipt"],
        status: "artifact_emitted",
    },
    BootstrapOperatorProofDescriptor {
        id: "cross_target_interface_proof",
        scope: "handoff",
        commands: &[
            "bootstrap_target_preflight",
            "bootstrap_replay_witness_run",
            "bootstrap_interface_report",
        ],
        workflows: &["cross_target_preflight_flow"],
        examples: &["cross_target_preflight_example"],
        gates: &["cross_target_operator_gate"],
        receipts: &[
            "receipts/p02/bootstrap_interface/bootstrap_target_preflight.receipt",
            "receipts/p02/bootstrap_interface/bootstrap_replay_witness_run.receipt",
            "receipts/p02/bootstrap_interface/bootstrap_interface_report.receipt",
        ],
        forbids: &["target_specific_truth_drift", "ambient_time_preflight"],
        status: "artifact_emitted",
    },
    BootstrapOperatorProofDescriptor {
        id: "p02_operator_acceptance_proof",
        scope: "gate",
        commands: &[
            "bootstrap_trust_status",
            "seed_runtime_law_verify",
            "host_extinction_audit",
            "bootstrap_falsification_run",
            "bootstrap_replay_witness_run",
            "bootstrap_operator_acceptance",
        ],
        workflows: &["bootstrap_operator_handoff_flow"],
        examples: &["operator_acceptance_example"],
        gates: &["p02_operator_acceptance_gate"],
        receipts: &[
            "receipts/p02/pass_0076_bootstrap_operator_interface.receipt",
            "goldens/p02/valid_bootstrap_operator_interface.receipt",
        ],
        forbids: &["phase_closed", "unreceipted_handoff"],
        status: "working_slice",
    },
];

pub const LYRALANG_BOOTSTRAP_OPERATOR_ARTIFACTS: &[BootstrapOperatorArtifactDescriptor] = &[
    BootstrapOperatorArtifactDescriptor { id: "bootstrap_operator_interface_contract", owner_root: "interfaces", path: "interfaces/p02/contracts/bootstrap_operator_interface.v1.lyra", artifact_kind: "contract", commands: &["bootstrap_interface_report"], status: "artifact_emitted" },
    BootstrapOperatorArtifactDescriptor { id: "bootstrap_operator_interface_law", owner_root: "ops", path: "ops/p02/interface/bootstrap_operator_interface.v1.lyra", artifact_kind: "law_manifest", commands: &["bootstrap_interface_report"], status: "artifact_emitted" },
    BootstrapOperatorArtifactDescriptor { id: "bootstrap_operator_interface_shell", owner_root: "shells", path: "shells/p02/bootstrap_operator_interface_shell.v1.lyra", artifact_kind: "operator_shell", commands: &["bootstrap_trust_status", "seed_runtime_law_verify", "host_extinction_audit", "bootstrap_operator_acceptance"], status: "artifact_emitted" },
    BootstrapOperatorArtifactDescriptor { id: "bootstrap_operator_interface_binary", owner_root: "src", path: "src/bin/lyra-p02-bootstrap-interface-check.rs", artifact_kind: "binary", commands: &["bootstrap_interface_report"], status: "artifact_emitted" },
    BootstrapOperatorArtifactDescriptor { id: "valid_bootstrap_operator_interface_fixture", owner_root: "fixtures", path: "fixtures/p02/bootstrap_operator_interface_inputs/valid_bootstrap_operator_interface.lyra", artifact_kind: "valid_fixture", commands: &["bootstrap_interface_report"], status: "artifact_emitted" },
    BootstrapOperatorArtifactDescriptor { id: "bootstrap_operator_interface_examples", owner_root: "examples", path: "examples/p02/bootstrap_operator_interface", artifact_kind: "example_pack", commands: &["bootstrap_trust_status", "seed_runtime_law_verify", "host_extinction_audit", "bootstrap_operator_acceptance"], status: "artifact_emitted" },
    BootstrapOperatorArtifactDescriptor { id: "golden_bootstrap_operator_interface_receipt", owner_root: "goldens", path: "goldens/p02/valid_bootstrap_operator_interface.receipt", artifact_kind: "golden_receipt", commands: &["bootstrap_interface_report"], status: "artifact_emitted" },
    BootstrapOperatorArtifactDescriptor { id: "execution_bootstrap_operator_interface_receipt", owner_root: "receipts", path: "receipts/p02/pass_0076_bootstrap_operator_interface.receipt", artifact_kind: "execution_receipt", commands: &["bootstrap_interface_report"], status: "artifact_emitted" },
    BootstrapOperatorArtifactDescriptor { id: "deterministic_bootstrap_operator_interface_report", owner_root: "k0", path: "k0/determinism/src/bootstrap_operator_interface.rs", artifact_kind: "report", commands: &["bootstrap_interface_report"], status: "artifact_emitted" },
];

pub fn bootstrap_operator_command_descriptor(
    id: &str,
) -> Option<&'static BootstrapOperatorCommandDescriptor> {
    LYRALANG_BOOTSTRAP_OPERATOR_COMMANDS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_operator_workflow_descriptor(
    id: &str,
) -> Option<&'static BootstrapOperatorWorkflowDescriptor> {
    LYRALANG_BOOTSTRAP_OPERATOR_WORKFLOWS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_operator_example_descriptor(
    id: &str,
) -> Option<&'static BootstrapOperatorExampleDescriptor> {
    LYRALANG_BOOTSTRAP_OPERATOR_EXAMPLES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_operator_gate_descriptor(
    id: &str,
) -> Option<&'static BootstrapOperatorAcceptanceGateDescriptor> {
    LYRALANG_BOOTSTRAP_OPERATOR_GATES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_operator_proof_descriptor(
    id: &str,
) -> Option<&'static BootstrapOperatorProofDescriptor> {
    LYRALANG_BOOTSTRAP_OPERATOR_PROOFS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_operator_artifact_descriptor(
    id: &str,
) -> Option<&'static BootstrapOperatorArtifactDescriptor> {
    LYRALANG_BOOTSTRAP_OPERATOR_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_operator_command_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_OPERATOR_COMMANDS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_operator_workflow_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_OPERATOR_WORKFLOWS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_operator_example_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_OPERATOR_EXAMPLES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_operator_gate_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_OPERATOR_GATES
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_operator_proof_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_OPERATOR_PROOFS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn bootstrap_operator_artifact_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_OPERATOR_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect()
}

pub fn bootstrap_operator_command_digest(id: &str) -> Option<String> {
    let item = bootstrap_operator_command_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_operator_interface.command_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            item.id,
            item.binary,
            item.surface,
            item.input,
            item.output,
            item.receipts.join(","),
            item.roles.join(","),
            item.targets.join(","),
            item.status
        ),
    ))
}
pub fn bootstrap_operator_workflow_digest(id: &str) -> Option<String> {
    let item = bootstrap_operator_workflow_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_operator_interface.workflow_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}|{}",
            item.id,
            item.order,
            item.commands.join(","),
            item.targets.join(","),
            item.examples.join(","),
            item.forbids.join(","),
            item.status
        ),
    ))
}
pub fn bootstrap_operator_example_digest(id: &str) -> Option<String> {
    let item = bootstrap_operator_example_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_operator_interface.example_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}",
            item.id,
            item.path,
            item.commands.join(","),
            item.expected_receipts.join(","),
            item.expected_verdict,
            item.status
        ),
    ))
}
pub fn bootstrap_operator_gate_digest(id: &str) -> Option<String> {
    let item = bootstrap_operator_gate_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_operator_interface.gate_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}|{}",
            item.id,
            item.workflow,
            item.required_receipts.join(","),
            item.required_examples.join(","),
            item.decision,
            item.forbids.join(","),
            item.status
        ),
    ))
}
pub fn bootstrap_operator_proof_digest(id: &str) -> Option<String> {
    let item = bootstrap_operator_proof_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_operator_interface.proof_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            item.id,
            item.scope,
            item.commands.join(","),
            item.workflows.join(","),
            item.examples.join(","),
            item.gates.join(","),
            item.receipts.join(","),
            item.forbids.join(","),
            item.status
        ),
    ))
}
pub fn bootstrap_operator_artifact_digest(id: &str) -> Option<String> {
    let item = bootstrap_operator_artifact_descriptor(id)?;
    Some(stable_hash_label(
        "lyra.p02.bootstrap_operator_interface.artifact_descriptor",
        &format!(
            "{}|{}|{}|{}|{}|{}",
            item.id,
            item.owner_root,
            item.path,
            item.artifact_kind,
            item.commands.join(","),
            item.status
        ),
    ))
}

pub fn bootstrap_operator_registry_signature() -> String {
    let mut lines = Vec::new();
    for id in bootstrap_operator_command_ids() {
        lines.push(format!("command:{id}"));
    }
    for id in bootstrap_operator_workflow_ids() {
        lines.push(format!("workflow:{id}"));
    }
    for id in bootstrap_operator_example_ids() {
        lines.push(format!("example:{id}"));
    }
    for id in bootstrap_operator_gate_ids() {
        lines.push(format!("gate:{id}"));
    }
    for id in bootstrap_operator_proof_ids() {
        lines.push(format!("proof:{id}"));
    }
    for id in bootstrap_operator_artifact_ids() {
        lines.push(format!("artifact:{id}"));
    }
    lines.sort();
    lines.join("\n")
}

pub fn bootstrap_operator_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_operator_interface.registry",
        &bootstrap_operator_registry_signature(),
    )
}

pub fn bootstrap_operator_carrier_signature() -> String {
    format!(
        "{}|{}",
        LYRA_P02_BOOTSTRAP_OPERATOR_INTERFACE_CARRIER,
        bootstrap_operator_registry_hash()
    )
}

pub fn bootstrap_operator_workflows_bind_known_commands() -> bool {
    let commands = bootstrap_operator_command_ids();
    LYRALANG_BOOTSTRAP_OPERATOR_WORKFLOWS.iter().all(|item| {
        item.commands
            .iter()
            .all(|command| commands.contains(command))
    })
}
pub fn bootstrap_operator_examples_bind_known_commands() -> bool {
    let commands = bootstrap_operator_command_ids();
    LYRALANG_BOOTSTRAP_OPERATOR_EXAMPLES.iter().all(|item| {
        item.commands
            .iter()
            .all(|command| commands.contains(command))
    })
}
pub fn bootstrap_operator_gates_bind_registry() -> bool {
    let workflows = bootstrap_operator_workflow_ids();
    let examples = bootstrap_operator_example_ids();
    LYRALANG_BOOTSTRAP_OPERATOR_GATES.iter().all(|item| {
        workflows.contains(&item.workflow)
            && item
                .required_examples
                .iter()
                .all(|example| examples.contains(example))
    })
}
pub fn bootstrap_operator_proofs_bind_registry() -> bool {
    let commands = bootstrap_operator_command_ids();
    let workflows = bootstrap_operator_workflow_ids();
    let examples = bootstrap_operator_example_ids();
    let gates = bootstrap_operator_gate_ids();
    LYRALANG_BOOTSTRAP_OPERATOR_PROOFS.iter().all(|item| {
        item.commands
            .iter()
            .all(|command| commands.contains(command))
            && item
                .workflows
                .iter()
                .all(|workflow| workflows.contains(workflow))
            && item
                .examples
                .iter()
                .all(|example| examples.contains(example))
            && item.gates.iter().all(|gate| gates.contains(gate))
    })
}
pub fn bootstrap_operator_artifacts_bind_commands() -> bool {
    let commands = bootstrap_operator_command_ids();
    LYRALANG_BOOTSTRAP_OPERATOR_ARTIFACTS.iter().all(|item| {
        item.commands
            .iter()
            .all(|command| commands.contains(command))
            && !item.path.is_empty()
            && !item.path.contains("..")
    })
}
pub fn bootstrap_operator_commands_cover_p02_018() -> bool {
    bootstrap_operator_command_ids().len() >= 8 && bootstrap_operator_gate_ids().len() >= 5
}
pub fn bootstrap_operator_no_forbidden_descriptor_claims() -> bool {
    let signature = bootstrap_operator_registry_signature().to_ascii_lowercase();
    !(signature.contains("manual_only")
        || signature.contains("network_required")
        || signature.contains("probabilistic")
        || signature.contains("phase_complete"))
}
