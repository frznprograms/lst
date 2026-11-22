use clap::Parser;
use std::path::PathBuf;

/// List a directory and its contents as a table.
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// Directory to list [default: current directory]
    pub path: Option<PathBuf>,

    /// Show hidden files (those starting with a dot), like `ls -a`
    #[arg(short, long)]
    pub all: bool,

    /// Add "Created" and "Last Modified" columns
    #[arg(short, long)]
    pub time: bool,

    /// How many levels deep to list (1 = only the directory's own contents) [default: no limit]
    #[arg(short, long, value_name = "N")]
    pub depth: Option<usize>,

    /// Don't apply .gitignore rules
    #[arg(long)]
    pub no_ignore: bool,

    /// Extra gitignore-style pattern to leave out of the listing, e.g. "*.toml" (repeatable)
    #[arg(long = "ignore", value_name = "PATTERN")]
    pub ignore: Vec<String>,
}
