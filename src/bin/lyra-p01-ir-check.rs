use std::env;
use std::fs;
use std::process;

use lyra_phase0::p01::validate_core_ir_surface;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lyra-p01-ir-check <surface.lyra>");
        process::exit(2);
    });
    let input = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("failed to read {path}: {error}");
        process::exit(2);
    });
    let (verdict, receipt) = validate_core_ir_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
