use lyra_phase0::p01::{
    parse_semantic_bedrock_receipts_surface, validate_semantic_bedrock_receipts_surface, ErrorCode,
    P01_SEMANTIC_BEDROCK_RECEIPTS_CONTRACT,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/semantic_bedrock_receipts_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn contract_header_and_golden_are_stable() {
    let input = fixture("valid_semantic_bedrock_receipts.lyra");
    assert!(input.starts_with(P01_SEMANTIC_BEDROCK_RECEIPTS_CONTRACT));
    let parsed = parse_semantic_bedrock_receipts_surface(&input).expect("valid parse");
    assert_eq!(parsed.header, P01_SEMANTIC_BEDROCK_RECEIPTS_CONTRACT);
    let (verdict, receipt) = validate_semantic_bedrock_receipts_surface(&input);
    assert!(
        verdict.accepted,
        "expected valid contract: {:?}",
        verdict.errors
    );
    let golden = std::fs::read_to_string("goldens/p01/valid_semantic_bedrock_receipts.receipt")
        .expect("golden must exist");
    assert_eq!(receipt.to_text(), golden);
}

#[test]
fn contract_rejects_bad_header_and_receipt_target() {
    let bad_header = fixture("valid_semantic_bedrock_receipts.lyra")
        .replace(P01_SEMANTIC_BEDROCK_RECEIPTS_CONTRACT, "BAD");
    let (verdict, _) = validate_semantic_bedrock_receipts_surface(&bad_header);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::InvalidHeader));

    let bad_target = fixture("invalid_receipt_target.lyra");
    let (verdict, _) = validate_semantic_bedrock_receipts_surface(&bad_target);
    assert!(verdict
        .errors
        .iter()
        .any(|error| error.code == ErrorCode::ReceiptHashMismatch));
}

#[test]
fn contract_file_binds_receipt_law_and_operator() {
    let contract =
        std::fs::read_to_string("interfaces/p01/contracts/semantic_bedrock_receipts.v1.lyra")
            .expect("P01-012 semantic bedrock receipts contract must exist");

    assert!(contract.starts_with("LYRA-P01-SEMANTIC-BEDROCK-RECEIPTS-CONTRACT v1"));
    assert!(contract.contains("task=P01-012"));
    assert!(contract.contains("surface=LYRA-P01-SEMANTIC-BEDROCK-RECEIPTS v1"));
    assert!(contract.contains("receipt=receipts/p01/pass_0041_semantic_bedrock_receipts.receipt"));
    assert!(contract.contains("operator=src/bin/lyra-p01-semantic-bedrock-receipts-check.rs"));

    let law = std::fs::read_to_string("ops/p01/control/semantic_bedrock_receipts_law.v1.lyra")
        .expect("P01-012 semantic bedrock receipts law must exist");
    assert!(law.contains("task=P01-012"));
    assert!(law.contains("receipt=receipts/p01/pass_0041_semantic_bedrock_receipts.receipt"));
}
