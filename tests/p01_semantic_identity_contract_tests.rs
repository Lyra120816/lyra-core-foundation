const CONTRACT: &str = include_str!("../interfaces/p01/contracts/semantic_identity.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p01/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p01/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p01/control/blocker_index.v1.lyra");
const GUIDE: &str = include_str!("../docs/p01/semantic_identity_guide.lyra");
const EXAMPLE: &str = include_str!("../examples/p01/operator/semantic_identity_review.lyra");

#[test]
fn semantic_identity_contract_names_p01_004() {
    assert!(CONTRACT.contains("LYRA-P01-SEMANTIC-IDENTITY-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P01-004"));
    for item in [
        "symbol",
        "declaration",
        "rewrite",
        "witness_row",
        "artifact",
        "fnv1a128_labeled",
    ] {
        assert!(CONTRACT.contains(item), "contract missing {item}");
    }
}

#[test]
fn control_files_reflect_p01_boundary_at_x05() {
    assert!(FRONTIER_LOCK.contains("current_task=P01-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P01-X04"));
    assert!(FRONTIER_LOCK.contains("next_frontier=P02"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p01/pass_0058_semantic_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P01-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("P01-004"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_primary_tasks=none"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P01-X05"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02"));
}

#[test]
fn developer_operator_surfaces_bind_real_p01_identity_command_and_receipt() {
    assert!(GUIDE.contains("lyra-p01-identity-check fixtures/p01/semantic_identity_inputs/valid_semantic_identity.lyra"));
    assert!(GUIDE.contains("receipts/p01/pass_0033_semantic_identity.receipt"));
    assert!(EXAMPLE.contains("expected=accepted receipt-bound semantic identity and digest suite"));
}
