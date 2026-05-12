use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSurfaceBinding {
    pub line_number: usize,
    pub id: String,
    pub surface_class: String,
    pub owner_root: String,
    pub visibility_status: String,
    pub boundary_status: String,
    pub challenge_suite: String,
    pub closure_law: String,
    pub retirement_gate: String,
    pub truth_effect: String,
    pub status: String,
}

impl ForeignSurfaceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("surface:{}", self.id)
    }
    pub fn visible(&self) -> bool {
        self.visibility_status == "visible_in_inventory"
    }
    pub fn bounded(&self) -> bool {
        self.boundary_status == "bounded_explicit"
    }
    pub fn challengeable(&self) -> bool {
        !self.challenge_suite.is_empty() && self.challenge_suite != "none"
    }
    pub fn closure_paired(&self) -> bool {
        !self.closure_law.is_empty()
            && self.closure_law != "none"
            && !self.retirement_gate.is_empty()
            && self.retirement_gate != "none"
    }
    pub fn truth_neutral(&self) -> bool {
        self.truth_effect == "no_truth_without_local_challenge"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSurfaceChallengeBinding {
    pub line_number: usize,
    pub id: String,
    pub suite_id: String,
    pub surface_id: String,
    pub required_fixture: String,
    pub negative_case: String,
    pub receipt_path: String,
    pub status: String,
}

impl ForeignSurfaceChallengeBinding {
    pub fn canonical_identity(&self) -> String {
        format!("challenge:{}", self.id)
    }
    pub fn binds_surface(&self, surface_id: &str) -> bool {
        self.surface_id == surface_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSurfaceClosureLawBinding {
    pub line_number: usize,
    pub id: String,
    pub surface_id: String,
    pub closure_gate: String,
    pub deletion_gate: String,
    pub retirement_receipt: String,
    pub allowed_closure_scope: String,
    pub status: String,
}

impl ForeignSurfaceClosureLawBinding {
    pub fn canonical_identity(&self) -> String {
        format!("closure_law:{}", self.id)
    }
    pub fn binds_surface(&self, surface_id: &str) -> bool {
        self.surface_id == surface_id
    }
    pub fn bounded_scope(&self) -> bool {
        self.allowed_closure_scope == "per_surface_only"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSurfaceVisibilityBinding {
    pub line_number: usize,
    pub id: String,
    pub surface_id: String,
    pub inventory_path: String,
    pub classification_path: String,
    pub evidence_path: String,
    pub status: String,
}

impl ForeignSurfaceVisibilityBinding {
    pub fn canonical_identity(&self) -> String {
        format!("visibility:{}", self.id)
    }
    pub fn binds_surface(&self, surface_id: &str) -> bool {
        self.surface_id == surface_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSurfaceClosureReceiptBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub surface_id: String,
    pub status: String,
}

impl ForeignSurfaceClosureReceiptBinding {
    pub fn canonical_identity(&self) -> String {
        format!("receipt:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSurfaceClosureSurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub previous_evidence_receipt: String,
    pub rules: BTreeMap<String, String>,
    pub surfaces: Vec<ForeignSurfaceBinding>,
    pub challenges: Vec<ForeignSurfaceChallengeBinding>,
    pub closure_laws: Vec<ForeignSurfaceClosureLawBinding>,
    pub visibility_proofs: Vec<ForeignSurfaceVisibilityBinding>,
    pub receipts: Vec<ForeignSurfaceClosureReceiptBinding>,
}

impl ForeignSurfaceClosureSurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn surface_by_id(&self, id: &str) -> Option<&ForeignSurfaceBinding> {
        self.surfaces.iter().find(|x| x.id == id)
    }
    pub fn challenge_by_id(&self, id: &str) -> Option<&ForeignSurfaceChallengeBinding> {
        self.challenges.iter().find(|x| x.id == id)
    }
    pub fn challenge_for_surface(
        &self,
        surface_id: &str,
    ) -> Option<&ForeignSurfaceChallengeBinding> {
        self.challenges.iter().find(|x| x.surface_id == surface_id)
    }
    pub fn closure_law_by_id(&self, id: &str) -> Option<&ForeignSurfaceClosureLawBinding> {
        self.closure_laws.iter().find(|x| x.id == id)
    }
    pub fn closure_law_for_surface(
        &self,
        surface_id: &str,
    ) -> Option<&ForeignSurfaceClosureLawBinding> {
        self.closure_laws
            .iter()
            .find(|x| x.surface_id == surface_id)
    }
    pub fn visibility_for_surface(
        &self,
        surface_id: &str,
    ) -> Option<&ForeignSurfaceVisibilityBinding> {
        self.visibility_proofs
            .iter()
            .find(|x| x.surface_id == surface_id)
    }
    pub fn receipt_by_id(&self, id: &str) -> Option<&ForeignSurfaceClosureReceiptBinding> {
        self.receipts.iter().find(|x| x.id == id)
    }
}
