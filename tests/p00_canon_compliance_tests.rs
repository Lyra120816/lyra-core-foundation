use lyra_phase0::p00::{validate_canon_compliance_surface, ErrorCode};

const VALID: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/valid_canon_compliance.lyra");
const INVALID_MISSING_MASTER: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_missing_master_source.lyra");
const INVALID_MISSING_ROADMAP: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_missing_roadmap_binding.lyra");
const INVALID_DUPLICATE_ROADMAP: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_duplicate_roadmap_binding.lyra");
const INVALID_ARCHIVE_OVERRIDE: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_archive_override.lyra");
const INVALID_AMBIENT_AUTHORITY: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_ambient_authority.lyra");
const INVALID_CANON_DRIFT: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_canon_drift_accepted.lyra");
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_missing_validation_receipt.lyra");
const INVALID_UNKNOWN_TASK: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_unknown_task.lyra");
const INVALID_SOURCE_UNBOUND: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_source_unbound.lyra");
const INVALID_UNSUPPORTED_CLOSURE: &str =
    include_str!("../fixtures/p00/canon_compliance_inputs/invalid_unsupported_closure.lyra");

fn assert_rejects_with(input: &str, expected: ErrorCode) {
    let (verdict, receipt) = validate_canon_compliance_surface(input);
    assert!(
        !verdict.accepted,
        "input unexpectedly accepted with receipt {}",
        receipt.receipt_hash
    );
    assert!(
        verdict.errors.iter().any(|error| error.code == expected),
        "expected {:?}, got {:?}",
        expected,
        verdict.errors
    );
}

#[test]
fn valid_canon_compliance_surface_is_accepted() {
    let (verdict, receipt) = validate_canon_compliance_surface(VALID);
    assert!(
        verdict.accepted,
        "valid canon compliance rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.verdict.status_token(), "ACCEPTED");
}

#[test]
fn rejects_missing_master_source() {
    assert_rejects_with(INVALID_MISSING_MASTER, ErrorCode::MissingCanonSource);
}

#[test]
fn rejects_missing_required_roadmap_binding() {
    assert_rejects_with(INVALID_MISSING_ROADMAP, ErrorCode::MissingRoadmapBinding);
}

#[test]
fn rejects_duplicate_roadmap_binding() {
    assert_rejects_with(
        INVALID_DUPLICATE_ROADMAP,
        ErrorCode::DuplicateRoadmapBinding,
    );
}

#[test]
fn rejects_archive_authority_override() {
    assert_rejects_with(
        INVALID_ARCHIVE_OVERRIDE,
        ErrorCode::ArchiveAuthorityOverride,
    );
}

#[test]
fn rejects_ambient_canon_authority() {
    assert_rejects_with(INVALID_AMBIENT_AUTHORITY, ErrorCode::AmbientCanonAuthority);
}

#[test]
fn rejects_canon_drift_acceptance() {
    assert_rejects_with(INVALID_CANON_DRIFT, ErrorCode::CanonDriftAccepted);
}

#[test]
fn rejects_missing_validation_receipt() {
    assert_rejects_with(INVALID_MISSING_RECEIPT, ErrorCode::MissingReceiptProof);
}

#[test]
fn rejects_unknown_roadmap_task() {
    assert_rejects_with(INVALID_UNKNOWN_TASK, ErrorCode::InvalidRoadmapBinding);
}

#[test]
fn rejects_unbound_source_reference() {
    assert_rejects_with(INVALID_SOURCE_UNBOUND, ErrorCode::CanonSourceUnbound);
}

#[test]
fn rejects_unsupported_closure_claim() {
    assert_rejects_with(
        INVALID_UNSUPPORTED_CLOSURE,
        ErrorCode::UnsupportedClosureStatus,
    );
}
