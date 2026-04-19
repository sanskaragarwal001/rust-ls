#[derive(Debug)]
pub struct Config {
    pub show_all: bool,
    pub list_format: bool,
    pub files: Vec<String>,
    pub output: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            show_all: false,
            list_format: false,
            files: vec![],
            output: None,
        }
    }
}

pub fn show_all_files() {
    println!("show all files");
}

pub fn show_list_format() {
    println!("show list format");
}
