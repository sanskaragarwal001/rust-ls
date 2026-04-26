use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::exit;

enum Flag {
    AlmostAll,
    Reverse,
    Recursive,
    SortedOrder,
    SizeInBytes,
    HumanReadableSize,
    ListFormat,
    NewLineFormat,
}

#[derive(Debug)]
pub struct LsConfig {
    pub almost_all: bool,
    pub reverse: bool,
    pub recursive: bool,
    pub sorted_order: bool,
    pub size_in_bytes: bool,
    pub human_readable_size: bool,
    pub newline: bool,
    pub list: bool,
    pub paths: Vec<PathBuf>,
}

pub struct LsParser {
    command_name: &'static str,
    options: HashMap<&'static str, Flag>,
}

impl LsConfig {
    pub fn default() -> Self {
        LsConfig {
            almost_all: false,
            reverse: false,
            recursive: false,
            sorted_order: true,
            size_in_bytes: false,
            human_readable_size: false,
            newline: false,
            list: false,
            paths: Vec::new(),
        }
    }
}

impl LsParser {
    pub fn new(name: &'static str) -> Self {
        let mut options = HashMap::new();
        options.insert("l", Flag::ListFormat);
        options.insert("--list", Flag::ListFormat);

        options.insert("A", Flag::AlmostAll);
        options.insert("--almost-all", Flag::AlmostAll);

        options.insert("r", Flag::Reverse);
        options.insert("--reverse", Flag::Reverse);

        options.insert("R", Flag::Recursive);
        options.insert("--recursive", Flag::Recursive);

        options.insert("s", Flag::SizeInBytes);
        options.insert("--size", Flag::SizeInBytes);

        options.insert("f", Flag::SortedOrder);
        options.insert("1", Flag::NewLineFormat);
        options.insert("h", Flag::HumanReadableSize);

        LsParser {
            command_name: name,
            options,
        }
    }

    pub fn parse_args(&self) -> LsConfig {
        let args: Vec<String> = env::args().collect();
        if !self.match_command_name(args.get(1)) {
            eprintln!("Command not started with {}", self.command_name);
            exit(1);
        }

        let mut config = LsConfig::default();

        let big_flag_regex = Regex::new(r"^--[a-zA-Z]+$").unwrap();
        let short_flag_regex = Regex::new(r"^-[a-zA-z0-9]+$").unwrap();

        for arg in args.into_iter().skip(2) {
            if big_flag_regex.is_match(&arg) {
                self.handle_flag(&arg, &mut config);
            } else if short_flag_regex.is_match(&arg) {
                for c in arg.chars().skip(1) {
                    let c = format!("{c}");
                    self.handle_flag(&c, &mut config);
                }
            } else {
                config.paths.push(PathBuf::from(arg));
            }
        }

        if config.paths.is_empty() {
            config.paths.push(PathBuf::from("."));
        }

        config
    }

    fn handle_flag(&self, arg: &String, config: &mut LsConfig) {
        match self.options.get(arg.as_str()) {
            Some(flag_type) => match flag_type {
                Flag::AlmostAll => config.almost_all = true,
                Flag::ListFormat => {
                    config.size_in_bytes = true;
                    config.newline = true;
                    config.list = true;
                }
                Flag::NewLineFormat => config.newline = true,
                Flag::HumanReadableSize => config.human_readable_size = true,
                Flag::Reverse => config.reverse = true,
                Flag::SortedOrder => config.sorted_order = false,
                Flag::Recursive => config.recursive = true,
                Flag::SizeInBytes => config.size_in_bytes = true,
            },
            None => {
                eprintln!("{}, invalid options -- '{}'", self.command_name, arg);
                exit(1);
            }
        }
    }

    fn match_command_name(&self, command_name: Option<&String>) -> bool {
        return match command_name {
            Some(name) => name == self.command_name,
            None => false,
        };
    }
}
