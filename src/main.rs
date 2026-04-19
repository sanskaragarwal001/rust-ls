use std::env;

use rust_ls::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config = Config::new();
    let mut end_of_flags = false;
    let mut expected_output = false;

    for arg in args.into_iter().skip(1) {
        if expected_output {
            if arg.starts_with("-") || arg.starts_with("--") {
                eprintln!("expected file receive flag.");
            } else {
                config.output = Some(arg);
            }
            expected_output = false;
            continue;
        }

        if arg == "--" {
            end_of_flags = true;
            continue;
        }

        if !end_of_flags {
            // Lond Options
            if arg.starts_with("--") {
                match arg.as_str() {
                    "--output" => expected_output = true,
                    "--all" => config.show_all = true,
                    "--list" => config.list_format = true,
                    _ => eprintln!("Unknown option: {arg}"),
                }
                continue;
            }

            // Short flags
            if arg.starts_with("-") {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'a' => config.show_all = true,
                        'l' => config.list_format = true,
                        _ => eprintln!("Unknown flag: {arg}"),
                    }
                }
                continue;
            }

            config.files.push(arg);
        }

        if expected_output {
            eprintln!("Error: --output requires a value but none was provided");
        }
    }

    println!("{:?}", config);
}
