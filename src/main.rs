use std::{io, path::PathBuf};

use clap::Parser;
use lst::cli::Cli;
use lst::traverse::walk;
use lst::util::entries::Entry;
use tabled::{
    Table,
    settings::{
        Alignment, Color,
        object::{Columns, Rows},
        style::Style,
    },
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
    let table = render_table(&lst);
    println!("{table}");

    Ok(())
}

fn render_table(data: &[Entry]) -> Table {
    let mut table = Table::new(data);
    table.with(Style::rounded());
    table.modify(Columns::one(1), Alignment::center());
    table.modify(Columns::one(2), Alignment::right());
    // header row goes last so it overrides the column alignments above
    table.modify(Rows::one(0), Alignment::center());

    for (i, entry) in data.iter().enumerate() {
        if entry.is_dir() {
            table.modify(Rows::one(i + 1), Color::FG_CYAN);
        }
    }

    table
}
