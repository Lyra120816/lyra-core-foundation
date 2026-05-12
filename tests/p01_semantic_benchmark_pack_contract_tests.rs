const CONTRACT: &str = include_str!("../interfaces/p01/contracts/semantic_benchmark_pack.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p01/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p01/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p01/control/blocker_index.v1.lyra");
const BENCHMARK_PACK: &str = include_str!("../ops/p01/closure/semantic_benchmark_pack.lyra");

#[test]
fn semantic_benchmark_pack_contract_names_p01_x03() {
    assert!(CONTRACT.contains("LYRA-P01-SEMANTIC-BENCHMARK-PACK-CONTRACT v1"));
    assert!(CONTRACT.contains("task=P01-X03"));
    assert!(CONTRACT.contains("surface=LYRA-P01-SEMANTIC-BENCHMARK-PACK v1"));
    assert!(CONTRACT.contains("receipt=receipts/p01/pass_0056_semantic_benchmark_pack.receipt"));
    assert!(CONTRACT.contains("next_frontier=P01-X04"));
}

#[test]
fn control_files_reflect_p01_boundary_at_x05() {
    assert!(CONTROL_FRONTIER.contains("current_task=P01-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P01-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P02"));
    assert!(CONTROL_FRONTIER
        .contains("truth_bound=receipts/p01/pass_0058_semantic_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P01-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_closure_outputs=none"));
    assert!(TRUTH_SNAPSHOT.contains("P01-X03"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P01-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P02"));
}

#[test]
fn emitted_benchmark_pack_lists_all_required_semantic_benchmark_families_without_global_closure() {
    for id in [
        "family:throughput",
        "family:latency",
        "family:correctness",
        "family:stability",
    ] {
        assert!(BENCHMARK_PACK.contains(id));
    }
    for id in [
        "target:throughput_semantic_surface_validation",
        "target:latency_semantic_validation_budget",
        "target:correctness_semantic_negative_corpus_rejection",
        "target:stability_semantic_hash_ordering",
    ] {
        assert!(BENCHMARK_PACK.contains(id));
    }
    assert!(!BENCHMARK_PACK.contains("global_closure:true"));
}
