use std::{
    fs::{self, File},
    path::Path,
};

use globset::{Glob, GlobSetBuilder};
use types::*;

mod book;
pub mod error;
mod load;
mod types;
pub use error::{Error, OutputError, Result};

use crate::book::Book;

pub fn render(
    path: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    clear_output: bool,
) -> Result<()> {
    use std::io::Write;
    let output_dir = output_dir.as_ref();

    if clear_output {
        if output_dir.exists() {
            println!("{}{}", file!(), line!());
            fs::remove_dir_all(output_dir)?;
        }
        fs::create_dir_all(output_dir)?;
    } else {
        println!("{}{}", file!(), line!());
        if output_dir.exists() {
            let None = fs::read_dir(output_dir)?.next() else {
            return Err(OutputError::NotEmpty.into());
        };
        } else {
            fs::create_dir_all(output_dir)?;
        }
    }
    let track = Track::load(path)?;

    let mut book_builder = Book::builder(&track.data.name);

    for (module, i_mod) in track.load_modules()?.iter().zip(1..) {
        let module_tag = to_numbered_tag(&module.data.name, i_mod);
        let module_out_dir = output_dir.join(Path::new(&module_tag));
        fs::create_dir(&module_out_dir)?;
        let mut chapter = book_builder.chapter(&module.data.name);

        for ((unit, topics), i_unit) in module.load_topics()?.iter().zip(1..) {
            let mut section = chapter.section(&unit.name);
            let unit_tag = to_numbered_tag(&unit.name, i_unit);
            let unit_out_dir = module_out_dir.join(unit_tag);
            fs::create_dir(&unit_out_dir)?;
            let exercise_out_dir = unit_out_dir.join("exercises");
            fs::create_dir(&exercise_out_dir)?;

            let template = fs::read_to_string(module.path.parent().unwrap().join(&unit.template))?;

            let mut topic_content = String::new();
            let mut topic_objectives = String::new();
            let mut topic_summary = String::new();
            for topic in topics {
                let topic_slides =
                    fs::read_to_string(topic.path.parent().unwrap().join(&topic.data.content))?;
                topic_content += "---\n\n";
                topic_content += topic_slides.trim();
                topic_content += "\n";

                for objective in &topic.data.objectives {
                    topic_objectives += &format!("- {}\n", objective.trim());
                }

                for item in &topic.data.summary {
                    topic_summary += &format!("- {}\n", item.trim());
                }

                for (exercise, i_exercise) in topic.data.exercises.iter().zip(1..) {
                    let exercise_dir = topic.path.parent().unwrap().join(&exercise.path);
                    section.subsection(&exercise.name, exercise_dir.join(&exercise.description));
                    let content = fs_extra::dir::get_dir_content(&exercise_dir).unwrap();
                    let exercise_tag = to_numbered_tag(&exercise.name, i_exercise);
                    let mut globset = GlobSetBuilder::new();
                    for include in &exercise.includes {
                        globset
                            .add(Glob::new(exercise_dir.join(include).to_str().unwrap()).unwrap());
                    }
                    let globset = globset.build().unwrap();
                    for included_file in content.files.iter().filter(|f| globset.is_match(f)) {
                        let file_relative = Path::new(&included_file)
                            .strip_prefix(&exercise_dir)
                            .unwrap();
                        let dest = exercise_out_dir.join(&exercise_tag).join(file_relative);
                        fs::create_dir_all(dest.parent().unwrap())?;
                        fs::copy(included_file, dest)?;
                    }
                }
            }
            section.add();

            let unit_content = template
                .replace("#[modmod:content]\n", &topic_content)
                .replace("#[modmod:objectives]", &topic_objectives)
                .replace("#[modmod:summary]", &topic_summary);
            let mut unit_slides = File::create(unit_out_dir.join("slides.md"))?;
            write!(unit_slides, "{unit_content}")?;
        }
        chapter.add();
    }

    let book = dbg!(book_builder.build());
    book.render(output_dir)?;
    Ok(())
}

fn to_numbered_tag(s: &str, i: i32) -> String {
    to_tag(format!("{i}-{s}"))
}

fn to_tag(mut s: String) -> String {
    s.make_ascii_lowercase();
    let mut tag = String::new();
    let mut words = s.split_whitespace();
    tag.push_str(words.next().unwrap());
    for word in words {
        tag.push('-');
        tag.push_str(word);
    }
    tag
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::render;
    const TRACK_ENTRY: &str = "data/rust-intro.track.toml";
    const OUTPUT_DIR: &str = "output";

    #[test]
    fn render_track() {
        render(Path::new(TRACK_ENTRY), Path::new(OUTPUT_DIR), true).unwrap();
    }
}
