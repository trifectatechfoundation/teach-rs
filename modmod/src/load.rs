use std::{any::type_name, fmt, fs, path::Path};

use crate::{Module, PathTo, Topic, Track};
use error_stack::{IntoReport, Result, ResultExt};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct LoadError(&'static str);

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unable to load item of type {}", self.0)
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
            .change_context(LoadError(type_name::<Self>()))?;
        let content = fs::read_to_string(&path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to read contents of file at path {path}",
                    path = path.to_string_lossy()
                )
            })
            .change_context(LoadError(type_name::<Self>()))?;
        let data = toml::from_str(&content)
            .into_report()
            .attach_printable_lazy(|| format!("Unable to parse TOML file with contents {content}"))
            .change_context(LoadError(type_name::<Self>()))?;
        Ok(PathTo { path, data })
    }
}

impl Load for Track {}
impl Load for Module {}
impl Load for Topic {}
