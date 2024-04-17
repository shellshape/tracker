use std::{ops::Deref, path::Path};

use anyhow::Result;
use figment::{
    providers::{Format, Json, Toml, Yaml},
    Figment,
};
use serde::Deserialize;

macro_rules! package_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

macro_rules! local_config_name {
    ($ext:expr) => {
        concat!(package_name!(), $ext)
    };
}

#[derive(Deserialize)]
pub struct Config {}

impl Config {
    pub fn parse<T: AsRef<Path>>(dir: Option<T>) -> Result<Self> {
        dir.map(Self::parse_from_file)
            .unwrap_or_else(Self::parse_from_cfgdir)
    }

    pub fn parse_from_cfgdir() -> Result<Self> {
        let dirs = dirs::config_dir()
            .map(|d| d.join(package_name!()))
            .ok_or_else(|| anyhow::anyhow!("could not resolve project directories"))?;

        Ok(Figment::new()
            .merge(Toml::file(local_config_name!(".toml")))
            .merge(Yaml::file(local_config_name!(".yaml")))
            .merge(Json::file(local_config_name!(".json")))
            .merge(Toml::file(dirs.join("config.toml")))
            .merge(Yaml::file(dirs.join("config.yml")))
            .merge(Json::file(dirs.join("config.json")))
            .extract()?)
    }

    pub fn parse_from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        let ext = path.as_ref().extension().unwrap_or_default();
        let mut figment = Figment::new();

        figment = match ext.to_string_lossy().deref() {
            "yml" | "yaml" => figment.merge(Yaml::file(path)),
            "toml" => figment.merge(Toml::file(path)),
            "json" => figment.merge(Json::file(path)),
            _ => anyhow::bail!("invalid config file type"),
        };

        Ok(figment.extract()?)
    }
}
