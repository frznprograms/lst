use std::{io, path::PathBuf};

use clap::Parser;
use lst::cli::Cli;
use lst::traverse::walk;
use tabled::{
    Table,
    settings::{Color, object::Rows, style::Style},
};

// TODO: account for files that should be ignored

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // default path will always be cwd
    let path = cli.path.unwrap_or_else(|| PathBuf::from("."));
    println!("{:?}", path);
    println!();

    // TODO: should be driven by cli flag in future
    let lst = walk(&path, None)?;
    // let lst = walk(&path, Some(1))?;
    println!("---");

    let mut table = Table::new(&lst);
    table.with(Style::rounded());

    for (i, entry) in lst.iter().enumerate() {
        if entry.is_dir() {
            table.modify(Rows::one(i + 1), Color::FG_CYAN);
        }
    }
    println!("{table}");

    Ok(())
}
