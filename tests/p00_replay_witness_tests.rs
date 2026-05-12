use lyra_phase0::p00::{deterministic_replay_report, validate_replay_witness_surface, ErrorCode};

const VALID: &str = include_str!("../fixtures/p00/replay_witness_inputs/valid_replay_witness.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_missing_replay_rule.lyra");
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_missing_receipt.lyra");
const INVALID_DUPLICATE_RECEIPT: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_duplicate_receipt.lyra");
const INVALID_BAD_HASH: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_bad_hash.lyra");
const INVALID_MISSING_WITNESS: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_missing_witness.lyra");
const INVALID_DUPLICATE_ORDER: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_duplicate_witness_order.lyra");
const INVALID_MISSING_LINK: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_missing_chain_link.lyra");
const INVALID_UNKNOWN_RECEIPT: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_unknown_receipt_reference.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_missing_replay_proof.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_ORPHAN_RECEIPT: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_orphan_receipt.lyra");
const INVALID_MUTABLE_REPLAY: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_mutable_replay.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p00/replay_witness_inputs/invalid_phase_closure_claim.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_replay_witness_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_replay_witness_surface() {
    let (verdict, receipt) = validate_replay_witness_surface(VALID);
    assert!(
        verdict.accepted,
        "valid replay witness rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P00-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn deterministic_replay_report_is_stable_and_sorted() {
    let receipts = [("z_receipt", "receipt_z"), ("a_receipt", "receipt_a")];
    let witnesses = [("z_witness", "witness_z"), ("a_witness", "witness_a")];
    let left = deterministic_replay_report("P00-017", &receipts, &witnesses);
    let right = deterministic_replay_report("P00-017", &receipts, &witnesses);
    assert_eq!(left, right);
    assert_eq!(left.receipt_count, 2);
    assert_eq!(left.witness_count, 2);
    assert_eq!(left.receipts[0].order, "001");
    assert_eq!(left.receipts[0].receipt_id, "a_receipt");
    assert_eq!(left.witnesses[0].witness_id, "a_witness");
}

#[test]
fn rejects_missing_replay_rule() {
    assert_rejects_with(INVALID_MISSING_RULE, ErrorCode::MissingReplayRule);
}
#[test]
fn rejects_missing_replay_receipt() {
    assert_rejects_with(INVALID_MISSING_RECEIPT, ErrorCode::MissingReplayReceipt);
}
#[test]
fn rejects_duplicate_replay_receipt() {
    assert_rejects_with(INVALID_DUPLICATE_RECEIPT, ErrorCode::DuplicateReplayReceipt);
}
#[test]
fn rejects_bad_hash() {
    assert_rejects_with(INVALID_BAD_HASH, ErrorCode::ReceiptHashMismatch);
}
#[test]
fn rejects_missing_replay_witness() {
    assert_rejects_with(INVALID_MISSING_WITNESS, ErrorCode::MissingReplayWitness);
}
#[test]
fn rejects_duplicate_witness_order() {
    assert_rejects_with(INVALID_DUPLICATE_ORDER, ErrorCode::InvalidReplayWitness);
}
#[test]
fn rejects_missing_chain_link() {
    assert_rejects_with(INVALID_MISSING_LINK, ErrorCode::MissingReceiptChainBinding);
}
#[test]
fn rejects_unknown_receipt_reference() {
    assert_rejects_with(INVALID_UNKNOWN_RECEIPT, ErrorCode::ReplayProofUnbound);
}
#[test]
fn rejects_missing_replay_proof() {
    assert_rejects_with(INVALID_MISSING_PROOF, ErrorCode::MissingReplayProof);
}
#[test]
fn rejects_unbound_proof_reference() {
    assert_rejects_with(INVALID_UNBOUND_PROOF, ErrorCode::ReplayProofUnbound);
}
#[test]
fn rejects_orphan_receipt_binding() {
    assert_rejects_with(INVALID_ORPHAN_RECEIPT, ErrorCode::OrphanReceiptBinding);
}
#[test]
fn rejects_mutable_replay() {
    assert_rejects_with(INVALID_MUTABLE_REPLAY, ErrorCode::ReplayDriftAccepted);
}
#[test]
fn rejects_phase_closure_claim() {
    assert_rejects_with(INVALID_PHASE_CLOSURE, ErrorCode::UnsupportedGlobalClosure);
}
