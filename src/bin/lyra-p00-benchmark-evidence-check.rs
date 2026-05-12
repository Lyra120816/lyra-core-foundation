use std::env;
use std::fs;
use std::process;

use lyra_phase0::p00::validate_benchmark_evidence_law_surface;

fn main() {
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| "lyra-p00-benchmark-evidence-check".to_string());
    let Some(path) = args.next() else {
        eprintln!("usage: {program} <benchmark-evidence-law-surface.lyra>");
        process::exit(2);
    };
    if args.next().is_some() {
        eprintln!("usage: {program} <benchmark-evidence-law-surface.lyra>");
        process::exit(2);
    }

    let input = match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(2);
        }
    };

    let (verdict, receipt) = validate_benchmark_evidence_law_surface(&input);
    print!("{}", receipt.to_text());
    if !verdict.accepted {
        process::exit(1);
    }
}
