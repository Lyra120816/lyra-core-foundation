use std::collections::{BTreeMap, BTreeSet};

use crate::k0_hash::stable_hash_label;
use crate::lyralang_symbolic_equality::{
    canonical_symbolic_term, normalize_symbolic_term, SymbolicEqualityError, SymbolicTerm,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticErrorObjectDescriptor {
    pub id: &'static str,
    pub severity: &'static str,
    pub domain: &'static str,
    pub subject: &'static str,
    pub message: &'static str,
    pub evidence_ref: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticChallengeObjectDescriptor {
    pub id: &'static str,
    pub target: &'static str,
    pub challenger: &'static str,
    pub claim_ref: &'static str,
    pub counter_evidence_ref: &'static str,
    pub adjudication_law: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticEvidenceObjectDescriptor {
    pub id: &'static str,
    pub kind: &'static str,
    pub source: &'static str,
    pub payload_digest: &'static str,
    pub witness: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticObjectLinkDescriptor {
    pub id: &'static str,
    pub from: &'static str,
    pub relation: &'static str,
    pub to: &'static str,
    pub law: &'static str,
    pub status: &'static str,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FirstClassDiagnosticObjectKind {
    Error,
    Challenge,
    Evidence,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorChallengeEvidenceError {
    UnknownObject { id: String },
    UnknownEvidence { id: String },
    BrokenObjectRef { id: String, reference: String },
    SymbolicProjection(SymbolicEqualityError),
}
impl From<SymbolicEqualityError> for ErrorChallengeEvidenceError {
    fn from(value: SymbolicEqualityError) -> Self {
        ErrorChallengeEvidenceError::SymbolicProjection(value)
    }
}

pub const LYRALANG_ERROR_OBJECT_DESCRIPTORS: &[SemanticErrorObjectDescriptor] = &[
    SemanticErrorObjectDescriptor {
        id: "parse_missing_token",
        severity: "reject",
        domain: "parser",
        subject: "parser.token",
        message: "parser reached declared eof without required token",
        evidence_ref: "evidence_parser_replay",
        status: "artifact_emitted",
    },
    SemanticErrorObjectDescriptor {
        id: "type_effect_violation",
        severity: "reject",
        domain: "checker",
        subject: "effect.capability",
        message: "declared effect exceeds admitted capability",
        evidence_ref: "evidence_type_trace",
        status: "artifact_emitted",
    },
    SemanticErrorObjectDescriptor {
        id: "capability_denied",
        severity: "reject",
        domain: "runtime",
        subject: "capability.fs.write",
        message: "capability gate denied unproven write authority",
        evidence_ref: "evidence_capability_policy",
        status: "artifact_emitted",
    },
    SemanticErrorObjectDescriptor {
        id: "proof_obligation_unmet",
        severity: "reject",
        domain: "proof",
        subject: "obligation.normalization",
        message: "normalization witness obligation has no proof row",
        evidence_ref: "evidence_proof_bundle",
        status: "artifact_emitted",
    },
    SemanticErrorObjectDescriptor {
        id: "receipt_mismatch",
        severity: "reject",
        domain: "receipt",
        subject: "receipt.chain",
        message: "receipt replay hash differs from canonical preimage",
        evidence_ref: "evidence_receipt_chain",
        status: "artifact_emitted",
    },
];
pub const LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS: &[SemanticChallengeObjectDescriptor] = &[
    SemanticChallengeObjectDescriptor {
        id: "challenge_parse_error",
        target: "parse_missing_token",
        challenger: "operator",
        claim_ref: "parser_reject",
        counter_evidence_ref: "evidence_parser_replay",
        adjudication_law: "replay_fixture_required",
        status: "artifact_emitted",
    },
    SemanticChallengeObjectDescriptor {
        id: "challenge_type_effect",
        target: "type_effect_violation",
        challenger: "checker",
        claim_ref: "effect_claim",
        counter_evidence_ref: "evidence_type_trace",
        adjudication_law: "typed_trace_required",
        status: "artifact_emitted",
    },
    SemanticChallengeObjectDescriptor {
        id: "challenge_capability",
        target: "capability_denied",
        challenger: "runtime",
        claim_ref: "capability_claim",
        counter_evidence_ref: "evidence_capability_policy",
        adjudication_law: "policy_receipt_required",
        status: "artifact_emitted",
    },
    SemanticChallengeObjectDescriptor {
        id: "challenge_proof",
        target: "proof_obligation_unmet",
        challenger: "proof",
        claim_ref: "proof_claim",
        counter_evidence_ref: "evidence_proof_bundle",
        adjudication_law: "proof_bundle_required",
        status: "artifact_emitted",
    },
    SemanticChallengeObjectDescriptor {
        id: "challenge_receipt",
        target: "receipt_mismatch",
        challenger: "receipt",
        claim_ref: "receipt_claim",
        counter_evidence_ref: "evidence_receipt_chain",
        adjudication_law: "receipt_replay_required",
        status: "artifact_emitted",
    },
];
pub const LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS: &[SemanticEvidenceObjectDescriptor] = &[
    SemanticEvidenceObjectDescriptor {
        id: "evidence_parser_replay",
        kind: "trace",
        source: "fixtures/p01/error_challenge_evidence_inputs/parser_trace.lyra",
        payload_digest: "fnv1a128:c231564589ea67eb86d866dda06ec3a9",
        witness: "parser_replay_witness",
        status: "artifact_emitted",
    },
    SemanticEvidenceObjectDescriptor {
        id: "evidence_type_trace",
        kind: "trace",
        source: "fixtures/p01/error_challenge_evidence_inputs/type_trace.lyra",
        payload_digest: "fnv1a128:69ae7f8bc16cf05361c511a55d4d5656",
        witness: "checker_effect_witness",
        status: "artifact_emitted",
    },
    SemanticEvidenceObjectDescriptor {
        id: "evidence_capability_policy",
        kind: "policy",
        source: "fixtures/p01/error_challenge_evidence_inputs/capability_policy.lyra",
        payload_digest: "fnv1a128:20a86b4ca912fc90a7e0df351b13af85",
        witness: "capability_policy_witness",
        status: "artifact_emitted",
    },
    SemanticEvidenceObjectDescriptor {
        id: "evidence_proof_bundle",
        kind: "proof_bundle",
        source: "fixtures/p01/error_challenge_evidence_inputs/proof_bundle.lyra",
        payload_digest: "fnv1a128:8a46544b8a7b8cdb70cf34d85a05cdfb",
        witness: "proof_obligation_witness",
        status: "artifact_emitted",
    },
    SemanticEvidenceObjectDescriptor {
        id: "evidence_receipt_chain",
        kind: "receipt_chain",
        source: "fixtures/p01/error_challenge_evidence_inputs/receipt_chain.lyra",
        payload_digest: "fnv1a128:9691602f160155be8466b376a744bbea",
        witness: "receipt_replay_witness",
        status: "artifact_emitted",
    },
];
pub const LYRALANG_OBJECT_LINK_DESCRIPTORS: &[SemanticObjectLinkDescriptor] = &[
    SemanticObjectLinkDescriptor {
        id: "error_parse_supported",
        from: "parse_missing_token",
        relation: "supported_by",
        to: "evidence_parser_replay",
        law: "error_evidence_ref_matches",
        status: "artifact_emitted",
    },
    SemanticObjectLinkDescriptor {
        id: "error_type_supported",
        from: "type_effect_violation",
        relation: "supported_by",
        to: "evidence_type_trace",
        law: "error_evidence_ref_matches",
        status: "artifact_emitted",
    },
    SemanticObjectLinkDescriptor {
        id: "challenge_parse_targets",
        from: "challenge_parse_error",
        relation: "challenges",
        to: "parse_missing_token",
        law: "challenge_target_bound",
        status: "artifact_emitted",
    },
    SemanticObjectLinkDescriptor {
        id: "challenge_parse_countered",
        from: "challenge_parse_error",
        relation: "countered_by",
        to: "evidence_parser_replay",
        law: "challenge_counter_evidence_bound",
        status: "artifact_emitted",
    },
    SemanticObjectLinkDescriptor {
        id: "receipt_error_supported",
        from: "receipt_mismatch",
        relation: "supported_by",
        to: "evidence_receipt_chain",
        law: "receipt_replay_bound",
        status: "artifact_emitted",
    },
];

pub fn error_object_ids() -> Vec<&'static str> {
    LYRALANG_ERROR_OBJECT_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn challenge_object_ids() -> Vec<&'static str> {
    LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn evidence_object_ids() -> Vec<&'static str> {
    LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn object_link_ids() -> Vec<&'static str> {
    LYRALANG_OBJECT_LINK_DESCRIPTORS
        .iter()
        .map(|item| item.id)
        .collect()
}
pub fn error_object_descriptor(id: &str) -> Option<&'static SemanticErrorObjectDescriptor> {
    LYRALANG_ERROR_OBJECT_DESCRIPTORS
        .iter()
        .find(|item| item.id == id)
}
pub fn challenge_object_descriptor(id: &str) -> Option<&'static SemanticChallengeObjectDescriptor> {
    LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS
        .iter()
        .find(|item| item.id == id)
}
pub fn evidence_object_descriptor(id: &str) -> Option<&'static SemanticEvidenceObjectDescriptor> {
    LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS
        .iter()
        .find(|item| item.id == id)
}
pub fn object_link_descriptor(id: &str) -> Option<&'static SemanticObjectLinkDescriptor> {
    LYRALANG_OBJECT_LINK_DESCRIPTORS
        .iter()
        .find(|item| item.id == id)
}
pub fn evidence_payload_digest(payload_text: &str) -> String {
    stable_hash_label("lyra.p01.error_challenge_evidence.payload", payload_text)
}
pub fn error_object_digest(item: &SemanticErrorObjectDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.error_challenge_evidence.error_object",
        &format!(
            "error_object:{}|severity:{}|domain:{}|subject:{}|message:{}|evidence_ref:{}|status:{}",
            item.id,
            item.severity,
            item.domain,
            item.subject,
            item.message,
            item.evidence_ref,
            item.status
        ),
    )
}
pub fn challenge_object_digest(item: &SemanticChallengeObjectDescriptor) -> String {
    stable_hash_label("lyra.p01.error_challenge_evidence.challenge_object", &format!("challenge_object:{}|target:{}|challenger:{}|claim_ref:{}|counter_evidence_ref:{}|adjudication_law:{}|status:{}", item.id, item.target, item.challenger, item.claim_ref, item.counter_evidence_ref, item.adjudication_law, item.status))
}
pub fn evidence_object_digest(item: &SemanticEvidenceObjectDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.error_challenge_evidence.evidence_object",
        &format!(
            "evidence_object:{}|kind:{}|source:{}|payload_digest:{}|witness:{}|status:{}",
            item.id, item.kind, item.source, item.payload_digest, item.witness, item.status
        ),
    )
}
pub fn object_link_digest(item: &SemanticObjectLinkDescriptor) -> String {
    stable_hash_label(
        "lyra.p01.error_challenge_evidence.object_link",
        &format!(
            "object_link:{}|from:{}|relation:{}|to:{}|law:{}|status:{}",
            item.id, item.from, item.relation, item.to, item.law, item.status
        ),
    )
}
fn text(value: &str) -> SymbolicTerm {
    SymbolicTerm::Text(value.to_string())
}
pub fn error_object_symbolic_term(item: &SemanticErrorObjectDescriptor) -> SymbolicTerm {
    SymbolicTerm::Record(vec![
        (
            "evidence_ref".to_string(),
            SymbolicTerm::Symbol(item.evidence_ref.to_string()),
        ),
        ("id".to_string(), text(item.id)),
        ("kind".to_string(), text("error")),
        ("message".to_string(), text(item.message)),
        ("severity".to_string(), text(item.severity)),
        ("subject".to_string(), text(item.subject)),
    ])
}
pub fn challenge_object_symbolic_term(item: &SemanticChallengeObjectDescriptor) -> SymbolicTerm {
    SymbolicTerm::Record(vec![
        ("adjudication_law".to_string(), text(item.adjudication_law)),
        ("claim_ref".to_string(), text(item.claim_ref)),
        (
            "counter_evidence_ref".to_string(),
            SymbolicTerm::Symbol(item.counter_evidence_ref.to_string()),
        ),
        ("id".to_string(), text(item.id)),
        ("kind".to_string(), text("challenge")),
        (
            "target".to_string(),
            SymbolicTerm::Symbol(item.target.to_string()),
        ),
    ])
}
pub fn evidence_object_symbolic_term(item: &SemanticEvidenceObjectDescriptor) -> SymbolicTerm {
    SymbolicTerm::Record(vec![
        ("id".to_string(), text(item.id)),
        ("kind".to_string(), text("evidence")),
        ("payload_digest".to_string(), text(item.payload_digest)),
        ("source".to_string(), text(item.source)),
        ("witness".to_string(), text(item.witness)),
    ])
}
pub fn canonical_error_object_text(
    item: &SemanticErrorObjectDescriptor,
) -> Result<String, ErrorChallengeEvidenceError> {
    Ok(canonical_symbolic_term(&error_object_symbolic_term(item))?)
}
pub fn canonical_challenge_object_text(
    item: &SemanticChallengeObjectDescriptor,
) -> Result<String, ErrorChallengeEvidenceError> {
    Ok(canonical_symbolic_term(&challenge_object_symbolic_term(
        item,
    ))?)
}
pub fn canonical_evidence_object_text(
    item: &SemanticEvidenceObjectDescriptor,
) -> Result<String, ErrorChallengeEvidenceError> {
    Ok(canonical_symbolic_term(&evidence_object_symbolic_term(
        item,
    ))?)
}
pub fn canonical_first_class_object_text(
    kind: FirstClassDiagnosticObjectKind,
    id: &str,
) -> Result<String, ErrorChallengeEvidenceError> {
    match kind {
        FirstClassDiagnosticObjectKind::Error => error_object_descriptor(id)
            .ok_or_else(|| ErrorChallengeEvidenceError::UnknownObject { id: id.to_string() })
            .and_then(canonical_error_object_text),
        FirstClassDiagnosticObjectKind::Challenge => challenge_object_descriptor(id)
            .ok_or_else(|| ErrorChallengeEvidenceError::UnknownObject { id: id.to_string() })
            .and_then(canonical_challenge_object_text),
        FirstClassDiagnosticObjectKind::Evidence => evidence_object_descriptor(id)
            .ok_or_else(|| ErrorChallengeEvidenceError::UnknownEvidence { id: id.to_string() })
            .and_then(canonical_evidence_object_text),
    }
}
pub fn normalized_first_class_object_text(
    kind: FirstClassDiagnosticObjectKind,
    id: &str,
) -> Result<String, ErrorChallengeEvidenceError> {
    let term = match kind {
        FirstClassDiagnosticObjectKind::Error => error_object_descriptor(id)
            .map(error_object_symbolic_term)
            .ok_or_else(|| ErrorChallengeEvidenceError::UnknownObject { id: id.to_string() })?,
        FirstClassDiagnosticObjectKind::Challenge => challenge_object_descriptor(id)
            .map(challenge_object_symbolic_term)
            .ok_or_else(|| ErrorChallengeEvidenceError::UnknownObject { id: id.to_string() })?,
        FirstClassDiagnosticObjectKind::Evidence => evidence_object_descriptor(id)
            .map(evidence_object_symbolic_term)
            .ok_or_else(|| ErrorChallengeEvidenceError::UnknownEvidence { id: id.to_string() })?,
    };
    Ok(normalize_symbolic_term(&term)?)
}
pub fn first_class_object_digest(
    kind: FirstClassDiagnosticObjectKind,
    id: &str,
) -> Result<String, ErrorChallengeEvidenceError> {
    let text = normalized_first_class_object_text(kind, id)?;
    Ok(stable_hash_label(
        "lyra.p01.error_challenge_evidence.first_class_object",
        &text,
    ))
}
pub fn diagnostic_object_ref_exists(id: &str) -> bool {
    error_object_descriptor(id).is_some()
        || challenge_object_descriptor(id).is_some()
        || evidence_object_descriptor(id).is_some()
}
pub fn validate_error_challenge_evidence_references() -> Result<(), ErrorChallengeEvidenceError> {
    for item in LYRALANG_ERROR_OBJECT_DESCRIPTORS {
        if evidence_object_descriptor(item.evidence_ref).is_none() {
            return Err(ErrorChallengeEvidenceError::UnknownEvidence {
                id: item.evidence_ref.to_string(),
            });
        }
    }
    for item in LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS {
        if !diagnostic_object_ref_exists(item.target) {
            return Err(ErrorChallengeEvidenceError::BrokenObjectRef {
                id: item.id.to_string(),
                reference: item.target.to_string(),
            });
        }
        if evidence_object_descriptor(item.counter_evidence_ref).is_none() {
            return Err(ErrorChallengeEvidenceError::UnknownEvidence {
                id: item.counter_evidence_ref.to_string(),
            });
        }
    }
    for item in LYRALANG_OBJECT_LINK_DESCRIPTORS {
        if !diagnostic_object_ref_exists(item.from) {
            return Err(ErrorChallengeEvidenceError::BrokenObjectRef {
                id: item.id.to_string(),
                reference: item.from.to_string(),
            });
        }
        if !diagnostic_object_ref_exists(item.to) {
            return Err(ErrorChallengeEvidenceError::BrokenObjectRef {
                id: item.id.to_string(),
                reference: item.to.to_string(),
            });
        }
    }
    Ok(())
}
pub fn canonical_error_object_signature(item: &SemanticErrorObjectDescriptor) -> String {
    format!("error_object:{}|severity:{}|domain:{}|subject:{}|message:{}|evidence_ref:{}|digest:{}|status:{}", item.id, item.severity, item.domain, item.subject, item.message, item.evidence_ref, error_object_digest(item), item.status)
}
pub fn canonical_challenge_object_signature(item: &SemanticChallengeObjectDescriptor) -> String {
    format!("challenge_object:{}|target:{}|challenger:{}|claim_ref:{}|counter_evidence_ref:{}|adjudication_law:{}|digest:{}|status:{}", item.id, item.target, item.challenger, item.claim_ref, item.counter_evidence_ref, item.adjudication_law, challenge_object_digest(item), item.status)
}
pub fn canonical_evidence_object_signature(item: &SemanticEvidenceObjectDescriptor) -> String {
    format!(
        "evidence_object:{}|kind:{}|source:{}|payload_digest:{}|witness:{}|digest:{}|status:{}",
        item.id,
        item.kind,
        item.source,
        item.payload_digest,
        item.witness,
        evidence_object_digest(item),
        item.status
    )
}
pub fn canonical_object_link_signature(item: &SemanticObjectLinkDescriptor) -> String {
    format!(
        "object_link:{}|from:{}|relation:{}|to:{}|law:{}|digest:{}|status:{}",
        item.id,
        item.from,
        item.relation,
        item.to,
        item.law,
        object_link_digest(item),
        item.status
    )
}
pub fn canonical_error_challenge_evidence_registry_signature() -> String {
    let mut rows = Vec::new();
    for item in LYRALANG_ERROR_OBJECT_DESCRIPTORS {
        rows.push(canonical_error_object_signature(item));
    }
    for item in LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS {
        rows.push(canonical_challenge_object_signature(item));
    }
    for item in LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS {
        rows.push(canonical_evidence_object_signature(item));
    }
    for item in LYRALANG_OBJECT_LINK_DESCRIPTORS {
        rows.push(canonical_object_link_signature(item));
    }
    rows.sort();
    rows.join("\n")
}
pub fn canonical_error_challenge_evidence_registry_hash() -> String {
    stable_hash_label(
        "lyra.p01.error_challenge_evidence.registry",
        &canonical_error_challenge_evidence_registry_signature(),
    )
}
pub fn first_class_object_map() -> BTreeMap<&'static str, &'static str> {
    let mut map = BTreeMap::new();
    for item in LYRALANG_ERROR_OBJECT_DESCRIPTORS {
        map.insert(item.id, "error");
    }
    for item in LYRALANG_CHALLENGE_OBJECT_DESCRIPTORS {
        map.insert(item.id, "challenge");
    }
    for item in LYRALANG_EVIDENCE_OBJECT_DESCRIPTORS {
        map.insert(item.id, "evidence");
    }
    map
}
pub fn linked_object_pairs() -> BTreeSet<(&'static str, &'static str)> {
    let mut pairs = BTreeSet::new();
    for item in LYRALANG_OBJECT_LINK_DESCRIPTORS {
        pairs.insert((item.from, item.to));
    }
    pairs
}
