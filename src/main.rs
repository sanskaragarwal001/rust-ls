use std::env;

use rust_ls::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config = Config::new();

    for arg in args.into_iter().skip(1) {
        match arg.as_str() {
            "-a" => config.show_all = true,
            "-l" => config.list_format = true,
            _ => config.files.push(arg),
        }
    }

    println!("{:?}", config);
}
