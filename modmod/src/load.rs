use std::{
    any::type_name,
    fmt, fs,
    path::{Path, PathBuf},
};

use error_stack::{IntoReport, Result, ResultExt};
use serde::{de::DeserializeOwned, Deserialize};

use crate::io::PathExt;

use super::{Exercise, Module, Topic, Track, Unit};

#[derive(Debug, Deserialize)]
pub struct TrackDef {
    pub name: String,
    pub modules: Vec<PathBuf>,
}

impl PathTo<TrackDef> {
    pub fn resolve(self) -> Result<Track, HydrateTrackError> {
        let PathTo {
            data,
            path: track_path,
        } = self;
        let TrackDef {
            name,
            modules: module_paths,
        } = data;

        let mut modules = Vec::with_capacity(module_paths.len());
        let base_path = track_path.parent().unwrap();
        for module_path in module_paths.into_iter() {
            modules.push(
                ModuleDef::load(&module_path, Some(base_path))
                    .change_context(HydrateTrackError)?
                    .resolve()?,
            );
        }

        Ok(Track { name, modules })
    }
}

#[derive(Debug, Deserialize)]
pub struct ModuleDef {
    pub name: String,
    pub description: String,
    pub units: Vec<UnitDef>,
}

impl PathTo<ModuleDef> {
    fn resolve(self) -> Result<Module, HydrateTrackError> {
        let PathTo {
            data: def,
            path: module_path,
        } = self;
        let ModuleDef {
            name,
            description,
            units: unit_defs,
        } = def;

        let mut units = Vec::with_capacity(unit_defs.len());
        let base_path = module_path.parent().unwrap();
        for unit_def in unit_defs {
            units.push(unit_def.resolve(base_path)?);
        }

        Ok(Module {
            name,
            description,
            units,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct UnitDef {
    pub name: String,
    #[serde(default = "crate::load::serde_defaults::unit_template_md")]
    pub template: PathBuf,
    pub topics: Vec<PathBuf>,
}

impl UnitDef {
    fn resolve(self, base_path: &Path) -> Result<Unit, HydrateTrackError> {
        let UnitDef {
            name,
            template,
            topics: topic_paths,
        } = self;

        let mut topics = Vec::with_capacity(topic_paths.len());
        for topic_path in topic_paths {
            topics.push(
                TopicDef::load(&topic_path, Some(base_path))
                    .change_context(HydrateTrackError)?
                    .resolve()?,
            );
        }
        let template = base_path
            .join(template)
            .canonicalize()
            .into_report()
            .change_context(HydrateTrackError)?;
        Ok(Unit {
            name,
            template,
            topics,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct TopicDef {
    pub name: String,
    #[serde(default)]
    pub exercises: Vec<ExerciseDef>,
    #[serde(default)]
    pub summary: Vec<String>,
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default = "crate::load::serde_defaults::topic_slides_md")]
    pub content: PathBuf,
    #[serde(default)]
    pub further_reading: Vec<String>,
}

impl PathTo<TopicDef> {
    fn resolve(self) -> Result<Topic, HydrateTrackError> {
        let PathTo {
            data: def,
            path: topic_path,
        } = self;

        let TopicDef {
            name,
            exercises: exercise_defs,
            summary,
            objectives,
            content,
            further_reading,
        } = def;

        let mut exercises = Vec::new();
        let base_path = topic_path.parent().unwrap();
        for exercise_def in exercise_defs {
            exercises.push(exercise_def.resolve(base_path)?)
        }

        let content = base_path
            .join(content)
            .canonicalize()
            .into_report()
            .change_context(HydrateTrackError)?;

        let images = base_path.join("images");
        let images = images
            .is_dir()
            .then_some(
                images
                    .get_dir_content()
                    .map(|c| c.files.into_iter().map(PathBuf::from)),
            )
            .transpose()?
            .into_iter()
            .flatten()
            .collect();

        Ok(Topic {
            name,
            exercises,
            summary,
            objectives,
            content,
            further_reading,
            images,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ExerciseDef {
    pub name: String,
    pub path: PathBuf,
    #[serde(default = "crate::load::serde_defaults::exercise_description_md")]
    pub description: PathBuf,
    #[serde(default = "crate::load::serde_defaults::exercise_includes")]
    pub includes: Vec<String>,
}

impl ExerciseDef {
    fn resolve(self, base_path: &Path) -> Result<Exercise, HydrateTrackError> {
        let ExerciseDef {
            name,
            path: exercise_path,
            description,
            includes,
        } = self;
        let path = base_path
            .join(exercise_path)
            .canonicalize()
            .into_report()
            .change_context(HydrateTrackError)?;
        let description = path
            .join(description)
            .canonicalize()
            .into_report()
            .change_context(HydrateTrackError)?;
        Ok(Exercise {
            name,
            path,
            description,
            includes,
        })
    }
}

#[derive(Debug)]
pub struct LoadError(&'static str, PathBuf);

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LoadError(ty, path) = self;
        write!(
            f,
            "unable to load item of type {ty} from path {path}",
            path = path.to_string_lossy()
        )
    }
}

impl error_stack::Context for LoadError {}

pub trait Load: DeserializeOwned + Sized + 'static {
    fn load(path: &Path, base_path: Option<&Path>) -> Result<PathTo<Self>, LoadError> {
        let path = base_path.map(|b| b.join(path)).unwrap_or(path.to_owned());
        let path = path
            .canonicalize()
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to canonicalize path {path}. Make sure the path leads to an existing file.",
                    path = path.to_string_lossy()
                )
            })
            .change_context_lazy(|| LoadError(type_name::<Self>(), path.clone()))?;
        let content = fs::read_to_string(&path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to read contents of file at path {path}",
                    path = path.to_string_lossy()
                )
            })
            .change_context_lazy(|| LoadError(type_name::<Self>(), path.clone()))?;
        let data = toml::from_str(&content)
            .into_report()
            .attach_printable_lazy(|| {
                format!("Unable to parse TOML file with contents '{content}'")
            })
            .change_context_lazy(|| LoadError(type_name::<Self>(), path.clone()))?;
        Ok(PathTo { path, data })
    }
}

impl Load for TrackDef {}
impl Load for ModuleDef {}
impl Load for TopicDef {}

#[derive(Debug)]
pub struct PathTo<T> {
    pub data: T,
    pub path: PathBuf,
}

#[derive(Debug, Default)]
pub struct HydrateTrackError;

impl fmt::Display for HydrateTrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error resolving track path references")
    }
}

impl error_stack::Context for HydrateTrackError {}

#[doc(hidden)]
pub mod serde_defaults {
    use std::path::PathBuf;

    pub fn exercise_description_md() -> PathBuf {
        PathBuf::from("description.md")
    }

    pub fn exercise_includes() -> Vec<String> {
        ["Cargo.toml", "Cargo.lock", "src/**/*"]
            .map(String::from)
            .to_vec()
    }

    pub fn topic_slides_md() -> PathBuf {
        PathBuf::from("slides.md")
    }

    pub fn unit_template_md() -> PathBuf {
        PathBuf::from("template.md")
    }
}
