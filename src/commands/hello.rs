use super::Command;
use anyhow::Result;
use clap::Args;

/// Say hello to world or someone you want to greet
#[derive(Args)]
pub struct Hello {
    /// Greet someone, if you want
    name: Option<String>,
}

impl Command for Hello {
    fn run(&self) -> Result<()> {
        println!(
            "Hello, {}!",
            self.name.clone().unwrap_or_else(|| "world".into())
        );
        Ok(())
    }
}
