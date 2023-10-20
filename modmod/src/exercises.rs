use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use error_stack::{IntoReport, Result, ResultExt};

use crate::{io::PathExt, to_prefixed_tag};

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
    module_exercises: Vec<ModuleExercises<'track>>,
}

impl<'track> ExerciseCollection<'track> {
    pub fn builder() -> ExerciseCollectionBuilder<'track> {
        ExerciseCollectionBuilder {
            collection: ExerciseCollection {
                module_exercises: vec![],
            },
        }
    }

    pub fn render(
        &self,
        output_dir: impl AsRef<Path>,
    ) -> Result<HashMap<PathBuf, PathBuf>, RenderExercisesError> {
        let output_dir = output_dir.as_ref();
        let exercise_root_dir = output_dir.join("exercises");
        exercise_root_dir.create_dir_all()?;
        let mut exercise_output_paths = HashMap::new();

        for mod_ex in self.module_exercises.iter() {
            let mod_ex_out_dir = {
                let mut d = exercise_root_dir.clone();
                d.push(to_prefixed_tag(mod_ex.name, mod_ex.index));
                d
            };
            mod_ex_out_dir.create_dir_all()?;

            for unit_ex in mod_ex.unit_exercises.iter() {
                let unit_ex_out_dir = {
                    let mut d = mod_ex_out_dir.clone();
                    d.push(to_prefixed_tag(unit_ex.name, unit_ex.index));
                    d
                };
                unit_ex_out_dir.create_dir_all()?;

                for ex_pack in unit_ex.exercises.iter() {
                    let ex_pack_out_dir = {
                        let mut d = unit_ex_out_dir.clone();
                        d.push(to_prefixed_tag(ex_pack.name, ex_pack.index));
                        d
                    };
                    ex_pack_out_dir.create_dir_all()?;

                    let content = ex_pack.path.get_dir_content()?;

                    // Create globset to match included files
                    let mut globset = globset::GlobSetBuilder::new();
                    for include in ex_pack.includes {
                        globset.add(
                            globset::Glob::new(ex_pack.path.join(include).to_str().unwrap())
                                .into_report()
                                .attach_printable_lazy(|| {
                                    format!("Error parsing include glob '{include}'")
                                })
                                .change_context(RenderExercisesError)?,
                        );
                    }
                    let globset = globset.build().unwrap();

                    for included_file in content.files.iter().filter(|f| globset.is_match(f)) {
                        let included_file_relative = Path::new(&included_file)
                            .strip_prefix(ex_pack.path)
                            .unwrap();
                        let included_file_dest = ex_pack_out_dir.join(included_file_relative);
                        let include_file_dest_dir = included_file_dest.parent().unwrap();
                        include_file_dest_dir.create_dir_all()?;
                        included_file.copy(included_file_dest)?;
                    }

                    let ex_pack_out_dir = ex_pack_out_dir
                        .strip_prefix(output_dir)
                        .unwrap()
                        .to_path_buf();
                    exercise_output_paths.insert(ex_pack.path.to_path_buf(), ex_pack_out_dir);
                }
            }
        }

        Ok(exercise_output_paths)
    }
}

#[derive(Debug)]
pub struct ModuleExercises<'track> {
    index: usize,
    name: &'track str,
    unit_exercises: Vec<UnitExercises<'track>>,
}

#[derive(Debug)]
pub struct UnitExercises<'track> {
    index: usize,
    name: &'track str,
    exercises: Vec<ExercisePackage<'track>>,
}

#[derive(Debug)]
pub struct ExercisePackage<'track> {
    index: usize,
    name: &'track str,
    path: &'track Path,
    includes: &'track [String],
}

pub struct ExerciseCollectionBuilder<'track> {
    collection: ExerciseCollection<'track>,
}

impl<'track> ExerciseCollectionBuilder<'track> {
    pub fn module(
        &mut self,
        name: &'track str,
        index: usize,
    ) -> ModuleExercisesBuilder<'track, '_> {
        ModuleExercisesBuilder {
            collection_buider: self,
            module_exercises: ModuleExercises {
                index,
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
    pub fn unit<'m>(
        &'m mut self,
        name: &'track str,
        index: usize,
    ) -> UnitExercisesBuilder<'track, 'c, 'm> {
        UnitExercisesBuilder {
            module_builder: self,
            unit_exercises: UnitExercises {
                index,
                name,
                exercises: vec![],
            },
        }
    }

    pub fn add(self) -> &'c mut ExerciseCollectionBuilder<'track> {
        self.collection_buider
            .collection
            .module_exercises
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
        let index = self.unit_exercises.exercises.len() + 1;
        self.unit_exercises.exercises.push(ExercisePackage {
            index,
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
