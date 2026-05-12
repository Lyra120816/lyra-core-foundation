use crate::k0_hash::stable_hash_label;

pub const LYRA_P02_FOREIGN_SURFACE_CLOSURE_CARRIER: &str = "lyra.p02.foreign_surface_closure";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForeignSurfaceClosureDescriptor {
    pub id: &'static str,
    pub surface_class: &'static str,
    pub owner_root: &'static str,
    pub challenge_suite: &'static str,
    pub closure_law: &'static str,
    pub retirement_gate: &'static str,
    pub truth_effect: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForeignSurfaceChallengeDescriptor {
    pub id: &'static str,
    pub suite_id: &'static str,
    pub surface_id: &'static str,
    pub required_fixture: &'static str,
    pub negative_case: &'static str,
    pub receipt_path: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForeignSurfaceClosureLawDescriptor {
    pub id: &'static str,
    pub surface_id: &'static str,
    pub closure_gate: &'static str,
    pub deletion_gate: &'static str,
    pub retirement_receipt: &'static str,
    pub allowed_closure_scope: &'static str,
}

pub const LYRALANG_FOREIGN_BOOTSTRAP_SURFACES: &[ForeignSurfaceClosureDescriptor] = &[
    ForeignSurfaceClosureDescriptor {
        id: "foreign_rust_toolchain",
        surface_class: "bootstrap_language",
        owner_root: "lyralang",
        challenge_suite: "suite_boundary_nonambient",
        closure_law: "law_rust_toolchain",
        retirement_gate: "gate_delete_when_lyralang_selfhosted",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_cargo_runner",
        surface_class: "build_runner",
        owner_root: "ops",
        challenge_suite: "suite_boundary_nonambient",
        closure_law: "law_cargo_runner",
        retirement_gate: "gate_delete_when_native_builder_exists",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_rust_stdlib",
        surface_class: "host_runtime",
        owner_root: "k0",
        challenge_suite: "suite_boundary_nonambient",
        closure_law: "law_rust_stdlib",
        retirement_gate: "gate_delete_when_native_runtime_exists",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_host_os",
        surface_class: "host_os",
        owner_root: "ops",
        challenge_suite: "suite_visibility_totality",
        closure_law: "law_host_os",
        retirement_gate: "gate_keep_bounded_interface",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_filesystem",
        surface_class: "host_io",
        owner_root: "k0",
        challenge_suite: "suite_boundary_nonambient",
        closure_law: "law_filesystem",
        retirement_gate: "gate_keep_canonical_io_interface",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_terminal",
        surface_class: "operator_tool",
        owner_root: "shells",
        challenge_suite: "suite_visibility_totality",
        closure_law: "law_terminal",
        retirement_gate: "gate_keep_operator_shell_boundary",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_zip_tool",
        surface_class: "archive_tool",
        owner_root: "ops",
        challenge_suite: "suite_deletion_gate",
        closure_law: "law_zip_tool",
        retirement_gate: "gate_replace_with_lyra_archive_writer",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_sha256sum",
        surface_class: "digest_tool",
        owner_root: "k0",
        challenge_suite: "suite_deletion_gate",
        closure_law: "law_sha256sum",
        retirement_gate: "gate_replace_with_lyra_digest_emitter",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_android_bridge",
        surface_class: "platform_bridge",
        owner_root: "products",
        challenge_suite: "suite_closure_scope",
        closure_law: "law_android_bridge",
        retirement_gate: "gate_bound_platform_surface",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_wasm_toolchain",
        surface_class: "target_toolchain",
        owner_root: "interfaces",
        challenge_suite: "suite_boundary_nonambient",
        closure_law: "law_wasm_toolchain",
        retirement_gate: "gate_replace_with_native_wasm_emitter",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_mobile_packager",
        surface_class: "packager",
        owner_root: "products",
        challenge_suite: "suite_closure_scope",
        closure_law: "law_mobile_packager",
        retirement_gate: "gate_replace_with_native_packager",
        truth_effect: "no_truth_without_local_challenge",
    },
    ForeignSurfaceClosureDescriptor {
        id: "foreign_baremetal_loader",
        surface_class: "bootloader",
        owner_root: "k0",
        challenge_suite: "suite_closure_scope",
        closure_law: "law_baremetal_loader",
        retirement_gate: "gate_replace_with_native_loader",
        truth_effect: "no_truth_without_local_challenge",
    },
];

pub const LYRALANG_FOREIGN_SURFACE_CHALLENGES: &[ForeignSurfaceChallengeDescriptor] = &[
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_rust_toolchain",
        suite_id: "suite_boundary_nonambient",
        surface_id: "foreign_rust_toolchain",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_rust_toolchain_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_rust_toolchain.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_cargo_runner",
        suite_id: "suite_boundary_nonambient",
        surface_id: "foreign_cargo_runner",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_cargo_runner_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_cargo_runner.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_rust_stdlib",
        suite_id: "suite_boundary_nonambient",
        surface_id: "foreign_rust_stdlib",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_rust_stdlib_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_rust_stdlib.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_host_os",
        suite_id: "suite_visibility_totality",
        surface_id: "foreign_host_os",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_host_os_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_host_os.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_filesystem",
        suite_id: "suite_boundary_nonambient",
        surface_id: "foreign_filesystem",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_filesystem_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_filesystem.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_terminal",
        suite_id: "suite_visibility_totality",
        surface_id: "foreign_terminal",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_terminal_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_terminal.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_zip_tool",
        suite_id: "suite_deletion_gate",
        surface_id: "foreign_zip_tool",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_zip_tool_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_zip_tool.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_sha256sum",
        suite_id: "suite_deletion_gate",
        surface_id: "foreign_sha256sum",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_sha256sum_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_sha256sum.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_android_bridge",
        suite_id: "suite_closure_scope",
        surface_id: "foreign_android_bridge",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_android_bridge_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_android_bridge.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_wasm_toolchain",
        suite_id: "suite_boundary_nonambient",
        surface_id: "foreign_wasm_toolchain",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_wasm_toolchain_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_wasm_toolchain.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_mobile_packager",
        suite_id: "suite_closure_scope",
        surface_id: "foreign_mobile_packager",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_mobile_packager_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_mobile_packager.receipt",
    },
    ForeignSurfaceChallengeDescriptor {
        id: "challenge_baremetal_loader",
        suite_id: "suite_closure_scope",
        surface_id: "foreign_baremetal_loader",
        required_fixture:
            "fixtures/p02/foreign_surface_closure_inputs/valid_foreign_surface_closure.lyra",
        negative_case: "invalid_baremetal_loader_unbounded.lyra",
        receipt_path: "receipts/p02/foreign_surface_challenges/foreign_baremetal_loader.receipt",
    },
];

pub const LYRALANG_FOREIGN_SURFACE_CLOSURE_LAWS: &[ForeignSurfaceClosureLawDescriptor] = &[
    ForeignSurfaceClosureLawDescriptor {
        id: "law_rust_toolchain",
        surface_id: "foreign_rust_toolchain",
        closure_gate: "gate_rust_toolchain_bounded_closeout",
        deletion_gate: "gate_delete_when_lyralang_selfhosted",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_rust_toolchain_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_cargo_runner",
        surface_id: "foreign_cargo_runner",
        closure_gate: "gate_cargo_runner_bounded_closeout",
        deletion_gate: "gate_delete_when_native_builder_exists",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_cargo_runner_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_rust_stdlib",
        surface_id: "foreign_rust_stdlib",
        closure_gate: "gate_rust_stdlib_bounded_closeout",
        deletion_gate: "gate_delete_when_native_runtime_exists",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_rust_stdlib_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_host_os",
        surface_id: "foreign_host_os",
        closure_gate: "gate_host_os_bounded_closeout",
        deletion_gate: "gate_keep_bounded_interface",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_host_os_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_filesystem",
        surface_id: "foreign_filesystem",
        closure_gate: "gate_filesystem_bounded_closeout",
        deletion_gate: "gate_keep_canonical_io_interface",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_filesystem_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_terminal",
        surface_id: "foreign_terminal",
        closure_gate: "gate_terminal_bounded_closeout",
        deletion_gate: "gate_keep_operator_shell_boundary",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_terminal_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_zip_tool",
        surface_id: "foreign_zip_tool",
        closure_gate: "gate_zip_tool_bounded_closeout",
        deletion_gate: "gate_replace_with_lyra_archive_writer",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_zip_tool_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_sha256sum",
        surface_id: "foreign_sha256sum",
        closure_gate: "gate_sha256sum_bounded_closeout",
        deletion_gate: "gate_replace_with_lyra_digest_emitter",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_sha256sum_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_android_bridge",
        surface_id: "foreign_android_bridge",
        closure_gate: "gate_android_bridge_bounded_closeout",
        deletion_gate: "gate_bound_platform_surface",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_android_bridge_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_wasm_toolchain",
        surface_id: "foreign_wasm_toolchain",
        closure_gate: "gate_wasm_toolchain_bounded_closeout",
        deletion_gate: "gate_replace_with_native_wasm_emitter",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_wasm_toolchain_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_mobile_packager",
        surface_id: "foreign_mobile_packager",
        closure_gate: "gate_mobile_packager_bounded_closeout",
        deletion_gate: "gate_replace_with_native_packager",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_mobile_packager_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
    ForeignSurfaceClosureLawDescriptor {
        id: "law_baremetal_loader",
        surface_id: "foreign_baremetal_loader",
        closure_gate: "gate_baremetal_loader_bounded_closeout",
        deletion_gate: "gate_replace_with_native_loader",
        retirement_receipt:
            "receipts/p02/foreign_surface_closure/foreign_baremetal_loader_retirement.receipt",
        allowed_closure_scope: "per_surface_only",
    },
];

pub fn foreign_bootstrap_surface_ids() -> Vec<&'static str> {
    LYRALANG_FOREIGN_BOOTSTRAP_SURFACES
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn foreign_surface_challenge_ids() -> Vec<&'static str> {
    LYRALANG_FOREIGN_SURFACE_CHALLENGES
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn foreign_surface_closure_law_ids() -> Vec<&'static str> {
    LYRALANG_FOREIGN_SURFACE_CLOSURE_LAWS
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn foreign_bootstrap_surface_descriptor(
    id: &str,
) -> Option<&'static ForeignSurfaceClosureDescriptor> {
    LYRALANG_FOREIGN_BOOTSTRAP_SURFACES
        .iter()
        .find(|x| x.id == id)
}
pub fn foreign_surface_challenge_descriptor(
    surface_id: &str,
) -> Option<&'static ForeignSurfaceChallengeDescriptor> {
    LYRALANG_FOREIGN_SURFACE_CHALLENGES
        .iter()
        .find(|x| x.surface_id == surface_id)
}
pub fn foreign_surface_closure_law_descriptor(
    surface_id: &str,
) -> Option<&'static ForeignSurfaceClosureLawDescriptor> {
    LYRALANG_FOREIGN_SURFACE_CLOSURE_LAWS
        .iter()
        .find(|x| x.surface_id == surface_id)
}
pub fn foreign_surface_all_truth_neutral() -> bool {
    LYRALANG_FOREIGN_BOOTSTRAP_SURFACES
        .iter()
        .all(|x| x.truth_effect == "no_truth_without_local_challenge")
}
pub fn foreign_surface_all_have_challenges() -> bool {
    LYRALANG_FOREIGN_BOOTSTRAP_SURFACES
        .iter()
        .all(|x| foreign_surface_challenge_descriptor(x.id).is_some())
}
pub fn foreign_surface_all_have_closure_laws() -> bool {
    LYRALANG_FOREIGN_BOOTSTRAP_SURFACES
        .iter()
        .all(|x| foreign_surface_closure_law_descriptor(x.id).is_some())
}
pub fn foreign_surface_closure_receipt_paths_are_local() -> bool {
    LYRALANG_FOREIGN_SURFACE_CLOSURE_LAWS
        .iter()
        .all(|x| x.retirement_receipt.starts_with("receipts/p02/"))
        && LYRALANG_FOREIGN_SURFACE_CHALLENGES
            .iter()
            .all(|x| x.receipt_path.starts_with("receipts/p02/"))
}

