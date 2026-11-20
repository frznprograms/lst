use std::io;
use std::path::Path;

use ignore::WalkBuilder;

use crate::util::entries::Entry;
use crate::util::ignore::IgnoreOptions;

/// Pre-order walk (a directory is listed before its contents, siblings sorted
/// by name) through a file directory.
/// When max_depth = Some(1), this functions as a plain `ls` command on CLI.
/// When max_depth = None, it traverses the entire file tree.
/// Dotfiles and anything matched by a `.gitignore` (nested ones, and ones in
/// parent directories of `root`) are skipped, as configured by `opts`.
/// Fails only if `root` itself cannot be read.
pub fn walk(
    root: &Path,
    max_depth: Option<usize>,
    opts: &IgnoreOptions,
) -> io::Result<Vec<Entry>> {
    // root failing to read is propagated to main; the walker would only
    // report it later as an item
    root.read_dir()?;

    let mut builder = WalkBuilder::new(root);
    builder
        .max_depth(max_depth)
        // .gitignore, .git/info/exclude, global ignore file, parent dirs
        .standard_filters(!opts.no_ignore)
        // set after standard_filters, which also toggles this
        .hidden(!opts.show_hidden)
        .overrides(opts.overrides(root))
        .sort_by_file_name(|a, b| a.cmp(b))
        // with -a, list `.git` itself but never descend into it
        .filter_entry(|e| {
            let in_dot_git = e
                .path()
                .parent()
                .and_then(Path::file_name)
                .is_some_and(|name| name == ".git");
            // depth 1 children of `root` are always kept, in case root is `.git`
            e.depth() <= 1 || !in_dot_git
        });

    let mut lst = Vec::new();
    for result in builder.build() {
        let dir_entry = match result {
            Ok(e) => e,
            Err(e) => {
                eprintln!("unable to read an entry under {}: {e}", root.display());
                continue;
            }
        };
        // the walker yields `root` itself first
        if dir_entry.depth() == 0 {
            continue;
        }

        match Entry::from_dir_entry(&dir_entry) {
            Ok(entry) => lst.push(entry),
            Err(e) => eprintln!(
                "unable to read metadata for {}: {e}",
                dir_entry.path().display()
            ),
        }
    }

    Ok(lst)
}
