use std::path::PathBuf;

use lst::traverse::walk;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    // default path will always be cwd
    let path = cli.path.unwrap_or_else(|| PathBuf::from("."));
    println!("{:?}", path);

    let lst = walk(&path);
    for entry in &lst {
        println!("{:?}", entry.name)
    }
}
