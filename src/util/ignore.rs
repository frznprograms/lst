use ignore::overrides::{Override, OverrideBuilder};
use std::path::Path;

/// What the user has asked to leave in or out of the listing. The traversal
/// (`ignore::WalkBuilder`) does the actual `.gitignore` handling, including
/// nested and parent `.gitignore` files; this only carries the choices.
#[derive(Debug, Default)]
pub struct IgnoreOptions {
    /// Include dotfiles, like `ls -a`.
    pub show_hidden: bool,
    /// Don't apply `.gitignore` (and similar) rules.
    pub no_ignore: bool,
    /// Extra gitignore-style patterns to leave out, applied on top of the rules above.
    pub patterns: Vec<String>,
}

impl IgnoreOptions {
    /// Turns the extra patterns into an `Override` rooted at `root`. Bad
    /// patterns are reported and skipped, never fatal.
    pub fn overrides(&self, root: &Path) -> Override {
        let mut builder = OverrideBuilder::new(root);
        for pattern in &self.patterns {
            // overrides are inverted compared to .gitignore: a leading `!` means "ignore this"
            if let Err(e) = builder.add(&format!("!{pattern}")) {
                eprintln!("invalid ignore pattern {pattern:?}: {e}");
            }
        }

        builder.build().unwrap_or_else(|e| {
            eprintln!("unable to build ignore patterns: {e}");
            Override::empty()
        })
    }
}
