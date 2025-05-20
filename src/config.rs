use anyhow::Result;
use fancy_duration::FancyDuration;
use figment::Figment;
use figment::providers::{Format, Json, Toml, Yaml};
use serde::Deserialize;
use std::ops::Deref;
use std::path::{Path, PathBuf};

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

fn default_break_regex() -> String {
    "(?i)^(?:break|pause)$".to_string()
}

fn default_start_regex() -> String {
    "(?i)^start$".to_string()
}

fn default_end_regex() -> String {
    "(?i)^end$".to_string()
}

fn default_storage_dir() -> PathBuf {
    dirs::home_dir()
        .expect("home directory")
        .join("time_trackings")
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_storage_dir")]
    pub storage_dir: PathBuf,

    #[serde(default = "default_start_regex")]
    pub start_regex: String,

    #[serde(default = "default_break_regex")]
    #[serde(alias = "pause_regex")] // backwards compat
    pub break_regex: String,

    #[serde(default = "default_end_regex")]
    pub end_regex: String,

    pub round_steps: Option<FancyDuration<chrono::Duration>>,
}

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
            _ => return Err(anyhow::anyhow!("invalid config file type")),
        };

        Ok(figment.extract()?)
    }
}
