use lyra_phase0::p02::validate_seed_runtime_replacement_milestone_surface;
fn main() {
    let mut args = std::env::args();
    let _program = args.next();
    let path=args.next().unwrap_or_else(||"fixtures/p02/seed_runtime_replacement_milestones_inputs/valid_seed_runtime_replacement_milestones.lyra".to_string());
    let input = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("read error {path}: {e}");
        std::process::exit(2)
    });
    let (verdict, receipt) = validate_seed_runtime_replacement_milestone_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        std::process::exit(1);
    }
}
