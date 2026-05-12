use lyra_phase0::p00::{
    P00_ENFORCEMENT_CONTRACT, REQUIRED_ENFORCEMENT_RULES, REQUIRED_IMPLEMENTATION_UNITS,
};

const CONTRACT: &str = include_str!("../interfaces/p00/contracts/enforcement_law.v1.lyra");

#[test]
fn enforcement_contract_binds_runtime_header_rules_and_units() {
    assert!(CONTRACT.contains(P00_ENFORCEMENT_CONTRACT));
    for rule in REQUIRED_ENFORCEMENT_RULES {
        assert!(
            CONTRACT.contains(rule),
            "contract missing enforcement rule {rule}"
        );
    }
    for unit in REQUIRED_IMPLEMENTATION_UNITS {
        assert!(
            CONTRACT.contains(unit.id),
            "contract missing implementation unit {}",
            unit.id
        );
        assert!(
            CONTRACT.contains(unit.path),
            "contract missing implementation path {}",
            unit.path
        );
    }
}
