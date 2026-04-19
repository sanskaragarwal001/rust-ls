#[derive(Debug)]
pub struct Config {
    pub all: bool,
    pub almost_all: bool,
    pub print_reverse: bool,
    pub show_subdirectories_content: bool,
    pub newline: bool,
    pub print_list_format: bool,
    pub size: bool,
    pub human_readable_size: bool,
    pub files: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            all: false,
            almost_all: false,
            print_reverse: false,
            show_subdirectories_content: false,
            newline: false,
            print_list_format: false,
            size: false,
            human_readable_size: false,
            files: vec![],
        }
    }
}
