use rust_ls::parser::LsParser;
use rust_ls::{print_on_console, read_directory};

fn main() {
    let ls_parser = LsParser::new("ls");

    let config = ls_parser.parse_args();

    for path in &config.paths {
        let mut entries = read_directory(path.as_path()).unwrap();
        println!("{}:", path.display());
        print_on_console(&mut entries, &config);
        print!("\n");
    }
}
