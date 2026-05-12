use std::env;
use std::fs;
use std::process;

use lyra_phase0::p00::validate_output_table_surface;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lyra-p00-output-table-check <surface.lyra>");
        process::exit(2);
    });
    let input = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("failed to read {path}: {error}");
        process::exit(2);
    });
    let (verdict, receipt) = validate_output_table_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
