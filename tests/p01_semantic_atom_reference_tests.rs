use lyra_phase0::p01::{
    canonical_semantic_atom_reference_registry_hash,
    canonical_semantic_atom_reference_registry_signature, parse_semantic_atom_reference_surface,
    semantic_atom_inspection_tool_descriptor, semantic_atom_inspection_tool_digest,
    semantic_atom_inspection_tool_ids, semantic_atom_reference_all_atoms_exported,
    semantic_atom_reference_example_descriptor, semantic_atom_reference_example_digest,
    semantic_atom_reference_example_ids, semantic_atom_reference_examples_cover_all_atoms,
    semantic_atom_reference_gate_descriptor, semantic_atom_reference_gate_digest,
    semantic_atom_reference_gate_ids, semantic_atom_reference_library_descriptor,
    semantic_atom_reference_library_digest, semantic_atom_reference_library_exports_atom,
    semantic_atom_reference_library_ids, validate_semantic_atom_reference_surface, ErrorCode,
    REQUIRED_SEMANTIC_ATOM_INSPECTION_TOOLS, REQUIRED_SEMANTIC_ATOM_REFERENCE_EXAMPLES,
    REQUIRED_SEMANTIC_ATOM_REFERENCE_GATES, REQUIRED_SEMANTIC_ATOM_REFERENCE_LIBRARIES,
    REQUIRED_SEMANTIC_ATOM_REFERENCE_RECEIPTS, REQUIRED_SEMANTIC_ATOM_REFERENCE_RULES,
};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "fixtures/p01/semantic_atom_reference_inputs/{name}"
    ))
    .expect("fixture must exist")
}

