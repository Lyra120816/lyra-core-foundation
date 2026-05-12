use lyra_phase0::p01::{
    canonical_core_ir_reuse_registry_hash, canonical_core_ir_reuse_registry_signature,
    core_ir_reuse_consumer_descriptor, core_ir_reuse_consumer_digest, core_ir_reuse_consumer_ids,
    core_ir_reuse_edge_descriptor, core_ir_reuse_edge_digest,
    core_ir_reuse_edge_endpoints_are_bound, core_ir_reuse_edge_ids, core_ir_reuse_gate_descriptor,
    core_ir_reuse_gate_digest, core_ir_reuse_gate_ids, core_ir_reuse_ref_is_bound,
    parse_core_ir_reuse_surface, validate_core_ir_reuse_surface, ErrorCode,
    REQUIRED_CORE_IR_REUSE_CONSUMERS, REQUIRED_CORE_IR_REUSE_EDGES, REQUIRED_CORE_IR_REUSE_GATES,
    REQUIRED_CORE_IR_REUSE_RECEIPTS, REQUIRED_CORE_IR_REUSE_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("fixtures/p01/core_ir_reuse_inputs/{name}"))
        .expect("fixture must exist")
}

#[test]
fn accepts_valid_core_ir_reuse_surface() {
    let input = fixture("valid_core_ir_reuse.lyra");
    let parsed = parse_core_ir_reuse_surface(&input).expect("valid core ir reuse parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-010");
    assert_eq!(parsed.rules.len(), REQUIRED_CORE_IR_REUSE_RULES.len());
    assert_eq!(
        parsed.consumers.len(),
        REQUIRED_CORE_IR_REUSE_CONSUMERS.len()
    );
    assert_eq!(parsed.edges.len(), REQUIRED_CORE_IR_REUSE_EDGES.len());
    assert_eq!(parsed.gates.len(), REQUIRED_CORE_IR_REUSE_GATES.len());
    assert_eq!(parsed.receipts.len(), REQUIRED_CORE_IR_REUSE_RECEIPTS.len());
    let (verdict, receipt) = validate_core_ir_reuse_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_binds_all_cross_phase_consumers_edges_and_gates() {
    let signature = canonical_core_ir_reuse_registry_signature();
    assert!(signature.contains("consumer:parser_surface"));
    assert!(signature.contains("consumer:product_surface"));
    assert!(signature.contains("edge:evaluator_to_vm_ir_edge"));
    assert!(signature.contains("gate:single_ir_contract_gate"));
    assert_eq!(
        core_ir_reuse_consumer_ids().len(),
        REQUIRED_CORE_IR_REUSE_CONSUMERS.len()
    );
    assert_eq!(
        core_ir_reuse_edge_ids().len(),
        REQUIRED_CORE_IR_REUSE_EDGES.len()
    );
    assert_eq!(
        core_ir_reuse_gate_ids().len(),
        REQUIRED_CORE_IR_REUSE_GATES.len()
    );
    assert!(canonical_core_ir_reuse_registry_hash().starts_with("fnv1a128:"));
}

#[test]
fn descriptors_bind_existing_core_ir_forms_and_fixtures() {
    assert!(core_ir_reuse_ref_is_bound("core_ir:text_ir"));
    assert!(core_ir_reuse_ref_is_bound("core_ir:binary_ir"));
    assert!(!core_ir_reuse_ref_is_bound("core_ir:private_ir"));
    for id in core_ir_reuse_consumer_ids() {
        let descriptor = core_ir_reuse_consumer_descriptor(id).expect("consumer descriptor exists");
        assert_eq!(descriptor.status, "artifact_emitted");
        assert!(core_ir_reuse_ref_is_bound(descriptor.core_ir_ref));
        assert!(
            std::fs::metadata(descriptor.fixture_path).is_ok(),
            "missing {}",
            descriptor.fixture_path
        );
        assert!(core_ir_reuse_consumer_digest(id)
            .expect("consumer digest")
            .starts_with("fnv1a128:"));
    }
    for id in core_ir_reuse_edge_ids() {
        let descriptor = core_ir_reuse_edge_descriptor(id).expect("edge descriptor exists");
        assert_eq!(descriptor.status, "execution_proven");
        assert!(core_ir_reuse_edge_endpoints_are_bound(descriptor));
        assert!(core_ir_reuse_edge_digest(id)
            .expect("edge digest")
            .starts_with("fnv1a128:"));
    }
    for id in core_ir_reuse_gate_ids() {
        let descriptor = core_ir_reuse_gate_descriptor(id).expect("gate descriptor exists");
        assert_eq!(descriptor.status, "execution_proven");
        assert!(core_ir_reuse_gate_digest(id)
            .expect("gate digest")
            .starts_with("fnv1a128:"));
    }
}

#[test]
fn rejects_required_core_ir_reuse_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_consumer.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        ("invalid_missing_edge.lyra", ErrorCode::MissingModelBinding),
        ("invalid_missing_gate.lyra", ErrorCode::MissingModelBinding),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_core_ir_reuse_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_descriptor_drift_duplicates_status_and_receipt_target() {
    for (fixture_name, expected) in [
        (
            "invalid_consumer_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_edge_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_gate_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_duplicate_consumer.lyra",
            ErrorCode::DuplicateCanonicalModel,
        ),
        (
            "invalid_receipt_target.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_core_ir_reuse_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_core_ir_reuse_truth_claims() {
    for (fixture_name, expected) in [
        (
            "invalid_private_ir_fork.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
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
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_core_ir_reuse_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
