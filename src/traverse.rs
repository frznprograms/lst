// NOTE: use OsString here since we want to account for non-unicode
// characters and so that it will work across different operating
// systems. Either way, conversion between String and OsString is easy.

use std::io;
use std::{
    ffi::OsString,
    fs::{FileType, ReadDir},
    os::unix::fs::MetadataExt,
    path::Path,
    time::SystemTime,
};

// cannot be named File to prevent clash with std::fs::File
pub struct FileInfo {
    name: OsString,
    file_type: FileType,
    size: u64,
    created: SystemTime,
    last_modified: SystemTime,
}

pub struct DirInfo {
    name: OsString,
}

pub enum Entry {
    LocalFile(FileInfo),
    LocalDir(DirInfo),
}

pub fn walk(p: &Path) -> io::Result<Vec<Entry>> {
    let mut lst: Vec<Entry> = Vec::new();
    // if this fails, no point continuing; let main handle it
    let iterator: ReadDir = p.read_dir()?;
    for entry in iterator {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    lst.push(Entry::LocalDir(DirInfo {
                        name: entry.file_name(),
                    }))
                } else if metadata.is_file() {
                    let created_sys_time = match metadata.created() {
                        Ok(time) => time,
                        Err(_) => SystemTime(), // TODO: change to sensible default
                    };
                    let modified_sys_time = match metadata.modified() {
                        Ok(time) => time,
                        Err(_) => SystemTime(),
                    };
                    lst.push(Entry::LocalFile(FileInfo {
                        name: entry.file_name(),
                        file_type: metadata.file_type(),
                        size: metadata.size(),
                        created: created_sys_time,
                        last_modified: modified_sys_time,
                    }))
                }
            } else {
                eprintln!("unable to read metadata from {}", entry.path().display());
                continue;
            }
        } else {
            eprintln!("unable to read {}", entry.file_path());
            continue;
        }
    }

    Ok(lst)
}
