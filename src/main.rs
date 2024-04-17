mod commands;
mod config;

use anyhow::Result;
use clap::{command, Parser};
use commands::*;
use config::Config;

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
    Hello
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cfg = Config::parse(cli.config)?;

    cli.commands.run()?;

    Ok(())
}
