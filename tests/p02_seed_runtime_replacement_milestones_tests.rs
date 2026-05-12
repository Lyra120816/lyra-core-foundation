use lyra_phase0::p02::{
    parse_seed_runtime_replacement_milestone_surface,
    validate_seed_runtime_replacement_milestone_surface, ErrorCode,
    REQUIRED_SEED_RUNTIME_REPLACEMENT_RECEIPTS, REQUIRED_SEED_RUNTIME_REPLACEMENT_RULES,
    REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS, REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGET_CLASSES,
};
fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/seed_runtime_replacement_milestones_inputs/{name}"
    ))
    .expect("fixture must exist")
}
#[test]
fn accepts_valid_seed_runtime_replacement_milestones() {
    let input = fixture("valid_seed_runtime_replacement_milestones.lyra");
    let parsed = parse_seed_runtime_replacement_milestone_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-009");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_SEED_RUNTIME_REPLACEMENT_RULES.len()
    );
    assert_eq!(
        parsed.milestones.len(),
        REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS.len()
    );
    assert_eq!(
        parsed.handoffs.len(),
        REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SEED_RUNTIME_REPLACEMENT_RECEIPTS.len()
    );
    for target in REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGETS {
        assert!(
            parsed.milestone_by_target(target).is_some(),
            "missing milestone {target}"
        );
        assert!(
            parsed.handoff_by_target(target).is_some(),
            "missing handoff {target}"
        );
    }
    for class in REQUIRED_SEED_RUNTIME_REPLACEMENT_TARGET_CLASSES {
        assert!(
            parsed.milestones.iter().any(|x| x.target_class == *class),
            "missing class {class}"
        );
    }
    assert!(parsed.milestones.iter().all(|x| x.holds_phase_open()));
    let (verdict, receipt) = validate_seed_runtime_replacement_milestone_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}
#[test]
fn rejects_invalid_replacement_milestone_rows() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_milestone.lyra",
            ErrorCode::MissingControlSurface,
        ),
        (
            "invalid_duplicate_milestone.lyra",
            ErrorCode::DuplicateControlSurface,
        ),
        (
            "invalid_unknown_target.lyra",
            ErrorCode::InvalidDeploymentTarget,
        ),
        ("invalid_bad_class.lyra", ErrorCode::InvalidDeploymentTarget),
        (
            "invalid_bad_replacement_unit.lyra",
            ErrorCode::InvalidImplementationUnit,
        ),
        (
            "invalid_bad_foreign_surface.lyra",
            ErrorCode::InvalidControlField,
        ),
        (
            "invalid_bad_entry_gate.lyra",
            ErrorCode::InvalidControlField,
        ),
        ("invalid_bad_proof_gate.lyra", ErrorCode::MissingEngineProof),
        (
            "invalid_bad_extinction_gate.lyra",
            ErrorCode::InvalidRollbackPath,
        ),
        (
            "invalid_bad_fallback_ref.lyra",
            ErrorCode::InvalidRollbackPath,
        ),
        (
            "invalid_phase_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_missing_handoff.lyra",
            ErrorCode::MissingControlSurface,
        ),
        (
            "invalid_bad_import_gate.lyra",
            ErrorCode::MissingReplayProof,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_seed_runtime_replacement_milestone_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
#[test]
fn rejects_forbidden_replacement_claims() {
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
            "invalid_placeholder_milestone.lyra",
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
            ErrorCode::UnsupportedClosureStatus,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_seed_runtime_replacement_milestone_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
