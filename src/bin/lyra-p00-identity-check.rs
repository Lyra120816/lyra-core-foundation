use std::env;
use std::fs;
use std::process::ExitCode;

use lyra_phase0::p00::validate_identity_law_surface;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: lyra-p00-identity-check <identity-law-file> [--receipt]");
        return ExitCode::from(2);
    };
    let emit_receipt = args.any(|arg| arg == "--receipt");

    let Ok(input) = fs::read_to_string(&path) else {
        eprintln!("lyra-p00-identity-check: cannot read input file");
        return ExitCode::from(2);
    };

    let (verdict, receipt) = validate_identity_law_surface(&input);
    if emit_receipt {
        print!("{}", receipt.to_text());
    } else {
        print!("{}", verdict.canonical_text());
    }

    if verdict.accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
