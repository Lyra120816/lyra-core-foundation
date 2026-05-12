use lyra_phase0::p02_bootstrap_retirement_law::validate_bootstrap_retirement_supersession_surface;

#[test]
fn bootstrap_retirement_contract_binds_valid_fixture_golden_and_receipt() {
    let contract = std::fs::read_to_string("interfaces/p02/contracts/bootstrap_retirement_supersession.v1.lyra").expect("contract exists");
    for required in [
        "required_header=LYRA-P02-BOOTSTRAP-RETIREMENT-SUPERSESSION v1",
        "required_receipt=receipt_bootstrap_retirement_supersession",
        "next_frontier=P03",
        "parser=ops/p02/src/bootstrap_retirement.rs",
        "valid_fixture=fixtures/p02/bootstrap_retirement_inputs/valid_bootstrap_retirement_supersession.lyra",
        "golden=goldens/p02/valid_bootstrap_retirement_supersession.receipt",
        "receipt=receipts/p02/pass_0087_bootstrap_retirement_supersession.receipt",
    ] { assert!(contract.contains(required), "contract missing {required}"); }
    let input = std::fs::read_to_string("fixtures/p02/bootstrap_retirement_inputs/valid_bootstrap_retirement_supersession.lyra").expect("fixture exists");
    let (verdict, receipt) = validate_bootstrap_retirement_supersession_surface(&input);
    assert!(verdict.accepted, "valid fixture must be accepted: {:?}", verdict.errors);
    let golden = std::fs::read_to_string("goldens/p02/valid_bootstrap_retirement_supersession.receipt").expect("golden exists");
    assert_eq!(receipt.to_text(), golden);
}

#[test]
fn bootstrap_retirement_negative_corpus_is_nonempty_and_rejected() {
    let mut checked = 0usize;
    for entry in std::fs::read_dir("fixtures/p02/bootstrap_retirement_inputs").expect("fixtures dir exists") {
        let path = entry.expect("dir entry").path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else { continue; };
        if !name.starts_with("invalid_") { continue; }
        let input = std::fs::read_to_string(&path).expect("invalid fixture readable");
        let (verdict, _) = validate_bootstrap_retirement_supersession_surface(&input);
        assert!(!verdict.accepted, "{name} must be rejected");
        checked += 1;
    }
    assert!(checked >= 12, "expected broad negative corpus");
}
