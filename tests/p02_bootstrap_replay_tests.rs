use lyra_phase0::p02::{
    deterministic_bootstrap_replay_suite_report, validate_bootstrap_replay_surface, ErrorCode,
    REQUIRED_BOOTSTRAP_REPLAY_ARTIFACTS, REQUIRED_BOOTSTRAP_REPLAY_LINKS,
    REQUIRED_BOOTSTRAP_REPLAY_PROOFS, REQUIRED_BOOTSTRAP_REPLAY_RECEIPTS,
    REQUIRED_BOOTSTRAP_REPLAY_WITNESSES,
};

const VALID: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/valid_bootstrap_replay.lyra");
const INVALID_MISSING_RULE: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_missing_replay_rule.lyra");
const INVALID_MISSING_RECEIPT: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_missing_receipt.lyra");
const INVALID_DUPLICATE_RECEIPT: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_duplicate_receipt.lyra");
const INVALID_BAD_HASH: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_bad_receipt_hash.lyra");
const INVALID_MISSING_WITNESS: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_missing_witness.lyra");
const INVALID_DUPLICATE_ORDER: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_duplicate_witness_order.lyra");
const INVALID_MISSING_LINK: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_missing_chain_link.lyra");
const INVALID_UNKNOWN_RECEIPT: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_unknown_receipt_reference.lyra");
const INVALID_MISSING_PROOF: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_missing_replay_proof.lyra");
const INVALID_UNBOUND_PROOF: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_unbound_proof_reference.lyra");
const INVALID_ORPHAN_RECEIPT: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_orphan_receipt.lyra");
const INVALID_MUTABLE_REPLAY: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_mutable_replay.lyra");
const INVALID_PHASE_CLOSURE: &str =
    include_str!("../fixtures/p02/bootstrap_replay_inputs/invalid_phase_closure_claim.lyra");

fn assert_rejects_with(input: &str, code: ErrorCode) {
    let (verdict, _receipt) = validate_bootstrap_replay_surface(input);
    assert!(!verdict.accepted, "surface should reject");
    assert!(
        verdict.errors.iter().any(|error| error.code == code),
        "expected {:?}, got {:?}",
        code,
        verdict.errors
    );
}

#[test]
fn accepts_valid_bootstrap_replay_surface() {
    let (verdict, receipt) = validate_bootstrap_replay_surface(VALID);
    assert!(
        verdict.accepted,
        "valid bootstrap replay rejected: {:?}",
        verdict.errors
    );
    assert_eq!(receipt.header, "LYRA-P02-RECEIPT v1");
    assert!(receipt.receipt_hash.starts_with("fnv1a128:"));
}

#[test]
fn bootstrap_replay_deterministic_report_is_stable_and_counted() {
    let hash = "fnv1a128:0123456789abcdef0123456789abcdef".to_string();
    let receipts = vec![
        (
            "bootstrap_replay_receipt".to_string(),
            "receipts/p02/pass_0075_bootstrap_replay.receipt".to_string(),
            hash.clone(),
            hash.clone(),
            hash.clone(),
            hash.clone(),
            "artifact_emitted".to_string(),
        ),
        (
            "bootstrap_falsification_receipt".to_string(),
            "receipts/p02/pass_0074_bootstrap_falsification.receipt".to_string(),
            hash.clone(),
            hash.clone(),
            hash.clone(),
            hash.clone(),
            "artifact_emitted".to_string(),
        ),
    ];
    let witnesses = vec![
        (
            "z_witness".to_string(),
            "002".to_string(),
            vec!["bootstrap_replay_receipt".to_string()],
            "z_preimage".to_string(),
            hash.clone(),
            vec!["z_command".to_string()],
            "execution_proven".to_string(),
        ),
        (
            "a_witness".to_string(),
            "001".to_string(),
            vec!["bootstrap_falsification_receipt".to_string()],
            "a_preimage".to_string(),
            hash.clone(),
            vec!["a_command".to_string()],
            "execution_proven".to_string(),
        ),
    ];
    let links = vec![(
        "a_link".to_string(),
        "bootstrap_falsification_receipt".to_string(),
        "bootstrap_replay_receipt".to_string(),
        "precedes".to_string(),
        vec![
            "bootstrap_falsification_receipt".to_string(),
            "bootstrap_replay_receipt".to_string(),
        ],
        "execution_proven".to_string(),
    )];
    let proofs = vec![(
        "a_proof".to_string(),
        "chain".to_string(),
        vec![
            "bootstrap_falsification_receipt".to_string(),
            "bootstrap_replay_receipt".to_string(),
        ],
        vec!["a_witness".to_string()],
        vec!["a_link".to_string()],
        vec!["a_command".to_string()],
        vec!["mutable_replay".to_string()],
        "execution_proven".to_string(),
    )];
    let artifacts = vec![(
        "a_artifact".to_string(),
        "k0".to_string(),
        "k0/determinism/src/bootstrap_replay.rs".to_string(),
        "deterministic_report".to_string(),
        "artifact_emitted".to_string(),
    )];
    let left = deterministic_bootstrap_replay_suite_report(
        &receipts, &witnesses, &links, &proofs, &artifacts,
    );
    let right = deterministic_bootstrap_replay_suite_report(
        &receipts, &witnesses, &links, &proofs, &artifacts,
    );
    assert_eq!(left, right);
    assert_eq!(left.receipt_count, 2);
    assert_eq!(left.witness_count, 2);
    assert_eq!(left.witnesses[0].order, "001");
    assert_eq!(left.witnesses[0].id, "a_witness");
}

#[test]
fn required_bootstrap_replay_inventory_is_broad() {
    assert_eq!(REQUIRED_BOOTSTRAP_REPLAY_RECEIPTS.len(), 17);
    assert_eq!(REQUIRED_BOOTSTRAP_REPLAY_WITNESSES.len(), 7);
    assert_eq!(REQUIRED_BOOTSTRAP_REPLAY_LINKS.len(), 16);
    assert_eq!(REQUIRED_BOOTSTRAP_REPLAY_PROOFS.len(), 5);
    assert_eq!(REQUIRED_BOOTSTRAP_REPLAY_ARTIFACTS.len(), 8);
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
fn rejects_bad_receipt_hash() {
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
