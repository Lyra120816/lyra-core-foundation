use lyra_phase0::p01::{
    bytes_to_hex, canonical_core_ir_registry_signature, canonical_text_ir, encode_binary_ir_frame,
    round_trip_binary_identity, round_trip_text_identity, validate_core_ir_surface, ErrorCode,
    REQUIRED_CORE_IR_FORMS, REQUIRED_CORE_IR_PARITIES, REQUIRED_CORE_IR_RECEIPTS,
    REQUIRED_CORE_IR_RULES, REQUIRED_CORE_IR_UPGRADES, REQUIRED_CORE_IR_VERSIONS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/core_ir_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_core_ir_surface() {
    let input = fixture("valid_core_ir_forms.lyra");
    let parsed = lyra_phase0::p01::parse_core_ir_surface(&input).expect("valid core ir parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-002");
    assert_eq!(parsed.rules.len(), REQUIRED_CORE_IR_RULES.len());
    assert_eq!(parsed.forms.len(), REQUIRED_CORE_IR_FORMS.len());
    assert_eq!(parsed.versions.len(), REQUIRED_CORE_IR_VERSIONS.len());
    assert_eq!(parsed.upgrades.len(), REQUIRED_CORE_IR_UPGRADES.len());
    assert_eq!(parsed.parities.len(), REQUIRED_CORE_IR_PARITIES.len());
    assert_eq!(parsed.receipts.len(), REQUIRED_CORE_IR_RECEIPTS.len());
    let (verdict, receipt) = validate_core_ir_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn canonical_core_ir_registry_is_stable_and_complete() {
    let signature = canonical_core_ir_registry_signature();
    assert!(signature.contains("ir_form:text_ir|medium:text|version:ir_v1"));
    assert!(signature.contains("ir_form:binary_ir|medium:binary|version:ir_v1"));
    assert!(signature.contains("encoding:length_prefixed_big_endian"));
}

#[test]
fn text_and_binary_round_trip_identity_is_byte_stable() {
    let text = canonical_text_ir("symbol", "symbol", "lyra.symbol").expect("text ir");
    assert!(round_trip_text_identity(&text).expect("text round trip"));
    let binary = encode_binary_ir_frame("symbol", "symbol", "lyra.symbol").expect("binary ir");
    assert!(round_trip_binary_identity(&binary).expect("binary round trip"));
    let hex = bytes_to_hex(&binary);
    assert_eq!(fixture("binary_minimal_symbol.lyrairb.hex").trim(), hex);
}

#[test]
fn rejects_required_core_ir_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_form.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_version.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_upgrade.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_missing_parity.lyra",
            ErrorCode::MissingFixtureProof,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_core_ir_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_duplicate_drift_and_unbound_core_ir_rows() {
    for (fixture_name, expected) in [
        (
            "invalid_duplicate_form.lyra",
            ErrorCode::DuplicateCanonicalModel,
        ),
        (
            "invalid_unknown_version_reference.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
        (
            "invalid_binary_header_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_unknown_atom.lyra",
            ErrorCode::InvalidCanonicalModel,
        ),
        (
            "invalid_forward_unknown_required_accepted.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_core_ir_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_core_ir_truth_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_ir.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        ("invalid_placeholder_ir.lyra", ErrorCode::PlaceholderAllowed),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_core_ir_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
