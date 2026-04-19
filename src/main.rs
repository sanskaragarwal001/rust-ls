use std::env;
use std::fs;

fn main() {
    /*
     * ls [path] - without flags
     */

    let args: Vec<String> = env::args().collect();

    let path = match args.last() {
        Some(path) => path,
        None => ".",
    };

    let contents = fs::read_dir(path).expect("failed to read the directory content.");
    for content in contents {
        match content {
            Ok(meta) => println!("{0:?}", meta.file_name()),
            Err(_) => eprintln!("failed to read the directory name."),
        }
    }
}
