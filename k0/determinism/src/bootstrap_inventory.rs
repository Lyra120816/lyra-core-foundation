use crate::k0_hash::stable_hash_label;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapSurfaceReport {
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
    pub evidence_count: usize,
    pub status: String,
    pub surface_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapInventoryReport {
    pub surface_count: usize,
    pub temporary_count: usize,
    pub observer_count: usize,
    pub bounded_permanent_count: usize,
    pub forbidden_count: usize,
    pub declared_visibility_count: usize,
    pub inventoried_count: usize,
    pub blocked_count: usize,
    pub receipt_bound_count: usize,
    pub surface_reports: Vec<BootstrapSurfaceReport>,
    pub inventory_hash: String,
}

pub fn deterministic_bootstrap_inventory_report(
    surfaces: &[(
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
        Vec<String>,
        String,
    )],
) -> BootstrapInventoryReport {
    let mut sorted = surfaces.to_vec();
    sorted.sort_by(|left, right| left.0.cmp(&right.0));

    let mut temporary_count = 0usize;
    let mut observer_count = 0usize;
    let mut bounded_permanent_count = 0usize;
    let mut forbidden_count = 0usize;
    let mut declared_visibility_count = 0usize;
    let mut inventoried_count = 0usize;
    let mut blocked_count = 0usize;
    let mut receipt_bound_count = 0usize;
    let mut surface_reports = Vec::new();
    let mut preimage = format!("surfaces:{}", sorted.len());

    for (
        id,
        owner_root,
        surface_type,
        path,
        role,
        classification,
        boundary,
        target,
        visibility,
        retirement_ref,
        mut evidence,
        status,
    ) in sorted
    {
        evidence.sort();
        match classification.as_str() {
            "temporary" => temporary_count += 1,
            "observer" => observer_count += 1,
            "bounded_permanent" => bounded_permanent_count += 1,
            "forbidden" => forbidden_count += 1,
            _ => {}
        }
        if visibility == "declared" {
            declared_visibility_count += 1;
        }
        if status == "inventoried" {
            inventoried_count += 1;
        }
        if status == "forbidden_declared" || status == "blocked_until_explicit_import" {
            blocked_count += 1;
        }
        if !evidence.is_empty() {
            receipt_bound_count += 1;
        }
        let surface_preimage = format!(
            "surface:{}|owner:{}|type:{}|path:{}|role:{}|classification:{}|boundary:{}|target:{}|visibility:{}|retirement:{}|evidence:{}|status:{}",
            id, owner_root, surface_type, path, role, classification, boundary, target, visibility, retirement_ref, evidence.join(","), status
        );
        let surface_hash = stable_hash_label("lyra.p02.bootstrap.surface", &surface_preimage);
        preimage.push('|');
        preimage.push_str(&surface_preimage);
        surface_reports.push(BootstrapSurfaceReport {
            id,
            owner_root,
            surface_type,
            path,
            role,
            classification,
            boundary,
            target,
            visibility,
            retirement_ref,
            evidence_count: evidence.len(),
            status,
            surface_hash,
        });
    }

    BootstrapInventoryReport {
        surface_count: surface_reports.len(),
        temporary_count,
        observer_count,
        bounded_permanent_count,
        forbidden_count,
        declared_visibility_count,
        inventoried_count,
        blocked_count,
        receipt_bound_count,
        surface_reports,
        inventory_hash: stable_hash_label("lyra.p02.bootstrap.inventory", &preimage),
    }
}
