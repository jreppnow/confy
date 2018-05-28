//! Easy configuration management
//!
//!

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate directories;
extern crate toml;

mod utils;

use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Error as IoError, ErrorKind::NotFound};
use std::{fs::File, path::PathBuf};

use utils::*;

/// Load a configuration from the standard OS local scope for
/// the current user.
pub fn load<T: Serialize + DeserializeOwned + Default>(name: &str) -> Result<T, IoError> {
    let project = ProjectDirs::from("rs", name, name);

    let path: PathBuf = [
        project.config_dir().to_str().unwrap(),
        &format!("{}.toml", name),
    ].iter()
        .collect();

    match File::open(path) {
        Ok(mut cfg) => Ok(toml::from_str(&cfg.get_string().unwrap()).unwrap()),
        Err(ref e) if e.kind() == NotFound => {
            utils::scaffold_directories()?;
            store(name, T::default())?;
            Ok(T::default())
        }
        Err(e) => Err(e.into()),
    }
}

/// Store a configuration object
pub fn store(name: &str, cfg: impl Serialize + Default) -> Result<(), IoError> {
    Ok(())
}
