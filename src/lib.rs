use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;

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

pub fn read_contents_of_given_directory(path: &Path) -> Result<Vec<OsString>, io::Error> {
    let dir_contents = fs::read_dir(path)?;

    let mut res: Vec<OsString> = Vec::new();
    for content in dir_contents {
        match content {
            Ok(c) => {
                res.push(c.file_name());
            }
            Err(err) => {
                eprintln!("{}", err.to_string())
            }
        }
    }

    Ok(res)
}

pub fn print_space(contents: &Vec<OsString>, includes_content_starts_with_period: bool) {
    for content in contents {
        if includes_content_starts_with_period {
            print!("{content:?} ");
        } else if content.to_string_lossy().starts_with(".") == false {
            print!("{content:?} ");
        }
    }
}

pub fn print_newline(contents: &Vec<OsString>, includes_content_starts_with_period: bool) {
    for content in contents {
        if includes_content_starts_with_period {
            println!("{content:?}");
        } else if content.to_string_lossy().starts_with(".") == false {
            println!("{content:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_all_directory_content() {
        let path = Path::new("/home/sanskar/Desktop");

        let contents = read_contents_of_given_directory(&path);
        assert_eq!(false, contents.unwrap().is_empty());
    }
}
