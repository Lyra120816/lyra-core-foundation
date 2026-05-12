use crate::k0_hash::stable_hash_label;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapTargetDescriptor {
    pub id: &'static str,
    pub target_class: &'static str,
    pub architecture: &'static str,
    pub runtime_lane: &'static str,
    pub proof_mode: &'static str,
    pub owner_root: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapTargetProofDescriptor {
    pub id: &'static str,
    pub target_id: &'static str,
    pub proof_family: &'static str,
    pub host_boundary_gate: &'static str,
}
pub const LYRA_P02_BOOTSTRAP_TARGET_MATRIX_CARRIER: &str =
    "lyra.p02.bootstrap_target_matrix.carrier.v1";
pub const LYRALANG_BOOTSTRAP_TARGETS: &[BootstrapTargetDescriptor] = &[
    BootstrapTargetDescriptor {
        id: "target_linux_x86_64",
        target_class: "linux",
        architecture: "x86_64",
        runtime_lane: "seed_runtime_posix",
        proof_mode: "native_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_linux_aarch64",
        target_class: "linux",
        architecture: "aarch64",
        runtime_lane: "seed_runtime_posix",
        proof_mode: "native_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_windows_x86_64",
        target_class: "windows",
        architecture: "x86_64",
        runtime_lane: "seed_runtime_win32",
        proof_mode: "native_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_windows_aarch64",
        target_class: "windows",
        architecture: "aarch64",
        runtime_lane: "seed_runtime_win32",
        proof_mode: "native_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_android_aarch64",
        target_class: "mobile",
        architecture: "aarch64",
        runtime_lane: "seed_runtime_mobile_sandbox",
        proof_mode: "bounded_platform_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_ios_aarch64",
        target_class: "mobile",
        architecture: "aarch64",
        runtime_lane: "seed_runtime_mobile_sandbox",
        proof_mode: "bounded_platform_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_wasm32_wasi",
        target_class: "wasm",
        architecture: "wasm32",
        runtime_lane: "seed_runtime_wasm_component",
        proof_mode: "wasm_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_wasm32_unknown",
        target_class: "wasm",
        architecture: "wasm32",
        runtime_lane: "seed_runtime_wasm_component",
        proof_mode: "wasm_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_baremetal_x86_64",
        target_class: "baremetal",
        architecture: "x86_64",
        runtime_lane: "seed_runtime_baremetal",
        proof_mode: "baremetal_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_baremetal_aarch64",
        target_class: "baremetal",
        architecture: "aarch64",
        runtime_lane: "seed_runtime_baremetal",
        proof_mode: "baremetal_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_baremetal_riscv64",
        target_class: "baremetal",
        architecture: "riscv64",
        runtime_lane: "seed_runtime_baremetal",
        proof_mode: "baremetal_bootstrap",
        owner_root: "k0",
    },
    BootstrapTargetDescriptor {
        id: "target_host_tooling_quarantine",
        target_class: "other",
        architecture: "host_abstract",
        runtime_lane: "seed_runtime_quarantine",
        proof_mode: "bounded_observer_bootstrap",
        owner_root: "ops",
    },
];
pub const LYRALANG_BOOTSTRAP_TARGET_PROOF_FAMILIES: &[&str] = &[
    "canonical_io",
    "deterministic_replay",
    "host_boundary",
    "receipt_chain",
    "rollback_lane",
];
pub fn bootstrap_target_ids() -> Vec<&'static str> {
    LYRALANG_BOOTSTRAP_TARGETS.iter().map(|x| x.id).collect()
}
pub fn bootstrap_target_descriptor(id: &str) -> Option<&'static BootstrapTargetDescriptor> {
    LYRALANG_BOOTSTRAP_TARGETS.iter().find(|x| x.id == id)
}
pub fn bootstrap_target_descriptor_signature(d: &BootstrapTargetDescriptor) -> String {
    format!(
        "{}:{}:{}:{}:{}:{}",
        d.id, d.target_class, d.architecture, d.runtime_lane, d.proof_mode, d.owner_root
    )
}
pub fn bootstrap_target_descriptor_digest(d: &BootstrapTargetDescriptor) -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_target.descriptor",
        &bootstrap_target_descriptor_signature(d),
    )
}
pub fn bootstrap_target_registry_signature() -> String {
    let mut sigs: Vec<String> = LYRALANG_BOOTSTRAP_TARGETS
        .iter()
        .map(bootstrap_target_descriptor_signature)
        .collect();
    sigs.sort();
    sigs.join("\n")
}
pub fn bootstrap_target_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_target.registry",
        &bootstrap_target_registry_signature(),
    )
}
pub fn bootstrap_target_has_class(target_id: &str, target_class: &str) -> bool {
    bootstrap_target_descriptor(target_id)
        .map(|x| x.target_class == target_class)
        .unwrap_or(false)
}
pub fn bootstrap_target_proof_family_known(family: &str) -> bool {
    LYRALANG_BOOTSTRAP_TARGET_PROOF_FAMILIES.contains(&family)
}
