use std::{io, path::PathBuf};

use lst::traverse::walk;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    path: Option<PathBuf>,
}

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
    Ok(())
}
