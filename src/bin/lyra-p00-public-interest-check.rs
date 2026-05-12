use std::{env, fs, process};

use lyra_phase0::p00::validate_public_interest_law_surface;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: lyra-p00-public-interest-check <surface.lyra>");
        process::exit(2);
    };

    let input = match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(2);
        }
    };

    let (verdict, receipt) = validate_public_interest_law_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
