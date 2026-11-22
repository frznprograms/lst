use std::{io, path::PathBuf, process::ExitCode};

use clap::Parser;
use lst::cli::Cli;
use lst::traverse::walk;
use lst::util::entries::Entry;
use lst::util::ignore::IgnoreOptions;
use tabled::{
    Table,
    builder::Builder,
    settings::{
        Alignment, Color,
        object::{Columns, Rows},
        style::Style,
    },
};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or_else(|| PathBuf::from("."));
    let opts = IgnoreOptions {
        show_hidden: cli.all,
        no_ignore: cli.no_ignore,
        patterns: cli.ignore,
    };

    match walk(&path, cli.depth, &opts) {
        Ok(entries) => {
            println!("{}", render_table(&entries, cli.time));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[lst] {}: {}", path.display(), describe(&e));
            ExitCode::FAILURE
        }
    }
}

/// Builds the rounded table: Type centred, Size right-aligned, directories in cyan.
fn render_table(data: &[Entry], include_times: bool) -> Table {
    let mut b = Builder::default();
    b.push_record(Entry::headers(include_times));
    for e in data {
        b.push_record(e.row(include_times));
    }

    let mut table = b.build();
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

/// The error's message without the " (os error N)" suffix std adds to OS
/// errors, e.g. "No such file or directory", matching `ls`.
fn describe(e: &io::Error) -> String {
    let mut msg = e.to_string();
    if let Some(i) = msg.rfind("(os error ") {
        msg.truncate(i);
    }

    msg
}
