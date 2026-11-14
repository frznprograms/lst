use std::cmp::Reverse;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

/// Helper fn. Reads all children from a directory. Sorts the entries
/// in alphabetical order by default.
pub(crate) fn read_children(dir: &Path) -> io::Result<Vec<DirEntry>> {
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
