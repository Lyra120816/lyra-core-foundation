use lyra_phase0::p02_bootstrap_output_table_law::validate_bootstrap_output_table_surface;

#[test]
fn bootstrap_output_table_contract_binds_valid_fixture_golden_and_receipt() {
    let contract =
        std::fs::read_to_string("interfaces/p02/contracts/bootstrap_output_table.v1.lyra")
            .expect("contract exists");
    for required in [
        "required_header=LYRA-P02-BOOTSTRAP-OUTPUT-TABLE v1",
        "required_audiences=developer,operator,product,enterprise,public_interest",
        "required_gap=p02_x05_retirement_supersession_law",
        "next_frontier=P02-X05",
        "parser=ops/p02/src/bootstrap_output_table.rs",
        "valid_fixture=fixtures/p02/bootstrap_output_table_inputs/valid_bootstrap_output_table.lyra",
        "golden=goldens/p02/valid_bootstrap_output_table.receipt",
        "receipt=receipts/p02/pass_0086_bootstrap_output_table.receipt",
    ] {
        assert!(contract.contains(required), "contract missing {required}");
    }
    let input = std::fs::read_to_string(
        "fixtures/p02/bootstrap_output_table_inputs/valid_bootstrap_output_table.lyra",
    )
    .expect("fixture exists");
    let (verdict, receipt) = validate_bootstrap_output_table_surface(&input);
    assert!(
        verdict.accepted,
        "valid fixture must be accepted: {:?}",
        verdict.errors
    );
    let golden = std::fs::read_to_string("goldens/p02/valid_bootstrap_output_table.receipt")
        .expect("golden exists");
    assert_eq!(receipt.to_text(), golden);
}

#[test]
fn bootstrap_output_table_negative_corpus_is_nonempty_and_rejected() {
    let mut checked = 0usize;
    for entry in std::fs::read_dir("fixtures/p02/bootstrap_output_table_inputs")
        .expect("fixtures dir exists")
    {
        let path = entry.expect("dir entry").path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !name.starts_with("invalid_") {
            continue;
        }
        let input = std::fs::read_to_string(&path).expect("invalid fixture readable");
        let (verdict, _) = validate_bootstrap_output_table_surface(&input);
        assert!(!verdict.accepted, "{name} must be rejected");
        checked += 1;
    }
    assert!(checked >= 20, "expected broad negative corpus");
}
