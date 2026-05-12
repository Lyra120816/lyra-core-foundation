use lyra_phase0::p00::{validate_benchmark_evidence_law_surface, ErrorCode};

const VALID: &str =
    include_str!("../fixtures/p00/benchmark_evidence_law_inputs/valid_benchmark_evidence_law.lyra");
const INVALID_MISSING_BENCHMARK: &str =
    include_str!("../fixtures/p00/benchmark_evidence_law_inputs/invalid_missing_benchmark.lyra");
const INVALID_DUPLICATE_BENCHMARK: &str =
    include_str!("../fixtures/p00/benchmark_evidence_law_inputs/invalid_duplicate_benchmark.lyra");
const INVALID_MISSING_EVIDENCE_FAMILY: &str = include_str!(
    "../fixtures/p00/benchmark_evidence_law_inputs/invalid_missing_evidence_family.lyra"
);
const INVALID_UNSTABLE_BENCHMARK_TARGET: &str = include_str!(
    "../fixtures/p00/benchmark_evidence_law_inputs/invalid_unstable_benchmark_target.lyra"
);
const INVALID_MISSING_RECEIPT_BINDING: &str = include_str!(
    "../fixtures/p00/benchmark_evidence_law_inputs/invalid_missing_receipt_binding.lyra"
);
const INVALID_GLOBAL_DONE_ALLOWED: &str =
    include_str!("../fixtures/p00/benchmark_evidence_law_inputs/invalid_global_done_allowed.lyra");
const INVALID_CLOSURE_UNKNOWN_BENCHMARK: &str = include_str!(
    "../fixtures/p00/benchmark_evidence_law_inputs/invalid_closure_formula_unknown_benchmark.lyra"
);
const INVALID_PHASE_CLOSURE_CLAIM: &str =
    include_str!("../fixtures/p00/benchmark_evidence_law_inputs/invalid_phase_closure_claim.lyra");
const INVALID_MISSING_COMMAND_RECORD: &str = include_str!(
    "../fixtures/p00/benchmark_evidence_law_inputs/invalid_missing_command_record.lyra"
);

fn assert_rejects_with(input: &str, expected: ErrorCode) {
    let (verdict, receipt) = validate_benchmark_evidence_law_surface(input);
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
fn valid_benchmark_evidence_law_is_accepted() {
    let (verdict, receipt) = validate_benchmark_evidence_law_surface(VALID);
    assert!(
        verdict.accepted,
        "valid benchmark/evidence law rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.verdict.status_token(), "ACCEPTED");
}

#[test]
fn rejects_missing_required_benchmark_targets() {
    assert_rejects_with(INVALID_MISSING_BENCHMARK, ErrorCode::MissingBenchmarkTarget);
}

#[test]
fn rejects_duplicate_benchmark_targets() {
    assert_rejects_with(
        INVALID_DUPLICATE_BENCHMARK,
        ErrorCode::DuplicateBenchmarkTarget,
    );
}

#[test]
fn rejects_missing_required_evidence_family() {
    assert_rejects_with(
        INVALID_MISSING_EVIDENCE_FAMILY,
        ErrorCode::MissingEvidenceBinding,
    );
}

#[test]
fn rejects_unstable_benchmark_target() {
    assert_rejects_with(
        INVALID_UNSTABLE_BENCHMARK_TARGET,
        ErrorCode::BenchmarkTargetUnstable,
    );
}

#[test]
fn rejects_missing_receipt_binding() {
    assert_rejects_with(
        INVALID_MISSING_RECEIPT_BINDING,
        ErrorCode::BenchmarkMissingReceipt,
    );
}

#[test]
fn rejects_global_done_allowance() {
    assert_rejects_with(
        INVALID_GLOBAL_DONE_ALLOWED,
        ErrorCode::UnsupportedGlobalClosure,
    );
}

#[test]
fn rejects_unknown_closure_benchmark_binding() {
    assert_rejects_with(
        INVALID_CLOSURE_UNKNOWN_BENCHMARK,
        ErrorCode::UnknownEvidencePath,
    );
}

#[test]
fn rejects_phase_closure_claim() {
    assert_rejects_with(
        INVALID_PHASE_CLOSURE_CLAIM,
        ErrorCode::UnsupportedGlobalClosure,
    );
}

#[test]
fn rejects_missing_command_record() {
    assert_rejects_with(
        INVALID_MISSING_COMMAND_RECORD,
        ErrorCode::MissingCommandRecord,
    );
}
