use std::path::Path;
use std::{env, process::exit};

use rust_ls::{Config, print_newline, print_space, read_contents_of_given_directory};
/*
 * parser without
 *
 * - conflicting args like -a or -A
 * - validation phase
 * - error handling stratergy
 * - add one value option (state machine)
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_ls_arguments(&args);

    for str_path in &config.files {
        let path = Path::new(str_path);

        let mut contents = read_contents_of_given_directory(&path).unwrap();
        contents.sort_by_key(|os_str| os_str.to_string_lossy().to_lowercase());

        if config.newline {
            print_newline(&contents, config.all || config.almost_all);
        } else {
            print_space(&contents, config.all || config.almost_all);
        }
    }
}

fn parse_ls_arguments(args: &Vec<String>) -> Config {
    let mut config = Config::new();
    for arg in args.into_iter().skip(1) {
        if arg.starts_with("---") {
            exit(1);
        }

        if arg.starts_with("--") {
            match arg.as_str() {
                "--all" => config.all = true,
                "--almost-all" => config.almost_all = true,
                "--human-readable" => config.human_readable_size = true,
                "--reverse" => config.print_reverse = true,
                "--recursive" => config.show_subdirectories_content = true,
                _ => {
                    eprintln!("Unknown flag {arg}");
                    exit(1);
                }
            }
            continue;
        } else if arg.starts_with("-") {
            for ch in arg.chars().skip(1) {
                match ch {
                    'A' => config.almost_all = true,
                    'a' => config.all = true,
                    '1' => config.newline = true,
                    'l' => config.print_list_format = true,
                    'h' => config.human_readable_size = true,
                    'r' => config.print_reverse = true,
                    'R' => config.show_subdirectories_content = true,
                    _ => {
                        eprintln!("Unknown flag {ch}");
                        exit(1);
                    }
                }
            }
            continue;
        } else {
            config.files.push(arg.clone());
        }
    }

    if config.files.is_empty() {
        config.files.push(String::from("."));
    }

    config
}
