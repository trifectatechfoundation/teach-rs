use indoc::indoc;
use std::{
    fmt,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use error_stack::{IntoReport, Result, ResultExt};

use crate::to_tag;

#[derive(Debug)]
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
        fs::create_dir_all(&book_src_dir)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to create book output directory at path {path}",
                    path = book_src_dir.to_string_lossy()
                )
            })
            .change_context(RenderBookError)?;
        let book_toml_path = book_out_dir.join("book.toml");
        let mut book_toml = File::create(&book_toml_path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to create book.toml file at {path}",
                    path = book_toml_path.to_string_lossy()
                )
            })
            .change_context(RenderBookError)?;
        write!(
            book_toml,
            indoc! {r#"
            [book]
            language = "en"
            multilingual = false
            
            [build]
            build-dir = "./target"
        "#}
        )
        .into_report()
        .attach_printable_lazy(|| {
            format!(
                "Unable to write into the book.toml file created at {path}",
                path = book_toml_path.to_string_lossy()
            )
        })
        .change_context(RenderBookError)?;

        let summary_md_path = book_src_dir.join("SUMMARY.md");
        let mut summary_md = File::create(&summary_md_path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to write into SUMMARY.md file at {path}",
                    path = summary_md_path.to_string_lossy()
                )
            })
            .change_context(RenderBookError)?;

        writeln!(summary_md, "# Summary")
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to write into SUMMARY.md file at {path}",
                    path = summary_md_path.to_string_lossy()
                )
            })
            .change_context(RenderBookError)?;
        writeln!(summary_md)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to write into SUMMARY.md file at {path}",
                    path = summary_md_path.to_string_lossy()
                )
            })
            .change_context(RenderBookError)?;
        for (chapter, chapter_i) in self.chapters.iter().zip(1..) {
            writeln!(summary_md, "# {chapter_i} - {}", chapter.title)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Unable to write into SUMMARY.md file at {path}",
                        path = summary_md_path.to_string_lossy()
                    )
                })
                .change_context(RenderBookError)?;

            for (section, section_i) in chapter.sections.iter().zip(1..) {
                let section_file_name =
                    Path::new(&to_tag(section.title.clone())).with_extension("md");
                writeln!(
                    summary_md,
                    "- [{section_i} - {}]({})",
                    section.title,
                    section_file_name.to_str().unwrap()
                )
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Unable to write into SUMMARY.md file at {path}",
                        path = summary_md_path.to_string_lossy()
                    )
                })
                .change_context(RenderBookError)?;

                let section_file_path = book_src_dir.join(&section_file_name);
                let mut section_file = File::create(&section_file_path)
                    .into_report()
                    .attach_printable_lazy(|| {
                        format!(
                            "Unable to write into section markdown file at {path}",
                            path = section_file_path.to_string_lossy()
                        )
                    })
                    .change_context(RenderBookError)?;

                writeln!(
                    section_file,
                    "# Unit {chapter_i}.{section_i} - {}",
                    section.title
                )
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Unable to write into section markdown file at {path}",
                        path = section_file_path.to_string_lossy()
                    )
                })
                .change_context(RenderBookError)?;
                writeln!(section_file)
                    .into_report()
                    .attach_printable_lazy(|| {
                        format!(
                            "Unable to write into section markdown file at {path}",
                            path = section_file_path.to_string_lossy()
                        )
                    })
                    .change_context(RenderBookError)?;
                for (subsection, subsection_i) in section.subsections.iter().zip(1..) {
                    writeln!(
                        section_file,
                        "## Exercise {chapter_i}.{section_i}.{subsection_i}: {}",
                        subsection.title
                    )
                    .into_report()
                    .attach_printable_lazy(|| {
                        format!(
                            "Unable to write into section markdown file at {path}",
                            path = section_file_path.to_string_lossy()
                        )
                    })
                    .change_context(RenderBookError)?;
                    writeln!(section_file)
                        .into_report()
                        .attach_printable_lazy(|| {
                            format!(
                                "Unable to write into section markdown file at {path}",
                                path = section_file_path.to_string_lossy()
                            )
                        })
                        .change_context(RenderBookError)?;
                    let content = fs::read_to_string(&subsection.content)
                        .into_report()
                        .attach_printable_lazy(|| {
                            format!(
                                "Unable read subsection content from path {path}",
                                path = subsection.content.to_string_lossy()
                            )
                        })
                        .change_context(RenderBookError)?;
                    writeln!(section_file, "{content}")
                        .into_report()
                        .attach_printable_lazy(|| {
                            format!(
                                "Unable to write into section markdown file at {path}",
                                path = section_file_path.to_string_lossy()
                            )
                        })
                        .change_context(RenderBookError)?;
                }
            }
            writeln!(summary_md)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Unable to write into SUMMARY.md file at {path}",
                        path = summary_md_path.to_string_lossy()
                    )
                })
                .change_context(RenderBookError)?;
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
