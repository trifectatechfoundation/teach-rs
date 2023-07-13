use std::path::{Path, PathBuf};

use crate::{load::Load, Result};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    #[serde(default = "exercise_description_md")]
    pub description: PathBuf,
    #[serde(default = "exercise_includes")]
    pub includes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Topic {
    pub name: String,
    #[serde(default)]
    pub dependencies: Vec<PathBuf>,
    #[serde(default)]
    pub exercises: Vec<Exercise>,
    #[serde(default)]
    pub summary: Vec<String>,
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default = "topic_slides_md")]
    pub content: PathBuf,
    #[serde(default)]
    pub further_reading: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Unit {
    pub name: String,
    #[serde(default = "unit_template_md")]
    pub template: PathBuf,
    pub topics: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Module {
    pub units: Vec<Unit>,
    pub name: String,
    pub description: String,
}

impl PathTo<Module> {
    #[allow(clippy::type_complexity)]
    pub fn load_topics(&self) -> Result<Vec<(&Unit, Vec<PathTo<Topic>>)>> {
        let Self { path, data } = self;
        let mut units = Vec::with_capacity(data.units.len());
        let base_path = path.parent().unwrap();

        for unit in data.units.iter() {
            let mut topics = Vec::with_capacity(unit.topics.len());
            for topic in unit.topics.iter() {
                topics.push(Topic::load(topic, Some(base_path))?)
            }
            units.push((unit, topics));
        }

        Ok(units)
    }
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub name: String,
    pub modules: Vec<PathBuf>,
    #[serde(default)]
    pub excluded_topics: Vec<PathBuf>,
}

impl Track {
    pub fn load(path: impl AsRef<Path>) -> Result<PathTo<Self>> {
        Load::load(path.as_ref(), None)
    }
}

impl PathTo<Track> {
    pub fn load_modules(&self) -> Result<Vec<PathTo<Module>>> {
        let Self { path, data } = self;
        let mut modules = Vec::with_capacity(data.modules.len());
        let base_path = Some(path.parent().unwrap());

        for path in data.modules.iter() {
            modules.push(Module::load(path, base_path)?);
        }

        Ok(modules)
    }
}

#[derive(Debug)]
pub struct PathTo<T> {
    pub data: T,
    pub path: PathBuf,
}

fn topic_slides_md() -> PathBuf {
    PathBuf::from("slides.md")
}

fn unit_template_md() -> PathBuf {
    PathBuf::from("template.md")
}

fn exercise_description_md() -> PathBuf {
    PathBuf::from("description.md")
}

fn exercise_includes() -> Vec<String> {
    ["Cargo.toml", "Cargo.lock", "src/**/*"]
        .map(String::from)
        .to_vec()
}
