use chrono::{DateTime, Local};
use parser::LsConfig;
use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::io::{self};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::exit;
use uzers::{get_group_by_gid, get_user_by_uid};

pub mod parser;

#[derive(Debug)]
pub struct FileMetaData {
    pub permission: String,
    pub nlink: u64,
    pub user_name: OsString,
    pub group_name: OsString,
    pub size_in_bytes: u64,
    pub last_modified: String,
    pub is_directory: bool,
    pub file_name: OsString,
}

impl FileMetaData {
    pub fn extract_metadata_from_dir_entry(entry: &DirEntry) -> Result<Self, io::Error> {
        let metadata = entry.metadata()?;

        let file_name = entry.file_name();
        let permission = metadata.permissions().mode();
        let nlink = metadata.nlink();
        let size = metadata.size();

        let user_name = match get_user_by_uid(metadata.uid()) {
            Some(user) => user.name().to_os_string(),
            None => OsString::from("User not found"),
        };

        let group_name = match get_group_by_gid(metadata.gid()) {
            Some(group) => group.name().to_os_string(),
            None => OsString::from("Group not found"),
        };

        let last_modified_time: DateTime<Local> = metadata.modified()?.into();
        let last_modified_time: String = last_modified_time.format("%b %d %H:%M").to_string();

        Ok(FileMetaData {
            permission: parse_permissions(permission),
            nlink: nlink,
            user_name: user_name,
            group_name: group_name,
            is_directory: metadata.is_dir(),
            size_in_bytes: size,
            last_modified: last_modified_time,
            file_name: file_name,
        })
    }
}

pub fn read_directory(path: &Path) -> Result<Vec<FileMetaData>, io::Error> {
    if !path.is_dir() {
        eprintln!("expected directory found {}", path.display());
        exit(1);
    }

    let entries = fs::read_dir(path)?;

    let mut res: Vec<FileMetaData> = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => {
                let entry = FileMetaData::extract_metadata_from_dir_entry(&entry)?;
                res.push(entry);
            }
            Err(err) => {
                eprintln!("unable to read the content {}", err.to_string());
            }
        }
    }

    Ok(res)
}

fn sort_directory_entries_by_file_name(contents: &mut Vec<FileMetaData>) {
    contents.sort_by(|a, b| {
        let a_file_name = &a.file_name;
        let b_file_name = &b.file_name;

        let a_str = a_file_name.to_string_lossy().to_lowercase();
        let b_str = b_file_name.to_string_lossy().to_lowercase();

        // Strip the leading dot if it exists for the sake of comparison
        let a_normalized = a_str.strip_prefix('.').unwrap_or(&a_str);
        let b_normalized = b_str.strip_prefix('.').unwrap_or(&b_str);

        a_normalized.cmp(b_normalized)
    });
}

pub fn print_on_console(path: &PathBuf, entries: &mut Vec<FileMetaData>, config: &LsConfig) {
    if config.sorted_order {
        sort_directory_entries_by_file_name(entries);
    }
    if config.reverse {
        entries.reverse();
    }

    for entry in entries.into_iter() {
        if config.almost_all == false && entry.file_name.to_string_lossy().starts_with(".") {
            continue;
        }

        let size = if config.human_readable_size {
            format_size(entry.size_in_bytes)
        } else {
            entry.size_in_bytes.to_string()
        };

        if config.newline {
            if config.list {
                println!(
                    "{} {} {} {} {} {} {}",
                    entry.permission,
                    entry.nlink,
                    entry.user_name.display(),
                    entry.group_name.display(),
                    size,
                    entry.last_modified,
                    entry.file_name.display()
                );
            } else if config.size_in_bytes {
                println!("({}) {}", size, entry.file_name.display());
            } else {
                println!("{}", entry.file_name.display());
            }
        } else {
            if config.size_in_bytes {
                print!("({}) {} ", size, entry.file_name.display());
            } else {
                print!("{} ", entry.file_name.display());
            }
        }
    }
    print!("\n");

    if config.recursive {
        for entry in entries.into_iter() {
            if config.almost_all == false && entry.file_name.to_string_lossy().starts_with(".") {
                continue;
            }
            if !entry.is_directory {
                continue;
            }

            let dir_path = path.join(&entry.file_name);
            let mut sub_entries = read_directory(&dir_path).unwrap();

            println!("{}", dir_path.display());
            print_on_console(&dir_path, &mut sub_entries, &config);
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

fn format_size(bytes: u64) -> String {
    let units = ["B", "K", "M", "G", "T", "P"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    // Scale the number down by 1024 for each unit
    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    // Formatting rules:
    // 1. If it's just bytes, don't show decimals (e.g., "12B")
    // 2. If it's a directory (usually 4096 bytes), show "4.0K"
    // 3. For larger files, show one decimal place (e.g., "120K", "1.2M")
    if unit_index == 0 {
        format!("{}{}", bytes, units[unit_index])
    } else {
        format!("{:.1}{}", size, units[unit_index])
    }
}
