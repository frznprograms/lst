use std::{
    fs::FileType,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    time::SystemTime,
};

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    path: Option<PathBuf>,
}

// TODO: check references and lifetimes
pub struct Entry {
    name: Option<&str>,
    file_type: FileType,
    size: u64,
    last_modified: SystemTime, // not really needed for dirs
}

// TODO: refactor: create an Enum so that Dir and File can be treated differently;
// then maybe no need for excessive use of Option

fn main() {
    let cli = Cli::parse();

    let path = match cli.path {
        Some(path) => path,
        None => PathBuf::from("."), // if no path is given, path will be "."
    };

    println!("{:?}", path);
}

fn walk(p: &Path) -> Vec<Entry> {
    let mut lst: Vec<Entry> = Vec::new();
    let mut stack: Vec<&Path> = Vec::new();
    stack.push(p);

    'main: while !stack.is_empty() {
        if let Some(entry) = stack.pop() {
            if entry.is_file() {
                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Unable to get metadata for {:?}", entry.to_str());
                        continue 'main;
                    }
                };
                let last_modified: SystemTime = match metadata.modified() {
                    Ok(time) => time,
                    Err(e) => {
                        println!("Unable to get metadata for {:?}", entry.to_str());
                        continue 'main;
                    }
                };
                lst.push(Entry {
                    name: entry.to_str(),
                    file_type: metadata.file_type(),
                    size: metadata.size(),
                    last_modified,
                })
            }
        }
    }

    lst
}