pub fn foreign_surface_closure_registry_hash() -> String {
    let surface_part = LYRALANG_FOREIGN_BOOTSTRAP_SURFACES
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}:{}:{}",
                x.id,
                x.surface_class,
                x.owner_root,
                x.challenge_suite,
                x.closure_law,
                x.retirement_gate,
                x.truth_effect
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let challenge_part = LYRALANG_FOREIGN_SURFACE_CHALLENGES
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                x.id, x.suite_id, x.surface_id, x.required_fixture, x.negative_case, x.receipt_path
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let closure_part = LYRALANG_FOREIGN_SURFACE_CLOSURE_LAWS
        .iter()
        .map(|x| {
            format!(
                "{}:{}:{}:{}:{}:{}",
                x.id,
                x.surface_id,
                x.closure_gate,
                x.deletion_gate,
                x.retirement_receipt,
                x.allowed_closure_scope
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    stable_hash_label(
        "lyra.p02.foreign_surface_closure.registry",
        &format!("{surface_part}|{challenge_part}|{closure_part}"),
    )
}

pub fn foreign_surface_closure_registry_signature() -> String {
    format!(
        "{}:{}:{}:{}",
        LYRA_P02_FOREIGN_SURFACE_CLOSURE_CARRIER,
        LYRALANG_FOREIGN_BOOTSTRAP_SURFACES.len(),
        LYRALANG_FOREIGN_SURFACE_CHALLENGES.len(),
        LYRALANG_FOREIGN_SURFACE_CLOSURE_LAWS.len()
    )
}
