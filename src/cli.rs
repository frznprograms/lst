use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    pub path: Option<PathBuf>,
}
