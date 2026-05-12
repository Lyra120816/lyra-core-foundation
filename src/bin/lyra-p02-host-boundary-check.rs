use lyra_phase0::p02::validate_host_boundary_challenge_surface;
use std::{env, fs, process};
fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: lyra-p02-host-boundary-check <surface.lyra>");
        process::exit(2);
    };
    let input = match fs::read_to_string(&path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("failed to read {path}: {e}");
            process::exit(2);
        }
    };
    let (verdict, receipt) = validate_host_boundary_challenge_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
