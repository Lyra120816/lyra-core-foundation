use lyra_phase0::p00::{
    deterministic_interface_manifest_report, validate_developer_operator_interface_surface,
    P00_DEVELOPER_OPERATOR_INTERFACE_CONTRACT, REQUIRED_INTERFACE_COMMANDS,
};

#[test]
fn contract_file_binds_p00_018_and_interface_header() {
    let contract =
        std::fs::read_to_string("interfaces/p00/contracts/developer_operator_interface.v1.lyra")
            .expect("contract exists");
    assert!(contract.starts_with(P00_DEVELOPER_OPERATOR_INTERFACE_CONTRACT));
    assert!(contract.contains("task=P00-018"));
    assert!(contract.contains("rejects=manual_only_interface,network_required_interface,interface_drift,unbound_proof,phase_closure"));
}

#[test]
fn interface_surface_covers_every_existing_p00_command() {
    let input = std::fs::read_to_string(
        "fixtures/p00/developer_operator_interface_inputs/valid_developer_operator_interface.lyra",
    )
    .expect("fixture exists");
    let (verdict, receipt) = validate_developer_operator_interface_surface(&input);
    assert!(
        verdict.accepted,
        "valid interface surface must accept: {:?}",
        verdict.errors
    );
    assert!(receipt.to_text().contains("verdict=ACCEPTED"));
    for command in REQUIRED_INTERFACE_COMMANDS {
        assert!(
            input.contains(&format!("command:{command}=")),
            "missing command {command}"
        );
    }
}

#[test]
fn k0_manifest_report_is_stable_and_sorted() {
    let report = deterministic_interface_manifest_report(
        &[
            (
                "validate_replay_witness".to_string(),
                "lyra-p00-replay-check".to_string(),
                "LYRA-P00-REPLAY-WITNESS v1".to_string(),
                vec!["receipts/p00/pass_0017_replay_witness.receipt".to_string()],
                vec!["operator".to_string(), "developer".to_string()],
            ),
            (
                "validate_interface_manifest".to_string(),
                "lyra-p00-interface-check".to_string(),
                P00_DEVELOPER_OPERATOR_INTERFACE_CONTRACT.to_string(),
                vec!["receipts/p00/pass_0018_developer_operator_interface.receipt".to_string()],
                vec!["developer".to_string(), "operator".to_string()],
            ),
        ],
        4,
        4,
        4,
    );
    assert_eq!(report.command_count, 2);
    assert_eq!(report.commands[0].id, "validate_interface_manifest");
    assert_eq!(report.commands[1].id, "validate_replay_witness");
    assert!(report.manifest_hash.starts_with("fnv1a128:"));
}
