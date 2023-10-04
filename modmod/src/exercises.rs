use std::{fmt, path::{PathBuf, Path}};

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct RenderExercisesError;

impl fmt::Display for RenderExercisesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unable to render exercises")
    }
}

impl error_stack::Context for RenderExercisesError {}

pub struct ExerciseCollection {}

impl ExerciseCollection {
    pub fn builder() -> ExerciseCollectionBuilder {
        todo!()
    }

    pub fn render(&self, output_dir: impl AsRef<Path>) -> Result<(), RenderExercisesError> {
        todo!();
    }
}

pub struct UnitExercises {}

pub struct Exercise {
    name: String,
    path: PathBuf,
    description: PathBuf,
    includes: Vec<String>,
}


pub struct ExerciseCollectionBuilder {
    units: Vec<UnitExercises>,
}

