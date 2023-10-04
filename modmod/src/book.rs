use indoc::indoc;
use std::{
    fmt,
    path::{Path, PathBuf},
};

use error_stack::Result;

use crate::{
    io::{create_dir_all, create_file, read_to_string, write_all, write_fmt},
    to_tag,
};

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RenderBookError;

impl fmt::Display for RenderBookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unable to render book")
    }
}

impl error_stack::Context for RenderBookError {}

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

    pub fn render(&self, out_dir: impl AsRef<Path>) -> Result<(), RenderBookError> {
        let book_out_dir = out_dir.as_ref().join("book");
        let book_src_dir = book_out_dir.join("src");
        create_dir_all(&book_src_dir)?;
        let book_toml_path = book_out_dir.join("book.toml");
        let mut book_toml = create_file(&book_toml_path)?;

        write_all(
            &mut book_toml,
            indoc! {r#"
                [book]
                language = "en"
                multilingual = false
                
                [build]
                build-dir = "./target"
            "#},
        )?;

        let summary_md_path = book_src_dir.join("SUMMARY.md");
        let summary_md = create_file(&summary_md_path)?;
        write_all(&summary_md, "# Summary\n\n")?;

        for (chapter, chapter_i) in self.chapters.iter().zip(1..) {
            // Sadly, at the time of writing, mdbook does not allow for custom section numbering.
            // Therefore, we insert a draft chapter to keep the section numbering in sync
            write_fmt(&summary_md, format_args!("- [{}]()\n", chapter.title))?;

            for (section, section_i) in chapter.sections.iter().zip(1..) {
                let section_file_name =
                    Path::new(&to_tag(section.title.clone())).with_extension("md");
                write_fmt(
                    &summary_md,
                    format_args!(
                        "\t- [{}]({})\n",
                        section.title,
                        section_file_name.to_str().unwrap()
                    ),
                )?;

                let section_file_path = book_src_dir.join(&section_file_name);

                let section_file = create_file(&section_file_path)?;
                write_fmt(
                    &section_file,
                    format_args!("# Unit {chapter_i}.{section_i} - {}\n\n", section.title),
                )?;

                for (subsection, subsection_i) in section.subsections.iter().zip(1..) {
                    write_fmt(
                        &section_file,
                        format_args!(
                            "## Exercise {chapter_i}.{section_i}.{subsection_i}: {}\n\n",
                            subsection.title
                        ),
                    )?;
                    let content = read_to_string(&subsection.content)?;
                    let content = content
                        // Insert exercise directory paths
                        .replace(
                            "#[modmod:exercise_dir]",
                            &subsection.out_dir.to_string_lossy(),
                        )
                        // Insert exercise references
                        .replace(
                            "#[modmod:exercise_ref]",
                            &format!("{chapter_i}.{section_i}.{subsection_i}"),
                        )
                        // Convert exercise sections into subsubsections
                        .replace("\n# ", "\n### ");
                    write_fmt(&section_file, format_args!("{}\n", content.trim()))?;
                }
            }
            write_all(&summary_md, "\n")?;
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
    pub out_dir: PathBuf,
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

    pub fn subsection(&mut self, title: &str, content: PathBuf, out_dir: PathBuf) {
        self.subsections.push(SubSection {
            title: title.to_string(),
            content,
            out_dir,
        });
    }
}
