use lyra_phase0::p02::validate_bootstrap_emergency_fallback_surface;
fn main() {
    let mut args = std::env::args();
    let _program = args.next();
    let Some(path) = args.next() else {
        eprintln!("usage: lyra-p02-emergency-fallback-check <surface.lyra>");
        std::process::exit(2);
    };
    let input = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("read error {path}: {e}");
        std::process::exit(2)
    });
    let (verdict, receipt) = validate_bootstrap_emergency_fallback_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        std::process::exit(1);
    }
}
