use std::fs::DirEntry;
use std::io;
use std::path::Path;

use crate::util::entries::Entry;
use crate::util::helpers::read_children;

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
