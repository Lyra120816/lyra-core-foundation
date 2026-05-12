use lyra_phase0::p01::{
    parse_error_challenge_evidence_surface, validate_error_challenge_evidence_surface, ErrorCode,
    P01_ERROR_CHALLENGE_EVIDENCE_CONTRACT,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/error_challenge_evidence_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn contract_header_and_golden_are_stable() {
    let input = fixture("valid_error_challenge_evidence.lyra");
    assert!(input.starts_with(P01_ERROR_CHALLENGE_EVIDENCE_CONTRACT));
    let parsed = parse_error_challenge_evidence_surface(&input).expect("valid parse");
    assert_eq!(parsed.header, P01_ERROR_CHALLENGE_EVIDENCE_CONTRACT);
    let (verdict, receipt) = validate_error_challenge_evidence_surface(&input);
    assert!(
        verdict.accepted,
        "expected valid contract: {:?}",
        verdict.errors
    );
    let golden = std::fs::read_to_string("goldens/p01/valid_error_challenge_evidence.receipt")
        .expect("golden must exist");
    assert_eq!(receipt.to_text(), golden);
}

#[test]
fn contract_rejects_bad_header_and_receipt_target() {
    let bad_header = fixture("valid_error_challenge_evidence.lyra")
        .replace(P01_ERROR_CHALLENGE_EVIDENCE_CONTRACT, "BAD");
    let (verdict, _) = validate_error_challenge_evidence_surface(&bad_header);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::InvalidHeader));

    let bad_target = fixture("invalid_receipt_target.lyra");
    let (verdict, _) = validate_error_challenge_evidence_surface(&bad_target);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::CanonicalModelUnbound));
}
