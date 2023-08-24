mod book;
mod io;
mod load;

use self::{
    book::{Book, BookBuilder, ChapterBuilder, SectionBuilder},
    load::{Load, TrackDef},
};
use error_stack::{IntoReport, Report, Result, ResultExt};
use io::{copy, create_dir_all, create_file, get_dir_content, read_to_string, write_all};
use std::{
    fmt,
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Track {
    pub name: String,
    pub modules: Vec<Module>,
}

impl Track {
    pub fn load_toml_def(path: impl AsRef<Path>) -> Result<Self, LoadTrackError> {
        let def = TrackDef::load(path.as_ref(), None).change_context(LoadTrackError)?;
        def.resolve().change_context(LoadTrackError)
    }

    pub fn render(
        &self,
        output_dir: impl AsRef<Path>,
        clear_output: bool,
    ) -> Result<(), LoadTrackError> {
        let output_dir = output_dir.as_ref();
        if output_dir.exists() {
            if clear_output {
                // remove output dir and contents
                fs::remove_dir_all(output_dir)
                    .into_report()
                    .change_context(LoadTrackError)?;
            } else {
                // Return error if output dir is not empty
                let None = fs::read_dir(output_dir).into_report().change_context(LoadTrackError)?.next() else {
                    return Err(Report::new(LoadTrackError).attach_printable("Output directory is not empty"));
                };
            }
        }
        // Ensure output dir exists
        create_dir_all(output_dir)?;

        // Render the modules in the track
        let mut book_builder = Book::builder(&self.name);
        for (module, index) in self.modules.iter().zip(1..) {
            module.render(&mut book_builder, index, output_dir)?;
        }

        // Build and render the exercise book
        let book = book_builder.build();
        book.render(output_dir).change_context(LoadTrackError)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub description: String,
    pub units: Vec<Unit>,
}

impl Module {
    fn render(
        &self,
        book_builder: &mut BookBuilder,
        index: i32,
        output_dir: impl AsRef<Path>,
    ) -> Result<(), LoadTrackError> {
        let module_tag = to_numbered_tag(&self.name, index);
        let module_out_dir = output_dir.as_ref().join(Path::new(&module_tag));
        create_dir_all(&module_out_dir)?;
        let mut chapter = book_builder.chapter(&self.name);

        for (unit, index) in self.units.iter().zip(1..) {
            unit.render(&mut chapter, index, &module_out_dir)?;
        }
        chapter.add();
        Ok(())
    }
}

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub template: PathBuf,
    pub topics: Vec<Topic>,
}

impl Unit {
    fn render(
        &self,
        chapter: &mut ChapterBuilder,
        index: i32,
        output_dir: impl AsRef<Path>,
    ) -> Result<(), LoadTrackError> {
        let mut section = chapter.section(&self.name);
        let unit_tag = to_numbered_tag(&self.name, index);
        let unit_out_dir = output_dir.as_ref().join(unit_tag);
        create_dir_all(&unit_out_dir)?;

        let exercise_out_dir = unit_out_dir.join("exercises");
        create_dir_all(&exercise_out_dir)?;

        let template_content = read_to_string(&self.template)?;
        let mut unit_content = String::new();
        let mut unit_objectives = String::new();
        let mut unit_summary = String::new();

        for topic in &self.topics {
            let (topic_content, topic_objectives, topic_summary) =
                topic.render(&mut section, &output_dir, &exercise_out_dir)?;
            unit_content += &topic_content;
            unit_objectives += &topic_objectives;
            unit_summary += &topic_summary;
        }

        let unit_content = template_content
            .replace("#[modmod:content]\n", &unit_content)
            .replace("#[modmod:objectives]", &unit_objectives)
            .replace("#[modmod:summary]", &unit_summary);
        let unit_slides_path = unit_out_dir.join("slides.md");
        let unit_slides_file = create_file(unit_slides_path)?;
        write_all(&unit_slides_file, unit_content)?;

        section.add();
        Ok(())
    }
}

#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub exercises: Vec<Exercise>,
    pub summary: Vec<String>,
    pub objectives: Vec<String>,
    pub content: PathBuf,
    pub further_reading: Vec<String>,
}

impl Topic {
    fn render(
        &self,
        section: &mut SectionBuilder,
        output_dir: impl AsRef<Path>,
        exercise_out_dir: impl AsRef<Path>,
    ) -> Result<(String, String, String), LoadTrackError> {
        let mut topic_content = String::from("---\n\n");
        let mut topic_objectives = String::new();
        let mut topic_summary = String::new();

        let topic_slides = read_to_string(&self.content)?;
        topic_content += topic_slides.trim();
        topic_content += "\n";

        for objective in &self.objectives {
            topic_objectives += &format!("- {}\n", objective.trim());
        }

        for summary_item in &self.summary {
            topic_summary += &format!("- {}\n", summary_item.trim());
        }

        for exercise in &self.exercises {
            exercise.render(section, &output_dir, &exercise_out_dir)?;
        }

        Ok((topic_content, topic_objectives, topic_summary))
    }
}

#[derive(Debug)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub description: PathBuf,
    pub includes: Vec<String>,
}

impl Exercise {
    fn render(
        &self,
        section: &mut SectionBuilder,
        output_dir: impl AsRef<Path>,
        exercise_out_dir: impl AsRef<Path>,
    ) -> Result<(), LoadTrackError> {
        let exercise_dir = output_dir.as_ref().join(&self.path);
        let exercise_tag = to_tag(self.name.clone());
        let exercise_out_dir = exercise_out_dir.as_ref().join(exercise_tag);

        section.subsection(
            &self.name,
            exercise_dir.join(&self.description),
            exercise_out_dir.strip_prefix(output_dir).unwrap().to_owned(),
        );

        let content = get_dir_content(&exercise_dir)?;

        // Create globset to match included files
        let mut globset = globset::GlobSetBuilder::new();
        for include in &self.includes {
            globset.add(
                globset::Glob::new(exercise_dir.join(include).to_str().unwrap())
                    .into_report()
                    .attach_printable_lazy(|| format!("Error parsing include glob '{include}'"))
                    .change_context(LoadTrackError)?,
            );
        }
        let globset = globset.build().unwrap();

        for included_file in content.files.iter().filter(|f| globset.is_match(f)) {
            let included_file_relative = Path::new(&included_file)
                .strip_prefix(&exercise_dir)
                .unwrap();
            let included_file_dest = exercise_out_dir.join(included_file_relative);
            let include_file_dest_dir = included_file_dest.parent().unwrap();
            create_dir_all(include_file_dest_dir)?;
            copy(included_file, included_file_dest)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct LoadTrackError;

impl fmt::Display for LoadTrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unable to load track")
    }
}

impl error_stack::Context for LoadTrackError {}

fn to_numbered_tag(s: &str, i: i32) -> String {
    to_tag(format!("{i}-{s}"))
}

fn to_tag(mut s: String) -> String {
    s.make_ascii_lowercase();
    let mut tag = String::new();
    let mut words = s.split_whitespace();

    match words.next() {
        Some(w) => tag.push_str(w),
        None => return s,
    }

    for word in words {
        tag.push('-');
        tag.push_str(word);
    }
    tag
}
