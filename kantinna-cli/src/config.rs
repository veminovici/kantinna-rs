use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::Path};

pub const CONFIG_TOML: &str = "kantinna.toml";
const PLUGINS_PATH: &str = "./kantinaa/plugins";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    plugins_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            plugins_path: PLUGINS_PATH.to_string(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents = toml::to_string(&self).unwrap();
        write!(f, "{contents}")
    }
}

impl Config {
    pub fn from_file_or_default<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::from_file(path).unwrap_or(Config::default())
    }

    fn plugins_path(&self) -> String {
        self.plugins_path.clone()
    }

    fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let txt = std::fs::read_to_string(path)?;
        toml::from_str(txt.as_str()).map_err(anyhow::Error::new)
    }
}
