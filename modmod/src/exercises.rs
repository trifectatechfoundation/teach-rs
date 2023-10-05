use std::{fmt, path::Path};

use error_stack::Result;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct RenderExercisesError;

impl fmt::Display for RenderExercisesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unable to render exercises")
    }
}

impl error_stack::Context for RenderExercisesError {}

#[derive(Debug)]
pub struct ExerciseCollection<'track> {
    modules: Vec<ModuleExercises<'track>>,
}

impl<'track> ExerciseCollection<'track> {
    pub fn builder() -> ExerciseCollectionBuilder<'track> {
        ExerciseCollectionBuilder {
            collection: ExerciseCollection { modules: vec![] },
        }
    }

    pub fn render(&self, output_dir: impl AsRef<Path>) -> Result<(), RenderExercisesError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct ModuleExercises<'track> {
    name: &'track str,
    unit_exercises: Vec<UnitExercises<'track>>,
}

#[derive(Debug)]
pub struct UnitExercises<'track> {
    name: &'track str,
    exercises: Vec<ExercisePackage<'track>>,
}

#[derive(Debug)]
pub struct ExercisePackage<'track> {
    name: &'track str,
    path: &'track Path,
    includes: &'track [String],
}

pub struct ExerciseCollectionBuilder<'track> {
    collection: ExerciseCollection<'track>,
}

impl<'track> ExerciseCollectionBuilder<'track> {
    pub fn module(&mut self, name: &'track str) -> ModuleExercisesBuilder<'track, '_> {
        ModuleExercisesBuilder {
            collection_buider: self,
            module_exercises: ModuleExercises {
                name,
                unit_exercises: vec![],
            },
        }
    }

    pub fn build(self) -> ExerciseCollection<'track> {
        self.collection
    }
}

pub struct ModuleExercisesBuilder<'track, 'c> {
    collection_buider: &'c mut ExerciseCollectionBuilder<'track>,
    module_exercises: ModuleExercises<'track>,
}

impl<'track, 'c> ModuleExercisesBuilder<'track, 'c> {
    pub fn unit<'m>(&'m mut self, name: &'track str) -> UnitExercisesBuilder<'track, 'c, 'm> {
        UnitExercisesBuilder {
            module_builder: self,
            unit_exercises: UnitExercises {
                name,
                exercises: vec![],
            },
        }
    }

    pub fn add(self) -> &'c mut ExerciseCollectionBuilder<'track> {
        self.collection_buider
            .collection
            .modules
            .push(self.module_exercises);
        self.collection_buider
    }
}

pub struct UnitExercisesBuilder<'track, 'c, 'm> {
    module_builder: &'m mut ModuleExercisesBuilder<'track, 'c>,
    unit_exercises: UnitExercises<'track>,
}

impl<'track, 'c, 'm> UnitExercisesBuilder<'track, 'c, 'm> {
    pub fn package(&mut self, name: &'track str, path: &'track Path, includes: &'track [String]) {
        self.unit_exercises.exercises.push(ExercisePackage {
            name,
            path,
            includes,
        })
    }

    pub fn add(self) -> &'m mut ModuleExercisesBuilder<'track, 'c> {
        self.module_builder
            .module_exercises
            .unit_exercises
            .push(self.unit_exercises);
        self.module_builder
    }
}
