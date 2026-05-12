use lyra_phase0::p00::{
    P00_BENCHMARK_EVIDENCE_LAW_CONTRACT, REQUIRED_BENCHMARKS, REQUIRED_BENCHMARK_EVIDENCE_RULES,
    REQUIRED_EVIDENCE_FAMILIES, REQUIRED_LOCAL_DEFINITIONS,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/benchmark_evidence_law.v1.lyra");
const FRONTIER_LOCK: &str = include_str!("../ops/p00/control/frontier_lock.v1.lyra");
const TRUTH_SNAPSHOT: &str = include_str!("../ops/p00/control/truth_snapshot.v1.lyra");
const BLOCKER_INDEX: &str = include_str!("../ops/p00/control/blocker_index.v1.lyra");

#[test]
fn contract_binds_runtime_header_rules_benchmarks_evidence_and_definitions() {
    assert!(CONTRACT.contains(P00_BENCHMARK_EVIDENCE_LAW_CONTRACT));
    for rule in REQUIRED_BENCHMARK_EVIDENCE_RULES {
        assert!(
            CONTRACT.contains(rule),
            "contract missing benchmark/evidence rule {rule}"
        );
    }
    for benchmark in REQUIRED_BENCHMARKS {
        assert!(
            CONTRACT.contains(benchmark.id),
            "contract missing benchmark {}",
            benchmark.id
        );
        assert!(
            CONTRACT.contains(benchmark.metric),
            "contract missing metric {}",
            benchmark.metric
        );
    }
    for family in REQUIRED_EVIDENCE_FAMILIES {
        assert!(
            CONTRACT.contains(family),
            "contract missing evidence family {family}"
        );
    }
    for definition in REQUIRED_LOCAL_DEFINITIONS {
        assert!(
            CONTRACT.contains(definition),
            "contract missing definition {definition}"
        );
    }
}

#[test]
fn control_plane_advances_to_p00_009_without_closing_phase() {
    assert!(FRONTIER_LOCK.contains("current_task=P00-X05"));
    assert!(FRONTIER_LOCK.contains("previous_frontier=P00-X04"));
    assert!(FRONTIER_LOCK
        .contains("truth_bound=receipts/p00/pass_0029_retirement_supersession.receipt"));
    assert!(TRUTH_SNAPSHOT.contains("status=closure_outputs_artifact_emitted"));
    assert!(TRUTH_SNAPSHOT.contains("closed=false"));
    assert!(TRUTH_SNAPSHOT.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("current_frontier=P00-X05"));
    assert!(BLOCKER_INDEX.contains("blocker:local_validation_evidence"));
    assert!(BLOCKER_INDEX.contains("next_immediate_frontier=P01"));
}
