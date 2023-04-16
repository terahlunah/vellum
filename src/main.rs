use crate::cli::Cli;
use std::{fs::File, io::Read};
use vellum::{
    config::BookConfig,
    epub::{Chapter, Epub, Section},
    error::Result,
};
pub mod cli;
use clap::Parser;

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    let mut config_file = File::open(cli.book)?;
    let mut config = String::new();
    config_file.read_to_string(&mut config)?;

    let BookConfig { meta, .. } = toml::from_str(&config)?;

    let mut epub = Epub::new(meta);

    epub.sections = vec![
        Section {
            title: "Part I".to_string(),
            chapters: vec![
                Chapter {
                    title: "Chapter 1".to_string(),
                    content: "OEBPS/chapter1.xhtml".to_string(),
                },
                Chapter {
                    title: "Chapter 2".to_string(),
                    content: "OEBPS/chapter2.xhtml".to_string(),
                },
            ],
        },
        Section {
            title: "Part II".to_string(),
            chapters: vec![
                Chapter {
                    title: "Chapter 3".to_string(),
                    content: "OEBPS/chapter3.xhtml".to_string(),
                },
                Chapter {
                    title: "Chapter 4".to_string(),
                    content: "OEBPS/chapter4.xhtml".to_string(),
                },
            ],
        },
    ];
    epub.generate()?;

    Ok(())
}
