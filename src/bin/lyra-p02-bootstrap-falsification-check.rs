use std::env;
use std::fs;
use std::process;

use lyra_phase0::p02::validate_bootstrap_falsification_surface;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: lyra-p02-bootstrap-falsification-check <surface.lyra>");
        process::exit(2);
    };
    let input = match fs::read_to_string(&path) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(2);
        }
    };
    let (verdict, receipt) = validate_bootstrap_falsification_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
