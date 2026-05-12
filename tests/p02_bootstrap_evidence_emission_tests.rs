use lyra_phase0::p02::{
    parse_bootstrap_evidence_emission_surface, validate_bootstrap_evidence_emission_surface,
    ErrorCode, REQUIRED_BOOTSTRAP_EVIDENCE_CHALLENGE_SUITES, REQUIRED_BOOTSTRAP_EVIDENCE_FIXTURES,
    REQUIRED_BOOTSTRAP_EVIDENCE_PROOF_FAMILIES, REQUIRED_BOOTSTRAP_EVIDENCE_RULES,
    REQUIRED_BOOTSTRAP_EVIDENCE_TARGETS, REQUIRED_BOOTSTRAP_EVIDENCE_TARGET_CLASSES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/bootstrap_evidence_emission_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_bootstrap_evidence_emission() {
    let input = fixture("valid_bootstrap_evidence_emission.lyra");
    let parsed = parse_bootstrap_evidence_emission_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-010");
    assert_eq!(parsed.rules.len(), REQUIRED_BOOTSTRAP_EVIDENCE_RULES.len());
    assert_eq!(
        parsed.fixtures.len(),
        REQUIRED_BOOTSTRAP_EVIDENCE_FIXTURES.len()
    );
    assert_eq!(
        parsed.target_reports.len(),
        REQUIRED_BOOTSTRAP_EVIDENCE_TARGETS.len()
    );
    assert_eq!(
        parsed.challenge_receipts.len(),
        REQUIRED_BOOTSTRAP_EVIDENCE_CHALLENGE_SUITES.len()
    );
    for fixture_id in REQUIRED_BOOTSTRAP_EVIDENCE_FIXTURES {
        assert!(
            parsed.fixture_by_id(fixture_id).is_some(),
            "missing fixture {fixture_id}"
        );
    }
    for target in REQUIRED_BOOTSTRAP_EVIDENCE_TARGETS {
        assert!(
            parsed.target_report_by_target(target).is_some(),
            "missing target report {target}"
        );
    }
    for class in REQUIRED_BOOTSTRAP_EVIDENCE_TARGET_CLASSES {
        assert!(
            parsed
                .target_reports
                .iter()
                .any(|report| report.target_class == *class),
            "missing class {class}"
        );
    }
    for report in &parsed.target_reports {
        assert_eq!(
            report.proof_count,
            REQUIRED_BOOTSTRAP_EVIDENCE_PROOF_FAMILIES.len()
        );
        for family in REQUIRED_BOOTSTRAP_EVIDENCE_PROOF_FAMILIES {
            assert!(
                report.required_families.iter().any(|x| x == family),
                "missing family {family}"
            );
        }
        assert!(report.pending_local_validation());
    }
    for suite in REQUIRED_BOOTSTRAP_EVIDENCE_CHALLENGE_SUITES {
        assert!(
            parsed.challenge_receipt_by_suite(suite).is_some(),
            "missing challenge {suite}"
        );
    }
    assert!(parsed.challenge_receipts.iter().all(|x| x.truth_neutral()));
    let (verdict, receipt) = validate_bootstrap_evidence_emission_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_duplicate_and_invalid_evidence_emission_rows() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_fixture.lyra",
            ErrorCode::MissingAcceptanceGolden,
        ),
        (
            "invalid_duplicate_report.lyra",
            ErrorCode::DuplicateDeploymentTarget,
        ),
        (
            "invalid_missing_target_report.lyra",
            ErrorCode::MissingDeploymentTarget,
        ),
        (
            "invalid_bad_proof_count.lyra",
            ErrorCode::InvalidProofBinding,
        ),
        (
            "invalid_missing_proof_family.lyra",
            ErrorCode::MissingProofBinding,
        ),
        (
            "invalid_missing_challenge_receipt.lyra",
            ErrorCode::MissingChallengeFixture,
        ),
        (
            "invalid_truth_promoting_challenge.lyra",
            ErrorCode::AmbientAuthority,
        ),
        (
            "invalid_unbound_fixture_receipt.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_evidence_emission_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_evidence_emission_claims() {
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
            "invalid_placeholder_evidence.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_bad_status.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_evidence_emission_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
