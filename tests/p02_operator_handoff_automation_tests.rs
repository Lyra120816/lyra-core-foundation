use lyra_phase0::p02::{
    parse_operator_handoff_automation_surface, validate_operator_handoff_automation_surface,
    ErrorCode, REQUIRED_OPERATOR_HANDOFF_ARTIFACTS, REQUIRED_OPERATOR_HANDOFF_CAPTURE_CHANNELS,
    REQUIRED_OPERATOR_HANDOFF_GATES, REQUIRED_OPERATOR_HANDOFF_RULES,
    REQUIRED_OPERATOR_HANDOFF_TARGETS, REQUIRED_OPERATOR_HANDOFF_TARGET_CLASSES,
    REQUIRED_OPERATOR_HANDOFF_WORKFLOWS,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/operator_handoff_automation_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_operator_handoff_automation() {
    let input = fixture("valid_operator_handoff_automation.lyra");
    let parsed = parse_operator_handoff_automation_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-011");
    assert_eq!(parsed.rules.len(), REQUIRED_OPERATOR_HANDOFF_RULES.len());
    assert_eq!(
        parsed.workflows.len(),
        REQUIRED_OPERATOR_HANDOFF_WORKFLOWS.len()
    );
    assert_eq!(
        parsed.capture_channels.len(),
        REQUIRED_OPERATOR_HANDOFF_CAPTURE_CHANNELS.len()
    );
    assert_eq!(
        parsed.target_handoffs.len(),
        REQUIRED_OPERATOR_HANDOFF_TARGETS.len()
    );
    assert_eq!(
        parsed.truth_gates.len(),
        REQUIRED_OPERATOR_HANDOFF_GATES.len()
    );
    for workflow in REQUIRED_OPERATOR_HANDOFF_WORKFLOWS {
        assert!(
            parsed.workflow_by_id(workflow).is_some(),
            "missing workflow {workflow}"
        );
    }
    for channel in REQUIRED_OPERATOR_HANDOFF_CAPTURE_CHANNELS {
        assert!(
            parsed.capture_channel_by_id(channel).is_some(),
            "missing channel {channel}"
        );
    }
    for target in REQUIRED_OPERATOR_HANDOFF_TARGETS {
        assert!(
            parsed.target_handoff_by_target(target).is_some(),
            "missing target {target}"
        );
    }
    for class in REQUIRED_OPERATOR_HANDOFF_TARGET_CLASSES {
        assert!(
            parsed
                .target_handoffs
                .iter()
                .any(|handoff| handoff.target_class == *class),
            "missing class {class}"
        );
    }
    assert!(parsed
        .capture_channels
        .iter()
        .all(|channel| channel.offline_only()));
    assert!(parsed
        .target_handoffs
        .iter()
        .all(|handoff| handoff.truth_neutral()));
    for handoff in &parsed.target_handoffs {
        for artifact in REQUIRED_OPERATOR_HANDOFF_ARTIFACTS {
            assert!(
                handoff.required_artifacts.iter().any(|x| x == artifact),
                "missing artifact {artifact}"
            );
        }
    }
    let (verdict, receipt) = validate_operator_handoff_automation_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn rejects_missing_duplicate_and_invalid_operator_handoff_rows() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_workflow.lyra",
            ErrorCode::MissingInterfaceWorkflow,
        ),
        (
            "invalid_duplicate_channel.lyra",
            ErrorCode::DuplicateInterfaceCommand,
        ),
        (
            "invalid_missing_target_handoff.lyra",
            ErrorCode::MissingDeploymentTarget,
        ),
        (
            "invalid_bad_channel_network.lyra",
            ErrorCode::AmbientNetworkAllowed,
        ),
        (
            "invalid_missing_required_artifact.lyra",
            ErrorCode::MissingDeliveryArtifact,
        ),
        (
            "invalid_truth_drift_handoff.lyra",
            ErrorCode::AmbientAuthority,
        ),
        ("invalid_missing_gate.lyra", ErrorCode::MissingReviewGate),
        ("invalid_bad_gate_path.lyra", ErrorCode::UnknownEvidencePath),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        (
            "invalid_bad_status.lyra",
            ErrorCode::UnsupportedClosureStatus,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_operator_handoff_automation_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_operator_handoff_claims() {
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
            "invalid_placeholder_handoff.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
        (
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_operator_handoff_automation_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
