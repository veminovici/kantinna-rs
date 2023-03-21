use serde::{Deserialize, Serialize};
use std::{env, fmt::Display, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppInfo {
    cwd: PathBuf,
    package: String,
    version: String,
    authors: String,
    homepage: String,
    os: String,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            cwd: env::current_dir().unwrap(),
            package: env!("CARGO_PKG_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            homepage: env!("CARGO_PKG_HOMEPAGE").to_owned(),
            authors: env!("CARGO_PKG_AUTHORS").to_owned(),
            os: os_info::get().to_string(),
        }
    }
}

pub const APP: &str = "app";

impl Display for AppInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{APP}]")?;
        let contents = toml::to_string(&self).unwrap();
        write!(f, "{contents}")
    }
}
