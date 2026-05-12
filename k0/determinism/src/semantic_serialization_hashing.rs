use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSerializationFamilyReport {
    pub id: String,
    pub owner_root: String,
    pub serializer: String,
    pub hash_domain: String,
    pub family_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSerializationObjectHashReport {
    pub id: String,
    pub object_ref: String,
    pub payload_hash: String,
    pub record_hash: String,
    pub object_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSerializationRoundTripReport {
    pub id: String,
    pub object_ref: String,
    pub text_identity: String,
    pub hash_identity: String,
    pub round_trip_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSerializationReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticSerializationHashingSuiteReport {
    pub family_count: usize,
    pub object_hash_count: usize,
    pub round_trip_count: usize,
    pub receipt_count: usize,
    pub family_reports: Vec<SemanticSerializationFamilyReport>,
    pub object_hash_reports: Vec<SemanticSerializationObjectHashReport>,
    pub round_trip_reports: Vec<SemanticSerializationRoundTripReport>,
    pub receipt_reports: Vec<SemanticSerializationReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_serialization_hashing_suite_report(
    families: &[(String, String, String, String, String, String)],
    object_hashes: &[(String, String, String, String, String, String, String)],
    round_trips: &[(String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> SemanticSerializationHashingSuiteReport {
    let mut family_reports: Vec<SemanticSerializationFamilyReport> = families
        .iter()
        .map(|item| {
            let preimage = format!(
                "serializer:{}|owner:{}|serializer:{}|hash_domain:{}|registry:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            SemanticSerializationFamilyReport {
                id: item.0.clone(),
                owner_root: item.1.clone(),
                serializer: item.2.clone(),
                hash_domain: item.3.clone(),
                family_hash: stable_hash_label(
                    "lyra.p01.semantic_serialization_hashing.family",
                    &preimage,
                ),
            }
        })
        .collect();
    family_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut object_hash_reports: Vec<SemanticSerializationObjectHashReport> = object_hashes.iter().map(|item| {
        let preimage = format!("object_hash:{}|family:{}|object_ref:{}|payload_hash:{}|record_hash:{}|comparison_key:{}|status:{}", item.0, item.1, item.2, item.3, item.4, item.5, item.6);
        SemanticSerializationObjectHashReport { id: item.0.clone(), object_ref: item.2.clone(), payload_hash: item.3.clone(), record_hash: item.4.clone(), object_hash: stable_hash_label("lyra.p01.semantic_serialization_hashing.object_hash", &preimage) }
    }).collect();
    object_hash_reports.sort_by(|left, right| left.object_ref.cmp(&right.object_ref));

    let mut round_trip_reports: Vec<SemanticSerializationRoundTripReport> = round_trips
        .iter()
        .map(|item| {
            let preimage = format!(
                "round_trip:{}|object_ref:{}|text_identity:{}|hash_identity:{}|law:{}|status:{}",
                item.0, item.1, item.2, item.3, item.4, item.5
            );
            SemanticSerializationRoundTripReport {
                id: item.0.clone(),
                object_ref: item.1.clone(),
                text_identity: item.2.clone(),
                hash_identity: item.3.clone(),
                round_trip_hash: stable_hash_label(
                    "lyra.p01.semantic_serialization_hashing.round_trip",
                    &preimage,
                ),
            }
        })
        .collect();
    round_trip_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut receipt_reports: Vec<SemanticSerializationReceiptReport> = receipts
        .iter()
        .map(|item| {
            let preimage = format!(
                "receipt:{}|path:{}|target:{}|status:{}",
                item.0, item.1, item.2, item.3
            );
            SemanticSerializationReceiptReport {
                id: item.0.clone(),
                path: item.1.clone(),
                target: item.2.clone(),
                receipt_hash: stable_hash_label(
                    "lyra.p01.semantic_serialization_hashing.receipt",
                    &preimage,
                ),
            }
        })
        .collect();
    receipt_reports.sort_by(|left, right| left.id.cmp(&right.id));

    let mut suite_lines = Vec::new();
    for item in &family_reports {
        suite_lines.push(format!("serializer:{}|{}", item.id, item.family_hash));
    }
    for item in &object_hash_reports {
        suite_lines.push(format!(
            "object_hash:{}|{}|{}",
            item.object_ref, item.payload_hash, item.record_hash
        ));
    }
    for item in &round_trip_reports {
        suite_lines.push(format!("round_trip:{}|{}", item.id, item.round_trip_hash));
    }
    for item in &receipt_reports {
        suite_lines.push(format!("receipt:{}|{}", item.id, item.receipt_hash));
    }
    suite_lines.sort();

    SemanticSerializationHashingSuiteReport {
        family_count: family_reports.len(),
        object_hash_count: object_hash_reports.len(),
        round_trip_count: round_trip_reports.len(),
        receipt_count: receipt_reports.len(),
        family_reports,
        object_hash_reports,
        round_trip_reports,
        receipt_reports,
        suite_hash: stable_hash_label(
            "lyra.p01.semantic_serialization_hashing.suite",
            &suite_lines.join("\n"),
        ),
    }
}
