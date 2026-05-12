use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectReport {
    pub id: String,
    pub kind: String,
    pub atom: String,
    pub owner_root: String,
    pub parent: String,
    pub object_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectRelationReport {
    pub id: String,
    pub from_object: String,
    pub to_object: String,
    pub relation_kind: String,
    pub relation_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectInvariantReport {
    pub id: String,
    pub scope: String,
    pub law: String,
    pub invariant_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectConformanceReport {
    pub id: String,
    pub object: String,
    pub ir_form: String,
    pub fixture: String,
    pub conformance_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectReceiptReport {
    pub id: String,
    pub path: String,
    pub target: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticObjectSuiteReport {
    pub object_count: usize,
    pub relation_count: usize,
    pub invariant_count: usize,
    pub conformance_count: usize,
    pub receipt_count: usize,
    pub root_object_count: usize,
    pub admitted_object_count: usize,
    pub object_reports: Vec<SemanticObjectReport>,
    pub relation_reports: Vec<SemanticObjectRelationReport>,
    pub invariant_reports: Vec<SemanticObjectInvariantReport>,
    pub conformance_reports: Vec<SemanticObjectConformanceReport>,
    pub receipt_reports: Vec<SemanticObjectReceiptReport>,
    pub suite_hash: String,
}

pub fn deterministic_semantic_object_suite_report(
    objects: &[(
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )],
    relations: &[(String, String, String, String, String, String)],
    invariants: &[(String, String, String, String, String)],
    conformances: &[(String, String, String, String, String, String)],
    receipts: &[(String, String, String, String)],
) -> SemanticObjectSuiteReport {
    let mut sorted_objects = objects.to_vec();
    sorted_objects.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_relations = relations.to_vec();
    sorted_relations.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_invariants = invariants.to_vec();
    sorted_invariants.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_conformances = conformances.to_vec();
    sorted_conformances.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted_receipts = receipts.to_vec();
    sorted_receipts.sort_by(|left, right| left.0.cmp(&right.0));

    let mut root_object_count = 0usize;
    let mut admitted_object_count = 0usize;
    let mut object_reports = Vec::new();
    let mut relation_reports = Vec::new();
    let mut invariant_reports = Vec::new();
    let mut conformance_reports = Vec::new();
    let mut receipt_reports = Vec::new();
    let mut preimage = format!(
        "objects:{}|relations:{}|invariants:{}|conformances:{}|receipts:{}",
        sorted_objects.len(),
        sorted_relations.len(),
        sorted_invariants.len(),
        sorted_conformances.len(),
        sorted_receipts.len()
    );

    for (id, kind, atom, owner_root, parent, ir_form, serialization, comparison, status, line) in
        sorted_objects
    {
        if parent == "none" {
            root_object_count += 1;
        }
        if status == "admitted" {
            admitted_object_count += 1;
        }
        let object_preimage = format!("object:{id}|kind:{kind}|atom:{atom}|owner:{owner_root}|parent:{parent}|ir:{ir_form}|serialization:{serialization}|comparison:{comparison}|status:{status}|line:{line}");
        let object_hash = stable_hash_label("lyra.p01.semantic_object.object", &object_preimage);
        preimage.push('|');
        preimage.push_str(&object_preimage);
        object_reports.push(SemanticObjectReport {
            id,
            kind,
            atom,
            owner_root,
            parent,
            object_hash,
        });
    }

    for (id, from_object, to_object, relation_kind, law, status) in sorted_relations {
        let relation_preimage = format!("relation:{id}|from:{from_object}|to:{to_object}|kind:{relation_kind}|law:{law}|status:{status}");
        let relation_hash =
            stable_hash_label("lyra.p01.semantic_object.relation", &relation_preimage);
        preimage.push('|');
        preimage.push_str(&relation_preimage);
        relation_reports.push(SemanticObjectRelationReport {
            id,
            from_object,
            to_object,
            relation_kind,
            relation_hash,
        });
    }

    for (id, scope, law, requires, status) in sorted_invariants {
        let invariant_preimage =
            format!("invariant:{id}|scope:{scope}|law:{law}|requires:{requires}|status:{status}");
        let invariant_hash =
            stable_hash_label("lyra.p01.semantic_object.invariant", &invariant_preimage);
        preimage.push('|');
        preimage.push_str(&invariant_preimage);
        invariant_reports.push(SemanticObjectInvariantReport {
            id,
            scope,
            law,
            invariant_hash,
        });
    }

    for (id, object, ir_form, fixture, round_trip, status) in sorted_conformances {
        let conformance_preimage = format!("conformance:{id}|object:{object}|ir:{ir_form}|fixture:{fixture}|round_trip:{round_trip}|status:{status}");
        let conformance_hash = stable_hash_label(
            "lyra.p01.semantic_object.conformance",
            &conformance_preimage,
        );
        preimage.push('|');
        preimage.push_str(&conformance_preimage);
        conformance_reports.push(SemanticObjectConformanceReport {
            id,
            object,
            ir_form,
            fixture,
            conformance_hash,
        });
    }

    for (id, path, target, status) in sorted_receipts {
        let receipt_preimage = format!("receipt:{id}|path:{path}|target:{target}|status:{status}");
        let receipt_hash = stable_hash_label("lyra.p01.semantic_object.receipt", &receipt_preimage);
        preimage.push('|');
        preimage.push_str(&receipt_preimage);
        receipt_reports.push(SemanticObjectReceiptReport {
            id,
            path,
            target,
            receipt_hash,
        });
    }

    let suite_hash = stable_hash_label("lyra.p01.semantic_object.suite", &preimage);
    SemanticObjectSuiteReport {
        object_count: object_reports.len(),
        relation_count: relation_reports.len(),
        invariant_count: invariant_reports.len(),
        conformance_count: conformance_reports.len(),
        receipt_count: receipt_reports.len(),
        root_object_count,
        admitted_object_count,
        object_reports,
        relation_reports,
        invariant_reports,
        conformance_reports,
        receipt_reports,
        suite_hash,
    }
}
