use lyra_phase0::p02::{
    parse_bootstrap_emergency_fallback_surface, validate_bootstrap_emergency_fallback_surface,
    ErrorCode, REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_RECEIPTS,
    REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_RULES, REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS,
    REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGET_CLASSES,
};
fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/bootstrap_emergency_fallback_inputs/{name}"
    ))
    .expect("fixture must exist")
}
#[test]
fn accepts_valid_bootstrap_emergency_fallback() {
    let input = fixture("valid_bootstrap_emergency_fallback.lyra");
    let parsed = parse_bootstrap_emergency_fallback_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-008");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_RULES.len()
    );
    assert_eq!(
        parsed.fallbacks.len(),
        REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS.len()
    );
    assert_eq!(
        parsed.rollbacks.len(),
        REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_RECEIPTS.len()
    );
    for target in REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGETS {
        assert!(
            parsed.fallback_by_target(target).is_some(),
            "missing fallback {target}"
        );
        assert!(
            parsed.rollback_by_target(target).is_some(),
            "missing rollback {target}"
        );
    }
    for class in REQUIRED_BOOTSTRAP_EMERGENCY_FALLBACK_TARGET_CLASSES {
        assert!(
            parsed.fallbacks.iter().any(|x| x.target_class == *class),
            "missing class {class}"
        );
    }
    assert!(parsed.fallbacks.iter().all(|x| x.holds_phase_open()));
    let (verdict, receipt) = validate_bootstrap_emergency_fallback_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}
#[test]
fn rejects_invalid_fallback_and_rollback_rows() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_fallback.lyra",
            ErrorCode::MissingControlSurface,
        ),
        (
            "invalid_duplicate_fallback.lyra",
            ErrorCode::DuplicateControlSurface,
        ),
        (
            "invalid_unknown_target.lyra",
            ErrorCode::InvalidDeploymentTarget,
        ),
        ("invalid_bad_class.lyra", ErrorCode::InvalidDeploymentTarget),
        (
            "invalid_bad_failure_state.lyra",
            ErrorCode::InvalidControlField,
        ),
        (
            "invalid_bad_freeze_action.lyra",
            ErrorCode::InvalidControlField,
        ),
        (
            "invalid_bad_fallback_action.lyra",
            ErrorCode::InvalidControlSurface,
        ),
        (
            "invalid_bad_rollback_path.lyra",
            ErrorCode::InvalidRollbackPath,
        ),
        (
            "invalid_bad_last_good_receipt.lyra",
            ErrorCode::RollbackWithoutReceipt,
        ),
        (
            "invalid_missing_challenge.lyra",
            ErrorCode::MissingChallengeFixture,
        ),
        (
            "invalid_bad_operator_state.lyra",
            ErrorCode::InvalidControlField,
        ),
        (
            "invalid_phase_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_missing_rollback.lyra",
            ErrorCode::MissingRollbackPath,
        ),
        (
            "invalid_bad_rollback_trigger.lyra",
            ErrorCode::InvalidRollbackAuthority,
        ),
        (
            "invalid_bad_replay_gate.lyra",
            ErrorCode::MissingReplayProof,
        ),
        (
            "invalid_bad_frontier_decision.lyra",
            ErrorCode::InvalidControlSurface,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_emergency_fallback_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
#[test]
fn rejects_forbidden_emergency_fallback_claims() {
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
            "invalid_placeholder_fallback.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_probabilistic_truth.lyra",
            ErrorCode::ProbabilisticTruthAllowed,
        ),
        (
            "invalid_bad_status.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_emergency_fallback_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
