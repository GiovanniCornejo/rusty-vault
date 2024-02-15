use std::result::Result;
use std::{env, process::ExitCode};

mod password_generator;
use password_generator::*;

fn usage(program: &str) {
    eprintln!("Usage: {program} <SUBCOMMAND> [OPTIONS]");
    eprintln!("\nSubcommands");
    eprintln!("  generate                             generate a password");
    eprintln!("  check                                check strength of a password");
    // Add more subcommands as needed

    eprintln!(
        "\nUse '{program} <SUBCOMMAND> --help' for more information on a specific subcommand"
    );
}

fn usage_generate(program: &str) {
    eprintln!("Usage: {program} generate [OPTIONS]");
    eprintln!("Generate a password");
    eprintln!("\nOptions:");
    eprintln!(
        "  -n, --length <LENGTH>                length of the generated password (default: {DEFAULT_MIN}-{DEFAULT_MAX}, minimum: {ALLOWED_MIN})"
    );
    eprintln!(
        "  -u, --min-upper <COUNT>              minimum uppercase letters (default: 1, minimum: 1)"
    );
    eprintln!(
        "  -l, --min-lower <COUNT>              minimum lowercase letters (default: 1, minimum: 1)"
    );
    eprintln!("  -d, --min-digits <COUNT>             minimum digits (default: 1, minimum: 1)");
    eprintln!("  -s, --min-special <COUNT>            minimum special characters (default: 1, minimum: 1)");
    eprintln!("\n  -h, --help                           show this help message and exit");
    eprintln!("\nNote: If the specified minimum counts exceed the specified length,");
    eprintln!("      the length will automatically be adjusted");
}

fn usage_check(program: &str) {
    eprintln!("Usage: {program} check <PASSWORD>");
    eprintln!("Measure a password's strength by checking its entropy, length, character variety, repeating patterns, and most common passwords");
    eprintln!("\n  -h, --help                           show this help message and exit");
}

fn parse_usize(arg: Option<String>, arg_type: &str) -> Result<usize, ()> {
    if let Some(arg) = arg {
        match arg.parse::<usize>() {
            Ok(count) => Ok(count),
            Err(_) => {
                eprintln!("ERROR: invalid {arg_type} count: {arg}");
                Err(())
            }
        }
    } else {
        eprintln!("ERROR: no {arg_type} argument provided");
        Err(())
    }
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program is provided");
    let subcommand = match args.next() {
        Some(subcommand) => subcommand,
        None => {
            usage(&program);
            return Ok(());
        }
    };

    match subcommand.as_str() {
        "generate" => {
            let mut pg = PasswordGenerator::new();

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "-n" | "--length" => pg.length(parse_usize(args.next(), "length")?)?,
                    "-u" | "--min-upper" => pg.min_upper(parse_usize(args.next(), "uppercase")?)?,
                    "-l" | "--min-lower" => pg.min_lower(parse_usize(args.next(), "lowercase")?)?,
                    "-d" | "--min-digits" => pg.min_digits(parse_usize(args.next(), "digits")?)?,
                    "-s" | "--min-special" => {
                        pg.min_special(parse_usize(args.next(), "special")?)?;
                    }
                    "-h" | "--help" => {
                        usage_generate(&program);
                        return Ok(());
                    }
                    _ => {
                        usage_generate(&program);
                        eprintln!("ERROR: unknown argument {arg}");
                        return Err(());
                    }
                }
            }

            let pw = pg.generate_password();
            println!("Generated password: {pw}");
        }
        "check" => {
            let mut pw = match args.next() {
                Some(s) if s == "-h" || s == "--help" => {
                    usage_check(&program);
                    return Ok(());
                }
                Some(s) => s,
                None => {
                    usage_check(&program);
                    return Ok(());
                }
            };
            for arg in args {
                pw += &arg;
            }

            match PasswordGenerator::validate_password(pw.trim(), true) {
                2 => println!("Password strength: VERY STRONG"),
                1 => println!("Password strength: STRONG"),
                0 => println!("Password strength: MEDIUM"),
                -1 => println!("Password strength: WEAK"),
                -2 => println!("Password strength: VERY WEAK"),
                _ => println!("Password strength: COMMON"),
            }
        }
        _ => {
            usage_generate(&program);
            eprintln!("ERROR: unknown subcommand {}", subcommand);
            return Err(());
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
