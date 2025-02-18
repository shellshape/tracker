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
    Edit
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::parse(cli.config)?;

    let store = Store::new(&config.storage_dir)?;

    cli.commands.run(&store, &config)?;

    Ok(())
}
