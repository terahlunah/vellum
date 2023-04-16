use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The toml book file
    #[arg(short, long, value_name = "FILE")]
    pub book: PathBuf,

    /// The name of the created epub file
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
}
