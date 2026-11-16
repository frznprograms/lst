use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

/// Decides which entries are left out of the listing: dotfiles (unless
/// `show_hidden`), plus anything matched by the root `.gitignore` or by extra
/// gitignore-style patterns.
pub struct IgnoreRules {
    matcher: Gitignore,
    ignore_hidden: bool,
}

// TODO: implement recursive ignore for nested .gitignore files

impl IgnoreRules {
    /// Builds the rules for a listing rooted at `root`. Only the `.gitignore`
    /// directly inside `root` is read. Bad patterns and unreadable files are
    /// reported and skipped, never fatal.
    pub fn new(root: &Path, show_hidden: bool, extra_patterns: &[String]) -> Self {
        let mut builder = GitignoreBuilder::new(root);

        // a missing .gitignore is normal, so only try to read one that exists
        let gitignore = root.join(".gitignore");
        if gitignore.is_file()
            && let Some(e) = builder.add(&gitignore)
        {
            eprintln!("unable to fully read {}: {e}", gitignore.display());
        }

        for pattern in extra_patterns {
            if let Err(e) = builder.add_line(None, pattern) {
                eprintln!("invalid ignore pattern {pattern:?}: {e}");
            }
        }

        let matcher = builder.build().unwrap_or_else(|e| {
            eprintln!("unable to build ignore rules: {e}");
            Gitignore::empty()
        });

        IgnoreRules {
            matcher,
            ignore_hidden: !show_hidden,
        }
    }

    /// Whether `path` should be skipped. `is_dir` matters because patterns
    /// like `target/` only match directories.
    pub fn is_ignored(&self, path: &Path, is_dir: bool) -> bool {
        let is_hidden = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with('.'));

        (self.ignore_hidden && is_hidden) || self.matcher.matched(path, is_dir).is_ignore()
    }
}
