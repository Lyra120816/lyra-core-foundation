use std::{env, fs, process};

use lyra_phase0::p00::validate_control_surface_format_law;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: lyra-p00-control-check <surface.lyra>");
        process::exit(2);
    };

    let input = match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(2);
        }
    };

    let (verdict, receipt) = validate_control_surface_format_law(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
