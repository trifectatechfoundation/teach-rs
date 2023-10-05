mod book;
mod exercises;
mod io;
mod load;
mod slides;

use self::{
    book::{Book, BookBuilder, ChapterBuilder, SectionBuilder},
    load::{Load, TrackDef},
};
use error_stack::{IntoReport, Report, Result, ResultExt};
use exercises::{
    ExerciseCollection, ExerciseCollectionBuilder, ModuleExercisesBuilder, UnitExercisesBuilder,
};
use io::{copy, create_dir_all, create_file, get_dir_content, read_to_string, write_all};
use slides::{
    SlideDeck, SlideDeckBuilder, SlidesPackage, SlidesPackageBuilder, SlidesSectionBuilder,
};
use std::{
    fmt, fs,
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
                let None = fs::read_dir(output_dir)
                    .into_report()
                    .change_context(LoadTrackError)?
                    .next()
                else {
                    return Err(Report::new(LoadTrackError)
                        .attach_printable("Output directory is not empty"));
                };
            }
        }
        // Ensure output dir exists
        create_dir_all(output_dir)?;

        // Render the modules in the track
        let mut book_builder = Book::builder(&self.name);
        let mut slides_builder = SlidesPackage::builder(&self.name);
        let mut exercises_builder = ExerciseCollection::builder();
        for (module, index) in self.modules.iter().zip(1..) {
            module.render(
                &mut book_builder,
                &mut slides_builder,
                &mut exercises_builder,
                index,
                output_dir,
            )?;
        }

        // Build and render the slides package
        let slides_package = slides_builder.build();
        slides_package
            .render(output_dir)
            .change_context(LoadTrackError)?;

        // Build and render the exercise book
        let book = book_builder.build();
        book.render(output_dir).change_context(LoadTrackError)?;

        // Build and render exercise packages
        let exercises = exercises_builder.build();
        exercises
            .render(output_dir)
            .change_context(LoadTrackError)?;

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
        slides: &mut SlidesPackageBuilder,
        exercises: &mut ExerciseCollectionBuilder,
        index: i32,
        output_dir: impl AsRef<Path>,
    ) -> Result<(), LoadTrackError> {
        let module_tag = to_numbered_tag(&self.name, index);
        let module_out_dir = output_dir.as_ref().join(Path::new(&module_tag));
        create_dir_all(&module_out_dir)?;

        let mut chapter = book_builder.chapter(&self.name);
        let mut module_exercises = exercises.module(&self.name);

        // Render all units in this module
        for (unit, index) in self.units.iter().zip(1..) {
            unit.render(
                &mut chapter,
                slides,
                &mut module_exercises,
                index,
                &module_out_dir,
            )?;
        }
        chapter.add();
        module_exercises.add();
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
        slides: &mut SlidesPackageBuilder,
        module_exercises: &mut ModuleExercisesBuilder,
        index: i32,
        output_dir: impl AsRef<Path>,
    ) -> Result<(), LoadTrackError> {
        let mut section = chapter.section(&self.name);
        let mut deck = slides.deck(&self.name, self.template.clone());
        let mut unit_exercises = module_exercises.unit(&self.name);

        let unit_tag = to_numbered_tag(&self.name, index);
        let unit_out_dir = output_dir.as_ref().join(unit_tag);
        create_dir_all(&unit_out_dir)?;

        let exercise_out_dir = unit_out_dir.join("exercises");
        create_dir_all(&exercise_out_dir)?;

        for topic in self.topics.iter() {
            topic.render(
                &mut section,
                &mut deck,
                &mut unit_exercises,
                unit_out_dir.clone(),
                exercise_out_dir.clone(),
            )?;
        }

        section.add();
        deck.add();
        unit_exercises.add();

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
        deck: &mut SlideDeckBuilder,
        unit_exercises: &mut UnitExercisesBuilder,
        output_dir: impl AsRef<Path>,
        exercise_out_dir: impl AsRef<Path>,
    ) -> Result<(), LoadTrackError> {
        let slides_section = deck.section(self.content.clone());

        for exercise in &self.exercises {
            exercise.render(section, unit_exercises, &output_dir, &exercise_out_dir)?;
        }

        slides_section.add();

        Ok(())
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
        unit_exercises: &mut UnitExercisesBuilder,
        output_dir: impl AsRef<Path>,
        exercise_out_dir: impl AsRef<Path>,
    ) -> Result<(), LoadTrackError> {
        let exercise_dir = output_dir.as_ref().join(&self.path);
        let exercise_tag = to_tag(self.name.clone());
        let exercise_out_dir = exercise_out_dir.as_ref().join(exercise_tag);

        section.subsection(
            &self.name,
            exercise_dir.join(&self.description),
            exercise_out_dir
                .strip_prefix(output_dir)
                .unwrap()
                .to_owned(),
        );

        unit_exercises.package(&self.name, self.path.clone(), self.includes.clone());

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
