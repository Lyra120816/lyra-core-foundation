use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOutputAudienceDescriptor {
    pub id: &'static str,
    pub outputs: &'static [&'static str],
    pub artifacts: &'static [&'static str],
    pub receipts: &'static [&'static str],
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOutputArtifactDescriptor {
    pub id: &'static str,
    pub audience: &'static str,
    pub artifact_kind: &'static str,
    pub owner_root: &'static str,
    pub path: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOutputReceiptDescriptor {
    pub id: &'static str,
    pub path: &'static str,
    pub target: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOutputContractDescriptor {
    pub id: &'static str,
    pub surface: &'static str,
    pub path: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapOutputGapDescriptor {
    pub id: &'static str,
    pub blocker: &'static str,
    pub next_frontier: &'static str,
    pub owner_root: &'static str,
    pub status: &'static str,
}

pub const LYRA_P02_BOOTSTRAP_OUTPUT_TABLE_CARRIER: &str =
    "bootstrap-output-table:phase=P02;task=P02-X04;closure=extended_open;next=P02-X05";
pub const LYRALANG_BOOTSTRAP_OUTPUT_AUDIENCES: &[BootstrapOutputAudienceDescriptor] = &[
    BootstrapOutputAudienceDescriptor {
        id: "developer",
        outputs: &["contracts", "cli", "fixtures", "goldens"],
        artifacts: &[
            "developer_bootstrap_contract_index",
            "developer_bootstrap_cli_matrix",
            "developer_bootstrap_fixture_corpus",
            "developer_bootstrap_golden_index",
        ],
        receipts: &["receipt_bootstrap_output_table"],
        status: "artifact_emitted",
    },
    BootstrapOutputAudienceDescriptor {
        id: "operator",
        outputs: &["controls", "receipts", "blockers", "closure_gate"],
        artifacts: &[
            "operator_bootstrap_control_plane",
            "operator_bootstrap_receipt_index",
            "operator_bootstrap_blocker_index",
            "operator_bootstrap_closure_gate",
        ],
        receipts: &[
            "receipt_bootstrap_closure",
            "receipt_bootstrap_dependency_matrix",
            "receipt_bootstrap_output_table",
        ],
        status: "artifact_emitted",
    },
    BootstrapOutputAudienceDescriptor {
        id: "product",
        outputs: &["reference", "examples", "packaging"],
        artifacts: &[
            "product_bootstrap_reference_surface",
            "product_bootstrap_examples",
            "product_bootstrap_packaging_surface",
        ],
        receipts: &[
            "receipt_bootstrap_packaging",
            "receipt_bootstrap_benchmark_pack",
            "receipt_bootstrap_output_table",
        ],
        status: "artifact_emitted",
    },
    BootstrapOutputAudienceDescriptor {
        id: "enterprise",
        outputs: &["offline_deployment", "release_bundle", "compliance"],
        artifacts: &[
            "enterprise_bootstrap_offline_deployment",
            "enterprise_bootstrap_release_bundle",
            "enterprise_bootstrap_compliance_hooks",
        ],
        receipts: &[
            "receipt_bootstrap_deployment",
            "receipt_bootstrap_economics",
            "receipt_bootstrap_output_table",
        ],
        status: "artifact_emitted",
    },
    BootstrapOutputAudienceDescriptor {
        id: "public_interest",
        outputs: &["access", "stewardship", "people_first_review"],
        artifacts: &[
            "public_interest_bootstrap_access_model",
            "public_interest_bootstrap_stewardship_frame",
            "public_interest_bootstrap_people_first_review",
            "public_interest_bootstrap_economics_surface",
        ],
        receipts: &[
            "receipt_bootstrap_economics",
            "receipt_bootstrap_redteam",
            "receipt_bootstrap_output_table",
        ],
        status: "artifact_emitted",
    },
];
pub const LYRALANG_BOOTSTRAP_OUTPUT_ARTIFACTS: &[BootstrapOutputArtifactDescriptor] = &[
    BootstrapOutputArtifactDescriptor {
        id: "developer_bootstrap_contract_index",
        audience: "developer",
        artifact_kind: "contract",
        owner_root: "interfaces",
        path: "interfaces/p02/contracts/bootstrap_output_table.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "developer_bootstrap_cli_matrix",
        audience: "developer",
        artifact_kind: "cli",
        owner_root: "shells",
        path: "shells/p02/bootstrap_output_table_shell.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "developer_bootstrap_fixture_corpus",
        audience: "developer",
        artifact_kind: "fixture",
        owner_root: "interfaces",
        path: "fixtures/p02/bootstrap_output_table_inputs/valid_bootstrap_output_table.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "developer_bootstrap_golden_index",
        audience: "developer",
        artifact_kind: "golden",
        owner_root: "interfaces",
        path: "goldens/p02/valid_bootstrap_output_table.receipt",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "operator_bootstrap_control_plane",
        audience: "operator",
        artifact_kind: "control",
        owner_root: "ops",
        path: "ops/p02/control/bootstrap_output_table_law.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "operator_bootstrap_receipt_index",
        audience: "operator",
        artifact_kind: "receipt",
        owner_root: "ops",
        path: "receipts/p02/pass_0086_bootstrap_output_table.receipt",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "operator_bootstrap_blocker_index",
        audience: "operator",
        artifact_kind: "closure",
        owner_root: "ops",
        path: "ops/p02/closure/p02_x04_output_table_gate.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "operator_bootstrap_closure_gate",
        audience: "operator",
        artifact_kind: "closure",
        owner_root: "ops",
        path: "ops/p02/closure/p02_x04_output_table_gate.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "product_bootstrap_reference_surface",
        audience: "product",
        artifact_kind: "product_ref",
        owner_root: "products",
        path: "products/p02/bootstrap_output_table_manifest.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "product_bootstrap_examples",
        audience: "product",
        artifact_kind: "example",
        owner_root: "products",
        path: "examples/p02/output_table/bootstrap_output_table_operator_review.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "product_bootstrap_packaging_surface",
        audience: "product",
        artifact_kind: "artifact",
        owner_root: "products",
        path: "products/p02/bootstrap_benchmark_pack_manifest.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "enterprise_bootstrap_offline_deployment",
        audience: "enterprise",
        artifact_kind: "artifact",
        owner_root: "ops",
        path: "ops/p02/deployment/bootstrap_deployment.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "enterprise_bootstrap_release_bundle",
        audience: "enterprise",
        artifact_kind: "artifact",
        owner_root: "products",
        path: "products/p02/bootstrap_deployment_manifest.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "enterprise_bootstrap_compliance_hooks",
        audience: "enterprise",
        artifact_kind: "artifact",
        owner_root: "products",
        path: "products/p02/bootstrap_economics_manifest.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "public_interest_bootstrap_access_model",
        audience: "public_interest",
        artifact_kind: "artifact",
        owner_root: "products",
        path: "products/p02/bootstrap_economics_manifest.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "public_interest_bootstrap_stewardship_frame",
        audience: "public_interest",
        artifact_kind: "doc",
        owner_root: "docs",
        path: "docs/p02/bootstrap_trust_public_value_frame.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "public_interest_bootstrap_people_first_review",
        audience: "public_interest",
        artifact_kind: "example",
        owner_root: "examples",
        path: "examples/p02/economics/bootstrap_public_casebook.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputArtifactDescriptor {
        id: "public_interest_bootstrap_economics_surface",
        audience: "public_interest",
        artifact_kind: "artifact",
        owner_root: "ops",
        path: "ops/p02/economics/bootstrap_economics_docket.v1.lyra",
        status: "artifact_emitted",
    },
];
pub const LYRALANG_BOOTSTRAP_OUTPUT_RECEIPTS: &[BootstrapOutputReceiptDescriptor] = &[
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_inventory",
        path: "receipts/p02/pass_0059_bootstrap_surface_inventory.receipt",
        target: "P02-001",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_extinction",
        path: "receipts/p02/pass_0060_bootstrap_extinction_ledger.receipt",
        target: "P02-002",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_seed_runtime_contracts",
        path: "receipts/p02/pass_0061_seed_runtime_contracts.receipt",
        target: "P02-003",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_seed_runtime_replacement",
        path: "receipts/p02/pass_0067_seed_runtime_replacement_milestones.receipt",
        target: "P02-009",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_packaging",
        path: "receipts/p02/pass_0077_bootstrap_packaging.receipt",
        target: "P02-019",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_deployment",
        path: "receipts/p02/pass_0078_bootstrap_deployment.receipt",
        target: "P02-020",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_economics",
        path: "receipts/p02/pass_0080_bootstrap_economics.receipt",
        target: "P02-022",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_redteam",
        path: "receipts/p02/pass_0081_bootstrap_redteam.receipt",
        target: "P02-023",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_closure",
        path: "receipts/p02/pass_0082_bootstrap_closure.receipt",
        target: "P02-024",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_dependency_matrix",
        path: "receipts/p02/pass_0083_bootstrap_dependency_matrix.receipt",
        target: "P02-X01",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_proof_family",
        path: "receipts/p02/pass_0084_bootstrap_proof_family.receipt",
        target: "P02-X02",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_benchmark_pack",
        path: "receipts/p02/pass_0085_bootstrap_benchmark_pack.receipt",
        target: "P02-X03",
        status: "artifact_emitted",
    },
    BootstrapOutputReceiptDescriptor {
        id: "receipt_bootstrap_output_table",
        path: "receipts/p02/pass_0086_bootstrap_output_table.receipt",
        target: "P02-X04",
        status: "artifact_emitted",
    },
];
pub const LYRALANG_BOOTSTRAP_OUTPUT_CONTRACTS: &[BootstrapOutputContractDescriptor] = &[
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_inventory",
        surface: "bootstrap_surface_inventory",
        path: "interfaces/p02/contracts/bootstrap_surface_inventory.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_extinction",
        surface: "bootstrap_extinction_ledger",
        path: "interfaces/p02/contracts/bootstrap_extinction_ledger.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_seed_runtime_contracts",
        surface: "seed_runtime_contracts",
        path: "interfaces/p02/contracts/seed_runtime_contracts.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_deployment",
        surface: "bootstrap_deployment",
        path: "interfaces/p02/contracts/bootstrap_deployment.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_economics",
        surface: "bootstrap_economics",
        path: "interfaces/p02/contracts/bootstrap_economics.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_closure",
        surface: "bootstrap_closure",
        path: "interfaces/p02/contracts/bootstrap_closure.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_dependency_matrix",
        surface: "bootstrap_dependency_matrix",
        path: "interfaces/p02/contracts/bootstrap_dependency_matrix.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_proof_family",
        surface: "bootstrap_proof_family",
        path: "interfaces/p02/contracts/bootstrap_proof_family.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_benchmark_pack",
        surface: "bootstrap_benchmark_pack",
        path: "interfaces/p02/contracts/bootstrap_benchmark_pack.v1.lyra",
        status: "artifact_emitted",
    },
    BootstrapOutputContractDescriptor {
        id: "contract_bootstrap_output_table",
        surface: "bootstrap_output_table",
        path: "interfaces/p02/contracts/bootstrap_output_table.v1.lyra",
        status: "artifact_emitted",
    },
];
pub const LYRALANG_BOOTSTRAP_OUTPUT_GAPS: &[BootstrapOutputGapDescriptor] =
    &[BootstrapOutputGapDescriptor {
        id: "p02_x05_retirement_supersession_law",
        blocker: "p02_x05_required_before_extended_closure",
        next_frontier: "P02-X05",
        owner_root: "ops",
        status: "open",
    }];

fn join(items: &[&str]) -> String {
    let mut v = items.to_vec();
    v.sort();
    v.join(",")
}
fn audience_preimage(item: &BootstrapOutputAudienceDescriptor) -> String {
    format!(
        "id:{}|outputs:{}|artifacts:{}|receipts:{}|status:{}",
        item.id,
        join(item.outputs),
        join(item.artifacts),
        join(item.receipts),
        item.status
    )
}
fn artifact_preimage(item: &BootstrapOutputArtifactDescriptor) -> String {
    format!(
        "id:{}|audience:{}|kind:{}|owner:{}|path:{}|status:{}",
        item.id, item.audience, item.artifact_kind, item.owner_root, item.path, item.status
    )
}
fn receipt_preimage(item: &BootstrapOutputReceiptDescriptor) -> String {
    format!(
        "id:{}|path:{}|target:{}|status:{}",
        item.id, item.path, item.target, item.status
    )
}
fn contract_preimage(item: &BootstrapOutputContractDescriptor) -> String {
    format!(
        "id:{}|surface:{}|path:{}|status:{}",
        item.id, item.surface, item.path, item.status
    )
}
fn gap_preimage(item: &BootstrapOutputGapDescriptor) -> String {
    format!(
        "id:{}|blocker:{}|next:{}|owner:{}|status:{}",
        item.id, item.blocker, item.next_frontier, item.owner_root, item.status
    )
}

pub fn bootstrap_output_audience_ids() -> Vec<&'static str> {
    let mut v = LYRALANG_BOOTSTRAP_OUTPUT_AUDIENCES
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    v.sort();
    v
}
pub fn bootstrap_output_artifact_ids() -> Vec<&'static str> {
    let mut v = LYRALANG_BOOTSTRAP_OUTPUT_ARTIFACTS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    v.sort();
    v
}
pub fn bootstrap_output_receipt_ids() -> Vec<&'static str> {
    let mut v = LYRALANG_BOOTSTRAP_OUTPUT_RECEIPTS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    v.sort();
    v
}
pub fn bootstrap_output_contract_ids() -> Vec<&'static str> {
    let mut v = LYRALANG_BOOTSTRAP_OUTPUT_CONTRACTS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    v.sort();
    v
}
pub fn bootstrap_output_gap_ids() -> Vec<&'static str> {
    let mut v = LYRALANG_BOOTSTRAP_OUTPUT_GAPS
        .iter()
        .map(|item| item.id)
        .collect::<Vec<_>>();
    v.sort();
    v
}
pub fn bootstrap_output_audience_descriptor(
    id: &str,
) -> Option<&'static BootstrapOutputAudienceDescriptor> {
    LYRALANG_BOOTSTRAP_OUTPUT_AUDIENCES
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_output_artifact_descriptor(
    id: &str,
) -> Option<&'static BootstrapOutputArtifactDescriptor> {
    LYRALANG_BOOTSTRAP_OUTPUT_ARTIFACTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_output_receipt_descriptor(
    id: &str,
) -> Option<&'static BootstrapOutputReceiptDescriptor> {
    LYRALANG_BOOTSTRAP_OUTPUT_RECEIPTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_output_contract_descriptor(
    id: &str,
) -> Option<&'static BootstrapOutputContractDescriptor> {
    LYRALANG_BOOTSTRAP_OUTPUT_CONTRACTS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_output_gap_descriptor(id: &str) -> Option<&'static BootstrapOutputGapDescriptor> {
    LYRALANG_BOOTSTRAP_OUTPUT_GAPS
        .iter()
        .find(|item| item.id == id)
}
pub fn bootstrap_output_registry_hash() -> String {
    let rows = [
        LYRALANG_BOOTSTRAP_OUTPUT_AUDIENCES
            .iter()
            .map(audience_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_OUTPUT_ARTIFACTS
            .iter()
            .map(artifact_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_OUTPUT_RECEIPTS
            .iter()
            .map(receipt_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_OUTPUT_CONTRACTS
            .iter()
            .map(contract_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
        LYRALANG_BOOTSTRAP_OUTPUT_GAPS
            .iter()
            .map(gap_preimage)
            .collect::<Vec<_>>()
            .join("\n"),
    ];
    stable_hash_label("lyra.p02.bootstrap_output_table.registry", &rows.join("\n"))
}
pub fn bootstrap_output_carrier_signature() -> String {
    stable_hash_label(
        "lyra.p02.bootstrap_output_table.carrier",
        LYRA_P02_BOOTSTRAP_OUTPUT_TABLE_CARRIER,
    )
}
pub fn bootstrap_output_no_forbidden_descriptor_claims() -> bool {
    LYRA_P02_BOOTSTRAP_OUTPUT_TABLE_CARRIER.contains("closure=extended_open")
        && LYRA_P02_BOOTSTRAP_OUTPUT_TABLE_CARRIER.contains("next=P02-X05")
}
pub fn bootstrap_output_artifacts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_OUTPUT_ARTIFACTS.iter().all(|item| {
        item.path.starts_with("interfaces/p02/")
            || item.path.starts_with("fixtures/p02/")
            || item.path.starts_with("goldens/p02/")
            || item.path.starts_with("ops/p02/")
            || item.path.starts_with("products/p02/")
            || item.path.starts_with("shells/p02/")
            || item.path.starts_with("docs/p02/")
            || item.path.starts_with("examples/p02/")
            || item.path.starts_with("receipts/p02/")
    })
}
pub fn bootstrap_output_receipts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_OUTPUT_RECEIPTS
        .iter()
        .all(|item| item.path.starts_with("receipts/p02/") && item.path.ends_with(".receipt"))
}
pub fn bootstrap_output_contracts_bind_paths() -> bool {
    LYRALANG_BOOTSTRAP_OUTPUT_CONTRACTS.iter().all(|item| {
        item.path.starts_with("interfaces/p02/contracts/") && item.path.ends_with(".lyra")
    })
}
pub fn bootstrap_output_audiences_bind_registry() -> bool {
    LYRALANG_BOOTSTRAP_OUTPUT_AUDIENCES.iter().all(|aud| {
        aud.artifacts
            .iter()
            .all(|id| bootstrap_output_artifact_descriptor(id).is_some())
            && aud
                .receipts
                .iter()
                .all(|id| bootstrap_output_receipt_descriptor(id).is_some())
    })
}
pub fn bootstrap_output_gaps_bind_next_frontier() -> bool {
    LYRALANG_BOOTSTRAP_OUTPUT_GAPS
        .iter()
        .all(|gap| gap.next_frontier == "P02-X05" && gap.status == "open")
}
