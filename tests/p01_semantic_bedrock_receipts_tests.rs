use lyra_phase0::p01::{
    canonical_semantic_bedrock_registry_hash, canonical_semantic_bedrock_registry_signature,
    parse_semantic_bedrock_receipts_surface, semantic_bedrock_anchor_descriptor,
    semantic_bedrock_anchor_digest, semantic_bedrock_anchor_ids,
    semantic_bedrock_anchors_point_to_one_core, semantic_bedrock_gate_descriptor,
    semantic_bedrock_gate_digest, semantic_bedrock_gate_ids,
    semantic_bedrock_no_forked_core_claims, semantic_bedrock_parity_fixture_descriptor,
    semantic_bedrock_parity_fixture_digest, semantic_bedrock_parity_fixture_ids,
    semantic_bedrock_parity_fixtures_cover_receipts, semantic_bedrock_receipt_descriptor,
    semantic_bedrock_receipt_digest, semantic_bedrock_receipt_ids,
    semantic_bedrock_receipts_cover_core_chain, validate_semantic_bedrock_receipts_surface,
    ErrorCode, REQUIRED_SEMANTIC_BEDROCK_ANCHORS, REQUIRED_SEMANTIC_BEDROCK_GATES,
    REQUIRED_SEMANTIC_BEDROCK_PARITY_FIXTURES, REQUIRED_SEMANTIC_BEDROCK_RECEIPTS,
    REQUIRED_SEMANTIC_BEDROCK_RECEIPT_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/semantic_bedrock_receipts_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_bedrock_receipts_surface() {
    let input = fixture("valid_semantic_bedrock_receipts.lyra");
    let parsed = parse_semantic_bedrock_receipts_surface(&input)
        .expect("valid semantic bedrock receipts parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-012");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_SEMANTIC_BEDROCK_RECEIPT_RULES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SEMANTIC_BEDROCK_RECEIPTS.len()
    );
    assert_eq!(
        parsed.anchors.len(),
        REQUIRED_SEMANTIC_BEDROCK_ANCHORS.len()
    );
    assert_eq!(
        parsed.fixtures.len(),
        REQUIRED_SEMANTIC_BEDROCK_PARITY_FIXTURES.len()
    );
    assert_eq!(parsed.gates.len(), REQUIRED_SEMANTIC_BEDROCK_GATES.len());
    let (verdict, receipt) = validate_semantic_bedrock_receipts_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_binds_receipts_anchors_fixtures_and_gates() {
    let signature = canonical_semantic_bedrock_registry_signature();
    assert!(signature.contains("receipt:receipt_semantic_atoms"));
    assert!(signature.contains("anchor:semantic_atoms_core_anchor"));
    assert!(signature.contains("fixture:semantic_atoms_receipt_parity"));
    assert!(signature.contains("gate:one_core_anchor_gate"));
    assert_eq!(
        semantic_bedrock_receipt_ids().len(),
        REQUIRED_SEMANTIC_BEDROCK_RECEIPTS.len()
    );
    assert_eq!(
        semantic_bedrock_anchor_ids().len(),
        REQUIRED_SEMANTIC_BEDROCK_ANCHORS.len()
    );
    assert_eq!(
        semantic_bedrock_parity_fixture_ids().len(),
        REQUIRED_SEMANTIC_BEDROCK_PARITY_FIXTURES.len()
    );
    assert_eq!(
        semantic_bedrock_gate_ids().len(),
        REQUIRED_SEMANTIC_BEDROCK_GATES.len()
    );
    assert!(canonical_semantic_bedrock_registry_hash().starts_with("fnv1a128:"));
}

#[test]
fn descriptors_bind_existing_artifacts_and_one_core() {
    assert!(semantic_bedrock_receipts_cover_core_chain());
    assert!(semantic_bedrock_anchors_point_to_one_core());
    assert!(semantic_bedrock_parity_fixtures_cover_receipts());
    assert!(semantic_bedrock_no_forked_core_claims());

    for id in semantic_bedrock_receipt_ids() {
        let descriptor =
            semantic_bedrock_receipt_descriptor(id).expect("receipt descriptor exists");
        assert_eq!(descriptor.status, "artifact_emitted");
        if descriptor.id != "receipt_semantic_bedrock_receipts" {
            assert!(
                std::fs::metadata(descriptor.path).is_ok(),
                "missing {}",
                descriptor.path
            );
        }
        assert!(semantic_bedrock_receipt_digest(id)
            .expect("receipt digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_bedrock_anchor_ids() {
        let descriptor = semantic_bedrock_anchor_descriptor(id).expect("anchor descriptor exists");
        assert_eq!(descriptor.core_ref, "lyra_p01_semantic_core");
        assert!(
            std::fs::metadata(descriptor.contract).is_ok(),
            "missing {}",
            descriptor.contract
        );
        assert!(
            std::fs::metadata(descriptor.law).is_ok(),
            "missing {}",
            descriptor.law
        );
        assert!(semantic_bedrock_anchor_digest(id)
            .expect("anchor digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_bedrock_parity_fixture_ids() {
        let descriptor =
            semantic_bedrock_parity_fixture_descriptor(id).expect("fixture descriptor exists");
        assert!(
            std::fs::metadata(descriptor.positive).is_ok(),
            "missing {}",
            descriptor.positive
        );
        assert!(
            std::fs::metadata(descriptor.negative).is_ok(),
            "missing {}",
            descriptor.negative
        );
        assert!(
            std::fs::metadata(descriptor.golden).is_ok(),
            "missing {}",
            descriptor.golden
        );
        assert!(semantic_bedrock_parity_fixture_digest(id)
            .expect("fixture digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_bedrock_gate_ids() {
        let descriptor = semantic_bedrock_gate_descriptor(id).expect("gate descriptor exists");
        assert_eq!(descriptor.status, "execution_proven");
        assert!(semantic_bedrock_gate_digest(id)
            .expect("gate digest")
            .starts_with("fnv1a128:"));
    }
}

#[test]
fn rejects_required_semantic_bedrock_gaps_and_drift() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingReceiptProof,
        ),
        (
            "invalid_missing_anchor.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_fixture.lyra",
            ErrorCode::MissingProofBinding,
        ),
        ("invalid_missing_gate.lyra", ErrorCode::MissingModelBinding),
        ("invalid_duplicate_receipt.lyra", ErrorCode::DuplicateEntry),
        (
            "invalid_receipt_descriptor_drift.lyra",
            ErrorCode::ReceiptHashMismatch,
        ),
        (
            "invalid_anchor_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_fixture_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_gate_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_orphan_anchor.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_orphan_fixture.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        ("invalid_wrong_task.lyra", ErrorCode::InvalidTask),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
        (
            "invalid_forked_core_claim.lyra",
            ErrorCode::SemanticDriftAccepted,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_bedrock_receipts_surface(&input);
        assert!(!verdict.accepted, "{fixture_name} should reject");
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} expected {:?}, got {:?}",
            expected,
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_bedrock_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_probabilistic_truth.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        (
            "invalid_placeholder_bedrock.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_bedrock_receipts_surface(&input);
        assert!(!verdict.accepted, "{fixture_name} should reject");
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} expected {:?}, got {:?}",
            expected,
            verdict.errors
        );
    }
}
