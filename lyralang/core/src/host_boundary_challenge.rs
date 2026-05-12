use crate::k0_hash::stable_hash_label;
pub const LYRA_P02_HOST_BOUNDARY_CHALLENGE_CARRIER: &str =
    "LYRA-P02-HOST-BOUNDARY-CHALLENGE-CARRIER v1";
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostBoundaryChallengeSuiteDescriptor {
    pub id: &'static str,
    pub boundary_surface: &'static str,
    pub suite_kind: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostBoundaryProbeDescriptor {
    pub id: &'static str,
    pub suite_id: &'static str,
    pub surface_ref: &'static str,
    pub expected_error: &'static str,
}
pub const LYRALANG_HOST_BOUNDARY_CHALLENGE_SUITES: &[HostBoundaryChallengeSuiteDescriptor] = &[
    HostBoundaryChallengeSuiteDescriptor {
        id: "suite_no_ambient_network_import",
        boundary_surface: "surface:git_repository_transport",
        suite_kind: "ambient_network_rejection",
    },
    HostBoundaryChallengeSuiteDescriptor {
        id: "suite_no_ambient_time_truth",
        boundary_surface: "surface:external_wall_clock",
        suite_kind: "ambient_time_rejection",
    },
    HostBoundaryChallengeSuiteDescriptor {
        id: "suite_no_hidden_randomness_truth",
        boundary_surface: "surface:host_operating_system",
        suite_kind: "hidden_randomness_rejection",
    },
    HostBoundaryChallengeSuiteDescriptor {
        id: "suite_no_unledgered_host_surface",
        boundary_surface: "surface:host_filesystem",
        suite_kind: "unledgered_surface_rejection",
    },
    HostBoundaryChallengeSuiteDescriptor {
        id: "suite_no_foreign_semantic_ownership",
        boundary_surface: "surface:rust_bootstrap_compiler",
        suite_kind: "semantic_ownership_rejection",
    },
    HostBoundaryChallengeSuiteDescriptor {
        id: "suite_operator_truth_containment",
        boundary_surface: "surface:operator_shell_terminal",
        suite_kind: "operator_truth_containment",
    },
    HostBoundaryChallengeSuiteDescriptor {
        id: "suite_foreign_runtime_quarantine",
        boundary_surface: "surface:rust_std_runtime",
        suite_kind: "foreign_runtime_quarantine",
    },
];
pub const LYRALANG_HOST_BOUNDARY_PROBES: &[HostBoundaryProbeDescriptor] = &[
    HostBoundaryProbeDescriptor {
        id: "probe_artifact_generation_python_helper",
        suite_id: "suite_no_unledgered_host_surface",
        surface_ref: "surface:artifact_generation_python_helper",
        expected_error: "root_ownership_violation",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_cargo_build_driver",
        suite_id: "suite_foreign_runtime_quarantine",
        surface_ref: "surface:cargo_build_driver",
        expected_error: "root_ownership_violation",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_cursor_codex_assisted_editor",
        suite_id: "suite_operator_truth_containment",
        surface_ref: "surface:cursor_codex_assisted_editor",
        expected_error: "ambient_authority",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_external_sha256sum_tool",
        suite_id: "suite_no_unledgered_host_surface",
        surface_ref: "surface:external_sha256sum_tool",
        expected_error: "closure_proof_unbound",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_external_wall_clock",
        suite_id: "suite_no_ambient_time_truth",
        surface_ref: "surface:external_wall_clock",
        expected_error: "ambient_time_allowed",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_external_zip_packager",
        suite_id: "suite_no_unledgered_host_surface",
        surface_ref: "surface:external_zip_packager",
        expected_error: "closure_proof_unbound",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_git_repository_transport",
        suite_id: "suite_no_ambient_network_import",
        surface_ref: "surface:git_repository_transport",
        expected_error: "ambient_network_allowed",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_host_filesystem",
        suite_id: "suite_no_unledgered_host_surface",
        surface_ref: "surface:host_filesystem",
        expected_error: "closure_proof_unbound",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_host_operating_system",
        suite_id: "suite_no_hidden_randomness_truth",
        surface_ref: "surface:host_operating_system",
        expected_error: "hidden_randomness_allowed",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_host_process_launcher",
        suite_id: "suite_foreign_runtime_quarantine",
        surface_ref: "surface:host_process_launcher",
        expected_error: "root_ownership_violation",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_lyra_text_contract_carrier",
        suite_id: "suite_no_unledgered_host_surface",
        surface_ref: "surface:lyra_text_contract_carrier",
        expected_error: "closure_proof_unbound",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_lyralang_bootstrap_stub_carrier",
        suite_id: "suite_no_foreign_semantic_ownership",
        surface_ref: "surface:lyralang_bootstrap_stub_carrier",
        expected_error: "root_ownership_violation",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_operator_shell_terminal",
        suite_id: "suite_operator_truth_containment",
        surface_ref: "surface:operator_shell_terminal",
        expected_error: "ambient_authority",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_physical_cpu_instruction_set",
        suite_id: "suite_foreign_runtime_quarantine",
        surface_ref: "surface:physical_cpu_instruction_set",
        expected_error: "root_ownership_violation",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_rust_bootstrap_compiler",
        suite_id: "suite_no_foreign_semantic_ownership",
        surface_ref: "surface:rust_bootstrap_compiler",
        expected_error: "root_ownership_violation",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_rust_std_runtime",
        suite_id: "suite_foreign_runtime_quarantine",
        surface_ref: "surface:rust_std_runtime",
        expected_error: "root_ownership_violation",
    },
    HostBoundaryProbeDescriptor {
        id: "probe_unbounded_network_bootstrap_fetch",
        suite_id: "suite_no_ambient_network_import",
        surface_ref: "surface:unbounded_network_bootstrap_fetch",
        expected_error: "ambient_network_allowed",
    },
];
pub fn host_boundary_challenge_suite_ids() -> Vec<&'static str> {
    LYRALANG_HOST_BOUNDARY_CHALLENGE_SUITES
        .iter()
        .map(|x| x.id)
        .collect()
}
pub fn host_boundary_probe_ids() -> Vec<&'static str> {
    LYRALANG_HOST_BOUNDARY_PROBES.iter().map(|x| x.id).collect()
}
pub fn host_boundary_challenge_covers_surface(surface_ref: &str) -> bool {
    LYRALANG_HOST_BOUNDARY_PROBES
        .iter()
        .any(|x| x.surface_ref == surface_ref)
}
pub fn host_boundary_suite_registry_signature() -> String {
    let mut f: Vec<String> = LYRALANG_HOST_BOUNDARY_CHALLENGE_SUITES
        .iter()
        .map(|x| {
            stable_hash_label(
                "lyra.p02.host_boundary.carrier.suite",
                &format!("{}|{}|{}", x.id, x.boundary_surface, x.suite_kind),
            )
        })
        .collect();
    f.extend(LYRALANG_HOST_BOUNDARY_PROBES.iter().map(|x| {
        stable_hash_label(
            "lyra.p02.host_boundary.carrier.probe",
            &format!(
                "{}|{}|{}|{}",
                x.id, x.suite_id, x.surface_ref, x.expected_error
            ),
        )
    }));
    f.sort();
    stable_hash_label("lyra.p02.host_boundary.carrier.registry", &f.join("|"))
}
pub fn host_boundary_suite_registry_hash() -> String {
    host_boundary_suite_registry_signature()
}
