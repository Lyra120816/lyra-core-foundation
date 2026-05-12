use lyra_phase0::p02::{
    parse_bootstrap_target_matrix_surface, validate_bootstrap_target_matrix_surface, ErrorCode,
    REQUIRED_BOOTSTRAP_TARGETS, REQUIRED_BOOTSTRAP_TARGET_CLASSES,
    REQUIRED_BOOTSTRAP_TARGET_MATRIX_RULES, REQUIRED_BOOTSTRAP_TARGET_PROOF_FAMILIES,
    REQUIRED_BOOTSTRAP_TARGET_RECEIPTS,
};
fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p02/bootstrap_target_matrix_inputs/{name}"
    ))
    .expect("fixture must exist")
}
#[test]
fn accepts_valid_bootstrap_target_matrix() {
    let input = fixture("valid_bootstrap_target_matrix.lyra");
    let parsed = parse_bootstrap_target_matrix_surface(&input).expect("valid parses");
    assert_eq!(parsed.phase, "P02");
    assert_eq!(parsed.task, "P02-006");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_BOOTSTRAP_TARGET_MATRIX_RULES.len()
    );
    assert_eq!(parsed.targets.len(), REQUIRED_BOOTSTRAP_TARGETS.len());
    assert_eq!(
        parsed.proofs.len(),
        REQUIRED_BOOTSTRAP_TARGETS.len() * REQUIRED_BOOTSTRAP_TARGET_PROOF_FAMILIES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_BOOTSTRAP_TARGET_RECEIPTS.len()
    );
    assert!(parsed.target_by_id("target_linux_x86_64").is_some());
    assert!(parsed.target_by_id("target_baremetal_riscv64").is_some());
    assert!(parsed.targets.iter().all(|t| t.binds_bootstrap_surface()));
    for class in REQUIRED_BOOTSTRAP_TARGET_CLASSES {
        assert!(
            parsed.targets.iter().any(|t| t.target_class == *class),
            "missing class {class}"
        );
    }
    for target in REQUIRED_BOOTSTRAP_TARGETS {
        for family in REQUIRED_BOOTSTRAP_TARGET_PROOF_FAMILIES {
            assert!(
                parsed
                    .proofs
                    .iter()
                    .any(|p| p.target_id == *target && p.proof_family == *family),
                "missing {target} {family}"
            );
        }
    }
    let (verdict, receipt) = validate_bootstrap_target_matrix_surface(&input);
    assert!(
        verdict.accepted,
        "expected accepted got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}
#[test]
fn rejects_missing_duplicate_and_invalid_matrix_rows() {
    for (name, expected) in [
        ("invalid_missing_rule.lyra", ErrorCode::MissingClosureRule),
        (
            "invalid_missing_target.lyra",
            ErrorCode::MissingDeploymentTarget,
        ),
        (
            "invalid_duplicate_target.lyra",
            ErrorCode::DuplicateDeploymentTarget,
        ),
        (
            "invalid_bad_target_class.lyra",
            ErrorCode::InvalidDeploymentTarget,
        ),
        (
            "invalid_bad_architecture.lyra",
            ErrorCode::InvalidDeploymentTarget,
        ),
        ("invalid_bad_owner_root.lyra", ErrorCode::InvalidOwnerRoot),
        ("invalid_missing_proof.lyra", ErrorCode::MissingProofBinding),
        (
            "invalid_unknown_proof_target.lyra",
            ErrorCode::InvalidProofBinding,
        ),
        (
            "invalid_bad_proof_family.lyra",
            ErrorCode::InvalidProofBinding,
        ),
        (
            "invalid_bad_host_boundary_gate.lyra",
            ErrorCode::InvalidChallengeFixture,
        ),
        (
            "invalid_missing_evidence.lyra",
            ErrorCode::MissingEvidenceBinding,
        ),
        (
            "invalid_unreceipted_target.lyra",
            ErrorCode::ClosureProofUnbound,
        ),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingClosureProof,
        ),
        ("invalid_bad_task.lyra", ErrorCode::InvalidTask),
    ] {
        let input = fixture(name);
        let (verdict, _) = validate_bootstrap_target_matrix_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
#[test]
fn rejects_forbidden_target_matrix_claims() {
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
            "invalid_placeholder_matrix.lyra",
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
        let (verdict, _) = validate_bootstrap_target_matrix_surface(&input);
        assert!(
            verdict.errors.iter().any(|e| e.code == expected),
            "{name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
