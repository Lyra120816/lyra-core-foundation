use lyra_phase0::p02::{
    parse_bootstrap_truth_cleanup_surface, validate_bootstrap_truth_cleanup_surface, ErrorCode,
    REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_RECEIPTS, REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_RULES,
    REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS, REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGET_CLASSES,
};
fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/bootstrap_truth_cleanup_inputs/{name}"
    ))
    .expect("fixture must exist")
}
#[test]
fn accepts_valid_bootstrap_truth_cleanup() {
    let input = fixture("valid_bootstrap_truth_cleanup.lyra");
    let parsed = parse_bootstrap_truth_cleanup_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-007");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_RULES.len()
    );
    assert_eq!(
        parsed.cleanups.len(),
        REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS.len()
    );
    assert_eq!(
        parsed.frontiers.len(),
        REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_RECEIPTS.len()
    );
    for target in REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGETS {
        assert!(
            parsed.cleanup_by_target(target).is_some(),
            "missing cleanup {target}"
        );
        assert!(
            parsed.frontier_by_target(target).is_some(),
            "missing frontier {target}"
        );
    }
    for class in REQUIRED_BOOTSTRAP_TRUTH_CLEANUP_TARGET_CLASSES {
        assert!(
            parsed.cleanups.iter().any(|x| x.target_class == *class),
            "missing class {class}"
        );
    }
    assert!(parsed.frontiers.iter().all(|x| x.holds_phase_open()));
    let (verdict, receipt) = validate_bootstrap_truth_cleanup_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}
#[test]
fn rejects_invalid_cleanup_and_frontier_rows() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_cleanup.lyra",
            ErrorCode::MissingControlSurface,
        ),
        (
            "invalid_duplicate_cleanup.lyra",
            ErrorCode::DuplicateControlSurface,
        ),
        (
            "invalid_unknown_target.lyra",
            ErrorCode::InvalidDeploymentTarget,
        ),
        (
            "invalid_bad_proven_action.lyra",
            ErrorCode::InvalidTransitionBinding,
        ),
        (
            "invalid_bad_retired_action.lyra",
            ErrorCode::InvalidRollbackPath,
        ),
        (
            "invalid_bad_truth_update.lyra",
            ErrorCode::InvalidControlField,
        ),
        (
            "invalid_bad_blocker_update.lyra",
            ErrorCode::MissingBlockerBinding,
        ),
        (
            "invalid_bad_frontier_decision.lyra",
            ErrorCode::InvalidControlSurface,
        ),
        (
            "invalid_missing_receipt_binding.lyra",
            ErrorCode::MissingReceiptProof,
        ),
        (
            "invalid_bad_rollback_path.lyra",
            ErrorCode::InvalidRollbackPath,
        ),
        (
            "invalid_missing_frontier.lyra",
            ErrorCode::MissingControlSurface,
        ),
        (
            "invalid_bad_next_frontier.lyra",
            ErrorCode::InvalidControlSurface,
        ),
        (
            "invalid_bad_pending_blocker.lyra",
            ErrorCode::MissingBlockerBinding,
        ),
        (
            "invalid_phase_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_truth_cleanup_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
#[test]
fn rejects_forbidden_truth_cleanup_claims() {
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
            "invalid_placeholder_cleanup.lyra",
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
        let (verdict, _) = validate_bootstrap_truth_cleanup_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
