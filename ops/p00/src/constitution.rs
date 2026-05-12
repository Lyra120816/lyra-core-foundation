pub struct GovernanceRequirement {
    pub namespace: &'static str,
    pub name: &'static str,
    pub value_contains: &'static str,
}

pub const P00_CONSTITUTION_CONTRACT: &str = "LYRA-P00-CONSTITUTION v1";

pub const P00_GOVERNANCE_REQUIREMENTS: &[GovernanceRequirement] = &[
    GovernanceRequirement {
        namespace: "principle",
        name: "determinism",
        value_contains: "reproducible",
    },
    GovernanceRequirement {
        namespace: "principle",
        name: "sovereignty",
        value_contains: "offline",
    },
    GovernanceRequirement {
        namespace: "principle",
        name: "proof_gate",
        value_contains: "evidence",
    },
    GovernanceRequirement {
        namespace: "principle",
        name: "zero_unsafe",
        value_contains: "unsafe rejected",
    },
    GovernanceRequirement {
        namespace: "principle",
        name: "people_first",
        value_contains: "public benefit",
    },
    GovernanceRequirement {
        namespace: "duty",
        name: "public_benefit",
        value_contains: "human operators",
    },
    GovernanceRequirement {
        namespace: "duty",
        name: "anti_extractive",
        value_contains: "no capture",
    },
    GovernanceRequirement {
        namespace: "duty",
        name: "transparent_challenge",
        value_contains: "challenge",
    },
    GovernanceRequirement {
        namespace: "ban",
        name: "ambient_network",
        value_contains: "forbidden",
    },
    GovernanceRequirement {
        namespace: "ban",
        name: "probabilistic_truth",
        value_contains: "forbidden",
    },
    GovernanceRequirement {
        namespace: "ban",
        name: "hidden_randomness",
        value_contains: "forbidden",
    },
    GovernanceRequirement {
        namespace: "ban",
        name: "fake_closure",
        value_contains: "forbidden",
    },
    GovernanceRequirement {
        namespace: "ban",
        name: "placeholder",
        value_contains: "forbidden",
    },
    GovernanceRequirement {
        namespace: "evidence",
        name: "p00-execution-receipts",
        value_contains: "required",
    },
    GovernanceRequirement {
        namespace: "evidence",
        name: "p00-negative-corpus",
        value_contains: "required",
    },
    GovernanceRequirement {
        namespace: "evidence",
        name: "p00-truth-gate",
        value_contains: "required",
    },
    GovernanceRequirement {
        namespace: "owner_root",
        name: "ops",
        value_contains: "governance",
    },
    GovernanceRequirement {
        namespace: "owner_root",
        name: "interfaces",
        value_contains: "contract",
    },
    GovernanceRequirement {
        namespace: "owner_root",
        name: "k0",
        value_contains: "determinism",
    },
];