#[test]
fn accepts_valid_semantic_atom_reference_surface() {
    let input = fixture("valid_semantic_atom_reference.lyra");
    let parsed =
        parse_semantic_atom_reference_surface(&input).expect("valid semantic atom reference parse");
    assert_eq!(parsed.phase, "P01");
    assert_eq!(parsed.task, "P01-011");
    assert_eq!(
        parsed.rules.len(),
        REQUIRED_SEMANTIC_ATOM_REFERENCE_RULES.len()
    );
    assert_eq!(
        parsed.libraries.len(),
        REQUIRED_SEMANTIC_ATOM_REFERENCE_LIBRARIES.len()
    );
    assert_eq!(
        parsed.examples.len(),
        REQUIRED_SEMANTIC_ATOM_REFERENCE_EXAMPLES.len()
    );
    assert_eq!(
        parsed.tools.len(),
        REQUIRED_SEMANTIC_ATOM_INSPECTION_TOOLS.len()
    );
    assert_eq!(
        parsed.gates.len(),
        REQUIRED_SEMANTIC_ATOM_REFERENCE_GATES.len()
    );
    assert_eq!(
        parsed.receipts.len(),
        REQUIRED_SEMANTIC_ATOM_REFERENCE_RECEIPTS.len()
    );
    let (verdict, receipt) = validate_semantic_atom_reference_surface(&input);
    assert!(
        verdict.accepted,
        "expected acceptance, got {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P01-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn registry_binds_reference_libraries_examples_tools_and_gates() {
    let signature = canonical_semantic_atom_reference_registry_signature();
    assert!(signature.contains("library:core_atom_reference_library"));
    assert!(signature.contains("example:symbol_atom_reference_example"));
    assert!(signature.contains("tool:semantic_atom_reference_cli"));
    assert!(signature.contains("gate:inspection_is_read_only_gate"));
    assert_eq!(
        semantic_atom_reference_library_ids().len(),
        REQUIRED_SEMANTIC_ATOM_REFERENCE_LIBRARIES.len()
    );
    assert_eq!(
        semantic_atom_reference_example_ids().len(),
        REQUIRED_SEMANTIC_ATOM_REFERENCE_EXAMPLES.len()
    );
    assert_eq!(
        semantic_atom_inspection_tool_ids().len(),
        REQUIRED_SEMANTIC_ATOM_INSPECTION_TOOLS.len()
    );
    assert_eq!(
        semantic_atom_reference_gate_ids().len(),
        REQUIRED_SEMANTIC_ATOM_REFERENCE_GATES.len()
    );
    assert!(canonical_semantic_atom_reference_registry_hash().starts_with("fnv1a128:"));
}

#[test]
fn descriptors_bind_existing_artifacts_and_all_atoms() {
    assert!(semantic_atom_reference_all_atoms_exported());
    assert!(semantic_atom_reference_examples_cover_all_atoms());
    assert!(semantic_atom_reference_library_exports_atom(
        "core_atom_reference_library",
        "symbol"
    ));
    assert!(!semantic_atom_reference_library_exports_atom(
        "core_atom_reference_library",
        "neural_guess"
    ));
    for id in semantic_atom_reference_library_ids() {
        let descriptor =
            semantic_atom_reference_library_descriptor(id).expect("library descriptor exists");
        assert_eq!(descriptor.status, "artifact_emitted");
        assert!(
            std::fs::metadata(descriptor.library_path).is_ok(),
            "missing {}",
            descriptor.library_path
        );
        assert!(semantic_atom_reference_library_digest(id)
            .expect("library digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_atom_reference_example_ids() {
        let descriptor =
            semantic_atom_reference_example_descriptor(id).expect("example descriptor exists");
        assert_eq!(descriptor.status, "artifact_emitted");
        assert!(
            std::fs::metadata(descriptor.example_path).is_ok(),
            "missing {}",
            descriptor.example_path
        );
        assert!(semantic_atom_reference_example_digest(id)
            .expect("example digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_atom_inspection_tool_ids() {
        let descriptor =
            semantic_atom_inspection_tool_descriptor(id).expect("tool descriptor exists");
        assert!(
            std::fs::metadata(descriptor.fixture_path).is_ok(),
            "missing {}",
            descriptor.fixture_path
        );
        assert!(semantic_atom_inspection_tool_digest(id)
            .expect("tool digest")
            .starts_with("fnv1a128:"));
    }
    for id in semantic_atom_reference_gate_ids() {
        let descriptor =
            semantic_atom_reference_gate_descriptor(id).expect("gate descriptor exists");
        assert_eq!(descriptor.status, "execution_proven");
        assert!(semantic_atom_reference_gate_digest(id)
            .expect("gate digest")
            .starts_with("fnv1a128:"));
    }
}

#[test]
fn rejects_required_semantic_atom_reference_gaps() {
    for (fixture_name, expected) in [
        (
            "invalid_missing_rule.lyra",
            ErrorCode::MissingCanonicalModelRule,
        ),
        (
            "invalid_missing_library.lyra",
            ErrorCode::MissingCanonicalModel,
        ),
        (
            "invalid_missing_example.lyra",
            ErrorCode::MissingModelBinding,
        ),
        ("invalid_missing_tool.lyra", ErrorCode::MissingModelBinding),
        ("invalid_missing_gate.lyra", ErrorCode::MissingModelBinding),
        (
            "invalid_missing_receipt.lyra",
            ErrorCode::MissingProofBinding,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_atom_reference_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_descriptor_drift_duplicates_unknown_atoms_and_receipt_target() {
    for (fixture_name, expected) in [
        (
            "invalid_library_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_example_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_tool_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_gate_descriptor_drift.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_duplicate_library.lyra",
            ErrorCode::DuplicateCanonicalModel,
        ),
        (
            "invalid_unknown_atom.lyra",
            ErrorCode::CanonicalModelDriftAccepted,
        ),
        (
            "invalid_receipt_target.lyra",
            ErrorCode::CanonicalModelUnbound,
        ),
        ("invalid_status.lyra", ErrorCode::UnsupportedClosureStatus),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_atom_reference_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}

#[test]
fn rejects_forbidden_semantic_atom_reference_truth_claims() {
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
            "invalid_global_closure_claim.lyra",
            ErrorCode::UnsupportedGlobalClosure,
        ),
        (
            "invalid_placeholder_reference.lyra",
            ErrorCode::PlaceholderAllowed,
        ),
    ] {
        let input = fixture(fixture_name);
        let (verdict, _) = validate_semantic_atom_reference_surface(&input);
        assert!(
            verdict.errors.iter().any(|error| error.code == expected),
            "{fixture_name} should reject with {expected:?}: {:?}",
            verdict.errors
        );
    }
}
