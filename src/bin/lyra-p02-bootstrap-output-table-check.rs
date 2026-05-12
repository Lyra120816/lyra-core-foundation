use std::{env, fs, process};

use lyra_phase0::p02_bootstrap_output_table_law::validate_bootstrap_output_table_surface;

fn main() {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: lyra-p02-bootstrap-output-table-check <surface.lyra>");
        process::exit(2);
    };
    if args.next().is_some() {
        eprintln!("usage: lyra-p02-bootstrap-output-table-check <surface.lyra>");
        process::exit(2);
    }
    let input = match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(2);
        }
    };
    let (verdict, receipt) = validate_bootstrap_output_table_surface(&input);
    print!("{}", receipt.to_text());
    process::exit(if verdict.accepted { 0 } else { 1 });
}
