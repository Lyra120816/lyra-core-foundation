const CONTRACT: &str = include_str!("../interfaces/p00/contracts/benchmark_pack.v1.lyra");
const CONTROL_FRONTIER: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");
const PACK: &str = include_str!("../ops/p00/closure/benchmark_pack.lyra");

#[test]
fn benchmark_pack_contract_names_p00_x03() {
    assert!(CONTRACT.contains("LYRA-P00-BENCHMARK-PACK v1"));
    assert!(CONTRACT.contains("task=P00-X03"));
    for family in [
        "family:throughput",
        "family:latency",
        "family:correctness",
        "family:stability",
    ] {
        assert!(CONTRACT.contains(family));
    }
}

#[test]
fn control_files_advance_to_benchmark_pack_and_keep_phase_open() {
    assert!(CONTROL_FRONTIER.contains("current_task=P00-X05"));
    assert!(CONTROL_FRONTIER.contains("previous_frontier=P00-X04"));
    assert!(CONTROL_FRONTIER.contains("next_frontier=P01"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(TRUTH_SNAPSHOT.contains("phase_closure=false"));
    assert!(TRUTH_SNAPSHOT.contains("remaining_closure_outputs=none"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(!BLOCKER_INDEX.contains("blocker:P00-X04="));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}

#[test]
fn emitted_pack_lists_required_benchmark_families_without_global_closure() {
    for family in [
        "family:throughput",
        "family:latency",
        "family:correctness",
        "family:stability",
    ] {
        assert!(PACK.contains(family));
    }
    assert!(PACK.contains("target:stability_hash_ordering"));
    assert!(!PACK.contains("global closure true"));
}
