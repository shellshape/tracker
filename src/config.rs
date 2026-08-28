use anyhow::Result;
use fancy_duration::FancyDuration;
use figment::Figment;
use figment::providers::{Format, Json, Toml, Yaml};
use serde::{Deserialize, Serialize};
use std::fmt;
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

pub fn default_storage_dir() -> PathBuf {
    dirs::data_dir()
        .expect("data directory")
        .join("time_trackings")
}

#[derive(Deserialize, Serialize)]
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

    #[serde(skip)]
    config_path: Option<PathBuf>,
}

fn merge_from_file(figment: Figment, path: impl AsRef<Path>) -> Result<Figment> {
    match path
        .as_ref()
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .deref()
    {
        "yml" | "yaml" => Ok(figment.merge(Yaml::file(path))),
        "toml" => Ok(figment.merge(Toml::file(path))),
        "json" => Ok(figment.merge(Json::file(path))),
        _ => Err(anyhow::anyhow!("invalid config file type")),
    }
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

        let candidates = [
            PathBuf::from(local_config_name!(".toml")),
            PathBuf::from(local_config_name!(".yaml")),
            PathBuf::from(local_config_name!(".json")),
            dirs.join("config.toml"),
            dirs.join("config.yml"),
            dirs.join("config.json"),
        ];

        let mut figment = Figment::new();
        for path in &candidates {
            figment = merge_from_file(figment, path)?;
        }

        let mut config: Self = figment.extract()?;
        config.config_path = candidates.into_iter().rev().find(|p| p.exists());

        Ok(config)
    }

    pub fn parse_from_file<T: AsRef<Path>>(path: T) -> Result<Self> {
        let path = path.as_ref();
        let mut figment = Figment::new();

        figment = merge_from_file(figment, path)?;

        let mut config: Self = figment.extract()?;
        config.config_path = Some(path.to_path_buf());

        Ok(config)
    }

    pub fn path(&self) -> Option<&Path> {
        self.config_path.as_deref()
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_json::ser::to_string_pretty(self).expect("config as json");
        f.write_str(&s)
    }
}
