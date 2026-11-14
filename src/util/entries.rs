use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::fs::{DirEntry, FileType, Metadata};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{io, vec};
use tabled::Tabled;

use Entry::{LocalDir, LocalFile};

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
    pub(crate) fn from_dir_entry(entry: &DirEntry, depth: usize) -> io::Result<Self> {
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

impl Tabled for Entry {
    const LENGTH: usize = 6;
    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            "Name".into(),
            "Type".into(),
            "Size (bytes)".into(),
            "Created".into(),
            "Last Modified".into(),
            "Depth".into(),
        ]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> {
        match self {
            LocalDir(d) => vec![
                d.name.to_string_lossy(),
                "dir".into(),
                "-".into(),
                "-".into(),
                "-".into(),
                d.depth.to_string().into(),
            ],
            LocalFile(f) => vec![
                f.name.to_string_lossy(),
                file_type_label(&f.file_type).into(),
                f.size.to_string().into(),
                format_time(f.created).into(),
                format_time(f.last_modified).into(),
                f.depth.to_string().into(),
            ],
        }
    }
}

fn file_type_label(file_type: &FileType) -> &'static str {
    if file_type.is_file() { "file" } else { "other" }
}

/// Seconds since the unix epoch, or "-" if the platform didn't provide a time.
fn format_time(time: Option<SystemTime>) -> String {
    time.and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or_else(|| "-".to_string(), |d| d.as_secs().to_string())
}

// TODO: implement
/// formats the size of a file, appending "B", "KB", "MB", "GB" where applicable
fn format_size() {}
