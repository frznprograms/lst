use std::{io, path::PathBuf};

use clap::Parser;
use lst::cli::Cli;
use lst::traverse::walk;
use lst::util::entries::Entry;
use lst::util::ignore::IgnoreOptions;
use tabled::{
    Table,
    settings::{
        Alignment, Color,
        object::{Columns, Rows},
        style::Style,
    },
};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // default path will always be cwd
    let path = cli.path.unwrap_or_else(|| PathBuf::from("."));
    let depth = cli.depth;
    dbg!(&path);

    let opts = IgnoreOptions {
        show_hidden: cli.all,
        no_ignore: cli.no_ignore,
        patterns: cli.ignore,
    };

    let lst = walk(&path, depth, &opts)?;
    // let lst = walk(&path, Some(1), &opts)?;

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
