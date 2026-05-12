use std::{env, fs, process};

use lyra_phase0::p02::validate_bootstrap_benchmark_pack_surface;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        "fixtures/p02/bootstrap_benchmark_pack_inputs/valid_bootstrap_benchmark_pack.lyra"
            .to_string()
    });
    let input = match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(2);
        }
    };
    let (verdict, receipt) = validate_bootstrap_benchmark_pack_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
