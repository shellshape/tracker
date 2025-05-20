mod commands;
mod config;
mod store;
mod util;

use anyhow::Result;
use clap::{Parser, command};
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

register_commands! {
    Add
    View
    Delete
    Edit
    Insert
}

#[cfg(feature = "clap-markdown")]
fn main() {
    clap_markdown::print_help_markdown::<Cli>();
}

#[cfg(not(feature = "clap-markdown"))]
fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::parse(cli.config)?;
    let store = Store::new(&config.storage_dir)?;

    cli.commands.run(&store, &config)
}
