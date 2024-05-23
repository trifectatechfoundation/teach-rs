use std::ffi::OsStr;

use error_stack::IntoReport;
use modmod::load::ExerciseDef;

use super::imports::*;

#[derive(Debug, Clone, clap::Args)]
pub struct CreateExercise {
    topic: PathBuf,
    #[arg(
        short = 'i',
        long = "index",
        help = "The index of the exercise in the list of exercises for the given topic. Defaults to the last one"
    )]
    index: Option<usize>,
    name: String,
}

impl CreateExercise {
    pub fn create(self, common_args: &CommonArgs) -> Result<(), ModModError> {
        let mut topic = modmod::load::TopicDef::load(&self.topic, None)
            .change_context(ModModError::default())?
            .data;

        let index = self
            .index
            .unwrap_or(topic.exercises.len())
            .min(topic.exercises.len());

        let exercises_path = self.topic.parent().unwrap().join("exercises");
        exercises_path.create_dir_all()?;

        let exercise_crate_path = exercises_path
            .canonicalize()
            .unwrap()
            .join(self.name.to_lowercase());
        if common_args.force {
            fs_extra::dir::remove(&exercise_crate_path)
                .into_report()
                .change_context(ModModError::default())?;
        }

        let output = std::process::Command::new("cargo")
            .args([
                OsStr::new("new"),
                OsStr::new("--name"),
                OsStr::new(&self.name),
                OsStr::new("--bin"),
                exercise_crate_path.as_os_str(),
            ])
            .output()
            .into_report()
            .change_context(ModModError::default())?;

        if !output.status.success() {
            let e = ModModError::report()
                .attach_printable("`cargo new` command process exited unsuccessfully")
                .attach_printable(format!(
                    r#"Stdout: "{}""#,
                    String::from_utf8_lossy(&output.stdout)
                ))
                .attach_printable(format!(
                    r#"Stderr: "{}""#,
                    String::from_utf8_lossy(&output.stderr)
                ))
                .attach_printable(output.status);

            return Err(e);
        }

        topic.exercises.insert(
            index,
            ExerciseDef {
                name: self.name,
                path: exercise_crate_path
                    .strip_prefix(&self.topic.parent().unwrap().canonicalize().unwrap())
                    .unwrap()
                    .to_owned(),
                ..Default::default()
            },
        );

        topic.exercises.dedup_by(|lhs, rhs| lhs.path == rhs.path);

        self.topic
            .create_file()?
            .write_all(toml::to_string_pretty(&topic).unwrap())?;

        Ok(())
    }
}
