use crate::p02_foreign_surface_closure_model::ForeignSurfaceClosureSurface;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSurfaceClosureReport {
    pub surface_count: usize,
    pub visible_count: usize,
    pub bounded_count: usize,
    pub challenge_bound_count: usize,
    pub closure_paired_count: usize,
    pub unpaired_surfaces: Vec<String>,
}

pub fn deterministic_foreign_surface_closure_report(
    surface: &ForeignSurfaceClosureSurface,
) -> ForeignSurfaceClosureReport {
    let mut unpaired_surfaces = Vec::new();
    let mut visible_count = 0;
    let mut bounded_count = 0;
    let mut challenge_bound_count = 0;
    let mut closure_paired_count = 0;
    for row in &surface.surfaces {
        if row.visible() {
            visible_count += 1;
        }
        if row.bounded() {
            bounded_count += 1;
        }
        if row.challengeable() && surface.challenge_for_surface(&row.id).is_some() {
            challenge_bound_count += 1;
        }
        if row.closure_paired() && surface.closure_law_for_surface(&row.id).is_some() {
            closure_paired_count += 1;
        }
        if !(row.visible()
            && row.bounded()
            && row.challengeable()
            && row.closure_paired()
            && surface.visibility_for_surface(&row.id).is_some()
            && surface.challenge_for_surface(&row.id).is_some()
            && surface.closure_law_for_surface(&row.id).is_some())
        {
            unpaired_surfaces.push(row.id.clone());
        }
    }
    unpaired_surfaces.sort();
    ForeignSurfaceClosureReport {
        surface_count: surface.surfaces.len(),
        visible_count,
        bounded_count,
        challenge_bound_count,
        closure_paired_count,
        unpaired_surfaces,
    }
}
