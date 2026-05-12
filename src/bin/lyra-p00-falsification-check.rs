use std::env;
use std::fs;
use std::process;

use lyra_phase0::p00::validate_falsification_surface;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lyra-p00-falsification-check <surface.lyra>");
        process::exit(2);
    });

    let input = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("read_error={error} path={path}");
        process::exit(2);
    });

    let (verdict, receipt) = validate_falsification_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
