use chrono::{DateTime, Local};
use std::ffi::OsString;
use std::fs::{self, DirEntry};
use std::io::{self};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::process::exit;
use uzers::{get_group_by_gid, get_user_by_uid};

#[derive(Debug)]
pub struct FileMetaData {
    pub permission: String,
    pub nlink: u64,
    pub user_name: OsString,
    pub group_name: OsString,
    pub size_in_bytes: u64,
    pub last_modified: String,
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

pub fn sort_directory_entries_by_file_name(contents: &mut Vec<FileMetaData>) {
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
