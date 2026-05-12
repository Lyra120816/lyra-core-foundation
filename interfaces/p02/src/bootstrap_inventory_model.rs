use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapSurfaceBinding {
    pub line_number: usize,
    pub id: String,
    pub owner_root: String,
    pub surface_type: String,
    pub path: String,
    pub role: String,
    pub classification: String,
    pub boundary: String,
    pub target: String,
    pub visibility: String,
    pub retirement_ref: String,
    pub evidence: Vec<String>,
    pub status: String,
}

impl BootstrapSurfaceBinding {
    pub fn canonical_identity(&self) -> String {
        format!("surface:{}", self.id)
    }
    pub fn is_forbidden(&self) -> bool {
        self.classification == "forbidden"
    }
    pub fn is_temporary(&self) -> bool {
        self.classification == "temporary"
    }
    pub fn is_observer(&self) -> bool {
        self.classification == "observer"
    }
    pub fn is_bounded_permanent(&self) -> bool {
        self.classification == "bounded_permanent"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapInventorySurface {
    pub header: String,
    pub phase: String,
    pub task: String,
    pub status: String,
    pub rules: BTreeMap<String, String>,
    pub surfaces: Vec<BootstrapSurfaceBinding>,
}

impl BootstrapInventorySurface {
    pub fn rule_value(&self, name: &str) -> Option<&str> {
        self.rules.get(name).map(String::as_str)
    }
    pub fn surface_by_id(&self, id: &str) -> Option<&BootstrapSurfaceBinding> {
        self.surfaces.iter().find(|item| item.id == id)
    }
    pub fn temporary_surfaces(&self) -> impl Iterator<Item = &BootstrapSurfaceBinding> {
        self.surfaces.iter().filter(|item| item.is_temporary())
    }
    pub fn observer_surfaces(&self) -> impl Iterator<Item = &BootstrapSurfaceBinding> {
        self.surfaces.iter().filter(|item| item.is_observer())
    }
    pub fn bounded_permanent_surfaces(&self) -> impl Iterator<Item = &BootstrapSurfaceBinding> {
        self.surfaces
            .iter()
            .filter(|item| item.is_bounded_permanent())
    }
    pub fn forbidden_surfaces(&self) -> impl Iterator<Item = &BootstrapSurfaceBinding> {
        self.surfaces.iter().filter(|item| item.is_forbidden())
    }
}
