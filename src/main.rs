use std::result::Result;
use std::{env, process::ExitCode};

fn usage(program: &str) {
    eprintln!("Usage: {program} generate [OPTIONS]");
    eprintln!("\nOptions:");
    eprintln!("    -h, --help                           show this help message and exit");
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program is provided");
    let subcommand = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand is provided");
    })?;

    Ok(());
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
