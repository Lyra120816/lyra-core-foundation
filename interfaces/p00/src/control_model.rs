use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlSurfaceBinding {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub schema: String,
    pub required_fields: Vec<String>,
    pub owner_root: String,
    pub status: String,
    pub evidence: Vec<String>,
}

impl ControlSurfaceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("surface:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlFieldBinding {
    pub line_number: usize,
    pub id: String,
    pub kind: String,
    pub required: String,
    pub value: String,
    pub stable: String,
}

impl ControlFieldBinding {
    pub fn canonical_identity(&self) -> String {
        format!("field:{}", self.id)
    }

    pub fn surface_id(&self) -> Option<&str> {
        self.id.split_once('.').map(|(surface, _)| surface)
    }

    pub fn field_name(&self) -> Option<&str> {
        self.id.split_once('.').map(|(_, field)| field)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassTemplateBinding {
    pub line_number: usize,
    pub id: String,
    pub path: String,
    pub requires: Vec<String>,
    pub forbids: Vec<String>,
    pub status: String,
    pub evidence: Vec<String>,
}

impl PassTemplateBinding {
    pub fn canonical_identity(&self) -> String {
        format!("template:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlClaim {
    pub line_number: usize,
    pub id: String,
    pub scope: String,
    pub status: String,
    pub surfaces: Vec<String>,
    pub templates: Vec<String>,
    pub receipts: Vec<String>,
    pub commands: Vec<String>,
}

impl ControlClaim {
    pub fn canonical_identity(&self) -> String {
        format!("claim:{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlSurfaceFormatLaw {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub surfaces: Vec<ControlSurfaceBinding>,
    pub fields: Vec<ControlFieldBinding>,
    pub templates: Vec<PassTemplateBinding>,
    pub claims: Vec<ControlClaim>,
}

impl ControlSurfaceFormatLaw {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }

    pub fn surface_by_id(&self, id: &str) -> Option<&ControlSurfaceBinding> {
        self.surfaces.iter().find(|surface| surface.id == id)
    }

    pub fn field_by_id(&self, id: &str) -> Option<&ControlFieldBinding> {
        self.fields.iter().find(|field| field.id == id)
    }

    pub fn template_by_id(&self, id: &str) -> Option<&PassTemplateBinding> {
        self.templates.iter().find(|template| template.id == id)
    }

    pub fn claim_by_id(&self, id: &str) -> Option<&ControlClaim> {
        self.claims.iter().find(|claim| claim.id == id)
    }
}
