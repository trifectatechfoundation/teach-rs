use std::{
    fmt,
    fs::{self, File},
    path::Path,
};

use globset::{Glob, GlobSetBuilder};
use types::*;

mod book;
mod load;
mod types;

use crate::book::Book;

use error_stack::{IntoReport, Report, Result, ResultExt};

#[derive(Debug)]
pub struct RenderTrackError;

impl fmt::Display for RenderTrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unable to render track")
    }
}

impl error_stack::Context for RenderTrackError {}

pub fn render(
    path: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    clear_output: bool,
) -> Result<(), RenderTrackError> {
    use std::io::Write;
    let output_dir = output_dir.as_ref();
    if clear_output {
        if output_dir.exists() {
            fs::remove_dir_all(output_dir)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Error removing output dir: {path}",
                        path = output_dir.to_string_lossy()
                    )
                })
                .change_context(RenderTrackError)?;
        }
        fs::create_dir_all(output_dir)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Error creating output dir: {path}",
                    path = output_dir.to_string_lossy()
                )
            })
            .change_context(RenderTrackError)?;
    } else {
        if output_dir.exists() {
            let None = fs::read_dir(output_dir)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Unable to check if output dir is empty at path {path}",
                        path = output_dir.to_string_lossy()
                    )
                })
                .change_context(RenderTrackError)?
                .next() else {
                    return Err(Report::new(RenderTrackError).attach_printable(format!("Specified output dir is not empty: {path}", path = output_dir.to_string_lossy())));
            };
        } else {
            fs::create_dir_all(output_dir)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Error creating output dir: {path}",
                        path = output_dir.to_string_lossy()
                    )
                })
                .change_context(RenderTrackError)?;
        }
    }
    let track = Track::load(&path)
        .attach_printable_lazy(|| {
            format!(
                "Error loading track at path {path}",
                path = path.as_ref().to_string_lossy()
            )
        })
        .change_context(RenderTrackError)?;

    let mut book_builder = Book::builder(&track.data.name);

    let modules = track
        .load_modules()
        .attach_printable_lazy(|| format!("Error loading track modules"))
        .change_context(RenderTrackError)?;

    for (module, i_mod) in modules.iter().zip(1..) {
        let module_tag = to_numbered_tag(&module.data.name, i_mod);
        let module_out_dir = output_dir.join(Path::new(&module_tag));
        fs::create_dir(&module_out_dir)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Error creating module output dir at path {path}",
                    path = module_out_dir.to_string_lossy()
                )
            })
            .change_context(RenderTrackError)?;
        let mut chapter = book_builder.chapter(&module.data.name);

        let topics = module
            .load_topics()
            .attach_printable("could not load module topics")
            .change_context(RenderTrackError)?;
        for ((unit, topics), i_unit) in topics.iter().zip(1..) {
            let mut section = chapter.section(&unit.name);
            let unit_tag = to_numbered_tag(&unit.name, i_unit);
            let unit_out_dir = module_out_dir.join(unit_tag);
            fs::create_dir(&unit_out_dir)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Error creating unit output dir at path {path}",
                        path = unit_out_dir.to_string_lossy()
                    )
                })
                .change_context(RenderTrackError)?;
            let exercise_out_dir = unit_out_dir.join("exercises");
            fs::create_dir(&exercise_out_dir)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Error creating exercise output dir at path {path}",
                        path = exercise_out_dir.to_string_lossy()
                    )
                })
                .change_context(RenderTrackError)?;

            let template_path = module.path.parent().unwrap().join(&unit.template);
            let template = fs::read_to_string(&template_path)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Error reading template at path {path}",
                        path = template_path.to_string_lossy()
                    )
                })
                .change_context(RenderTrackError)?;

            let mut topic_content = String::new();
            let mut topic_objectives = String::new();
            let mut topic_summary = String::new();
            for topic in topics {
                let topic_slides_path = topic.path.parent().unwrap().join(&topic.data.content);
                let topic_slides = fs::read_to_string(&topic_slides_path)
                    .into_report()
                    .attach_printable_lazy(|| {
                        format!(
                            "Error reading topic slides at path {path}",
                            path = topic_slides_path.to_string_lossy()
                        )
                    })
                    .change_context(RenderTrackError)?;
                topic_content += "---\n\n";
                topic_content += topic_slides.trim();
                topic_content += "\n";

                for objective in &topic.data.objectives {
                    topic_objectives += &format!("- {}\n", objective.trim());
                }

                for item in &topic.data.summary {
                    topic_summary += &format!("- {}\n", item.trim());
                }

                for exercise in &topic.data.exercises {
                    let exercise_dir = topic.path.parent().unwrap().join(&exercise.path);
                    section.subsection(&exercise.name, exercise_dir.join(&exercise.description));
                    let content = fs_extra::dir::get_dir_content(&exercise_dir).unwrap();
                    let exercise_tag = to_tag(exercise.name.clone());
                    let mut globset = GlobSetBuilder::new();
                    for include in &exercise.includes {
                        globset.add(
                            Glob::new(exercise_dir.join(include).to_str().unwrap())
                                .into_report()
                                .attach_printable_lazy(|| {
                                    format!("Error parsing include glob '{include}'")
                                })
                                .change_context(RenderTrackError)?,
                        );
                    }
                    let globset = globset.build().unwrap();
                    for included_file in content.files.iter().filter(|f| globset.is_match(f)) {
                        let file_relative = Path::new(&included_file)
                            .strip_prefix(&exercise_dir)
                            .unwrap();
                        let dest = exercise_out_dir.join(&exercise_tag).join(file_relative);
                        let include_dest_dir = dest.parent().unwrap();
                        fs::create_dir_all(&include_dest_dir)
                            .into_report()
                            .attach_printable_lazy(|| {
                                format!(
                                    "Error creating include destination dir at path {path}",
                                    path = include_dest_dir.to_string_lossy()
                                )
                            })
                            .change_context(RenderTrackError)?;
                        fs::copy(&included_file, &dest)
                            .into_report()
                            .attach_printable_lazy(|| {
                                format!(
                                    "Error copying include file {included_file} to dest at path {dest}",
                                    dest = dest.to_string_lossy()
                                )
                            })
                            .change_context(RenderTrackError)?;
                    }
                }
            }
            section.add();

            let unit_content = template
                .replace("#[modmod:content]\n", &topic_content)
                .replace("#[modmod:objectives]", &topic_objectives)
                .replace("#[modmod:summary]", &topic_summary);
            let unit_slides_path = unit_out_dir.join("slides.md");
            let mut unit_slides = File::create(&unit_slides_path)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Error creating unit slides file at path {path}",
                        path = unit_slides_path.to_string_lossy()
                    )
                })
                .change_context(RenderTrackError)?;
            write!(unit_slides, "{unit_content}")
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Unable to write into the slides.md file created at {path}",
                        path = unit_slides_path.to_string_lossy()
                    )
                })
                .change_context(RenderTrackError)?;
        }
        chapter.add();
    }
    let book = book_builder.build();
    book.render(output_dir)
        .attach_printable("could not render book")
        .change_context(RenderTrackError)?;
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
