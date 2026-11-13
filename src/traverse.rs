use std::cmp::Reverse;
use std::ffi::OsStr;
use std::fs::{DirEntry, Metadata};
use std::io;
use std::{ffi::OsString, fs::FileType, path::Path, time::SystemTime};

use crate::traverse::Entry::{LocalDir, LocalFile};

// cannot be named File to prevent clash with std::fs::File
pub struct FileInfo {
    pub name: OsString,
    pub file_type: FileType,
    pub size: u64,
    pub created: Option<SystemTime>,
    pub last_modified: Option<SystemTime>,
    pub depth: usize,
}

pub struct DirInfo {
    pub name: OsString,
    pub depth: usize,
}

// TODO: implement display trait on Entry
pub enum Entry {
    LocalFile(FileInfo),
    LocalDir(DirInfo),
}

impl Entry {
    /// Associated fn of Entry. Build an Entry from a DirEntry, Fails only if
    /// metadata can't be read.
    fn from_dir_entry(entry: &DirEntry, depth: usize) -> io::Result<Self> {
        let meta: Metadata = entry.metadata()?;
        let name = entry.file_name();
        if meta.is_dir() {
            Ok(LocalDir(DirInfo { name, depth }))
        } else if meta.is_file() {
            Ok(LocalFile(FileInfo {
                name,
                file_type: meta.file_type(),
                size: meta.len(),
                created: meta.created().ok(),
                last_modified: meta.modified().ok(),
                depth,
            }))
        } else {
            // TODO: need to account for symlinks
            Err(io::Error::other("not a regular file or directory"))
        }
    }

    /// Retrieve name of an entry.
    pub fn name(&self) -> &OsStr {
        match self {
            LocalFile(f) => &f.name,
            LocalDir(d) => &d.name,
        }
    }

    /// Retrieve depth of an entry.
    pub fn depth(&self) -> usize {
        match self {
            LocalFile(f) => f.depth,
            LocalDir(d) => d.depth,
        }
    }

    /// Identify is entry is a directory. Check that it matches the local
    /// directory struct LocalDir as defined.
    pub fn is_dir(&self) -> bool {
        let res = matches!(self, Entry::LocalDir(_));
        res
    }
}

/// Helper fn. Reads all children from a directory. Sorts the entries
/// in alphabetical order by default.
fn read_children(dir: &Path) -> io::Result<Vec<DirEntry>> {
    let mut children = Vec::new();
    for child in dir.read_dir()? {
        let child = match child {
            Ok(c) => c,
            Err(e) => {
                eprintln!("unable to read an entry in {}: {e}", dir.display());
                continue;
            }
        };
        children.push(child);
    }
    children.sort_by_cached_key(|e| Reverse(e.file_name()));
    Ok(children)
}

/// Iterative pre-order DFS to walk through a file directory.
/// When max_depth = Some(1), this functions as a plain `ls` command on CLI.
/// When max_depth = None, it traverses the entire file tree.
/// Fails only if `root` itself cannot be read.
pub fn walk(root: &Path, max_depth: Option<usize>) -> io::Result<Vec<Entry>> {
    let mut lst = Vec::new();
    let mut stack: Vec<(DirEntry, usize)> = Vec::new();

    // root failing to read is propagated to main
    // sort children in reverse order, so they are popped from stack in order
    let root_children: Vec<DirEntry> = read_children(root)?;
    stack.extend(root_children.into_iter().map(|e| (e, 1)));

    while let Some((dir_entry, depth)) = stack.pop() {
        let entry = match Entry::from_dir_entry(&dir_entry, depth) {
            Ok(en) => en,
            Err(e) => {
                eprintln!(
                    "unable to read metadata for {}: {e}",
                    dir_entry.path().display()
                );
                continue;
            }
        };
        // check recursive descend condition
        let descend: bool = entry.is_dir() && max_depth.is_none_or(|max| depth < max);
        lst.push(entry);
        if !descend {
            continue;
        }

        let children = match read_children(&dir_entry.path()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "unable to open directory {}: {e}",
                    dir_entry.path().display()
                );
                continue;
            }
        };
        // reverse and add to the stack with higher depth
        stack.extend(children.into_iter().map(|e| (e, depth + 1)));
    }

    Ok(lst)
}
