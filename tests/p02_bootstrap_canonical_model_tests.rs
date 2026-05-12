use lyra_phase0::p02::{
    parse_bootstrap_canonical_model_surface, validate_bootstrap_canonical_model_surface, ErrorCode,
    REQUIRED_BOOTSTRAP_CANONICAL_FIELDS, REQUIRED_BOOTSTRAP_CANONICAL_INVARIANTS,
    REQUIRED_BOOTSTRAP_CANONICAL_MODELS, REQUIRED_BOOTSTRAP_CANONICAL_PROOFS,
    REQUIRED_BOOTSTRAP_CANONICAL_RECEIPTS, REQUIRED_BOOTSTRAP_CANONICAL_RELATIONS,
    REQUIRED_BOOTSTRAP_CANONICAL_RULES, REQUIRED_BOOTSTRAP_CANONICAL_SCHEMAS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/bootstrap_canonical_model_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_bootstrap_canonical_model() {
    let input = fixture("valid_bootstrap_canonical_model.lyra");
    let parsed = parse_bootstrap_canonical_model_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-014");
    assert_eq!(parsed.rules.len(), REQUIRED_BOOTSTRAP_CANONICAL_RULES.len());
    assert_eq!(
        parsed.models.len(),
        REQUIRED_BOOTSTRAP_CANONICAL_MODELS.len()
    );
    assert_eq!(
        parsed.schemas.len(),
        REQUIRED_BOOTSTRAP_CANONICAL_SCHEMAS.len()
    );
    assert_eq!(
        parsed.fields.len(),
        REQUIRED_BOOTSTRAP_CANONICAL_FIELDS.len()
    );
    assert_eq!(
        parsed.relations.len(),
        REQUIRED_BOOTSTRAP_CANONICAL_RELATIONS.len()
    );
    assert_eq!(
        parsed.invariants.len(),
        REQUIRED_BOOTSTRAP_CANONICAL_INVARIANTS.len()
    );
    assert_eq!(
        parsed.proofs.len(),
        REQUIRED_BOOTSTRAP_CANONICAL_PROOFS.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_BOOTSTRAP_CANONICAL_RECEIPTS.len()
    );
    for model in REQUIRED_BOOTSTRAP_CANONICAL_MODELS {
        let row = parsed.model_by_id(model).expect("required model present");
        assert!(row.local_schema(), "model must bind a local contract path");
        assert!(
            row.deterministic_hash_policy(),
            "model must use deterministic hash policy"
        );
        assert!(
            !parsed.fields_for_model(model).is_empty(),
            "model must have fields"
        );
        assert!(
            !parsed.invariants_for_model(model).is_empty(),
            "model must have invariant coverage"
        );
    }
    let (verdict, receipt) = validate_bootstrap_canonical_model_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_duplicate_and_invalid_bootstrap_canonical_model() {
    for (name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_model.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        ("invalid_duplicate_model.lyra", ErrorCode::DuplicateEntry),
        ("invalid_bad_model_root.lyra", ErrorCode::InvalidOwnerRoot),
        (
            "invalid_bad_schema_path.lyra",
            ErrorCode::UnknownEvidencePath,
        ),
        (
            "invalid_bad_hash_policy.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_missing_schema.lyra",
            ErrorCode::MissingSchemaBinding,
        ),
        (
            "invalid_bad_schema_encoding.lyra",
            ErrorCode::InvalidSchemaBinding,
        ),
        ("invalid_missing_field.lyra", ErrorCode::MissingFieldBinding),
        (
            "invalid_bad_field_type.lyra",
            ErrorCode::InvalidFieldBinding,
        ),
        (
            "invalid_bad_field_order.lyra",
            ErrorCode::InvalidFieldBinding,
        ),
        (
            "invalid_missing_relation.lyra",
            ErrorCode::MissingModelBinding,
        ),
        (
            "invalid_bad_relation_model.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
        (
            "invalid_missing_invariant.lyra",
            ErrorCode::MissingInvariantBinding,
        ),
        (
            "invalid_bad_invariant_reject.lyra",
            ErrorCode::InvalidInvariantBinding,
        ),
        (
            "invalid_unreceipted_invariant.lyra",
            ErrorCode::UnknownEvidencePath,
        ),
        ("invalid_missing_proof.lyra", ErrorCode::MissingProofBinding),
        (
            "invalid_bad_proof_scope.lyra",
            ErrorCode::InvalidProofBinding,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingReceiptProof,
        ),
        (
            "invalid_bad_receipt_path.lyra",
            ErrorCode::UnknownEvidencePath,
        ),
        (
            "invalid_bad_status.lyra",
            ErrorCode::UnsupportedEvidenceClaim,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_canonical_model_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_bootstrap_canonical_model_claims() {
    for (name, expected) in [
        (
            "invalid_network_required.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_hidden_randomness.lyra",
            ErrorCode::HiddenRandomnessAllowed,
        ),
        ("invalid_ambient_time.lyra", ErrorCode::AmbientTimeAllowed),
        (
            "invalid_probabilistic_field.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_placeholder_model.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_foreign_truth_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_canonical_model_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
