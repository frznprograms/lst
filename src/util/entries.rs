use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::fs::{DirEntry, FileType, Metadata};
use std::time::SystemTime;
use std::{io, iter, vec};
use tabled::Tabled;

use Entry::{LocalDir, LocalFile};
use time::OffsetDateTime;
use time::format_description::BorrowedFormatItem;
use time::macros::format_description;

// cannot be named File to prevent clash with std::fs::File
#[derive(Debug)]
pub struct FileInfo {
    pub name: OsString,
    pub file_type: FileType,
    pub size: u64,
    pub created: Option<SystemTime>,
    pub last_modified: Option<SystemTime>,
    pub depth: usize,
}

#[derive(Debug)]
pub struct DirInfo {
    pub name: OsString,
    pub depth: usize,
}

#[derive(Debug)]
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
    const LENGTH: usize = 5;
    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            "Name".into(),
            "Type".into(),
            "Size".into(),
            "Created".into(),
            "Last Modified".into(),
        ]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> {
        match self {
            LocalDir(d) => vec![
                format_name(d.name.to_string_lossy(), d.depth, true),
                "dir".into(),
                "-".into(),
                "-".into(),
                "-".into(),
            ],
            LocalFile(f) => vec![
                format_name(f.name.to_string_lossy(), f.depth, false),
                "file".into(),
                format_size(f.size).into(),
                format_time(f.created).into(),
                format_time(f.last_modified).into(),
            ],
        }
    }
}

// format the name of a file/dir by adding a small indent based on the
// depth of the file/dir relative to the root.
// returns a Cow<str>, so no need to convert further
fn format_name(name: Cow<str>, depth: usize, is_dir: bool) -> Cow<str> {
    let indent = 2 * (depth - 1);
    if indent == 0 && !is_dir {
        return name;
    };

    let mut res = String::with_capacity(indent + name.len() + 1);
    // same as for 0..indent {res.push_str(" ")};
    res.extend(iter::repeat_n(" ", indent));
    res.push_str(&name);
    if is_dir {
        res.push('/');
    }

    Cow::Owned(res)
}

/// Formats a file size with a suffix, using decimal units (1 KB = 1000 B).
/// Returns "B", "KB", "MB" or "GB" where applicable. Bytes are shown exactly,
/// larger units are rounded to 2 d.p.
fn format_size(size: u64) -> String {
    let bytes = size as f64;
    match size {
        0..1_000 => format!("{size} B"),
        1_000..1_000_000 => format!("{:.2} KB", bytes / 1e3),
        1_000_000..1_000_000_000 => format!("{:.2} MB", bytes / 1e6),
        _ => format!("{:.2} GB", bytes / 1e9),
    }
}

/// formats the time in a prettier format.
fn format_time(time: Option<SystemTime>) -> String {
    let system_time = match time {
        Some(t) => t,
        None => return "_".to_string(),
    };
    let dt: OffsetDateTime = OffsetDateTime::from(system_time);
    let fmt: &[BorrowedFormatItem<'_>] =
        format_description!("[year]-[month]-[day] [hour]:[minute]");

    // capture error if any and return "-", else just return the time
    dt.format(fmt).unwrap_or_else(|_| "-".to_string())
}

// TODO: implement cli flag to toggle usage of bytes or higher
