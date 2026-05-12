use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_BOOTSTRAP_EXTINCTION_CARRIER: &str = "LYRA-P02-BOOTSTRAP-EXTINCTION-CARRIER v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapExtinctionDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub classification: &'static str,
    pub deletion_action: &'static str,
    pub successor: &'static str,
    pub ledger_state: &'static str,
}

pub const LYRALANG_BOOTSTRAP_EXTINCTION_ENTRIES: &[BootstrapExtinctionDescriptor] = &[
    BootstrapExtinctionDescriptor {
        id: "artifact_generation_python_helper",
        owner_root: "ops",
        classification: "temporary",
        deletion_action: "delete_after_native_artifact_emitter",
        successor: "lyra_native_artifact_emitter",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "cargo_build_driver",
        owner_root: "ops",
        classification: "temporary",
        deletion_action: "delete_after_native_build_driver",
        successor: "lyra_native_build_driver",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "cursor_codex_assisted_editor",
        owner_root: "ops",
        classification: "observer",
        deletion_action: "quarantine_and_discard",
        successor: "none_observer_only",
        ledger_state: "contained",
    },
    BootstrapExtinctionDescriptor {
        id: "external_sha256sum_tool",
        owner_root: "ops",
        classification: "observer",
        deletion_action: "quarantine_and_discard",
        successor: "none_observer_only",
        ledger_state: "contained",
    },
    BootstrapExtinctionDescriptor {
        id: "external_wall_clock",
        owner_root: "k0",
        classification: "forbidden",
        deletion_action: "deny_import_and_delete_reference",
        successor: "none_forbidden",
        ledger_state: "forbidden_no_import",
    },
    BootstrapExtinctionDescriptor {
        id: "external_zip_packager",
        owner_root: "ops",
        classification: "temporary",
        deletion_action: "delete_after_native_packager",
        successor: "lyra_native_packager",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "git_repository_transport",
        owner_root: "ops",
        classification: "observer",
        deletion_action: "quarantine_and_discard",
        successor: "none_observer_only",
        ledger_state: "contained",
    },
    BootstrapExtinctionDescriptor {
        id: "host_filesystem",
        owner_root: "k0",
        classification: "temporary",
        deletion_action: "delete_after_native_storage_driver",
        successor: "lyra_native_storage_driver",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "host_operating_system",
        owner_root: "k0",
        classification: "temporary",
        deletion_action: "delete_after_native_target_kernel",
        successor: "lyra_native_target_kernel",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "host_process_launcher",
        owner_root: "shells",
        classification: "temporary",
        deletion_action: "delete_after_native_process_launcher",
        successor: "lyra_native_process_launcher",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "lyra_text_contract_carrier",
        owner_root: "interfaces",
        classification: "temporary",
        deletion_action: "delete_after_native_contract_surface",
        successor: "lyra_native_contract_surface",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "lyralang_bootstrap_stub_carrier",
        owner_root: "lyralang",
        classification: "temporary",
        deletion_action: "delete_after_self_hosted_lyralang_carrier",
        successor: "lyralang_native_carrier",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "operator_shell_terminal",
        owner_root: "shells",
        classification: "temporary",
        deletion_action: "delete_after_native_operator_shell",
        successor: "lyra_native_operator_shell",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "physical_cpu_instruction_set",
        owner_root: "k0",
        classification: "bounded_permanent",
        deletion_action: "retain_as_target_descriptor",
        successor: "target_descriptor_bound",
        ledger_state: "retained_by_target_descriptor",
    },
    BootstrapExtinctionDescriptor {
        id: "rust_bootstrap_compiler",
        owner_root: "k0",
        classification: "temporary",
        deletion_action: "delete_after_lyralang_native_compiler",
        successor: "lyralang_native_compiler",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "rust_std_runtime",
        owner_root: "k0",
        classification: "temporary",
        deletion_action: "delete_after_lyra_native_runtime",
        successor: "lyra_native_runtime",
        ledger_state: "deletion_scheduled",
    },
    BootstrapExtinctionDescriptor {
        id: "unbounded_network_bootstrap_fetch",
        owner_root: "ops",
        classification: "forbidden",
        deletion_action: "deny_import_and_delete_reference",
        successor: "none_forbidden",
        ledger_state: "forbidden_no_import",
    },
];

pub fn bootstrap_extinction_descriptor(id: &str) -> Option<BootstrapExtinctionDescriptor> {
    LYRALANG_BOOTSTRAP_EXTINCTION_ENTRIES
        .iter()
        .copied()
        .find(|item| item.id == id)
}

pub fn bootstrap_extinction_ids() -> Vec<&'static str> {
    let mut ids: Vec<&'static str> = LYRALANG_BOOTSTRAP_EXTINCTION_ENTRIES
        .iter()
        .map(|item| item.id)
        .collect();
    ids.sort();
    ids
}

pub fn bootstrap_extinction_descriptor_signature(item: &BootstrapExtinctionDescriptor) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        item.id,
        item.owner_root,
        item.classification,
        item.deletion_action,
        item.successor,
        item.ledger_state
    )
}

pub fn bootstrap_extinction_descriptor_digest(item: &BootstrapExtinctionDescriptor) -> String {
    stable_hash_label(
        "lyra.p02.lyralang.extinction.descriptor",
        &bootstrap_extinction_descriptor_signature(item),
    )
}

pub fn bootstrap_extinction_registry_signature() -> String {
    let mut signatures: Vec<String> = LYRALANG_BOOTSTRAP_EXTINCTION_ENTRIES
        .iter()
        .map(bootstrap_extinction_descriptor_signature)
        .collect();
    signatures.sort();
    signatures.join("\n")
}

pub fn bootstrap_extinction_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.lyralang.extinction.registry",
        &bootstrap_extinction_registry_signature(),
    )
}

pub fn bootstrap_extinction_covers_required_surface(id: &str) -> bool {
    LYRALANG_BOOTSTRAP_EXTINCTION_ENTRIES
        .iter()
        .any(|item| item.id == id)
}

pub fn bootstrap_extinction_has_action(id: &str, deletion_action: &str) -> bool {
    bootstrap_extinction_descriptor(id)
        .map(|item| item.deletion_action == deletion_action)
        .unwrap_or(false)
}

pub fn bootstrap_extinction_artifacts_bind_paths(paths: &[&str]) -> bool {
    paths.iter().all(|path| {
        path.starts_with("ops/p02/")
            || path.starts_with("interfaces/p02/")
            || path.starts_with("fixtures/p02/")
            || path.starts_with("goldens/p02/")
            || path.starts_with("receipts/p02/")
            || path.starts_with("tests/")
            || path.starts_with("src/bin/")
            || path.starts_with("lyralang/core/src/")
            || path.starts_with("k0/determinism/src/")
            || path.starts_with("products/p02/")
            || path.starts_with("docs/p02/")
            || path.starts_with("examples/p02/")
            || path.starts_with("shells/p02/")
    })
}
