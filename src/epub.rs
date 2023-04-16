use crate::{config::MetaConfig, error::Result, template::Template};
use regex::Regex;
use std::{fs::File, io::Write};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

pub struct Epub {
    pub meta: MetaConfig,
    pub sections: Vec<Section>,
}

// The mimetype file is a required component of an EPUB 3 file and must be the first file in the archive.
// It defines the media type of the EPUB file and must be stored uncompressed as ASCII and without any extra data.
// The mimetype string must be "application/epub+zip" and should not include any whitespace or newlines.
const MIMETYPE_CONTENTS: &[u8] = b"application/epub+zip";

const CONTAINER_TEMPLATE: &str = include_str!("../template/META-INF/container.xml");
const CONTENT_TEMPLATE: &str = include_str!("../template/OEBPS/content.opf");
const NAV_TEMPLATE: &str = include_str!("../template/OEBPS/nav.xhtml");
const CHAPTER_TEMPLATE: &str = include_str!("../template/OEBPS/chapter.xhtml");

impl Epub {
    pub fn new(meta: MetaConfig) -> Self {
        Self {
            meta,
            sections: vec![],
        }
    }

    pub fn generate(&self) -> Result<()> {
        let mut zip_file = File::create(format!("{}.epub", self.meta.title.clone()))?;
        let mut zip = ZipWriter::new(&mut zip_file);

        let options = FileOptions::default();

        // Write mimetype file at the root
        zip.start_file(
            "mimetype",
            FileOptions::default().compression_method(CompressionMethod::Stored),
        )?;
        zip.write_all(MIMETYPE_CONTENTS)?;

        // Add container.xml file
        zip.start_file("META-INF/container.xml", options)?;
        zip.write_all(CONTAINER_TEMPLATE.as_bytes())?;

        // Add content.opf file
        zip.start_file("OEBPS/content.opf", options)?;
        let content = Template::load(CONTENT_TEMPLATE)
            .set("generator", "Vellum 0.1")
            .set(
                "manifest_items",
                self.sections
                    .iter()
                    .flat_map(Section::manifest_items)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .set(
                "spine_items",
                self.sections
                    .iter()
                    .flat_map(Section::spine_items)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .render();
        zip.write_all(content.as_bytes())?;

        // Add nav.xhtml file
        zip.start_file("OEBPS/nav.xhtml", options)?;
        let nav = Template::load(NAV_TEMPLATE)
            .set(
                "nav_items",
                self.sections
                    .iter()
                    .flat_map(Section::nav_items)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .render();
        zip.write_all(nav.as_bytes())?;

        // Add chapters files
        for section in &self.sections {
            for chapter in &section.chapters {
                let slug = chapter.slug();
                zip.start_file(format!("OEBPS/chapter-{slug}.xhtml"), options)?;

                let content = Template::load(CHAPTER_TEMPLATE)
                    .set("chapter_title", &chapter.title)
                    .set("chapter_content", &chapter.content)
                    .render();
                zip.write_all(content.as_bytes())?;
            }
        }

        // Finish writing and save the zip on the disk
        zip.finish()?;
        Ok(())
    }
}

pub struct Section {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

impl Section {
    pub fn manifest_items(&self) -> Vec<String> {
        self.chapters.iter().map(Chapter::manifest_item).collect()
    }

    pub fn spine_items(&self) -> Vec<String> {
        self.chapters.iter().map(Chapter::spine_item).collect()
    }

    pub fn nav_items(&self) -> Vec<String> {
        self.chapters.iter().map(Chapter::nav_item).collect()
    }

    pub fn slug(&self) -> String {
        let slug_regex = Regex::new(r#"[^\w]+"#).unwrap();
        slug_regex
            .replace_all(&self.title.to_ascii_lowercase(), "-")
            .to_string()
    }
}

pub struct Chapter {
    pub title: String,
    pub content: String, // path to the XHTML file for the chapter
}

impl Chapter {
    pub fn manifest_item(&self) -> String {
        let slug = self.slug();
        format!("<item id=\"chapter-{slug}\" href=\"chapter-{slug}.xhtml\" media-type=\"application/xhtml+xml\"/>")
    }

    pub fn spine_item(&self) -> String {
        let slug = self.slug();
        format!("<itemref idref=\"chapter-{slug}\"/>")
    }

    pub fn nav_item(&self) -> String {
        let slug = self.slug();
        let title = &self.title;
        format!("<li><a href=\"chapter-{slug}.xhtml\">{title}</a></li>")
    }

    pub fn slug(&self) -> String {
        let slug_regex = Regex::new(r#"[^\w]+"#).unwrap();
        slug_regex
            .replace_all(&self.title.to_ascii_lowercase(), "-")
            .to_string()
    }
}
