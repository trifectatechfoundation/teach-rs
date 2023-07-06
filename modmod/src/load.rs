use std::{fs, path::Path};

use serde::de::DeserializeOwned;

use crate::{Module, PathTo, Topic, Track};

pub trait Load: DeserializeOwned + Sized {
    fn load(path: &Path, base_path: Option<&Path>) -> crate::Result<PathTo<Self>> {
        let path = base_path.map(|b| b.join(path)).unwrap_or(path.to_owned()).canonicalize()?;
        let content = fs::read_to_string(&path)?;
        let data = toml::from_str(&content)?;

        Ok(PathTo {
            path: path.canonicalize()?,
            data,
        })
    }
}

impl Load for Track {}
impl Load for Module {}
impl Load for Topic {}