use std::result::Result;
use std::{env, process::ExitCode};

mod password_generator;
use password_generator::*;

fn usage(program: &str) {
    eprintln!("Usage: {program} generate [OPTIONS]");
    eprintln!("\nOptions:");
    eprintln!(
        "    -l, --length <LENGTH>                specify the length of the generated password (default: 12-16)"
    );
    eprintln!("    -U, --no-uppercase                   exclude uppercase letters");
    eprintln!("    -L, --no-lowercase                   exclude lowercase letters");
    eprintln!("    -d, --no-digits                      exclude digits");
    eprintln!("    -s, --no-special-chars               exclude special characters");
    eprintln!("    -h, --help                           show this help message and exit");
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program is provided");
    let subcommand = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand is provided");
    })?;

    match subcommand.as_str() {
        "generate" => {
            let mut pw_length: Option<usize> = None;
            let mut uppercase = true;
            let mut lowercase = true;
            let mut digits = true;
            let mut special_chars = true;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "-l" | "--length" => {
                        if let Some(arg) = args.next() {
                            match arg.parse::<usize>() {
                                Ok(len) => pw_length = Some(len),
                                Err(_) => {
                                    eprintln!("ERROR: invalid length argument {arg}");
                                    return Err(());
                                }
                            }
                        } else {
                            eprintln!("ERROR: no length argument provided");
                            return Err(());
                        }
                    }
                    "-U" | "--no-uppercase" => {
                        uppercase = false;
                    }
                    "-L" | "--no-lowercase" => {
                        lowercase = false;
                    }
                    "-d" | "--no-digits" => {
                        digits = false;
                    }
                    "-s" | "--no-special-chars" => {
                        special_chars = false;
                    }
                    "-h" | "--help" => {
                        usage(&program);
                        return Ok(());
                    }
                    _ => {
                        usage(&program);
                        eprintln!("ERROR: unknown argument {arg}");
                        return Err(());
                    }
                }
            }

            if !uppercase && !lowercase && !digits && !special_chars {
                eprintln!("ERROR: password generator must include at least one character set");
                return Err(());
            }

            let pg = PasswordGenerator::new(pw_length, uppercase, lowercase, digits, special_chars);
            let pw = pg.generate_password();
            println!("Generated password: {pw}");

            Ok(())
        }
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand {}", subcommand);
            Err(())
        }
    }
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
