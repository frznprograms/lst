use std::cmp::Reverse;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

use crate::util::ignore::IgnoreRules;

/// Helper fn. Reads all children from a directory, skipping any that
/// `ignore_rules` says to ignore. Sorts the entries in alphabetical order
/// by default.
pub(crate) fn read_children(dir: &Path, ignore_rules: &IgnoreRules) -> io::Result<Vec<DirEntry>> {
    let mut children = Vec::new();
    for child in dir.read_dir()? {
        let child = match child {
            Ok(c) => c,
            Err(e) => {
                eprintln!("unable to read an entry in {}: {e}", dir.display());
                continue;
            }
        };
        // file_type() usually comes free with the directory read, unlike path().is_dir()
        let is_dir = child.file_type().is_ok_and(|t| t.is_dir());
        if ignore_rules.is_ignored(&child.path(), is_dir) {
            continue;
        }
        children.push(child);
    }
    children.sort_by_cached_key(|e| Reverse(e.file_name()));
    Ok(children)
}
