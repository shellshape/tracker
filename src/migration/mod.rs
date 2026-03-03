use crate::config::Config;
use anyhow::Result;

mod migrations;
mod store;
mod util;

pub fn migrate(config: &Config) -> Result<()> {
    migrations::default_config_dir()?;
    migrations::sqlite_database(config)?;
    Ok(())
}
