use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    pub path: Option<PathBuf>,

    /// Show hidden files (those starting with a dot), like `ls -a`
    #[arg(short, long)]
    pub all: bool,

    pub depth: Option<usize>,

    /// Don't apply .gitignore rules
    #[arg(long)]
    pub no_ignore: bool,

    /// Extra gitignore-style pattern to leave out of the listing, e.g. "*.toml" (repeatable)
    #[arg(long = "ignore", value_name = "PATTERN")]
    pub ignore: Vec<String>,
}
