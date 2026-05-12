use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapInventoryDescriptor {
    pub id: &'static str,
    pub owner_root: &'static str,
    pub surface_type: &'static str,
    pub classification: &'static str,
    pub boundary: &'static str,
    pub retirement_ref: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_INVENTORY_CARRIER: &str =
    "lyralang.carrier.p02.bootstrap_inventory.v1";

pub const LYRALANG_BOOTSTRAP_INVENTORY_SURFACES: &[BootstrapInventoryDescriptor] = &[
    BootstrapInventoryDescriptor {
        id: "artifact_generation_python_helper",
        owner_root: "ops",
        surface_type: "artifact_tooling",
        classification: "temporary",
        boundary: "quarantined_generation_helper",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "cargo_build_driver",
        owner_root: "ops",
        surface_type: "build_tool",
        classification: "temporary",
        boundary: "explicit_operator_command",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "cursor_codex_assisted_editor",
        owner_root: "ops",
        surface_type: "developer_tool",
        classification: "observer",
        boundary: "no_truth_authority",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "external_sha256sum_tool",
        owner_root: "ops",
        surface_type: "digest_tool",
        classification: "observer",
        boundary: "receipt_cross_check_only",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "external_wall_clock",
        owner_root: "k0",
        surface_type: "host_time",
        classification: "forbidden",
        boundary: "no_ambient_time",
        retirement_ref: "forbidden_surface_no_import",
    },
    BootstrapInventoryDescriptor {
        id: "external_zip_packager",
        owner_root: "ops",
        surface_type: "artifact_tooling",
        classification: "temporary",
        boundary: "deterministic_archive_emission",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "git_repository_transport",
        owner_root: "ops",
        surface_type: "source_transport",
        classification: "observer",
        boundary: "human_reviewed_import_only",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "host_filesystem",
        owner_root: "k0",
        surface_type: "host_runtime",
        classification: "temporary",
        boundary: "explicit_path_injection",
        retirement_ref: "P02-009",
    },
    BootstrapInventoryDescriptor {
        id: "host_operating_system",
        owner_root: "k0",
        surface_type: "host_runtime",
        classification: "temporary",
        boundary: "target_declared_host",
        retirement_ref: "P02-009",
    },
    BootstrapInventoryDescriptor {
        id: "host_process_launcher",
        owner_root: "shells",
        surface_type: "operator_shell",
        classification: "temporary",
        boundary: "explicit_operator_command",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "lyra_text_contract_carrier",
        owner_root: "interfaces",
        surface_type: "contract_carrier",
        classification: "temporary",
        boundary: "canonical_text_until_native_surface",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "lyralang_bootstrap_stub_carrier",
        owner_root: "lyralang",
        surface_type: "language_carrier",
        classification: "temporary",
        boundary: "native_language_migration",
        retirement_ref: "P02-009",
    },
    BootstrapInventoryDescriptor {
        id: "operator_shell_terminal",
        owner_root: "shells",
        surface_type: "operator_shell",
        classification: "temporary",
        boundary: "explicit_human_command",
        retirement_ref: "P02-002",
    },
    BootstrapInventoryDescriptor {
        id: "physical_cpu_instruction_set",
        owner_root: "k0",
        surface_type: "hardware_substrate",
        classification: "bounded_permanent",
        boundary: "target_descriptor_substrate",
        retirement_ref: "bounded_by_target_descriptor",
    },
    BootstrapInventoryDescriptor {
        id: "rust_bootstrap_compiler",
        owner_root: "k0",
        surface_type: "bootstrap_compiler",
        classification: "temporary",
        boundary: "seed_runtime_runway",
        retirement_ref: "P02-009",
    },
    BootstrapInventoryDescriptor {
        id: "rust_std_runtime",
        owner_root: "k0",
        surface_type: "bootstrap_runtime",
        classification: "temporary",
        boundary: "seed_runtime_runway",
        retirement_ref: "P02-009",
    },
    BootstrapInventoryDescriptor {
        id: "unbounded_network_bootstrap_fetch",
        owner_root: "ops",
        surface_type: "remote_dependency",
        classification: "forbidden",
        boundary: "no_network_truth_path",
        retirement_ref: "forbidden_surface_no_import",
    },
];

pub fn bootstrap_inventory_descriptor(id: &str) -> Option<&'static BootstrapInventoryDescriptor> {
    LYRALANG_BOOTSTRAP_INVENTORY_SURFACES
        .iter()
        .find(|item| item.id == id)
}

pub fn bootstrap_inventory_ids() -> Vec<&'static str> {
    let mut ids: Vec<_> = LYRALANG_BOOTSTRAP_INVENTORY_SURFACES
        .iter()
        .map(|item| item.id)
        .collect();
    ids.sort();
    ids
}

pub fn bootstrap_inventory_descriptor_signature(item: &BootstrapInventoryDescriptor) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}",
        item.id,
        item.owner_root,
        item.surface_type,
        item.classification,
        item.boundary,
        item.retirement_ref
    )
}

pub fn bootstrap_inventory_descriptor_digest(item: &BootstrapInventoryDescriptor) -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_inventory.descriptor",
        &bootstrap_inventory_descriptor_signature(item),
    )
}

pub fn bootstrap_inventory_registry_signature() -> String {
    let mut signatures: Vec<_> = LYRALANG_BOOTSTRAP_INVENTORY_SURFACES
        .iter()
        .map(bootstrap_inventory_descriptor_signature)
        .collect();
    signatures.sort();
    signatures.join("\n")
}

pub fn bootstrap_inventory_registry_hash() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_inventory.registry",
        &bootstrap_inventory_registry_signature(),
    )
}

pub fn bootstrap_inventory_covers_required_surface(id: &str) -> bool {
    bootstrap_inventory_descriptor(id).is_some()
}

pub fn bootstrap_inventory_has_classification(classification: &str) -> bool {
    LYRALANG_BOOTSTRAP_INVENTORY_SURFACES
        .iter()
        .any(|item| item.classification == classification)
}
