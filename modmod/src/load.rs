use std::{any::type_name, fs, path::Path};

use serde::de::DeserializeOwned;

use crate::{Module, PathTo, Topic, Track};

pub trait Load: DeserializeOwned + Sized {
    fn load(path: &Path, base_path: Option<&Path>) -> crate::Result<PathTo<Self>> {
        let path = base_path.map(|b| b.join(path)).unwrap_or(path.to_owned());
        println!("Attempting to load {} at {path:?}", type_name::<Self>());
        let path = path.canonicalize()?;
        let content = fs::read_to_string(&path)?;
        let data = toml::from_str(&content)?;
        println!("Loaded {} from {path:?}", type_name::<Self>());
        Ok(PathTo {
            path: path.canonicalize()?,
            data,
        })
    }
}

impl Load for Track {}
impl Load for Module {}
impl Load for Topic {}
