use lyra_phase0::p02::{
    parse_foreign_surface_closure_surface, validate_foreign_surface_closure_surface, ErrorCode,
    REQUIRED_FOREIGN_BOOTSTRAP_CLASSES, REQUIRED_FOREIGN_BOOTSTRAP_SURFACES,
    REQUIRED_FOREIGN_SURFACE_CHALLENGE_SUITES, REQUIRED_FOREIGN_SURFACE_CLOSURE_RECEIPTS,
    REQUIRED_FOREIGN_SURFACE_CLOSURE_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/foreign_surface_closure_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_foreign_surface_closure() {
    let input = fixture("valid_foreign_surface_closure.lyra");
    let parsed = parse_foreign_surface_closure_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-012");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_FOREIGN_SURFACE_CLOSURE_RULES.len()
    );
    assert_eq!(
        parsed.surfaces.len(),
        REQUIRED_FOREIGN_BOOTSTRAP_SURFACES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_FOREIGN_SURFACE_CLOSURE_RECEIPTS.len()
    );
    for surface in REQUIRED_FOREIGN_BOOTSTRAP_SURFACES {
        let row = parsed
            .surface_by_id(surface)
            .expect("required surface present");
        assert!(row.visible(), "surface must be visible");
        assert!(row.bounded(), "surface must be bounded");
        assert!(row.challengeable(), "surface must be challengeable");
        assert!(row.closure_paired(), "surface must be closure paired");
        assert!(
            row.truth_neutral(),
            "surface must not advance truth directly"
        );
        assert!(
            parsed.challenge_for_surface(surface).is_some(),
            "surface challenge missing"
        );
        assert!(
            parsed.closure_law_for_surface(surface).is_some(),
            "surface closure law missing"
        );
        assert!(
            parsed.visibility_for_surface(surface).is_some(),
            "surface visibility proof missing"
        );
    }
    for class in REQUIRED_FOREIGN_BOOTSTRAP_CLASSES {
        assert!(
            parsed.surfaces.iter().any(|x| x.surface_class == *class),
            "missing class {class}"
        );
    }
    for suite in REQUIRED_FOREIGN_SURFACE_CHALLENGE_SUITES {
        assert!(
            parsed.challenges.iter().any(|x| x.suite_id == *suite),
            "missing suite {suite}"
        );
    }
    let (verdict, receipt) = validate_foreign_surface_closure_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_duplicate_and_invalid_foreign_surface_rows() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_surface.lyra",
            ErrorCode::MissingDeliveryArtifact,
        ),
        ("invalid_duplicate_surface.lyra", ErrorCode::DuplicateEntry),
        (
            "invalid_not_visible.lyra",
            ErrorCode::MissingEvidenceBinding,
        ),
        (
            "invalid_unbounded_surface.lyra",
            ErrorCode::ClosureDriftAccepted,
        ),
        (
            "invalid_missing_challenge.lyra",
            ErrorCode::MissingChallengeFixture,
        ),
        (
            "invalid_bad_challenge_suite.lyra",
            ErrorCode::InvalidChallengeFixture,
        ),
        (
            "invalid_missing_closure_law.lyra",
            ErrorCode::MissingClosureTask,
        ),
        (
            "invalid_bad_closure_scope.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_missing_visibility.lyra",
            ErrorCode::MissingEvidenceBinding,
        ),
        (
            "invalid_bad_visibility_path.lyra",
            ErrorCode::UnknownEvidencePath,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        (
            "invalid_bad_receipt_path.lyra",
            ErrorCode::UnknownEvidencePath,
        ),
        (
            "invalid_bad_status.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
        (
            "invalid_foreign_truth_drift.lyra",
            ErrorCode::ClosureDriftAccepted,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_foreign_surface_closure_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_foreign_surface_closure_claims() {
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
            "invalid_probabilistic_truth.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_placeholder_closure.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_foreign_surface_closure_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
