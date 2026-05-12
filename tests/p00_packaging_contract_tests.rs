use lyra_phase0::p00::{
    deterministic_packaging_suite_report, validate_packaging_surface, P00_PACKAGING_CONTRACT,
    REQUIRED_PACKAGE_UNITS,
};

#[test]
fn contract_file_binds_p00_019_and_packaging_header() {
    let contract = std::fs::read_to_string("interfaces/p00/contracts/packaging_surface.v1.lyra")
        .expect("contract exists");
    assert!(contract.starts_with(P00_PACKAGING_CONTRACT));
    assert!(contract.contains("task=P00-019"));
    assert!(contract.contains("rejects=missing_package,duplicate_package,missing_bundle,missing_check,unbound_proof,network_required_packaging,packaging_drift,phase_closure"));
}

#[test]
fn packaging_surface_covers_required_packages() {
    let input =
        std::fs::read_to_string("fixtures/p00/packaging_inputs/valid_packaging_surface.lyra")
            .expect("fixture exists");
    let (verdict, receipt) = validate_packaging_surface(&input);
    assert!(
        verdict.accepted,
        "valid packaging surface must accept: {:?}",
        verdict.errors
    );
    assert!(receipt.to_text().contains("verdict=ACCEPTED"));
    for package in REQUIRED_PACKAGE_UNITS {
        assert!(
            input.contains(&format!("package:{package}=")),
            "missing package {package}"
        );
    }
}

#[test]
fn k0_packaging_report_is_stable_and_sorted() {
    let report = deterministic_packaging_suite_report(
        &[
            (
                "p00_receipt_chain".to_string(),
                "receipt_set".to_string(),
                vec!["receipts/p00/pass_0019_packaging.receipt".to_string()],
                vec![
                    "lyra-p00-replay-check".to_string(),
                    "lyra-p00-packaging-check".to_string(),
                ],
                vec!["receipts/p00/pass_0019_packaging.receipt".to_string()],
            ),
            (
                "p00_validator_suite".to_string(),
                "binary_group".to_string(),
                vec!["src/bin/lyra-p00-packaging-check.rs".to_string()],
                vec!["lyra-p00-packaging-check".to_string()],
                vec!["receipts/p00/pass_0019_packaging.receipt".to_string()],
            ),
        ],
        &[(
            "p00_local_truth_gate_bundle".to_string(),
            "001".to_string(),
            vec![
                "p00_validator_suite".to_string(),
                "p00_receipt_chain".to_string(),
            ],
            vec!["products/p00/local_truth_gate.bundle.lyra".to_string()],
            vec!["receipts/p00/pass_0019_packaging.receipt".to_string()],
        )],
        6,
        4,
    );
    assert_eq!(report.package_count, 2);
    assert_eq!(report.package_reports[0].id, "p00_receipt_chain");
    assert_eq!(report.package_reports[1].id, "p00_validator_suite");
    assert!(report.suite_hash.starts_with("fnv1a128:"));
}
