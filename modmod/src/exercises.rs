use std::{
    fmt,
    path::{Path, PathBuf},
};

use error_stack::Result;

use crate::Exercise;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct RenderExercisesError;

impl fmt::Display for RenderExercisesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unable to render exercises")
    }
}

impl error_stack::Context for RenderExercisesError {}

pub struct ExerciseCollection {
    modules: Vec<ModuleExercises>,
}

impl ExerciseCollection {
    pub fn builder() -> ExerciseCollectionBuilder {
        ExerciseCollectionBuilder {
            collection: ExerciseCollection { modules: vec![] },
        }
    }

    pub fn render(&self, output_dir: impl AsRef<Path>) -> Result<(), RenderExercisesError> {
        todo!();
    }
}

pub struct ModuleExercises {
    name: String,
    unit_exercises: Vec<UnitExercises>,
}

pub struct UnitExercises {
    name: String,
    exercises: Vec<ExercisePackage>,
}

pub struct ExercisePackage {
    name: String,
    path: PathBuf,
    includes: Vec<String>,
}

pub struct ExerciseCollectionBuilder {
    collection: ExerciseCollection,
}

impl ExerciseCollectionBuilder {
    pub fn module(&mut self, name: &str) -> ModuleExercisesBuilder {
        ModuleExercisesBuilder {
            collection_buider: self,
            module_exercises: ModuleExercises {
                name: name.to_string(),
                unit_exercises: vec![],
            },
        }
    }

    pub fn build(self) -> ExerciseCollection {
        self.collection
    }
}

pub struct ModuleExercisesBuilder<'c> {
    collection_buider: &'c mut ExerciseCollectionBuilder,
    module_exercises: ModuleExercises,
}

impl<'c> ModuleExercisesBuilder<'c> {
    pub fn unit<'m>(&'m mut self, name: &str) -> UnitExercisesBuilder<'c, 'm> {
        UnitExercisesBuilder {
            module_builder: self,
            unit_exercises: UnitExercises {
                name: name.to_string(),
                exercises: vec![],
            },
        }
    }

    pub fn add(self) -> &'c mut ExerciseCollectionBuilder {
        self.collection_buider
            .collection
            .modules
            .push(self.module_exercises);
        self.collection_buider
    }
}

pub struct UnitExercisesBuilder<'c, 'm> {
    module_builder: &'m mut ModuleExercisesBuilder<'c>,
    unit_exercises: UnitExercises,
}

impl<'c, 'm> UnitExercisesBuilder<'c, 'm> {
    pub fn package(&mut self, name: &str, path: PathBuf, includes: Vec<String>) {
        self.unit_exercises.exercises.push(ExercisePackage {
            name: name.to_string(),
            path,
            includes,
        })
    }

    pub fn add(self) -> &'m mut ModuleExercisesBuilder<'c> {
        self.module_builder
            .module_exercises
            .unit_exercises
            .push(self.unit_exercises);
        self.module_builder
    }
}
