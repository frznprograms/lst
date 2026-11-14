use std::{io, path::PathBuf};

use clap::Parser;
use lst::cli::Cli;
use lst::traverse::walk;
use tabled::{Table, settings::style::Style};

// TODO: account for files that should be ignored
// TODO: add indent for different levels in table
// TODO: add forward slash for directories in table

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // default path will always be cwd
    let path = cli.path.unwrap_or_else(|| PathBuf::from("."));
    println!("{:?}", path);
    println!();

    // TODO: should be driven by cli flag in future
    let lst = walk(&path, None)?;
    // let lst = walk(&path, Some(1))?;

    for entry in &lst {
        let indent: String = " ".repeat(entry.depth() - 1);
        let suffix: &str = if entry.is_dir() { "/" } else { "" };
        println!("{}{}{}", indent, entry.name().to_string_lossy(), suffix);
    }
    println!("---");

    let mut table = Table::new(&lst);
    table.with(Style::rounded());
    println!("{table}");

    Ok(())
}
