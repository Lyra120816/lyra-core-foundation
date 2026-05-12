use lyra_phase0::p02::validate_bootstrap_truth_cleanup_surface;
use std::{env, fs, process};
fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        "fixtures/p02/bootstrap_truth_cleanup_inputs/valid_bootstrap_truth_cleanup.lyra".to_string()
    });
    let input = match fs::read_to_string(&path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("failed to read {path}: {e}");
            process::exit(2);
        }
    };
    let (verdict, receipt) = validate_bootstrap_truth_cleanup_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
