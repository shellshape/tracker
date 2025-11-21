use anyhow::Result;
use clap::Parser;
use commands::*;
use config::Config;
use store::Store;

mod commands;
mod config;
mod migrations;
mod store;
mod util;

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

    migrations::migrate()?;

    let store = Store::new(&config.storage_dir)?;

    cli.commands.run(&store, &config)
}
