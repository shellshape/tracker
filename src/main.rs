mod commands;
mod config;
mod store;
mod util;

use anyhow::Result;
use clap::{command, Parser};
use commands::*;
use config::Config;
use store::Store;

/// Simple tool to do time tracking
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a config file
    #[arg(short, long)]
    config: Option<String>,

    #[command(subcommand)]
    commands: Commands,
}

// List the names of your sub commands here.
register_commands! {
    Add
    View
    Delete
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let _ = Config::parse(cli.config)?;

    let store = Store::new(
        dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("could not find home directory"))?
            .join("time_trackings"),
    )?;

    cli.commands.run(&store)?;

    Ok(())
}
