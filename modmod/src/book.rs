use indoc::indoc;
use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use error_stack::Result;

use crate::{
    io::{PathExt, WriteExt},
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
pub struct Book<'track> {
    pub title: &'track str,
    pub chapters: Vec<Chapter<'track>>,
}

impl<'track> Book<'track> {
    pub fn builder(title: &'track str) -> BookBuilder {
        BookBuilder {
            book: Book {
                title,
                chapters: vec![],
            },
        }
    }

    pub fn render(
        &self,
        exercise_paths: &HashMap<PathBuf, PathBuf>,
        out_dir: impl AsRef<Path>,
    ) -> Result<(), RenderBookError> {
        let book_out_dir = out_dir.as_ref().join("book");
        let book_src_dir = book_out_dir.join("src");
        book_src_dir.create_dir_all()?;

        let book_toml_path = book_out_dir.join("book.toml");
        let mut book_toml = book_toml_path.create_file()?;
        book_toml.write_all(indoc! {r#"
                [book]
                language = "en"
                multilingual = false
                
                [build]
                build-dir = "./target"
            "#})?;

        let summary_md_path = book_src_dir.join("SUMMARY.md");

        let mut summary_md = summary_md_path.create_file()?;
        summary_md.write_all("# Summary\n\n")?;

        for (chapter, chapter_i) in self.chapters.iter().zip(1..) {
            // Sadly, at the time of writing, mdbook does not allow for custom section numbering.
            // Therefore, we insert a draft chapter to keep the section numbering in sync
            summary_md.write_fmt(format_args!("- [{}]()\n", chapter.title))?;

            for (section, section_i) in chapter.sections.iter().zip(1..) {
                let section_file_name = Path::new(&to_tag(section.title)).with_extension("md");
                summary_md.write_fmt(format_args!(
                    "\t- [{}]({})\n",
                    section.title,
                    section_file_name.to_str().unwrap()
                ))?;

                let section_file_path = book_src_dir.join(&section_file_name);
                let mut section_file = section_file_path.create_file()?;
                section_file.write_fmt(format_args!(
                    "# Unit {chapter_i}.{section_i} - {}\n\n",
                    section.title
                ))?;

                for (subsection, subsection_i) in section.subsections.iter().zip(1..) {
                    section_file.write_fmt(format_args!(
                        "## Exercise {chapter_i}.{section_i}.{subsection_i}: {}\n\n",
                        subsection.title
                    ))?;
                    let exercise_out_dir = &exercise_paths[subsection.exercise_path];
                    let content = subsection.content.read_to_string()?;
                    let content = content
                        // Insert exercise directory paths
                        .replace(
                            "#[modmod:exercise_dir]",
                            &exercise_out_dir.to_string_lossy(),
                        )
                        // Insert exercise references
                        .replace(
                            "#[modmod:exercise_ref]",
                            &format!("{chapter_i}.{section_i}.{subsection_i}"),
                        )
                        // Convert exercise sections into subsubsections
                        .replace("\n# ", "\n### ");
                    section_file.write_fmt(format_args!("{}\n", content.trim()))?;
                }
            }
            summary_md.write_all("\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Chapter<'track> {
    pub title: &'track str,
    pub sections: Vec<Section<'track>>,
}

#[derive(Debug)]
pub struct Section<'track> {
    pub title: &'track str,
    pub subsections: Vec<SubSection<'track>>,
}

#[derive(Debug)]
pub struct SubSection<'track> {
    pub title: &'track str,
    pub content: &'track Path,
    pub exercise_path: &'track Path,
}

pub struct BookBuilder<'track> {
    book: Book<'track>,
}

impl<'track> BookBuilder<'track> {
    pub fn chapter<'b>(&'b mut self, title: &'track str) -> ChapterBuilder<'track, 'b> {
        ChapterBuilder {
            book_builder: self,
            chapter: Chapter {
                title,
                sections: vec![],
            },
        }
    }

    pub fn build(self) -> Book<'track> {
        self.book
    }
}

pub struct ChapterBuilder<'track, 'b> {
    book_builder: &'b mut BookBuilder<'track>,
    chapter: Chapter<'track>,
}

impl<'track, 'b> ChapterBuilder<'track, 'b> {
    pub fn section<'c>(&'c mut self, title: &'track str) -> SectionBuilder<'track, 'b, 'c> {
        SectionBuilder {
            chapter_builder: self,
            section: Section {
                title,
                subsections: vec![],
            },
        }
    }

    pub fn add(self) -> &'b mut BookBuilder<'track> {
        self.book_builder.book.chapters.push(self.chapter);
        self.book_builder
    }
}

pub struct SectionBuilder<'track, 'b, 'c> {
    chapter_builder: &'c mut ChapterBuilder<'track, 'b>,
    section: Section<'track>,
}

impl<'track, 'b, 'c> SectionBuilder<'track, 'b, 'c> {
    pub fn subsection(
        &mut self,
        title: &'track str,
        content: &'track Path,
        exercise_path: &'track Path,
    ) {
        self.section.subsections.push(SubSection {
            title,
            content,
            exercise_path,
        })
    }

    pub fn add(self) -> &'c mut ChapterBuilder<'track, 'b> {
        self.chapter_builder.chapter.sections.push(self.section);
        self.chapter_builder
    }
}
