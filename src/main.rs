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

    let path = match cli.path {
        Some(path) => path,
        None => PathBuf::from("."), // if no path is given, path will be "."
    };

    println!("{:?}", path);

    let lst = walk(&path);
    for name in &lst {
        println!("{:?}", name)
    }
}
