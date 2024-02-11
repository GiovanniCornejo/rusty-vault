use std::result::Result;
use std::{env, process::ExitCode};

mod password_generator;
use password_generator::*;

fn usage(program: &str) {
    eprintln!("Usage: {program} <SUBCOMMAND> [OPTIONS]");
    eprintln!("\nSubcommands");
    eprintln!("  generate                             generate a password");
    eprintln!("  create                               create your own password");
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
        "  -n, --length <LENGTH>                length of the generated password (default: 12-16)"
    );
    eprintln!("  -u, --min-upper <COUNT>              minimum uppercase letters (default: 1)");
    eprintln!("  -l, --min-lower <COUNT>              minimum lowercase letters (default: 1)");
    eprintln!("  -d, --min-digits <COUNT>             minimum digits (default: 1)");
    eprintln!("  -s, --min-special <COUNT>            minimum special characters (default: 1)");
    eprintln!("\n  -h, --help                           show this help message and exit");
}

fn usage_check(program: &str) {
    eprintln!("Usage: {program} check <PASSWORD>");
    eprintln!("Check strength of a custom password");
    eprintln!("\n  -h, --help                           show this help message and exit");
}

fn parse_count(arg: Option<String>, arg_type: &str) -> Result<usize, ()> {
    if let Some(arg) = arg {
        match arg.parse::<usize>() {
            Ok(count) => {
                if arg_type == "length" && count < ABSOLUTE_MIN {
                    eprintln!("ERROR: length must be at least {ABSOLUTE_MIN}");
                    return Err(());
                } else if count < 1 {
                    eprintln!("ERROR: minimum count for {arg_type} must be at least 1");
                    return Err(());
                }
                Ok(count)
            }
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
    let mut args = env::args().peekable();
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
            // Default settings
            let mut pw_length: Option<usize> = None;
            let mut min_uppercase = 1;
            let mut min_lowercase = 1;
            let mut min_digits = 1;
            let mut min_special = 1;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "-n" | "--length" => pw_length = Some(parse_count(args.next(), "length")?),
                    "-u" | "--min-upper" => min_uppercase = parse_count(args.next(), "uppercase")?,
                    "-l" | "--min-lower" => min_lowercase = parse_count(args.next(), "lowercase")?,
                    "-d" | "--min-digits" => min_digits = parse_count(args.next(), "digits")?,
                    "-s" | "--min-special" => min_special = parse_count(args.next(), "special")?,
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

            let pg = PasswordGeneratorBuilder::new()
                .length(pw_length)
                .min_uppercase(min_uppercase)
                .min_lowercase(min_lowercase)
                .min_digits(min_digits)
                .min_special(min_special)
                .build()?;

            let pw = pg.generate_password();
            println!("Generated password: {pw}");
        }
        "check" => {
            if args.peek().is_none() {
                usage_check(&program);
                return Ok(());
            }
            // while let Some(arg) = args.next() {
            //     match arg.as_str() {
            //         _ => {
            //             usage_check(&program);
            //             eprintln!("ERROR: unknown argument {arg}");
            //             return Err(());
            //         }
            //     }
            // }
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
