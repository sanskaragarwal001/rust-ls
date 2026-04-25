use chrono::{DateTime, Local};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::path::PathBuf;
use uzers::{get_group_by_gid, get_user_by_uid};

#[derive(Debug)]
pub struct Config {
    pub almost_all: bool,
    pub print_reverse: bool,
    pub show_subdirectories_content: bool,
    pub display_directory_order: bool,
    pub newline: bool,
    pub print_list_format: bool,
    pub size: bool,
    pub human_readable_size: bool,
    pub files: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            almost_all: false,
            print_reverse: false,
            display_directory_order: false,
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

pub fn print_btree(contents: &BTreeMap<PathBuf, Vec<String>>, config: &Config) {
    for (key, value) in contents.iter() {
        println!("{}", key.display());

        if config.newline {
            for entry in value {
                println!("{entry}");
            }
        } else {
            for entry in value {
                print!("{entry} ");
            }
        }

        println!("\n");
    }
}

pub fn read_recursive(
    path: &Path,
    config: &Config,
) -> Result<BTreeMap<PathBuf, Vec<String>>, io::Error> {
    let mut all_data = BTreeMap::new();

    let mut entries = Vec::new();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if !config.almost_all && name.starts_with('.') {
            continue;
        }

        entries.push(name);
    }

    if config.display_directory_order == false {
        entries.sort_by(|a, b| {
            let a_n = a.to_lowercase();
            let b_n = b.to_lowercase();

            a_n.strip_prefix('.')
                .unwrap_or(&a_n)
                .cmp(b_n.strip_prefix('.').unwrap_or(&b_n))
        });
    }
    if config.display_directory_order == false && config.print_reverse {
        entries.reverse();
    }

    all_data.insert(path.to_path_buf(), entries.clone());

    for name in entries {
        let full_path = path.join(&name);
        if full_path.is_dir() {
            let sub_results = read_recursive(&full_path, config)?;
            all_data.extend(sub_results);
        }
    }

    Ok(all_data)
}

pub fn read_list(config: &Config) {
    // let sys_time = SystemTime::now();
    for str_path in &config.files {
        let path = Path::new(str_path);

        for dir in fs::read_dir(&path).unwrap() {
            let dir = dir.unwrap();

            let path = dir.path();
            let metadata = path.metadata().expect("metadata call failed");

            // let duration = sys_time
            //     .duration_since(metadata.modified().unwrap())
            //     .expect("Time went backwards");
            let last_modified_time: DateTime<Local> = metadata.modified().unwrap().into();
            println!(
                "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
                parse_permissions(metadata.permissions().mode()),
                metadata.nlink(),
                get_user_by_uid(metadata.uid()).unwrap().name(),
                get_group_by_gid(metadata.gid()).unwrap().name(),
                metadata.size(),
                last_modified_time.format("%b %d %H:%M").to_string(),
                dir.file_name(),
            );
        }
    }
}

fn parse_permissions(mode: u32) -> String {
    let mut s = String::with_capacity(10);

    // 1. Determine File Type
    let file_type = if mode & 0o170000 == 0o040000 {
        'd'
    } else if mode & 0o170000 == 0o120000 {
        'l'
    } else {
        '-'
    };
    s.push(file_type);

    // 2. Define the bits we want to check
    let bits = [
        (0o400, 'r'),
        (0o200, 'w'),
        (0o100, 'x'), // Owner
        (0o040, 'r'),
        (0o020, 'w'),
        (0o010, 'x'), // Group
        (0o004, 'r'),
        (0o002, 'w'),
        (0o001, 'x'), // Others
    ];

    for (bit, char) in bits {
        if mode & bit != 0 {
            s.push(char);
        } else {
            s.push('-');
        }
    }

    s
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
