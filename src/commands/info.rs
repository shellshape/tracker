use super::Command;
use crate::config::Config;
use crate::db::Database;
use anyhow::Result;
use clap::Args;
use std::borrow::Cow;
use std::path::Path;

/// Show information like config or database location
#[derive(Args)]
pub struct Info {}

impl Command for Info {
    fn run(&self, db: &Database, config: &Config) -> Result<()> {
        let config_location = config
            .path()
            .map(Path::to_string_lossy)
            .unwrap_or(Cow::Borrowed("<no config file provided or existing>"));
        let db_path = db.path().to_string_lossy();

        println!(
            "\
            Database Path:  {db_path}\n\
            Config Path:    {config_location}\n\
            Config:\n\
            {config}\
            "
        );

        Ok(())
    }
}
