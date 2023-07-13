use indoc::indoc;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::to_tag;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub chapters: Vec<Chapter>,
}

impl Book {
    pub fn builder(title: &str) -> BookBuilder {
        BookBuilder {
            title: title.to_string(),
            chapters: vec![],
        }
    }

    pub fn render(&self, root_dir: impl AsRef<Path>) -> crate::Result<()> {
        let book_root_dir = root_dir.as_ref().join("book");
        let book_src_dir = book_root_dir.join("src");
        fs::create_dir_all(&book_src_dir)?;

        let mut book_toml = File::create(&book_root_dir.join("book.toml"))?;
        write!(
            book_toml,
            indoc! {r#"
            [book]
            language = "en"
            multilingual = false
            
            [build]
            build-dir = "./target"
        "#}
        )?;

        let mut summary_md = File::create(&book_src_dir.join("SUMMARY.md"))?;

        writeln!(summary_md, "# Summary")?;
        writeln!(summary_md)?;
        for (chapter, chapter_i) in self.chapters.iter().zip(1..) {
            writeln!(summary_md, "# {chapter_i} - {}", chapter.title)?;

            for (section, section_i) in chapter.sections.iter().zip(1..) {
                let section_file_name =
                    Path::new(&to_tag(section.title.clone())).with_extension("md");
                writeln!(
                    summary_md,
                    "- [{section_i} - {}]({})",
                    section.title,
                    section_file_name.to_str().unwrap()
                )?;

                let mut section_file = File::create(book_src_dir.join(&section_file_name))?;

                writeln!(
                    section_file,
                    "# Unit {chapter_i}.{section_i} - {}",
                    section.title
                )?;
                writeln!(section_file)?;
                for (subsection, subsection_i) in section.subsections.iter().zip(1..) {
                    writeln!(
                        section_file,
                        "## Exercise {chapter_i}.{section_i}.{subsection_i}: {}",
                        subsection.title
                    )?;
                    writeln!(section_file)?;
                    let content = fs::read_to_string(&subsection.content)?;

                    write!(section_file, "{content}")?;
                }
            }
            writeln!(summary_md)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Chapter {
    pub title: String,
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub struct Section {
    pub title: String,
    pub subsections: Vec<SubSection>,
}

#[derive(Debug)]
pub struct SubSection {
    pub title: String,
    pub content: PathBuf,
}

pub struct BookBuilder {
    title: String,
    chapters: Vec<Chapter>,
}

impl BookBuilder {
    pub fn chapter(&mut self, title: &str) -> ChapterBuilder {
        ChapterBuilder {
            book_builder: self,
            title: title.to_string(),
            sections: vec![],
        }
    }

    pub fn build(self) -> Book {
        let BookBuilder { title, chapters } = self;
        Book { title, chapters }
    }
}

pub struct ChapterBuilder<'bb> {
    book_builder: &'bb mut BookBuilder,
    title: String,
    sections: Vec<Section>,
}

impl<'bb> ChapterBuilder<'bb> {
    pub fn add(self) -> &'bb mut BookBuilder {
        let ChapterBuilder {
            book_builder,
            title,
            sections,
        } = self;
        book_builder.chapters.push(Chapter { title, sections });
        book_builder
    }

    pub fn section<'cb>(&'cb mut self, title: &str) -> SectionBuilder<'bb, 'cb> {
        SectionBuilder {
            chapter_builder: self,
            title: title.to_string(),
            subsections: vec![],
        }
    }
}

pub struct SectionBuilder<'bb, 'cb> {
    chapter_builder: &'cb mut ChapterBuilder<'bb>,
    title: String,
    subsections: Vec<SubSection>,
}

impl<'bb, 'cb> SectionBuilder<'bb, 'cb> {
    pub fn add(self) -> &'cb mut ChapterBuilder<'bb> {
        let SectionBuilder {
            chapter_builder,
            title,
            subsections,
        } = self;
        chapter_builder
            .sections
            .push(Section { title, subsections });
        chapter_builder
    }

    pub fn subsection(&mut self, title: &str, content: PathBuf) {
        self.subsections.push(SubSection {
            title: title.to_string(),
            content,
        });
    }
}
