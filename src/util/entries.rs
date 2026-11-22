use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::fs::{FileType, Metadata};
use std::time::SystemTime;
use std::{io, iter};

use ignore::DirEntry;
use time::OffsetDateTime;
use time::format_description::BorrowedFormatItem;
use time::macros::format_description;

use crate::util::icons::icon_for;
use Entry::{LocalDir, LocalFile};

/// A file (or symlink) in the listing. Not called `File` to avoid clashing
/// with `std::fs::File`.
#[derive(Debug)]
pub struct FileInfo {
    pub name: OsString,
    pub file_type: FileType,
    pub size: u64,
    pub created: Option<SystemTime>,
    pub last_modified: Option<SystemTime>,
    pub depth: usize,
}

/// A directory in the listing.
#[derive(Debug)]
pub struct DirInfo {
    pub name: OsString,
    pub depth: usize,
}

/// One row of the listing: either a file or a directory.
#[derive(Debug)]
pub enum Entry {
    LocalFile(FileInfo),
    LocalDir(DirInfo),
}

impl Entry {
    /// Builds an `Entry` from a walker `DirEntry`. Depth comes from the walker
    /// (children of the root are depth 1). Fails if the metadata can't be
    /// read, or if the entry is neither a file, directory nor symlink (e.g. a
    /// socket or FIFO).
    pub(crate) fn from_dir_entry(entry: &DirEntry) -> io::Result<Self> {
        // the walker doesn't follow links, so this is the link's own
        // metadata (lstat), not its target's
        let meta: Metadata = entry.metadata().map_err(io::Error::other)?;
        let name = entry.file_name().to_os_string();
        let depth = entry.depth();
        if meta.is_dir() {
            Ok(LocalDir(DirInfo { name, depth }))
        } else if meta.is_file() || meta.is_symlink() {
            Ok(LocalFile(FileInfo {
                name,
                file_type: meta.file_type(),
                size: meta.len(),
                created: meta.created().ok(),
                last_modified: meta.modified().ok(),
                depth,
            }))
        } else {
            Err(io::Error::other("not a regular file or directory"))
        }
    }

    /// The entry's file name (without its parent path).
    pub fn name(&self) -> &OsStr {
        match self {
            LocalFile(f) => &f.name,
            LocalDir(d) => &d.name,
        }
    }

    /// How deep the entry sits below the root (children of the root are 1).
    pub fn depth(&self) -> usize {
        match self {
            LocalFile(f) => f.depth,
            LocalDir(d) => d.depth,
        }
    }

    /// Whether the entry is a directory.
    pub fn is_dir(&self) -> bool {
        matches!(self, LocalDir(_))
    }

    /// Column headers for the table, matching the cells from [`Entry::row`].
    pub fn headers(include_times: bool) -> Vec<&'static str> {
        let mut res = vec!["Name", "Type", "Size"];
        if include_times {
            res.extend_from_slice(&["Created", "Last Modified"]);
        }
        res
    }

    /// The entry's table cells, in the same order as [`Entry::headers`].
    pub fn row(&self, include_times: bool) -> Vec<Cow<'_, str>> {
        match self {
            LocalDir(d) => {
                let mut res = vec![
                    format_name(&d.name.to_string_lossy(), d.depth, true),
                    "dir".into(),
                    "-".into(),
                ];
                if include_times {
                    res.extend_from_slice(&["-".into(), "-".into()]);
                }
                res
            }
            LocalFile(f) => {
                let mut res = vec![
                    format_name(&f.name.to_string_lossy(), f.depth, false),
                    "file".into(),
                    format_size(f.size).into(),
                ];
                if include_times {
                    res.extend_from_slice(&[
                        format_time(f.created).into(),
                        format_time(f.last_modified).into(),
                    ]);
                }
                res
            }
        }
    }
}

/// Builds the Name cell: two spaces of indent per level below the root, the
/// Nerd Font icon, then the name (with a trailing `/` for directories).
fn format_name(name: &str, depth: usize, is_dir: bool) -> Cow<'static, str> {
    let indent = 2 * depth.saturating_sub(1);
    let icon = icon_for(name, is_dir);

    // icon + space + name + optional '/'
    let mut res = String::with_capacity(indent + icon.len() + name.len() + 2);
    res.extend(iter::repeat_n(' ', indent));
    res.push_str(icon);
    res.push(' ');
    res.push_str(name);
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
        0..1_000 => format!("{size}  B"), // extra space added on purpose
        1_000..1_000_000 => format!("{:.2} KB", bytes / 1e3),
        1_000_000..1_000_000_000 => format!("{:.2} MB", bytes / 1e6),
        _ => format!("{:.2} GB", bytes / 1e9),
    }
}

/// Formats a timestamp as `YYYY-MM-DD HH:MM` (UTC). Returns "-" when the
/// platform doesn't provide the time or it can't be formatted.
fn format_time(time: Option<SystemTime>) -> String {
    const FMT: &[BorrowedFormatItem<'_>] =
        format_description!("[year]-[month]-[day] [hour]:[minute]");

    time.and_then(|t| OffsetDateTime::from(t).format(FMT).ok())
        .unwrap_or_else(|| "-".to_string())
}
