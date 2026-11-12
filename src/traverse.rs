// NOTE: use OsString here since we want to account for non-unicode
// characters and so that it will work across different operating
// systems. Either way, conversion between String and OsString is easy.

use std::cmp::Reverse;
use std::fs::{DirEntry, Metadata};
use std::io;
use std::{
    ffi::OsString,
    fs::{FileType, ReadDir},
    path::Path,
    time::SystemTime,
};

// cannot be named File to prevent clash with std::fs::File
pub struct FileInfo {
    pub name: OsString,
    pub file_type: FileType,
    pub size: u64,
    pub created: Option<SystemTime>,
    pub last_modified: Option<SystemTime>,
    pub depth: u64,
}

pub struct DirInfo {
    pub name: OsString,
    pub depth: u64,
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
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("unable to read an entry in {}: {e}", p.display());
                continue;
            }
        };
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "unable to read metadata for {}: {e}",
                    entry.path().display()
                );
                continue;
            }
        };
        if metadata.is_dir() {
            lst.push(Entry::LocalDir(DirInfo {
                name: entry.file_name(),
                depth: 1,
            }))
        } else if metadata.is_file() {
            // alternative to .ok() would be .unwrap_or(SystemTime::UNIX_EPOCH)
            let created_sys_time = metadata.created().ok();
            let modified_sys_time = metadata.modified().ok();
            lst.push(Entry::LocalFile(FileInfo {
                name: entry.file_name(),
                file_type: metadata.file_type(),
                size: metadata.len(), // better than .size(), works on every platform
                created: created_sys_time,
                last_modified: modified_sys_time,
                depth: 1,
            }))
            // TODO: add another branch for symlinks
        }
    }

    Ok(lst)
}

pub fn walk_recursive(p: &Path) -> io::Result<Vec<Entry>> {
    let mut lst = Vec::new();
    let mut stack: Vec<(DirEntry, u64)> = Vec::new();

    // get the first layer of children
    let mut children: Vec<DirEntry> = Vec::new();
    for entry in p.read_dir()? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("unable to read an entry in {}: {e}", p.display());
                continue;
            }
        };
        children.push(entry);
    }
    // sort children in reverse order, so they are popped from stack in order
    children.sort_by_cached_key(|e| Reverse(e.file_name()));
    stack.extend(children.into_iter().map(|e| (e, 1)));

    while let Some((entry, depth)) = stack.pop() {
        let metadata: Metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "unable to read metadata for {}: {e}",
                    entry.path().display()
                );
                continue;
            }
        };
        // let name = metadata.
        if metadata.is_dir() {
            lst.push(Entry::LocalDir(DirInfo {
                name: entry.file_name(),
                depth,
            }));
            let children_iter = match entry.path().read_dir() {
                Ok(it) => it,
                Err(e) => {
                    eprintln!("unable to open directory {}: {e}", entry.path().display());
                    continue;
                }
            };
            // repeat the same reversal operation
            let mut children: Vec<DirEntry> = Vec::new();
            for entry in children_iter {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => {
                        // TODO: implement better error message
                        continue;
                    }
                };
                children.push(entry);
            }
            children.sort_by_cached_key(|e| Reverse(e.file_name()));
            for child in children {
                stack.push((child, depth + 1));
            }
        } else if metadata.is_file() {
            let created_sys_time = metadata.created().ok();
            let modified_sys_time = metadata.modified().ok();
            lst.push(Entry::LocalFile(FileInfo {
                name: entry.file_name(),
                file_type: metadata.file_type(),
                size: metadata.len(),
                created: created_sys_time,
                last_modified: modified_sys_time,
                depth,
            }))
        }
    }

    Ok(lst)
}
