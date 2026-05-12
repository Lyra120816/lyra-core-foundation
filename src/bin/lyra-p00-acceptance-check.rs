use std::{env, fs, process};

use lyra_phase0::p00::validate_acceptance_proof_surface;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: lyra-p00-acceptance-check <surface.lyra>");
        process::exit(2);
    };

    let input = match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(2);
        }
    };

    let (verdict, receipt) = validate_acceptance_proof_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
